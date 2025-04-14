use crate::models::passkey::Passkey;
use crate::{auth::AuthEntity, db::AuthRsDatabase, errors::{ApiError, ApiResult, AppError}, models::{
    audit_log::{AuditLog, AuditLogAction, AuditLogEntityType},
    http_response::HttpResponse,
    user::{User},
}, utils::response::json_response, REGISTRATIONS};
use rocket::{
    http::Status,
    post,
    serde::{json::Json, Deserialize, Serialize},
};
use rocket_db_pools::Connection;
use std::collections::HashMap;
use mongodb::bson::{DateTime, Uuid};
use webauthn_rs::prelude::{
    PublicKeyCredential,
    RegisterPublicKeyCredential, RequestChallengeResponse,
};
use crate::routes::auth::passkey::get_webauthn;

// DTO for passkey registration finish request
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct PasskeyRegisterFinishRequest {
    pub registration_id: Uuid,
    pub credential: RegisterPublicKeyCredential,
}

// Response for passkey registration finish
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct PasskeyRegisterFinishResponse {
    pub id: String,
    pub name: String,
    pub created_at: DateTime,
}

#[post("/auth/passkeys/register/finish", format = "json", data = "<data>")]
pub async fn register_finish(
    db: Connection<AuthRsDatabase>,
    data: Json<PasskeyRegisterFinishRequest>,
    req_entity: AuthEntity,
) -> (Status, Json<HttpResponse<PasskeyRegisterFinishResponse>>) {
    match process_register_finish(db, data.into_inner(), req_entity).await {
        Ok(response) => json_response(HttpResponse {
            status: 200,
            message: "Passkey registered successfully".to_string(),
            data: Some(response),
        }),
        Err(err) => json_response(err.into()),
    }
}

async fn process_register_finish(
    db: Connection<AuthRsDatabase>,
    data: PasskeyRegisterFinishRequest,
    req_entity: AuthEntity,
) -> ApiResult<PasskeyRegisterFinishResponse> {
    if req_entity.is_token() {
        return Err(ApiError::Forbidden("Forbidden".to_string()));
    }

    let Some(user) = req_entity.user else {
        return Err(ApiError::NotFound("User not found".into()));
    };

    // Get the registration state
    let (user_id, reg_state) = REGISTRATIONS
        .lock()
        .await
        .remove(&data.registration_id)
        .ok_or(ApiError::InvalidState("Registration not found".to_string()))?;

    if user.id != user_id {
        return Err(ApiError::Unauthorized(
            "User IDs are non matching!".to_string(),
        ));
    }

    let mut passkeys = Passkey::get_by_owner(user_id, &db)
        .await
        .map_err(|_| ApiError::AppError(AppError::PasskeyNotFound(user_id)))?;

    // Initialize Webauthn
    let webauthn = get_webauthn();

    // Verify and process registration
    let result = webauthn
        .finish_passkey_registration(&data.credential, &reg_state)
        .map_err(|e| ApiError::AppError(AppError::WebauthnError))?;

    // Find the user (get the mutable user)
    //TODO: Check if we can use the req_entity user as mut
    let mut user = User::get_by_id(user_id, &db)
        .await
        .map_err(|_| ApiError::NotFound("User not found".to_string()))?;

    let passkey = Passkey::new(
        result.cred_id(),
        format!("Passkey {}", passkeys.len() + 1),
        user_id,
        result.clone(),
    );

    let old_values = HashMap::from([(
        "passkeys".to_string(),
        passkeys
            .iter()
            .map(|pk| pk.id.to_string())
            .collect::<Vec<_>>()
            .join(","),
    )]);

    // Add passkey to user and update
    passkeys.push(passkey.clone());

    let new_values = HashMap::from([(
        "passkeys".to_string(),
        passkeys
            .iter()
            .map(|pk| pk.id.to_string())
            .collect::<Vec<_>>()
            .join(","),
    )]);

    user.update(&db)
        .await
        .map_err(|e| ApiError::AppError(AppError::DatabaseError(e.to_string())))?;

    AuditLog::new(
        user_id.clone(),
        AuditLogEntityType::User,
        AuditLogAction::Update,
        "Added passkey.".to_string(),
        user_id,
        Some(old_values),
        Some(new_values),
    )
    .insert(&db)
    .await
    .map_err(|e| ApiError::AppError(AppError::DatabaseError(e.to_string())))?;

    // Return success response
    Ok(PasskeyRegisterFinishResponse {
        id: passkey.id,
        name: passkey.name,
        created_at: passkey.created_at,
    })
}
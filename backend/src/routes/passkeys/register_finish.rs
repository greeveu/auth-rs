use crate::models::passkey::Passkey;
use crate::routes::auth::passkey::get_webauthn;
use crate::{
    auth::AuthEntity,
    db::AuthRsDatabase,
    errors::{ApiError, ApiResult, AppError},
    models::{
        audit_log::{AuditLog, AuditLogAction, AuditLogEntityType},
        http_response::HttpResponse,
    },
    utils::response::json_response,
};
use base64::Engine;
use mongodb::bson::{DateTime, Uuid};
use rocket::{
    http::Status,
    post,
    serde::{json::Json, Deserialize, Serialize},
};
use rocket_db_pools::Connection;
use webauthn_rs::prelude::RegisterPublicKeyCredential;

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

#[post("/passkeys/register/finish", format = "json", data = "<data>")]
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

    let session_id = format!("passkey_reg_{}", data.registration_id);
    let session = crate::models::session::Session::get_by_id(&session_id, &db)
        .await
        .map_err(|e| ApiError::InternalError(format!("Failed to retrieve registration session: {}", e)))?
        .ok_or(ApiError::InvalidState("Registration not found".to_string()))?;

    let (user_id, state_base64) = match session.data {
        crate::models::session::SessionData::PasskeyRegistration { user_id, state } => (user_id, state),
        _ => return Err(ApiError::InvalidState("Invalid session type".to_string())),
    };

    let state_bytes = base64::engine::general_purpose::STANDARD.decode(&state_base64)
        .map_err(|e| ApiError::InternalError(format!("Failed to decode registration state: {}", e)))?;
    
    let reg_state: webauthn_rs::prelude::PasskeyRegistration = serde_json::from_slice(&state_bytes)
        .map_err(|e| ApiError::InternalError(format!("Failed to deserialize registration state: {}", e)))?;

    if user.id != user_id {
        return Err(ApiError::Unauthorized(
            "User IDs are non matching!".to_string(),
        ));
    }

    // Initialize Webauthn
    let webauthn = get_webauthn();

    // Verify and process registration
    let result = webauthn
        .finish_passkey_registration(&data.credential, &reg_state)
        .map_err(|_| ApiError::AppError(AppError::WebauthnError))?;

    let passkey = Passkey::new(
        result.cred_id(),
        "New Passkey".to_string(),
        user_id,
        result.clone(),
    )
    .insert(&db)
    .await
    .map_err(|e| ApiError::AppError(AppError::DatabaseError(e.to_string())))?;

    AuditLog::new(
        passkey.id.clone(),
        AuditLogEntityType::Passkey,
        AuditLogAction::Create,
        "Registered passkey.".to_string(),
        user_id,
        None,
        None,
    )
    .insert(&db)
    .await
    .map_err(|e| ApiError::AppError(AppError::DatabaseError(e.to_string())))?;

    let _ = crate::models::session::Session::delete_by_id(&session_id, &db).await;

    Ok(PasskeyRegisterFinishResponse {
        id: passkey.id,
        name: passkey.name,
        created_at: passkey.created_at,
    })
}

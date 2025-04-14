use crate::models::passkey;
use crate::models::passkey::Passkey;
use crate::{auth::AuthEntity, db::AuthRsDatabase, errors::{ApiError, ApiResult, AppError}, models::{
    audit_log::{AuditLog, AuditLogAction, AuditLogEntityType},
    http_response::HttpResponse,
    user::{User, UserDTO},
}, utils::response::json_response, REGISTRATIONS};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;
use lazy_static::lazy_static;
use mongodb::bson::{DateTime, Uuid};
use rocket::{
    get,
    http::Status,
    post,
    serde::{json::Json, Deserialize, Serialize},
};
use rocket_db_pools::Connection;
use std::collections::HashMap;
use std::sync::Mutex;
use url::Url;
use webauthn_rs::prelude::{
    CreationChallengeResponse, PasskeyAuthentication, PasskeyRegistration, PublicKeyCredential,
    RegisterPublicKeyCredential, RequestChallengeResponse,
};
use webauthn_rs::{Webauthn, WebauthnBuilder};
use crate::routes::auth::passkey::get_webauthn;

// Response for passkey registration start
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct PasskeyRegisterStartResponse {
    pub registration_id: Uuid,
    pub challenge: CreationChallengeResponse,
}

#[get("/auth/passkeys/register/start")]
pub async fn register_start(
    db: Connection<AuthRsDatabase>,
    req_entity: AuthEntity,
) -> (Status, Json<HttpResponse<PasskeyRegisterStartResponse>>) {
    match process_register_start(db, req_entity).await {
        Ok(response) => json_response(HttpResponse {
            status: 200,
            message: "Passkey registration initiated".to_string(),
            data: Some(response),
        }),
        Err(err) => json_response(err.into()),
    }
}

async fn process_register_start(
    db: Connection<AuthRsDatabase>,
    req_entity: AuthEntity,
) -> ApiResult<PasskeyRegisterStartResponse> {
    if req_entity.is_token() {
        return Err(ApiError::Forbidden("Forbidden".to_string()));
    }

    let Some(user) = req_entity.user else {
        return Err(ApiError::NotFound("User not found".into()));
    };

    let passkeys = Passkey::get_by_owner(user.id, &db)
        .await
        .map_err(|_| ApiError::AppError(AppError::PasskeyNotFound(user.id)))?;

    // Initialize Webauthn
    let webauthn = get_webauthn();

    let excluded_credentials = passkeys
        .iter()
        .map(|passkey| passkey.credential.cred_id().clone())
        .collect::<Vec<_>>();

    let Ok((challenge, reg_state)) = webauthn.start_passkey_registration(
        uuid::Uuid::from_slice(&user.id.bytes()).unwrap(),
        &user.email,
        &(user.first_name + " " + &user.last_name),
        Some(excluded_credentials),
    ) else {
        return Err(ApiError::AppError(AppError::WebauthnError));
    };

    // Store registration state
    let registration_id = Uuid::new();
    REGISTRATIONS
        .lock()
        .await
        .insert(registration_id, (user.id, reg_state));

    Ok(PasskeyRegisterStartResponse {
        registration_id,
        challenge,
    })
}
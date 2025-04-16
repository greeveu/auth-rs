use crate::models::http_response::HttpResponse;
use crate::models::passkey::Passkey;
use crate::utils::response::json_response;
use crate::{auth::AuthEntity, db::AuthRsDatabase, errors::{ApiError, ApiResult, AppError}, REGISTRATIONS};
use mongodb::bson::Uuid;
use rocket::{
    get,
    http::Status,
    serde::{json::Json, Serialize},
};
use rocket_db_pools::Connection;
use webauthn_rs::prelude::CreationChallengeResponse;
use crate::routes::auth::passkey::get_webauthn;

// Response for passkey registration start
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct PasskeyRegisterStartResponse {
    pub registration_id: Uuid,
    pub challenge: CreationChallengeResponse,
}

#[get("/passkeys/register/start?<type>")]
pub async fn register_start(
    db: Connection<AuthRsDatabase>,
    req_entity: AuthEntity,
    r#type: &str,
) -> (Status, Json<HttpResponse<PasskeyRegisterStartResponse>>) {
    match process_register_start(db, req_entity, r#type).await {
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
    auth_type: &str,
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

    // Prepare user identifier and display name
    let user_id = uuid::Uuid::from_slice(&user.id.bytes()).unwrap();
    let user_email = &user.email;
    let display_name = &(user.first_name.clone() + " " + &user.last_name);
    
    // Get challenge and registration state based on auth type
    let (challenge, reg_state) = if auth_type == "virtual" {
        webauthn.start_google_passkey_in_google_password_manager_only_registration(
            user_id,
            user_email,
            display_name,
            Some(excluded_credentials),
        )
        .map_err(|_| ApiError::AppError(AppError::WebauthnError))?
    } else if auth_type == "physical" {
        webauthn.start_passkey_registration(
            user_id,
            user_email,
            display_name,
            Some(excluded_credentials),
        )
        .map_err(|_| ApiError::AppError(AppError::WebauthnError))?
    } else {
        return Err(ApiError::BadRequest(format!("Unsupported auth type: {}", auth_type)));
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
use crate::models::passkey;
use crate::models::passkey::Passkey;
use crate::{
    auth::AuthEntity,
    db::AuthRsDatabase,
    errors::{ApiError, ApiResult, AppError},
    models::{
        audit_log::{AuditLog, AuditLogAction, AuditLogEntityType},
        http_response::HttpResponse,
        user::{User, UserDTO},
    },
    utils::response::json_response,
};
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

// In-memory storage for registration and authentication sessions
lazy_static! {
    static ref REGISTRATIONS: Mutex<HashMap<Uuid, (Uuid, PasskeyRegistration)>> =
        Mutex::new(HashMap::new());
    static ref AUTHENTICATIONS: Mutex<HashMap<Uuid, PasskeyAuthentication>> =
        Mutex::new(HashMap::new());
}

//TODO: First create a config file for these values, secondly check if this needs to be instantiated every time or if it can be a static variable
// Initialize Webauthn instance
pub fn get_webauthn() -> Webauthn {
    let rp_id = "localhost"; // Should match your domain
    let rp_origin = Url::parse(&format!("http://{}", rp_id)).expect("Invalid URL");
    WebauthnBuilder::new(rp_id, &rp_origin)
        .expect("Invalid configuration")
        .rp_name(&"auth-rs")
        .build()
        .unwrap()
}

// DTO for passkey authentication finish request
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct PasskeyAuthenticateFinishRequest {
    pub authentication_id: Uuid,
    pub credential: PublicKeyCredential,
}

// Response for passkey authentication start
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct PasskeyAuthenticateStartResponse {
    pub challenge: RequestChallengeResponse,
    pub authentication_id: Uuid,
}

// Response for passkey authentication finish
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct PasskeyAuthenticateFinishResponse {
    pub user: UserDTO,
    pub token: String,
}

#[get("/auth/passkeys/authenticate/start")]
pub async fn authenticate_start() -> (Status, Json<HttpResponse<PasskeyAuthenticateStartResponse>>)
{
    match process_authenticate_start().await {
        Ok(response) => json_response(HttpResponse {
            status: 200,
            message: "Authentication initiated".to_string(),
            data: Some(response),
        }),
        Err(err) => json_response(err.into()),
    }
}

async fn process_authenticate_start() -> ApiResult<PasskeyAuthenticateStartResponse> {
    // Initialize Webauthn
    let webauthn = get_webauthn();

    // Generate challenge for authentication
    let (challenge, auth_state) = webauthn
        .start_passkey_authentication(&[])
        .map_err(|_| ApiError::AppError(AppError::WebauthnError))?;

    // Store authentication state
    let authentication_id = Uuid::new();
    AUTHENTICATIONS
        .lock()
        .unwrap()
        .insert(authentication_id, auth_state);

    Ok(PasskeyAuthenticateStartResponse {
        challenge,
        authentication_id,
    })
}

#[post("/auth/passkeys/authenticate/finish", format = "json", data = "<data>")]
pub async fn authenticate_finish(
    db: Connection<AuthRsDatabase>,
    data: Json<PasskeyAuthenticateFinishRequest>,
) -> (
    Status,
    Json<HttpResponse<PasskeyAuthenticateFinishResponse>>,
) {
    match process_authenticate_finish(db, data.into_inner()).await {
        Ok(response) => json_response(HttpResponse {
            status: 200,
            message: "Authentication successful".to_string(),
            data: Some(response),
        }),
        Err(err) => json_response(err.into()),
    }
}

//TODO: !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// !!!!!!!!VERIFY THIS FOR SECURITY!!!!!!
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
async fn process_authenticate_finish(
    db: Connection<AuthRsDatabase>,
    data: PasskeyAuthenticateFinishRequest,
) -> ApiResult<PasskeyAuthenticateFinishResponse> {
    // Get the authentication state
    let auth_state = AUTHENTICATIONS
        .lock()
        .unwrap()
        .remove(&data.authentication_id)
        .ok_or(ApiError::InvalidState(
            "Authentication not found".to_string(),
        ))?;

    // Initialize Webauthn
    let webauthn = get_webauthn();

    // Get credential ID as base64
    let credential_id = URL_SAFE_NO_PAD.encode(&data.credential.id);

    // Find user with this credential
    let passkey = Passkey::get_by_id(&credential_id, &db)
        .await
        .map_err(|_| ApiError::NotFound("Passkey not found with this credential".to_string()))?;

    let user = User::get_by_id(passkey.owner, &db)
        .await
        .map_err(|_| ApiError::NotFound("User not found with this credential".to_string()))?;

    // Verify authentication
    let result = webauthn
        .finish_passkey_authentication(&data.credential, &auth_state)
        .map_err(|e| ApiError::AppError(AppError::WebauthnError))?;

    // Update counter if needed
    //TODO: Check if this is required!
    // if pk.counter != result.counter() {
    //     pk.counter = result.1.counter;
    // }
    //
    // // Update user if counter changed
    // if should_update {
    //     user.update(&db)
    //         .await
    //         .map_err(|e| ApiError::AppError(AppError::DatabaseError(e.to_string())))?;
    // }

    AuditLog::new(
        user.clone().id,
        AuditLogEntityType::User,
        AuditLogAction::Login,
        "Login successful.".to_string(),
        user.id,
        None,
        None,
    )
    .insert(&db)
    .await
    .ok();

    // Return success with user information and token
    Ok(PasskeyAuthenticateFinishResponse {
        user: user.to_dto(),
        token: user.token,
    })
}

use mongodb::bson::Uuid;
use rocket::{
    delete, get,
    http::Status,
    patch,
    serde::{json::Json, Deserialize},
};
use rocket_db_pools::Connection;

use crate::{
    auth::AuthEntity,
    db::AuthRsDatabase,
    errors::{ApiError, ApiResult, AppError},
    models::{http_response::HttpResponse, passkey::PasskeyDTO, user::User},
    utils::response::json_response,
};
use crate::models::passkey::Passkey;

// DTO for updating passkey metadata
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct PasskeyUpdateRequest {
    pub name: Option<String>,
}

#[get("/passkeys")]
pub async fn list_passkeys(
    db: Connection<AuthRsDatabase>,
    req_entity: AuthEntity,
) -> (Status, Json<HttpResponse<Vec<PasskeyDTO>>>) {
    if !req_entity.is_user() || !req_entity.user.as_ref().unwrap().is_admin() {
        return json_response(HttpResponse::forbidden("Forbidden"));
    }

    match process_list_passkeys(db, req_entity).await {
        Ok(passkeys) => json_response(HttpResponse {
            status: 200,
            message: "Passkeys retrieved successfully".to_string(),
            data: Some(passkeys),
        }),
        Err(err) => json_response(err.into()),
    }
}

async fn process_list_passkeys(
    db: Connection<AuthRsDatabase>,
    req_entity: AuthEntity,
) -> ApiResult<Vec<PasskeyDTO>> {
    // Get the authenticated user
    let passkeys = Passkey::get_all(&db, None)
        .await
        .map_err(|e| ApiError::NotFound(format!("User not found: {}", e)))?
        .iter().map(|passkey| passkey.to_dto())
        .collect();

    // Get all passkey DTOs
    Ok(passkeys)
}

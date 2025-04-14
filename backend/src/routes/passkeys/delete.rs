use rocket::{
    delete,
    http::Status,
    patch,
    serde::{json::Json, Deserialize},
};
use rocket_db_pools::Connection;

use crate::{
    auth::AuthEntity,
    db::AuthRsDatabase,
    errors::{ApiError, ApiResult, AppError},
    models::{http_response::HttpResponse},
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

#[delete("/passkeys/<passkey_id>")]
pub async fn delete_passkey(
    db: Connection<AuthRsDatabase>,
    req_entity: AuthEntity,
    passkey_id: String,
) -> (Status, Json<HttpResponse<()>>) {
    let passkey = Passkey::get_by_id(passkey_id.as_str(), &db)
        .await
        .map_err(|_| ApiError::NotFound("Passkey not found".to_string()))?;

    // Verify that the user ID in the request matches the authenticated user's ID
    if passkey.owner != req_entity.user_id {
        return json_response(
            ApiError::Unauthorized("Cannot delete passkeys for another user".to_string()).into(),
        );
    }

    match process_delete_passkey(db, passkey_id.as_str()).await {
        Ok(_) => json_response(HttpResponse {
            status: 200,
            message: "Passkey deleted successfully".to_string(),
            data: None,
        }),
        Err(err) => json_response(err.into()),
    }
}

async fn process_delete_passkey(
    db: Connection<AuthRsDatabase>,
    passkey_id: &str,
) -> ApiResult<()> {
    // Get the authenticated user
    let passkey = Passkey::get_by_id(passkey_id, &db)
        .await
        .map_err(|e| ApiError::NotFound(format!("User not found: {}", e)))?;

    // Update the user
    passkey.delete(&db)
        .await
        .map_err(|e| ApiError::AppError(AppError::DatabaseError(e.to_string())))?;

    Ok(())
}

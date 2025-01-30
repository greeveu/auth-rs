use mongodb::bson::Uuid;
use rocket::{post, serde::{json::Json, Deserialize, Serialize}};
use rocket_db_pools::Connection;

use crate::{db::AuthRsDatabase, models::oauth_token::OAuthToken, OAUTH_CODES};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct TokenOAuthData {
    client_id: Uuid,
    client_secret: String,
    user_id: Option<Uuid>,
    code: String,
    redirect_uri: String,
    scope: Vec<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct TokenOAuthResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    scope: String,
}

#[allow(unused)]
#[post("/oauth/token", format = "json", data = "<data>")] 
pub async fn get_oauth_token(db: Connection<AuthRsDatabase>, data: Json<TokenOAuthData>) -> Option<Json<TokenOAuthResponse>> {
    let data = data.into_inner();

    let codes = OAUTH_CODES.lock().await;
    let code_data  = match codes.get(&data.code) {
        Some(code_data) => code_data,
        None => return None
    };

    if code_data.client_id != data.client_id || code_data.client_secret != data.client_secret || code_data.redirect_uri != data.redirect_uri {
        return None
    }

    let token = match OAuthToken::new(code_data.client_id, code_data.user_id.unwrap(), code_data.scope.clone(), 30 * 24 * 60 * 60).unwrap().insert(&db).await {
        Ok(token) => token,
        Err(_) => return None
    };

    Some(Json(TokenOAuthResponse {
        access_token: token.token,
        token_type: "Bearer".to_string(),
        expires_in: token.expires_in,
        scope: token.scope.join(","),
    }))
}
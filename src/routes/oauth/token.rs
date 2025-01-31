use mongodb::bson::Uuid;
use rocket::{post, serde::{json::Json, Deserialize, Serialize}};
use rocket_db_pools::Connection;

use crate::{db::AuthRsDatabase, models::{oauth_scope::OAuthScope, oauth_token::OAuthToken}, OAUTH_CODES};
use std::sync::Mutex;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct TokenOAuthData {
    pub client_id: Uuid,
    pub client_secret: String,
    pub user_id: Option<Uuid>,
    pub code: u16,
    pub redirect_uri: String,
    pub scope: Vec<OAuthScope>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct TokenOAuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub scope: String,
}

#[allow(unused)]
#[post("/oauth/token", format = "json", data = "<data>")] 
pub async fn get_oauth_token(db: Connection<AuthRsDatabase>, data: Json<TokenOAuthData>) -> Option<Json<TokenOAuthResponse>> {
    let data = data.into_inner();

    let mut codes = OAUTH_CODES.lock().await;
    let code_data  = match codes.get(&data.code) {
        Some(code_data) => code_data,
        None => return None
    };

    println!("{:?}", code_data);

    if code_data.client_id != data.client_id || code_data.client_secret != data.client_secret || code_data.redirect_uri != data.redirect_uri {
        return None
    }

    let token = match OAuthToken::new(code_data.client_id, code_data.user_id.unwrap(), code_data.scope.clone(), 30 * 24 * 60 * 60).unwrap().insert(&db).await {
        Ok(token) => token,
        Err(_) => return None
    };

    println!("{:?}", token);

    codes.remove(&data.code);

    println!("{:?}", codes);

    drop(codes);

    Some(Json(TokenOAuthResponse {
        access_token: token.token,
        token_type: "Bearer".to_string(),
        expires_in: token.expires_in,
        scope: token.scope.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(","),
    }))
}
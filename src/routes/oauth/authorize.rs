use mongodb::bson::Uuid;
use rocket::{post, serde::{json::Json, Deserialize, Serialize}, tokio};
use rocket_db_pools::Connection;

use crate::{auth::auth::AuthEntity, db::AuthRsDatabase, models::{oauth_application::OAuthApplication, oauth_scope::OAuthScope}, OAUTH_CODES};

use super::token::TokenOAuthData;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeOAuthData {
    client_id: Uuid,
    redirect_uri: String,
    scope: Vec<OAuthScope>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeOAuthResponse {
    pub client_id: Uuid,
    pub redirect_uri: String,
    pub code: u16
}

#[allow(unused)]
#[post("/oauth/authorize", format = "json", data = "<data>")] 
pub async fn authorize(db: Connection<AuthRsDatabase>, req_entity: AuthEntity, data: Json<AuthorizeOAuthData>) -> Option<Json<AuthorizeOAuthResponse>> {
    let data = data.into_inner();

    if !req_entity.is_user() {
        return None
    }

    let code = rand::random::<u16>();

    let oauth_application = match OAuthApplication::get_full_by_id(data.client_id.clone(), &db).await {
        Ok(app) => app,
        Err(err) => return None
    };

    if !oauth_application.redirect_uris.contains(&data.redirect_uri) {
        return None
    }

    let mut codes = OAUTH_CODES.lock().await;
    codes.insert(code, TokenOAuthData {
        client_id: oauth_application.id,
        client_secret: oauth_application.secret,
        user_id: Some(req_entity.user_id),
        code: code,
        scope: Some(data.scope.clone()),
        grant_type: "authorization_code".to_string(),
        redirect_uri: data.redirect_uri.clone()
    });
    drop(codes);

    
    // delete code after 5 minutes
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;
        let mut codes = OAUTH_CODES.lock().await;
        codes.remove(&code);
        drop(codes);
    });

    Some(Json(AuthorizeOAuthResponse {
        client_id: data.client_id,
        redirect_uri: data.redirect_uri,
        code: code
    }))
}
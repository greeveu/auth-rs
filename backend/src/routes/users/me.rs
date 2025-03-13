use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{
    auth::auth::AuthEntity,
    db::AuthRsDatabase,
    models::{
        http_response::HttpResponse,
        oauth_scope::{OAuthScope, ScopeActions},
        user::{User, UserMinimal},
    },
};

#[allow(unused)]
#[get("/users/@me", format = "json")]
pub async fn get_current_user(
    db: Connection<AuthRsDatabase>,
    req_entity: AuthEntity,
) -> Json<HttpResponse<UserMinimal>> {
    if req_entity.is_token()
        && (!req_entity
            .token
            .clone()
            .unwrap()
            .check_scope(OAuthScope::Users(ScopeActions::Read))
            || req_entity
                .token
                .clone()
                .unwrap()
                .check_scope(OAuthScope::Users(ScopeActions::All)))
    {
        return Json(HttpResponse::forbidden("Forbidden"));
    }

    match User::get_by_id(req_entity.user_id, &db).await {
        Ok(user) => Json(HttpResponse::success("Found user by id", user)),
        Err(err) => Json(err),
    }
}

#[allow(unused)]
#[get("/users/@me/plain", format = "json")]
pub async fn get_current_user_plain(
    db: Connection<AuthRsDatabase>,
    req_entity: AuthEntity,
) -> Option<Json<UserMinimal>> {
    if req_entity.is_token()
        && (!req_entity
            .token
            .clone()
            .unwrap()
            .check_scope(OAuthScope::Users(ScopeActions::Read))
            || req_entity
                .token
                .clone()
                .unwrap()
                .check_scope(OAuthScope::Users(ScopeActions::All)))
    {
        return None;
    }

    match User::get_by_id(req_entity.user_id, &db).await {
        Ok(user) => Some(Json(user)),
        Err(err) => None,
    }
}

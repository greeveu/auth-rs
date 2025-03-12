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
    utils::parse_uuid,
};

#[allow(unused)]
#[get("/users/<id>", format = "json")]
pub async fn get_user_by_id(
    db: Connection<AuthRsDatabase>,
    req_entity: AuthEntity,
    id: &str,
) -> Json<HttpResponse<UserMinimal>> {
    if req_entity.is_token()
        && (!req_entity
            .token
            .as_ref()
            .unwrap()
            .check_scope(OAuthScope::Users(ScopeActions::Read))
            || req_entity
                .token
                .as_ref()
                .unwrap()
                .check_scope(OAuthScope::Users(ScopeActions::All)))
    {
        return Json(HttpResponse {
            status: 403,
            message: "Forbidden".to_string(),
            data: None,
        });
    }

    let uuid = match parse_uuid(id) {
        Ok(uuid) => uuid,
        Err(err) => return Json(HttpResponse::from(err)),
    };

    if (req_entity.is_user()
        && req_entity.user_id != uuid
        && !req_entity.user.as_ref().unwrap().is_admin())
        || req_entity.is_token() && req_entity.user_id != uuid
    {
        return Json(HttpResponse {
            status: 403,
            message: "Missing permissions!".to_string(),
            data: None,
        });
    }

    match User::get_by_id(uuid, &db).await {
        Ok(user) => Json(HttpResponse {
            status: 200,
            message: "Found user by id".to_string(),
            data: Some(user),
        }),
        Err(err) => Json(err),
    }
}

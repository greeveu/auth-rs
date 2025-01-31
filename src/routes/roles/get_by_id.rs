use mongodb::bson::Uuid;
use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{auth::auth::AuthEntity, db::AuthRsDatabase, models::{http_response::HttpResponse, oauth_scope::{OAuthScope, ScopeActions}, role::Role}};

#[allow(unused)]
#[get("/roles/<id>", format = "json")] 
pub async fn get_role_by_id(db: Connection<AuthRsDatabase>, req_entity: AuthEntity, id: &str) -> Json<HttpResponse<Role>> {
    if req_entity.is_token() && !(req_entity.token.clone().unwrap().check_scope(OAuthScope::Roles(ScopeActions::Read)) || req_entity.token.unwrap().check_scope(OAuthScope::Roles(ScopeActions::All))) {
        return Json(HttpResponse {
            status: 403,
            message: "Forbidden".to_string(),
            data: None
        });
    }

    let uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(err) => return Json(HttpResponse {
            status: 400,
            message: format!("Invalid UUID: {:?}", err),
            data: None
        })
    };


    match Role::get_by_id(uuid, &db).await {
        Ok(role) => Json(HttpResponse {
            status: 200,
            message: "Found role by id".to_string(),
            data: Some(role),
        }),
        Err(err) => Json(err)
    }
}
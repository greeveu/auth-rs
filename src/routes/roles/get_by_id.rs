use mongodb::bson::Uuid;
use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{db::AuthRsDatabase, models::{http_response::HttpResponse, role::Role, user::User}};

#[allow(unused)]
#[get("/roles/<id>", format = "json")] 
pub async fn get_role_by_id(db: Connection<AuthRsDatabase>, req_user: User, id: &str) -> Json<HttpResponse<Role>> {
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
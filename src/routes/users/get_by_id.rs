use mongodb::bson::Uuid;
use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{db::AuthRsDatabase, models::{http_response::HttpResponse, user::{User, UserMinimal}}};

#[allow(unused)]
#[get("/users/<id>", format = "json")] 
pub async fn get_user_by_id(db: Connection<AuthRsDatabase>, req_user: User, id: &str) -> Json<HttpResponse<UserMinimal>> {
    let uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(err) => return Json(HttpResponse {
            status: 400,
            message: format!("Invalid UUID: {:?}", err),
            data: None
        })
    };

    if req_user.id != uuid && !req_user.is_global_admin() {
        return Json(HttpResponse {
            status: 403,
            message: "Missing permissions!".to_string(),
            data: None
        });
    }


    match User::get_by_id(uuid, &db).await {
        Ok(user) => Json(HttpResponse {
            status: 200,
            message: "Found user by id".to_string(),
            data: Some(user),
        }),
        Err(err) => Json(err)
    }
}
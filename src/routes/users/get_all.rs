use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{auth::auth::AuthEntity, db::AuthRsDatabase, models::{http_response::HttpResponse, user::{User, UserMinimal}}};

#[allow(unused)]
#[get("/users", format = "json")] 
pub async fn get_all_users(db: Connection<AuthRsDatabase>, req_entity: AuthEntity) -> Json<HttpResponse<Vec<UserMinimal>>> {
    if !req_entity.is_user() {
        return Json(HttpResponse {
            status: 403,
            message: "Forbidden".to_string(),
            data: None
        });
    }
    
    if !req_entity.user.unwrap().is_system_admin() {
        return Json(HttpResponse {
            status: 403,
            message: "Missing permissions!".to_string(),
            data: None
        });
    }

    match User::get_all(&db).await {
        Ok(users) => Json(HttpResponse {
            status: 200,
            message: "Successfully retrieved all users".to_string(),
            data: Some(users),
        }),
        Err(err) => Json(err)
    }
}
use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{db::AuthRsDatabase, models::{http_response::HttpResponse, role::Role, user::User}};

#[allow(unused)]
#[get("/roles", format = "json")] 
pub async fn get_all_roles(db: Connection<AuthRsDatabase>, req_user: User) -> Json<HttpResponse<Vec<Role>>> {
    if !req_user.is_global_admin() {
        return Json(HttpResponse {
            status: 403,
            message: "Missing permissions!".to_string(),
            data: None
        });
    }

    match Role::get_all(&db).await {
        Ok(roles) => Json(HttpResponse {
            status: 200,
            message: "Successfully retrieved all roles".to_string(),
            data: Some(roles),
        }),
        Err(err) => Json(err)
    }
}
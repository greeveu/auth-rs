use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{db::AuthRsDatabase, models::{http_response::HttpResponse, user::{User, UserMinimal}}};

#[allow(unused)]
#[get("/users/@me", format = "json")] 
pub async fn get_current_user(db: Connection<AuthRsDatabase>, req_user: User) -> Json<HttpResponse<UserMinimal>> {
    match User::get_by_id(req_user.id, &db).await {
        Ok(user) => Json(HttpResponse {
            status: 200,
            message: "Found user by id".to_string(),
            data: Some(user),
        }),
        Err(err) => Json(err)
    }
}

#[allow(unused)]
#[get("/users/@me/plain", format = "json")] 
pub async fn get_current_user_plain(db: Connection<AuthRsDatabase>, req_user: User) -> Option<Json<UserMinimal>> {
    match User::get_by_id(req_user.id, &db).await {
        Ok(user) => Some(Json(user)),
        Err(err) => None
    }
}
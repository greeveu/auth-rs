use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{auth::auth::AuthEntity, db::AuthRsDatabase, models::{http_response::HttpResponse, oauth_application::{OAuthApplication, OAuthApplicationMinimal}}};

#[allow(unused)]
#[get("/oauth-applications", format = "json")] 
pub async fn get_all_oauth_applications(db: Connection<AuthRsDatabase>, req_entity: AuthEntity) -> Json<HttpResponse<Vec<OAuthApplicationMinimal>>> {
    if !req_entity.is_user() || !req_entity.user.unwrap().is_system_admin() {
        return Json(HttpResponse {
            status: 403,
            message: "Forbidden".to_string(),
            data: None
        });
    }

    match OAuthApplication::get_all(&db).await {
        Ok(oauth_applications) => Json(HttpResponse {
            status: 200,
            message: "Successfully retrieved all oauth applications".to_string(),
            data: Some(oauth_applications),
        }),
        Err(err) => Json(err)
    }
}
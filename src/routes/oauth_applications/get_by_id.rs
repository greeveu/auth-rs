use mongodb::bson::Uuid;
use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{db::AuthRsDatabase, models::{http_response::HttpResponse, oauth_application::{OAuthApplication, OAuthApplicationMinimal}, user::User}};

#[allow(unused)]
#[get("/oauth-applications/<id>", format = "json")] 
pub async fn get_oauth_application_by_id(db: Connection<AuthRsDatabase>, req_user: User, id: &str) -> Json<HttpResponse<OAuthApplicationMinimal>> {
    let uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(err) => return Json(HttpResponse {
            status: 400,
            message: format!("Invalid UUID: {:?}", err),
            data: None
        })
    };


    match OAuthApplication::get_by_id(uuid, &db).await {
        Ok(oauth_application) => {
            if req_user.id != oauth_application.owner && !req_user.is_global_admin() {
                return Json(HttpResponse {
                    status: 403,
                    message: "Missing permissions!".to_string(),
                    data: None
                });
            }

            Json(HttpResponse {
                status: 200,
                message: "Found oauth_application by id".to_string(),
                data: Some(oauth_application),
            })
        }
        Err(err) => Json(err)
    }
}
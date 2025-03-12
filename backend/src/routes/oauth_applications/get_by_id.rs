use mongodb::bson::Uuid;
use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{
    auth::auth::AuthEntity,
    db::AuthRsDatabase,
    models::{
        http_response::HttpResponse,
        oauth_application::{OAuthApplication, OAuthApplicationMinimal},
    },
};

#[allow(unused)]
#[get("/oauth-applications/<id>", format = "json")]
pub async fn get_oauth_application_by_id(
    db: Connection<AuthRsDatabase>,
    req_entity: AuthEntity,
    id: &str,
) -> Json<HttpResponse<OAuthApplicationMinimal>> {
    if !req_entity.is_user() {
        return Json(HttpResponse {
            status: 403,
            message: "Forbidden".to_string(),
            data: None,
        });
    }

    let uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(err) => {
            return Json(HttpResponse {
                status: 400,
                message: format!("Invalid UUID: {:?}", err),
                data: None,
            })
        }
    };

    match OAuthApplication::get_by_id(uuid, &db).await {
        Ok(oauth_application) => Json(HttpResponse {
            status: 200,
            message: "Found oauth_application by id".to_string(),
            data: Some(oauth_application),
        }),
        Err(err) => Json(err),
    }
}

use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{
    auth::auth::AuthEntity,
    db::AuthRsDatabase,
    models::{
        http_response::HttpResponse,
        oauth_application::{OAuthApplication, OAuthApplicationMinimal},
    },
    utils::parse_uuid,
};

#[allow(unused)]
#[get("/oauth-applications/<id>", format = "json")]
pub async fn get_oauth_application_by_id(
    db: Connection<AuthRsDatabase>,
    req_entity: AuthEntity,
    id: &str,
) -> Json<HttpResponse<OAuthApplicationMinimal>> {
    if !req_entity.is_user() {
        return Json(HttpResponse::forbidden("Forbidden"));
    }

    let uuid = match parse_uuid(id) {
        Ok(uuid) => uuid,
        Err(err) => return Json(HttpResponse::from(err)),
    };

    match OAuthApplication::get_by_id(uuid, &db).await {
        Ok(oauth_application) => Json(HttpResponse::success("Found oauth_application by id", oauth_application)),
        Err(err) => Json(err),
    }
}

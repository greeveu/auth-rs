use mongodb::bson::doc;
use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{
    auth::auth::AuthEntity,
    db::AuthRsDatabase,
    models::{
        http_response::HttpResponse,
        oauth_application::{OAuthApplication, OAuthApplicationMinimal},
        oauth_scope::{OAuthScope, ScopeActions},
    },
};

#[allow(unused)]
#[get("/oauth-applications", format = "json")]
pub async fn get_all_oauth_applications(
    db: Connection<AuthRsDatabase>,
    req_entity: AuthEntity,
) -> Json<HttpResponse<Vec<OAuthApplicationMinimal>>> {
    if req_entity.is_token()
        && (!req_entity
            .token
            .clone()
            .unwrap()
            .check_scope(OAuthScope::OAuthApplications(ScopeActions::Read))
            && !req_entity
                .token
                .unwrap()
                .check_scope(OAuthScope::OAuthApplications(ScopeActions::All)))
    {
        return Json(HttpResponse::forbidden("Forbidden"));
    }

    let filter = match req_entity.user.unwrap().is_admin() {
        true => None,
        false => Some(doc! {
            "owner": req_entity.user_id
        }),
    };

    let applications = match OAuthApplication::get_all(&db, filter).await {
        Ok(oauth_applications) => oauth_applications,
        Err(err) => return Json(err),
    };

    Json(HttpResponse::success("Successfully retrieved your oauth applications", applications))
}

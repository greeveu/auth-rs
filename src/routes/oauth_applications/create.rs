use rocket::{error, post, serde::{json::Json, Deserialize}};
use rocket_db_pools::Connection;

use crate::{db::AuthRsDatabase, models::{audit_log::{AuditLog, AuditLogAction, AuditLogEntityType}, http_response::HttpResponse, oauth_application::OAuthApplication, user::User}};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateOAuthApplicationData {
    name: String,
    description: Option<String>,
    redirect_uris: Vec<String>,
}

#[allow(unused)]
#[post("/oauth-applications", format = "json", data = "<data>")] 
pub async fn create_oauth_application(db: Connection<AuthRsDatabase>, req_user: User, data: Json<CreateOAuthApplicationData>) -> Json<HttpResponse<OAuthApplication>> { 
    let data = data.into_inner();

    let oauth_application = match OAuthApplication::new(data.name, data.description, data.redirect_uris, req_user.id) {
        Ok(oauth_application) => oauth_application,
        Err(err) => return Json(err)
    };
    
    match oauth_application.insert(&db).await {
        Ok(oauth_application) => {
            match AuditLog::new(oauth_application.id, AuditLogEntityType::OAuthApplication, AuditLogAction::Create, "OAuth Application created.".to_string(), req_user.id, None, None).insert(&db).await {
                Ok(_) => (),
                Err(err) => error!("{}", err)
            }

            Json(HttpResponse {
                status: 201,
                message: "OAuth Application created".to_string(),
                data: Some(oauth_application)
            })
        },
        Err(err) => Json(err)
    }
}
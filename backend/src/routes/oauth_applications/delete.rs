use mongodb::bson::Uuid;
use rocket::{delete, error, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{auth::auth::AuthEntity, db::AuthRsDatabase, models::{audit_log::{AuditLog, AuditLogAction, AuditLogEntityType}, http_response::HttpResponse, oauth_application::OAuthApplication}};

#[allow(unused)]
#[delete("/oauth-applications/<id>", format = "json")]
pub async fn delete_oauth_application(db: Connection<AuthRsDatabase>, req_entity: AuthEntity, id: &str) -> Json<HttpResponse<()>> {
    if !req_entity.is_user() {
        return Json(HttpResponse {
            status: 403,
            message: "Forbidden".to_string(),
            data: None
        });
    }
    
    let uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(err) => return Json(HttpResponse {
            status: 400,
            message: format!("Invalid UUID: {:?}", err),
            data: None
        })
    };

    let oauth_application = match OAuthApplication::get_full_by_id(uuid, &db).await {
        Ok(oauth_application) => oauth_application,
        Err(err) => return Json(HttpResponse {
            status: 404,
            message: format!("OAuth Application does not exist: {:?}", err),
            data: None
        })
    };

    if req_entity.user_id != oauth_application.owner && !req_entity.user.unwrap().is_admin() {
        return Json(HttpResponse {
            status: 403,
            message: "Missing permissions!".to_string(),
            data: None
        });
    }

    match oauth_application.delete(&db).await {
        Ok(oauth_application) => {
            match AuditLog::new(oauth_application.id, AuditLogEntityType::OAuthApplication, AuditLogAction::Delete, "OAuthApplication deleted.".to_string(), req_entity.user_id, None, None).insert(&db).await {
                Ok(_) => (),
                Err(err) => error!("{}", err)
            }

            Json(HttpResponse {
                status: 200,
                message: "OAuth Application deleted".to_string(),
                data: None,
            })
    },
        Err(err) => Json(err)
    }
}
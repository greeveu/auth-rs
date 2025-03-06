use std::collections::HashMap;
use mongodb::bson::Uuid;
use rocket::{error, patch, serde::{json::Json, Deserialize}};
use rocket_db_pools::Connection;

use crate::{auth::auth::AuthEntity, db::AuthRsDatabase, models::{audit_log::{AuditLog, AuditLogAction, AuditLogEntityType}, http_response::HttpResponse, oauth_application::{OAuthApplication, OAuthApplicationMinimal}}};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateOAuthApplicationData {
    name: Option<String>,
    description: Option<String>,
    redirect_uris: Option<Vec<String>>,
}

#[allow(unused)]
#[patch("/oauth-applications/<id>", format = "json", data = "<data>")] 
pub async fn update_oauth_application(db: Connection<AuthRsDatabase>, req_entity: AuthEntity, id: &str, data: Json<UpdateOAuthApplicationData>) -> Json<HttpResponse<OAuthApplicationMinimal>> { 
    let data = data.into_inner();

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

    let old_oauth_application = match OAuthApplication::get_by_id(uuid, &db).await {
        Ok(oauth_application) => oauth_application,
        Err(err) => return Json(err)
    };

    if req_entity.user_id != old_oauth_application.owner && !req_entity.user.unwrap().is_admin() {
        return Json(HttpResponse {
            status: 403,
            message: "Missing permissions!".to_string(),
            data: None
        });
    }

    let mut new_oauth_application = match old_oauth_application.clone().to_full(&db).await {
        Ok(oauth_application) => oauth_application,
        Err(err) => return Json(err)
    };

    let mut old_values: HashMap<String, String> = HashMap::new();
    let mut new_values: HashMap<String, String> = HashMap::new();

    if data.name.is_some() {
        new_oauth_application.name = data.name.unwrap();
        old_values.insert("name".to_string(), old_oauth_application.name.clone());
        new_values.insert("name".to_string(), new_oauth_application.name.clone());
    }
    if data.description.is_some() {
        new_oauth_application.description = match data.description.clone().unwrap().is_empty() {
            true => None,
            false => Some(data.description.unwrap())
        };
        old_values.insert("description".to_string(), old_oauth_application.description.clone().unwrap_or("None".to_string()));
        new_values.insert("description".to_string(), new_oauth_application.description.clone().unwrap_or("None".to_string()));
    }
    if data.redirect_uris.is_some() {
        new_oauth_application.redirect_uris = data.redirect_uris.unwrap();
        old_values.insert("redirect_uris".to_string(), old_oauth_application.redirect_uris.clone().join(", "));
        new_values.insert("redirect_uris".to_string(), new_oauth_application.redirect_uris.clone().join(", "));
    }

    if new_values.is_empty() {
        return Json(HttpResponse {
            status: 200,
            message: "No updates applied.".to_string(),
            data: Some(new_oauth_application.to_minimal())
        });
    }

    match new_oauth_application.update(&db).await {
        Ok(oauth_application) => {
            match AuditLog::new(oauth_application.id, AuditLogEntityType::OAuthApplication, AuditLogAction::Update, "OAuthApplication updated.".to_string(), req_entity.user_id, Some(old_values), Some(new_values)).insert(&db).await {
                Ok(_) => (),
                Err(err) => error!("{}", err)
            }
            
            Json(HttpResponse {
                status: 200,
                message: "OAuth Application updated".to_string(),
                data: Some(oauth_application)
            })
        },
        Err(err) => Json(err)
    }
}
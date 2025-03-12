use mongodb::bson::Uuid;
use rocket::{
    error, patch,
    serde::{json::Json, Deserialize},
};
use rocket_db_pools::Connection;
use std::collections::HashMap;

use crate::{
    auth::auth::AuthEntity,
    db::AuthRsDatabase,
    models::{
        audit_log::{AuditLog, AuditLogAction, AuditLogEntityType},
        http_response::HttpResponse,
        role::Role,
    },
};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateRoleData {
    name: Option<String>,
}

#[allow(unused)]
#[patch("/roles/<id>", format = "json", data = "<data>")]
pub async fn update_role(
    db: Connection<AuthRsDatabase>,
    req_entity: AuthEntity,
    id: &str,
    data: Json<UpdateRoleData>,
) -> Json<HttpResponse<Role>> {
    let data = data.into_inner();

    if !req_entity.is_user() {
        return Json(HttpResponse {
            status: 403,
            message: "Forbidden".to_string(),
            data: None,
        });
    }

    if !req_entity.user.unwrap().is_admin() {
        return Json(HttpResponse {
            status: 403,
            message: "Missing permissions!".to_string(),
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

    let old_role = match Role::get_by_id(uuid, &db).await {
        Ok(role) => role,
        Err(err) => return Json(err),
    };

    // Prevent modification of system roles
    if old_role.system {
        return Json(HttpResponse {
            status: 403,
            message: "Cannot modify system role".to_string(),
            data: None,
        });
    }

    let mut new_role = old_role.clone();

    let mut old_values: HashMap<String, String> = HashMap::new();
    let mut new_values: HashMap<String, String> = HashMap::new();

    if data.name.is_some() && old_role.name != data.name.clone().unwrap() {
        new_role.name = data.name.unwrap();
        old_values.insert("name".to_string(), old_role.name.clone());
        new_values.insert("name".to_string(), new_role.name.clone());
    }

    if new_values.is_empty() {
        return Json(HttpResponse {
            status: 200,
            message: "No updates applied.".to_string(),
            data: Some(new_role),
        });
    }

    match new_role.update(&db).await {
        Ok(role) => {
            match AuditLog::new(
                role.id,
                AuditLogEntityType::Role,
                AuditLogAction::Update,
                "Role updated.".to_string(),
                req_entity.user_id,
                Some(old_values),
                Some(new_values),
            )
            .insert(&db)
            .await
            {
                Ok(_) => (),
                Err(err) => error!("{}", err),
            }

            Json(HttpResponse {
                status: 200,
                message: "Role updated".to_string(),
                data: Some(role),
            })
        }
        Err(err) => Json(err),
    }
}

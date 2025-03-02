use std::collections::HashMap;
use mongodb::bson::Uuid;
use pwhash::bcrypt;
use rocket::{error, patch, serde::{json::Json, Deserialize}};
use rocket_db_pools::Connection;

use crate::{auth::auth::AuthEntity, db::AuthRsDatabase, models::{audit_log::{AuditLog, AuditLogAction, AuditLogEntityType}, http_response::HttpResponse, role::Role, user::{User, UserMinimal}}, DEFAULT_ROLE_ID};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")] 
pub struct UpdateUserData {
    email: Option<String>,
    password: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    roles: Option<Vec<Uuid>>,
    disabled: Option<bool>,
}

#[allow(unused)]
#[patch("/users/<id>", format = "json", data = "<data>")] 
pub async fn update_user(db: Connection<AuthRsDatabase>, req_entity: AuthEntity, id: &str, data: Json<UpdateUserData>) -> Json<HttpResponse<UserMinimal>> { 
    let data = data.into_inner();

    if !req_entity.is_user() {
        return Json(HttpResponse {
            status: 403,
            message: "Missing permissions!".to_string(),
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

    if req_entity.user_id != uuid && !req_entity.user.clone().unwrap().is_system_admin() {
        return Json(HttpResponse {
            status: 403,
            message: "Missing permissions!".to_string(),
            data: None
        });
    }

    let old_user = match User::get_by_id(uuid, &db).await {
        Ok(user) => user,
        Err(err) => return Json(err)
    };

    let mut new_user = match old_user.clone().to_full(&db).await {
        Ok(user) => user,
        Err(err) => return Json(err)
    };

    let mut old_values: HashMap<String, String> = HashMap::new();
    let mut new_values: HashMap<String, String> = HashMap::new();

    // TODO: Add password / 2fa validation
    if data.email.is_some() {
        new_user.email = data.email.unwrap();
        old_values.insert("email".to_string(), old_user.email.clone());
        new_values.insert("email".to_string(), new_user.email.clone());
    }
    // TODO: Add password / 2fa validation
    if data.password.is_some() {
        let password_hash = match bcrypt::hash(data.password.unwrap()) {
            Ok(hash) => hash,
            Err(err) => return Json(HttpResponse {
                status: 500,
                message: format!("Failed to hash password: {:?}", err),
                data: None
            })
        };
        
        new_user.password_hash = password_hash;
        old_values.insert("password".to_string(), "HIDDEN".to_string());
        new_values.insert("password".to_string(), "HIDDEN".to_string());
    }
    if data.first_name.is_some() {
        new_user.first_name = data.first_name.unwrap();
        old_values.insert("firstName".to_string(), old_user.first_name.clone());
        new_values.insert("firstName".to_string(), new_user.first_name.clone());
    }
    if data.last_name.is_some() {
        new_user.last_name = data.last_name.unwrap();
        old_values.insert("lastName".to_string(), old_user.last_name.clone());
        new_values.insert("lastName".to_string(), new_user.last_name.clone());
    }
    if data.roles.is_some() && req_entity.user.clone().unwrap().is_system_admin() {
        new_user.roles = data.roles.unwrap();

        let available_roles = match Role::get_all(&db, None).await {
            Ok(roles) => roles,
            Err(err) => return Json(HttpResponse {
                status: 500,
                message: err.message,
                data: None
            })
        };

        for role_id in new_user.roles.iter() {
            if !available_roles.iter().any(|r| r.id == *role_id) {
                return Json(HttpResponse {
                    status: 400,
                    message: format!("Role with ID {:?} does not exist.", role_id),
                    data: None
                });
            }
        }

        if !new_user.roles.contains(&DEFAULT_ROLE_ID) {
            new_user.roles.push(*DEFAULT_ROLE_ID);
        }
        old_values.insert("roles".to_string(), old_user.roles.iter().map(|r| r.to_string()).collect::<Vec<String>>().join(","));
        new_values.insert("roles".to_string(), new_user.roles.iter().map(|r| r.to_string()).collect::<Vec<String>>().join(","));
    }
    if data.disabled.is_some() && req_entity.user.unwrap().is_system_admin() {
        new_user.disabled = data.disabled.unwrap();
        old_values.insert("disabled".to_string(), old_user.disabled.to_string());
        new_values.insert("disabled".to_string(), new_user.disabled.to_string());
    }

    if new_values.is_empty() {
        return Json(HttpResponse {
            status: 200,
            message: "No updates applied.".to_string(),
            data: Some(new_user.to_minimal())
        });
    }

    match new_user.update(&db).await {
        Ok(user) => {
            match AuditLog::new(user.id, AuditLogEntityType::User, AuditLogAction::Update, "User updated.".to_string(), req_entity.user_id, Some(old_values), Some(new_values)).insert(&db).await {
                Ok(_) => (),
                Err(err) => error!("{}", err)
            }
            
            Json(HttpResponse {
                status: 200,
                message: "User updated".to_string(),
                data: Some(user)
            })
        },
        Err(err) => Json(err)
    }
}
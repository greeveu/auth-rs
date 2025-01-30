use mongodb::bson::Uuid;
use rocket::{delete, error, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{db::AuthRsDatabase, models::{audit_log::{AuditLog, AuditLogAction, AuditLogEntityType}, http_response::HttpResponse, role::Role, user::User}};

#[allow(unused)]
#[delete("/roles/<id>", format = "json")]
pub async fn delete_role(db: Connection<AuthRsDatabase>, req_user: User, id: &str) -> Json<HttpResponse<()>> {
    if !req_user.is_global_admin() {
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

    let role = match Role::get_by_id(uuid, &db).await {
        Ok(tenant) => tenant,
        Err(err) => return Json(HttpResponse {
            status: 404,
            message: format!("Role does not exist: {:?}", err),
            data: None
        })
    };


    match role.delete(&db).await {
        Ok(role) => {
            // TODO: Implement author_id -> maybe admin action
            match AuditLog::new(role.id, AuditLogEntityType::Role, AuditLogAction::Delete, "Role deleted.".to_string(), req_user.id, None, None).insert(&db).await {
                Ok(_) => (),
                Err(err) => error!("{}", err)
            }

            Json(HttpResponse {
                status: 200,
                message: "Role deleted".to_string(),
                data: None,
            })
    },
        Err(err) => Json(err)
    }
}
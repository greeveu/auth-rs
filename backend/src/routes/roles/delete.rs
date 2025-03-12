use mongodb::bson::Uuid;
use rocket::{delete, error, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{
    auth::auth::AuthEntity,
    db::AuthRsDatabase,
    models::{
        audit_log::{AuditLog, AuditLogAction, AuditLogEntityType},
        http_response::HttpResponse,
        role::Role,
    },
};

#[allow(unused)]
#[delete("/roles/<id>", format = "json")]
pub async fn delete_role(
    db: Connection<AuthRsDatabase>,
    req_entity: AuthEntity,
    id: &str,
) -> Json<HttpResponse<()>> {
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

    let role = match Role::get_by_id(uuid, &db).await {
        Ok(role) => role,
        Err(err) => {
            return Json(HttpResponse {
                status: 404,
                message: format!("Role does not exist: {:?}", err),
                data: None,
            })
        }
    };

    if role.system {
        return Json(HttpResponse {
            status: 400,
            message: "Cannot delete system role".to_string(),
            data: None,
        });
    }

    match role.delete(&db).await {
        Ok(role) => {
            match AuditLog::new(
                role.id,
                AuditLogEntityType::Role,
                AuditLogAction::Delete,
                "Role deleted.".to_string(),
                req_entity.user_id,
                None,
                None,
            )
            .insert(&db)
            .await
            {
                Ok(_) => (),
                Err(err) => error!("{}", err),
            }

            Json(HttpResponse {
                status: 200,
                message: "Role deleted".to_string(),
                data: None,
            })
        }
        Err(err) => Json(err),
    }
}

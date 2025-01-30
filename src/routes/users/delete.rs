use mongodb::bson::Uuid;
use rocket::{delete, error, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{db::AuthRsDatabase, models::{audit_log::{AuditLog, AuditLogAction, AuditLogEntityType}, http_response::HttpResponse, user::User}};

#[allow(unused)]
#[delete("/users/<id>", format = "json")]
pub async fn delete_user(db: Connection<AuthRsDatabase>, req_user: User, id: &str) -> Json<HttpResponse<()>> {
    let uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(err) => return Json(HttpResponse {
            status: 400,
            message: format!("Invalid UUID: {:?}", err),
            data: None
        })
    };

    if req_user.id != uuid && !req_user.is_global_admin() {
        return Json(HttpResponse {
            status: 403,
            message: "Missing permissions!".to_string(),
            data: None
        });
    }

    let user = match User::get_full_by_id(uuid, &db).await {
        Ok(user) => user,
        Err(err) => return Json(HttpResponse {
            status: 404,
            message: format!("User does not exist: {:?}", err),
            data: None
        })
    };


    match user.delete(&db).await {
        Ok(user) => {
            match AuditLog::new(user.id, AuditLogEntityType::User, AuditLogAction::Delete, "User deleted.".to_string(), req_user.id, None, None).insert(&db).await {
                Ok(_) => (),
                Err(err) => error!("{}", err)
            }

            Json(HttpResponse {
                status: 200,
                message: "User deleted".to_string(),
                data: None,
            })
    },
        Err(err) => Json(err)
    }
}
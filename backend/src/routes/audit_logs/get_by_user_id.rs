use rocket::{get, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{
    auth::auth::AuthEntity,
    db::AuthRsDatabase,
    models::{
        audit_log::AuditLog,
        http_response::HttpResponse,
        oauth_scope::{OAuthScope, ScopeActions},
    },
    utils::parse_uuid,
};

#[allow(unused)]
#[get("/users/<id>/audit-logs", format = "json")]
pub async fn get_audit_logs_by_user_id(
    db: Connection<AuthRsDatabase>,
    req_entity: AuthEntity,
    id: &str,
) -> Json<HttpResponse<Vec<AuditLog>>> {
    let user_uuid = match parse_uuid(id) {
        Ok(uuid) => uuid,
        Err(err) => return Json(HttpResponse::from(err)),
    };

    if !req_entity.is_user()
        && !req_entity
            .token
            .unwrap()
            .check_scope(OAuthScope::AuditLogs(ScopeActions::Read))
        || (req_entity.user.clone().unwrap().id != user_uuid
            && !req_entity.user.unwrap().is_admin())
    {
        return Json(HttpResponse {
            status: 403,
            message: "Missing permissions!".to_string(),
            data: None,
        });
    }

    match AuditLog::get_by_user_id(user_uuid, &db).await {
        Ok(audit_logs) => Json(HttpResponse {
            status: 200,
            message: "Audit Logs found by user id".to_string(),
            data: Some(audit_logs),
        }),
        Err(err) => Json(err),
    }
}

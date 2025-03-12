use mongodb::bson::Uuid;
use rocket::{delete, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{
    auth::auth::AuthEntity,
    db::AuthRsDatabase,
    models::{
        http_response::HttpResponse,
        oauth_application::OAuthApplication,
        oauth_scope::{OAuthScope, ScopeActions},
        oauth_token::OAuthToken,
    },
};

#[allow(unused)]
#[delete("/connections/<id>")]
pub async fn disconnect(
    db: Connection<AuthRsDatabase>,
    req_entity: AuthEntity,
    id: &str,
) -> Json<HttpResponse<()>> {
    if req_entity.is_token()
        && (!req_entity
            .token
            .clone()
            .unwrap()
            .check_scope(OAuthScope::Connections(ScopeActions::Delete))
            || req_entity
                .token
                .clone()
                .unwrap()
                .check_scope(OAuthScope::Connections(ScopeActions::All)))
    {
        return Json(HttpResponse {
            status: 403,
            message: "Forbidden".to_string(),
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

    if (req_entity.is_user()
        && req_entity.user_id != uuid
        && !req_entity.user.clone().unwrap().is_admin())
    {
        return Json(HttpResponse {
            status: 403,
            message: "Missing permissions!".to_string(),
            data: None,
        });
    }

    let oauth_application = match OAuthApplication::get_by_id(uuid, &db).await {
        Ok(application) => application,
        Err(err) => {
            return Json(HttpResponse {
                status: err.status,
                message: err.message,
                data: None,
            })
        }
    };

    let tokens = match OAuthToken::get_by_application_id(oauth_application.id, &db).await {
        Ok(tokens) => tokens,
        Err(err) => {
            return Json(HttpResponse {
                status: 500,
                message: err.message,
                data: None,
            })
        }
    };

    if tokens.is_empty() {
        return Json(HttpResponse {
            status: 404,
            message: "You are not connected to that application".to_string(),
            data: None,
        });
    }

    for token in tokens {
        match token.delete(&db).await {
            Ok(_) => (),
            Err(err) => {
                return Json(HttpResponse {
                    status: 500,
                    message: err.message,
                    data: None,
                })
            }
        }
    }

    Json(HttpResponse {
        status: 200,
        message: "Successfully disconnected from application".to_string(),
        data: None,
    })
}

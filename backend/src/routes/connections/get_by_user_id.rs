use mongodb::bson::{doc, Uuid};
use rocket_db_pools::Connection;
use rocket::{get, serde::json::Json};
use crate::{auth::auth::AuthEntity, db::AuthRsDatabase, models::{http_response::HttpResponse, oauth_application::{OAuthApplication, OAuthApplicationMinimal}, oauth_scope::{OAuthScope, ScopeActions}, oauth_token::OAuthToken}};

#[allow(unused)]
#[get("/users/<id>/connections", format = "json")] 
pub async fn get_by_user_id(db: Connection<AuthRsDatabase>, req_entity: AuthEntity, id: &str) -> Json<HttpResponse<Vec<OAuthApplicationMinimal>>> {
    if req_entity.is_token() && (!req_entity.token.clone().unwrap().check_scope(OAuthScope::Connections(ScopeActions::Read)) || req_entity.token.clone().unwrap().check_scope(OAuthScope::Connections(ScopeActions::All))) {
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

    if (req_entity.is_user() && req_entity.user_id != uuid && !req_entity.user.clone().unwrap().is_system_admin()) || req_entity.is_token() && req_entity.user_id != uuid {
        return Json(HttpResponse {
            status: 403,
            message: "Missing permissions!".to_string(),
            data: None
        });
    }

    let connected_applications = match OAuthToken::get_by_user_id(uuid, &db).await {
        Ok(tokens) => tokens,
        Err(err) => {
            return Json(HttpResponse {
                status: 500,
                message: err.message,
                data: None
            });
        }
    }.iter().map(|token| token.clone().application_id).collect::<Vec<Uuid>>();

    let filter = doc! {
        "_id": {
            "$in": connected_applications
        }
    };

    let applications = match OAuthApplication::get_all(&db, Some(filter)).await {
        Ok(applications) => applications,
        Err(err) => {
            return Json(HttpResponse {
                status: 500,
                message: err.message,
                data: None
            });
        }
    };


    Json(HttpResponse {
        status: 200,
        message: "Found connections by user id".to_string(),
        data: Some(applications),
    })
}
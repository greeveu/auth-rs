use mongodb::bson::Uuid;
use rocket::{post, serde::{json::Json, Deserialize}};
use rocket_db_pools::Connection;

use crate::{auth::{auth::AuthEntity, mfa::MfaHandler}, db::AuthRsDatabase, models::{http_response::HttpResponse, user::{User, UserMinimal}}, routes::auth::login::LoginResponse};


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct EnableMfaData {
    pub password: String
}

#[allow(unused)]
#[post("/users/<id>/mfa/totp/enable", format = "json", data= "<data>")]
pub async fn enable_totp_mfa(db: Connection<AuthRsDatabase>, req_entity: AuthEntity, id: &str, data: Json<EnableMfaData>) -> Json<HttpResponse<LoginResponse>> {
    let mfa_data = data.into_inner();
    
    if req_entity.is_token() {
        return Json(HttpResponse {
            status: 403,
            message: "Forbidden!".to_string(),
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

    if req_entity.is_user() && req_entity.user_id != uuid && !req_entity.user.clone().unwrap().is_system_admin() {
        return Json(HttpResponse {
            status: 403,
            message: "Missing permissions!".to_string(),
            data: None
        });
    }

    let user = match User::get_by_id(uuid, &db).await {
        Ok(user) => user.to_full(&db).await.unwrap(),
        Err(err) => return Json(HttpResponse {
            status: 500,
            message: format!("Failed to get user: {:?}", err),
            data: None
        })
    };

    if !user.verify_password(&mfa_data.password) {
        return Json(HttpResponse {
            status: 401,
            message: "Incorrect password!".to_string(),
            data: None
        });
    }

    if user.totp_secret.is_some() {
        return Json(HttpResponse {
            status: 400,
            message: "TOTP MFA is already enabled!".to_string(),
            data: None
        });
    }

    let mut flow = MfaHandler::start_enable_flow(&user).await.unwrap();

    Json(HttpResponse {
        status: 200,
        message: "TOTP MFA enable flow started.".to_string(),
        data: Some(LoginResponse {
            user: Some(user.to_minimal()),
            token: Some(flow.totp.unwrap().get_qr_base64().unwrap()),
            mfa_required: true,
            mfa_flow_id: Some(flow.flow_id)
        })
    })
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct DisableMfaData {
    pub code: Option<String>,
    pub password: Option<String>
}

#[allow(unused)]
#[post("/users/<id>/mfa/totp/disable", format = "json", data= "<data>")] 
pub async fn disable_totp_mfa(db: Connection<AuthRsDatabase>, req_entity: AuthEntity, id: &str, data: Json<DisableMfaData>) -> Json<HttpResponse<UserMinimal>> {
    let mfa_data = data.into_inner();
    
    if req_entity.is_token() {
        return Json(HttpResponse {
            status: 403,
            message: "Forbidden!".to_string(),
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

    if req_entity.is_user() && req_entity.user_id != uuid && !req_entity.user.clone().unwrap().is_system_admin() {
        return Json(HttpResponse {
            status: 403,
            message: "Missing permissions!".to_string(),
            data: None
        });
    }

    let mut user = match User::get_by_id(uuid, &db).await {
        Ok(user) => user.to_full(&db).await.unwrap(),
        Err(err) => return Json(HttpResponse {
            status: 500,
            message: format!("Failed to get user: {:?}", err),
            data: None
        })
    };

    if !user.totp_secret.is_some() {
        return Json(HttpResponse {
            status: 400,
            message: "TOTP MFA is not enabled!".to_string(),
            data: None
        });
    }

    if mfa_data.code.is_none() && mfa_data.password.is_none() {
        return Json(HttpResponse {
            status: 400,
            message: "Missing TOTP code or password!".to_string(),
            data: None
        });
    }

    if mfa_data.code.is_some() {
        match MfaHandler::verify_totp(&user, user.totp_secret.clone().unwrap(), &mfa_data.code.unwrap()).await {
            true => (),
            false => return Json(HttpResponse {
                status: 401,
                message: "Invalid TOTP code!".to_string(),
                data: None
            })
        };
    } else {
        match user.verify_password(&mfa_data.password.unwrap()) {
            true => (),
            false => return Json(HttpResponse {
                status: 401,
                message: "Incorrect password!".to_string(),
                data: None
            })
        }
    }

    match MfaHandler::disable_totp(&mut user, req_entity, &db).await {
        Ok(user) => Json(HttpResponse {
            status: 200,
            message: "TOTP MFA disabled.".to_string(),
            data: Some(user.to_minimal())
        }),
        Err(err) => Json(HttpResponse {
            status: 500,
            message: format!("Failed to disable TOTP: {:?}", err),
            data: None
        })
    }
}
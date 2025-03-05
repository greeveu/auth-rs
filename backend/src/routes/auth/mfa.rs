use std::collections::HashMap;

use mongodb::bson::Uuid;
use rocket::{post, serde::{json::Json, Serialize, Deserialize}};
use rocket_db_pools::Connection;
use totp_rs::TOTP;

use crate::{auth::mfa::{MfaState, MfaType}, db::AuthRsDatabase, models::{audit_log::{AuditLog, AuditLogAction, AuditLogEntityType}, http_response::HttpResponse}, MFA_SESSIONS};

use super::login::LoginResponse;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct MfaData {
    pub flow_id: Uuid,
    pub code: String
}

#[allow(unused)]
#[post("/auth/mfa", format = "json", data = "<data>")] 
pub async fn mfa(db: Connection<AuthRsDatabase>, data: Json<MfaData>) -> Json<HttpResponse<LoginResponse>> {
    let mfa_data = data.into_inner();

    let mfa_sessions = MFA_SESSIONS.lock().await;

    let cloned_sessions = mfa_sessions.clone();
    let flow = match cloned_sessions.get(&mfa_data.flow_id) {
        Some(flow) => flow,
        None => return Json(HttpResponse {
            status: 404,
            message: "Invalid or expired MFA flow".to_string(),
            data: None
        })
    };

    drop(mfa_sessions);

    if flow.state == MfaState::Complete {
        return Json(HttpResponse {
            status: 400,
            message: "MFA flow already complete".to_string(),
            data: None
        });
    }

    if flow.r#type == MfaType::TOTP || flow.r#type == MfaType::EnableTOTP {
        if !flow.verify_current_totp(&mfa_data.code).await {
            return Json(HttpResponse {
                status: 401,
                message: "Invalid TOTP code".to_string(),
                data: None
            })
        }

        if flow.r#type == MfaType::EnableTOTP && flow.totp.is_some() && flow.user.totp_secret.is_none() {
            let mut user = flow.user.clone();
            user.totp_secret = Some(flow.totp.clone().unwrap().get_secret_base32());
            match user.update(&db).await {
                Ok(_) => {
                    let new_values = HashMap::from([("totpSecret".to_string(), user.totp_secret.clone().unwrap_or("".to_string()))]);
                    let old_values = HashMap::from([("totpSecret".to_string(), "".to_string())]);

                    match AuditLog::new(user.id, AuditLogEntityType::User, AuditLogAction::Update, "Enable TOTP.".to_string(), user.id, Some(old_values), Some(new_values)).insert(&db).await {
                        Ok(_) => (),
                        Err(err) => eprintln!("{:?}", err)
                    };

                    Json(HttpResponse {
                        status: 200,
                        message: "TOTP enabled".to_string(),
                        data: Some(LoginResponse {
                            user: Some(user.to_minimal()),
                            token: Some(TOTP::get_qr_base64(&flow.totp.clone().unwrap()).unwrap()),
                            mfa_required: false,
                            mfa_flow_id: None
                        })
                    })
                },
                Err(err) => Json(HttpResponse {
                    status: 500,
                    message: format!("Failed to enable TOTP: {:?}", err),
                    data: None
                })
            }
        } else {
            Json(HttpResponse {
                status: 200,
                message: "MFA complete".to_string(),
                data: Some(LoginResponse {
                    user: Some(flow.user.clone().to_minimal()),
                    token: Some(flow.user.token.clone()),
                    mfa_required: false,
                    mfa_flow_id: None
                })
            })
        }
    } else {
        Json(HttpResponse {
            status: 400,
            message: "Invalid MFA type".to_string(),
            data: None
        })
    }
}
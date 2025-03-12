use mongodb::bson::Uuid;
use rocket::{
    post,
    serde::{json::Json, Deserialize, Serialize},
};
use rocket_db_pools::Connection;

use crate::{
    auth::mfa::MfaHandler,
    db::AuthRsDatabase,
    models::{
        http_response::HttpResponse,
        user::{User, UserMinimal},
    },
};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub user: Option<UserMinimal>,
    pub token: Option<String>,
    pub mfa_required: bool,
    pub mfa_flow_id: Option<Uuid>,
}

#[allow(unused)]
#[post("/auth/login", format = "json", data = "<data>")]
pub async fn login(
    db: Connection<AuthRsDatabase>,
    data: Json<LoginData>,
) -> Json<HttpResponse<LoginResponse>> {
    let login_data = data.into_inner();

    let user = match User::get_by_email(&login_data.email, &db).await {
        Ok(user) => user,
        Err(err) => {
            return Json(HttpResponse {
                status: 500,
                message: err.message,
                data: None,
            })
        }
    };

    if user.disabled {
        return Json(HttpResponse {
            status: 403,
            message: "User is disabled".to_string(),
            data: None,
        });
    }

    let user_full = match user.to_full(&db).await {
        Ok(user) => user,
        Err(err) => {
            return Json(HttpResponse {
                status: 500,
                message: format!("Failed to get full user: {:?}", err),
                data: None,
            })
        }
    };

    if !user_full.verify_password(&login_data.password) {
        return Json(HttpResponse {
            status: 401,
            message: "Invalid email or password".to_string(),
            data: None,
        });
    }

    if MfaHandler::is_mfa_required(&user_full) {
        let mfa_flow = MfaHandler::start_login_flow(&user_full).await.unwrap();

        return Json(HttpResponse {
            status: 401,
            message: "MFA required".to_string(),
            data: Some(LoginResponse {
                user: None,
                token: None,
                mfa_required: true,
                mfa_flow_id: Some(mfa_flow.flow_id),
            }),
        });
    }

    Json(HttpResponse {
        status: 200,
        message: "Login successful".to_string(),
        data: Some(LoginResponse {
            user: Some(user),
            token: Some(user_full.token),
            mfa_required: false,
            mfa_flow_id: None,
        }),
    })
}

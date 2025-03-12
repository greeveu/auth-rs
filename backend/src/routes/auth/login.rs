use mongodb::bson::Uuid;
use rocket::{
    post,
    serde::{json::Json, Deserialize, Serialize},
};
use rocket_db_pools::Connection;

use crate::{
    auth::mfa::MfaHandler,
    db::AuthRsDatabase,
    errors::{ApiError, ApiResult},
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

// Process login and return a Result
async fn process_login(
    db: &Connection<AuthRsDatabase>,
    login_data: LoginData,
) -> ApiResult<LoginResponse> {
    let user = User::get_by_email(&login_data.email, db)
        .await
        .map_err(|err| ApiError::InternalError(err.message))?;

    if user.disabled {
        return Err(ApiError::Forbidden("User is disabled".to_string()));
    }

    let user_full = user
        .to_full(db)
        .await
        .map_err(|err| ApiError::InternalError(format!("Failed to get full user: {:?}", err)))?;

    if !user_full.verify_password(&login_data.password) {
        return Err(ApiError::Unauthorized(
            "Invalid email or password".to_string(),
        ));
    }

    if MfaHandler::is_mfa_required(&user_full) {
        let mfa_flow = MfaHandler::start_login_flow(&user_full)
            .await
            .map_err(|err| ApiError::InternalError(format!("Failed to start MFA flow: {}", err)))?;

        return Ok(LoginResponse {
            user: None,
            token: None,
            mfa_required: true,
            mfa_flow_id: Some(mfa_flow.flow_id),
        });
    }

    Ok(LoginResponse {
        user: Some(user),
        token: Some(user_full.token),
        mfa_required: false,
        mfa_flow_id: None,
    })
}

#[allow(unused)]
#[post("/auth/login", format = "json", data = "<data>")]
pub async fn login(
    db: Connection<AuthRsDatabase>,
    data: Json<LoginData>,
) -> Json<HttpResponse<LoginResponse>> {
    let login_data = data.into_inner();
    
    match process_login(&db, login_data).await {
        Ok(response) => {
            let message = if response.mfa_required {
                "MFA required"
            } else {
                "Login successful"
            };
            Json(HttpResponse::success(message, response))
        }
        Err(err) => Json(HttpResponse::from(err)),
    }
}

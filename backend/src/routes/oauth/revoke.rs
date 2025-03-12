use rocket::{post, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{
    auth::auth::AuthEntity,
    db::AuthRsDatabase,
    models::{http_response::HttpResponse, oauth_token::OAuthToken},
};

#[allow(unused)]
#[post("/oauth/token/revoke", format = "json")]
pub async fn revoke_oauth_token(
    db: Connection<AuthRsDatabase>,
    req_entity: AuthEntity,
) -> Json<HttpResponse<()>> {
    if !req_entity.is_token() {
        return Json(HttpResponse {
            status: 403,
            message: "This endpoint is reserved for revoking OAuth tokens".to_string(),
            data: None,
        });
    }

    let oauth_token = match OAuthToken::get_by_token(
        &req_entity.token.unwrap().token,
        &db.database(OAuthToken::COLLECTION_NAME),
    )
    .await
    {
        Ok(token) => token,
        Err(err) => {
            return Json(HttpResponse {
                status: 404,
                message: format!("How tf did you get here?!?!?! : {:?}", err),
                data: None,
            })
        }
    };

    match oauth_token.delete(&db).await {
        Ok(_) => Json(HttpResponse {
            status: 200,
            message: "Token revoked".to_string(),
            data: None,
        }),
        Err(err) => Json(err),
    }
}

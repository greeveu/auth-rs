use rocket::{post, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{
    db::AuthRsDatabase,
    models::{http_response::HttpResponse, user::UserMinimal},
    routes::users::create::{create_user, CreateUserData},
};

/*
    I am honestly not quiet sure why I am making this a thing but it felt intuitive soooooooo.....
*/

#[allow(unused)]
#[post("/auth/register", format = "json", data = "<data>")]
pub async fn register(
    db: Connection<AuthRsDatabase>,
    data: Json<CreateUserData>,
) -> Json<HttpResponse<UserMinimal>> {
    create_user(db, data).await
}

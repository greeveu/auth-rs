use rocket::{http::Status, outcome::Outcome, request::FromRequest, Request};
use crate::{db::{get_main_db_name, AuthRsDatabase}, models::user::User};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<User, (Status, ()), Status> {
        let db = match request.guard::<&AuthRsDatabase>().await {
            Outcome::Success(db) => db.database(get_main_db_name()),
            _ => return Outcome::Error((Status::InternalServerError, ()))
        };
        
        let auth_header = request.headers().get_one("Authorization");

        match auth_header {
            Some(token) => {
                if !token.starts_with("Bearer ") {
                    return Outcome::Forward(Status::Unauthorized);
                }

                match User::get_full_by_token(token.replace("Bearer ", "").to_owned(), db).await {
                    Ok(user) => Outcome::Success(user),
                    Err(_) => Outcome::Forward(Status::Unauthorized)
                }
            },
            None => Outcome::Forward(Status::Unauthorized)
        }
    }
}
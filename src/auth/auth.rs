use mongodb::bson::Uuid;
use rocket::{http::Status, outcome::Outcome, request::FromRequest, Request};
use crate::{db::{get_main_db_name, AuthRsDatabase}, models::{oauth_token::OAuthToken, user::User}};

#[derive(Debug, Clone)]
pub struct AuthEntity {
    pub user_id: Uuid,
    pub user: Option<User>,
    pub token: Option<OAuthToken>
}

impl AuthEntity {
    pub fn from_user(user: User) -> Self {
        Self {
            user_id: user.id,
            user: Some(user),
            token: None
        }
    }

    pub fn from_token(token: OAuthToken) -> Self {
        Self {
            user_id: token.user_id,
            user: None,
            token: Some(token)
        }
    }

    pub fn is_user(&self) -> bool {
        self.user.is_some()
    }

    pub fn is_token(&self) -> bool {
        self.token.is_some()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthEntity {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<AuthEntity, (Status, ()), Status> {
        let db = match request.guard::<&AuthRsDatabase>().await {
            Outcome::Success(db) => db.database(get_main_db_name()),
            _ => return Outcome::Error((Status::InternalServerError, ()))
        };
        
        let auth_header = request.headers().get_one("Authorization");

        match auth_header {
            Some(token) => {
                if token.split(" ").count() != 2 {
                    return Outcome::Forward(Status::Unauthorized)
                }
                match token.split(" ").nth(0) {
                    Some("Bearer") => match User::get_full_by_token(token.split(" ").nth(1).unwrap().to_owned(), &db).await {
                        Ok(user) => Outcome::Success(AuthEntity::from_user(user)),
                        Err(_) => match OAuthToken::get_by_token(token.split(" ").nth(1).unwrap(), &db).await {
                            Ok(token) => Outcome::Success(AuthEntity::from_token(token)),
                            Err(_) => Outcome::Forward(Status::Unauthorized)
                        }
                    },
                    _ => return Outcome::Forward(Status::Unauthorized)
                    
                }
            },
            None => Outcome::Forward(Status::Unauthorized)
        }
    }
}
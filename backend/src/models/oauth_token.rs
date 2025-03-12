use super::{http_response::HttpResponse, oauth_scope::OAuthScope};
use crate::db::{get_main_db, AuthRsDatabase};
use anyhow::Result;
use mongodb::bson::{doc, DateTime, Uuid};
use rand::Rng;
use rocket::{
    futures::StreamExt,
    serde::{Deserialize, Serialize},
};
use rocket_db_pools::{
    mongodb::{Collection, Database},
    Connection,
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct OAuthToken {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub application_id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub scope: Vec<OAuthScope>,
    pub expires_in: u64,
    pub created_at: DateTime,
}

impl OAuthToken {
    pub const COLLECTION_NAME: &'static str = "oauth-tokens";

    fn generate_token() -> String {
        let mut rng = rand::rng();
        let token: String = (0..128)
            .map(|_| {
                let idx = rng.random_range(0..62);
                match idx {
                    0..=9 => (b'0' + idx as u8) as char,
                    10..=35 => (b'a' + (idx - 10) as u8) as char,
                    _ => (b'A' + (idx - 36) as u8) as char,
                }
            })
            .collect();
        token
    }

    /// Checks if the token is expired
    pub fn is_expired(&self) -> bool {
        let created_at = self.created_at.timestamp_millis() as u64;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_millis() as u64;

        // Convert expires_in from seconds to milliseconds
        let expires_in_ms = self.expires_in * 1000;

        // Check if the token has expired
        now > created_at + expires_in_ms
    }

    pub fn new(
        application_id: Uuid,
        user_id: Uuid,
        scope: Vec<OAuthScope>,
        expires_in: u64,
    ) -> Result<Self, HttpResponse<OAuthToken>> {
        Ok(Self {
            id: Uuid::new(),
            application_id,
            user_id,
            token: Self::generate_token(),
            scope,
            expires_in,
            created_at: DateTime::now(),
        })
    }

    #[allow(unused)]
    pub async fn insert(
        &self,
        connection: &Connection<AuthRsDatabase>,
    ) -> Result<OAuthToken, HttpResponse<OAuthToken>> {
        let db = Self::get_collection(connection);

        match db.insert_one(self.clone(), None).await {
            Ok(_) => Ok(self.clone()),
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error inserting oauth token: {:?}", err),
                data: None,
            }),
        }
    }

    #[allow(unused)]
    pub fn check_scope(&self, scope: OAuthScope) -> bool {
        self.scope.contains(&scope)
    }

    #[allow(unused)]
    pub async fn get_by_token(
        token: &str,
        mut db: &Database,
    ) -> Result<OAuthToken, HttpResponse<OAuthToken>> {
        let db: Collection<OAuthToken> = db.collection(Self::COLLECTION_NAME);

        let filter = doc! {
            "token": token
        };
        match db.find_one(filter, None).await.unwrap() {
            Some(token) => {
                if (token.created_at.timestamp_millis() + token.expires_in as i64)
                    < DateTime::now().timestamp_millis()
                {
                    Err(HttpResponse {
                        status: 401,
                        message: "Token expired".to_string(),
                        data: None,
                    })
                } else {
                    Ok(token)
                }
            }
            None => Err(HttpResponse {
                status: 404,
                message: "Token not found".to_string(),
                data: None,
            }),
        }
    }

    #[allow(unused)]
    pub async fn get_by_application_id(
        application_id: Uuid,
        connection: &Connection<AuthRsDatabase>,
    ) -> Result<Vec<OAuthToken>, HttpResponse<Vec<OAuthToken>>> {
        let db = Self::get_collection(connection);

        let filter = doc! {
            "applicationId": application_id
        };
        match db.find(filter, None).await {
            Ok(cursor) => {
                let tokens = cursor
                    .map(|doc| {
                        let token: OAuthToken = doc.unwrap();
                        token
                    })
                    .collect::<Vec<OAuthToken>>()
                    .await;
                Ok(tokens)
            }
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error fetching tokens: {:?}", err),
                data: None,
            }),
        }
    }

    #[allow(unused)]
    pub async fn get_by_user_id(
        user_id: Uuid,
        connection: &Connection<AuthRsDatabase>,
    ) -> Result<Vec<OAuthToken>, HttpResponse<Vec<OAuthToken>>> {
        let db = Self::get_collection(connection);

        let filter = doc! {
            "userId": user_id
        };
        match db.find(filter, None).await {
            Ok(cursor) => {
                let tokens = cursor
                    .map(|doc| {
                        let token: OAuthToken = doc.unwrap();
                        token
                    })
                    .collect::<Vec<OAuthToken>>()
                    .await;
                Ok(tokens)
            }
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error fetching tokens: {:?}", err),
                data: None,
            }),
        }
    }

    #[allow(unused)]
    pub async fn get_by_user_and_application_id(
        user_id: Uuid,
        application_id: Uuid,
        connection: &Connection<AuthRsDatabase>,
    ) -> Result<Vec<OAuthToken>, HttpResponse<Vec<OAuthToken>>> {
        let db = Self::get_collection(connection);

        let filter = doc! {
            "userId": user_id,
            "applicationId": application_id
        };
        match db.find(filter, None).await {
            Ok(cursor) => {
                let tokens = cursor
                    .map(|doc| {
                        let token: OAuthToken = doc.unwrap();
                        return token;
                    })
                    .collect::<Vec<OAuthToken>>()
                    .await;
                Ok(tokens)
            }
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error fetching tokens: {:?}", err),
                data: None,
            }),
        }
    }

    #[allow(unused)]
    pub async fn reauthenticate(
        &mut self,
        scope: Vec<OAuthScope>,
        connection: &Connection<AuthRsDatabase>,
    ) -> Result<OAuthToken, HttpResponse<OAuthToken>> {
        let db = Self::get_collection(connection);

        let filter = doc! {
            "_id": self.id
        };

        self.scope = scope;
        self.expires_in = 30 * 24 * 60 * 60;

        match db.replace_one(filter, self.clone(), None).await {
            Ok(_) => Ok(self.clone()),
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error reauthenticating token: {:?}", err),
                data: None,
            }),
        }
    }

    #[allow(unused)]
    pub async fn delete(
        &self,
        connection: &Connection<AuthRsDatabase>,
    ) -> Result<OAuthToken, HttpResponse<()>> {
        let db = Self::get_collection(connection);

        let filter = doc! {
            "_id": self.id
        };
        match db.delete_one(filter, None).await {
            Ok(_) => Ok(self.clone()),
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error deleting oauth token: {:?}", err),
                data: None,
            }),
        }
    }

    #[allow(unused)]
    fn get_collection(connection: &Connection<AuthRsDatabase>) -> Collection<Self> {
        let db = get_main_db(connection);
        db.collection(Self::COLLECTION_NAME)
    }
}

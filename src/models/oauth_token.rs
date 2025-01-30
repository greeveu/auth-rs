use anyhow::Result;
use mongodb::bson::{doc, DateTime, Uuid};
use rand::Rng;
use rocket_db_pools::{mongodb::Collection, Connection};
use rocket::{futures::StreamExt, serde::{Deserialize, Serialize}};
use crate::db::{get_main_db, AuthRsDatabase};

use super::http_response::HttpResponse;

#[derive(Debug, Clone, Serialize, Deserialize)] 
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")] 
pub struct OAuthToken {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub application_id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub scope: Vec<String>,
    pub expires_in: u64,
    pub created_at: String,
}

impl OAuthToken {
    pub const COLLECTION_NAME: &'static str = "oauth-tokens";

    fn generate_token() -> String {
        rand::rng().sample_iter(rand::distr::Alphanumeric).take(128).map(char::from).collect()
    }

    pub fn new(application_id: Uuid, user_id: Uuid, scope: Vec<String>, expires_in: u64) -> Result<Self, HttpResponse<OAuthToken>> {
        Ok(Self {
            id: Uuid::new(),
            application_id,
            user_id,
            token: Self::generate_token(),
            scope,
            expires_in,
            created_at: DateTime::now().to_string(),
        })
    }

    #[allow(unused)]
    pub async fn insert(&self, connection: &Connection<AuthRsDatabase>) -> Result<OAuthToken, HttpResponse<OAuthToken>> {
        let db = Self::get_collection(connection);

        match db.insert_one(self.clone(), None).await {
            Ok(_) => Ok(self.clone()),
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error inserting oauth token: {:?}", err),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn get_by_token(token: &str, connection: &Connection<AuthRsDatabase>) -> Result<OAuthToken, HttpResponse<OAuthToken>> {
        let db = Self::get_collection(connection);

        let filter = doc! {
            "token": token
        };
        match db.find_one(filter, None).await.unwrap() {
            Some(token) => Ok(token),
            None => Err(HttpResponse {
                status: 404,
                message: "Token not found".to_string(),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn get_by_application_id(application_id: Uuid, connection: &Connection<AuthRsDatabase>) -> Result<Vec<OAuthToken>, HttpResponse<Vec<OAuthToken>>> {
        let db = Self::get_collection(connection);

        let filter = doc! {
            "application_id": application_id
        };
        match db.find(filter, None).await {
            Ok(cursor) => {
                let tokens = cursor.map(|doc| {
                    let token: OAuthToken = doc.unwrap();
                    return token;
                }).collect::<Vec<OAuthToken>>().await;
                Ok(tokens)
            },
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error fetching tokens: {:?}", err),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn get_by_user_id(user_id: Uuid, connection: &Connection<AuthRsDatabase>) -> Result<Vec<OAuthToken>, HttpResponse<Vec<OAuthToken>>> {
        let db = Self::get_collection(connection);

        let filter = doc! {
            "user_id": user_id
        };
        match db.find(filter, None).await {
            Ok(cursor) => {
                let tokens = cursor.map(|doc| {
                    let token: OAuthToken = doc.unwrap();
                    return token;
                }).collect::<Vec<OAuthToken>>().await;
                Ok(tokens)
            },
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error fetching tokens: {:?}", err),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn delete(&self, connection: &Connection<AuthRsDatabase>) -> Result<OAuthToken, HttpResponse<()>> {
        let db = Self::get_collection(connection);

        let filter = doc! {
            "_id": self.id
        };
        match db.delete_one(filter, None).await {
            Ok(_) => Ok(self.clone()),
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error deleting oauth token: {:?}", err),
                data: None
            })
        }
    }

    #[allow(unused)]
    fn get_collection(connection: &Connection<AuthRsDatabase>) -> Collection<Self> {
        let db = get_main_db(connection);
        db.collection(Self::COLLECTION_NAME)
    }
}
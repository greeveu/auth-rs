use anyhow::Result;
use mongodb::bson::{doc, DateTime, Document, Uuid};
use rand::Rng;
use rocket_db_pools::{mongodb::Collection, Connection};
use rocket::{futures::StreamExt, serde::{Deserialize, Serialize}};
use crate::db::{get_main_db, AuthRsDatabase};

use super::http_response::HttpResponse;

#[derive(Debug, Clone, Serialize, Deserialize)] 
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")] 
pub struct OAuthApplication {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub redirect_uris: Vec<String>,
    pub secret: String,
    pub owner: Uuid,
    pub created_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)] 
#[serde(crate = "rocket::serde")] 
#[serde(rename_all = "camelCase")] 
pub struct OAuthApplicationMinimal {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub redirect_uris: Vec<String>,
    pub owner: Uuid,
    pub created_at: DateTime,
}

impl OAuthApplicationMinimal {
    pub async  fn to_full(&self, connection: &Connection<AuthRsDatabase>) -> Result<OAuthApplication, HttpResponse<OAuthApplicationMinimal>> {
        OAuthApplication::get_full_by_id(self.id.clone(), connection).await
    }
}

impl OAuthApplication {
    pub const COLLECTION_NAME: &'static str = "oauth-applications";

    fn generate_secret() -> String {
        rand::rng().sample_iter(rand::distr::Alphanumeric).take(64).map(char::from).collect()
    }

    pub fn new(name: String, description: Option<String>, redirect_uris: Vec<String>, owner: Uuid) -> Result<Self, HttpResponse<OAuthApplication>> {
        Ok(Self {
            id: Uuid::new(),
            name,
            description,
            redirect_uris,
            secret: Self::generate_secret(),
            owner,
            created_at: DateTime::now(),
        })
    }

    pub fn to_minimal(&self) -> OAuthApplicationMinimal {
        OAuthApplicationMinimal {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            redirect_uris: self.redirect_uris.clone(),
            owner: self.owner.clone(),
            created_at: self.created_at.clone()
        }
    }

    // ONLY USE THIS INTERNALLY!
    #[allow(unused)]
    pub async fn get_full_by_id(id: Uuid, connection: &Connection<AuthRsDatabase>) -> Result<OAuthApplication, HttpResponse<OAuthApplicationMinimal>> {
        let db = Self::get_collection(connection);

        let filter = doc! {
            "_id": id
        };
        match db.find_one(filter, None).await.unwrap() {
            Some(oauth_application) => Ok(oauth_application),
            None => Err(HttpResponse {
                status: 404,
                message: "OAuth Application not found".to_string(),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn get_by_id(id: Uuid, connection: &Connection<AuthRsDatabase>) -> Result<OAuthApplicationMinimal, HttpResponse<OAuthApplicationMinimal>> {
        let db = Self::get_collection(connection);

        let filter = doc! {
            "_id": id
        };
        match db.find_one(filter, None).await.unwrap() {
            Some(oauth_application) => Ok(oauth_application.to_minimal()),
            None => Err(HttpResponse {
                status: 404,
                message: "OAuth Application not found".to_string(),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn get_all(connection: &Connection<AuthRsDatabase>, filter: Option<Document>) -> Result<Vec<OAuthApplicationMinimal>, HttpResponse<Vec<OAuthApplicationMinimal>>> {
        let db = Self::get_collection(connection);

        match db.find(filter, None).await {
            Ok(cursor) => {
                let oauth_applications = cursor.map(|doc| {
                    let oauth_application: OAuthApplication = doc.unwrap();
                    return oauth_application.to_minimal();
                }).collect::<Vec<OAuthApplicationMinimal>>().await;
                Ok(oauth_applications)
            },
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error fetching OAuth Applications: {:?}", err),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn insert(&self, connection: &Connection<AuthRsDatabase>) -> Result<OAuthApplication, HttpResponse<OAuthApplication>> {
        let db = Self::get_collection(connection);

        match db.insert_one(self.clone(), None).await {
            Ok(_) => Ok(self.clone()),
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error inserting OAuth Application: {:?}", err),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn update(&self, connection: &Connection<AuthRsDatabase>) -> Result<OAuthApplicationMinimal, HttpResponse<OAuthApplicationMinimal>> {
        let db = Self::get_collection(connection);

        let filter = doc! {
            "_id": self.id
        };
        match db.replace_one(filter, self.clone(), None).await {
            Ok(_) => Ok(self.clone().to_minimal()),
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error updating OAuth Application: {:?}", err),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn delete(&self, connection: &Connection<AuthRsDatabase>) -> Result<OAuthApplicationMinimal, HttpResponse<()>> {
        let db = Self::get_collection(connection);

        let filter = doc! {
            "_id": self.id
        };
        match db.delete_one(filter, None).await {
            Ok(_) => Ok(self.clone().to_minimal()),
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error deleting OAuth Application: {:?}", err),
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
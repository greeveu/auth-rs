use anyhow::Result;
use mongodb::bson::{doc, DateTime, Document, Uuid};
use rocket_db_pools::{mongodb::Collection, Connection};
use rocket::{futures::StreamExt, serde::{Deserialize, Serialize}};
use crate::db::{get_main_db, AuthRsDatabase};

use super::http_response::HttpResponse;

#[derive(Debug, Clone, Serialize, Deserialize)] 
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")] 
pub struct Role {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub name: String,
    pub system: bool,
    pub created_at: DateTime,
}

impl Role {
    pub const COLLECTION_NAME: &'static str = "roles";

    pub fn new(name: String) -> Result<Self, HttpResponse<Role>> {
        Ok(Self {
            id: Uuid::new(),
            name,
            system: false,
            created_at: DateTime::now(),
        })
    }

    pub fn new_system(id: Uuid, name: String) -> Result<Self, HttpResponse<Role>> {
        Ok(Self {
            id,
            name,
            system: true,
            created_at: DateTime::now(),
        })
    }

    #[allow(unused)]
    pub async fn get_by_id(id: Uuid, connection: &Connection<AuthRsDatabase>) -> Result<Role, HttpResponse<Role>> {
        let db = Self::get_collection(connection);

        let filter = doc! {
            "_id": id
        };
        match db.find_one(filter, None).await.unwrap() {
            Some(role) => Ok(role),
            None => Err(HttpResponse {
                status: 404,
                message: "Role not found".to_string(),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn get_by_name(name: &str, connection: &Connection<AuthRsDatabase>) -> Result<Role, HttpResponse<Role>> {
        let db = Self::get_collection(connection);

        let filter = doc! {
            "name": name
        };
        match db.find_one(filter, None).await.unwrap() {
            Some(role) => Ok(role),
            None => Err(HttpResponse {
                status: 404,
                message: "Role not found".to_string(),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn get_all(connection: &Connection<AuthRsDatabase>, filter: Option<Document>) -> Result<Vec<Role>, HttpResponse<Vec<Role>>> {
        let db = Self::get_collection(connection);

        match db.find(filter, None).await {
            Ok(cursor) => {
                let roles = cursor.map(|doc| {
                    let role: Role = doc.unwrap();
                    return role;
                }).collect::<Vec<Role>>().await;
                Ok(roles)
            },
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error fetching roles: {:?}", err),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn insert(&self, connection: &Connection<AuthRsDatabase>) -> Result<Role, HttpResponse<Role>> {
        let db = Self::get_collection(connection);

        match db.insert_one(self.clone(), None).await {
            Ok(_) => Ok(self.clone()),
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error inserting role: {:?}", err),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn update(&self, connection: &Connection<AuthRsDatabase>) -> Result<Role, HttpResponse<Role>> {
        let db = Self::get_collection(connection);

        let filter = doc! {
            "_id": self.id
        };
        match db.replace_one(filter, self.clone(), None).await {
            Ok(_) => Ok(self.clone()),
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error updating role: {:?}", err),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn delete(&self, connection: &Connection<AuthRsDatabase>) -> Result<Role, HttpResponse<()>> {
        let db = Self::get_collection(connection);

        let filter = doc! {
            "_id": self.id
        };
        match db.delete_one(filter, None).await {
            Ok(_) => Ok(self.clone()),
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error deleting role: {:?}", err),
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
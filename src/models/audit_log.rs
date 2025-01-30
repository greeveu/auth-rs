use std::collections::HashMap;
use anyhow::Result;
use mongodb::bson::{doc, DateTime, Uuid};
use rocket_db_pools::{mongodb::Collection, Connection};
use rocket::{futures::StreamExt, serde::{Deserialize, Serialize}}; 
use crate::db::{get_logs_db, AuthRsDatabase};

use super::http_response::HttpResponse;

#[derive(Debug, Clone, Serialize, Deserialize)] 
#[serde(crate = "rocket::serde")] 
pub struct AuditLog {
    #[serde(rename = "_id")]
	pub id: Uuid,
    #[serde(rename = "entityId")]
	pub entity_id: Uuid,
    #[serde(rename = "entityType")]
    pub entity_type: AuditLogEntityType,
	pub action: AuditLogAction,
	pub reason: String,
    #[serde(rename = "userId")]
	pub author_id: Uuid,
    #[serde(rename = "oldValues")]
	pub old_values: Option<HashMap<String, String>>,
    #[serde(rename = "newValues")]
	pub new_values: Option<HashMap<String, String>>,
    #[serde(rename = "createdAt")]
	pub created_at: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum AuditLogAction {
    Create,
    Update,
    Delete
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum AuditLogEntityType {
    User,
    Role,
    Unknown
}

#[allow(unused)]
impl AuditLogEntityType {
    pub fn from_string<T>(entity_type: &str) -> Result<Self, HttpResponse<T>> {
        match entity_type.to_uppercase().as_str() {
            "USER" => Ok(AuditLogEntityType::User),
            "ROLE" => Ok(AuditLogEntityType::Role),
            _ => Err(HttpResponse { status: 400, message: format!(""), data: None })
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            &AuditLogEntityType::User => "USER".to_string(),
            &AuditLogEntityType::Role => "ROLE".to_string(),
            _ => "UNKNOWN".to_string()
        }
    }
}



impl AuditLog {
    pub const COLLECTION_NAME_USERS: &'static str = "ueser-logs";
    pub const COLLECTION_NAME_ROLES: &'static str = "role-logs";

    #[allow(unused)]
    pub fn new(entity_id: Uuid, entity_type: AuditLogEntityType, action: AuditLogAction, reason: String, author_id: Uuid, old_values: Option<HashMap<String, String>>, new_values: Option<HashMap<String, String>>) -> Self {
        Self {
            id: Uuid::new(),
            entity_id,
            entity_type,
            action,
            reason,
            author_id,
            old_values,
            new_values,
            created_at: DateTime::now().to_string(),
        }
    }

    #[allow(unused)]
    pub async fn get_by_id<T>(id: Uuid, entity_type: AuditLogEntityType, connection: &Connection<AuthRsDatabase>) -> Result<Self, HttpResponse<T>> {
        let db = match Self::get_collection(&entity_type, connection) {
            Some(db) => db,
            None => return Err(HttpResponse {
                status: 400,
                message: "Invalid audit log entity type provided".to_string(),
                data: None
            })
        };
        

        let filter = doc! {
            "_id": id
        };
        match db.find_one(filter, None).await.unwrap() {
            Some(audit_log) => Ok(audit_log),
            None => return Err(HttpResponse {
                status: 404,
                message: "Audit log not found".to_string(),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn get_by_entity_id<T>(entity_id: Uuid, entity_type: AuditLogEntityType, connection: &Connection<AuthRsDatabase>) -> Result<Vec<Self>, HttpResponse<T>> {
        let db = match Self::get_collection(&entity_type, connection) {
            Some(db) => db,
            None => return Err(HttpResponse {
                status: 400,
                message: "Invalid audit log entity type provided".to_string(),
                data: None
            })
        };
        

        let filter = doc! {
            "entityId": entity_id
        };
        match db.find(filter, None).await {
            Ok(cursor) => {
                let audit_logs = cursor.map(|doc| doc.unwrap()).collect::<Vec<Self>>().await;
                Ok(audit_logs)
            },
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error fetching audit logs: {:?}", err),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn get_by_user_id<T>(user_id: Uuid, entity_type: AuditLogEntityType, connection: &Connection<AuthRsDatabase>) -> Result<Vec<Self>, HttpResponse<T>> {
        let db = match Self::get_collection(&entity_type, connection) {
            Some(db) => db,
            None => return Err(HttpResponse {
                status: 400,
                message: "Invalid audit log entity type provided".to_string(),
                data: None
            })
        };
        

        let filter = doc! {
            "userId": user_id
        };
        match db.find(filter, None).await {
            Ok(cursor) => {
                let audit_logs = cursor.map(|doc| doc.unwrap()).collect::<Vec<Self>>().await;
                Ok(audit_logs)
            },
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error fetching audit logs: {:?}", err),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn get_all_from_type<T>(entity_type: AuditLogEntityType, connection: &Connection<AuthRsDatabase>) -> Result<Vec<Self>, HttpResponse<T>> {
        let db = match Self::get_collection(&entity_type, connection) {
            Some(db) => db,
            None => return Err(HttpResponse {
                status: 400,
                message: "Invalid audit log entity type provided".to_string(),
                data: None
            })
        }; 
        
        match db.find(None, None).await {
            Ok(cursor) => {
                let audit_logs = cursor.map(|doc| doc.unwrap()).collect::<Vec<Self>>().await;
                Ok(audit_logs)
            },
            Err(err) => Err(HttpResponse {
                status: 500,
                message: format!("Error fetching audit logs: {:?}", err),
                data: None
            })
        }
    }

    #[allow(unused)]
    pub async fn insert(&self, connection: &Connection<AuthRsDatabase>) -> Result<(), String> {
        let db = match Self::get_collection(&self.entity_type, connection) {
            Some(db) => db,
            None => return Err("Invalid audit log entity type provided".to_string())
        };

        match db.insert_one(self.clone(), None).await {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Error inserting audit log: {:?}", err))
        }
    }

    fn get_collection(entity_type: &AuditLogEntityType, connection: &Connection<AuthRsDatabase>) -> Option<Collection<AuditLog>> {
        let db = get_logs_db(connection);

        match entity_type {
            &AuditLogEntityType::User => return Some(db.collection(Self::COLLECTION_NAME_USERS)),
            &AuditLogEntityType::Role => return Some(db.collection(Self::COLLECTION_NAME_ROLES)),
            &AuditLogEntityType::Unknown => None
        }
    }
}
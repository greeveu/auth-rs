use std::collections::HashMap;
use anyhow::Result;
use mongodb::bson::{doc, DateTime, Uuid};
use rocket_db_pools::{mongodb::Collection, Connection};
use rocket::{futures::StreamExt, serde::{Deserialize, Serialize}}; 
use crate::db::{get_logs_db, AuthRsDatabase};

use super::http_response::HttpResponse;

#[derive(Debug, Clone, Serialize, Deserialize)] 
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")] 
pub struct AuditLog {
    #[serde(rename = "_id")]
	pub id: Uuid,
	pub entity_id: Uuid,
    pub entity_type: AuditLogEntityType,
	pub action: AuditLogAction,
	pub reason: String,
	pub author_id: Uuid,
	pub old_values: Option<HashMap<String, String>>,
	pub new_values: Option<HashMap<String, String>>,
	pub created_at: DateTime
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
    OAuthApplication,
    Unknown
}

#[allow(unused)]
impl AuditLogEntityType {
    pub fn from_string<T>(entity_type: &str) -> Result<Self, HttpResponse<T>> {
        match entity_type.to_uppercase().as_str() {
            "USER" => Ok(AuditLogEntityType::User),
            "ROLE" => Ok(AuditLogEntityType::Role),
            "OAUTH_APPLICATION" => Ok(AuditLogEntityType::OAuthApplication),
            _ => Err(HttpResponse { status: 400, message: format!(""), data: None })
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            &AuditLogEntityType::User => "USER".to_string(),
            &AuditLogEntityType::Role => "ROLE".to_string(),
            &AuditLogEntityType::OAuthApplication => "OAUTH_APPLICATION".to_string(),
            _ => "UNKNOWN".to_string()
        }
    }
}



impl AuditLog {
    pub const COLLECTION_NAME_USERS: &'static str = "user-logs";
    pub const COLLECTION_NAME_ROLES: &'static str = "role-logs";
    pub const COLLECTION_NAME_OAUTH_APPLICATIONS: &'static str = "oauth-application-logs";

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
            created_at: DateTime::now(),
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
    pub async fn get_by_user_id<T>(author_id: Uuid, connection: &Connection<AuthRsDatabase>) -> Result<Vec<Self>, HttpResponse<T>> {
        let mut all_logs = vec![];
        let user_logs_collection = Self::get_collection(&AuditLogEntityType::User, &connection).unwrap();
        let role_logs_collection = Self::get_collection(&AuditLogEntityType::Role, &connection).unwrap();
        let oauth_application_logs_collection = Self::get_collection(&AuditLogEntityType::OAuthApplication, &connection).unwrap();

        let filter = doc! {
            "authorId": author_id
        };

        let user_logs = match user_logs_collection.find(filter.clone(), None).await {
            Ok(cursor) => {
                let audit_logs = cursor.map(|doc| doc.unwrap()).collect::<Vec<Self>>().await;
                audit_logs
            },
            Err(err) => return Err(HttpResponse {
                status: 500,
                message: format!("Error fetching user audit logs: {:?}", err),
                data: None
            })
        };

        let role_logs = match role_logs_collection.find(filter.clone(), None).await {
            Ok(cursor) => {
                let audit_logs = cursor.map(|doc| doc.unwrap()).collect::<Vec<Self>>().await;
                audit_logs
            },
            Err(err) => return Err(HttpResponse {
                status: 500,
                message: format!("Error fetching role audit logs: {:?}", err),
                data: None
            })
        };

        let oauth_application_logs = match oauth_application_logs_collection.find(filter.clone(), None).await {
            Ok(cursor) => {
                let audit_logs = cursor.map(|doc| doc.unwrap()).collect::<Vec<Self>>().await;
                audit_logs
            },
            Err(err) => return Err(HttpResponse {
                status: 500,
                message: format!("Error fetching oauth application audit logs: {:?}", err),
                data: None
            })
        };

        all_logs.extend(user_logs);
        all_logs.extend(role_logs);
        all_logs.extend(oauth_application_logs);

        all_logs.sort_by(|a, b| a.created_at.cmp(&b.created_at));

        Ok(all_logs)
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
            &AuditLogEntityType::OAuthApplication => return Some(db.collection(Self::COLLECTION_NAME_OAUTH_APPLICATIONS)),
            &AuditLogEntityType::Unknown => None
        }
    }
}
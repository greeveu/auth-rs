use crate::{
    auth::mfa::MfaHandler,
    db::{get_main_db, AuthRsDatabase},
    errors::{AppError, AppResult},
    routes::oauth::token::TokenOAuthData,
};
use mongodb::bson::{doc, DateTime, Uuid};
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SessionData {
    OAuthCode(TokenOAuthData),
    MfaSession(MfaHandler),
    PasskeyRegistration { user_id: Uuid, state: String },
    PasskeyAuthentication(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    #[serde(rename = "_id")]
    pub id: String,
    pub data: SessionData,
    pub expires_at: DateTime,
}

impl Session {
    pub const COLLECTION_NAME: &'static str = "sessions";

    pub fn new_oauth_code(code: u32, data: TokenOAuthData, ttl_seconds: i64) -> Self {
        Self {
            id: format!("oauth_{}", code),
            data: SessionData::OAuthCode(data),
            expires_at: DateTime::from(
                SystemTime::now()
                    .checked_add(std::time::Duration::from_secs(ttl_seconds as u64))
                    .unwrap(),
            ),
        }
    }

    pub fn new_mfa_session(flow_id: Uuid, handler: MfaHandler, ttl_seconds: i64) -> Self {
        Self {
            id: format!("mfa_{}", flow_id),
            data: SessionData::MfaSession(handler),
            expires_at: DateTime::from(
                SystemTime::now()
                    .checked_add(std::time::Duration::from_secs(ttl_seconds as u64))
                    .unwrap(),
            ),
        }
    }

    pub fn new_passkey_registration(
        registration_id: Uuid,
        user_id: Uuid,
        state: String,
        ttl_seconds: i64,
    ) -> Self {
        Self {
            id: format!("passkey_reg_{}", registration_id),
            data: SessionData::PasskeyRegistration {
                user_id,
                state,
            },
            expires_at: DateTime::from(
                SystemTime::now()
                    .checked_add(std::time::Duration::from_secs(ttl_seconds as u64))
                    .unwrap(),
            ),
        }
    }

    pub fn new_passkey_authentication(
        authentication_id: Uuid,
        state: String,
        ttl_seconds: i64,
    ) -> Self {
        Self {
            id: format!("passkey_auth_{}", authentication_id),
            data: SessionData::PasskeyAuthentication(state),
            expires_at: DateTime::from(
                SystemTime::now()
                    .checked_add(std::time::Duration::from_secs(ttl_seconds as u64))
                    .unwrap(),
            ),
        }
    }

    pub async fn insert(&self, db: &Connection<AuthRsDatabase>) -> AppResult<()> {
        let collection = get_main_db(db).collection::<Session>(Self::COLLECTION_NAME);

        collection
            .insert_one(self.clone(), None)
            .await
            .map_err(AppError::RocketMongoError)?;

        Ok(())
    }

    pub async fn get_by_id(id: &str, db: &Connection<AuthRsDatabase>) -> AppResult<Option<Self>> {
        let collection = get_main_db(db).collection::<Session>(Self::COLLECTION_NAME);

        let filter = doc! {
            "_id": id
        };

        let session = collection
            .find_one(filter, None)
            .await
            .map_err(AppError::RocketMongoError)?;

        if let Some(ref s) = session {
            if SystemTime::from(s.expires_at) < SystemTime::now() {
                Self::delete_by_id(id, db).await?;
                return Ok(None);
            }
        }

        Ok(session)
    }

    pub async fn delete_by_id(id: &str, db: &Connection<AuthRsDatabase>) -> AppResult<()> {
        let collection = get_main_db(db).collection::<Session>(Self::COLLECTION_NAME);

        let filter = doc! {
            "_id": id
        };

        collection
            .delete_one(filter, None)
            .await
            .map_err(AppError::RocketMongoError)?;

        Ok(())
    }
}


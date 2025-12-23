use std::{collections::HashMap, env};

use anyhow::Result;
use mongodb::bson::{doc, Uuid};
use rocket_db_pools::Connection;
use totp_rs::{Algorithm, Secret, TOTP};

use crate::{
    db::{get_main_db, AuthRsDatabase},
    models::{
        audit_log::{AuditLog, AuditLogAction, AuditLogEntityType},
        user::User,
    },
};

use super::AuthEntity;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MfaState {
    Pending,
    Complete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MfaType {
    Totp,
    EnableTotp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaHandler {
    pub flow_id: Uuid,
    pub state: MfaState,
    pub r#type: MfaType,
    pub user: User,
    #[serde(skip)]
    pub totp: Option<TOTP>,
}

impl MfaHandler {
    pub fn is_mfa_required(user: &User) -> bool {
        user.totp_secret.is_some()
    }

    pub async fn start_enable_flow(user: &User, db: &Connection<AuthRsDatabase>) -> Result<Self, String> {
        let flow = Self {
            flow_id: Uuid::new(),
            state: MfaState::Pending,
            r#type: MfaType::EnableTotp,
            user: user.clone(),
            totp: Some(
                TOTP::new(
                    Algorithm::SHA1,
                    6,
                    1,
                    30,
                    Secret::generate_secret().to_bytes().unwrap(),
                    Some(env::var("TOTP_ISSUER_NAME").unwrap_or_else(|_| "auth-rs".to_string())),
                    user.email.to_string(),
                )
                .unwrap(),
            ),
        };

        let session = crate::models::session::Session::new_mfa_session(flow.flow_id, flow.clone(), 300);
        if let Err(e) = session.insert(db).await {
            return Err(format!("Failed to store MFA session: {:?}", e));
        }

        Ok(flow)
    }

    pub async fn start_login_flow(user: &User, db: &Connection<AuthRsDatabase>) -> Result<Self, String> {
        if user.totp_secret.is_none() {
            return Err("User does not have TOTP enabled".to_string());
        }

        let mut flow = Self {
            flow_id: Uuid::new(),
            state: MfaState::Pending,
            r#type: MfaType::Totp,
            user: user.clone(),
            totp: None,
        };

        flow.totp = Some(
            TOTP::new(
                Algorithm::SHA1,
                6,
                1,
                30,
                Secret::Encoded(user.totp_secret.as_ref().unwrap().to_string())
                    .to_bytes()
                    .unwrap(),
                Some(env::var("TOTP_ISSUER_NAME").unwrap_or_else(|_| "auth-rs".to_string())),
                user.email.to_string(),
            )
            .unwrap(),
        );

        let session = crate::models::session::Session::new_mfa_session(flow.flow_id, flow.clone(), 300);
        if let Err(e) = session.insert(db).await {
            return Err(format!("Failed to store MFA session: {:?}", e));
        }

        Ok(flow)
    }

    pub async fn verify_current_totp(&self, code: &str, db: &Connection<AuthRsDatabase>) -> bool {
        if self.totp.is_none() || code.is_empty() {
            return false;
        }

        if self.totp.as_ref().unwrap().generate_current().unwrap() == code {
            let session_id = format!("mfa_{}", self.flow_id);
            let _ = crate::models::session::Session::delete_by_id(&session_id, db).await;

            true
        } else {
            false
        }
    }

    pub async fn verify_totp(user: &User, secret: String, code: &str) -> bool {
        let totp_result = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            Secret::Encoded(secret).to_bytes().unwrap(),
            Some(env::var("TOTP_ISSUER_NAME").unwrap_or_else(|_| "auth-rs".to_string())),
            user.email.to_string(),
        );

        if totp_result.is_err() {
            return false;
        }

        totp_result.unwrap().check_current(code).unwrap_or(false)
    }

    pub async fn disable_totp(
        user: &mut User,
        req_user: AuthEntity,
        db: &Connection<AuthRsDatabase>,
    ) -> Result<User, String> {
        let mut new_values =
            HashMap::from([("totp_secret".to_string(), "***********".to_string())]);
        let mut old_values =
            HashMap::from([("totp_secret".to_string(), "***********".to_string())]);

        user.totp_secret = None;

        let new_token = User::generate_token();
        new_values.insert("token".to_string(), "***********".to_string());
        old_values.insert("token".to_string(), "***********".to_string());

        user.token = new_token;

        let filter = doc! {
            "_id": user.id
        };
        match get_main_db(db)
            .collection(User::COLLECTION_NAME)
            .replace_one(filter, user.clone(), None)
            .await
        {
            Ok(_) => {
                match AuditLog::new(
                    user.id.to_string(),
                    AuditLogEntityType::User,
                    AuditLogAction::Update,
                    "Disable TOTP.".to_string(),
                    req_user.user_id,
                    Some(old_values),
                    Some(new_values),
                )
                .insert(db)
                .await
                {
                    Ok(_) => (),
                    Err(err) => eprintln!("{:?}", err),
                };
                Ok(user.to_owned())
            }
            Err(err) => Err(format!("Failed to disable TOTP: {:?}", err)),
        }
    }
}

impl PartialEq for MfaState {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (MfaState::Pending, MfaState::Pending) | (MfaState::Complete, MfaState::Complete)
        )
    }
}

impl PartialEq for MfaType {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (MfaType::Totp, MfaType::Totp) | (MfaType::EnableTotp, MfaType::EnableTotp)
        )
    }
}

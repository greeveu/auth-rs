use std::fmt::Display;

use rocket::serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

#[derive(Debug, Clone)]
pub enum OAuthScope {
    Users(ScopeActions),
    Roles(ScopeActions),
    AuditLogs(ScopeActions),
}

#[derive(Debug, Clone)]
pub enum ScopeActions {
    Create,
    Read,
    Update,
    Delete,
    All
}

impl Display for ScopeActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeActions::Create => write!(f, "create"),
            ScopeActions::Read => write!(f, "read"),
            ScopeActions::Update => write!(f, "update"),
            ScopeActions::Delete => write!(f, "delete"),
            ScopeActions::All => write!(f, "*"),
        }
    }
}

impl Serialize for ScopeActions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            ScopeActions::Create => "create",
            ScopeActions::Read => "read",
            ScopeActions::Update => "update",
            ScopeActions::Delete => "delete",
            ScopeActions::All => "*",
        };
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for ScopeActions {
    fn deserialize<D>(deserializer: D) -> Result<ScopeActions, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "create" => Ok(ScopeActions::Create),
            "read" => Ok(ScopeActions::Read),
            "update" => Ok(ScopeActions::Update),
            "delete" => Ok(ScopeActions::Delete),
            "*" => Ok(ScopeActions::All),
            _ => Err(Error::custom("Invalid scope action")),
        }
    }
}

impl PartialEq for ScopeActions {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ScopeActions::Create, ScopeActions::Create) => true,
            (ScopeActions::Read, ScopeActions::Read) => true,
            (ScopeActions::Update, ScopeActions::Update) => true,
            (ScopeActions::Delete, ScopeActions::Delete) => true,
            (ScopeActions::All, ScopeActions::All) => true,
            _ => false,
        }
    }
    
}

impl Display for OAuthScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OAuthScope::Users(actions) => write!(f, "user:{}", actions.to_string()),
            OAuthScope::Roles(actions) => write!(f, "roles:{}", actions.to_string()),
            OAuthScope::AuditLogs(actions) => write!(f, "audit_logs:{}", actions.to_string()),
        }
    }
}

impl PartialEq for OAuthScope {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (OAuthScope::Users(a), OAuthScope::Users(b)) => a == b,
            (OAuthScope::Roles(a), OAuthScope::Roles(b)) => a == b,
            (OAuthScope::AuditLogs(a), OAuthScope::AuditLogs(b)) => a == b,
            _ => false,
        }
    }
}

impl From<&str> for ScopeActions {
    fn from(s: &str) -> Self {
        match s {
            "create" => ScopeActions::Create,
            "read" => ScopeActions::Read,
            "update" => ScopeActions::Update,
            "delete" => ScopeActions::Delete,
            "*" => ScopeActions::All,
            _ => ScopeActions::Read,
        }
    }
}

impl TryFrom<String> for OAuthScope {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err("Invalid scope");
        }

        let actions = match parts[1] {
            "create" => ScopeActions::Create,
            "read" => ScopeActions::Read,
            "update" => ScopeActions::Update,
            "delete" => ScopeActions::Delete,
            "*" => ScopeActions::All,
            _ => return Err("Invalid scope action"),  
        };

        match parts[0] {
            "user" => Ok(OAuthScope::Users(actions)),
            "roles" => Ok(OAuthScope::Roles(actions)),
            "audit_logs" => Ok(OAuthScope::AuditLogs(actions)),
            _ => Err("Invalid scope")
        }
    }
}

impl Serialize for OAuthScope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match &self {
            OAuthScope::Users(actions) => format!("user:{:?}", actions.to_string()),
            OAuthScope::Roles(actions) => format!("roles:{:?}", actions.to_string()),
            OAuthScope::AuditLogs(actions) => format!("audit_logs:{:?}", actions.to_string()),
        };
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for OAuthScope {
    fn deserialize<D>(deserializer: D) -> Result<OAuthScope, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(Error::custom("Invalid scope"));
        }

        let actions = match parts[1] {
            "create" => ScopeActions::Create,
            "read" => ScopeActions::Read,
            "update" => ScopeActions::Update,
            "delete" => ScopeActions::Delete,
            "*" => ScopeActions::All,
            _ => return Err(Error::custom("Invalid scope action")),  
        };

        match parts[0] {
            "user" => Ok(OAuthScope::Users(actions)),
            "roles" => Ok(OAuthScope::Roles(actions)),
            "audit_logs" => Ok(OAuthScope::AuditLogs(actions)),
            _ => Err(Error::custom("Invalid scope"))
        }
    }
}
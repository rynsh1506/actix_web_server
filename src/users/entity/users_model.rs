use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, Type, PartialEq)]
#[sqlx(type_name = "user_status")]
#[serde(rename_all = "UPPERCASE")]
pub enum UserStatus {
    ACTIVE,
    DELETED,
}

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub status: Option<UserStatus>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

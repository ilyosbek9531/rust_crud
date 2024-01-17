use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};


#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: Option<String>
}


#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>
}
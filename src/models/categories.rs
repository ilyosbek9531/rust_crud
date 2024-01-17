use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct CategoryModel {
    pub id: Uuid,
    pub category_name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Deserialize)]
pub struct CreateCategory {
    pub category_name: String
}


#[derive(Debug, Deserialize)]
pub struct UpdateCategory {
    pub category_name: Option<String>
}
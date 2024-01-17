use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;


#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct RatingModel {
    pub id: Uuid,
    pub rating: i32,
    pub product_id: Uuid,
    pub user_id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}


#[derive(Debug, Deserialize)]
pub struct CreateRating {
    pub rating: i32,
    pub product_id: Uuid,
    pub user_id: Uuid
}

#[derive(Debug, Deserialize)]
pub struct UpdateRating {
    pub rating: Option<i32>,
    pub product_id: Option<Uuid>,
    pub user_id: Option<Uuid>
}



#[derive(Debug, Deserialize)]
pub struct RatingFilterOptions {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub product_id: Option<Uuid>,
    pub user_id: Option<Uuid>
}
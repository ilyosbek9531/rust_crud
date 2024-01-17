use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;


#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct PurchaseModel {
    pub id: Uuid,
    pub product_id: Uuid,
    pub user_id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}


#[derive(Debug, Deserialize)]
pub struct CreatePurchase {
    pub product_id: Uuid,
    pub user_id: Uuid
}

#[derive(Debug, Deserialize)]
pub struct UpdatePurchase {
    pub product_id: Option<Uuid>,
    pub user_id: Option<Uuid>
}



#[derive(Debug, Deserialize)]
pub struct PurchaseFilterOptions {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub product_id: Option<Uuid>,
    pub user_id: Option<Uuid>
}
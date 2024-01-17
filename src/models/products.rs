use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;
// use bigdecimal::BigDecimal;
use sqlx::types::BigDecimal;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct ProductModel {
    pub id: Uuid,
    pub product_name: String,
    pub price: BigDecimal,
    pub category_id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}


#[derive(Debug, Deserialize)]
pub struct CreateProduct {
    pub product_name: String,
    pub price: BigDecimal,
    pub category_id: Uuid
}


#[derive(Debug, Deserialize)]
pub struct UpdateProduct {
    pub product_name: Option<String>,
    pub price: Option<BigDecimal>,
    pub category_id: Option<Uuid>
}

#[derive(Debug, Deserialize)]
pub struct ProductFilterOptions {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub category_id: Option<Uuid>
}
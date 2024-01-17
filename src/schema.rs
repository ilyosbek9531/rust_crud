use serde::Deserialize;
use uuid::Uuid;


#[derive(Debug, Deserialize)]
pub struct FilterOptions {
    pub limit: Option<usize>,
    pub offset: Option<usize>
}

#[derive(Debug, Deserialize)]
pub struct PathOptions {
    pub id: Uuid
}
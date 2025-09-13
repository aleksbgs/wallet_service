use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Transfer {
    pub id: i32,
    pub from_address: String,
    pub to_address: String,
    pub amount: i64,
    pub timestamp: Option<DateTime<Utc>>, // Changed to Option to match potential SQLx inference
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TransferRequest {
    pub from: String,
    pub to: String,
    pub amount: i64,
}
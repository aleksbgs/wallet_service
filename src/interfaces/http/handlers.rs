use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::error;

use crate::application::{GetBalance, GetTransferHistory, TransferFunds};
use crate::domain::TransferRequest;
use crate::interfaces::messaging::MessageQueue;

#[derive(Clone)]
pub struct AppState {
    pub transfer_funds: Arc<TransferFunds>,
    pub get_balance: Arc<GetBalance>,
    pub get_transfer_history: Arc<GetTransferHistory>,
    pub message_queue: Arc<Mutex<dyn MessageQueue + Send + Sync>>,
}

pub async fn transfer(
    State(state): State<Arc<AppState>>,
    Json(req): Json<TransferRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    state.message_queue.lock().await.publish_transfer(&req).await
        .map_err(|e| {
            error!("Failed to publish transfer: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    Ok((StatusCode::ACCEPTED, "Transfer request accepted".to_string()))
}

pub async fn get_balance(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    state.get_balance.execute(&address).await
        .map(|balance| (StatusCode::OK, balance.to_string()))
        .map_err(|e| {
            error!("Failed to get balance for {}: {}", address, e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })
}

pub async fn get_transfers(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    state.get_transfer_history.execute(&address).await
        .map(|transfers| (StatusCode::OK, Json(transfers)))
        .map_err(|e| {
            error!("Failed to get transfers for {}: {}", address, e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })
}
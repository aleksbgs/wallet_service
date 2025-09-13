use axum::{extract::{State}, http::StatusCode, response::IntoResponse, Json};
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

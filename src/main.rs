use axum::{Router, Server, routing, Json};
use axum::extract::{Path, State};
use std::net::SocketAddr;
use std::sync::Arc;
use axum::http::StatusCode;
use tokio::sync::Mutex;
use log::error;
use futures_util::stream::StreamExt; // Import the correct StreamExt trait

use crate::application::{GetBalance, GetTransferHistory, TransferFunds};
use crate::infrastructure::{database::init_pool, message_queue::init_rabbitmq};
use crate::interfaces::http::handlers::{transfer, AppState};
use crate::interfaces::messaging::rabbitmq::RabbitMQ;
use crate::interfaces::repository::postgres::PostgresWalletRepository;

mod application;
mod domain;
mod infrastructure;
mod interfaces;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init(); // Initialize logging

    // Initialize dependencies
    let pool = init_pool().await.map_err(|e| {
        error!("Failed to initialize database pool: {}", e);
        e
    })?;
    let (rabbitmq_conn, rabbitmq_channel) = init_rabbitmq().await.map_err(|e| {
        error!("Failed to initialize RabbitMQ: {}", e);
        e
    })?;

    // Set up repository and use cases
    let repository = Box::new(PostgresWalletRepository::new(pool.clone()));
    let transfer_funds = Arc::new(TransferFunds::new(repository.clone()));
    let get_balance_use_case = Arc::new(GetBalance::new(repository.clone())); // Renamed to avoid shadowing
    let get_transfer_history = Arc::new(GetTransferHistory::new(repository));

    let message_queue = Arc::new(Mutex::new(RabbitMQ::new(rabbitmq_channel.clone())));

    // Set up RabbitMQ consumer
    let consumer_channel = rabbitmq_conn.create_channel().await.map_err(|e| {
        error!("Failed to create consumer channel: {}", e);
        e
    })?;
    let consumer = consumer_channel
        .basic_consume(
            "transfers",
            "transfer_consumer",
            lapin::options::BasicConsumeOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await
        .map_err(|e| {
            error!("Failed to start consumer: {}", e);
            e
        })?;

    let consumer_pool = pool.clone();
    let consumer_handle = tokio::spawn(async move {
        let mut consumer_stream = consumer; // Convert to Stream
        while let Some(delivery_result) = consumer_stream.next().await {
            match delivery_result {
                Ok(delivery) => {
                    if let Ok(req) = serde_json::from_slice::<domain::TransferRequest>(&delivery.data) {
                        let repo = PostgresWalletRepository::new(consumer_pool.clone());
                        let use_case = TransferFunds::new(Box::new(repo));
                        if let Err(e) = use_case.execute(req).await {
                            error!("Failed to process transfer: {}", e);
                        }
                    }
                    if let Err(e) = delivery.ack(lapin::options::BasicAckOptions::default()).await {
                        error!("Failed to ack message: {}", e);
                    }
                }
                Err(e) => error!("Consumer stream error: {}", e),
            }
        }
    });

    // Configure Axum router with wrapper functions
    let app_state = Arc::new(AppState {
        transfer_funds,
        get_balance: get_balance_use_case, // Use the use case instance
        get_transfer_history,
        message_queue,
    });

    let app = Router::new()
        .route("/transfer", routing::post(transfer))
        .route("/balance/:address", routing::get({
            let get_balance_use_case = app_state.get_balance.clone(); // Capture use case
            move |_state: State<Arc<AppState>>, Path(address): Path<String>| async move {
                match get_balance_use_case.execute(&address).await {
                    Ok(balance) => Ok((StatusCode::OK, balance.to_string())),
                    Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
                }
            }
        }))
        .route("/transfers/:address", routing::get({
            let get_transfer_history = app_state.get_transfer_history.clone(); // Capture use case
            move |_state: State<Arc<AppState>>, Path(address): Path<String>| async move {
                match get_transfer_history.execute(&address).await {
                    Ok(transfers) => Ok((StatusCode::OK, Json(transfers))),
                    Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
                }
            }
        }))
        .with_state(app_state);

    // Start Axum server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let server = Server::bind(&addr)
        .serve(app.into_make_service());
    server.await.map_err(|e| {
        error!("Failed to start server: {}", e);
        e
    })?;

    // Keep consumer running
    let _ = consumer_handle.await?; // Wait for consumer to complete
    Ok(())
}
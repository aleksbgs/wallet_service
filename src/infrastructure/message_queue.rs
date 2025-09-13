use lapin::{Connection, ConnectionProperties};
use std::env;

pub async fn init_rabbitmq() -> Result<(Connection, lapin::Channel), lapin::Error> {
    let rabbitmq_url = env::var("RABBITMQ_URL").expect("RABBITMQ_URL must be set in .env file");
    let conn = Connection::connect(&rabbitmq_url, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;
    channel
        .queue_declare(
            "transfers",
            lapin::options::QueueDeclareOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await?;
    Ok((conn, channel))
}
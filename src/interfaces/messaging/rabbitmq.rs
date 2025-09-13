use async_trait::async_trait;
use crate::domain::TransferRequest;
use crate::interfaces::MessageQueue;

pub struct RabbitMQ {
    channel: lapin::Channel,
}

impl RabbitMQ {
    pub fn new(channel: lapin::Channel) -> Self {
        Self { channel }
    }
}

#[async_trait]
impl MessageQueue for RabbitMQ {
    async fn publish_transfer(&self, req: &TransferRequest) -> Result<(), Box<dyn std::error::Error>> {
        let payload = serde_json::to_vec(req)?;
        self.channel
            .basic_publish(
                "",
                "transfers",
                lapin::options::BasicPublishOptions::default(),
                &payload,
                lapin::BasicProperties::default(),
            )
            .await?;
        Ok(())
    }
}
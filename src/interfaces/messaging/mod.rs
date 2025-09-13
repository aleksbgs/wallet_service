pub mod rabbitmq;

pub use async_trait::async_trait;

#[async_trait]
pub trait MessageQueue: Send + Sync {
    async fn publish_transfer(
        &self,
        req: &crate::domain::TransferRequest,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
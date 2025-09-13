pub mod postgres;

pub use async_trait::async_trait;

#[async_trait]
pub trait WalletRepository: Send + Sync {
    async fn get_wallet(&self, address: &str) -> Result<Option<crate::domain::Wallet>, sqlx::Error>;
    async fn get_wallet_tx(
        &self,
        address: &str,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<Option<crate::domain::Wallet>, sqlx::Error>;
    async fn update_balance(
        &self,
        address: &str,
        balance: i64,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<(), sqlx::Error>;
    async fn record_transfer(
        &self,
        req: &crate::domain::TransferRequest,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<(), sqlx::Error>;
    async fn get_transfers(&self, address: &str) -> Result<Vec<crate::domain::Transfer>, sqlx::Error>;
    async fn begin_transaction(&self) -> Result<sqlx::Transaction<'_, sqlx::Postgres>, sqlx::Error>;
}
use async_trait::async_trait;
use sqlx::{PgPool, Transaction};
use crate::domain::{Transfer, TransferRequest, Wallet};
use crate::interfaces::repository::WalletRepository;

#[derive(Clone)]
pub struct PostgresWalletRepository {
    pool: PgPool,
}

impl PostgresWalletRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WalletRepository for PostgresWalletRepository {
    async fn get_wallet(&self, address: &str) -> Result<Option<Wallet>, sqlx::Error> {
        sqlx::query_as!(Wallet, "SELECT address, balance FROM wallets WHERE address = $1", address)
            .fetch_optional(&self.pool)
            .await
    }

    async fn get_wallet_tx(
        &self,
        address: &str,
        tx: &mut Transaction<'_, sqlx::Postgres>,
    ) -> Result<Option<Wallet>, sqlx::Error> {
        sqlx::query_as!(Wallet, "SELECT address, balance FROM wallets WHERE address = $1", address)
            .fetch_optional(tx)
            .await
    }

    async fn update_balance(
        &self,
        address: &str,
        balance: i64,
        tx: &mut Transaction<'_, sqlx::Postgres>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO wallets (address, balance) VALUES ($1, $2)
             ON CONFLICT (address) DO UPDATE SET balance = EXCLUDED.balance",
            address,
            balance
        )
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn record_transfer(
        &self,
        req: &TransferRequest,
        tx: &mut Transaction<'_, sqlx::Postgres>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO transfers (from_address, to_address, amount) VALUES ($1, $2, $3)",
            req.from,
            req.to,
            req.amount
        )
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn get_transfers(&self, address: &str) -> Result<Vec<Transfer>, sqlx::Error> {
        sqlx::query_as!(
            Transfer,
            "SELECT id, from_address, to_address, amount, timestamp FROM transfers WHERE from_address = $1 OR to_address = $1 ORDER BY timestamp DESC",
            address
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn begin_transaction(&self) -> Result<Transaction<'_, sqlx::Postgres>, sqlx::Error> {
        self.pool.begin().await
    }
}
use crate::application::{GetBalance, GetTransferHistory, TransferFunds};
use crate::domain::{Wallet, Transfer, TransferRequest};
use crate::interfaces::repository::WalletRepository;
use async_trait::async_trait;
use sqlx::{Transaction, Postgres};
use chrono::Utc;

// -------------------- FAKE REPOSITORY --------------------

struct FakeWalletRepo {
    wallets: Vec<Wallet>,
    transfers: Vec<Transfer>,
}

impl FakeWalletRepo {
    fn new() -> Self {
        Self {
            wallets: vec![],
            transfers: vec![],
        }
    }
}

#[async_trait]
impl WalletRepository for FakeWalletRepo {
    async fn get_wallet(&self, address: &str) -> Result<Option<Wallet>, sqlx::Error> {
        Ok(self.wallets.iter().cloned().find(|w| w.address == address))
    }

    async fn get_wallet_tx(
        &self,
        _address: &str,
        _tx: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Wallet>, sqlx::Error> {
        Ok(None) // not used in unit tests
    }

    async fn update_balance(
        &self,
        _address: &str,
        _balance: i64,
        _tx: &mut Transaction<'_, Postgres>,
    ) -> Result<(), sqlx::Error> {
        Ok(())
    }

    async fn record_transfer(
        &self,
        _req: &TransferRequest,
        _tx: &mut Transaction<'_, Postgres>,
    ) -> Result<(), sqlx::Error> {
        Ok(())
    }

    async fn get_transfers(&self, address: &str) -> Result<Vec<Transfer>, sqlx::Error> {
        Ok(self.transfers.iter().cloned().filter(|t| t.from_address == address || t.to_address == address).collect())
    }

    async fn begin_transaction(&self) -> Result<Transaction<'_, Postgres>, sqlx::Error> {
        unimplemented!()
    }
}

// -------------------- SIMPLE UNIT TESTS --------------------

#[tokio::test]
async fn test_get_balance_existing_wallet() {
    let mut repo = FakeWalletRepo::new();
    repo.wallets.push(Wallet { address: "0x123".to_string(), balance: 1000 });

    let get_balance = GetBalance::new(Box::new(repo));
    let balance = get_balance.execute("0x123").await.unwrap();

    assert_eq!(balance, 1000);
}

#[tokio::test]
async fn test_get_balance_new_wallet() {
    let repo = FakeWalletRepo::new();

    let get_balance = GetBalance::new(Box::new(repo));
    let balance = get_balance.execute("0x456").await.unwrap();

    assert_eq!(balance, 0);
}

#[tokio::test]
async fn test_get_transfer_history_empty() {
    let repo = FakeWalletRepo::new();

    let get_history = GetTransferHistory::new(Box::new(repo));
    let history = get_history.execute("0x123").await.unwrap();

    assert!(history.is_empty());
}

#[tokio::test]
async fn test_get_transfer_history_populated() {
    let mut repo = FakeWalletRepo::new();
    let transfer = Transfer {
        id: 1,
        from_address: "0x123".to_string(),
        to_address: "0x456".to_string(),
        amount: 500,
        timestamp: Some(Utc::now()),
    };
    repo.transfers.push(transfer.clone());

    let get_history = GetTransferHistory::new(Box::new(repo));
    let history = get_history.execute("0x123").await.unwrap();

    assert_eq!(history, vec![transfer]);
}

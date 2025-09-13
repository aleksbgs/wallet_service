use crate::domain::Wallet;
use crate::interfaces::repository::WalletRepository;

pub struct GetBalance {
    repository: Box<dyn WalletRepository>,
}

impl GetBalance {
    pub fn new(repository: Box<dyn WalletRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, address: &str) -> Result<i64, String> {
        let wallet = self
            .repository
            .get_wallet(address)
            .await
            .map_err(|e| e.to_string())?
            .unwrap_or(Wallet {
                address: address.to_string(),
                balance: 0,
            });
        Ok(wallet.balance)
    }
}
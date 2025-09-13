use crate::domain::Transfer;
use crate::interfaces::repository::WalletRepository;

pub struct GetTransferHistory {
    repository: Box<dyn WalletRepository>,
}

impl GetTransferHistory {
    pub fn new(repository: Box<dyn WalletRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, address: &str) -> Result<Vec<Transfer>, String> {
        self.repository
            .get_transfers(address)
            .await
            .map_err(|e| e.to_string())
    }
}
use crate::domain::{TransferRequest, Wallet};
use crate::interfaces::repository::WalletRepository;
use log::error;

pub struct TransferFunds {
    repository: Box<dyn WalletRepository>,
}

impl TransferFunds {
    pub fn new(repository: Box<dyn WalletRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, req: TransferRequest) -> Result<(), String> {
        match self.repository.begin_transaction().await {
            Ok(mut tx) => {
                // Fetch sender wallet
                let sender = self
                    .repository
                    .get_wallet_tx(&req.from, &mut tx)
                    .await
                    .map_err(|e| {
                        error!("Failed to get sender wallet: {}", e);
                        e.to_string()
                    })?;

                let sender_balance = sender
                    .as_ref()
                    .map_or(0, |w| w.balance);
                if sender_balance < req.amount {
                    tx.rollback().await.map_err(|e| {
                        error!("Failed to rollback on insufficient balance: {}", e);
                        e.to_string()
                    })?;
                    return Err("Insufficient balance".to_string());
                }

                self.repository
                    .update_balance(&req.from, sender_balance - req.amount, &mut tx)
                    .await
                    .map_err(|e| {
                        error!("Failed to update sender balance: {}", e);
                        e.to_string()
                    })?;

                let receiver = match self
                    .repository
                    .get_wallet_tx(&req.to, &mut tx)
                    .await
                    .map_err(|e| {
                        error!("Failed to get receiver wallet: {}", e);
                        e.to_string()
                    })?
                {
                    Some(wallet) => wallet,
                    None => Wallet {
                        address: req.to.clone(),
                        balance: 0,
                    },
                };

                self.repository
                    .update_balance(&req.to, receiver.balance + req.amount, &mut tx)
                    .await
                    .map_err(|e| {
                        error!("Failed to update receiver balance: {}", e);
                        e.to_string()
                    })?;

                self.repository
                    .record_transfer(&req, &mut tx)
                    .await
                    .map_err(|e| {
                        error!("Failed to record transfer: {}", e);
                        e.to_string()
                    })?;

                tx.commit().await.map_err(|e| {
                    error!("Failed to commit transaction: {}", e);
                    e.to_string()
                })?;
                Ok(())
            }
            Err(e) => {
                error!("Failed to begin transaction: {}", e);
                Err(e.to_string())
            }
        }
    }
}
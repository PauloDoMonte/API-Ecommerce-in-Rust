// src/models/payment_management/transaction.rs

use chrono::{DateTime, Utc};

/// Representa uma transação de pagamento.
#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub amount: f64,
    pub payment_method: String, // Método de pagamento utilizado
    pub timestamp: DateTime<Utc>, // Data e hora da transação
    pub status: TransactionStatus, // Status da transação
}

#[derive(Debug, Clone)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
}

impl Transaction {
    /// Cria uma nova transação.
    pub fn new(id: String, amount: f64, payment_method: String) -> Self {
        Self {
            id,
            amount,
            payment_method,
            timestamp: Utc::now(),
            status: TransactionStatus::Pending,
        }
    }

    /// Atualiza o status da transação.
    pub fn update_status(&mut self, status: TransactionStatus) {
        self.status = status;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let mut transaction = Transaction::new("txn_1".to_string(), 100.0, "Credit Card".to_string());
        assert_eq!(transaction.amount, 100.0);
        assert_eq!(transaction.payment_method, "Credit Card");
        assert_eq!(transaction.status, TransactionStatus::Pending);

        transaction.update_status(TransactionStatus::Completed);
        assert_eq!(transaction.status, TransactionStatus::Completed);
    }
}
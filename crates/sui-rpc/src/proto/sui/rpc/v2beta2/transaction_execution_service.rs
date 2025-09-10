use super::*;

impl ExecuteTransactionRequest {
    pub fn new(transaction: Transaction) -> Self {
        Self {
            transaction: Some(transaction),
            ..Default::default()
        }
    }
}

impl ExecuteTransactionResponse {
    pub fn new(transaction: ExecutedTransaction) -> Self {
        Self {
            transaction: Some(transaction),
            ..Default::default()
        }
    }
}

impl SimulateTransactionRequest {
    pub fn new(transaction: Transaction) -> Self {
        Self {
            transaction: Some(transaction),
            ..Default::default()
        }
    }
}

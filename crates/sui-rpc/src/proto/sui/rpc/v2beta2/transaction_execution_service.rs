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

    pub fn with_checks(mut self, checks: simulate_transaction_request::TransactionChecks) -> Self {
        self.set_checks(checks);
        self
    }
}

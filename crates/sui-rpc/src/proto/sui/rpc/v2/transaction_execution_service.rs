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

impl ::prost::Name for InsufficientGasBalance {
    const NAME: &'static str = "InsufficientGasBalance";
    const PACKAGE: &'static str = "sui.rpc.v2";
    fn full_name() -> ::prost::alloc::string::String {
        "sui.rpc.v2.InsufficientGasBalance".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/sui.rpc.v2.InsufficientGasBalance".into()
    }
}

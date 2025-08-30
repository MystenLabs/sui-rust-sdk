use prost_types::FieldMask;
use sui_sdk_types::Address;
use sui_sdk_types::Digest;

use super::*;

impl GetObjectRequest {
    pub fn new(object_id: &Address) -> Self {
        Self {
            object_id: Some(object_id.to_string()),
            version: None,
            read_mask: None,
        }
    }

    pub fn with_version(mut self, version: u64) -> Self {
        self.version = Some(version);
        self
    }

    pub fn with_read_mask(mut self, read_mask: FieldMask) -> Self {
        self.read_mask = Some(read_mask);
        self
    }
}

impl GetObjectResponse {
    pub fn new(object: Object) -> Self {
        Self {
            object: Some(object),
        }
    }
}

impl BatchGetObjectsRequest {
    pub fn with_read_mask(mut self, read_mask: FieldMask) -> Self {
        self.read_mask = Some(read_mask);
        self
    }
}

impl BatchGetObjectsResponse {
    pub fn new(objects: Vec<GetObjectResult>) -> Self {
        Self { objects }
    }
}

impl GetObjectResult {
    pub fn new_object(object: Object) -> Self {
        Self {
            result: Some(get_object_result::Result::Object(object)),
        }
    }

    pub fn new_error(error: crate::proto::google::rpc::Status) -> Self {
        Self {
            result: Some(get_object_result::Result::Error(error)),
        }
    }

    pub fn to_result(self) -> Result<Object, crate::proto::google::rpc::Status> {
        match self.result {
            Some(get_object_result::Result::Object(object)) => Ok(object),
            Some(get_object_result::Result::Error(error)) => Err(error),
            None => Err(crate::proto::google::rpc::Status {
                code: tonic::Code::NotFound.into(),
                ..Default::default()
            }),
        }
    }

    pub fn object(&self) -> Option<&Object> {
        if let Some(get_object_result::Result::Object(object)) = &self.result {
            Some(object)
        } else {
            None
        }
    }

    pub fn error(&self) -> Option<&crate::proto::google::rpc::Status> {
        if let Some(get_object_result::Result::Error(error)) = &self.result {
            Some(error)
        } else {
            None
        }
    }
}

impl GetTransactionRequest {
    pub fn new(digest: &Digest) -> Self {
        Self {
            digest: Some(digest.to_string()),
            read_mask: None,
        }
    }

    pub fn with_read_mask(mut self, read_mask: FieldMask) -> Self {
        self.read_mask = Some(read_mask);
        self
    }
}

impl GetTransactionResponse {
    pub fn new(transaction: ExecutedTransaction) -> Self {
        Self {
            transaction: Some(transaction),
        }
    }
}

impl BatchGetTransactionsRequest {
    pub fn with_read_mask(mut self, read_mask: FieldMask) -> Self {
        self.read_mask = Some(read_mask);
        self
    }
}

impl BatchGetTransactionsResponse {
    pub fn new(transactions: Vec<GetTransactionResult>) -> Self {
        Self { transactions }
    }
}

impl GetTransactionResult {
    pub fn new_transaction(transaction: ExecutedTransaction) -> Self {
        Self {
            result: Some(get_transaction_result::Result::Transaction(transaction)),
        }
    }

    pub fn new_error(error: crate::proto::google::rpc::Status) -> Self {
        Self {
            result: Some(get_transaction_result::Result::Error(error)),
        }
    }

    pub fn to_result(self) -> Result<ExecutedTransaction, crate::proto::google::rpc::Status> {
        match self.result {
            Some(get_transaction_result::Result::Transaction(transaction)) => Ok(transaction),
            Some(get_transaction_result::Result::Error(error)) => Err(error),
            None => Err(crate::proto::google::rpc::Status {
                code: tonic::Code::NotFound.into(),
                ..Default::default()
            }),
        }
    }

    pub fn transaction(&self) -> Option<&ExecutedTransaction> {
        if let Some(get_transaction_result::Result::Transaction(transaction)) = &self.result {
            Some(transaction)
        } else {
            None
        }
    }

    pub fn error(&self) -> Option<&crate::proto::google::rpc::Status> {
        if let Some(get_transaction_result::Result::Error(error)) = &self.result {
            Some(error)
        } else {
            None
        }
    }
}

impl GetCheckpointRequest {
    pub fn latest() -> Self {
        Self {
            read_mask: None,
            checkpoint_id: None,
        }
    }

    pub fn by_sequence_number(checkpoint: u64) -> Self {
        Self {
            read_mask: None,
            checkpoint_id: Some(get_checkpoint_request::CheckpointId::SequenceNumber(
                checkpoint,
            )),
        }
    }

    pub fn by_digest(digest: &Digest) -> Self {
        Self {
            read_mask: None,
            checkpoint_id: Some(get_checkpoint_request::CheckpointId::Digest(
                digest.to_string(),
            )),
        }
    }

    pub fn with_read_mask(mut self, read_mask: FieldMask) -> Self {
        self.read_mask = Some(read_mask);
        self
    }
}

impl GetCheckpointResponse {
    pub fn new(checkpoint: Checkpoint) -> Self {
        Self {
            checkpoint: Some(checkpoint),
        }
    }
}

impl GetEpochRequest {
    pub fn latest() -> Self {
        Self {
            epoch: None,
            read_mask: None,
        }
    }

    pub fn new(epoch: u64) -> Self {
        Self {
            epoch: Some(epoch),
            read_mask: None,
        }
    }

    pub fn with_read_mask(mut self, read_mask: FieldMask) -> Self {
        self.read_mask = Some(read_mask);
        self
    }
}

impl GetEpochResponse {
    pub fn new(epoch: Epoch) -> Self {
        Self { epoch: Some(epoch) }
    }
}

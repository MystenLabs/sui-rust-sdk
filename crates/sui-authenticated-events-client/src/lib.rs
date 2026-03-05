// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

mod converters;
mod epoch_cache;
pub mod mmr;
pub mod proof;
mod stream;
pub mod types;

use std::sync::Arc;
use std::time::Duration;

use epoch_cache::EpochCache;
use futures::stream::Stream;
use sui_rpc::field::FieldMask;
use sui_rpc::field::FieldMaskUtil;
use sui_rpc::proto::sui::rpc::v2::GetCheckpointRequest;
use sui_rpc::proto::sui::rpc::v2::GetEpochRequest;
use sui_rpc::proto::sui::rpc::v2::event_service_client::EventServiceClient;
use sui_rpc::proto::sui::rpc::v2::ledger_service_client::LedgerServiceClient;
use sui_rpc::proto::sui::rpc::v2::proof_service_client::ProofServiceClient;
use sui_sdk_types::Address;
use sui_sdk_types::Event;
use sui_sdk_types::Identifier;
use sui_sdk_types::ObjectData;
use sui_sdk_types::StructTag;
use sui_sdk_types::ValidatorCommittee;
use thiserror::Error;
use tonic::transport::Channel;

use crate::proof::base::Proof;
use crate::proof::base::ProofContents;
use crate::proof::base::ProofTarget;
use crate::proof::base::ProofVerifier;
use crate::proof::committee::extract_new_committee_info;
use crate::proof::error::ProofError;
use crate::proof::ocs::OCSProof;
use crate::proof::ocs::OCSTarget;
use crate::types::AccumulatorKey;
use crate::types::EventStreamHead;
use crate::types::Field;
use crate::types::derive_event_stream_head_object_id;

#[derive(Debug, Clone)]
pub struct AuthenticatedEvent {
    pub event: Event,
    pub checkpoint: u64,
    pub accumulator_version: u64,
    pub transaction_idx: u32,
    pub event_idx: u32,
}

impl TryFrom<sui_rpc::proto::sui::rpc::v2::AuthenticatedEvent> for AuthenticatedEvent {
    type Error = ClientError;

    fn try_from(
        event: sui_rpc::proto::sui::rpc::v2::AuthenticatedEvent,
    ) -> Result<Self, Self::Error> {
        let proto_event = event
            .event
            .ok_or_else(|| ClientError::InternalError("Missing event data".to_string()))?;

        let contents = proto_event
            .contents
            .ok_or_else(|| ClientError::InternalError("Missing event contents".to_string()))?;

        let event_bytes = contents
            .value
            .ok_or_else(|| ClientError::InternalError("Missing event value".to_string()))?;

        let package_id = proto_event
            .package_id
            .as_ref()
            .ok_or_else(|| ClientError::InternalError("Missing package_id".to_string()))?;
        let package_id: Address = package_id
            .parse()
            .map_err(|e| ClientError::InternalError(format!("Invalid package_id: {}", e)))?;

        let module = proto_event
            .module
            .as_ref()
            .ok_or_else(|| ClientError::InternalError("Missing module".to_string()))?;
        let module: Identifier = module
            .parse()
            .map_err(|e| ClientError::InternalError(format!("Invalid module: {}", e)))?;

        let sender = proto_event
            .sender
            .as_ref()
            .ok_or_else(|| ClientError::InternalError("Missing sender".to_string()))?;
        let sender: Address = sender
            .parse()
            .map_err(|e| ClientError::InternalError(format!("Invalid sender: {}", e)))?;

        let event_type = proto_event
            .event_type
            .as_ref()
            .ok_or_else(|| ClientError::InternalError("Missing event_type".to_string()))?;
        let type_tag: StructTag = event_type
            .parse()
            .map_err(|e| ClientError::InternalError(format!("Invalid event_type: {}", e)))?;

        let event_data = Event {
            package_id,
            module,
            sender,
            type_: type_tag,
            contents: event_bytes.to_vec(),
        };

        let checkpoint = event
            .checkpoint
            .ok_or_else(|| ClientError::InternalError("Missing checkpoint".to_string()))?;
        let transaction_idx = event
            .transaction_idx
            .ok_or_else(|| ClientError::InternalError("Missing transaction_idx".to_string()))?;
        let event_idx = event
            .event_idx
            .ok_or_else(|| ClientError::InternalError("Missing event_idx".to_string()))?;
        let accumulator_version = event
            .accumulator_version
            .ok_or_else(|| ClientError::InternalError("Missing accumulator_version".to_string()))?;

        Ok(AuthenticatedEvent {
            event: event_data,
            checkpoint,
            accumulator_version,
            transaction_idx,
            event_idx,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub page_size: u32,
    pub poll_interval: Duration,
    pub max_pagination_iterations: usize,
    pub rpc_timeout: Duration,
}

impl ClientConfig {
    pub fn new(
        page_size: u32,
        poll_interval: Duration,
        max_pagination_iterations: usize,
        rpc_timeout: Duration,
    ) -> Result<Self, String> {
        if page_size == 0 {
            return Err("page_size must be greater than 0".to_string());
        }
        if page_size > 1000 {
            return Err("page_size must not exceed 1000 (server limit)".to_string());
        }
        if poll_interval.is_zero() {
            return Err("poll_interval must be greater than 0".to_string());
        }
        if max_pagination_iterations == 0 {
            return Err("max_pagination_iterations must be greater than 0".to_string());
        }
        if rpc_timeout.is_zero() {
            return Err("rpc_timeout must be greater than 0".to_string());
        }

        Ok(Self {
            page_size,
            poll_interval,
            max_pagination_iterations,
            rpc_timeout,
        })
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            page_size: 1000,
            poll_interval: Duration::from_secs(1),
            max_pagination_iterations: 100,
            rpc_timeout: Duration::from_secs(30),
        }
    }
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Verification failed: {0}")]
    VerificationError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("RPC error: {0}")]
    RpcError(#[from] tonic::Status),

    #[error("Transport error: {0}")]
    TransportError(#[from] tonic::transport::Error),
}

impl From<bcs::Error> for ClientError {
    fn from(e: bcs::Error) -> Self {
        ClientError::InternalError(format!("BCS deserialization failed: {}", e))
    }
}

impl From<ProofError> for ClientError {
    fn from(e: ProofError) -> Self {
        ClientError::VerificationError(e.to_string())
    }
}

impl ClientError {
    pub(crate) fn is_terminal(&self) -> bool {
        match self {
            ClientError::RpcError(status) => !Self::is_retriable_grpc_code(status.code()),
            ClientError::TransportError(_) => false,
            _ => true,
        }
    }

    fn is_retriable_grpc_code(code: tonic::Code) -> bool {
        matches!(
            code,
            tonic::Code::Unavailable
                | tonic::Code::DeadlineExceeded
                | tonic::Code::ResourceExhausted
                | tonic::Code::Aborted
        )
    }
}

pub struct AuthenticatedEventsClient {
    event_service: EventServiceClient<tonic::transport::Channel>,
    proof_service: ProofServiceClient<tonic::transport::Channel>,
    ledger_service: LedgerServiceClient<tonic::transport::Channel>,
    epoch_cache: Arc<tokio::sync::Mutex<EpochCache>>,
    config: ClientConfig,
}

impl AuthenticatedEventsClient {
    pub async fn new(
        rpc_url: &str,
        genesis_committee: ValidatorCommittee,
    ) -> Result<Self, ClientError> {
        Self::new_with_config(rpc_url, genesis_committee, ClientConfig::default()).await
    }

    pub async fn new_with_config(
        rpc_url: &str,
        genesis_committee: ValidatorCommittee,
        config: ClientConfig,
    ) -> Result<Self, ClientError> {
        let endpoint = Channel::from_shared(rpc_url.to_string())
            .map_err(|e| ClientError::InternalError(format!("Invalid RPC URL: {}", e)))?
            .connect_timeout(Duration::from_secs(5))
            .timeout(config.rpc_timeout);

        const MAX_RETRIES: u32 = 10;
        let mut last_err = None;
        for _ in 0..MAX_RETRIES {
            match endpoint.connect().await {
                Ok(ch) => {
                    let event_service = EventServiceClient::new(ch.clone());
                    let proof_service = ProofServiceClient::new(ch.clone());
                    let ledger_service = LedgerServiceClient::new(ch);
                    let epoch_cache = EpochCache::new(genesis_committee);

                    return Ok(Self {
                        event_service,
                        proof_service,
                        ledger_service,
                        epoch_cache: Arc::new(tokio::sync::Mutex::new(epoch_cache)),
                        config,
                    });
                }
                Err(e) => {
                    last_err = Some(e);
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
        }
        Err(last_err.unwrap().into())
    }

    fn extract_stream_head_from_object(object_bcs: &[u8]) -> Result<EventStreamHead, ClientError> {
        let object: sui_sdk_types::Object = bcs::from_bytes(object_bcs).map_err(|e| {
            ClientError::InternalError(format!("Failed to deserialize Object: {}", e))
        })?;

        let contents = match object.data() {
            ObjectData::Struct(move_struct) => move_struct.contents(),
            ObjectData::Package(_) => {
                return Err(ClientError::InternalError(
                    "Expected a Move struct, got a package".to_string(),
                ));
            }
        };

        let field: Field<AccumulatorKey, EventStreamHead> =
            bcs::from_bytes(contents).map_err(|e| {
                ClientError::InternalError(format!(
                    "Failed to deserialize EventStreamHead field: {}",
                    e
                ))
            })?;
        Ok(field.value)
    }

    pub async fn stream_events(
        self: Arc<Self>,
        stream_id: Address,
    ) -> Result<impl Stream<Item = Result<AuthenticatedEvent, ClientError>>, ClientError> {
        let config = self.config.clone();
        let stream_object_id = derive_event_stream_head_object_id(stream_id)
            .map_err(|e| ClientError::InternalError(e.to_string()))?;

        let result = self
            .fetch_current_stream_head_and_verify(stream_object_id)
            .await?;

        let (verified_head, start_checkpoint) = match result {
            Some((head, checkpoint)) => (Some(head), checkpoint + 1),
            None => (None, 0),
        };

        stream::create_event_stream_with_head(
            self,
            stream_id,
            stream_object_id,
            start_checkpoint,
            verified_head,
            config,
        )
        .await
    }

    pub async fn stream_events_from_checkpoint(
        self: Arc<Self>,
        stream_id: Address,
        last_verified_checkpoint: u64,
    ) -> Result<impl Stream<Item = Result<AuthenticatedEvent, ClientError>>, ClientError> {
        let stream_object_id = derive_event_stream_head_object_id(stream_id)
            .map_err(|e| ClientError::InternalError(e.to_string()))?;

        let (verified_head, start_checkpoint) = if last_verified_checkpoint == 0 {
            (None, 0)
        } else {
            let verified_head = self
                .fetch_and_verify_stream_head(stream_object_id, last_verified_checkpoint)
                .await?;

            (Some(verified_head), last_verified_checkpoint + 1)
        };

        let config = self.config.clone();
        stream::create_event_stream_with_head(
            self,
            stream_id,
            stream_object_id,
            start_checkpoint,
            verified_head,
            config,
        )
        .await
    }

    async fn get_committee_for_checkpoint(
        &self,
        checkpoint: u64,
    ) -> Result<ValidatorCommittee, ClientError> {
        self.trust_ratchet_to_checkpoint(checkpoint).await?;

        let epoch_cache = self.epoch_cache.lock().await;
        let committee = epoch_cache
            .get_committee_for_checkpoint(checkpoint)
            .expect("Committee must exist after ensure_committee_for_checkpoint succeeded")
            .clone();

        Ok(committee)
    }

    async fn trust_ratchet_to_checkpoint(&self, checkpoint: u64) -> Result<(), ClientError> {
        loop {
            let (is_in_completed_epoch, current_epoch, current_committee, current_epoch_start) = {
                let epoch_cache = self.epoch_cache.lock().await;
                let is_in_completed_epoch =
                    checkpoint < epoch_cache.current_epoch_start_checkpoint();
                let current_epoch = epoch_cache.current_epoch();
                let current_epoch_start = epoch_cache.current_epoch_start_checkpoint();
                let current_committee = epoch_cache.current_committee().clone();
                (
                    is_in_completed_epoch,
                    current_epoch,
                    current_committee,
                    current_epoch_start,
                )
            };

            if is_in_completed_epoch {
                return Ok(());
            }

            let result = self
                .fetch_and_verify_next_epoch(current_epoch, &current_committee, checkpoint)
                .await?;

            let Some((end_of_epoch_checkpoint, next_committee)) = result else {
                return Ok(());
            };

            let mut epoch_cache = self.epoch_cache.lock().await;
            if epoch_cache.current_epoch() == current_epoch {
                epoch_cache.apply_ratchet_update(
                    current_epoch_start,
                    end_of_epoch_checkpoint,
                    current_committee,
                    next_committee,
                );
            }
        }
    }

    pub(crate) fn event_service(&self) -> EventServiceClient<tonic::transport::Channel> {
        self.event_service.clone()
    }

    pub(crate) async fn fetch_and_verify_stream_head(
        &self,
        stream_object_id: Address,
        checkpoint: u64,
    ) -> Result<EventStreamHead, ClientError> {
        let committee = self.get_committee_for_checkpoint(checkpoint).await?;

        let mut proof_client = self.proof_service.clone();

        let mut request = sui_rpc::proto::sui::rpc::v2::GetObjectInclusionProofRequest::default();
        request.object_id = Some(stream_object_id.to_string());
        request.checkpoint = Some(checkpoint);

        let response = match proof_client.get_object_inclusion_proof(request).await {
            Ok(resp) => resp.into_inner(),
            Err(status) if status.code() == tonic::Code::FailedPrecondition => {
                return Err(ClientError::InternalError(format!(
                    "EventStreamHead was not updated at checkpoint {} (no events were emitted)",
                    checkpoint
                )));
            }
            Err(status) => return Err(ClientError::RpcError(status)),
        };

        let object_data_bytes = response
            .object_data
            .as_ref()
            .ok_or_else(|| ClientError::InternalError("Missing object data".to_string()))?;

        let stream_head = Self::extract_stream_head_from_object(object_data_bytes)?;

        self.verify_ocs_inclusion_proof(&committee, &response)?;

        Ok(stream_head)
    }

    async fn fetch_current_stream_head_and_verify(
        &self,
        stream_object_id: Address,
    ) -> Result<Option<(EventStreamHead, u64)>, ClientError> {
        let mut ledger_client = self.ledger_service.clone();

        let mut request = sui_rpc::proto::sui::rpc::v2::GetObjectRequest::default();
        request.object_id = Some(stream_object_id.to_string());
        request.read_mask = Some(FieldMask::from_paths(["bcs"]));

        let response = match ledger_client.get_object(request).await {
            Ok(r) => r.into_inner(),
            Err(status) if status.code() == tonic::Code::NotFound => {
                return Ok(None);
            }
            Err(status) => return Err(ClientError::RpcError(status)),
        };

        let proto_object = response
            .object
            .ok_or_else(|| ClientError::InternalError("Missing object in response".to_string()))?;

        let bcs_data = proto_object
            .bcs
            .ok_or_else(|| ClientError::InternalError("Missing bcs data".to_string()))?;

        let object_data_bytes = bcs_data
            .value
            .ok_or_else(|| ClientError::InternalError("Missing bcs value".to_string()))?;

        let stream_head = Self::extract_stream_head_from_object(&object_data_bytes)?;
        let checkpoint = stream_head.checkpoint_seq;

        let verified_head = self
            .fetch_and_verify_stream_head(stream_object_id, checkpoint)
            .await?;

        Ok(Some((verified_head, checkpoint)))
    }

    async fn fetch_and_verify_next_epoch(
        &self,
        current_epoch: u64,
        current_committee: &ValidatorCommittee,
        to_checkpoint: u64,
    ) -> Result<Option<(u64, ValidatorCommittee)>, ClientError> {
        let mut ledger_client = self.ledger_service.clone();
        let response = ledger_client
            .get_epoch(GetEpochRequest::new(current_epoch))
            .await;

        let end_of_epoch_checkpoint_seq = match response {
            Ok(resp) => {
                let epoch_info =
                    resp.into_inner()
                        .epoch
                        .ok_or(ClientError::InternalError(format!(
                            "Failed to get last checkpoint of epoch {}: Missing epoch info",
                            current_epoch
                        )))?;
                match epoch_info.last_checkpoint {
                    Some(end) => end,
                    None => return Ok(None),
                }
            }
            Err(status) if status.code() == tonic::Code::NotFound => return Ok(None),
            Err(status) => {
                return Err(ClientError::InternalError(format!(
                    "Failed to get last checkpoint of epoch {}: {}",
                    current_epoch, status
                )));
            }
        };

        if to_checkpoint <= end_of_epoch_checkpoint_seq {
            return Ok(None);
        }

        let mut get_cp_request =
            GetCheckpointRequest::by_sequence_number(end_of_epoch_checkpoint_seq);
        get_cp_request.read_mask =
            Some(FieldMask::from_paths(["summary", "signature", "contents"]));

        let checkpoint_response = ledger_client
            .get_checkpoint(get_cp_request)
            .await
            .map_err(|status| {
                ClientError::InternalError(format!(
                    "Failed to fetch checkpoint {}: {}",
                    end_of_epoch_checkpoint_seq, status
                ))
            })?
            .into_inner();

        let proto_checkpoint = checkpoint_response
            .checkpoint
            .ok_or(ClientError::InternalError(
                "Missing checkpoint in response".to_string(),
            ))?;

        let proto_summary = proto_checkpoint
            .summary
            .as_ref()
            .ok_or(ClientError::InternalError(
                "Missing checkpoint summary".to_string(),
            ))?;

        let summary: sui_sdk_types::CheckpointSummary = proto_summary.try_into().map_err(|e| {
            ClientError::InternalError(format!("Failed to convert checkpoint summary: {:?}", e))
        })?;

        let proto_sig = proto_checkpoint
            .signature
            .as_ref()
            .ok_or(ClientError::InternalError(
                "Missing checkpoint signature".to_string(),
            ))?;

        let signature: sui_sdk_types::ValidatorAggregatedSignature =
            proto_sig.try_into().map_err(|e| {
                ClientError::InternalError(format!(
                    "Failed to convert checkpoint signature: {:?}",
                    e
                ))
            })?;

        use sui_crypto::bls12381::ValidatorCommitteeSignatureVerifier;
        let verifier = ValidatorCommitteeSignatureVerifier::new(current_committee.clone())
            .map_err(|e| {
                ClientError::VerificationError(format!(
                    "Failed to create verifier for epoch {}: {}",
                    current_epoch, e
                ))
            })?;

        verifier
            .verify_checkpoint_summary(&summary, &signature)
            .map_err(|e| {
                ClientError::VerificationError(format!(
                    "Failed to verify checkpoint {}: {}",
                    end_of_epoch_checkpoint_seq, e
                ))
            })?;

        let next_committee = extract_new_committee_info(&summary).map_err(|e| {
            ClientError::VerificationError(format!(
                "Failed to extract committee from checkpoint {}: {}",
                end_of_epoch_checkpoint_seq, e
            ))
        })?;

        Ok(Some((end_of_epoch_checkpoint_seq, next_committee)))
    }

    fn verify_ocs_inclusion_proof(
        &self,
        committee: &ValidatorCommittee,
        response: &sui_rpc::proto::sui::rpc::v2::GetObjectInclusionProofResponse,
    ) -> Result<(), ClientError> {
        let checkpoint_summary_bytes = response
            .checkpoint_summary
            .as_ref()
            .ok_or_else(|| ClientError::InternalError("Missing checkpoint summary".to_string()))?;

        let signed_summary: sui_sdk_types::SignedCheckpointSummary =
            bcs::from_bytes(checkpoint_summary_bytes)?;

        let object_ref_proto = response
            .object_ref
            .as_ref()
            .ok_or_else(|| ClientError::InternalError("Missing object_ref".to_string()))?;

        let inclusion_proof = response
            .inclusion_proof
            .as_ref()
            .ok_or_else(|| ClientError::InternalError("Missing inclusion proof".to_string()))?;

        let object_ref = converters::proto_object_ref_to_object_reference(object_ref_proto)?;
        let ocs_inclusion_proof =
            converters::proto_ocs_inclusion_proof_to_light_client_proof(inclusion_proof)?;

        let target = OCSTarget::new_inclusion_target(object_ref);

        let proof = Proof {
            targets: ProofTarget::ObjectCheckpointState(target),
            checkpoint_summary: signed_summary,
            proof_contents: ProofContents::ObjectCheckpointStateProof(OCSProof::Inclusion(
                ocs_inclusion_proof,
            )),
        };

        proof.verify(committee)?;

        Ok(())
    }
}

// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use sui_sdk_types::Address;
use sui_sdk_types::Digest;
use sui_sdk_types::ObjectReference;

use crate::ClientError;
use crate::proof::ocs::OCSInclusionProof;

pub(super) fn proto_object_ref_to_object_reference(
    proto: &sui_rpc::proto::sui::rpc::v2::ObjectReference,
) -> Result<ObjectReference, ClientError> {
    let object_id_str = proto
        .object_id
        .as_ref()
        .ok_or_else(|| ClientError::InternalError("Missing object_id".to_string()))?;

    let object_id: Address = object_id_str
        .parse()
        .map_err(|e| ClientError::InternalError(format!("Invalid object_id: {}", e)))?;

    let version = proto
        .version
        .ok_or_else(|| ClientError::InternalError("Missing version".to_string()))?;

    let digest_str = proto
        .digest
        .as_ref()
        .ok_or_else(|| ClientError::InternalError("Missing digest".to_string()))?;

    let digest: Digest = digest_str
        .parse()
        .map_err(|e| ClientError::InternalError(format!("Invalid digest: {}", e)))?;

    Ok(ObjectReference::new(object_id, version, digest))
}

pub(super) fn proto_ocs_inclusion_proof_to_light_client_proof(
    proto: &sui_rpc::proto::sui::rpc::v2::ObjectInclusionProof,
) -> Result<OCSInclusionProof, ClientError> {
    let merkle_proof_bytes = proto
        .merkle_proof
        .as_ref()
        .ok_or_else(|| ClientError::InternalError("Missing merkle_proof".to_string()))?;

    let merkle_proof: sui_crypto::merkle::MerkleProof = bcs::from_bytes(merkle_proof_bytes)?;

    let leaf_index = proto
        .leaf_index
        .ok_or_else(|| ClientError::InternalError("Missing leaf_index".to_string()))?
        as usize;

    let tree_root_bytes = proto
        .tree_root
        .as_ref()
        .ok_or_else(|| ClientError::InternalError("Missing tree_root".to_string()))?;

    if tree_root_bytes.len() != 32 {
        return Err(ClientError::InternalError(format!(
            "Invalid tree_root length: {}",
            tree_root_bytes.len()
        )));
    }
    let mut tree_root_arr = [0u8; 32];
    tree_root_arr.copy_from_slice(tree_root_bytes);
    let tree_root = Digest::new(tree_root_arr);

    Ok(OCSInclusionProof {
        merkle_proof,
        leaf_index,
        tree_root,
    })
}

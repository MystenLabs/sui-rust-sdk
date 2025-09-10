use super::*;
use crate::field::FieldMaskTree;
use crate::merge::Merge;
use crate::proto::TryFromProtoError;
use tap::Pipe;

//
// ValidatorAggregatedSignature
//

impl From<sui_sdk_types::ValidatorAggregatedSignature> for ValidatorAggregatedSignature {
    fn from(value: sui_sdk_types::ValidatorAggregatedSignature) -> Self {
        let mut bitmap = Vec::new();
        value.bitmap.serialize_into(&mut bitmap).unwrap();

        Self {
            epoch: Some(value.epoch),
            signature: Some(value.signature.as_bytes().to_vec().into()),
            bitmap: Some(bitmap.into()),
        }
    }
}

impl TryFrom<&ValidatorAggregatedSignature> for sui_sdk_types::ValidatorAggregatedSignature {
    type Error = TryFromProtoError;

    fn try_from(value: &ValidatorAggregatedSignature) -> Result<Self, Self::Error> {
        let epoch = value
            .epoch
            .ok_or_else(|| TryFromProtoError::missing("epoch"))?;
        let signature = value
            .signature
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("signature"))?
            .as_ref()
            .pipe(sui_sdk_types::Bls12381Signature::from_bytes)
            .map_err(|e| {
                TryFromProtoError::invalid(ValidatorAggregatedSignature::SIGNATURE_FIELD, e)
            })?;
        let bitmap = sui_sdk_types::Bitmap::deserialize_from(value.bitmap()).map_err(|e| {
            TryFromProtoError::invalid(ValidatorAggregatedSignature::BITMAP_FIELD, e)
        })?;

        Ok(Self {
            epoch,
            signature,
            bitmap,
        })
    }
}

//
// ValidatorCommitteeMember
//

impl From<sui_sdk_types::ValidatorCommitteeMember> for ValidatorCommitteeMember {
    fn from(value: sui_sdk_types::ValidatorCommitteeMember) -> Self {
        Self {
            public_key: Some(value.public_key.as_bytes().to_vec().into()),
            weight: Some(value.stake),
        }
    }
}

impl TryFrom<&ValidatorCommitteeMember> for sui_sdk_types::ValidatorCommitteeMember {
    type Error = TryFromProtoError;

    fn try_from(
        ValidatorCommitteeMember { public_key, weight }: &ValidatorCommitteeMember,
    ) -> Result<Self, Self::Error> {
        let public_key = public_key
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("public_key"))?
            .as_ref()
            .pipe(sui_sdk_types::Bls12381PublicKey::from_bytes)
            .map_err(|e| {
                TryFromProtoError::invalid(ValidatorCommitteeMember::PUBLIC_KEY_FIELD, e)
            })?;
        let stake = weight.ok_or_else(|| TryFromProtoError::missing("weight"))?;
        Ok(Self { public_key, stake })
    }
}

//
// ValidatorCommittee
//

impl From<sui_sdk_types::ValidatorCommittee> for ValidatorCommittee {
    fn from(value: sui_sdk_types::ValidatorCommittee) -> Self {
        Self {
            epoch: Some(value.epoch),
            members: value.members.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<&ValidatorCommittee> for sui_sdk_types::ValidatorCommittee {
    type Error = TryFromProtoError;

    fn try_from(value: &ValidatorCommittee) -> Result<Self, Self::Error> {
        let epoch = value
            .epoch
            .ok_or_else(|| TryFromProtoError::missing("epoch"))?;
        Ok(Self {
            epoch,
            members: value
                .members
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}

//
// CircomG1
//

impl From<sui_sdk_types::CircomG1> for CircomG1 {
    fn from(value: sui_sdk_types::CircomG1) -> Self {
        let [e0, e1, e2] = value.0;

        Self {
            e0: Some(e0.to_string()),
            e1: Some(e1.to_string()),
            e2: Some(e2.to_string()),
        }
    }
}

impl TryFrom<&CircomG1> for sui_sdk_types::CircomG1 {
    type Error = TryFromProtoError;

    fn try_from(value: &CircomG1) -> Result<Self, Self::Error> {
        let e0 = value
            .e0
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("e0"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(CircomG1::E0_FIELD, e))?;
        let e1 = value
            .e1
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("e1"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(CircomG1::E1_FIELD, e))?;
        let e2 = value
            .e2
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("e2"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(CircomG1::E2_FIELD, e))?;

        Ok(Self([e0, e1, e2]))
    }
}

//
// CircomG2
//

impl From<sui_sdk_types::CircomG2> for CircomG2 {
    fn from(value: sui_sdk_types::CircomG2) -> Self {
        let [[e00, e01], [e10, e11], [e20, e21]] = value.0;

        Self {
            e00: Some(e00.to_string()),
            e01: Some(e01.to_string()),
            e10: Some(e10.to_string()),
            e11: Some(e11.to_string()),
            e20: Some(e20.to_string()),
            e21: Some(e21.to_string()),
        }
    }
}

impl TryFrom<&CircomG2> for sui_sdk_types::CircomG2 {
    type Error = TryFromProtoError;

    fn try_from(value: &CircomG2) -> Result<Self, Self::Error> {
        let e00 = value
            .e00
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("e00"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(CircomG2::E00_FIELD, e))?;
        let e01 = value
            .e01
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("e01"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(CircomG2::E01_FIELD, e))?;

        let e10 = value
            .e10
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("e10"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(CircomG2::E10_FIELD, e))?;
        let e11 = value
            .e11
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("e11"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(CircomG2::E11_FIELD, e))?;

        let e20 = value
            .e20
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("e20"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(CircomG2::E20_FIELD, e))?;
        let e21 = value
            .e21
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("e21"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(CircomG2::E21_FIELD, e))?;

        Ok(Self([[e00, e01], [e10, e11], [e20, e21]]))
    }
}

//
// ZkLoginClaim
//

impl From<sui_sdk_types::ZkLoginClaim> for ZkLoginClaim {
    fn from(
        sui_sdk_types::ZkLoginClaim { value, index_mod_4 }: sui_sdk_types::ZkLoginClaim,
    ) -> Self {
        Self {
            value: Some(value),
            index_mod_4: Some(index_mod_4.into()),
        }
    }
}

impl TryFrom<&ZkLoginClaim> for sui_sdk_types::ZkLoginClaim {
    type Error = TryFromProtoError;

    fn try_from(ZkLoginClaim { value, index_mod_4 }: &ZkLoginClaim) -> Result<Self, Self::Error> {
        let value = value
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("value"))?
            .into();
        let index_mod_4 = index_mod_4
            .ok_or_else(|| TryFromProtoError::missing("index_mod_4"))?
            .try_into()
            .map_err(|e| TryFromProtoError::invalid(ZkLoginClaim::INDEX_MOD_4_FIELD, e))?;

        Ok(Self { value, index_mod_4 })
    }
}

//
// ZkLoginProof
//

impl From<sui_sdk_types::ZkLoginProof> for ZkLoginProof {
    fn from(value: sui_sdk_types::ZkLoginProof) -> Self {
        Self {
            a: Some(value.a.into()),
            b: Some(value.b.into()),
            c: Some(value.c.into()),
        }
    }
}

impl TryFrom<&ZkLoginProof> for sui_sdk_types::ZkLoginProof {
    type Error = TryFromProtoError;

    fn try_from(value: &ZkLoginProof) -> Result<Self, Self::Error> {
        let a = value
            .a
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("a"))?
            .try_into()?;
        let b = value
            .b
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("b"))?
            .try_into()?;
        let c = value
            .c
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("c"))?
            .try_into()?;

        Ok(Self { a, b, c })
    }
}

//
// ZkLoginInputs
//

impl From<sui_sdk_types::ZkLoginInputs> for ZkLoginInputs {
    fn from(value: sui_sdk_types::ZkLoginInputs) -> Self {
        Self {
            proof_points: Some(value.proof_points().clone().into()),
            iss_base64_details: Some(value.iss_base64_details().clone().into()),
            header_base64: Some(value.header_base64().into()),
            address_seed: Some(value.address_seed().to_string()),
        }
    }
}

impl TryFrom<&ZkLoginInputs> for sui_sdk_types::ZkLoginInputs {
    type Error = TryFromProtoError;

    fn try_from(value: &ZkLoginInputs) -> Result<Self, Self::Error> {
        let proof_points = value
            .proof_points
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("proof_points"))?
            .try_into()?;
        let iss_base64_details = value
            .iss_base64_details
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("iss_base64_details"))?
            .try_into()?;
        let header_base64 = value
            .header_base64
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("header_base64"))?
            .into();
        let address_seed = value
            .address_seed
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("address_seed"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(ZkLoginInputs::ADDRESS_SEED_FIELD, e))?;

        Self::new(
            proof_points,
            iss_base64_details,
            header_base64,
            address_seed,
        )
        .map_err(|e| TryFromProtoError::invalid("inputs", e))
    }
}

//
// ZkLoginAuthenticator
//

impl From<sui_sdk_types::ZkLoginAuthenticator> for ZkLoginAuthenticator {
    fn from(value: sui_sdk_types::ZkLoginAuthenticator) -> Self {
        let public_identifier = Some(value.inputs.public_identifier().into());
        let jwk_id = Some(value.inputs.jwk_id().into());
        Self {
            inputs: Some(value.inputs.into()),
            max_epoch: Some(value.max_epoch),
            signature: Some(value.signature.into()),
            public_identifier,
            jwk_id,
        }
    }
}

impl TryFrom<&ZkLoginAuthenticator> for sui_sdk_types::ZkLoginAuthenticator {
    type Error = TryFromProtoError;

    fn try_from(value: &ZkLoginAuthenticator) -> Result<Self, Self::Error> {
        let inputs = value
            .inputs
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("inputs"))?
            .try_into()?;
        let max_epoch = value
            .max_epoch
            .ok_or_else(|| TryFromProtoError::missing("max_epoch"))?;
        let signature = value
            .signature
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("signature"))?
            .try_into()?;

        Ok(Self {
            inputs,
            max_epoch,
            signature,
        })
    }
}

//
// ZkLoginPublicIdentifier
//

impl From<&sui_sdk_types::ZkLoginPublicIdentifier> for ZkLoginPublicIdentifier {
    fn from(value: &sui_sdk_types::ZkLoginPublicIdentifier) -> Self {
        Self {
            iss: Some(value.iss().to_owned()),
            address_seed: Some(value.address_seed().to_string()),
        }
    }
}

impl TryFrom<&ZkLoginPublicIdentifier> for sui_sdk_types::ZkLoginPublicIdentifier {
    type Error = TryFromProtoError;

    fn try_from(value: &ZkLoginPublicIdentifier) -> Result<Self, Self::Error> {
        let iss = value
            .iss
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("iss"))?
            .into();
        let address_seed = value
            .address_seed
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("address_seed"))?
            .parse()
            .map_err(|e| {
                TryFromProtoError::invalid(ZkLoginPublicIdentifier::ADDRESS_SEED_FIELD, e)
            })?;

        Self::new(iss, address_seed)
            .ok_or_else(|| {
                TryFromProtoError::invalid(ZkLoginPublicIdentifier::ISS_FIELD, "invalid iss")
            })?
            .pipe(Ok)
    }
}

//
// SignatureScheme
//

impl TryFrom<&SignatureScheme> for sui_sdk_types::SignatureScheme {
    type Error = TryFromProtoError;

    fn try_from(value: &SignatureScheme) -> Result<Self, Self::Error> {
        use SignatureScheme::*;

        match value {
            Ed25519 => Self::Ed25519,
            Secp256k1 => Self::Secp256k1,
            Secp256r1 => Self::Secp256r1,
            Multisig => Self::Multisig,
            Bls12381 => Self::Bls12381,
            Zklogin => Self::ZkLogin,
            Passkey => Self::Passkey,
        }
        .pipe(Ok)
    }
}

//
// SimpleSignature
//

impl From<sui_sdk_types::SimpleSignature> for SimpleSignature {
    fn from(value: sui_sdk_types::SimpleSignature) -> Self {
        let (signature, public_key) = match &value {
            sui_sdk_types::SimpleSignature::Ed25519 {
                signature,
                public_key,
            } => (signature.as_bytes(), public_key.as_bytes()),
            sui_sdk_types::SimpleSignature::Secp256k1 {
                signature,
                public_key,
            } => (signature.as_bytes(), public_key.as_bytes()),
            sui_sdk_types::SimpleSignature::Secp256r1 {
                signature,
                public_key,
            } => (signature.as_bytes(), public_key.as_bytes()),
            _ => return Self::default(),
        };

        Self {
            scheme: Some(value.scheme().to_u8().into()),
            signature: Some(signature.to_vec().into()),
            public_key: Some(public_key.to_vec().into()),
        }
    }
}

impl TryFrom<&SimpleSignature> for sui_sdk_types::SimpleSignature {
    type Error = TryFromProtoError;

    fn try_from(value: &SimpleSignature) -> Result<Self, Self::Error> {
        use sui_sdk_types::Ed25519PublicKey;
        use sui_sdk_types::Ed25519Signature;
        use sui_sdk_types::Secp256k1PublicKey;
        use sui_sdk_types::Secp256k1Signature;
        use sui_sdk_types::Secp256r1PublicKey;
        use sui_sdk_types::Secp256r1Signature;
        use SignatureScheme;

        let scheme = value
            .scheme
            .ok_or_else(|| TryFromProtoError::missing("scheme"))?
            .pipe(SignatureScheme::try_from)
            .map_err(|e| TryFromProtoError::invalid(SimpleSignature::SCHEME_FIELD, e))?;
        let signature = value
            .signature
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("signature"))?;
        let public_key = value
            .public_key
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("public_key"))?;

        match scheme {
            SignatureScheme::Ed25519 => Self::Ed25519 {
                signature: Ed25519Signature::from_bytes(signature)
                    .map_err(|e| TryFromProtoError::invalid(SimpleSignature::SIGNATURE_FIELD, e))?,
                public_key: Ed25519PublicKey::from_bytes(public_key).map_err(|e| {
                    TryFromProtoError::invalid(SimpleSignature::PUBLIC_KEY_FIELD, e)
                })?,
            },
            SignatureScheme::Secp256k1 => Self::Secp256k1 {
                signature: Secp256k1Signature::from_bytes(signature)
                    .map_err(|e| TryFromProtoError::invalid(SimpleSignature::SIGNATURE_FIELD, e))?,
                public_key: Secp256k1PublicKey::from_bytes(public_key).map_err(|e| {
                    TryFromProtoError::invalid(SimpleSignature::PUBLIC_KEY_FIELD, e)
                })?,
            },
            SignatureScheme::Secp256r1 => Self::Secp256r1 {
                signature: Secp256r1Signature::from_bytes(signature)
                    .map_err(|e| TryFromProtoError::invalid(SimpleSignature::SIGNATURE_FIELD, e))?,
                public_key: Secp256r1PublicKey::from_bytes(public_key).map_err(|e| {
                    TryFromProtoError::invalid(SimpleSignature::PUBLIC_KEY_FIELD, e)
                })?,
            },
            SignatureScheme::Multisig
            | SignatureScheme::Bls12381
            | SignatureScheme::Zklogin
            | SignatureScheme::Passkey => {
                return Err(TryFromProtoError::invalid(
                    SimpleSignature::SCHEME_FIELD,
                    "invalid or unknown signature scheme",
                ))
            }
        }
        .pipe(Ok)
    }
}

//
// PasskeyAuthenticator
//

impl From<sui_sdk_types::PasskeyAuthenticator> for PasskeyAuthenticator {
    fn from(value: sui_sdk_types::PasskeyAuthenticator) -> Self {
        Self {
            authenticator_data: Some(value.authenticator_data().to_vec().into()),
            client_data_json: Some(value.client_data_json().to_owned()),
            signature: Some(value.signature().into()),
        }
    }
}

impl TryFrom<&PasskeyAuthenticator> for sui_sdk_types::PasskeyAuthenticator {
    type Error = TryFromProtoError;

    fn try_from(value: &PasskeyAuthenticator) -> Result<Self, Self::Error> {
        let authenticator_data = value
            .authenticator_data
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("authenticator_data"))?
            .to_vec();
        let client_data_json = value
            .client_data_json
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("client_data_json"))?
            .into();

        let signature = value
            .signature
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("signature"))?
            .try_into()?;

        Self::new(authenticator_data, client_data_json, signature).ok_or_else(|| {
            TryFromProtoError::invalid(
                PasskeyAuthenticator::CLIENT_DATA_JSON_FIELD,
                "invalid passkey",
            )
        })
    }
}

//
// MultisigMemberPublicKey
//

impl From<&sui_sdk_types::MultisigMemberPublicKey> for MultisigMemberPublicKey {
    fn from(value: &sui_sdk_types::MultisigMemberPublicKey) -> Self {
        use sui_sdk_types::MultisigMemberPublicKey::*;

        let mut message = Self::default();

        let scheme = match value {
            Ed25519(public_key) => {
                message.public_key = Some(public_key.as_bytes().to_vec().into());
                SignatureScheme::Ed25519
            }
            Secp256k1(public_key) => {
                message.public_key = Some(public_key.as_bytes().to_vec().into());
                SignatureScheme::Secp256k1
            }
            Secp256r1(public_key) => {
                message.public_key = Some(public_key.as_bytes().to_vec().into());
                SignatureScheme::Secp256r1
            }
            ZkLogin(zklogin_id) => {
                message.zklogin = Some(zklogin_id.into());
                SignatureScheme::Zklogin
            }
            Passkey(public_key) => {
                message.public_key = Some(public_key.inner().as_bytes().to_vec().into());
                SignatureScheme::Passkey
            }
            _ => return Self::default(),
        };

        message.set_scheme(scheme);
        message
    }
}

impl TryFrom<&MultisigMemberPublicKey> for sui_sdk_types::MultisigMemberPublicKey {
    type Error = TryFromProtoError;

    fn try_from(value: &MultisigMemberPublicKey) -> Result<Self, Self::Error> {
        use sui_sdk_types::Ed25519PublicKey;
        use sui_sdk_types::Secp256k1PublicKey;
        use sui_sdk_types::Secp256r1PublicKey;

        match value.scheme() {
            SignatureScheme::Ed25519 => Self::Ed25519(
                Ed25519PublicKey::from_bytes(value.public_key()).map_err(|e| {
                    TryFromProtoError::invalid(MultisigMemberPublicKey::PUBLIC_KEY_FIELD, e)
                })?,
            ),
            SignatureScheme::Secp256k1 => {
                Self::Secp256k1(Secp256k1PublicKey::from_bytes(value.public_key()).map_err(
                    |e| TryFromProtoError::invalid(MultisigMemberPublicKey::PUBLIC_KEY_FIELD, e),
                )?)
            }
            SignatureScheme::Secp256r1 => {
                Self::Secp256r1(Secp256r1PublicKey::from_bytes(value.public_key()).map_err(
                    |e| TryFromProtoError::invalid(MultisigMemberPublicKey::PUBLIC_KEY_FIELD, e),
                )?)
            }
            SignatureScheme::Zklogin => Self::ZkLogin(
                value
                    .zklogin
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("zklogin"))?
                    .try_into()?,
            ),
            SignatureScheme::Passkey => Self::Passkey(sui_sdk_types::PasskeyPublicKey::new(
                Secp256r1PublicKey::from_bytes(value.public_key()).map_err(|e| {
                    TryFromProtoError::invalid(MultisigMemberPublicKey::PUBLIC_KEY_FIELD, e)
                })?,
            )),
            SignatureScheme::Multisig | SignatureScheme::Bls12381 => {
                return Err(TryFromProtoError::invalid(
                    MultisigMemberPublicKey::SCHEME_FIELD,
                    "invalid MultisigMemberPublicKey scheme",
                ))
            }
        }
        .pipe(Ok)
    }
}

//
// MultisigMember
//

impl From<&sui_sdk_types::MultisigMember> for MultisigMember {
    fn from(value: &sui_sdk_types::MultisigMember) -> Self {
        Self {
            public_key: Some(value.public_key().into()),
            weight: Some(value.weight().into()),
        }
    }
}

impl TryFrom<&MultisigMember> for sui_sdk_types::MultisigMember {
    type Error = TryFromProtoError;

    fn try_from(value: &MultisigMember) -> Result<Self, Self::Error> {
        let public_key = value
            .public_key
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("public_key"))?
            .try_into()?;
        let weight = value
            .weight
            .ok_or_else(|| TryFromProtoError::missing("weight"))?
            .try_into()
            .map_err(|e| TryFromProtoError::invalid(MultisigMember::WEIGHT_FIELD, e))?;

        Ok(Self::new(public_key, weight))
    }
}

//
// MultisigCommittee
//

impl From<&sui_sdk_types::MultisigCommittee> for MultisigCommittee {
    fn from(value: &sui_sdk_types::MultisigCommittee) -> Self {
        Self {
            members: value.members().iter().map(Into::into).collect(),
            threshold: Some(value.threshold().into()),
        }
    }
}

impl TryFrom<&MultisigCommittee> for sui_sdk_types::MultisigCommittee {
    type Error = TryFromProtoError;

    fn try_from(value: &MultisigCommittee) -> Result<Self, Self::Error> {
        let members = value
            .members
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;
        let threshold = value
            .threshold
            .ok_or_else(|| TryFromProtoError::missing("threshold"))?
            .try_into()
            .map_err(|e| TryFromProtoError::invalid(MultisigCommittee::THRESHOLD_FIELD, e))?;

        Ok(Self::new(members, threshold))
    }
}

//
// MultisigMemberSignature
//

impl From<&sui_sdk_types::MultisigMemberSignature> for MultisigMemberSignature {
    fn from(value: &sui_sdk_types::MultisigMemberSignature) -> Self {
        use sui_sdk_types::MultisigMemberSignature::*;

        let mut message = Self::default();

        let scheme = match value {
            Ed25519(signature) => {
                message.signature = Some(signature.as_bytes().to_vec().into());
                SignatureScheme::Ed25519
            }
            Secp256k1(signature) => {
                message.signature = Some(signature.as_bytes().to_vec().into());
                SignatureScheme::Secp256k1
            }
            Secp256r1(signature) => {
                message.signature = Some(signature.as_bytes().to_vec().into());
                SignatureScheme::Secp256r1
            }
            ZkLogin(zklogin_id) => {
                message.zklogin = Some((**zklogin_id).clone().into());
                SignatureScheme::Zklogin
            }
            Passkey(p) => {
                message.passkey = Some(p.clone().into());
                SignatureScheme::Passkey
            }
            _ => return Self::default(),
        };

        message.set_scheme(scheme);
        message
    }
}

impl TryFrom<&MultisigMemberSignature> for sui_sdk_types::MultisigMemberSignature {
    type Error = TryFromProtoError;

    fn try_from(value: &MultisigMemberSignature) -> Result<Self, Self::Error> {
        use sui_sdk_types::Ed25519Signature;
        use sui_sdk_types::Secp256k1Signature;
        use sui_sdk_types::Secp256r1Signature;

        match value.scheme() {
            SignatureScheme::Ed25519 => Self::Ed25519(
                Ed25519Signature::from_bytes(value.signature()).map_err(|e| {
                    TryFromProtoError::invalid(MultisigMemberSignature::SIGNATURE_FIELD, e)
                })?,
            ),
            SignatureScheme::Secp256k1 => Self::Secp256k1(
                Secp256k1Signature::from_bytes(value.signature()).map_err(|e| {
                    TryFromProtoError::invalid(MultisigMemberSignature::SIGNATURE_FIELD, e)
                })?,
            ),
            SignatureScheme::Secp256r1 => Self::Secp256r1(
                Secp256r1Signature::from_bytes(value.signature()).map_err(|e| {
                    TryFromProtoError::invalid(MultisigMemberSignature::SIGNATURE_FIELD, e)
                })?,
            ),
            SignatureScheme::Zklogin => Self::ZkLogin(Box::new(
                value
                    .zklogin
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("zklogin"))?
                    .try_into()?,
            )),
            SignatureScheme::Passkey => Self::Passkey(
                value
                    .passkey
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("passkey"))?
                    .try_into()?,
            ),
            SignatureScheme::Multisig | SignatureScheme::Bls12381 => {
                return Err(TryFromProtoError::invalid(
                    MultisigMemberSignature::SCHEME_FIELD,
                    "invalid MultisigMemberSignature scheme",
                ))
            }
        }
        .pipe(Ok)
    }
}

//
// MultisigAggregatedSignature
//

impl From<&sui_sdk_types::MultisigAggregatedSignature> for MultisigAggregatedSignature {
    fn from(value: &sui_sdk_types::MultisigAggregatedSignature) -> Self {
        Self {
            signatures: value.signatures().iter().map(Into::into).collect(),
            bitmap: Some(value.bitmap().into()),
            legacy_bitmap: value.legacy_bitmap().map(|roaring| {
                let mut bitmap = Vec::new();
                roaring.serialize_into(&mut bitmap).unwrap();
                bitmap.into()
            }),
            committee: Some(value.committee().into()),
        }
    }
}

impl TryFrom<&MultisigAggregatedSignature> for sui_sdk_types::MultisigAggregatedSignature {
    type Error = TryFromProtoError;

    fn try_from(value: &MultisigAggregatedSignature) -> Result<Self, Self::Error> {
        let signatures = value
            .signatures
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;
        let bitmap = value
            .bitmap
            .ok_or_else(|| TryFromProtoError::missing("bitmap"))?
            .try_into()
            .map_err(|e| {
                TryFromProtoError::invalid(MultisigAggregatedSignature::BITMAP_FIELD, e)
            })?;
        let committee = value
            .committee
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("committee"))?
            .try_into()?;

        let mut signature = Self::new(committee, signatures, bitmap);

        if let Some(legacy_bitmap) = value.legacy_bitmap_opt() {
            let legacy_bitmap =
                sui_sdk_types::Bitmap::deserialize_from(legacy_bitmap).map_err(|e| {
                    TryFromProtoError::invalid(MultisigAggregatedSignature::LEGACY_BITMAP_FIELD, e)
                })?;
            signature.with_legacy_bitmap(legacy_bitmap);
        }

        Ok(signature)
    }
}

//
// UserSignature
//

impl From<sui_sdk_types::UserSignature> for UserSignature {
    fn from(value: sui_sdk_types::UserSignature) -> Self {
        Self::merge_from(value, &FieldMaskTree::new_wildcard())
    }
}

impl Merge<sui_sdk_types::UserSignature> for UserSignature {
    fn merge(&mut self, source: sui_sdk_types::UserSignature, mask: &FieldMaskTree) {
        use sui_sdk_types::UserSignature::*;
        use user_signature::Signature;

        if mask.contains(Self::BCS_FIELD.name) {
            self.bcs = Some(Bcs {
                name: Some("UserSignatureBytes".to_owned()),
                value: Some(source.to_bytes().into()),
            });
        }

        if mask.contains(Self::SCHEME_FIELD.name) {
            self.scheme = Some(source.scheme().to_u8().into());
        }

        match source {
            Simple(simple) => {
                if mask.contains(Self::SIMPLE_FIELD.name) {
                    self.signature = Some(Signature::Simple(simple.into()));
                }
            }
            Multisig(ref multisig) => {
                if mask.contains(Self::MULTISIG_FIELD.name) {
                    self.signature = Some(Signature::Multisig(multisig.into()));
                }
            }
            ZkLogin(zklogin) => {
                if mask.contains(Self::ZKLOGIN_FIELD.name) {
                    self.signature = Some(Signature::Zklogin((*zklogin).into()));
                }
            }
            Passkey(passkey) => {
                if mask.contains(Self::PASSKEY_FIELD.name) {
                    self.signature = Some(Signature::Passkey(passkey.into()));
                }
            }
            _ => {}
        }
    }
}

impl Merge<&UserSignature> for UserSignature {
    fn merge(&mut self, source: &UserSignature, mask: &FieldMaskTree) {
        use user_signature::Signature;

        let UserSignature {
            bcs,
            scheme,
            signature,
        } = source;

        if mask.contains(Self::BCS_FIELD.name) {
            self.bcs = bcs.clone();
        }

        if mask.contains(Self::SCHEME_FIELD.name) {
            self.scheme = *scheme;
        }

        if matches!(signature, Some(Signature::Simple(_))) && mask.contains(Self::SIMPLE_FIELD.name)
            || matches!(signature, Some(Signature::Multisig(_)))
                && mask.contains(Self::MULTISIG_FIELD.name)
            || matches!(signature, Some(Signature::Zklogin(_)))
                && mask.contains(Self::ZKLOGIN_FIELD.name)
            || matches!(signature, Some(Signature::Passkey(_)))
                && mask.contains(Self::PASSKEY_FIELD.name)
        {
            self.signature = signature.clone();
        }
    }
}

impl TryFrom<&UserSignature> for sui_sdk_types::UserSignature {
    type Error = TryFromProtoError;

    fn try_from(value: &UserSignature) -> Result<Self, Self::Error> {
        use user_signature::Signature;

        if let Some(bcs) = &value.bcs {
            if let Ok(sig) = Self::from_bytes(bcs.value()) {
                return Ok(sig);
            } else {
                return bcs
                    .deserialize()
                    .map_err(|e| TryFromProtoError::invalid(UserSignature::BCS_FIELD, e));
            }
        }

        let _scheme = value
            .scheme
            .ok_or_else(|| TryFromProtoError::missing("scheme"))?
            .pipe(SignatureScheme::try_from)
            .map_err(|e| TryFromProtoError::invalid(UserSignature::SCHEME_FIELD, e));

        match &value.signature {
            Some(Signature::Simple(simple)) => Self::Simple(simple.try_into()?),
            Some(Signature::Multisig(multisig)) => Self::Multisig(multisig.try_into()?),
            Some(Signature::Zklogin(zklogin)) => Self::ZkLogin(Box::new(zklogin.try_into()?)),
            Some(Signature::Passkey(passkey)) => Self::Passkey(passkey.try_into()?),
            None => {
                return Err(TryFromProtoError::invalid(
                    "signature",
                    "invalid or unknown signature scheme",
                ))
            }
        }
        .pipe(Ok)
    }
}

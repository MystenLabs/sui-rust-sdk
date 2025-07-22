use crate::SignatureError;
use crate::Verifier;
use sui_sdk_types::MultisigAggregatedSignature;
use sui_sdk_types::MultisigCommittee;
use sui_sdk_types::MultisigMemberPublicKey;
use sui_sdk_types::MultisigMemberSignature;
use sui_sdk_types::UserSignature;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct MultisigVerifier {
    #[cfg(feature = "zklogin")]
    zklogin_verifier: Option<crate::zklogin::ZkloginVerifier>,
}

impl MultisigVerifier {
    pub fn new() -> Self {
        Default::default()
    }

    fn verify_member_signature(
        &self,
        message: &[u8],
        member_public_key: &MultisigMemberPublicKey,
        signature: &MultisigMemberSignature,
    ) -> Result<(), SignatureError> {
        match (member_public_key, signature) {
            #[cfg(not(feature = "ed25519"))]
            (MultisigMemberPublicKey::Ed25519(_), MultisigMemberSignature::Ed25519(_)) => Err(
                SignatureError::from_source("support for ed25519 is not enabled"),
            ),
            #[cfg(feature = "ed25519")]
            (
                MultisigMemberPublicKey::Ed25519(ed25519_public_key),
                MultisigMemberSignature::Ed25519(ed25519_signature),
            ) => crate::ed25519::Ed25519VerifyingKey::new(ed25519_public_key)?
                .verify(message, ed25519_signature),

            #[cfg(not(feature = "secp256k1"))]
            (MultisigMemberPublicKey::Secp256k1(_), MultisigMemberSignature::Secp256k1(_)) => Err(
                SignatureError::from_source("support for secp256k1 is not enabled"),
            ),
            #[cfg(feature = "secp256k1")]
            (
                MultisigMemberPublicKey::Secp256k1(k1_public_key),
                MultisigMemberSignature::Secp256k1(k1_signature),
            ) => crate::secp256k1::Secp256k1VerifyingKey::new(k1_public_key)?
                .verify(message, k1_signature),

            #[cfg(not(feature = "secp256r1"))]
            (MultisigMemberPublicKey::Secp256r1(_), MultisigMemberSignature::Secp256r1(_)) => Err(
                SignatureError::from_source("support for secp256r1 is not enabled"),
            ),
            #[cfg(feature = "secp256r1")]
            (
                MultisigMemberPublicKey::Secp256r1(r1_public_key),
                MultisigMemberSignature::Secp256r1(r1_signature),
            ) => crate::secp256r1::Secp256r1VerifyingKey::new(r1_public_key)?
                .verify(message, r1_signature),

            #[cfg(not(feature = "zklogin"))]
            (MultisigMemberPublicKey::ZkLogin(_), MultisigMemberSignature::ZkLogin(_)) => Err(
                SignatureError::from_source("support for zklogin is not enabled"),
            ),
            #[cfg(feature = "zklogin")]
            (
                MultisigMemberPublicKey::ZkLogin(zklogin_identifier),
                MultisigMemberSignature::ZkLogin(zklogin_authenticator),
            ) => {
                let zklogin_verifier = self
                    .zklogin_verifier()
                    .ok_or_else(|| SignatureError::from_source("no zklogin verifier provided"))?;

                // verify that the member identifier and the authenticator match
                if zklogin_identifier
                    != &zklogin_authenticator
                        .inputs
                        .public_identifier()
                        .map_err(SignatureError::from_source)?
                {
                    return Err(SignatureError::from_source(
                        "member zklogin identifier does not match signature",
                    ));
                }

                zklogin_verifier.verify(message, zklogin_authenticator.as_ref())
            }

            #[cfg(not(feature = "passkey"))]
            (MultisigMemberPublicKey::Passkey(_), MultisigMemberSignature::Passkey(_)) => Err(
                SignatureError::from_source("support for passkey is not enabled"),
            ),
            #[cfg(feature = "passkey")]
            (
                MultisigMemberPublicKey::Passkey(passkey_public_key),
                MultisigMemberSignature::Passkey(passkey_authenticator),
            ) => {
                // Verify that the member pubkey matches the authenticator
                if passkey_public_key != &passkey_authenticator.public_key() {
                    return Err(SignatureError::from_source(
                        "member passkey public_key does not match authenticator",
                    ));
                }

                crate::passkey::PasskeyVerifier::default().verify(message, passkey_authenticator)
            }

            _ => Err(SignatureError::from_source(
                "member and signature scheme do not match",
            )),
        }
    }
}

#[cfg(feature = "zklogin")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "zklogin")))]
impl MultisigVerifier {
    pub fn with_zklogin_verifier(&mut self, zklogin_verifier: crate::zklogin::ZkloginVerifier) {
        self.zklogin_verifier = Some(zklogin_verifier);
    }

    pub fn zklogin_verifier(&self) -> Option<&crate::zklogin::ZkloginVerifier> {
        self.zklogin_verifier.as_ref()
    }

    pub fn zklogin_verifier_mut(&mut self) -> Option<&mut crate::zklogin::ZkloginVerifier> {
        self.zklogin_verifier.as_mut()
    }
}

impl Verifier<MultisigAggregatedSignature> for MultisigVerifier {
    fn verify(
        &self,
        message: &[u8],
        signature: &MultisigAggregatedSignature,
    ) -> Result<(), SignatureError> {
        if !signature.committee().is_valid() {
            return Err(SignatureError::from_source("invalid MultisigCommittee"));
        }

        if signature.signatures().len() != signature.bitmap().count_ones() as usize {
            return Err(SignatureError::from_source(
                "number of signatures does not match bitmap",
            ));
        }

        if signature.signatures().len() > signature.committee().members().len() {
            return Err(SignatureError::from_source(
                "more signatures than committee members",
            ));
        }

        let weight = BitmapIndices::new(signature.bitmap())
            .map(|member_idx| {
                signature
                    .committee()
                    .members()
                    .get(member_idx as usize)
                    .ok_or_else(|| SignatureError::from_source("invalid bitmap"))
            })
            .zip(signature.signatures())
            .map(|(maybe_member, signature)| {
                let member = maybe_member?;
                self.verify_member_signature(message, member.public_key(), signature)
                    .map(|()| member.weight() as u16)
            })
            .sum::<Result<u16, SignatureError>>()?;

        if weight >= signature.committee().threshold() {
            Ok(())
        } else {
            Err(SignatureError::from_source(
                "signature weight does not exceed threshold",
            ))
        }
    }
}

impl Verifier<UserSignature> for MultisigVerifier {
    fn verify(&self, message: &[u8], signature: &UserSignature) -> Result<(), SignatureError> {
        let UserSignature::Multisig(signature) = signature else {
            return Err(SignatureError::from_source("not a multisig signature"));
        };

        self.verify(message, signature)
    }
}

/// Interpret a bitmap of 01s as a list of indices that is set to 1s.
/// e.g. 22 = 0b10110, then the result is [1, 2, 4].
struct BitmapIndices {
    bitmap: u16,
    range: std::ops::Range<u8>,
}

impl BitmapIndices {
    pub fn new(bitmap: u16) -> Self {
        Self {
            bitmap,
            range: 0..(u16::BITS as u8),
        }
    }
}

impl Iterator for BitmapIndices {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        #[allow(clippy::while_let_on_iterator)]
        while let Some(i) = self.range.next() {
            if self.bitmap & (1 << i) != 0 {
                return Some(i);
            }
        }

        None
    }
}

/// Verifier that will verify all UserSignature variants
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UserSignatureVerifier {
    inner: MultisigVerifier,
}

impl UserSignatureVerifier {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(feature = "zklogin")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "zklogin")))]
impl UserSignatureVerifier {
    pub fn with_zklogin_verifier(&mut self, zklogin_verifier: crate::zklogin::ZkloginVerifier) {
        self.inner.with_zklogin_verifier(zklogin_verifier);
    }

    pub fn zklogin_verifier(&self) -> Option<&crate::zklogin::ZkloginVerifier> {
        self.inner.zklogin_verifier()
    }

    pub fn zklogin_verifier_mut(&mut self) -> Option<&mut crate::zklogin::ZkloginVerifier> {
        self.inner.zklogin_verifier_mut()
    }
}

impl Verifier<UserSignature> for UserSignatureVerifier {
    fn verify(&self, message: &[u8], signature: &UserSignature) -> Result<(), SignatureError> {
        match signature {
            UserSignature::Simple(simple_signature) => {
                crate::simple::SimpleVerifier.verify(message, simple_signature)
            }
            UserSignature::Multisig(multisig) => self.inner.verify(message, multisig),

            #[cfg(not(feature = "zklogin"))]
            UserSignature::ZkLogin(_) => Err(SignatureError::from_source(
                "support for zklogin is not enabled",
            )),
            #[cfg(feature = "zklogin")]
            UserSignature::ZkLogin(zklogin_authenticator) => {
                let zklogin_verifier = self
                    .zklogin_verifier()
                    .ok_or_else(|| SignatureError::from_source("no zklogin verifier provided"))?;

                zklogin_verifier.verify(message, zklogin_authenticator.as_ref())
            }
            #[cfg(not(feature = "passkey"))]
            UserSignature::Passkey(_) => Err(SignatureError::from_source(
                "support for passkey is not enabled",
            )),
            #[cfg(feature = "passkey")]
            UserSignature::Passkey(passkey_authenticator) => {
                crate::passkey::PasskeyVerifier::default().verify(message, passkey_authenticator)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MultisigAggregator {
    committee: MultisigCommittee,
    signatures: std::collections::BTreeMap<usize, MultisigMemberSignature>,
    signed_weight: u16,
    message: Vec<u8>,
    verifier: MultisigVerifier,
}

impl MultisigAggregator {
    pub fn new_with_transaction(
        committee: MultisigCommittee,
        transaction: &sui_sdk_types::Transaction,
    ) -> Self {
        Self {
            committee,
            signatures: Default::default(),
            signed_weight: 0,
            message: transaction.signing_digest().to_vec(),
            verifier: Default::default(),
        }
    }

    pub fn new_with_message(
        committee: MultisigCommittee,
        message: &sui_sdk_types::PersonalMessage<'_>,
    ) -> Self {
        Self {
            committee,
            signatures: Default::default(),
            signed_weight: 0,
            message: message.signing_digest().to_vec(),
            verifier: Default::default(),
        }
    }

    pub fn verifier(&self) -> &MultisigVerifier {
        &self.verifier
    }

    pub fn verifier_mut(&mut self) -> &mut MultisigVerifier {
        &mut self.verifier
    }

    pub fn add_signature(&mut self, signature: UserSignature) -> Result<(), SignatureError> {
        use std::collections::btree_map::Entry;

        let (public_key, signature) = multisig_pubkey_and_signature_from_user_signature(signature)?;
        let member_idx = self
            .committee
            .members()
            .iter()
            .position(|member| member.public_key() == &public_key)
            .ok_or_else(|| {
                SignatureError::from_source(
                    "provided signature does not belong to committee member",
                )
            })?;

        self.verifier()
            .verify_member_signature(&self.message, &public_key, &signature)?;

        match self.signatures.entry(member_idx) {
            Entry::Vacant(v) => {
                v.insert(signature);
            }
            Entry::Occupied(_) => {
                return Err(SignatureError::from_source(
                    "duplicate signature from same committee member",
                ))
            }
        }

        self.signed_weight += self.committee.members()[member_idx].weight() as u16;

        Ok(())
    }

    pub fn finish(&mut self) -> Result<MultisigAggregatedSignature, SignatureError> {
        if self.signed_weight < self.committee.threshold() {
            return Err(SignatureError::from_source(
                "insufficient signature weight to reach threshold",
            ));
        }

        let (signatures, bitmap) = self.signatures.clone().into_iter().fold(
            (Vec::new(), 0),
            |(mut signatures, mut bitmap), (member_idx, signature)| {
                bitmap |= 1 << member_idx;
                signatures.push(signature);
                (signatures, bitmap)
            },
        );

        Ok(MultisigAggregatedSignature::new(
            self.committee.clone(),
            signatures,
            bitmap,
        ))
    }
}

fn multisig_pubkey_and_signature_from_user_signature(
    signature: UserSignature,
) -> Result<(MultisigMemberPublicKey, MultisigMemberSignature), SignatureError> {
    use sui_sdk_types::SimpleSignature;
    match signature {
        UserSignature::Simple(SimpleSignature::Ed25519 {
            signature,
            public_key,
        }) => Ok((
            MultisigMemberPublicKey::Ed25519(public_key),
            MultisigMemberSignature::Ed25519(signature),
        )),
        UserSignature::Simple(SimpleSignature::Secp256k1 {
            signature,
            public_key,
        }) => Ok((
            MultisigMemberPublicKey::Secp256k1(public_key),
            MultisigMemberSignature::Secp256k1(signature),
        )),
        UserSignature::Simple(SimpleSignature::Secp256r1 {
            signature,
            public_key,
        }) => Ok((
            MultisigMemberPublicKey::Secp256r1(public_key),
            MultisigMemberSignature::Secp256r1(signature),
        )),

        #[cfg(not(feature = "zklogin"))]
        UserSignature::ZkLogin(_) => Err(SignatureError::from_source(
            "support for zklogin is not enabled",
        )),
        #[cfg(feature = "zklogin")]
        UserSignature::ZkLogin(zklogin_authenticator) => {
            let zklogin_identifier = zklogin_authenticator
                .inputs
                .public_identifier()
                .map_err(SignatureError::from_source)?;
            Ok((
                MultisigMemberPublicKey::ZkLogin(zklogin_identifier),
                MultisigMemberSignature::ZkLogin(zklogin_authenticator),
            ))
        }

        #[cfg(not(feature = "passkey"))]
        UserSignature::Passkey(_) => Err(SignatureError::from_source(
            "support for passkey is not enabled",
        )),
        #[cfg(feature = "passkey")]
        UserSignature::Passkey(passkey_authenticator) => Ok((
            MultisigMemberPublicKey::Passkey(passkey_authenticator.public_key()),
            MultisigMemberSignature::Passkey(passkey_authenticator),
        )),

        UserSignature::Multisig(_) => Err(SignatureError::from_source("invalid siganture scheme")),
    }
}

use crate::SignatureError;
use signature::Verifier;
use sui_sdk_types::SimpleSignature;
use sui_sdk_types::UserSignature;

pub struct SimpleVerifier;

impl Verifier<SimpleSignature> for SimpleVerifier {
    #[allow(unused_variables)]
    fn verify(&self, message: &[u8], signature: &SimpleSignature) -> Result<(), SignatureError> {
        match signature {
            #[cfg(feature = "ed25519")]
            SimpleSignature::Ed25519 {
                signature,
                public_key,
            } => {
                let verifying_key = crate::ed25519::Ed25519VerifyingKey::new(public_key)?;
                verifying_key.verify(message, signature)
            }
            #[cfg(not(feature = "ed25519"))]
            SimpleSignature::Ed25519 { .. } => Err(SignatureError::from_source(
                "support for ed25519 is not enabled",
            )),
            #[cfg(feature = "secp256k1")]
            SimpleSignature::Secp256k1 {
                signature,
                public_key,
            } => {
                let verifying_key = crate::secp256k1::Secp256k1VerifyingKey::new(public_key)?;
                verifying_key.verify(message, signature)
            }
            #[cfg(not(feature = "secp256k1"))]
            SimpleSignature::Secp256k1 { .. } => Err(SignatureError::from_source(
                "support for secp256k1 is not enabled",
            )),
            #[cfg(feature = "secp256r1")]
            SimpleSignature::Secp256r1 {
                signature,
                public_key,
            } => {
                let verifying_key = crate::secp256r1::Secp256r1VerifyingKey::new(public_key)?;
                verifying_key.verify(message, signature)
            }
            #[cfg(not(feature = "secp256r1"))]
            SimpleSignature::Secp256r1 { .. } => Err(SignatureError::from_source(
                "support for secp256r1 is not enabled",
            )),
            _ => Err(SignatureError::from_source("unknown signature scheme")),
        }
    }
}

impl Verifier<UserSignature> for SimpleVerifier {
    fn verify(&self, message: &[u8], signature: &UserSignature) -> Result<(), SignatureError> {
        let UserSignature::Simple(signature) = signature else {
            return Err(SignatureError::from_source("not a simple signature"));
        };

        <Self as Verifier<SimpleSignature>>::verify(self, message, signature)
    }
}

#[cfg(any(feature = "ed25519", feature = "secp256r1", feature = "secp256k1",))]
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(feature = "ed25519", feature = "secp256r1", feature = "secp256k1",)))
)]
#[rustfmt::skip]
pub use keypair::{SimpleKeypair, SimpleVerifiyingKey};

#[cfg(any(feature = "ed25519", feature = "secp256r1", feature = "secp256k1",))]
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(feature = "ed25519", feature = "secp256r1", feature = "secp256k1",)))
)]
mod keypair {
    use crate::SignatureError;
    use signature::Signer;
    use signature::Verifier;
    use sui_sdk_types::MultisigMemberPublicKey;
    use sui_sdk_types::SignatureScheme;
    use sui_sdk_types::SimpleSignature;
    use sui_sdk_types::UserSignature;

    #[derive(Debug, Clone)]
    pub struct SimpleKeypair {
        inner: InnerKeypair,
    }

    #[derive(Debug, Clone)]
    enum InnerKeypair {
        #[cfg(feature = "ed25519")]
        Ed25519(crate::ed25519::Ed25519PrivateKey),
        #[cfg(feature = "secp256k1")]
        Secp256k1(crate::secp256k1::Secp256k1PrivateKey),
        #[cfg(feature = "secp256r1")]
        Secp256r1(crate::secp256r1::Secp256r1PrivateKey),
    }

    impl SimpleKeypair {
        pub fn scheme(&self) -> SignatureScheme {
            match &self.inner {
                #[cfg(feature = "ed25519")]
                InnerKeypair::Ed25519(private_key) => private_key.scheme(),
                #[cfg(feature = "secp256k1")]
                InnerKeypair::Secp256k1(private_key) => private_key.scheme(),
                #[cfg(feature = "secp256r1")]
                InnerKeypair::Secp256r1(private_key) => private_key.scheme(),
            }
        }

        pub fn verifying_key(&self) -> SimpleVerifiyingKey {
            let verifying_key = match &self.inner {
                #[cfg(feature = "ed25519")]
                InnerKeypair::Ed25519(private_key) => {
                    InnerVerifyingKey::Ed25519(private_key.verifying_key())
                }
                #[cfg(feature = "secp256k1")]
                InnerKeypair::Secp256k1(private_key) => {
                    InnerVerifyingKey::Secp256k1(private_key.verifying_key())
                }
                #[cfg(feature = "secp256r1")]
                InnerKeypair::Secp256r1(private_key) => {
                    InnerVerifyingKey::Secp256r1(private_key.verifying_key())
                }
            };

            SimpleVerifiyingKey {
                inner: verifying_key,
            }
        }

        pub fn public_key(&self) -> MultisigMemberPublicKey {
            self.verifying_key().public_key()
        }

        #[cfg(feature = "pem")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "pem")))]
        /// Deserialize PKCS#8 private key from ASN.1 DER-encoded data (binary format).
        pub fn from_der(bytes: &[u8]) -> Result<Self, SignatureError> {
            let private_key =
                pkcs8::PrivateKeyInfo::try_from(bytes).map_err(SignatureError::from_source)?;

            match private_key
                .algorithm
                .oids()
                .map_err(SignatureError::from_source)?
            {
                #[cfg(feature = "ed25519")]
                (ed25519_dalek::pkcs8::ALGORITHM_OID, None) => private_key
                    .try_into()
                    .map(crate::ed25519::Ed25519PrivateKey::from_dalek)
                    .map(InnerKeypair::Ed25519)
                    .map_err(SignatureError::from_source),

                #[cfg(feature = "secp256r1")]
                (
                    p256::elliptic_curve::ALGORITHM_OID,
                    Some(<p256::NistP256 as pkcs8::AssociatedOid>::OID),
                ) => private_key
                    .try_into()
                    .map(crate::secp256r1::Secp256r1PrivateKey::from_p256)
                    .map(InnerKeypair::Secp256r1)
                    .map_err(SignatureError::from_source),

                #[cfg(feature = "secp256k1")]
                (
                    k256::elliptic_curve::ALGORITHM_OID,
                    Some(<k256::Secp256k1 as pkcs8::AssociatedOid>::OID),
                ) => private_key
                    .try_into()
                    .map(crate::secp256k1::Secp256k1PrivateKey::from_k256)
                    .map(InnerKeypair::Secp256k1)
                    .map_err(SignatureError::from_source),

                _ => Err(SignatureError::from_source(
                    "unsupported or invalid private key type",
                )),
            }
            .map(|inner| Self { inner })
        }

        #[cfg(feature = "pem")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "pem")))]
        /// Serialize this private key as DER-encoded PKCS#8
        pub fn to_der(&self) -> Result<Vec<u8>, SignatureError> {
            match &self.inner {
                #[cfg(feature = "ed25519")]
                InnerKeypair::Ed25519(private_key) => private_key.to_der(),
                #[cfg(feature = "secp256k1")]
                InnerKeypair::Secp256k1(private_key) => private_key.to_der(),
                #[cfg(feature = "secp256r1")]
                InnerKeypair::Secp256r1(private_key) => private_key.to_der(),
            }
        }

        #[cfg(feature = "pem")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "pem")))]
        /// Deserialize PKCS#8-encoded private key from PEM.
        pub fn from_pem(s: &str) -> Result<Self, SignatureError> {
            use pkcs8::der::pem::PemLabel;

            let (label, doc) =
                pkcs8::SecretDocument::from_pem(s).map_err(SignatureError::from_source)?;
            pkcs8::PrivateKeyInfo::validate_pem_label(label)
                .map_err(SignatureError::from_source)?;
            Self::from_der(doc.as_bytes())
        }

        #[cfg(feature = "pem")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "pem")))]
        /// Serialize this private key as DER-encoded PKCS#8
        pub fn to_pem(&self) -> Result<String, SignatureError> {
            match &self.inner {
                #[cfg(feature = "ed25519")]
                InnerKeypair::Ed25519(private_key) => private_key.to_pem(),
                #[cfg(feature = "secp256k1")]
                InnerKeypair::Secp256k1(private_key) => private_key.to_pem(),
                #[cfg(feature = "secp256r1")]
                InnerKeypair::Secp256r1(private_key) => private_key.to_pem(),
            }
        }

        #[cfg(feature = "bech32")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "bech32")))]
        /// Decode a Bech32 `suiprivkey` string produced by the Sui CLI.
        ///
        /// The leading flag byte selects the scheme. Only the simple schemes
        /// (Ed25519, Secp256k1, Secp256r1) are accepted, matching the
        /// upstream `sui-types` parser.
        pub fn from_suiprivkey(s: &str) -> Result<Self, SignatureError> {
            let (scheme, key) = crate::suipriv::decode(s)?;
            let inner = match scheme {
                #[cfg(feature = "ed25519")]
                SignatureScheme::Ed25519 => {
                    let bytes: [u8; crate::ed25519::Ed25519PrivateKey::LENGTH] =
                        key.try_into().map_err(|_: Vec<u8>| {
                            SignatureError::from_source(
                                "suipriv key has invalid length for ed25519",
                            )
                        })?;
                    InnerKeypair::Ed25519(crate::ed25519::Ed25519PrivateKey::new(bytes))
                }
                #[cfg(feature = "secp256k1")]
                SignatureScheme::Secp256k1 => {
                    let bytes: [u8; crate::secp256k1::Secp256k1PrivateKey::LENGTH] =
                        key.try_into().map_err(|_: Vec<u8>| {
                            SignatureError::from_source(
                                "suipriv key has invalid length for secp256k1",
                            )
                        })?;
                    InnerKeypair::Secp256k1(crate::secp256k1::Secp256k1PrivateKey::new(bytes)?)
                }
                #[cfg(feature = "secp256r1")]
                SignatureScheme::Secp256r1 => {
                    let bytes: [u8; crate::secp256r1::Secp256r1PrivateKey::LENGTH] =
                        key.try_into().map_err(|_: Vec<u8>| {
                            SignatureError::from_source(
                                "suipriv key has invalid length for secp256r1",
                            )
                        })?;
                    InnerKeypair::Secp256r1(crate::secp256r1::Secp256r1PrivateKey::new(bytes))
                }
                other => {
                    return Err(SignatureError::from_source(format!(
                        "unsupported scheme `{}` in suipriv encoding",
                        other.name(),
                    )));
                }
            };
            Ok(Self { inner })
        }

        #[cfg(feature = "bech32")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "bech32")))]
        /// Encode this private key as a Bech32 `suiprivkey` string.
        pub fn to_suiprivkey(&self) -> Result<String, SignatureError> {
            match &self.inner {
                #[cfg(feature = "ed25519")]
                InnerKeypair::Ed25519(private_key) => private_key.to_suiprivkey(),
                #[cfg(feature = "secp256k1")]
                InnerKeypair::Secp256k1(private_key) => private_key.to_suiprivkey(),
                #[cfg(feature = "secp256r1")]
                InnerKeypair::Secp256r1(private_key) => private_key.to_suiprivkey(),
            }
        }
    }

    impl Signer<SimpleSignature> for SimpleKeypair {
        fn try_sign(&self, message: &[u8]) -> Result<SimpleSignature, SignatureError> {
            match &self.inner {
                #[cfg(feature = "ed25519")]
                InnerKeypair::Ed25519(private_key) => private_key.try_sign(message),
                #[cfg(feature = "secp256k1")]
                InnerKeypair::Secp256k1(private_key) => private_key.try_sign(message),
                #[cfg(feature = "secp256r1")]
                InnerKeypair::Secp256r1(private_key) => private_key.try_sign(message),
            }
        }
    }

    impl Signer<UserSignature> for SimpleKeypair {
        fn try_sign(&self, msg: &[u8]) -> Result<UserSignature, SignatureError> {
            <Self as Signer<SimpleSignature>>::try_sign(self, msg).map(UserSignature::Simple)
        }
    }

    #[cfg(feature = "ed25519")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "ed25519")))]
    impl From<crate::ed25519::Ed25519PrivateKey> for SimpleKeypair {
        fn from(private_key: crate::ed25519::Ed25519PrivateKey) -> Self {
            Self {
                inner: InnerKeypair::Ed25519(private_key),
            }
        }
    }

    #[cfg(feature = "secp256r1")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "secp256r1")))]
    impl From<crate::secp256r1::Secp256r1PrivateKey> for SimpleKeypair {
        fn from(private_key: crate::secp256r1::Secp256r1PrivateKey) -> Self {
            Self {
                inner: InnerKeypair::Secp256r1(private_key),
            }
        }
    }

    #[cfg(feature = "secp256k1")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "secp256k1")))]
    impl From<crate::secp256k1::Secp256k1PrivateKey> for SimpleKeypair {
        fn from(private_key: crate::secp256k1::Secp256k1PrivateKey) -> Self {
            Self {
                inner: InnerKeypair::Secp256k1(private_key),
            }
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct SimpleVerifiyingKey {
        inner: InnerVerifyingKey,
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    enum InnerVerifyingKey {
        #[cfg(feature = "ed25519")]
        Ed25519(crate::ed25519::Ed25519VerifyingKey),
        #[cfg(feature = "secp256k1")]
        Secp256k1(crate::secp256k1::Secp256k1VerifyingKey),
        #[cfg(feature = "secp256r1")]
        Secp256r1(crate::secp256r1::Secp256r1VerifyingKey),
    }

    impl SimpleVerifiyingKey {
        pub fn scheme(&self) -> SignatureScheme {
            match &self.inner {
                #[cfg(feature = "ed25519")]
                InnerVerifyingKey::Ed25519(verifying_key) => verifying_key.public_key().scheme(),
                #[cfg(feature = "secp256k1")]
                InnerVerifyingKey::Secp256k1(verifying_key) => verifying_key.public_key().scheme(),
                #[cfg(feature = "secp256r1")]
                InnerVerifyingKey::Secp256r1(verifying_key) => verifying_key.public_key().scheme(),
            }
        }

        pub fn public_key(&self) -> MultisigMemberPublicKey {
            match &self.inner {
                #[cfg(feature = "ed25519")]
                InnerVerifyingKey::Ed25519(verifying_key) => {
                    MultisigMemberPublicKey::Ed25519(verifying_key.public_key())
                }
                #[cfg(feature = "secp256k1")]
                InnerVerifyingKey::Secp256k1(verifying_key) => {
                    MultisigMemberPublicKey::Secp256k1(verifying_key.public_key())
                }
                #[cfg(feature = "secp256r1")]
                InnerVerifyingKey::Secp256r1(verifying_key) => {
                    MultisigMemberPublicKey::Secp256r1(verifying_key.public_key())
                }
            }
        }

        #[cfg(feature = "pem")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "pem")))]
        /// Deserialize public key from ASN.1 DER-encoded data (binary format).
        pub fn from_der(bytes: &[u8]) -> Result<Self, SignatureError> {
            let public_key = pkcs8::SubjectPublicKeyInfoRef::try_from(bytes)
                .map_err(SignatureError::from_source)?;

            match public_key
                .algorithm
                .oids()
                .map_err(SignatureError::from_source)?
            {
                #[cfg(feature = "ed25519")]
                (ed25519_dalek::pkcs8::ALGORITHM_OID, None) => public_key
                    .try_into()
                    .map(crate::ed25519::Ed25519VerifyingKey::from_dalek)
                    .map(InnerVerifyingKey::Ed25519)
                    .map_err(SignatureError::from_source),

                #[cfg(feature = "secp256r1")]
                (
                    p256::elliptic_curve::ALGORITHM_OID,
                    Some(<p256::NistP256 as pkcs8::AssociatedOid>::OID),
                ) => public_key
                    .try_into()
                    .map(crate::secp256r1::Secp256r1VerifyingKey::from_p256)
                    .map(InnerVerifyingKey::Secp256r1)
                    .map_err(SignatureError::from_source),

                #[cfg(feature = "secp256k1")]
                (
                    k256::elliptic_curve::ALGORITHM_OID,
                    Some(<k256::Secp256k1 as pkcs8::AssociatedOid>::OID),
                ) => public_key
                    .try_into()
                    .map(crate::secp256k1::Secp256k1VerifyingKey::from_k256)
                    .map(InnerVerifyingKey::Secp256k1)
                    .map_err(SignatureError::from_source),

                _ => Err(SignatureError::from_source(
                    "unsupported or invalid public key type",
                )),
            }
            .map(|inner| Self { inner })
        }

        #[cfg(feature = "pem")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "pem")))]
        /// Serialize this public key as DER-encoded data
        pub fn to_der(&self) -> Result<Vec<u8>, SignatureError> {
            match &self.inner {
                #[cfg(feature = "ed25519")]
                InnerVerifyingKey::Ed25519(verifying_key) => verifying_key.to_der(),
                #[cfg(feature = "secp256k1")]
                InnerVerifyingKey::Secp256k1(verifying_key) => verifying_key.to_der(),
                #[cfg(feature = "secp256r1")]
                InnerVerifyingKey::Secp256r1(verifying_key) => verifying_key.to_der(),
            }
        }

        #[cfg(feature = "pem")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "pem")))]
        /// Deserialize public key from PEM.
        pub fn from_pem(s: &str) -> Result<Self, SignatureError> {
            use pkcs8::der::pem::PemLabel;

            let (label, doc) = pkcs8::Document::from_pem(s).map_err(SignatureError::from_source)?;
            pkcs8::SubjectPublicKeyInfoRef::validate_pem_label(label)
                .map_err(SignatureError::from_source)?;
            Self::from_der(doc.as_bytes())
        }

        #[cfg(feature = "pem")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "pem")))]
        /// Serialize this public key as PEM
        pub fn to_pem(&self) -> Result<String, SignatureError> {
            match &self.inner {
                #[cfg(feature = "ed25519")]
                InnerVerifyingKey::Ed25519(verifying_key) => verifying_key.to_pem(),
                #[cfg(feature = "secp256k1")]
                InnerVerifyingKey::Secp256k1(verifying_key) => verifying_key.to_pem(),
                #[cfg(feature = "secp256r1")]
                InnerVerifyingKey::Secp256r1(verifying_key) => verifying_key.to_pem(),
            }
        }
    }

    impl Verifier<SimpleSignature> for SimpleVerifiyingKey {
        fn verify(
            &self,
            message: &[u8],
            signature: &SimpleSignature,
        ) -> Result<(), SignatureError> {
            match &self.inner {
                #[cfg(feature = "ed25519")]
                InnerVerifyingKey::Ed25519(verifying_key) => {
                    verifying_key.verify(message, signature)
                }
                #[cfg(feature = "secp256k1")]
                InnerVerifyingKey::Secp256k1(verifying_key) => {
                    verifying_key.verify(message, signature)
                }
                #[cfg(feature = "secp256r1")]
                InnerVerifyingKey::Secp256r1(verifying_key) => {
                    verifying_key.verify(message, signature)
                }
            }
        }
    }

    impl Verifier<UserSignature> for SimpleVerifiyingKey {
        fn verify(&self, message: &[u8], signature: &UserSignature) -> Result<(), SignatureError> {
            let UserSignature::Simple(signature) = signature else {
                return Err(SignatureError::from_source("not a simple signature"));
            };

            <Self as Verifier<SimpleSignature>>::verify(self, message, signature)
        }
    }

    #[cfg(feature = "ed25519")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "ed25519")))]
    impl From<crate::ed25519::Ed25519VerifyingKey> for SimpleVerifiyingKey {
        fn from(verifying_key: crate::ed25519::Ed25519VerifyingKey) -> Self {
            Self {
                inner: InnerVerifyingKey::Ed25519(verifying_key),
            }
        }
    }

    #[cfg(feature = "secp256r1")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "secp256r1")))]
    impl From<crate::secp256r1::Secp256r1VerifyingKey> for SimpleVerifiyingKey {
        fn from(verifying_key: crate::secp256r1::Secp256r1VerifyingKey) -> Self {
            Self {
                inner: InnerVerifyingKey::Secp256r1(verifying_key),
            }
        }
    }

    #[cfg(feature = "secp256k1")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "secp256k1")))]
    impl From<crate::secp256k1::Secp256k1VerifyingKey> for SimpleVerifiyingKey {
        fn from(verifying_key: crate::secp256k1::Secp256k1VerifyingKey) -> Self {
            Self {
                inner: InnerVerifyingKey::Secp256k1(verifying_key),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ed25519::Ed25519PrivateKey;
    use crate::ed25519::Ed25519VerifyingKey;
    use crate::secp256k1::Secp256k1PrivateKey;
    use crate::secp256k1::Secp256k1VerifyingKey;
    use crate::secp256r1::Secp256r1PrivateKey;
    use crate::secp256r1::Secp256r1VerifyingKey;
    use test_strategy::proptest;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[proptest]
    fn ed25519_pem_der(signer: Ed25519PrivateKey) {
        //
        // Private Key
        //
        let public_key = signer.public_key();
        let ed25519_der = signer.to_der().unwrap();
        let ed25519_pem = signer.to_pem().unwrap();

        // der and pem round-trip
        let from_der = Ed25519PrivateKey::from_der(&ed25519_der).unwrap();
        assert_eq!(from_der.public_key(), public_key);
        let from_pem = Ed25519PrivateKey::from_pem(&ed25519_pem).unwrap();
        assert_eq!(from_pem.public_key(), public_key);

        // der and pem bytes don't convert to secp256r1 or secp256k1
        Secp256r1PrivateKey::from_der(&ed25519_der).unwrap_err();
        Secp256r1PrivateKey::from_pem(&ed25519_pem).unwrap_err();
        Secp256k1PrivateKey::from_der(&ed25519_der).unwrap_err();
        Secp256k1PrivateKey::from_pem(&ed25519_pem).unwrap_err();

        // SimpleKeypair parses
        let keypair_from_der = SimpleKeypair::from_der(&ed25519_der).unwrap();
        assert_eq!(ed25519_der, keypair_from_der.to_der().unwrap());
        let keypair_from_pem = SimpleKeypair::from_pem(&ed25519_pem).unwrap();
        assert_eq!(ed25519_pem, keypair_from_pem.to_pem().unwrap());

        //
        // Verifying Key
        //
        let verifying_key = signer.verifying_key();
        let der = verifying_key.to_der().unwrap();
        let pem = verifying_key.to_pem().unwrap();

        // der and pem round-trip
        let from_der = Ed25519VerifyingKey::from_der(&der).unwrap();
        assert_eq!(from_der.public_key(), public_key);
        let from_pem = Ed25519VerifyingKey::from_pem(&pem).unwrap();
        assert_eq!(from_pem.public_key(), public_key);

        // der and pem bytes don't convert to secp256r1 or secp256k1
        Secp256r1VerifyingKey::from_der(&der).unwrap_err();
        Secp256r1VerifyingKey::from_pem(&pem).unwrap_err();
        Secp256k1VerifyingKey::from_der(&der).unwrap_err();
        Secp256k1VerifyingKey::from_pem(&pem).unwrap_err();

        // SimpleKeypair parses
        let from_der = SimpleVerifiyingKey::from_der(&der).unwrap();
        assert_eq!(der, from_der.to_der().unwrap());
        let from_pem = SimpleVerifiyingKey::from_pem(&pem).unwrap();
        assert_eq!(pem, from_pem.to_pem().unwrap());
    }

    #[proptest]
    fn secp256r1_pem_der(signer: Secp256r1PrivateKey) {
        //
        // Private Key
        //
        let public_key = signer.public_key();
        let secp256r1_der = signer.to_der().unwrap();
        let secp256r1_pem = signer.to_pem().unwrap();

        // der and pem round-trip
        let from_der = Secp256r1PrivateKey::from_der(&secp256r1_der).unwrap();
        assert_eq!(from_der.public_key(), public_key);
        let from_pem = Secp256r1PrivateKey::from_pem(&secp256r1_pem).unwrap();
        assert_eq!(from_pem.public_key(), public_key);

        // der and pem bytes don't convert to ed25519 or secp256k1
        Ed25519PrivateKey::from_der(&secp256r1_der).unwrap_err();
        Ed25519PrivateKey::from_pem(&secp256r1_pem).unwrap_err();
        Secp256k1PrivateKey::from_der(&secp256r1_der).unwrap_err();
        Secp256k1PrivateKey::from_pem(&secp256r1_pem).unwrap_err();

        // SimpleKeypair parses
        let keypair_from_der = SimpleKeypair::from_der(&secp256r1_der).unwrap();
        assert_eq!(secp256r1_der, keypair_from_der.to_der().unwrap());
        let keypair_from_pem = SimpleKeypair::from_pem(&secp256r1_pem).unwrap();
        assert_eq!(secp256r1_pem, keypair_from_pem.to_pem().unwrap());

        //
        // Verifying Key
        //
        let verifying_key = signer.verifying_key();
        let der = verifying_key.to_der().unwrap();
        let pem = verifying_key.to_pem().unwrap();

        // der and pem round-trip
        let from_der = Secp256r1VerifyingKey::from_der(&der).unwrap();
        assert_eq!(from_der.public_key(), public_key);
        let from_pem = Secp256r1VerifyingKey::from_pem(&pem).unwrap();
        assert_eq!(from_pem.public_key(), public_key);

        // der and pem bytes don't convert to ed25519 or secp256k1
        Ed25519VerifyingKey::from_der(&der).unwrap_err();
        Ed25519VerifyingKey::from_pem(&pem).unwrap_err();
        Secp256k1VerifyingKey::from_der(&der).unwrap_err();
        Secp256k1VerifyingKey::from_pem(&pem).unwrap_err();

        // SimpleKeypair parses
        let from_der = SimpleVerifiyingKey::from_der(&der).unwrap();
        assert_eq!(der, from_der.to_der().unwrap());
        let from_pem = SimpleVerifiyingKey::from_pem(&pem).unwrap();
        assert_eq!(pem, from_pem.to_pem().unwrap());
    }

    #[proptest]
    fn secp256k1_pem_der(signer: Secp256k1PrivateKey) {
        //
        // Private Key
        //
        let public_key = signer.public_key();
        let secp256k1_der = signer.to_der().unwrap();
        let secp256k1_pem = signer.to_pem().unwrap();

        // der and pem round-trip
        let from_der = Secp256k1PrivateKey::from_der(&secp256k1_der).unwrap();
        assert_eq!(from_der.public_key(), public_key);
        let from_pem = Secp256k1PrivateKey::from_pem(&secp256k1_pem).unwrap();
        assert_eq!(from_pem.public_key(), public_key);

        // der and pem bytes don't convert to secp256r1 or ed25519
        Ed25519PrivateKey::from_der(&secp256k1_der).unwrap_err();
        Ed25519PrivateKey::from_pem(&secp256k1_pem).unwrap_err();
        Secp256r1PrivateKey::from_der(&secp256k1_der).unwrap_err();
        Secp256r1PrivateKey::from_pem(&secp256k1_pem).unwrap_err();

        // SimpleKeypair parses
        let keypair_from_der = SimpleKeypair::from_der(&secp256k1_der).unwrap();
        assert_eq!(secp256k1_der, keypair_from_der.to_der().unwrap());
        let keypair_from_pem = SimpleKeypair::from_pem(&secp256k1_pem).unwrap();
        assert_eq!(secp256k1_pem, keypair_from_pem.to_pem().unwrap());

        //
        // Verifying Key
        //
        let verifying_key = signer.verifying_key();
        let der = verifying_key.to_der().unwrap();
        let pem = verifying_key.to_pem().unwrap();

        // der and pem round-trip
        let from_der = Secp256k1VerifyingKey::from_der(&der).unwrap();
        assert_eq!(from_der.public_key(), public_key);
        let from_pem = Secp256k1VerifyingKey::from_pem(&pem).unwrap();
        assert_eq!(from_pem.public_key(), public_key);

        // der and pem bytes don't convert to ed25519 or secp256r1
        Ed25519VerifyingKey::from_der(&der).unwrap_err();
        Ed25519VerifyingKey::from_pem(&pem).unwrap_err();
        Secp256r1VerifyingKey::from_der(&der).unwrap_err();
        Secp256r1VerifyingKey::from_pem(&pem).unwrap_err();

        // SimpleKeypair parses
        let from_der = SimpleVerifiyingKey::from_der(&der).unwrap();
        assert_eq!(der, from_der.to_der().unwrap());
        let from_pem = SimpleVerifiyingKey::from_pem(&pem).unwrap();
        assert_eq!(pem, from_pem.to_pem().unwrap());
    }

    // Round-trip and rejection tests for the suiprivkey Bech32 format.
    //
    // These mirror the format produced by the Sui CLI and the upstream
    // `sui-types` crate. `EXPECTED_ED25519_VECTOR` is taken directly from
    // `crates/sui-types/src/unit_tests/crypto_tests.rs` in the main sui repo
    // and is included as a regression vector against any future encoding
    // drift.
    #[cfg(feature = "bech32")]
    mod bech32 {
        use super::*;
        use sui_sdk_types::SignatureScheme;

        // Upstream test vector: `Ed25519KeyPair::generate(&mut StdRng::from_seed([0; 32]))`
        // encoded with `SuiKeyPair::encode()` produces this string. The leading
        // flag byte is 0x00 (Ed25519); the remaining 32 bytes are the private
        // key.
        const UPSTREAM_ED25519_SUIPRIVKEY: &str =
            "suiprivkey1qzdlfxn2qa2lj5uprl8pyhexs02sg2wrhdy7qaq50cqgnffw4c2477kg9h3";

        #[proptest]
        fn ed25519_round_trip(signer: Ed25519PrivateKey) {
            let encoded = signer.to_suiprivkey().unwrap();
            let decoded = Ed25519PrivateKey::from_suiprivkey(&encoded).unwrap();
            assert_eq!(decoded.public_key(), signer.public_key());

            // SimpleKeypair dispatch agrees.
            let keypair = SimpleKeypair::from_suiprivkey(&encoded).unwrap();
            assert_eq!(keypair.scheme(), signer.scheme());
            assert_eq!(encoded, keypair.to_suiprivkey().unwrap());
        }

        #[proptest]
        fn secp256k1_round_trip(signer: Secp256k1PrivateKey) {
            let encoded = signer.to_suiprivkey().unwrap();
            let decoded = Secp256k1PrivateKey::from_suiprivkey(&encoded).unwrap();
            assert_eq!(decoded.public_key(), signer.public_key());

            let keypair = SimpleKeypair::from_suiprivkey(&encoded).unwrap();
            assert_eq!(keypair.scheme(), signer.scheme());
            assert_eq!(encoded, keypair.to_suiprivkey().unwrap());
        }

        #[proptest]
        fn secp256r1_round_trip(signer: Secp256r1PrivateKey) {
            let encoded = signer.to_suiprivkey().unwrap();
            let decoded = Secp256r1PrivateKey::from_suiprivkey(&encoded).unwrap();
            assert_eq!(decoded.public_key(), signer.public_key());

            let keypair = SimpleKeypair::from_suiprivkey(&encoded).unwrap();
            assert_eq!(keypair.scheme(), signer.scheme());
            assert_eq!(encoded, keypair.to_suiprivkey().unwrap());
        }

        #[test]
        fn upstream_ed25519_vector_round_trips() {
            let keypair = SimpleKeypair::from_suiprivkey(UPSTREAM_ED25519_SUIPRIVKEY).unwrap();
            assert_eq!(keypair.scheme(), SignatureScheme::Ed25519);
            assert_eq!(
                keypair.to_suiprivkey().unwrap(),
                UPSTREAM_ED25519_SUIPRIVKEY
            );

            // Per-scheme decoder accepts it too.
            Ed25519PrivateKey::from_suiprivkey(UPSTREAM_ED25519_SUIPRIVKEY).unwrap();
            // Wrong-scheme per-scheme decoders reject it.
            Secp256k1PrivateKey::from_suiprivkey(UPSTREAM_ED25519_SUIPRIVKEY).unwrap_err();
            Secp256r1PrivateKey::from_suiprivkey(UPSTREAM_ED25519_SUIPRIVKEY).unwrap_err();
        }

        #[test]
        fn rejects_wrong_hrp() {
            // Same payload as the upstream Ed25519 vector but encoded with a
            // different HRP — must fail.
            let bytes = ::bech32::primitives::decode::CheckedHrpstring::new::<::bech32::Bech32>(
                UPSTREAM_ED25519_SUIPRIVKEY,
            )
            .unwrap()
            .byte_iter()
            .collect::<Vec<_>>();
            let wrong_hrp = ::bech32::Hrp::parse("notsui").unwrap();
            let encoded = ::bech32::encode::<::bech32::Bech32>(wrong_hrp, &bytes).unwrap();

            SimpleKeypair::from_suiprivkey(&encoded).unwrap_err();
            Ed25519PrivateKey::from_suiprivkey(&encoded).unwrap_err();
        }

        #[test]
        fn rejects_bech32m_checksum() {
            // Re-encode the upstream Ed25519 payload using the Bech32m
            // checksum variant. A correctly-implemented decoder must reject
            // this, since the suipriv format uses BIP-173 Bech32 only.
            let bytes = ::bech32::primitives::decode::CheckedHrpstring::new::<::bech32::Bech32>(
                UPSTREAM_ED25519_SUIPRIVKEY,
            )
            .unwrap()
            .byte_iter()
            .collect::<Vec<_>>();
            let hrp = ::bech32::Hrp::parse("suiprivkey").unwrap();
            let bech32m = ::bech32::encode::<::bech32::Bech32m>(hrp, &bytes).unwrap();
            assert_ne!(bech32m, UPSTREAM_ED25519_SUIPRIVKEY);

            SimpleKeypair::from_suiprivkey(&bech32m).unwrap_err();
            Ed25519PrivateKey::from_suiprivkey(&bech32m).unwrap_err();
        }
    }
}

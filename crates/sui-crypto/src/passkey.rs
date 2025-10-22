use crate::SignatureError;
use crate::secp256r1::Secp256r1VerifyingKey;
use signature::Verifier;
use sui_sdk_types::PasskeyAuthenticator;
use sui_sdk_types::SimpleSignature;
use sui_sdk_types::UserSignature;

#[derive(Default, Clone, Debug)]
pub struct PasskeyVerifier {}

impl PasskeyVerifier {
    pub fn new() -> Self {
        Self {}
    }
}

impl Verifier<PasskeyAuthenticator> for PasskeyVerifier {
    fn verify(
        &self,
        message: &[u8],
        authenticator: &PasskeyAuthenticator,
    ) -> Result<(), SignatureError> {
        let SimpleSignature::Secp256r1 {
            signature,
            public_key,
        } = authenticator.signature()
        else {
            return Err(SignatureError::from_source("not a secp256r1 signature"));
        };

        if message != authenticator.challenge() {
            return Err(SignatureError::from_source(
                "passkey challenge does not match expected message",
            ));
        }

        // Construct passkey signing message = authenticator_data || sha256(client_data_json).
        let mut message = authenticator.authenticator_data().to_owned();
        let client_data_hash = {
            use sha2::Digest;

            let mut hasher = sha2::Sha256::new();
            hasher.update(authenticator.client_data_json().as_bytes());
            hasher.finalize()
        };
        message.extend_from_slice(&client_data_hash);

        let verifying_key = Secp256r1VerifyingKey::new(&public_key)?;

        verifying_key.verify(&message, &signature)
    }
}

impl Verifier<UserSignature> for PasskeyVerifier {
    fn verify(&self, message: &[u8], signature: &UserSignature) -> Result<(), SignatureError> {
        let UserSignature::Passkey(authenticator) = signature else {
            return Err(SignatureError::from_source("not a passkey authenticator"));
        };

        <Self as Verifier<PasskeyAuthenticator>>::verify(self, message, authenticator)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::SuiVerifier;
    use sui_sdk_types::Transaction;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[test]
    fn transaction_signing_fixture() {
        let transaction = "AAAAACdZawPnpJRjmVcwDu6xrIumtq5NLO+6GHbs0iGdCoD7AQ0T0TolicYERdSvyCRjSSduDZLbSpBsZBoib+lF48EBcgAAAAAAAAAgpQr/Mudl9BdzyBdkbqTlqBw4/aJ21kAD/jpJKa05im4nWWsD56SUY5lXMA7usayLprauTSzvuhh27NIhnQqA++gDAAAAAAAAgIQeAAAAAAAA";
        let signature = "BiVJlg3liA6MaHQ0Fw9kdmBbj+SuuaKGMseZXPO6gx2XYx0AAAAAhgF7InR5cGUiOiJ3ZWJhdXRobi5nZXQiLCJjaGFsbGVuZ2UiOiJXellBZmVvbHcweU15bEFheDRvbzNjVC1rdEVaM0xmenZXcURqakxKZVRvIiwib3JpZ2luIjoiaHR0cDovL2xvY2FsaG9zdDo1MTczIiwiY3Jvc3NPcmlnaW4iOmZhbHNlfWICfOgpQ38QYao9Gj0/bqmWYNkuxvbuN3lz4uzFcXeVMEVivX41eC9H+tk+UnvUvKzThtf+uMLFzerU0zZLi8le4QJJsAUcyjsP/1UPAesax8UOC14M62FjAqtqaR46wR7jCg==";

        let transaction: Transaction = {
            use base64ct::Encoding;
            let bytes = base64ct::Base64::decode_vec(transaction).unwrap();
            bcs::from_bytes(&bytes).unwrap()
        };
        let signature = UserSignature::from_base64(signature).unwrap();

        let verifier = PasskeyVerifier::default();
        verifier
            .verify_transaction(&transaction, &signature)
            .unwrap();
    }
}

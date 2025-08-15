use signature::Signer;
use sui_sdk_types::PersonalMessage;
use sui_sdk_types::ZkLoginInputs;

use crate::ed25519::Ed25519PrivateKey;
use crate::SuiVerifier;

use super::*;

/// Returns a valid zklogin material for testing only.
fn test_zklogin_material() -> (Jwk, JwkId, ZkLoginInputs, Ed25519PrivateKey, u64) {
    let inputs = serde_json::json!({
        "proof_points": {
            "a": [
                "17318089125952421736342263717932719437717844282410187957984751939942898251250",
                "11373966645469122582074082295985388258840681618268593976697325892280915681207",
                "1"
            ],
            "b": [
                [
                    "5939871147348834997361720122238980177152303274311047249905942384915768690895",
                    "4533568271134785278731234570361482651996740791888285864966884032717049811708"
                ],
                [
                    "10564387285071555469753990661410840118635925466597037018058770041347518461368",
                    "12597323547277579144698496372242615368085801313343155735511330003884767957854"
                ],
                ["1","0"]
            ],
            "c": [
                "15791589472556826263231644728873337629015269984699404073623603352537678813171",
                "4547866499248881449676161158024748060485373250029423904113017422539037162527",
                "1"
            ]
        },
        "iss_base64_details": {
            "value": "wiaXNzIjoiaHR0cHM6Ly9pZC50d2l0Y2gudHYvb2F1dGgyIiw",
            "index_mod_4": 2
        },
        "header_base64": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IjEifQ",
        "address_seed": "20794788559620669596206457022966176986688727876128223628113916380927502737911"
    });

    let zklogin_inputs: ZkLoginInputs = serde_json::from_value(inputs).unwrap();

    let key = Ed25519PrivateKey::new([
        155, 244, 154, 106, 7, 85, 249, 83, 129, 31, 206, 18, 95, 38, 131, 213, 4, 41, 195, 187,
        73, 224, 116, 20, 126, 0, 137, 165, 46, 174, 21, 95,
    ]);

    let jwk: Jwk = serde_json::from_str(r#"{"alg":"RS256","e":"AQAB","kid":"1","kty":"RSA","n":"6lq9MQ-q6hcxr7kOUp-tHlHtdcDsVLwVIw13iXUCvuDOeCi0VSuxCCUY6UmMjy53dX00ih2E4Y4UvlrmmurK0eG26b-HMNNAvCGsVXHU3RcRhVoHDaOwHwU72j7bpHn9XbP3Q3jebX6KIfNbei2MiR0Wyb8RZHE-aZhRYO8_-k9G2GycTpvc-2GBsP8VHLUKKfAs2B6sW3q3ymU6M0L-cFXkZ9fHkn9ejs-sqZPhMJxtBPBxoUIUQFTgv4VXTSv914f_YkNw-EjuwbgwXMvpyr06EyfImxHoxsZkFYB-qBYHtaMxTnFsZBr6fn8Ha2JqT1hoP7Z5r5wxDu3GQhKkHw","use":"sig"}"#).unwrap();
    let jwk_id = JwkId {
        iss: "https://id.twitch.tv/oauth2".to_string(),
        kid: "1".to_string(),
    };
    let max_epoch = 10;
    (jwk, jwk_id, zklogin_inputs, key, max_epoch)
}

#[test]
fn zklogin_sign_personal_message() {
    let message = PersonalMessage(b"hello world".into());

    let (jwk, jwk_id, inputs, key, max_epoch) = test_zklogin_material();
    let signature = key.sign(&message.signing_digest());
    let zklogin_authenticator = ZkLoginAuthenticator {
        inputs,
        max_epoch,
        signature,
    };
    let mut verifier = ZkloginVerifier::new_dev();
    verifier.jwks_mut().insert(jwk_id, jwk);

    verifier
        .verify(&message.signing_digest(), &zklogin_authenticator)
        .unwrap();

    let user_signature = UserSignature::ZkLogin(zklogin_authenticator.into());
    verifier
        .verify_personal_message(&message, &user_signature)
        .unwrap();
}

use base64ct::Encoding;
use signature::Signer;
use signature::Verifier;
use sui_sdk_types::Ed25519Signature;
use sui_sdk_types::PersonalMessage;
use sui_sdk_types::SimpleSignature;
use sui_sdk_types::ZkLoginAuthenticator;
use sui_sdk_types::ZkLoginInputs;
use sui_sdk_types::ZkLoginInputsV2;

use crate::SuiVerifier;
use crate::ed25519::Ed25519PrivateKey;

use super::*;

/// Ephemeral Ed25519 key bound to the V1 proof fixtures.
pub(super) fn test_eph_key() -> Ed25519PrivateKey {
    Ed25519PrivateKey::new([
        155, 244, 154, 106, 7, 85, 249, 83, 129, 31, 206, 18, 95, 38, 131, 213, 4, 41, 195, 187,
        73, 224, 116, 20, 126, 0, 137, 165, 46, 174, 21, 95,
    ])
}

/// A `SimpleSignature` carrying the test ephemeral public key and a zeroed signature, for tests
/// that exercise proof verification only (which reads the public key, never the signature bytes).
pub(super) fn test_eph_simple_signature() -> SimpleSignature {
    SimpleSignature::Ed25519 {
        signature: Ed25519Signature::new([0; 64]),
        public_key: test_eph_key().public_key(),
    }
}

fn test_zklogin_v1_material() -> (Jwk, ZkLoginInputs, Ed25519PrivateKey, u64) {
    let v1_inputs: ZkLoginInputs = serde_json::from_value(serde_json::json!({
        "proof_points": {
            "a": [
                "3010136044534492475575413496484577590932720513683681989179363462925612714982",
                "5652552440722407852175866026012563042753395701135168680885840271325003476209",
                "1"
            ],
            "b": [
                [
                    "4134651808856881772397689190468686413285935393238712522589459332601848471034",
                    "20925849764837854473271908435319609501643424506601952398791677633465938999818"
                ],
                [
                    "16706593113195287380129955552431425955520047611810267551522210853347280177225",
                    "8975261226848675342214968602342036744422056685641356205140655491003281432470"
                ],
                ["1","0"]
            ],
            "c": [
                "14516934896859103165306714687867025901326990259356520077875194067697843674675",
                "12419034695817580775970400081639888434145575090674091981609146109263143679873",
                "1"
            ]
        },
        "iss_base64_details": {
            "value": "wiaXNzIjoiaHR0cHM6Ly9vYXV0aC5zdWkuaW8iLC",
            "index_mod_4": 2
        },
        "header_base64": "eyJraWQiOiJzdWkta2V5LWlkIiwidHlwIjoiSldUIiwiYWxnIjoiUlMyNTYifQ",
        "address_seed": "1930628255822123795956154519923524356793387287437090556144422698180443693114"
    }))
    .unwrap();

    let v1_jwk: Jwk = serde_json::from_str(r#"{"kty":"RSA","e":"AQAB","n":"wdar6dBP7b88u6o190uXBK4yPS76OxFjiHDPwOyJwJiiAE3YRq8y_pfyQVua9nEasgrPfEBzVoC5QCld8NAC88uvbNKyoLZ6w5KlFjp88q2Sfy7nAzh3I-AnjsL6zl43NZrKep4_Nmb5nJeafcEi6B5MsYYDWW1Nn26O7ob9IWRNWcaTKhOhqDzbiN_66azN-NgaP_uHjlu1xuMV0kRt-luLSTXJS3S2n8PVNCxr3Gr2oViUpP8kSa-0i5vsg3gNtSfBY97AbChwOnyazj_PoaVD6zbEC4L5gvRZJA6vyoAWF_ZumvkwC3fxcFIBnSKMlGzS_Z-bUyQDCQy_BG9AOw","alg":"RS256"}"#).unwrap();
    (v1_jwk, v1_inputs, test_eph_key(), 10)
}

fn test_zklogin_v2_material() -> (Jwk, ZkLoginInputsV2, Ed25519PrivateKey, u64) {
    let v2_inputs: ZkLoginInputsV2 = serde_json::from_value(serde_json::json!({
        "proofPoints": {
            "a": [
                "1371643420601076155071497420714858416197064562408404288964292331962783840851",
                "19396877194785923751476703426893506818011151828246825902628471533583539407641",
                "1"
            ],
            "b": [
                [
                    "20687828581533664136120493362699441695057661815758971316785498991900830426962",
                    "7846406698692509067213956119701171150421503410437995558375571599342456334007"
                ],
                [
                    "11265721058523555761885280357669969801110341082831300401803206755173988948537",
                    "19627242777264320737967573724679191116086574368019114169687969546643536129667"
                ],
                [
                    "1",
                    "0"
                ]
            ],
            "c": [
                "3433718082966757818210050974623195811955588196720324281939291625333034520147",
                "15753440238674154976446278288278567234697909781679908703446892070844354825673",
                "1"
            ]
        },
        "extIss": "\"iss\":\"https://jwt-tester.mystenlabs.com\",",
        "headerBase64": "eyJraWQiOiJzdWkta2V5LWlkLTgxOTIiLCJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9",
        "addressSeed": "0x610301ea5c8fe4c6e3269adba043d9b3b68f2d537444a9378a980f3d2eeb0fc5"
    }))
    .unwrap();
    let v2_jwk: Jwk = serde_json::from_str(r#"{"kty":"RSA","e":"AQAB","n":"lViYJOuLB6EZenCimgyWrwOH_QBEkCZxSIEfcQgP5MrZkRlohbrTAN1YpXGRaqugp9A4mRzCmi9ddXscpRBSsLefdPJJLG8lQZ2qrw6X2-6HD5kDFd6-K7JZS-_GOEfr5xGEDm8_MS_SorbmneKspL0n4MPYWH8qke4OBFCwL6WzGBU9rqDuvhYmafmkvVvOtHIqekBxNrCud7Spv43BHdiBM0V-jUquuNM3oK97i_GVLjGfwrGRpR3tK4nva_ryiHh9Ajs68If7-ZhIoLJ05lRsHJJpqsloiEqlCZwhge9zEMnNkoaIzdQr-xLy0GPnr5W0gikjlSGYiInfx9ITADwK3W33xdOB7npM7lqJY73Njbuw8hBQicU8t0M0gvvWfmh1KDeA5IqffZgue-ka9Jj1nrYmZtd0JimQpPDUiGbLv69gQJZcLVQWf9z6mVC4gNm8VU2OafssnolrvNndC3wIm8AgqzVzn_DIOcMQdhIe8jTF3hu1_6R4Id3KoA5Hb3uI2H86-8RjhSG2wKb3zi44yKSmxEDhzl7i450PQX64JK4ftv5jb9vSw5unpikmVvGlGsuvrqWFuWKBcrcXLgyar8pGvRO8fR9ifDHSj-D2fBiLnhK0-iqsJeU8XnfJhUvKxSjXejwsoQeLqlgq9-PgCDP3dE61fkqGpJ1UZjZ44Q9Vh4YLCPAO6oX8btXSkwreuP5m0UtWgFsc-ynWbt6NYS7JlsMtJNWybM4_auqRdil_cPMwFsUgjocztGLeG304YH-GehmyBJyGKuDIiXL9RfLoZ35jKawrWJb4UqckKWV5kOKeXsXdKtMw96ABFumcnhrzxAsqwshS5a2lT8P7Cdd9g3T1JXI7JM1AnJU9_gPXmJoc3yEFNf-JxEf00URoy2xUusyyxYdTswLJp3NQP4VjrAGwnsp7gHKC-V-mJ21FpQCHsV0JQ-1x-E3du9hkpsjTtGkffetEsV8k9enbkudox7WIlsnPcA8y7aY4lnaBqLLSzaj2GOf4KTN4cRpcPzOmSvgcVVYYQXDjRw45X86P1WJG8UDl6Wkl044tAdQRuIxW8QVzBFWWxeXcoagOBKn1_DV0RKUX9Ud4LLauy81rUNfoAcnolz9nippTBEZA_4OOBvXhdngCYaoZyjAkmYdPhKIkghGhKoVVKiEJ1Ua6nUr3zB9WFlTO9lODeV9h0tgKGtKGu3UBeaRCQSMv9gZK-eGIpcqjsqK_rEf4htdDZUBzfOJ0VtCiFYUUBPiuJNuIf9xQGVDE7qZufK1irvGug8jvWSWzB4pGLP75PnPH7B9axnXrxssaIR90Y3Vr9ih_ptzcfNrwD_wiGHUTy698FHu2fXp51HbSEQ","alg":"RS256"}"#).unwrap();
    (v2_jwk, v2_inputs, Ed25519PrivateKey::new([42; 32]), 10)
}

#[test]
fn zklogin_personal_message_respects_circuit_mode() {
    let message = PersonalMessage(b"hello world".into());
    let digest = message.signing_digest();

    let (v1_jwk, v1_inputs, v1_key, v1_max_epoch) = test_zklogin_v1_material();
    let (v2_jwk, v2_inputs, v2_key, v2_max_epoch) = test_zklogin_v2_material();
    let v1_jwk_id = v1_inputs.jwk_id().clone();
    let v2_jwk_id = v2_inputs.jwk_id().clone();
    let v1 = UserSignature::ZkLogin(Box::new(ZkLoginAuthenticator {
        inputs: v1_inputs,
        max_epoch: v1_max_epoch,
        signature: v1_key.sign(&digest),
    }));
    let v2 = UserSignature::ZkLoginV2(Box::new(ZkLoginAuthenticatorV2 {
        inputs: VersionedZkLoginInputs::V2(v2_inputs),
        max_epoch: v2_max_epoch,
        user_signature: v2_key.sign(&digest),
    }));

    let mut verifier = ZkloginVerifier::new_dev();
    verifier
        .jwks_mut()
        .extend([(v1_jwk_id, v1_jwk), (v2_jwk_id, v2_jwk)]);
    assert_eq!(verifier.circuit_mode(), ZkLoginCircuitMode::V1Only);

    for (mode, allows_v1, allows_v2) in [
        (ZkLoginCircuitMode::V1Only, true, false),
        (ZkLoginCircuitMode::Both, true, true),
        (ZkLoginCircuitMode::V2Only, false, true),
    ] {
        verifier.set_circuit_mode(mode);
        for (signature, should_verify) in [(&v1, allows_v1), (&v2, allows_v2)] {
            let result = verifier.verify_personal_message(&message, signature);
            assert_eq!(
                result.is_ok(),
                should_verify,
                "{mode:?} with {:?}: {result:?}",
                signature.scheme(),
            );
        }
    }
}

/// Regression test: an oversized JWK modulus (> 256 bytes) must return an
/// error instead of panicking via `unwrap()`.
#[test]
fn zklogin_verify_oversized_jwk_modulus_returns_error() {
    let message = PersonalMessage(b"hello world".into());

    let (mut jwk, inputs, key, max_epoch) = test_zklogin_v1_material();
    let jwk_id = inputs.jwk_id().clone();

    // Replace the modulus with a 257-byte value encoded as base64url; the v1 circuit only
    // supports 2048-bit (256-byte) RSA keys.
    jwk.n = base64ct::Base64UrlUnpadded::encode_string(&[0x42u8; 257]);

    let signature = key.sign(&message.signing_digest());
    let zklogin_authenticator = ZkLoginAuthenticator {
        inputs,
        max_epoch,
        signature,
    };
    let mut verifier = ZkloginVerifier::new_dev();
    verifier.jwks_mut().insert(jwk_id, jwk);

    let result = verifier.verify(&message.signing_digest(), &zklogin_authenticator);
    assert!(
        result.is_err(),
        "expected error for oversized modulus, got Ok"
    );
}

#[test]
fn zklogin_v1_inputs_hash_matches_fastcrypto() {
    use base64ct::Base64UrlUnpadded;

    let (jwk, inputs, _, max_epoch) = test_zklogin_v1_material();
    let signature = test_eph_simple_signature();
    let modulus = Base64UrlUnpadded::decode_vec(&jwk.n).unwrap();
    let hash = verify::calculate_v1_inputs_hash(&inputs, &signature, &modulus, max_epoch).unwrap();
    assert_eq!(
        hash.to_string(),
        "10694871766605632598609552840913903724536924605230723954224229765332817156326",
    );
}

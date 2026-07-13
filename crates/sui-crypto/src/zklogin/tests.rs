use base64ct::Encoding;
use signature::Signer;
use signature::Verifier;
use sui_sdk_types::Ed25519Signature;
use sui_sdk_types::PersonalMessage;
use sui_sdk_types::SimpleSignature;
use sui_sdk_types::ZkLoginAuthenticator;
use sui_sdk_types::ZkLoginInputs;

use crate::SuiVerifier;
use crate::ed25519::Ed25519PrivateKey;

use super::*;

/// The ephemeral ed25519 key shared by the zklogin test vectors (generated from seed `[0; 32]`);
/// the pinned proofs bind its public key.
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

    let key = test_eph_key();

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

/// The address seed shared by both pinned proofs.
const PINNED_PROOF_ADDRESS_SEED: &str =
    "1930628255822123795956154519923524356793387287437090556144422698180443693114";

/// Pinned v1-circuit proof (from prover-dev, over a TestIssuer JWT) and v2-circuit proof (from
/// prover-dev-v2, over a TestIssuerKey8192 JWT). Both use the same ephemeral key, max_epoch 10,
/// and the same address seed.
fn pinned_circuit_proof_material() -> (ZkloginVerifier, ZkLoginInputs, ZkLoginInputs) {
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
        "address_seed": PINNED_PROOF_ADDRESS_SEED
    }))
    .unwrap();

    let v2_inputs: ZkLoginInputs = serde_json::from_value(serde_json::json!({
        "proof_points": {
            "a": [
                "10290383706848897825088985381697541801790334043318488983431378625753497718833",
                "17591517929249273401895310772015184603780222740894596527224318218206984537104",
                "1"
            ],
            "b": [
                [
                    "3918033264923917639046061736301179481793021691718139448529347745111416321846",
                    "18123684393181543832037347354416686302931880789657446564169805732802329293676"
                ],
                [
                    "5501473627788010270179873817591115039637415124881199561215844494127224527091",
                    "5357636725999458520558358539710311237463411851594252917784356338840007452395"
                ],
                ["1","0"]
            ],
            "c": [
                "14357495699847490388821026099380711662414404327434223254697278410014042621999",
                "18065473806655831852488138949447369308006710206520416428472279919490765303978",
                "1"
            ]
        },
        "iss_base64_details": {
            "value": "wiaXNzIjoiaHR0cHM6Ly9qd3QtdGVzdGVyLm15c3RlbmxhYnMuY29tIiw",
            "index_mod_4": 2
        },
        "header_base64": "eyJraWQiOiJzdWkta2V5LWlkLTgxOTIiLCJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9",
        "address_seed": PINNED_PROOF_ADDRESS_SEED
    }))
    .unwrap();

    let v1_jwk: Jwk = serde_json::from_str(r#"{"kty":"RSA","e":"AQAB","n":"wdar6dBP7b88u6o190uXBK4yPS76OxFjiHDPwOyJwJiiAE3YRq8y_pfyQVua9nEasgrPfEBzVoC5QCld8NAC88uvbNKyoLZ6w5KlFjp88q2Sfy7nAzh3I-AnjsL6zl43NZrKep4_Nmb5nJeafcEi6B5MsYYDWW1Nn26O7ob9IWRNWcaTKhOhqDzbiN_66azN-NgaP_uHjlu1xuMV0kRt-luLSTXJS3S2n8PVNCxr3Gr2oViUpP8kSa-0i5vsg3gNtSfBY97AbChwOnyazj_PoaVD6zbEC4L5gvRZJA6vyoAWF_ZumvkwC3fxcFIBnSKMlGzS_Z-bUyQDCQy_BG9AOw","alg":"RS256"}"#).unwrap();
    let v2_jwk: Jwk = serde_json::from_str(r#"{"kty":"RSA","e":"AQAB","n":"lViYJOuLB6EZenCimgyWrwOH_QBEkCZxSIEfcQgP5MrZkRlohbrTAN1YpXGRaqugp9A4mRzCmi9ddXscpRBSsLefdPJJLG8lQZ2qrw6X2-6HD5kDFd6-K7JZS-_GOEfr5xGEDm8_MS_SorbmneKspL0n4MPYWH8qke4OBFCwL6WzGBU9rqDuvhYmafmkvVvOtHIqekBxNrCud7Spv43BHdiBM0V-jUquuNM3oK97i_GVLjGfwrGRpR3tK4nva_ryiHh9Ajs68If7-ZhIoLJ05lRsHJJpqsloiEqlCZwhge9zEMnNkoaIzdQr-xLy0GPnr5W0gikjlSGYiInfx9ITADwK3W33xdOB7npM7lqJY73Njbuw8hBQicU8t0M0gvvWfmh1KDeA5IqffZgue-ka9Jj1nrYmZtd0JimQpPDUiGbLv69gQJZcLVQWf9z6mVC4gNm8VU2OafssnolrvNndC3wIm8AgqzVzn_DIOcMQdhIe8jTF3hu1_6R4Id3KoA5Hb3uI2H86-8RjhSG2wKb3zi44yKSmxEDhzl7i450PQX64JK4ftv5jb9vSw5unpikmVvGlGsuvrqWFuWKBcrcXLgyar8pGvRO8fR9ifDHSj-D2fBiLnhK0-iqsJeU8XnfJhUvKxSjXejwsoQeLqlgq9-PgCDP3dE61fkqGpJ1UZjZ44Q9Vh4YLCPAO6oX8btXSkwreuP5m0UtWgFsc-ynWbt6NYS7JlsMtJNWybM4_auqRdil_cPMwFsUgjocztGLeG304YH-GehmyBJyGKuDIiXL9RfLoZ35jKawrWJb4UqckKWV5kOKeXsXdKtMw96ABFumcnhrzxAsqwshS5a2lT8P7Cdd9g3T1JXI7JM1AnJU9_gPXmJoc3yEFNf-JxEf00URoy2xUusyyxYdTswLJp3NQP4VjrAGwnsp7gHKC-V-mJ21FpQCHsV0JQ-1x-E3du9hkpsjTtGkffetEsV8k9enbkudox7WIlsnPcA8y7aY4lnaBqLLSzaj2GOf4KTN4cRpcPzOmSvgcVVYYQXDjRw45X86P1WJG8UDl6Wkl044tAdQRuIxW8QVzBFWWxeXcoagOBKn1_DV0RKUX9Ud4LLauy81rUNfoAcnolz9nippTBEZA_4OOBvXhdngCYaoZyjAkmYdPhKIkghGhKoVVKiEJ1Ua6nUr3zB9WFlTO9lODeV9h0tgKGtKGu3UBeaRCQSMv9gZK-eGIpcqjsqK_rEf4htdDZUBzfOJ0VtCiFYUUBPiuJNuIf9xQGVDE7qZufK1irvGug8jvWSWzB4pGLP75PnPH7B9axnXrxssaIR90Y3Vr9ih_ptzcfNrwD_wiGHUTy698FHu2fXp51HbSEQ","alg":"RS256"}"#).unwrap();

    let mut verifier = ZkloginVerifier::new_dev();
    verifier.jwks_mut().insert(
        JwkId {
            iss: "https://oauth.sui.io".to_string(),
            kid: "sui-key-id".to_string(),
        },
        v1_jwk,
    );
    verifier.jwks_mut().insert(
        JwkId {
            iss: "https://jwt-tester.mystenlabs.com".to_string(),
            kid: "sui-key-id-8192".to_string(),
        },
        v2_jwk,
    );

    (verifier, v1_inputs, v2_inputs)
}

/// Regression test: an oversized JWK modulus (> 256 bytes) must return an
/// error instead of panicking via `unwrap()`.
#[test]
fn zklogin_verify_oversized_jwk_modulus_returns_error() {
    let message = PersonalMessage(b"hello world".into());

    let (mut jwk, jwk_id, inputs, key, max_epoch) = test_zklogin_material();

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
fn zklogin_verify_v1_v2_scenarios() {
    let message = PersonalMessage(b"zklogin v2 test".into());
    let key = test_eph_key();
    let signature: sui_sdk_types::SimpleSignature = key.sign(&message.signing_digest());

    let (mut verifier, v1_inputs, v2_inputs) = pinned_circuit_proof_material();
    let v1_authenticator = ZkLoginAuthenticator {
        inputs: v1_inputs,
        max_epoch: 10,
        signature: signature.clone(),
    };
    let v2_authenticator = ZkLoginAuthenticator {
        inputs: v2_inputs,
        max_epoch: 10,
        signature,
    };

    assert_eq!(verifier.circuit_mode(), ZkLoginCircuitMode::V1Only);

    for (mode, v1_accepted, v2_accepted) in [
        (ZkLoginCircuitMode::V1Only, true, false), // in v1 mode, v1 accepted only
        (ZkLoginCircuitMode::Both, true, true),    // in both mode, both accepted
        (ZkLoginCircuitMode::V2Only, false, true), // in v2 mode, v2 accepted only
    ] {
        verifier.set_circuit_mode(mode);
        assert_eq!(
            verifier
                .verify(&message.signing_digest(), &v1_authenticator)
                .is_ok(),
            v1_accepted,
        );
        assert_eq!(
            verifier
                .verify(&message.signing_digest(), &v2_authenticator)
                .is_ok(),
            v2_accepted,
        );
    }
}

#[test]
fn zklogin_all_inputs_hash_matches_fastcrypto() {
    use base64ct::Base64UrlUnpadded;

    let (verifier, v1_inputs, v2_inputs) = pinned_circuit_proof_material();
    let signature = test_eph_simple_signature();

    for (inputs, (iss, kid), version, expected) in [
        (
            &v1_inputs,
            ("https://oauth.sui.io", "sui-key-id"),
            CircuitVersion::V1,
            "10694871766605632598609552840913903724536924605230723954224229765332817156326",
        ),
        (
            &v2_inputs,
            ("https://jwt-tester.mystenlabs.com", "sui-key-id-8192"),
            CircuitVersion::V2,
            "578543552377868609337700892750585372402246557338687807093764998910511040182",
        ),
    ] {
        let jwk = verifier
            .jwks()
            .get(&JwkId {
                iss: iss.to_string(),
                kid: kid.to_string(),
            })
            .unwrap();
        let modulus = Base64UrlUnpadded::decode_vec(&jwk.n).unwrap();
        let hash =
            verify::calculate_all_inputs_hash(inputs, &signature, &modulus, 10, version).unwrap();
        assert_eq!(hash.to_string(), expected);
    }
}

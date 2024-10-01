use std::str::FromStr;

use crate::SignatureError;
use ark_bn254::Fq;
use ark_bn254::Fq2;
use ark_bn254::Fr;
use ark_bn254::G1Affine;
use ark_bn254::G1Projective;
use ark_bn254::G2Affine;
use ark_bn254::G2Projective;
use ark_ff::PrimeField;
use ark_groth16::PreparedVerifyingKey;
use ark_groth16::Proof;
use sui_sdk_types::types::Bn254FieldElement;
use sui_sdk_types::types::CircomG1;
use sui_sdk_types::types::CircomG2;
use sui_sdk_types::types::Ed25519PublicKey;
use sui_sdk_types::types::Jwk;
use sui_sdk_types::types::Secp256k1PublicKey;
use sui_sdk_types::types::Secp256r1PublicKey;
use sui_sdk_types::types::SimpleSignature;
use sui_sdk_types::types::ZkLoginInputs;
use sui_sdk_types::types::ZkLoginProof;

use super::POSEIDON;

#[derive(Clone, Debug)]
pub struct VerifyingKey {
    inner: PreparedVerifyingKey<ark_bn254::Bn254>,
}

const fn str_to_bn254(s: &str) -> Bn254FieldElement {
    match Bn254FieldElement::from_str_radix_10(s) {
        Ok(e) => e,
        Err(_) => panic!("unable to convert bn254"),
    }
}

const fn build_circom_g1([e0, e1, e2]: [&str; 3]) -> CircomG1 {
    CircomG1([str_to_bn254(e0), str_to_bn254(e1), str_to_bn254(e2)])
}

const fn build_circom_g2([[e00, e01], [e10, e11], [e20, e21]]: [[&str; 2]; 3]) -> CircomG2 {
    CircomG2([
        [str_to_bn254(e00), str_to_bn254(e01)],
        [str_to_bn254(e10), str_to_bn254(e11)],
        [str_to_bn254(e20), str_to_bn254(e21)],
    ])
}

fn circom_to_arkworks_g1(g1: &CircomG1) -> Result<G1Affine, SignatureError> {
    let CircomG1([f0, f1, f2]) = g1;

    let g1: G1Affine =
        G1Projective::new_unchecked(bn254_to_fq(f0), bn254_to_fq(f1), bn254_to_fq(f2)).into();

    if !g1.is_on_curve() || !g1.is_in_correct_subgroup_assuming_on_curve() {
        return Err(SignatureError::from_source("invalid G1 input"));
    }

    Ok(g1)
}

fn circom_to_arkworks_g2(g2: &CircomG2) -> Result<G2Affine, SignatureError> {
    let CircomG2([[f00, f01], [f10, f11], [f20, f21]]) = g2;

    let g2: G2Affine = G2Projective::new_unchecked(
        Fq2::new(bn254_to_fq(f00), bn254_to_fq(f01)),
        Fq2::new(bn254_to_fq(f10), bn254_to_fq(f11)),
        Fq2::new(bn254_to_fq(f20), bn254_to_fq(f21)),
    )
    .into();

    if !g2.is_on_curve() || !g2.is_in_correct_subgroup_assuming_on_curve() {
        return Err(SignatureError::from_source("invalid G2 input"));
    }

    Ok(g2)
}

fn bn254_to_fq(f: &Bn254FieldElement) -> Fq {
    Fq::from_be_bytes_mod_order(f.padded())
}

fn bn254_to_fr(f: &Bn254FieldElement) -> Fr {
    Fr::from_be_bytes_mod_order(f.padded())
}

fn mainnet_verifying_key() -> VerifyingKey {
    const CIRCOM_ALPHA_G1: CircomG1 = build_circom_g1([
        "21529901943976716921335152104180790524318946701278905588288070441048877064089",
        "7775817982019986089115946956794180159548389285968353014325286374017358010641",
        "1",
    ]);

    const CIRCOM_BETA_G2: CircomG2 = build_circom_g2([
        [
            "6600437987682835329040464538375790690815756241121776438004683031791078085074",
            "16207344858883952201936462217289725998755030546200154201671892670464461194903",
        ],
        [
            "17943105074568074607580970189766801116106680981075272363121544016828311544390",
            "18339640667362802607939727433487930605412455701857832124655129852540230493587",
        ],
        ["1", "0"],
    ]);

    const CIRCOM_GAMMA_G2: CircomG2 = build_circom_g2([
        [
            "10857046999023057135944570762232829481370756359578518086990519993285655852781",
            "11559732032986387107991004021392285783925812861821192530917403151452391805634",
        ],
        [
            "8495653923123431417604973247489272438418190587263600148770280649306958101930",
            "4082367875863433681332203403145435568316851327593401208105741076214120093531",
        ],
        ["1", "0"],
    ]);

    const CIRCOM_DELTA_G2: CircomG2 = build_circom_g2([
        [
            "19260309516619721648285279557078789954438346514188902804737557357941293711874",
            "2480422554560175324649200374556411861037961022026590718777465211464278308900",
        ],
        [
            "14489104692423540990601374549557603533921811847080812036788172274404299703364",
            "12564378633583954025611992187142343628816140907276948128970903673042690269191",
        ],
        ["1", "0"],
    ]);

    const CIRCOM_GAMMA_ABC_G1: [CircomG1; 2] = [
        build_circom_g1([
            "1607694606386445293170795095076356565829000940041894770459712091642365695804",
            "18066827569413962196795937356879694709963206118612267170825707780758040578649",
            "1",
        ]),
        build_circom_g1([
            "20653794344898475822834426774542692225449366952113790098812854265588083247207",
            "3296759704176575765409730962060698204792513807296274014163938591826372646699",
            "1",
        ]),
    ];

    let vk = ark_groth16::VerifyingKey {
        alpha_g1: circom_to_arkworks_g1(&CIRCOM_ALPHA_G1).unwrap(),
        beta_g2: circom_to_arkworks_g2(&CIRCOM_BETA_G2).unwrap(),
        gamma_g2: circom_to_arkworks_g2(&CIRCOM_GAMMA_G2).unwrap(),
        delta_g2: circom_to_arkworks_g2(&CIRCOM_DELTA_G2).unwrap(),
        gamma_abc_g1: CIRCOM_GAMMA_ABC_G1
            .iter()
            .map(circom_to_arkworks_g1)
            .collect::<Result<_, _>>()
            .unwrap(),
    };

    VerifyingKey {
        inner: PreparedVerifyingKey::from(vk),
    }
}

/// Load a fixed verifying key from zkLogin.vkey output. This is based on a local setup and should not use in production.
fn dev_verifying_key() -> VerifyingKey {
    const CIRCOM_ALPHA_G1: CircomG1 = build_circom_g1([
        "20491192805390485299153009773594534940189261866228447918068658471970481763042",
        "9383485363053290200918347156157836566562967994039712273449902621266178545958",
        "1",
    ]);

    const CIRCOM_BETA_G2: CircomG2 = build_circom_g2([
        [
            "6375614351688725206403948262868962793625744043794305715222011528459656738731",
            "4252822878758300859123897981450591353533073413197771768651442665752259397132",
        ],
        [
            "10505242626370262277552901082094356697409835680220590971873171140371331206856",
            "21847035105528745403288232691147584728191162732299865338377159692350059136679",
        ],
        ["1", "0"],
    ]);

    const CIRCOM_GAMMA_G2: CircomG2 = build_circom_g2([
        [
            "10857046999023057135944570762232829481370756359578518086990519993285655852781",
            "11559732032986387107991004021392285783925812861821192530917403151452391805634",
        ],
        [
            "8495653923123431417604973247489272438418190587263600148770280649306958101930",
            "4082367875863433681332203403145435568316851327593401208105741076214120093531",
        ],
        ["1", "0"],
    ]);

    const CIRCOM_DELTA_G2: CircomG2 = build_circom_g2([
        [
            "10857046999023057135944570762232829481370756359578518086990519993285655852781",
            "11559732032986387107991004021392285783925812861821192530917403151452391805634",
        ],
        [
            "8495653923123431417604973247489272438418190587263600148770280649306958101930",
            "4082367875863433681332203403145435568316851327593401208105741076214120093531",
        ],
        ["1", "0"],
    ]);

    const CIRCOM_GAMMA_ABC_G1: [CircomG1; 2] = [
        build_circom_g1([
            "20701306374481714853949730154526815782802808896228594855451770849676897643964",
            "2766989084754673216772682210231588284954002353414778477810174100808747060165",
            "1",
        ]),
        build_circom_g1([
            "501195541410525737371980194958674422793469475773065719916327137354779402600",
            "13527631693157515024233848630878973193664410306029731429350155106228769355415",
            "1",
        ]),
    ];

    let vk = ark_groth16::VerifyingKey {
        alpha_g1: circom_to_arkworks_g1(&CIRCOM_ALPHA_G1).unwrap(),
        beta_g2: circom_to_arkworks_g2(&CIRCOM_BETA_G2).unwrap(),
        gamma_g2: circom_to_arkworks_g2(&CIRCOM_GAMMA_G2).unwrap(),
        delta_g2: circom_to_arkworks_g2(&CIRCOM_DELTA_G2).unwrap(),
        gamma_abc_g1: CIRCOM_GAMMA_ABC_G1
            .iter()
            .map(circom_to_arkworks_g1)
            .collect::<Result<_, _>>()
            .unwrap(),
    };

    VerifyingKey::new(PreparedVerifyingKey::from(vk))
}

impl VerifyingKey {
    fn new(inner: PreparedVerifyingKey<ark_bn254::Bn254>) -> Self {
        Self { inner }
    }

    pub fn new_mainnet() -> Self {
        mainnet_verifying_key()
    }

    pub fn new_dev() -> Self {
        dev_verifying_key()
    }

    pub fn verify_zklogin(
        &self,
        jwk: &Jwk,
        inputs: &ZkLoginInputs,
        signature: &SimpleSignature,
        max_epoch: u64,
    ) -> Result<(), SignatureError> {
        use base64ct::Base64UrlUnpadded;
        use base64ct::Encoding;
        // Decode modulus to bytes.
        let modulus = Base64UrlUnpadded::decode_vec(&jwk.n)
            .map_err(|e| SignatureError::from_source(e.to_string()))?;

        let proof = zklogin_proof_to_arkworks(&inputs.proof_points).unwrap();
        let input_hash = calculate_all_inputs_hash(inputs, signature, &modulus, max_epoch).unwrap();

        self.verify_proof(&proof, &[input_hash])
    }

    fn verify_proof(
        &self,
        proof: &Proof<ark_bn254::Bn254>,
        public_inputs: &[ark_bn254::Fr],
    ) -> Result<(), SignatureError> {
        use ark_snark::SNARK;

        if ark_groth16::Groth16::<ark_bn254::Bn254>::verify_with_processed_vk(
            &self.inner,
            public_inputs,
            proof,
        )
        .map_err(|e| SignatureError::from_source(e.to_string()))?
        {
            Ok(())
        } else {
            Err(SignatureError::from_source("Groth16 proof verify failed"))
        }
    }
}

fn zklogin_proof_to_arkworks(
    proof: &ZkLoginProof,
) -> Result<Proof<ark_bn254::Bn254>, SignatureError> {
    Ok(Proof {
        a: circom_to_arkworks_g1(&proof.a)?,
        b: circom_to_arkworks_g2(&proof.b)?,
        c: circom_to_arkworks_g1(&proof.c)?,
    })
}

/// Given a SimpleSignature convert the corrisponding public key, prefixed with the signature
/// scheme flag, to two Bn254Frs
pub fn public_key_to_frs(signature: &SimpleSignature) -> (Fr, Fr) {
    // buf length of the longest public key secp256r1/secp256k1 of 33 bytes plus 1 byte for the
    // scheme
    let mut buf = [0u8; 34];

    buf[0] = signature.scheme().to_u8();

    let buf = match signature {
        SimpleSignature::Ed25519 { public_key, .. } => {
            buf[1..Ed25519PublicKey::LENGTH + 1].copy_from_slice(public_key.inner());
            &buf[..Ed25519PublicKey::LENGTH + 1]
        }
        SimpleSignature::Secp256k1 { public_key, .. } => {
            buf[1..Secp256k1PublicKey::LENGTH + 1].copy_from_slice(public_key.inner());
            &buf[..Secp256k1PublicKey::LENGTH + 1]
        }
        SimpleSignature::Secp256r1 { public_key, .. } => {
            buf[1..Secp256r1PublicKey::LENGTH + 1].copy_from_slice(public_key.inner());
            &buf[..Secp256r1PublicKey::LENGTH + 1]
        }
    };

    //TODO this comment is wrong...
    // Split the bytes deterministically such that the first element contains the first 128
    // bits of the hash, and the second element contains the latter ones.
    let (first_half, second_half) = buf.split_at(buf.len() - 16);

    let eph_public_key_0 = Fr::from_be_bytes_mod_order(first_half);
    let eph_public_key_1 = Fr::from_be_bytes_mod_order(second_half);
    (eph_public_key_0, eph_public_key_1)
}

pub(crate) type U256 = bnum::BUintD8<32>;
pub(crate) type U2048 = bnum::BUintD8<256>;

const MAX_HEADER_LEN: u8 = 248;
const PACK_WIDTH: u8 = 248;
#[allow(unused)]
const ISS: &str = "iss";
#[allow(unused)]
const BASE64_URL_CHARSET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
const MAX_EXT_ISS_LEN: u8 = 165;
const MAX_ISS_LEN_B64: u8 = 4 * (1 + MAX_EXT_ISS_LEN / 3);

/// Pads a stream of bytes and maps it to a field element
pub fn hash_ascii_str_to_field(s: &str, max_size: u8) -> Result<Fr, SignatureError> {
    let str_padded = str_to_padded_char_codes(s, max_size)?;
    hash_to_field(&str_padded, 8, PACK_WIDTH)
}

fn str_to_padded_char_codes(s: &str, max_len: u8) -> Result<Vec<U256>, SignatureError> {
    let arr: Vec<U256> = s.bytes().map(U256::from).collect();
    pad_with_zeroes(arr, max_len)
}

fn pad_with_zeroes(in_arr: Vec<U256>, out_count: u8) -> Result<Vec<U256>, SignatureError> {
    if in_arr.len() > out_count as usize {
        return Err(SignatureError::from_source("in_arr too long"));
    }
    let mut padded = in_arr;
    padded.resize(out_count as usize, U256::ZERO);
    Ok(padded)
}

/// Maps a stream of bigints to a single field element. First we convert the base from
/// inWidth to packWidth. Then we compute the poseidon hash of the "packed" input.
/// input is the input vector containing equal-width big ints. inWidth is the width of
/// each input element.
fn hash_to_field<T: ToBits>(
    input: &[T],
    in_width: u16,
    pack_width: u8,
) -> Result<Fr, SignatureError> {
    let packed = convert_base(input, in_width, pack_width)?;

    POSEIDON.hash(&packed).map_err(SignatureError::from_source)
}

/// Helper function to pack field elements from big ints.
fn convert_base<T: ToBits>(
    in_arr: &[T],
    in_width: u16,
    out_width: u8,
) -> Result<Vec<Fr>, SignatureError> {
    if out_width == 0 {
        return Err(SignatureError::from_source("invalid input"));
    }
    let bits = big_int_array_to_bits(in_arr, in_width as usize)?;
    let mut packed: Vec<Fr> = bits
        .rchunks(out_width as usize)
        .map(|chunk| {
            Fr::from_be_bytes_mod_order(U256::from_radix_be(chunk, 2).unwrap().to_be().digits())
        })
        .collect();
    packed.reverse();
    match packed.len() != (in_arr.len() * in_width as usize).div_ceil(out_width as usize) {
        true => Err(SignatureError::from_source("invalid input")),
        false => Ok(packed),
    }
}

/// Convert a big int array to a bit array with 0 paddings.
fn big_int_array_to_bits<T: ToBits>(
    integers: &[T],
    intended_size: usize,
) -> Result<Vec<u8>, SignatureError> {
    use itertools::Itertools;
    use std::cmp::Ordering::Equal;
    use std::cmp::Ordering::Greater;
    use std::cmp::Ordering::Less;

    integers
        .iter()
        .map(|integer| {
            let bits = integer.to_bits();
            match bits.len().cmp(&intended_size) {
                Less => {
                    let extra_bits = intended_size - bits.len();
                    let mut padded = vec![0; extra_bits];
                    padded.extend(bits);
                    Ok(padded)
                }
                Equal => Ok(bits),
                Greater => Err(SignatureError::from_source("invalid input")),
            }
        })
        .flatten_ok()
        .collect()
}

trait ToBits {
    fn to_bits(&self) -> Vec<u8>;
}

impl ToBits for U256 {
    fn to_bits(&self) -> Vec<u8> {
        self.to_radix_be(2)
    }
}

impl ToBits for U2048 {
    fn to_bits(&self) -> Vec<u8> {
        self.to_radix_be(2)
    }
}

/// Calculate the poseidon hash from selected fields from inputs, along with the ephemeral pubkey.
pub fn calculate_all_inputs_hash(
    inputs: &ZkLoginInputs,
    signature: &SimpleSignature,
    modulus: &[u8],
    max_epoch: u64,
) -> Result<Fr, SignatureError> {
    if inputs.header_base64.len() > MAX_HEADER_LEN as usize {
        return Err(SignatureError::from_source("header too long"));
    }

    let (first, second) = public_key_to_frs(signature);

    let address_seed = bn254_to_fr(&inputs.address_seed);
    let max_epoch_f = Fr::from_be_bytes_mod_order(U256::from(max_epoch).to_be().digits());
    let index_mod_4_f = Fr::from_be_bytes_mod_order(
        U256::from(inputs.iss_base64_details.index_mod_4)
            .to_be()
            .digits(),
    );

    let iss_base64_f = hash_ascii_str_to_field(&inputs.iss_base64_details.value, MAX_ISS_LEN_B64)?;
    let header_f = hash_ascii_str_to_field(&inputs.header_base64, MAX_HEADER_LEN)?;
    let modulus_f = hash_to_field(&[U2048::from_be_slice(modulus).unwrap()], 2048, PACK_WIDTH)?;

    POSEIDON
        .hash(&[
            first,
            second,
            address_seed,
            max_epoch_f,
            iss_base64_f,
            index_mod_4_f,
            header_f,
            modulus_f,
        ])
        .map_err(SignatureError::from_source)
}

/// Calculate the Sui address based on address seed and address params.
#[allow(unused)]
fn gen_address_seed(
    salt: &str,
    name: &str,  // i.e. "sub"
    value: &str, // i.e. the sub value
    aud: &str,   // i.e. the client ID
) -> Result<String, SignatureError> {
    let salt_hash = POSEIDON
        .hash(&[bn254_to_fr(
            &Bn254FieldElement::from_str(salt).map_err(SignatureError::from_source)?,
        )])
        .map_err(SignatureError::from_source)?;
    gen_address_seed_with_salt_hash(salt_hash, name, value, aud)
}

const MAX_KEY_CLAIM_NAME_LENGTH: u8 = 32;
const MAX_KEY_CLAIM_VALUE_LENGTH: u8 = 115;
const MAX_AUD_VALUE_LENGTH: u8 = 145;

/// Same as [`gen_address_seed`] but takes the poseidon hash of the salt as input instead of the salt.
pub(crate) fn gen_address_seed_with_salt_hash(
    salt_hash: Fr,
    name: &str,  // i.e. "sub"
    value: &str, // i.e. the sub value
    aud: &str,   // i.e. the client ID
) -> Result<String, SignatureError> {
    Ok(POSEIDON
        .hash(&[
            hash_ascii_str_to_field(name, MAX_KEY_CLAIM_NAME_LENGTH)?,
            hash_ascii_str_to_field(value, MAX_KEY_CLAIM_VALUE_LENGTH)?,
            hash_ascii_str_to_field(aud, MAX_AUD_VALUE_LENGTH)?,
            salt_hash,
        ])
        .map_err(SignatureError::from_source)?
        .to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use sui_sdk_types::types::Ed25519Signature;

    #[cfg(test)]
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[test]
    fn test_verify_zklogin_google() {
        let user_salt = "206703048842351542647799591018316385612";

        let pubkey = Ed25519PublicKey::new([
            185, 198, 238, 22, 48, 239, 62, 113, 17, 68, 166, 72, 219, 6, 187, 178, 40, 79, 114,
            116, 207, 190, 229, 63, 252, 238, 80, 60, 193, 164, 146, 0,
        ]);
        let signature = SimpleSignature::Ed25519 {
            signature: Ed25519Signature::new([0; 64]),
            public_key: pubkey,
        };

        // Get the address seed.
        let address_seed = gen_address_seed(
            user_salt,
            "sub",
            "106294049240999307923",
            "25769832374-famecqrhe2gkebt5fvqms2263046lj96.apps.googleusercontent.com",
        )
        .unwrap();

        let inputs = serde_json::json!({
            "proof_points": {
                "a": [
                    "8247215875293406890829839156897863742504615191361518281091302475904551111016",
                    "6872980335748205979379321982220498484242209225765686471076081944034292159666",
                    "1"
                ],
                "b": [
                    [
                        "21419680064642047510915171723230639588631899775315750803416713283740137406807",
                        "21566716915562037737681888858382287035712341650647439119820808127161946325890"
                    ],
                    [
                        "17867714710686394159919998503724240212517838710399045289784307078087926404555",
                        "21812769875502013113255155836896615164559280911997219958031852239645061854221"
                    ],
                    ["1","0"]
                ],
                "c": [
                    "7530826803702928198368421787278524256623871560746240215547076095911132653214",
                    "16244547936249959771862454850485726883972969173921727256151991751860694123976",
                    "1"
                ]
            },
            "iss_base64_details": {
                "value": "yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC",
                "index_mod_4": 1
            },
            "header_base64": "eyJhbGciOiJSUzI1NiIsImtpZCI6IjZmNzI1NDEwMWY1NmU0MWNmMzVjOTkyNmRlODRhMmQ1NTJiNGM2ZjEiLCJ0eXAiOiJKV1QifQ",
            "address_seed": address_seed
        });

        let zklogin_inputs: ZkLoginInputs = serde_json::from_value(inputs).unwrap();

        let jwk = Jwk {
            kty: "RSA".to_string(),
            e: "AQAB".to_string(),
            n: "oUriU8GqbRw-avcMn95DGW1cpZR1IoM6L7krfrWvLSSCcSX6Ig117o25Yk7QWBiJpaPV0FbP7Y5-DmThZ3SaF0AXW-3BsKPEXfFfeKVc6vBqk3t5mKlNEowjdvNTSzoOXO5UIHwsXaxiJlbMRalaFEUm-2CKgmXl1ss_yGh1OHkfnBiGsfQUndKoHiZuDzBMGw8Sf67am_Ok-4FShK0NuR3-q33aB_3Z7obC71dejSLWFOEcKUVCaw6DGVuLog3x506h1QQ1r0FXKOQxnmqrRgpoHqGSouuG35oZve1vgCU4vLZ6EAgBAbC0KL35I7_0wUDSMpiAvf7iZxzJVbspkQ".to_string(),
            alg: "RS256".to_string(),
        };

        VerifyingKey::new_mainnet()
            .verify_zklogin(&jwk, &zklogin_inputs, &signature, 10)
            .unwrap();
    }

    #[test]
    fn test_public_key_to_frs() {
        let pubkey = Ed25519PublicKey::new([
            185, 198, 238, 22, 48, 239, 62, 113, 17, 68, 166, 72, 219, 6, 187, 178, 40, 79, 114,
            116, 207, 190, 229, 63, 252, 238, 80, 60, 193, 164, 146, 0,
        ]);
        let signature = SimpleSignature::Ed25519 {
            signature: Ed25519Signature::new([0; 64]),
            public_key: pubkey,
        };
        let (actual_0, actual_1) = public_key_to_frs(&signature);
        let expect_0 = Fr::from(ark_ff::BigInt([
            1244302228903607218,
            13386648721483054705,
            0,
            0,
        ]));

        let expect_1 = Fr::from(ark_ff::BigInt([
            18225592963892023808,
            2904666130704426303,
            0,
            0,
        ]));
        assert_eq!(actual_0, expect_0);
        assert_eq!(actual_1, expect_1);
    }

    #[test]
    fn test_hash_ascii_str_to_field() {
        let actual = hash_ascii_str_to_field("sub", 32).unwrap();
        let expect = Fr::from(ark_ff::BigInt([
            9420274050661827129,
            9736100402995345242,
            10431892319505233812,
            1450152190758097105,
        ]));
        assert_eq!(actual, expect);

        let actual = hash_ascii_str_to_field("106294049240999307923", 115).unwrap();
        let expect = Fr::from(ark_ff::BigInt([
            1616959301818912987,
            17318965991705091209,
            15303466056770245354,
            1596136658728187659,
        ]));
        assert_eq!(actual, expect);

        let actual = hash_ascii_str_to_field(
            "25769832374-famecqrhe2gkebt5fvqms2263046lj96.apps.googleusercontent.com",
            145,
        )
        .unwrap();
        let expect = Fr::from(ark_ff::BigInt([
            5030944271044826582,
            8577618269522081956,
            6962871209781429610,
            2149811477348923117,
        ]));
        assert_eq!(actual, expect);

        let actual =
            hash_ascii_str_to_field("yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC", 224)
                .unwrap();
        let expect = Fr::from(ark_ff::BigInt([
            6021918591354572765,
            14069258108381575504,
            1736509627917450843,
            2767185135515367512,
        ]));
        assert_eq!(actual, expect);

        let actual = hash_ascii_str_to_field(
        "eyJhbGciOiJSUzI1NiIsImtpZCI6IjZmNzI1NDEwMWY1NmU0MWNmMzVjOTkyNmRlODRhMmQ1NTJiNGM2ZjEiLCJ0eXAiOiJKV1QifQ",
        248,
    ).unwrap();
        let expect = Fr::from(ark_ff::BigInt([
            4239129243150064016,
            15469804315138207306,
            17534492051703966556,
            2100329545252322607,
        ]));
        assert_eq!(actual, expect);
    }
}

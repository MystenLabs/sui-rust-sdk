use argon2::Algorithm;
use argon2::Argon2;
use argon2::Params;
use argon2::Version;
use serde::Deserialize;
use serde::de;
use sui_sdk_types::Address;
use sui_sdk_types::Digest;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

const POW_VERSION: u64 = 1;
const POW_DOMAIN: &str = "sui-faucet-pow/1";
const POW_ALGORITHM: &str = "argon2d";
const POW_ARGON2_VERSION: u64 = 19;
const POW_MEMORY_SIZE: u32 = 8192;
const POW_ITERATIONS: u32 = 1;
const POW_PARALLELISM: u32 = 1;
const POW_HASH_LENGTH: usize = 32;
const POW_SALT: &[u8] = b"sui-faucet-pow-1";
const MIN_DIFFICULTY: u64 = 2;
const MAX_DIFFICULTY: u64 = 1 << 48;

#[derive(Clone)]
pub struct FaucetClient {
    faucet_url: http::Uri,
    inner: reqwest::Client,
}

impl FaucetClient {
    /// URL for the testnet faucet.
    pub const TESTNET: &str = "https://faucet.testnet.sui.io";
    /// URL for the devnet faucet.
    pub const DEVNET: &str = "https://faucet.devnet.sui.io";
    /// URL for the local faucet.
    pub const LOCAL: &str = "http://localhost:9123";

    /// Construct a client for a faucet service URL.
    ///
    /// The URL must not include an API path. The service must implement the v3 faucet API.
    pub fn new<T>(faucet_url: T) -> Result<Self, BoxError>
    where
        T: TryInto<http::Uri>,
        T::Error: Into<BoxError>,
    {
        let inner = reqwest::Client::new();
        let faucet_url = faucet_url.try_into().map_err(Into::into)?;
        Ok(Self { faucet_url, inner })
    }

    /// Fetch a proof-of-work challenge for `recipient`.
    pub async fn create_challenge(&self, recipient: Address) -> Result<PowChallenge, BoxError> {
        let response = self
            .inner
            .get(self.endpoint("v3/challenge"))
            .query(&[("recipient", recipient.to_string())])
            .send()
            .await?;
        let challenge: PowChallenge = self.json_response(response).await?;
        challenge.validate(recipient)?;
        Ok(challenge)
    }

    /// Solve a fresh challenge and submit its proof for `recipient`.
    ///
    /// This method blocks while it grinds the proof. Use [`Self::create_challenge`],
    /// [`PowChallenge::solve`], and [`Self::submit`] to schedule that work separately.
    pub async fn request(&self, recipient: Address) -> Result<FaucetResponse, BoxError> {
        let challenge = self.create_challenge(recipient).await?;
        let solution = challenge.solve()?;
        self.submit(&challenge, &solution).await
    }

    /// Submit a solved proof-of-work challenge.
    pub async fn submit(
        &self,
        challenge: &PowChallenge,
        solution: &PowSolution,
    ) -> Result<FaucetResponse, BoxError> {
        challenge.validate(challenge.recipient)?;
        let response = self
            .inner
            .post(self.endpoint("v3/gas"))
            .json(&FaucetRequest {
                recipient: challenge.recipient,
                checkpoint_seq: challenge.checkpoint_seq.to_string(),
                nonce: solution.nonce.to_string(),
                hash_hex: solution.hash_hex(),
            })
            .send()
            .await?;
        self.json_response(response).await
    }

    fn endpoint(&self, path: &str) -> String {
        format!(
            "{}/{path}",
            self.faucet_url.to_string().trim_end_matches('/')
        )
    }

    async fn json_response<T>(&self, response: reqwest::Response) -> Result<T, BoxError>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = response.status();
        if status.is_success() {
            return Ok(response.json().await?);
        }

        let error = response.json::<FaucetErrorResponse>().await.ok();
        let message = error
            .map(|error| format!("{} ({})", error.error, error.code))
            .unwrap_or_else(|| status.to_string());
        Err(format!("faucet request failed: {message}").into())
    }
}

/// Represent a v3 proof-of-work challenge returned by the faucet.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowChallenge {
    pub version: u64,
    pub domain: String,
    pub algorithm: String,
    pub argon2_version: u64,
    pub memory_size: u32,
    pub iterations: u32,
    pub parallelism: u32,
    pub hash_length: usize,
    pub salt: String,
    pub network: String,
    pub chain_id: String,
    #[serde(deserialize_with = "deserialize_u64")]
    pub checkpoint_seq: u64,
    pub checkpoint_digest: String,
    pub random_bytes: String,
    pub faucet_address: Address,
    pub recipient: Address,
    #[serde(deserialize_with = "deserialize_u64")]
    pub difficulty: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    pub difficulty_base: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    pub threshold: u64,
    pub expected_attempts: u64,
    pub window_seconds: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    pub amount_mist: u64,
}

impl PowChallenge {
    /// Solve this challenge and return the proof accepted by the v3 faucet.
    ///
    /// This method is CPU and memory intensive. Call it from a blocking task when running in an
    /// asynchronous application.
    pub fn solve(&self) -> Result<PowSolution, BoxError> {
        self.validate(self.recipient)?;
        let params = Params::new(
            POW_MEMORY_SIZE,
            POW_ITERATIONS,
            POW_PARALLELISM,
            Some(POW_HASH_LENGTH),
        )
        .map_err(|error| format!("invalid proof-of-work parameters: {error}"))?;
        let mut memory_blocks = vec![argon2::Block::default(); params.block_count()];
        let argon2 = Argon2::new(Algorithm::Argon2d, Version::V0x13, params);

        for nonce in 0..=u64::MAX {
            let mut hash = [0; POW_HASH_LENGTH];
            argon2
                .hash_password_into_with_memory(
                    self.preimage(nonce).as_bytes(),
                    POW_SALT,
                    &mut hash,
                    &mut memory_blocks,
                )
                .map_err(|error| format!("failed to compute proof of work: {error}"))?;
            if hash_to_u64(&hash) < self.threshold {
                return Ok(PowSolution { nonce, hash });
            }
        }

        Err("exhausted the proof-of-work nonce space".into())
    }

    fn validate(&self, recipient: Address) -> Result<(), BoxError> {
        let valid_parameters = self.version == POW_VERSION
            && self.domain == POW_DOMAIN
            && self.algorithm == POW_ALGORITHM
            && self.argon2_version == POW_ARGON2_VERSION
            && self.memory_size == POW_MEMORY_SIZE
            && self.iterations == POW_ITERATIONS
            && self.parallelism == POW_PARALLELISM
            && self.hash_length == POW_HASH_LENGTH
            && self.salt.as_bytes() == POW_SALT;
        if !valid_parameters {
            return Err("faucet challenge uses unsupported proof-of-work parameters".into());
        }
        if self.recipient != recipient {
            return Err("faucet challenge recipient does not match the requested recipient".into());
        }
        if !(MIN_DIFFICULTY..=MAX_DIFFICULTY).contains(&self.difficulty) {
            return Err("faucet challenge has an invalid difficulty".into());
        }
        if self.difficulty_base != self.difficulty
            || self.threshold != threshold_for(self.difficulty)
        {
            return Err("faucet challenge has an inconsistent difficulty".into());
        }
        Ok(())
    }

    fn preimage(&self, nonce: u64) -> String {
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{nonce}",
            self.domain,
            self.chain_id,
            self.checkpoint_seq,
            self.checkpoint_digest,
            self.random_bytes,
            self.faucet_address,
            self.recipient,
        )
    }
}

/// Represent a solved faucet proof-of-work challenge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowSolution {
    pub nonce: u64,
    pub hash: [u8; POW_HASH_LENGTH],
}

impl PowSolution {
    fn hash_hex(&self) -> String {
        self.hash.iter().map(|byte| format!("{byte:02x}")).collect()
    }
}

/// Represent a successful v3 faucet payout.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaucetResponse {
    pub digest: Digest,
    pub recipient: Address,
    #[serde(deserialize_with = "deserialize_u64")]
    pub amount_mist: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    pub difficulty: u64,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct FaucetRequest {
    recipient: Address,
    checkpoint_seq: String,
    nonce: String,
    hash_hex: String,
}

#[derive(Deserialize)]
struct FaucetErrorResponse {
    error: String,
    code: String,
}

fn deserialize_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: de::Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    if value.is_empty()
        || (value.len() > 1 && value.starts_with('0'))
        || !value.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(de::Error::custom("expected a canonical decimal u64"));
    }
    value.parse().map_err(de::Error::custom)
}

fn hash_to_u64(hash: &[u8; POW_HASH_LENGTH]) -> u64 {
    let mut value = [0; 8];
    value.copy_from_slice(&hash[..8]);
    u64::from_be_bytes(value)
}

fn threshold_for(difficulty: u64) -> u64 {
    ((1u128 << 64) / u128::from(difficulty)) as u64
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use std::io::Write;
    use std::net::TcpListener;
    use std::str::FromStr;

    use super::*;

    const RECIPIENT: &str = "0x0000000000000000000000000000000000000000000000000000000000000001";

    #[tokio::test]
    async fn creates_a_v3_proof_of_work_challenge() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0; 4096];
            let size = stream.read(&mut request).unwrap();
            let request = String::from_utf8_lossy(&request[..size]).into_owned();
            let body = challenge_json();
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len(),
            )
            .unwrap();
            request
        });

        let client = FaucetClient::new(format!("http://{address}/")).unwrap();
        let challenge = client
            .create_challenge(Address::from_str(RECIPIENT).unwrap())
            .await
            .unwrap();

        assert_eq!(challenge.recipient, Address::from_str(RECIPIENT).unwrap());
        assert_eq!(challenge.checkpoint_seq, 42);
        assert_eq!(challenge.difficulty, 2);
        assert!(
            server
                .join()
                .unwrap()
                .starts_with(&format!("GET /v3/challenge?recipient={RECIPIENT} HTTP/1.1"))
        );
    }

    #[tokio::test]
    async fn solves_and_submits_a_v3_gas_request() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut challenge_stream, _) = listener.accept().unwrap();
            let challenge_request = read_http_request(&mut challenge_stream);
            write_json_response(&mut challenge_stream, &challenge_json());

            let (mut gas_stream, _) = listener.accept().unwrap();
            let gas_request = read_http_request(&mut gas_stream);
            let body = format!(
                r#"{{"status":"success","digest":"{}","recipient":"{RECIPIENT}","amountMist":"1000000000","difficulty":"2"}}"#,
                "1".repeat(32),
            );
            write_json_response(&mut gas_stream, &body);
            (challenge_request, gas_request)
        });

        let client = FaucetClient::new(format!("http://{address}")).unwrap();
        let payout = client
            .request(Address::from_str(RECIPIENT).unwrap())
            .await
            .unwrap();
        let (challenge_request, gas_request) = server.join().unwrap();
        let gas_body = gas_request.split_once("\r\n\r\n").unwrap().1;
        let gas_body: serde_json::Value = serde_json::from_str(gas_body).unwrap();

        assert!(
            challenge_request
                .starts_with(&format!("GET /v3/challenge?recipient={RECIPIENT} HTTP/1.1"))
        );
        assert!(gas_request.starts_with("POST /v3/gas HTTP/1.1"));
        assert_eq!(gas_body["recipient"], RECIPIENT);
        assert_eq!(gas_body["checkpointSeq"], "42");
        assert!(gas_body["nonce"].as_str().is_some());
        assert_eq!(
            gas_body["hashHex"].as_str().unwrap().len(),
            POW_HASH_LENGTH * 2
        );
        assert_eq!(payout.recipient, Address::from_str(RECIPIENT).unwrap());
        assert_eq!(payout.amount_mist, 1_000_000_000);
        assert_eq!(payout.difficulty, 2);
    }

    #[test]
    fn solves_a_v1_challenge() {
        let challenge: PowChallenge = serde_json::from_str(&challenge_json()).unwrap();
        let solution = challenge.solve().unwrap();

        assert!(hash_to_u64(&solution.hash) < challenge.threshold);
        assert_eq!(solution.hash_hex().len(), POW_HASH_LENGTH * 2);
    }

    #[test]
    fn rejects_unsupported_challenge_parameters_before_grinding() {
        let json = challenge_json().replace("\"memorySize\": 8192", "\"memorySize\": 8193");
        let challenge: PowChallenge = serde_json::from_str(&json).unwrap();

        assert!(challenge.solve().is_err());
    }

    fn read_http_request(stream: &mut std::net::TcpStream) -> String {
        let mut bytes = Vec::new();
        let mut buffer = [0; 1024];
        loop {
            let size = stream.read(&mut buffer).unwrap();
            bytes.extend_from_slice(&buffer[..size]);
            let Some(headers_end) = bytes.windows(4).position(|window| window == b"\r\n\r\n")
            else {
                continue;
            };
            let headers = std::str::from_utf8(&bytes[..headers_end]).unwrap();
            let content_length = headers
                .lines()
                .find_map(|header| header.strip_prefix("content-length: "))
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or_default();
            if bytes.len() >= headers_end + 4 + content_length {
                return String::from_utf8(bytes).unwrap();
            }
        }
    }

    fn write_json_response(stream: &mut std::net::TcpStream, body: &str) {
        write!(
            stream,
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len(),
        )
        .unwrap();
    }

    fn challenge_json() -> String {
        format!(
            r#"{{
                "version": 1,
                "domain": "sui-faucet-pow/1",
                "algorithm": "argon2d",
                "argon2Version": 19,
                "memorySize": 8192,
                "iterations": 1,
                "parallelism": 1,
                "hashLength": 32,
                "salt": "sui-faucet-pow-1",
                "network": "testnet",
                "chainId": "chain-id",
                "checkpointSeq": "42",
                "checkpointDigest": "checkpoint-digest",
                "randomBytes": "random-bytes",
                "faucetAddress": "0x0000000000000000000000000000000000000000000000000000000000000002",
                "recipient": "{RECIPIENT}",
                "difficulty": "2",
                "difficultyBase": "2",
                "threshold": "9223372036854775808",
                "expectedAttempts": 2,
                "windowSeconds": 60,
                "amountMist": "1000000000"
            }}"#,
        )
    }
}

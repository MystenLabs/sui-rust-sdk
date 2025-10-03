use reqwest::StatusCode;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use sui_sdk_types::Address;
use sui_sdk_types::Digest;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfo {
    pub amount: u64,
    pub id: Address,
    pub transfer_tx_digest: Digest,
}

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

    /// Construct a new `FaucetClient` with the given faucet service URL. This [`FaucetClient`]
    /// expects that the service provides this endpoint: /v2/gas. As such, do not
    /// provide the request endpoint, just the top level service endpoint.
    pub fn new<T>(faucet_url: T) -> Result<Self, BoxError>
    where
        T: TryInto<http::Uri>,
        T::Error: Into<BoxError>,
    {
        let inner = reqwest::Client::new();
        let faucet_url = faucet_url.try_into().map_err(Into::into)?;
        Ok(FaucetClient { faucet_url, inner })
    }

    /// Make a faucet request. It returns a list of [`CoinInfo`] type, which upon success contains
    /// the information about the coins sent.
    pub async fn request(&self, address: Address) -> Result<Vec<CoinInfo>, BoxError> {
        const FAUCET_REQUEST_PATH: &str = "v2/gas";

        let address = address.to_string();
        let json_body = json![{
            "FixedAmountRequest": {
                "recipient": &address
            }
        }];

        let url = format!("{}{}", self.faucet_url, FAUCET_REQUEST_PATH);
        let resp = self
            .inner
            .post(url)
            .header("content-type", "application/json")
            .json(&json_body)
            .send()
            .await?;
        match resp.status() {
            StatusCode::ACCEPTED | StatusCode::CREATED | StatusCode::OK => {
                let faucet_resp: FaucetResponse = resp.json().await?;

                match faucet_resp.status {
                    RequestStatus::Success => {
                        Ok(faucet_resp.coins_sent.unwrap_or_default())
                    }
                    RequestStatus::Failure(err) => {
                        Err(format!("Faucet request was unsuccessful: {err}").into())
                    }
                }
            }
            StatusCode::TOO_MANY_REQUESTS => {
                Err("Faucet service received too many requests from this IP address. Please try again after 60 minutes.".into())
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                Err("Faucet service is currently overloaded or unavailable. Please try again later.".into())
            }
            status_code => {
                Err(format!("Faucet request was unsuccessful: {status_code}").into())
            }
        }
    }
}

#[derive(Deserialize)]
enum RequestStatus {
    Success,
    Failure(FaucetError),
}

#[derive(Deserialize)]
struct FaucetResponse {
    status: RequestStatus,
    coins_sent: Option<Vec<CoinInfo>>,
}

#[derive(Deserialize)]
enum FaucetError {
    MissingTurnstileTokenHeader,
    TooManyRequests(String),
    Internal(String),
    InvalidUserAgent(String),
}

impl std::fmt::Display for FaucetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FaucetError::MissingTurnstileTokenHeader => f.write_str(
"Missing X-Turnstile-Token header. For testnet tokens, please use the Web UI: https://faucet.sui.io"
            ),
            FaucetError::TooManyRequests(s) => write!(f, "Request limiit exceeded: {s}"),
            FaucetError::Internal(s) => write!(f, "Internal error: {s}"),
            FaucetError::InvalidUserAgent(s) => write!(f, "Invalid user agent: {s}"),
        }
    }
}

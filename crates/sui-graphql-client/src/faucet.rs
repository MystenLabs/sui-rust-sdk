// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use sui_types::Address;
use sui_types::Digest;

use anyhow::bail;
use reqwest::StatusCode;
use reqwest::Url;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use tracing::error as tracing_error;
use tracing::info;

pub const FAUCET_DEVNET_HOST: &str = "https://faucet.devnet.sui.io";
pub const FAUCET_TESTNET_HOST: &str = "https://faucet.testnet.sui.io";
pub const FAUCET_LOCAL_HOST: &str = "http://localhost:9123";

pub const FAUCET_REQUEST_PATH: &str = "v2/gas";

pub struct FaucetClient {
    faucet_url: Url,
    inner: reqwest::Client,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestStatus {
    Success,
    Failure(FaucetError),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FaucetResponse {
    pub status: RequestStatus,
    pub coins_sent: Option<Vec<CoinInfo>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfo {
    pub amount: u64,
    pub id: Address,
    pub transfer_tx_digest: Digest,
}

#[derive(Serialize, Deserialize, Error, Debug, PartialEq, Eq)]
pub enum FaucetError {
    #[error("Missing X-Turnstile-Token header. For testnet tokens, please use the Web UI: https://faucet.sui.io")]
    MissingTurnstileTokenHeader,

    #[error("Request limit exceeded. {0}")]
    TooManyRequests(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Invalid user agent: {0}")]
    InvalidUserAgent(String),
}

impl FaucetClient {
    /// Construct a new `FaucetClient` with the given faucet service URL. This [`FaucetClient`]
    /// expects that the service provides this endpoint: /v2/gas. As such, do not
    /// provide the request endpoint, just the top level service endpoint.
    pub fn new(faucet_url: &str) -> Self {
        let inner = reqwest::Client::new();
        let faucet_url = Url::parse(faucet_url).expect("Invalid faucet URL");
        FaucetClient { faucet_url, inner }
    }

    /// Set to local faucet.
    pub fn local() -> Self {
        Self {
            faucet_url: Url::parse(FAUCET_LOCAL_HOST).expect("Invalid faucet URL"),
            inner: reqwest::Client::new(),
        }
    }

    /// Set to devnet faucet.
    pub fn devnet() -> Self {
        Self {
            faucet_url: Url::parse(FAUCET_DEVNET_HOST).expect("Invalid faucet URL"),
            inner: reqwest::Client::new(),
        }
    }

    /// Set to testnet faucet.
    pub fn testnet() -> Self {
        Self {
            faucet_url: Url::parse(FAUCET_TESTNET_HOST).expect("Invalid faucet URL"),
            inner: reqwest::Client::new(),
        }
    }

    /// Make a faucet request. It returns a [`FaucetResponse`] type, which upon success contains
    /// the information about the coin sent.
    pub async fn request(&self, address: Address) -> Result<FaucetResponse, anyhow::Error> {
        let address = address.to_string();
        let json_body = json![{
            "FixedAmountRequest": {
                "recipient": &address
            }
        }];

        let url = format!("{}{}", self.faucet_url, FAUCET_REQUEST_PATH);
        info!(
            "Requesting gas from faucet for address {} : {}",
            address, url
        );
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
                        info!("Faucet request was successful: {:?}", faucet_resp);
                        Ok(faucet_resp)
                    }
                    RequestStatus::Failure(err) => {
                        tracing_error!("Faucet request was unsuccessful: {:?}", err);
                        bail!("Faucet request was unsuccessful: {:?}", err)
                    }
                }
            }
            StatusCode::TOO_MANY_REQUESTS => {
                tracing_error!("Faucet service received too many requests from this IP address.");
                bail!("Faucet service received too many requests from this IP address. Please try again after 60 minutes.");
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                tracing_error!("Faucet service is currently overloaded or unavailable.");
                bail!("Faucet service is currently overloaded or unavailable. Please try again later.");
            }
            status_code => {
                tracing_error!("Faucet request was unsuccessful: {status_code}");
                bail!("Faucet request was unsuccessful: {status_code}");
            }
        }
    }
}

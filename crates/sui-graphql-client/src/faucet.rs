// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use sui_types::types::{Address, ObjectId, TransactionDigest};

use anyhow::{anyhow, bail};
use reqwest::{StatusCode, Url};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;
use tracing::{error, info};

pub const FAUCET_DEVNET_HOST: &str = "https://faucet.devnet.sui.io";
pub const FAUCET_TESTNET_HOST: &str = "https://faucet.testnet.sui.io";
pub const FAUCET_LOCAL_HOST: &str = "http://localhost:9123";

const FAUCET_REQUEST_TIMEOUT: Duration = Duration::from_secs(120);
const FAUCET_POLL_INTERVAL: Duration = Duration::from_secs(2);

pub struct FaucetClient {
    faucet_url: Url,
    inner: reqwest::Client,
}

#[derive(serde::Deserialize)]
struct FaucetResponse {
    task: Option<String>,
    error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct BatchStatusFaucetResponse {
    pub status: Option<BatchSendStatus>,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum BatchSendStatusType {
    Inprogress,
    Succeeded,
    Discarded,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BatchSendStatus {
    pub status: BatchSendStatusType,
    pub transferred_gas_objects: Option<FaucetReceipt>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FaucetReceipt {
    pub sent: Vec<CoinInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BatchFaucetReceipt {
    pub task: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfo {
    pub amount: u64,
    pub id: ObjectId,
    pub transfer_tx_digest: TransactionDigest,
}

impl FaucetClient {
    /// Construct a new `FaucetClient` with the given faucet service URL. This [`FaucetClient`]
    /// expects that the service provides two endpoints: /v1/gas and /v1/status. As such, do not
    /// provide the request endpoint, just the top level service endpoint.
    ///
    /// - /v1/gas is used to request gas
    /// - /v1/status/taks-uuid is used to check the status of the request
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

    /// Request gas from the faucet. Note that this will return the UUID of the request and not
    /// wait until the token is received. Use `request_and_wait` to wait for the token.
    pub async fn request(&self, address: Address) -> Result<Option<String>, anyhow::Error> {
        self.request_impl(address).await
    }

    /// Internal implementation of a faucet request. It returns the task Uuid as a String.
    async fn request_impl(&self, address: Address) -> Result<Option<String>, anyhow::Error> {
        let address = address.to_string();
        let json_body = json![{
            "FixedAmountRequest": {
                "recipient": &address
            }
        }];
        let url = format!("{}v1/gas", self.faucet_url);
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
            StatusCode::ACCEPTED | StatusCode::CREATED => {
                let faucet_resp: FaucetResponse = resp.json().await?;

                if let Some(err) = faucet_resp.error {
                    error!("Faucet request was unsuccessful: {err}");
                    bail!("Faucet request was unsuccessful: {err}")
                } else {
                    info!("Request succesful: {:?}", faucet_resp.task);
                    Ok(faucet_resp.task)
                }
            }
            StatusCode::TOO_MANY_REQUESTS => {
                error!("Faucet service received too many requests from this IP address.");
                bail!("Faucet service received too many requests from this IP address. Please try again after 60 minutes.");
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                error!("Faucet service is currently overloaded or unavailable.");
                bail!("Faucet service is currently overloaded or unavailable. Please try again later.");
            }
            status_code => {
                error!("Faucet request was unsuccessful: {status_code}");
                bail!("Faucet request was unsuccessful: {status_code}");
            }
        }
    }

    /// Request gas from the faucet and wait until the request is completed and token is
    /// transferred. Returns `FaucetReceipt` if the request is successful, which contains the list
    /// of tokens transferred, and the transaction digest.
    ///
    /// Note that the faucet is heavily rate-limited, so calling repeatedly the faucet would likely
    /// result in a 429 code or 502 code.
    pub async fn request_and_wait(
        &self,
        address: Address,
    ) -> Result<Option<FaucetReceipt>, anyhow::Error> {
        let request_id = self.request(address).await?;
        if let Some(request_id) = request_id {
            let poll_response = tokio::time::timeout(FAUCET_REQUEST_TIMEOUT, async {
                let mut interval = tokio::time::interval(FAUCET_POLL_INTERVAL);
                loop {
                    interval.tick().await;
                    info!("Polling faucet request status: {request_id}");
                    let req = self.request_status(request_id.clone()).await;

                    if let Ok(Some(poll_response)) = req {
                        match poll_response.status {
                            BatchSendStatusType::Succeeded => {
                                info!("Faucet request {request_id} succeeded");
                                break Ok(poll_response);
                            }
                            BatchSendStatusType::Discarded => {
                                break Ok(BatchSendStatus {
                                    status: BatchSendStatusType::Discarded,
                                    transferred_gas_objects: None,
                                });
                            }
                            BatchSendStatusType::Inprogress => {
                                continue;
                            }
                        }
                    } else if let Some(err) = req.err() {
                        error!("Faucet request {request_id} failed. Error: {:?}", err);
                        break Err(anyhow!(
                            "Faucet request {request_id} failed. Error: {:?}",
                            err
                        ));
                    }
                }
            })
            .await
            .map_err(|_| {
                error!(
                    "Faucet request {request_id} timed out. Timeout set to {} seconds",
                    FAUCET_REQUEST_TIMEOUT.as_secs()
                );
                anyhow!("Faucet request timed out")
            })??;
            Ok(poll_response.transferred_gas_objects)
        } else {
            Ok(None)
        }
    }

    /// Check the faucet request status.
    ///
    /// Possible statuses are defined in: [`BatchSendStatusType`]
    pub async fn request_status(
        &self,
        id: String,
    ) -> Result<Option<BatchSendStatus>, anyhow::Error> {
        let status_url = format!("{}v1/status/{}", self.faucet_url, id);
        info!("Checking status of faucet request: {status_url}");
        let response = self.inner.get(&status_url).send().await?;
        if response.status() == StatusCode::TOO_MANY_REQUESTS {
            bail!("Cannot fetch request status due to too many requests from this IP address.");
        } else if response.status() == StatusCode::BAD_GATEWAY {
            bail!("Cannot fetch request status due to a bad gateway.")
        }
        let json = response
            .json::<BatchStatusFaucetResponse>()
            .await
            .map_err(|e| {
                error!("Failed to parse faucet response: {:?}", e);
                anyhow!("Failed to parse faucet response: {:?}", e)
            })?;
        Ok(json.status)
    }
}

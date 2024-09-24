// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use anyhow::bail;
use reqwest::{StatusCode, Url};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sui_types::types::{Address, ObjectId, TransactionDigest};

use crate::{DEVNET_HOST, LOCAL_HOST};

const FAUCET_DEVNET_HOST: &str = "https://faucet.devnet.sui.io/v1/gas";
const FAUCET_TESTNET_HOST: &str = "https://faucet.testnet.sui.io/v1/gas";
const FAUCET_LOCAL_HOST: &str = "http://localhost:9123/v1/gas";

const FAUCET_REQUEST_TIMEOUT: Duration = Duration::from_secs(120);
const FAUCET_POLL_INTERVAL: Duration = Duration::from_secs(2);

pub struct FaucetClient {
    faucet_url: Url,
    inner: reqwest::Client,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatchFaucetResponse {
    // This string is the Uuid for the req
    pub task: Option<String>,
    pub error: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct FaucetResponse {
    task: Option<String>,
    error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatchStatusFaucetResponse {
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
pub struct BatchFaucetReceipt {
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
    /// Construct a new `FaucetClient` with the given `reqwest::Client`. Defaults to Testnet
    /// faucet.
    pub(crate) fn new(inner: reqwest::Client, rpc: &Url) -> Self {
        let faucet_url = if rpc.as_str() == LOCAL_HOST {
            Url::parse(FAUCET_LOCAL_HOST).expect("Invalid faucet URL")
        } else if rpc.as_str() == DEVNET_HOST {
            Url::parse(FAUCET_DEVNET_HOST).expect("Invalid faucet URL")
        } else {
            Url::parse(FAUCET_TESTNET_HOST).expect("Invalid faucet URL")
        };
        FaucetClient { faucet_url, inner }
    }

    /// Set to local faucet.
    pub fn local(mut self) -> Self {
        self.faucet_url = Url::parse(FAUCET_LOCAL_HOST).expect("Invalid faucet URL");
        self
    }

    /// Set to devnet faucet.
    pub fn devnet(mut self) -> Self {
        self.faucet_url = Url::parse(FAUCET_DEVNET_HOST).expect("Invalid faucet URL");
        self
    }

    /// Set to testnet faucet.
    pub fn testnet(mut self) -> Self {
        self.faucet_url = Url::parse(FAUCET_TESTNET_HOST).expect("Invalid faucet URL");
        self
    }

    /// Request gas from the faucet. Note that this will return the UUID of the request and not
    /// wait until the token is received. Use `request_and_wait` to wait for the token.
    pub async fn request(&self, address: Address) -> Result<Option<String>, anyhow::Error> {
        self.request_impl(address, &self.faucet_url).await
    }

    /// Internal implementation of a faucet request. It returns the task Uuid as a String.
    async fn request_impl(
        &self,
        address: Address,
        faucet_url: &Url,
    ) -> Result<Option<String>, anyhow::Error> {
        let address = address.to_string();
        let json_body = json![{
            "FixedAmountRequest": {
                "recipient": &address
            }
        }];
        let resp = self
            .inner
            .post(faucet_url.as_ref())
            .header("content-type", "application/json")
            .json(&json_body)
            .send()
            .await?;
        match resp.status() {
            StatusCode::ACCEPTED | StatusCode::CREATED => {
                let faucet_resp: FaucetResponse = resp.json().await?;

                if let Some(err) = faucet_resp.error {
                    bail!("Faucet request was unsuccessful: {err}")
                } else {
                    Ok(faucet_resp.task)
                }
            }
            StatusCode::TOO_MANY_REQUESTS => {
                bail!("Faucet service received too many requests from this IP address. Please try again after 60 minutes.");
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                bail!("Faucet service is currently overloaded or unavailable. Please try again later.");
            }
            status_code => {
                bail!("Faucet request was unsuccessful: {status_code}");
            }
        }
    }

    /// Request gas from the faucet and wait until the request is completed and token is
    /// transferred. Returns `FaucetReceipt` if the request is successful, which contains the list
    /// of tokens transferred, and the transaction digest.
    pub async fn request_and_wait(
        &self,
        address: Address,
    ) -> Result<Option<FaucetReceipt>, anyhow::Error> {
        let request_id = self.request(address).await?;
        if let Some(request_id) = request_id {
            let status_url = Url::parse(
                self.faucet_url
                    .to_string()
                    .replace("gas", "status")
                    .as_str(),
            )
            .expect("Invalid faucet status URL");

            let poll_response = tokio::time::timeout(FAUCET_REQUEST_TIMEOUT, async {
                let mut interval = tokio::time::interval(FAUCET_POLL_INTERVAL);
                loop {
                    interval.tick().await;
                    eprint!("Polling faucet status...");
                    let req = self
                        .request_status(request_id.clone(), status_url.clone())
                        .await;

                    if let Ok(Some(poll_response)) = req {
                        eprintln!("response: {:?}", poll_response);
                        match poll_response.status {
                            BatchSendStatusType::Succeeded => {
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
                    } else if req.is_err() {
                        break Err(anyhow::anyhow!("Faucet request failed. {:?}", req.err()));
                    }
                }
            })
            .await
            .map_err(|_| anyhow::anyhow!("Faucet request timed out"))??;
            Ok(poll_response.transferred_gas_objects)
        } else {
            Ok(None)
        }
    }

    /// Request gas from a custom faucet service.
    pub async fn request_url(
        &self,
        address: Address,
        url: Url,
    ) -> Result<Option<String>, anyhow::Error> {
        self.request_impl(address, &url).await
    }

    /// Check the faucet request status.
    ///
    /// Possible statuses are defined in: [`BatchSendStatusType`]
    pub async fn request_status(
        &self,
        id: String,
        url: Url,
    ) -> Result<Option<BatchSendStatus>, anyhow::Error> {
        let url = format!("{}/{}", url, id);
        let response = self.inner.get(&url).send().await?;
        let json = response.json::<BatchStatusFaucetResponse>().await?;
        Ok(json.status)
    }
}

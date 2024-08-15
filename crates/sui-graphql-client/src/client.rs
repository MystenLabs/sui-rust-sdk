// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{Api, GraphQLClient};

const RPC_SERVER_HOST: &str = "https://sui-devnet.mystenlabs.com/graphql/beta";

#[derive(Clone, Debug)]
pub struct SuiClient {
    rpc: String,
    rpc_version: String,
    graphql_client: GraphQLClient,
    api: Api,
    // write_api: WriteApi,
}

impl SuiClient {
    /// Initialize a new SuiClient with testnet as the default network and no fullnode.
    /// In this mode, the client can only execute read queries.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the server address for the GraphQL RPC client. It should be a valid URL with a host and
    /// port number.
    pub fn set_rpc_server(&mut self, server: &str) -> Result<(), anyhow::Error> {
        self.rpc = server.parse::<String>()?;
        Ok(())
    }

    /// Set the version for the GraphQL RPC client. The default version is stable.
    ///
    /// By default, the GraphQL service can serve three versions: stable, beta, and legacy.
    pub fn set_version(&mut self, version: &str) {
        self.rpc_version = version.to_string();
    }

    /// Get a reference to the API client.
    pub fn read_api(&self) -> &Api {
        &self.api
    }

    // pub fn write_api(&self) -> &WriteApi {
    //     &self.write_api
    // }

    /// Get a reference to the write API.

    /// Get a mutable reference to the API client.
    pub(crate) fn api_mut(&mut self) -> &mut Api {
        &mut self.api
    }
}

pub trait DefaultConfigs {
    fn set_localnet(&mut self) -> &Self;
    fn set_devnet(&mut self) -> &Self;
    fn set_testnet(&mut self) -> &Self;
    fn set_mainnet(&mut self) -> &Self;
}

impl DefaultConfigs for SuiClient {
    fn set_localnet(&mut self) -> &Self {
        let url = "http://localhost:9125/graphql";
        // self.api_mut().set_rpc_server(url);
        self.rpc = url.to_string();
        self
    }

    fn set_devnet(&mut self) -> &Self {
        let url = "https://sui-devnet.mystenlabs.com/graphql";
        self.rpc = url.to_string();
        // self.api_mut().set_rpc_server(url);
        self
    }

    fn set_testnet(&mut self) -> &Self {
        let url = "https://sui-testnet.mystenlabs.com/graphql";
        self.rpc = url.to_string();
        // self.api_mut().set_rpc_server(url);
        self
    }

    fn set_mainnet(&mut self) -> &Self {
        let url = "https://sui-mainnet.mystenlabs.com/graphql";
        self.rpc = url.to_string();
        // self.api_mut().set_rpc_server(url);
        self
    }
}

impl Default for SuiClient {
    fn default() -> Self {
        let graphql_client =
            GraphQLClient::new(RPC_SERVER_HOST.to_string(), None, reqwest::Client::new());
        Self {
            rpc: RPC_SERVER_HOST
                .parse()
                .expect("Cannot parse RPC server host"),
            rpc_version: "".to_string(),
            graphql_client: graphql_client.clone(),
            api: Api::new(graphql_client),
        }
    }
}

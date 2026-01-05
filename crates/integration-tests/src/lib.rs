use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;
use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;
use std::process::Child;
use std::process::Command;
use sui_crypto::SuiSigner;
use sui_crypto::ed25519::Ed25519PrivateKey;
use sui_rpc::Client;
use sui_rpc::field::FieldMask;
use sui_rpc::field::FieldMaskUtil;
use sui_rpc::proto::sui::rpc::v2::ExecuteTransactionRequest;
use sui_sdk_types::Address;
use sui_sdk_types::Digest;
use sui_sdk_types::Identifier;
use sui_sdk_types::SignatureScheme;
use sui_transaction_builder::Function;
use sui_transaction_builder::ObjectInput;
use sui_transaction_builder::TransactionBuilder;
use sui_transaction_builder::intent::CoinWithBalance;
use tempfile::TempDir;
use tokio::time::Duration;
use tokio::time::sleep;

const DEFAULT_NUM_VALIDATORS: usize = 1;
const DEFAULT_EPOCH_DURATION_MS: u64 = 60_000;
const NETWORK_STARTUP_TIMEOUT_SECS: u64 = 30;
const NETWORK_STARTUP_POLL_INTERVAL_SECS: u64 = 1;

fn sui_binary() -> &'static Path {
    static SUI_BINARY: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();

    SUI_BINARY
        .get_or_init(|| {
            if let Ok(path) = std::env::var("SUI_BINARY") {
                return PathBuf::from(path);
            }
            if let Ok(output) = Command::new("which").arg("sui").output()
                && output.status.success()
            {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path.is_empty() {
                    return PathBuf::from(path);
                }
            }
            panic!("sui binary not found. Please install sui or set SUI_BINARY env var")
        })
        .as_path()
}

async fn wait_for_ready(client: &mut Client) -> Result<()> {
    // Wait till the network has started up and at least one checkpoint has been produced
    for _ in 0..NETWORK_STARTUP_TIMEOUT_SECS {
        if let Ok(resp) = client
            .ledger_client()
            .get_service_info(sui_rpc::proto::sui::rpc::v2::GetServiceInfoRequest::default())
            .await
            && resp.into_inner().checkpoint_height() > 0
        {
            return Ok(());
        }
        sleep(Duration::from_secs(NETWORK_STARTUP_POLL_INTERVAL_SECS)).await;
    }
    anyhow::bail!(
        "Network failed to start within {}s timeout",
        NETWORK_STARTUP_TIMEOUT_SECS,
    )
}

/// Handle for a Sui network running via pre-compiled binary
pub struct SuiNetworkHandle {
    /// Child process running sui
    process: Child,

    /// Temporary directory for config (auto-cleanup on drop)
    pub dir: TempDir,

    /// Network endpoints
    pub rpc_url: String,
    pub client: Client,

    /// Network configuration
    pub num_validators: usize,
    pub epoch_duration_ms: u64,

    pub validator_keys: BTreeMap<Address, Ed25519PrivateKey>,
    pub user_keys: Vec<Ed25519PrivateKey>,
}

impl Drop for SuiNetworkHandle {
    fn drop(&mut self) {
        let _ = self.process.kill();
    }
}

pub struct SuiNetworkBuilder {
    // pub dir: Option<PathBuf>,
    pub num_validators: usize,
    pub epoch_duration_ms: u64,
    pub sui_binary_path: Option<PathBuf>, // Optional custom binary
}

impl Default for SuiNetworkBuilder {
    fn default() -> Self {
        Self {
            num_validators: DEFAULT_NUM_VALIDATORS,
            epoch_duration_ms: DEFAULT_EPOCH_DURATION_MS,
            sui_binary_path: None,
            // dir: None,
        }
    }
}

impl SuiNetworkBuilder {
    pub fn with_num_validators(mut self, n: usize) -> Self {
        self.num_validators = n;
        self
    }

    pub fn with_epoch_duration_ms(mut self, ms: u64) -> Self {
        self.epoch_duration_ms = ms;
        self
    }

    pub fn with_binary(mut self, path: PathBuf) -> Self {
        self.sui_binary_path = Some(path);
        self
    }

    // pub fn dir(mut self, dir: &Path) -> Self {
    //     self.dir = Some(dir.to_owned());
    //     self
    // }

    pub async fn build(self) -> Result<SuiNetworkHandle> {
        let dir = TempDir::new()?;
        self.generate_genesis(dir.path())?;
        let (validator_keys, user_keys) = load_keys(dir.path())?;

        let rpc_port = get_available_port();
        let process = self.start_network(dir.path(), rpc_port)?;

        let rpc_url = format!("http://127.0.0.1:{rpc_port}");

        let mut client = sui_rpc::Client::new(&rpc_url)?;
        wait_for_ready(&mut client).await?;
        let mut sui = SuiNetworkHandle {
            process,
            dir,
            rpc_url,
            client,
            num_validators: self.num_validators,
            epoch_duration_ms: self.epoch_duration_ms,
            validator_keys,
            user_keys,
        };

        // Make sure SuiSystemState has been upgraded to v2
        sui.upgrade_sui_system_state().await?;

        // Make sure validator accounts are funded
        let fund_requests = sui
            .validator_keys
            .keys()
            // give each validator 1M SUI
            .map(|address| (*address, 1_000_000 * 1_000_000_000))
            .collect::<Vec<_>>();
        sui.fund(&fund_requests).await?;

        Ok(sui)
    }

    fn generate_genesis(&self, dir: &Path) -> Result<()> {
        std::fs::create_dir_all(dir)?;
        let mut cmd = Command::new(sui_binary());
        cmd.arg("genesis")
            .arg("--working-dir")
            .arg(dir)
            .arg("--epoch-duration-ms")
            .arg(self.epoch_duration_ms.to_string())
            .arg("--committee-size")
            .arg(self.num_validators.to_string())
            .arg("--with-faucet");
        let status = cmd.status()?;
        if !status.success() {
            return Err(anyhow::anyhow!("Failed to generate genesis"));
        }
        Ok(())
    }

    fn start_network(&self, dir: &Path, rpc_port: u16) -> Result<Child> {
        let stdout_name = dir.join("out.stdout");
        let stdout = std::fs::File::create(stdout_name)?;
        let stderr_name = dir.join("out.stderr");
        let stderr = std::fs::File::create(stderr_name)?;

        let mut cmd = Command::new(sui_binary());

        cmd.arg("start")
            .arg("--network.config")
            .arg(dir)
            .arg("--fullnode-rpc-port")
            .arg(rpc_port.to_string())
            .stdout(stdout)
            .stderr(stderr)
            .spawn()
            .map_err(|e| anyhow!("Failed to run `sui start`: {e}"))
    }
}

fn keypair_from_base64(b64: &str) -> Result<Ed25519PrivateKey> {
    let bytes = <base64ct::Base64 as base64ct::Encoding>::decode_vec(b64)?;

    let keypair =
        match SignatureScheme::from_byte(*bytes.first().ok_or_else(|| anyhow!("Invalid key"))?)
            .map_err(|e| anyhow!("{e}"))?
        {
            SignatureScheme::Ed25519 => Ed25519PrivateKey::new(
                bytes
                    .get(1..)
                    .ok_or_else(|| anyhow!("Invalid key"))?
                    .try_into()?,
            ),
            SignatureScheme::Secp256k1 => bail!("invalid key"),
            SignatureScheme::Secp256r1 => bail!("invalid key"),
            _ => bail!("invalid key"),
        };

    Ok(keypair)
}

fn ed25519_private_key_from_base64(b64: &str) -> Result<Ed25519PrivateKey> {
    let bytes = <base64ct::Base64 as base64ct::Encoding>::decode_vec(b64)?;
    Ok(Ed25519PrivateKey::new((&bytes[..]).try_into()?))
}

fn load_keys(dir: &Path) -> Result<(BTreeMap<Address, Ed25519PrivateKey>, Vec<Ed25519PrivateKey>)> {
    #[derive(serde::Deserialize)]
    struct Config {
        validator_configs: Vec<NodeConfig>,
        account_keys: Vec<String>,
    }

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct NodeConfig {
        account_key_pair: RawKey,
    }

    #[derive(serde::Deserialize)]
    struct RawKey {
        value: String,
    }

    let raw = std::fs::read(dir.join("network.yaml"))?;
    let network_config: Config = serde_yaml::from_slice(&raw)?;

    let mut validator_keys = BTreeMap::new();

    for validator in network_config.validator_configs {
        let keypair = keypair_from_base64(&validator.account_key_pair.value)?;
        let address = keypair.public_key().derive_address();
        validator_keys.insert(address, keypair);
    }

    let mut user_keys = vec![];

    for raw_key in network_config.account_keys {
        user_keys.push(ed25519_private_key_from_base64(&raw_key)?);
    }

    Ok((validator_keys, user_keys))
}

impl SuiNetworkHandle {
    pub async fn fund(&mut self, requests: &[(Address, u64)]) -> Result<()> {
        let private_key = self.user_keys.first().unwrap();
        let sender = private_key.public_key().derive_address();

        let mut builder = TransactionBuilder::new();
        builder.set_sender(sender);

        for (address, amount) in requests {
            let recipient = builder.pure(address);
            let coin = builder.intent(CoinWithBalance::sui(*amount));
            builder.transfer_objects(vec![coin], recipient);
        }

        let transaction = builder.build(&mut self.client).await?;

        let signature = private_key.sign_transaction(&transaction)?;

        let response = self
            .client
            .execute_transaction_and_wait_for_checkpoint(
                ExecuteTransactionRequest::new(transaction.into())
                    .with_signatures(vec![signature.into()])
                    .with_read_mask(FieldMask::from_str("*")),
                std::time::Duration::from_secs(10),
            )
            .await?
            .into_inner();

        assert!(
            response.transaction().effects().status().success(),
            "fund failed"
        );
        Ok(())
    }

    async fn upgrade_sui_system_state(&mut self) -> Result<()> {
        let private_key = self.user_keys.first().unwrap();
        let sender = private_key.public_key().derive_address();

        let mut builder = TransactionBuilder::new();
        builder.set_sender(sender);
        let sui_system = builder.object(ObjectInput::new(Address::from_static("0x5")));
        builder.move_call(
            Function::new(
                Address::from_static("0x3"),
                Identifier::from_static("sui_system"),
                Identifier::from_static("active_validator_addresses"),
            ),
            vec![sui_system],
        );

        let transaction = builder.build(&mut self.client).await?;

        let signature = private_key.sign_transaction(&transaction)?;

        let response = self
            .client
            .execute_transaction_and_wait_for_checkpoint(
                ExecuteTransactionRequest::new(transaction.into())
                    .with_signatures(vec![signature.into()])
                    .with_read_mask(FieldMask::from_str("*")),
                std::time::Duration::from_secs(10),
            )
            .await?
            .into_inner();

        assert!(
            response.transaction().effects().status().success(),
            "upgrade_sui_system_state failed"
        );
        Ok(())
    }

    pub fn build_package(&self, package: &Path) -> Result<(sui_sdk_types::Publish, Digest)> {
        #[derive(serde_derive::Deserialize)]
        struct MoveBuildOutput {
            modules: Vec<String>,
            dependencies: Vec<Address>,
            digest: Vec<u8>,
        }
        let client_config = self.dir.path().join("client.yaml");

        let mut cmd = Command::new(sui_binary());
        cmd.arg("move")
            .arg("--client.config")
            .arg(client_config)
            .arg("-p")
            .arg(package)
            .arg("build")
            .arg("--ignore-chain") // TODO remove once 1.62 is released
            .arg("--dump-bytecode-as-base64");
        let output = cmd.output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "stdout: {}\n\n stderr: {}",
                output.stdout.escape_ascii(),
                output.stderr.escape_ascii()
            ));
        }

        let move_build_output: MoveBuildOutput = serde_json::from_slice(&output.stdout)?;
        let modules = move_build_output
            .modules
            .into_iter()
            .map(|b64| <base64ct::Base64 as base64ct::Encoding>::decode_vec(&b64))
            .collect::<Result<Vec<_>, _>>()?;
        let digest = Digest::from_bytes(move_build_output.digest)?;

        Ok((
            sui_sdk_types::Publish {
                modules,
                dependencies: move_build_output.dependencies,
            },
            digest,
        ))
    }
}

/// Return an ephemeral, available port. On unix systems, the port returned will be in the
/// TIME_WAIT state ensuring that the OS won't hand out this port for some grace period.
/// Callers should be able to bind to this port given they use SO_REUSEADDR.
fn get_available_port() -> u16 {
    const MAX_PORT_RETRIES: u32 = 1000;

    for _ in 0..MAX_PORT_RETRIES {
        if let Ok(port) = get_ephemeral_port() {
            return port;
        }
    }

    panic!("Error: could not find an available port on localhost");
}

fn get_ephemeral_port() -> std::io::Result<u16> {
    use std::net::SocketAddr;
    use std::net::TcpListener;
    use std::net::TcpStream;

    // Request a random available port from the OS
    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0)))?;
    let addr = listener.local_addr()?;

    // Create and accept a connection (which we'll promptly drop) in order to force the port
    // into the TIME_WAIT state, ensuring that the port will be reserved from some limited
    // amount of time (roughly 60s on some Linux systems)
    let _sender = TcpStream::connect(addr)?;
    let _incoming = listener.accept()?;

    Ok(addr.port())
}

#[cfg(test)]
mod tests {
    use futures::stream::StreamExt;
    use sui_rpc::field::FieldMask;
    use sui_rpc::field::FieldMaskUtil;
    use sui_rpc::proto::sui::rpc::v2::SubscribeCheckpointsRequest;

    use super::*;

    #[tokio::test]
    async fn it_works() -> Result<(), anyhow::Error> {
        let mut sui = SuiNetworkBuilder::default().build().await?;

        // stream ~10 checkpoints to make sure things work
        let mut stream = sui
            .client
            .subscription_client()
            .subscribe_checkpoints(
                SubscribeCheckpointsRequest::default()
                    .with_read_mask(FieldMask::from_str("sequence_number")),
            )
            .await?
            .into_inner();

        let mut count = 0;
        let mut last = None;
        while let Some(item) = stream.next().await {
            let checkpoint = item.unwrap();
            let cursor = checkpoint.cursor.unwrap();
            assert_eq!(
                cursor,
                checkpoint.checkpoint.unwrap().sequence_number.unwrap()
            );
            println!("checkpoint: {cursor}");

            if let Some(last) = last {
                assert_eq!(last, cursor - 1);
            }
            last = Some(cursor);

            // Subscribe for 10 checkponts to ensure the subscription system works
            count += 1;
            if count > 10 {
                break;
            }
        }

        assert!(count >= 10);

        Ok(())
    }
}

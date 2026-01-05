use crate::Argument;
use crate::TransactionBuilder;

mod coin_with_balance;
pub use coin_with_balance::CoinWithBalance;

const MAX_GAS_OBJECTS: usize = 250; // 256
#[allow(unused)]
const MAX_COMMANDS: usize = 1000; // 1024
#[allow(unused)]
const MAX_INPUT_OBJECTS: usize = 2000; // 2048
const MAX_ARGUMENTS: usize = 500; // 512

pub(crate) type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub(crate) trait Intent: std::any::Any + Send + Sync {
    fn register(self, builder: &mut TransactionBuilder) -> Argument;
}

#[async_trait::async_trait]
pub(crate) trait IntentResolver: std::any::Any + std::fmt::Debug + Send {
    // Perform any required resolutions
    async fn resolve(
        &self,
        builder: &mut TransactionBuilder,
        client: &mut sui_rpc::Client,
    ) -> Result<(), BoxError>;
}

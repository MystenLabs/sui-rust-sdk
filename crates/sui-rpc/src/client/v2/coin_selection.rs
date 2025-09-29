use futures::StreamExt;
use prost_types::FieldMask;

use crate::client::v2::Client;
use crate::client::v2::Result;
use crate::field::FieldMaskUtil;
use crate::proto::sui::rpc::v2::ListOwnedObjectsRequest;
use crate::proto::sui::rpc::v2::Object;

pub const SUI_COIN_TYPE: &str = "0x2::sui::SUI";

impl Client {
    /// Selects coins of a specific type owned by an address until the total value meets the required amount.
    ///
    /// # Arguments
    /// * `owner_address` - The address that owns the coins
    /// * `coin_type` - The type of coins to select (e.g., "0x2::sui::SUI")
    /// * `amount` - The minimum total amount needed
    /// * `exclude` - Array of object IDs to exclude from selection
    ///
    /// # Returns
    /// A vector of `Object` instances representing the selected coins
    ///
    /// # Errors
    /// Returns an error if there are insufficient funds to meet the required amount or if there is an RPC error
    pub async fn select_coins(
        &self,
        owner_address: impl Into<String>,
        coin_type: impl Into<String>,
        amount: impl Into<u128>,
        exclude: &[&str],
    ) -> Result<Vec<Object>> {
        let coin_type = coin_type.into();
        let owner_address_str = owner_address.into();
        let amount = amount.into();
        let list_request = ListOwnedObjectsRequest::default()
            .with_owner(&owner_address_str)
            .with_object_type(format!("0x2::coin::Coin<{coin_type}>"))
            .with_page_size(500u32)
            .with_read_mask(FieldMask::from_paths([
                "object_id",
                "version",
                "digest",
                "balance",
            ]));

        let mut coin_stream = Box::pin(self.list_owned_objects(list_request));
        let mut selected_coins = Vec::new();
        let mut total = 0u128;

        while let Some(object_result) = coin_stream.next().await {
            let object = object_result?;

            if exclude.contains(&object.object_id()) {
                continue;
            }

            total += object.balance() as u128;
            selected_coins.push(object);

            if total >= amount {
                return Ok(selected_coins);
            }
        }

        Err(tonic::Status::failed_precondition(format!(
            "Insufficient funds for address [{owner_address_str}], requested amount: {amount}, total available: {total}",
        )))
    }

    /// Selects up to N coins of a specific type owned by an address.
    ///
    /// # Arguments
    /// * `owner_address` - The address that owns the coins
    /// * `coin_type` - The type of coins to select (e.g., "0x2::sui::SUI")
    /// * `n` - The maximum number of coins to select
    /// * `exclude` - Array of object IDs to exclude from selection
    ///
    /// # Returns
    /// A vector of `Object` instances representing the selected coins (may be fewer than `n` if not enough coins are available)
    ///
    /// # Errors
    /// Returns an error if there is an RPC error during coin retrieval
    pub async fn select_up_to_n_largest_coins(
        &self,
        owner_address: impl Into<String>,
        coin_type: impl Into<String>,
        n: impl Into<usize>,
        exclude: &[&str],
    ) -> Result<Vec<Object>> {
        let mut selected_coins = vec![];

        let n = n.into();
        let coin_type = coin_type.into();
        let list_request = ListOwnedObjectsRequest::default()
            .with_owner(owner_address)
            .with_object_type(format!("0x2::coin::Coin<{coin_type}>"))
            .with_page_size(500u32)
            .with_read_mask(FieldMask::from_paths([
                "object_id",
                "version",
                "digest",
                "balance",
            ]));

        let mut coin_stream = Box::pin(self.list_owned_objects(list_request));

        while selected_coins.len() < n {
            match coin_stream.next().await {
                Some(Ok(object)) => {
                    if !exclude.contains(&object.object_id()) {
                        selected_coins.push(object);
                    }
                }
                Some(Err(e)) => return Err(e),
                None => break,
            }
        }

        Ok(selected_coins)
    }
}

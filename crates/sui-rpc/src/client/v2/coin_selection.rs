use futures::StreamExt;
use prost_types::FieldMask;
use std::str::FromStr;

use sui_sdk_types::{Address, StructTag, TypeTag};

use crate::client::v2::Client;
use crate::client::v2::Result;
use crate::field::FieldMaskUtil;
use crate::proto::sui::rpc::v2::ListOwnedObjectsRequest;
use crate::proto::sui::rpc::v2::Object;

impl Client {
    /// Selects coins of a specific type owned by an address until the total value meets the required amount.
    ///
    /// # Arguments
    /// * `owner_address` - The address that owns the coins
    /// * `coin_type` - The TypeTag of coins to select
    /// * `amount` - The minimum total amount needed
    /// * `exclude` - Array of addresses to exclude from selection
    ///
    /// # Returns
    /// A vector of `Object` instances representing the selected coins
    ///
    /// # Errors
    /// Returns an error if there are insufficient funds to meet the required amount or if there is an RPC error
    pub async fn select_coins(
        &self,
        owner_address: impl Into<Address>,
        coin_type: impl Into<TypeTag>,
        amount: impl Into<u64>,
        exclude: &[Address],
    ) -> Result<Vec<Object>> {
        let owner_address = owner_address.into();
        let coin_type = coin_type.into();
        let amount = amount.into();
        let coin_struct = StructTag::coin(coin_type);
        let list_request = ListOwnedObjectsRequest::default()
            .with_owner(owner_address.to_string())
            .with_object_type(coin_struct.to_string())
            .with_page_size(500u32)
            .with_read_mask(FieldMask::from_paths([
                "object_id",
                "version",
                "digest",
                "balance",
            ]));

        let mut coin_stream = Box::pin(self.list_owned_objects(list_request));
        let mut selected_coins = Vec::new();
        let mut total = 0u64;

        while let Some(object_result) = coin_stream.next().await {
            let object = object_result?;

            if Address::from_str(object.object_id()).is_ok_and(|addr| exclude.contains(&addr)) {
                continue;
            }

            total = total.saturating_add(object.balance());
            selected_coins.push(object);

            if total >= amount {
                return Ok(selected_coins);
            }
        }

        Err(tonic::Status::failed_precondition(format!(
            "Insufficient funds for address [{owner_address}], requested amount: {amount}, total available: {total}"
        )))
    }

    /// Selects up to N coins of a specific type owned by an address.
    ///
    /// # Arguments
    /// * `owner_address` - The address that owns the coins
    /// * `coin_type` - The TypeTag of coins to select
    /// * `n` - The maximum number of coins to select
    /// * `exclude` - Array of addresses to exclude from selection
    ///
    /// # Returns
    /// A vector of `Object` instances representing the selected coins (may be fewer than `n` if not enough coins are available)
    ///
    /// # Errors
    /// Returns an error if there is an RPC error during coin retrieval
    pub async fn select_up_to_n_largest_coins(
        &self,
        owner_address: impl Into<Address>,
        coin_type: impl Into<TypeTag>,
        n: impl Into<usize>,
        exclude: &[Address],
    ) -> Result<Vec<Object>> {
        let mut selected_coins = vec![];

        let n = n.into();
        let coin_type = coin_type.into();
        let owner_address = owner_address.into();
        let coin_struct = StructTag::coin(coin_type);
        let list_request = ListOwnedObjectsRequest::default()
            .with_owner(owner_address.to_string())
            .with_object_type(coin_struct.to_string())
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
                    if !Address::from_str(object.object_id())
                        .is_ok_and(|addr| exclude.contains(&addr))
                    {
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

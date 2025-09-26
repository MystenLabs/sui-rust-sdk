use futures::StreamExt;
use prost_types::FieldMask;

use crate::client::v2::Client;
use crate::client::v2::Result;
use crate::field::FieldMaskUtil;
use crate::proto::sui::rpc::v2::{ListOwnedObjectsRequest, Object};

pub const SUI_COIN_TYPE: &str = "0x2::sui::SUI";

impl Client {
    pub async fn select_coins(
        &self,
        address: &str,
        coin_type: &str,
        amount: u128,
        exclude: &[&str],
    ) -> Result<Vec<Object>> {
        let list_request = ListOwnedObjectsRequest::default()
            .with_owner(address)
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
            "Insufficient funds for address [{address}], requested amount: {amount}, total available: {total}",
        )))
    }

    pub async fn select_up_to_n_largest_coins(
        &self,
        address: &str,
        coin_type: &str,
        n: usize,
    ) -> Result<Vec<Object>> {
        let mut selected_coins = vec![];

        let list_request = ListOwnedObjectsRequest::default()
            .with_owner(address)
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
                    selected_coins.push(object);
                }
                Some(Err(e)) => return Err(e),
                None => break,
            }
        }

        Ok(selected_coins)
    }
}

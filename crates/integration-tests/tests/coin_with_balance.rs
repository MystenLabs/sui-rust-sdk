use anyhow::Result;
use futures::TryStreamExt;
use integration_tests::*;
use sui_crypto::SuiSigner;
use sui_rpc::field::FieldMask;
use sui_rpc::field::FieldMaskUtil;
use sui_rpc::proto::sui::rpc::v2::ExecuteTransactionRequest;
use sui_rpc::proto::sui::rpc::v2::ListOwnedObjectsRequest;
use sui_sdk_types::Address;
use sui_sdk_types::TransactionKind;
use sui_transaction_builder::TransactionBuilder;
use sui_transaction_builder::intent::CoinWithBalance;

#[tokio::test]
async fn large_number_of_requests() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let recipient = Address::ZERO;

    let requests = vec![(recipient, 1_000_000_000u64); 500];
    sui.fund(&requests).await?;
    sui.fund(&requests).await?;

    let coins = sui
        .client
        .list_owned_objects(ListOwnedObjectsRequest::default().with_owner(recipient))
        .try_collect::<Vec<_>>()
        .await?;

    assert_eq!(coins.len(), 1000);

    // Build a request that requires filling out gas coins and multiple merge_coins
    let mut builder = TransactionBuilder::new();
    builder.set_sender(recipient);
    let arg = builder.intent(CoinWithBalance::sui(950));
    let self_address = builder.pure(&recipient);
    builder.transfer_objects(vec![arg], self_address);
    builder.build(&mut sui.client).await.unwrap();

    // Build a request that doesn't use the gas coin but requires multiple merge_coins
    let mut builder = TransactionBuilder::new();
    builder.set_sender(recipient);
    let arg = builder.intent(CoinWithBalance::sui(950).with_use_gas_coin(false));
    let self_address = builder.pure(&recipient);
    builder.transfer_objects(vec![arg], self_address);
    builder.build(&mut sui.client).await.unwrap();
    Ok(())
}

#[tokio::test]
async fn zero_value_requests() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let private_key = sui.user_keys.first().unwrap();
    let sender = private_key.public_key().derive_address();
    let recipient = Address::ZERO;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let arg = builder.intent(CoinWithBalance::sui(0));
    let recipient_address = builder.pure(&recipient);
    builder.transfer_objects(vec![arg], recipient_address);
    let transaction = builder.build(&mut sui.client).await?;

    if let TransactionKind::ProgrammableTransaction(pt) = &transaction.kind
        && let sui_sdk_types::Command::MoveCall(call) = pt.commands.first().unwrap()
    {
        assert!(
            call.package == Address::TWO
                && call.module.as_str() == "coin"
                && call.function.as_str() == "zero"
        )
    } else {
        panic!("failed to use 0x2::coin::zero to create zero value coin");
    }

    let signature = private_key.sign_transaction(&transaction)?;

    let _response = sui
        .client
        .execute_transaction_and_wait_for_checkpoint(
            ExecuteTransactionRequest::new(transaction.into())
                .with_signatures(vec![signature.into()])
                .with_read_mask(FieldMask::from_str("*")),
            std::time::Duration::from_secs(10),
        )
        .await?
        .into_inner();

    let coins = sui
        .client
        .list_owned_objects(ListOwnedObjectsRequest::default().with_owner(recipient))
        .try_collect::<Vec<_>>()
        .await?;

    assert_eq!(coins.len(), 1);
    assert_eq!(coins[0].balance(), 0);

    Ok(())
}

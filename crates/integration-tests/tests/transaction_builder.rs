use anyhow::Result;
use integration_tests::SuiNetworkBuilder;
use std::str::FromStr;
use sui_crypto::SuiSigner;
use sui_rpc::field::FieldMask;
use sui_rpc::field::FieldMaskUtil;
use sui_rpc::proto::sui::rpc::v2::ExecuteTransactionRequest;
use sui_sdk_types::Address;
use sui_sdk_types::ObjectReference;
use sui_sdk_types::StructTag;
use sui_sdk_types::TypeTag;
use sui_transaction_builder::Function;
use sui_transaction_builder::ObjectInput;
use sui_transaction_builder::TransactionBuilder;

#[tokio::test]
async fn test_move_call() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let private_key = sui.user_keys.first().unwrap();
    let sender = private_key.public_key().derive_address();

    // Check that `0x1::option::is_none` move call works when passing `1`
    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);

    // set up the sender, gas object, gas budget, and gas price and return the pk to sign
    let function = Function::new(
        "0x1".parse().unwrap(),
        "option".parse().unwrap(),
        "is_none".parse().unwrap(),
    )
    .with_type_args(vec![TypeTag::U64]);
    let input = builder.pure(&Some(1u64));
    builder.move_call(function, vec![input]);

    let tx = builder.build(&mut sui.client).await.unwrap();
    let signature = private_key.sign_transaction(&tx).unwrap();

    let response = sui
        .client
        .execute_transaction_and_wait_for_checkpoint(
            ExecuteTransactionRequest::new(tx.into())
                .with_signatures(vec![signature.into()])
                .with_read_mask(FieldMask::from_str("*")),
            std::time::Duration::from_secs(10),
        )
        .await?
        .into_inner();

    assert!(response.transaction().effects().status().success(),);
    Ok(())
}

// #[tokio::test]
// async fn test_split_transfer() {
//     let client = Client::new_localhost();
//     let mut tx = TransactionBuilder::new();
//     let (_, pk, _) = helper_setup(&mut tx, &client).await;

//     // transfer 1 SUI from Gas coin
//     let amount = tx.input(Serialized(&1_000_000_000u64));
//     let result = tx.split_coins(tx.gas(), vec![amount]);
//     let recipient_address = Address::generate(rand::thread_rng());
//     let recipient = tx.input(Serialized(&recipient_address));
//     tx.transfer_objects(vec![result], recipient);

//     let tx = tx.finish().unwrap();
//     let sig = pk.sign_transaction(&tx).unwrap();

//     let effects = client.execute_tx(vec![sig], &tx).await;
//     wait_for_tx_and_check_effects_status_success(&client, tx.digest(), effects).await;

//     // check that recipient has 1 coin
//     let recipient_coins = client
//         .coins(recipient_address, None, PaginationFilter::default())
//         .await
//         .unwrap();
//     assert_eq!(recipient_coins.data().len(), 1);
// }

// #[tokio::test]
// async fn test_split_without_transfer_should_fail() {
//     let client = Client::new_localhost();
//     let mut tx = TransactionBuilder::new();
//     let (_, pk, coins) = helper_setup(&mut tx, &client).await;

//     let coin = coins.first().unwrap().id();
//     let coin_obj: Input = (&client.object(*coin, None).await.unwrap().unwrap()).into();
//     let coin_input = tx.input(coin_obj.with_owned_kind());

//     // transfer 1 SUI
//     let amount = tx.input(Serialized(&1_000_000_000u64));
//     tx.split_coins(coin_input, vec![amount]);

//     let tx = tx.finish().unwrap();
//     let sig = pk.sign_transaction(&tx).unwrap();

//     let effects = client.execute_tx(vec![sig], &tx).await;
//     assert!(effects.is_ok());

//     // wait for the transaction to be finalized
//     loop {
//         let tx_digest = client.transaction(tx.digest()).await.unwrap();
//         if tx_digest.is_some() {
//             break;
//         }
//     }
//     assert!(effects.is_ok());
//     let status = effects.unwrap();
//     let expected_status = ExecutionStatus::Success;
//     // The tx failed, so we expect Failure instead of Success
//     assert_ne!(&expected_status, status.as_ref().unwrap().status());
// }

// #[tokio::test]
// async fn test_merge_coins() {
//     let client = Client::new_localhost();
//     let mut tx = TransactionBuilder::new();
//     let (address, pk, coins) = helper_setup(&mut tx, &client).await;

//     let coin1 = coins.first().unwrap().id();
//     let coin1_obj: Input = (&client.object(*coin1, None).await.unwrap().unwrap()).into();
//     let coin_to_merge = tx.input(coin1_obj.with_owned_kind());

//     let mut coins_to_merge = vec![];
//     // last coin is used for gas, first coin is the one we merge into
//     for c in coins[1..&coins.len() - 1].iter() {
//         let coin: Input = (&client.object(*c.id(), None).await.unwrap().unwrap()).into();
//         coins_to_merge.push(tx.input(coin.with_owned_kind()));
//     }

//     tx.merge_coins(coin_to_merge, coins_to_merge);
//     let tx = tx.finish().unwrap();
//     let sig = pk.sign_transaction(&tx).unwrap();

//     let effects = client.execute_tx(vec![sig], &tx).await;
//     wait_for_tx_and_check_effects_status_success(&client, tx.digest(), effects).await;

//     // check that there are two coins
//     let coins_after = client
//         .coins(address, None, PaginationFilter::default())
//         .await
//         .unwrap();
//     assert_eq!(coins_after.data().len(), 2);
// }

// #[tokio::test]
// async fn test_make_move_vec() {
//     let client = Client::new_localhost();
//     let mut tx = TransactionBuilder::new();
//     let (_, pk, _) = helper_setup(&mut tx, &client).await;

//     let input = tx.input(Serialized(&1u64));
//     tx.make_move_vec(Some(TypeTag::U64), vec![input]);

//     let tx = tx.finish().unwrap();
//     let sig = pk.sign_transaction(&tx).unwrap();

//     let effects = client.execute_tx(vec![sig], &tx).await;
//     wait_for_tx_and_check_effects_status_success(&client, tx.digest(), effects).await;
// }

#[tokio::test]
async fn test_publish() {
    let mut sui = SuiNetworkBuilder::default().build().await.unwrap();
    let private_key = sui.user_keys.first().unwrap();
    let sender = private_key.public_key().derive_address();

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);

    let package_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "packages/test_publish_v1");
    let (bytecode, _package_digest) = sui.build_package(package_dir.as_ref()).unwrap();
    let upgrade_cap = builder.publish(bytecode.modules, bytecode.dependencies);
    let sender_arg = builder.pure(&sender);
    builder.transfer_objects(vec![upgrade_cap], sender_arg);

    let tx = builder.build(&mut sui.client).await.unwrap();
    let signature = private_key.sign_transaction(&tx).unwrap();

    let response = sui
        .client
        .execute_transaction_and_wait_for_checkpoint(
            ExecuteTransactionRequest::new(tx.into())
                .with_signatures(vec![signature.into()])
                .with_read_mask(FieldMask::from_str("*")),
            std::time::Duration::from_secs(10),
        )
        .await
        .unwrap()
        .into_inner();

    assert!(response.transaction().effects().status().success());
}

#[tokio::test]
async fn test_upgrade() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await.unwrap();
    let private_key = sui.user_keys.first().unwrap();
    let sender = private_key.public_key().derive_address();

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);

    let package_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "packages/test_publish_v2");
    let (bytecode, _package_digest) = sui.build_package(package_dir.as_ref()).unwrap();
    let upgrade_cap = builder.publish(bytecode.modules, bytecode.dependencies);
    let sender_arg = builder.pure(&sender);
    builder.transfer_objects(vec![upgrade_cap], sender_arg);

    let tx = builder.build(&mut sui.client).await.unwrap();
    let signature = private_key.sign_transaction(&tx).unwrap();

    let response = sui
        .client
        .execute_transaction_and_wait_for_checkpoint(
            ExecuteTransactionRequest::new(tx.into())
                .with_signatures(vec![signature.into()])
                .with_read_mask(FieldMask::from_str("*")),
            std::time::Duration::from_secs(10),
        )
        .await
        .unwrap()
        .into_inner();

    assert!(response.transaction().effects().status().success());

    let upgrade_cap = {
        let upgrade_cap_type = StructTag::from_str("0x2::package::UpgradeCap")?.to_string();
        let upgrade_cap = response
            .transaction()
            .effects()
            .changed_objects()
            .iter()
            .find(|o| o.object_type() == upgrade_cap_type)
            .unwrap();
        ObjectReference::new(
            upgrade_cap.object_id().parse()?,
            upgrade_cap.output_version(),
            upgrade_cap.output_digest().parse()?,
        )
    };

    let package_id = response
        .transaction()
        .effects()
        .changed_objects()
        .iter()
        .find(|o| o.object_type() == "package")
        .unwrap()
        .object_id()
        .parse::<Address>()?;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);

    let package_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "packages/test_publish_v2");
    let (bytecode, package_digest) = sui.build_package(package_dir.as_ref()).unwrap();
    let upgrade_cap = builder.object(ObjectInput::new(*upgrade_cap.object_id()));
    let policy = builder.pure(&0u8);
    let package_digest = builder.pure(&package_digest);

    // we need this ticket to authorize the upgrade
    let upgrade_ticket = builder.move_call(
        Function::new(
            "0x2".parse().unwrap(),
            "package".parse().unwrap(),
            "authorize_upgrade".parse().unwrap(),
        ),
        vec![upgrade_cap, policy, package_digest],
    );
    // now we can upgrade the package
    let upgrade_receipt = builder.upgrade(
        bytecode.modules,
        bytecode.dependencies,
        package_id,
        upgrade_ticket,
    );

    // commit the upgrade
    builder.move_call(
        Function::new(
            "0x2".parse().unwrap(),
            "package".parse().unwrap(),
            "commit_upgrade".parse().unwrap(),
        ),
        vec![upgrade_cap, upgrade_receipt],
    );

    let tx = builder.build(&mut sui.client).await.unwrap();
    let signature = private_key.sign_transaction(&tx).unwrap();

    let response = sui
        .client
        .execute_transaction_and_wait_for_checkpoint(
            ExecuteTransactionRequest::new(tx.into())
                .with_signatures(vec![signature.into()])
                .with_read_mask(FieldMask::from_str("*")),
            std::time::Duration::from_secs(10),
        )
        .await
        .unwrap()
        .into_inner();

    assert!(response.transaction().effects().status().success());

    Ok(())
}

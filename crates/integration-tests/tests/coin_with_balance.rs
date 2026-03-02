use anyhow::Result;
use futures::TryStreamExt;
use integration_tests::*;
use sui_crypto::SuiSigner;
use sui_crypto::ed25519::Ed25519PrivateKey;
use sui_rpc::field::FieldMask;
use sui_rpc::field::FieldMaskUtil;
use sui_rpc::proto::sui::rpc::v2::ExecuteTransactionRequest;
use sui_rpc::proto::sui::rpc::v2::ListOwnedObjectsRequest;
use sui_sdk_types::Address;
use sui_sdk_types::Input;
use sui_sdk_types::StructTag;
use sui_sdk_types::TransactionKind;
use sui_transaction_builder::Error;
use sui_transaction_builder::TransactionBuilder;
use sui_transaction_builder::intent::CoinWithBalance;

const MIST_PER_SUI: u64 = 1_000_000_000;

fn fresh_account() -> (Ed25519PrivateKey, Address) {
    let private_key = Ed25519PrivateKey::generate(rand_core::OsRng);
    let sender = private_key.public_key().derive_address();
    (private_key, sender)
}

/// Extract the PTB inputs from a transaction.
fn ptb_inputs(transaction: &sui_sdk_types::Transaction) -> &[Input] {
    match &transaction.kind {
        TransactionKind::ProgrammableTransaction(pt) => &pt.inputs,
        other => panic!("expected ProgrammableTransaction, got {other:?}"),
    }
}

fn has_funds_withdrawal(transaction: &sui_sdk_types::Transaction) -> bool {
    ptb_inputs(transaction)
        .iter()
        .any(|i| matches!(i, Input::FundsWithdrawal(_)))
}

/// Check whether the transaction uses coin objects — either as PTB inputs
/// (non-gas path) or as gas payment objects (gas path).
fn has_coin_objects(transaction: &sui_sdk_types::Transaction) -> bool {
    let has_ptb_coin_inputs = ptb_inputs(transaction)
        .iter()
        .any(|i| matches!(i, Input::ImmutableOrOwned(_)));
    let has_gas_objects = !transaction.gas_payment.objects.is_empty();
    has_ptb_coin_inputs || has_gas_objects
}

/// Helper to sign, execute, and assert success.
async fn execute(
    client: &mut sui_rpc::Client,
    private_key: &sui_crypto::ed25519::Ed25519PrivateKey,
    transaction: sui_sdk_types::Transaction,
) -> Result<sui_rpc::proto::sui::rpc::v2::ExecuteTransactionResponse> {
    let signature = private_key.sign_transaction(&transaction)?;
    let response = client
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
        "transaction execution failed"
    );
    Ok(response)
}

fn sui_coin_type() -> StructTag {
    StructTag::coin(StructTag::sui().into())
}

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

#[tokio::test]
async fn basic_single_request() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();
    let recipient = Address::ZERO;

    // Fund the sender
    sui.fund(&[(sender, 5 * MIST_PER_SUI)]).await?;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let coin = builder.intent(CoinWithBalance::sui(MIST_PER_SUI));
    let recipient_arg = builder.pure(&recipient);
    builder.transfer_objects(vec![coin], recipient_arg);
    let transaction = builder.build(&mut sui.client).await?;

    // Coins are sufficient — should use coin objects, no FundsWithdrawal
    assert!(
        !has_funds_withdrawal(&transaction),
        "should not use FundsWithdrawal when coins are sufficient"
    );

    execute(&mut sui.client, &private_key, transaction).await?;

    let coins = sui
        .client
        .list_owned_objects(
            ListOwnedObjectsRequest::default()
                .with_owner(recipient)
                .with_object_type(sui_coin_type())
                .with_read_mask(FieldMask::from_str("balance")),
        )
        .try_collect::<Vec<_>>()
        .await?;

    assert_eq!(coins.len(), 1);
    assert_eq!(coins[0].balance(), MIST_PER_SUI);

    Ok(())
}

#[tokio::test]
async fn multiple_amounts_single_transaction() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();
    let recipient = Address::ZERO;

    // Fund the sender with enough for all transfers + gas
    sui.fund(&[(sender, 20 * MIST_PER_SUI)]).await?;

    let amounts = [MIST_PER_SUI, 2 * MIST_PER_SUI, 3 * MIST_PER_SUI];

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let recipient_arg = builder.pure(&recipient);
    for amount in &amounts {
        let coin = builder.intent(CoinWithBalance::sui(*amount));
        builder.transfer_objects(vec![coin], recipient_arg);
    }
    let transaction = builder.build(&mut sui.client).await?;

    // Coins are sufficient — should use coin objects, no FundsWithdrawal
    assert!(
        !has_funds_withdrawal(&transaction),
        "should not use FundsWithdrawal when coins are sufficient"
    );

    execute(&mut sui.client, &private_key, transaction).await?;

    let coins = sui
        .client
        .list_owned_objects(
            ListOwnedObjectsRequest::default()
                .with_owner(recipient)
                .with_object_type(sui_coin_type())
                .with_read_mask(FieldMask::from_str("balance")),
        )
        .try_collect::<Vec<_>>()
        .await?;

    assert_eq!(coins.len(), amounts.len());
    let mut balances: Vec<u64> = coins.iter().map(|c| c.balance()).collect();
    balances.sort();
    let mut expected = amounts.to_vec();
    expected.sort();
    assert_eq!(balances, expected);

    Ok(())
}

#[tokio::test]
async fn gas_coin_with_address_balance_fallback() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();
    let recipient = Address::ZERO;

    // Fund the sender with a small amount of SUI coins
    sui.fund(&[(sender, 2 * MIST_PER_SUI)]).await?;
    // Deposit a larger amount into their address balance
    sui.deposit_to_address_balance(sender, 10 * MIST_PER_SUI)
        .await?;

    // Request more than the coin balance but within total (coins + AB)
    let request_amount = 5 * MIST_PER_SUI;
    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let coin = builder.intent(CoinWithBalance::sui(request_amount));
    let recipient_arg = builder.pure(&recipient);
    builder.transfer_objects(vec![coin], recipient_arg);
    let transaction = builder.build(&mut sui.client).await?;

    // Coins + AB fallback — should use both coin objects and FundsWithdrawal
    assert!(
        has_funds_withdrawal(&transaction),
        "should use FundsWithdrawal for AB fallback"
    );
    assert!(
        has_coin_objects(&transaction),
        "should still use coin objects"
    );

    execute(&mut sui.client, &private_key, transaction).await?;

    let coins = sui
        .client
        .list_owned_objects(
            ListOwnedObjectsRequest::default()
                .with_owner(recipient)
                .with_object_type(sui_coin_type())
                .with_read_mask(FieldMask::from_str("balance")),
        )
        .try_collect::<Vec<_>>()
        .await?;

    assert_eq!(coins.len(), 1);
    assert_eq!(coins[0].balance(), request_amount);

    Ok(())
}

#[tokio::test]
async fn gas_coin_only_address_balance() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();
    let recipient = Address::ZERO;

    // Only deposit into address balance — no coin objects for this account
    sui.deposit_to_address_balance(sender, 10 * MIST_PER_SUI)
        .await?;

    // Request from AB only
    let request_amount = 3 * MIST_PER_SUI;
    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let coin = builder.intent(CoinWithBalance::sui(request_amount));
    let recipient_arg = builder.pure(&recipient);
    builder.transfer_objects(vec![coin], recipient_arg);
    let transaction = builder.build(&mut sui.client).await?;

    // Pure AB — should use FundsWithdrawal, no coin object inputs
    assert!(
        has_funds_withdrawal(&transaction),
        "should use FundsWithdrawal when no coins available"
    );
    assert!(
        !has_coin_objects(&transaction),
        "should not have coin objects when using pure AB"
    );

    execute(&mut sui.client, &private_key, transaction).await?;

    let coins = sui
        .client
        .list_owned_objects(
            ListOwnedObjectsRequest::default()
                .with_owner(recipient)
                .with_object_type(sui_coin_type())
                .with_read_mask(FieldMask::from_str("balance")),
        )
        .try_collect::<Vec<_>>()
        .await?;

    assert_eq!(coins.len(), 1);
    assert_eq!(coins[0].balance(), request_amount);

    Ok(())
}

#[tokio::test]
async fn non_gas_coin_with_address_balance_fallback() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();
    let recipient = Address::ZERO;

    // Fund the sender with SUI coins (some will be used for gas, some for the request)
    sui.fund(&[(sender, 5 * MIST_PER_SUI)]).await?;
    // Deposit additional SUI into their address balance
    sui.deposit_to_address_balance(sender, 10 * MIST_PER_SUI)
        .await?;

    // Use with_use_gas_coin(false) so it goes through resolve_coin_type
    // Request more than coins hold (excluding gas coin) to trigger AB fallback
    let request_amount = 8 * MIST_PER_SUI;
    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let coin = builder.intent(CoinWithBalance::sui(request_amount).with_use_gas_coin(false));
    let recipient_arg = builder.pure(&recipient);
    builder.transfer_objects(vec![coin], recipient_arg);
    let transaction = builder.build(&mut sui.client).await?;

    // Coins + AB fallback — should use both coin objects and FundsWithdrawal
    assert!(
        has_funds_withdrawal(&transaction),
        "should use FundsWithdrawal for AB fallback"
    );
    assert!(
        has_coin_objects(&transaction),
        "should still use coin objects"
    );

    execute(&mut sui.client, &private_key, transaction).await?;

    let coins = sui
        .client
        .list_owned_objects(
            ListOwnedObjectsRequest::default()
                .with_owner(recipient)
                .with_object_type(sui_coin_type())
                .with_read_mask(FieldMask::from_str("balance")),
        )
        .try_collect::<Vec<_>>()
        .await?;

    assert_eq!(coins.len(), 1);
    assert_eq!(coins[0].balance(), request_amount);

    Ok(())
}

#[tokio::test]
async fn non_gas_coin_only_address_balance() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();
    let recipient = Address::ZERO;

    // Only deposit into address balance — no coin objects for this account.
    // Both gas and the requested coin will come from AB.
    sui.deposit_to_address_balance(sender, 10 * MIST_PER_SUI)
        .await?;

    // Use with_use_gas_coin(false) — no SUI coins exist, so resolve_coin_type
    // must go through the coins.is_empty() branch and use pure AB
    let request_amount = 3 * MIST_PER_SUI;
    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let coin = builder.intent(CoinWithBalance::sui(request_amount).with_use_gas_coin(false));
    let recipient_arg = builder.pure(&recipient);
    builder.transfer_objects(vec![coin], recipient_arg);
    let transaction = builder.build(&mut sui.client).await?;

    // Pure AB — should use FundsWithdrawal, no coin object inputs
    assert!(
        has_funds_withdrawal(&transaction),
        "should use FundsWithdrawal when no coins available"
    );
    assert!(
        !has_coin_objects(&transaction),
        "should not have coin objects when using pure AB"
    );

    execute(&mut sui.client, &private_key, transaction).await?;

    let coins = sui
        .client
        .list_owned_objects(
            ListOwnedObjectsRequest::default()
                .with_owner(recipient)
                .with_object_type(sui_coin_type())
                .with_read_mask(FieldMask::from_str("balance")),
        )
        .try_collect::<Vec<_>>()
        .await?;

    assert_eq!(coins.len(), 1);
    assert_eq!(coins[0].balance(), request_amount);

    Ok(())
}

// --- unhappy-path tests ---

#[tokio::test]
async fn insufficient_balance_gas_coin_path() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (_, sender) = fresh_account();

    // Fund with 2 SUI but request 100 SUI — far exceeds total balance
    sui.fund(&[(sender, 2 * MIST_PER_SUI)]).await?;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let coin = builder.intent(CoinWithBalance::sui(100 * MIST_PER_SUI));
    let recipient_arg = builder.pure(&Address::ZERO);
    builder.transfer_objects(vec![coin], recipient_arg);

    let err = builder.build(&mut sui.client).await.unwrap_err();
    assert!(
        matches!(&err, Error::Input(msg) if msg.contains("does not have sufficient balance")),
        "expected insufficient balance error, got: {err}"
    );

    Ok(())
}

#[tokio::test]
async fn insufficient_balance_non_gas_coin_path() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (_, sender) = fresh_account();

    // Fund with 5 SUI but request 100 SUI via non-gas path
    sui.fund(&[(sender, 5 * MIST_PER_SUI)]).await?;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let coin = builder.intent(CoinWithBalance::sui(100 * MIST_PER_SUI).with_use_gas_coin(false));
    let recipient_arg = builder.pure(&Address::ZERO);
    builder.transfer_objects(vec![coin], recipient_arg);

    let err = builder.build(&mut sui.client).await.unwrap_err();
    assert!(
        matches!(&err, Error::Input(msg) if msg.contains("does not have sufficient balance")),
        "expected insufficient balance error, got: {err}"
    );

    Ok(())
}

#[tokio::test]
async fn insufficient_balance_with_address_balance() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (_, sender) = fresh_account();

    // Fund with 2 SUI coins + 3 SUI in AB = 5 SUI total, but request 50 SUI
    sui.fund(&[(sender, 2 * MIST_PER_SUI)]).await?;
    sui.deposit_to_address_balance(sender, 3 * MIST_PER_SUI)
        .await?;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let coin = builder.intent(CoinWithBalance::sui(50 * MIST_PER_SUI));
    let recipient_arg = builder.pure(&Address::ZERO);
    builder.transfer_objects(vec![coin], recipient_arg);

    let err = builder.build(&mut sui.client).await.unwrap_err();
    assert!(
        matches!(&err, Error::Input(msg) if msg.contains("does not have sufficient balance")),
        "expected insufficient balance error, got: {err}"
    );

    Ok(())
}

#[tokio::test]
async fn zero_balance_account() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (_, sender) = fresh_account();

    // Don't fund the account at all — zero balance
    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let coin = builder.intent(CoinWithBalance::sui(MIST_PER_SUI));
    let recipient_arg = builder.pure(&Address::ZERO);
    builder.transfer_objects(vec![coin], recipient_arg);

    let err = builder.build(&mut sui.client).await.unwrap_err();
    assert!(
        matches!(&err, Error::Input(msg) if msg.contains("does not have sufficient balance")),
        "expected insufficient balance error, got: {err}"
    );

    Ok(())
}

#[tokio::test]
async fn missing_sender() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;

    // Don't set sender
    let mut builder = TransactionBuilder::new();
    let coin = builder.intent(CoinWithBalance::sui(MIST_PER_SUI));
    let recipient_arg = builder.pure(&Address::ZERO);
    builder.transfer_objects(vec![coin], recipient_arg);

    let err = builder.build(&mut sui.client).await.unwrap_err();
    assert!(
        matches!(err, Error::MissingSender),
        "expected MissingSender error, got: {err}"
    );

    Ok(())
}

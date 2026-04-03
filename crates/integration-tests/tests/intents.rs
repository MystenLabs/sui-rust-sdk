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
use sui_sdk_types::Command;
use sui_sdk_types::Identifier;
use sui_sdk_types::Input;
use sui_sdk_types::StructTag;
use sui_sdk_types::TransactionKind;
use sui_transaction_builder::Error;
use sui_transaction_builder::Function;
use sui_transaction_builder::TransactionBuilder;
use sui_transaction_builder::intent::Balance;
use sui_transaction_builder::intent::Coin;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

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

/// Extract the PTB commands from a transaction.
fn ptb_commands(transaction: &sui_sdk_types::Transaction) -> &[Command] {
    match &transaction.kind {
        TransactionKind::ProgrammableTransaction(pt) => &pt.commands,
        other => panic!("expected ProgrammableTransaction, got {other:?}"),
    }
}

fn has_funds_withdrawal(transaction: &sui_sdk_types::Transaction) -> bool {
    ptb_inputs(transaction)
        .iter()
        .any(|i| matches!(i, Input::FundsWithdrawal(_)))
}

/// Check whether the transaction uses coin objects -- either as PTB inputs
/// (non-gas path) or as gas payment objects (gas path).
fn has_coin_objects(transaction: &sui_sdk_types::Transaction) -> bool {
    let has_ptb_coin_inputs = ptb_inputs(transaction)
        .iter()
        .any(|i| matches!(i, Input::ImmutableOrOwned(_)));
    let has_gas_objects = !transaction.gas_payment.objects.is_empty();
    has_ptb_coin_inputs || has_gas_objects
}

/// Check whether the transaction contains a call to the given function.
fn has_move_call(
    transaction: &sui_sdk_types::Transaction,
    package: Address,
    module: &str,
    function: &str,
) -> bool {
    ptb_commands(transaction).iter().any(|cmd| {
        if let Command::MoveCall(call) = cmd {
            call.package == package
                && call.module.as_str() == module
                && call.function.as_str() == function
        } else {
            false
        }
    })
}

/// Helper to sign, execute, and assert success.
async fn execute(
    client: &mut sui_rpc::Client,
    private_key: &Ed25519PrivateKey,
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

/// Build a `coin::from_balance` call so we can transfer a `Balance<SUI>` as
/// a coin for easy verification.
fn balance_to_coin(builder: &mut TransactionBuilder, balance_arg: crate::Argument) -> Argument {
    builder.move_call(
        Function::new(
            Address::TWO,
            Identifier::from_static("coin"),
            Identifier::from_static("from_balance"),
        )
        .with_type_args(vec![StructTag::sui().into()]),
        vec![balance_arg],
    )
}

use sui_transaction_builder::Argument;

/// List SUI coins owned by `owner`, returning `(count, sorted_balances)`.
async fn owned_sui_coins(client: &mut sui_rpc::Client, owner: Address) -> Result<Vec<u64>> {
    let coins = client
        .list_owned_objects(
            ListOwnedObjectsRequest::default()
                .with_owner(owner)
                .with_object_type(sui_coin_type())
                .with_read_mask(FieldMask::from_str("balance")),
        )
        .try_collect::<Vec<_>>()
        .await?;
    let mut balances: Vec<u64> = coins.iter().map(|c| c.balance()).collect();
    balances.sort();
    Ok(balances)
}

// ===========================================================================
// Coin intent tests
// ===========================================================================

#[tokio::test]
async fn coin_basic_single_request() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();
    let recipient = Address::ZERO;

    sui.fund(&[(sender, 5 * MIST_PER_SUI)]).await?;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let coin = builder.intent(Coin::sui(MIST_PER_SUI));
    let recipient_arg = builder.pure(&recipient);
    builder.transfer_objects(vec![coin], recipient_arg);
    let transaction = builder.build(&mut sui.client).await?;

    // Coins are sufficient -- should use coin objects, no FundsWithdrawal.
    assert!(
        !has_funds_withdrawal(&transaction),
        "should not use FundsWithdrawal when coins are sufficient"
    );

    execute(&mut sui.client, &private_key, transaction).await?;

    let balances = owned_sui_coins(&mut sui.client, recipient).await?;
    assert_eq!(balances, [MIST_PER_SUI]);

    Ok(())
}

#[tokio::test]
async fn coin_multiple_amounts_single_transaction() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();
    let recipient = Address::ZERO;

    sui.fund(&[(sender, 20 * MIST_PER_SUI)]).await?;

    let amounts = [MIST_PER_SUI, 2 * MIST_PER_SUI, 3 * MIST_PER_SUI];

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let recipient_arg = builder.pure(&recipient);
    for amount in &amounts {
        let coin = builder.intent(Coin::sui(*amount));
        builder.transfer_objects(vec![coin], recipient_arg);
    }
    let transaction = builder.build(&mut sui.client).await?;

    assert!(!has_funds_withdrawal(&transaction));
    execute(&mut sui.client, &private_key, transaction).await?;

    let balances = owned_sui_coins(&mut sui.client, recipient).await?;
    let mut expected = amounts.to_vec();
    expected.sort();
    assert_eq!(balances, expected);

    Ok(())
}

#[tokio::test]
async fn coin_gas_coin_with_address_balance_fallback() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();
    let recipient = Address::ZERO;

    // 5 SUI in coins, 3 SUI in AB. Request 7 SUI (AB < 7, forces Path 2).
    sui.fund(&[(sender, 5 * MIST_PER_SUI)]).await?;
    sui.deposit_to_address_balance(sender, 3 * MIST_PER_SUI)
        .await?;

    let request_amount = 7 * MIST_PER_SUI;
    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let coin = builder.intent(Coin::sui(request_amount));
    let recipient_arg = builder.pure(&recipient);
    builder.transfer_objects(vec![coin], recipient_arg);
    let transaction = builder.build(&mut sui.client).await?;

    assert!(has_funds_withdrawal(&transaction));
    assert!(has_coin_objects(&transaction));

    execute(&mut sui.client, &private_key, transaction).await?;

    let balances = owned_sui_coins(&mut sui.client, recipient).await?;
    assert_eq!(balances, [request_amount]);

    Ok(())
}

#[tokio::test]
async fn coin_gas_coin_only_address_balance() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();
    let recipient = Address::ZERO;

    // Only deposit into address balance -- no coin objects for this account.
    sui.deposit_to_address_balance(sender, 10 * MIST_PER_SUI)
        .await?;

    let request_amount = 3 * MIST_PER_SUI;
    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let coin = builder.intent(Coin::sui(request_amount));
    let recipient_arg = builder.pure(&recipient);
    builder.transfer_objects(vec![coin], recipient_arg);
    let transaction = builder.build(&mut sui.client).await?;

    assert!(has_funds_withdrawal(&transaction));
    assert!(!has_coin_objects(&transaction));

    execute(&mut sui.client, &private_key, transaction).await?;

    let balances = owned_sui_coins(&mut sui.client, recipient).await?;
    assert_eq!(balances, [request_amount]);

    Ok(())
}

#[tokio::test]
async fn coin_non_gas_with_address_balance_fallback() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();
    let recipient = Address::ZERO;

    // 5 SUI in coins, 3 SUI in AB. Request 6 SUI (AB < 6, forces Path 2).
    sui.fund(&[(sender, 5 * MIST_PER_SUI)]).await?;
    sui.deposit_to_address_balance(sender, 3 * MIST_PER_SUI)
        .await?;

    let request_amount = 6 * MIST_PER_SUI;
    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let coin = builder.intent(Coin::sui(request_amount).with_use_gas_coin(false));
    let recipient_arg = builder.pure(&recipient);
    builder.transfer_objects(vec![coin], recipient_arg);
    let transaction = builder.build(&mut sui.client).await?;

    assert!(has_funds_withdrawal(&transaction));
    assert!(has_coin_objects(&transaction));

    execute(&mut sui.client, &private_key, transaction).await?;

    let balances = owned_sui_coins(&mut sui.client, recipient).await?;
    assert_eq!(balances, [request_amount]);

    Ok(())
}

#[tokio::test]
async fn coin_non_gas_only_address_balance() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();
    let recipient = Address::ZERO;

    // Only AB, no coins. use_gas_coin(false) -> resolve_coin_type.
    sui.deposit_to_address_balance(sender, 10 * MIST_PER_SUI)
        .await?;

    let request_amount = 3 * MIST_PER_SUI;
    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let coin = builder.intent(Coin::sui(request_amount).with_use_gas_coin(false));
    let recipient_arg = builder.pure(&recipient);
    builder.transfer_objects(vec![coin], recipient_arg);
    let transaction = builder.build(&mut sui.client).await?;

    assert!(has_funds_withdrawal(&transaction));
    assert!(!has_coin_objects(&transaction));

    execute(&mut sui.client, &private_key, transaction).await?;

    let balances = owned_sui_coins(&mut sui.client, recipient).await?;
    assert_eq!(balances, [request_amount]);

    Ok(())
}

#[tokio::test]
async fn coin_zero_value_request() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let private_key = sui.user_keys.first().unwrap();
    let sender = private_key.public_key().derive_address();
    let recipient = Address::ZERO;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let arg = builder.intent(Coin::sui(0));
    let recipient_address = builder.pure(&recipient);
    builder.transfer_objects(vec![arg], recipient_address);
    let transaction = builder.build(&mut sui.client).await?;

    assert!(
        has_move_call(&transaction, Address::TWO, "coin", "zero"),
        "should use coin::zero for zero-value Coin intent"
    );
    assert!(
        !has_move_call(&transaction, Address::TWO, "balance", "zero"),
        "should NOT use balance::zero for Coin intent"
    );

    execute(&mut sui.client, private_key, transaction).await?;

    let balances = owned_sui_coins(&mut sui.client, recipient).await?;
    assert_eq!(balances, [0]);

    Ok(())
}

#[tokio::test]
async fn coin_large_number_of_requests() -> Result<()> {
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

    // Build a request that requires filling out gas coins and multiple
    // merge_coins.
    let mut builder = TransactionBuilder::new();
    builder.set_sender(recipient);
    let arg = builder.intent(Coin::sui(950));
    let self_address = builder.pure(&recipient);
    builder.transfer_objects(vec![arg], self_address);
    builder.build(&mut sui.client).await.unwrap();

    // Build a request that doesn't use the gas coin but requires multiple
    // merge_coins.
    let mut builder = TransactionBuilder::new();
    builder.set_sender(recipient);
    let arg = builder.intent(Coin::sui(950).with_use_gas_coin(false));
    let self_address = builder.pure(&recipient);
    builder.transfer_objects(vec![arg], self_address);
    builder.build(&mut sui.client).await.unwrap();
    Ok(())
}

/// The CoinWithBalance type alias should work identically to Coin.
#[tokio::test]
async fn coin_with_balance_alias_works() -> Result<()> {
    use sui_transaction_builder::intent::CoinWithBalance;

    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();

    sui.fund(&[(sender, 5 * MIST_PER_SUI)]).await?;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let coin = builder.intent(CoinWithBalance::sui(MIST_PER_SUI));
    let recipient = builder.pure(&Address::ZERO);
    builder.transfer_objects(vec![coin], recipient);
    let transaction = builder.build(&mut sui.client).await?;

    execute(&mut sui.client, &private_key, transaction).await?;

    let balances = owned_sui_coins(&mut sui.client, Address::ZERO).await?;
    assert_eq!(balances, [MIST_PER_SUI]);

    Ok(())
}

// ---------------------------------------------------------------------------
// Coin intent -- remainder handling
// ---------------------------------------------------------------------------

/// Non-gas coin path with Coin intents and AB used should send the
/// remainder back to AB via coin::send_funds.
#[tokio::test]
async fn coin_remainder_sent_to_ab_when_ab_used() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();

    // 2 SUI in coins, 5 SUI in AB. Request 5 SUI (AB < 5... wait,
    // AB=5 >= 5 so Path 1). Use sum > AB to force Path 2.
    // 5 SUI in coins, 3 SUI in AB. Request 6 SUI.
    sui.fund(&[(sender, 5 * MIST_PER_SUI)]).await?;
    sui.deposit_to_address_balance(sender, 3 * MIST_PER_SUI)
        .await?;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let coin = builder.intent(Coin::sui(6 * MIST_PER_SUI).with_use_gas_coin(false));
    let recipient = builder.pure(&Address::ZERO);
    builder.transfer_objects(vec![coin], recipient);

    let transaction = builder.build(&mut sui.client).await?;

    // Coin-only with AB shortfall: remainder (from consolidated coins)
    // sent back to AB.
    assert!(has_funds_withdrawal(&transaction));
    assert!(
        has_move_call(&transaction, Address::TWO, "coin", "send_funds"),
        "should call coin::send_funds for remainder when AB is used"
    );

    execute(&mut sui.client, &private_key, transaction).await?;

    let balances = owned_sui_coins(&mut sui.client, Address::ZERO).await?;
    assert_eq!(balances, [6 * MIST_PER_SUI]);

    Ok(())
}

// ---------------------------------------------------------------------------
// Coin intent -- error cases
// ---------------------------------------------------------------------------

#[tokio::test]
async fn coin_insufficient_balance_gas_path() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (_, sender) = fresh_account();

    sui.fund(&[(sender, 2 * MIST_PER_SUI)]).await?;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let coin = builder.intent(Coin::sui(100 * MIST_PER_SUI));
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
async fn coin_insufficient_balance_non_gas_path() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (_, sender) = fresh_account();

    sui.fund(&[(sender, 5 * MIST_PER_SUI)]).await?;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let coin = builder.intent(Coin::sui(100 * MIST_PER_SUI).with_use_gas_coin(false));
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
async fn coin_insufficient_balance_with_address_balance() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (_, sender) = fresh_account();

    sui.fund(&[(sender, 2 * MIST_PER_SUI)]).await?;
    sui.deposit_to_address_balance(sender, 3 * MIST_PER_SUI)
        .await?;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let coin = builder.intent(Coin::sui(50 * MIST_PER_SUI));
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
async fn coin_zero_balance_account() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (_, sender) = fresh_account();

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let coin = builder.intent(Coin::sui(MIST_PER_SUI));
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
async fn coin_missing_sender() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;

    let mut builder = TransactionBuilder::new();
    let coin = builder.intent(Coin::sui(MIST_PER_SUI));
    let recipient_arg = builder.pure(&Address::ZERO);
    builder.transfer_objects(vec![coin], recipient_arg);

    let err = builder.build(&mut sui.client).await.unwrap_err();
    assert!(
        matches!(err, Error::MissingSender),
        "expected MissingSender error, got: {err}"
    );

    Ok(())
}

// ===========================================================================
// Balance intent tests -- path 1 (direct withdrawal)
// ===========================================================================

/// Single Balance intent fulfilled entirely from address balance.
#[tokio::test]
async fn balance_direct_withdrawal_from_ab() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();

    // Only deposit into address balance -- no coin objects.
    sui.deposit_to_address_balance(sender, 10 * MIST_PER_SUI)
        .await?;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let bal = builder.intent(Balance::sui(MIST_PER_SUI));
    let coin = balance_to_coin(&mut builder, bal);
    let recipient = builder.pure(&Address::ZERO);
    builder.transfer_objects(vec![coin], recipient);

    let transaction = builder.build(&mut sui.client).await?;

    // Path 1: should use FundsWithdrawal (balance::redeem_funds), no coin
    // objects.
    assert!(has_funds_withdrawal(&transaction));
    assert!(!has_coin_objects(&transaction));
    assert!(has_move_call(
        &transaction,
        Address::TWO,
        "balance",
        "redeem_funds"
    ));
    assert!(!has_move_call(
        &transaction,
        Address::TWO,
        "coin",
        "redeem_funds"
    ));

    execute(&mut sui.client, &private_key, transaction).await?;

    let balances = owned_sui_coins(&mut sui.client, Address::ZERO).await?;
    assert_eq!(balances, [MIST_PER_SUI]);

    Ok(())
}

/// Multiple Balance intents of SUI, all fulfilled from AB (path 1).
#[tokio::test]
async fn balance_multiple_direct_withdrawal() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();

    sui.deposit_to_address_balance(sender, 20 * MIST_PER_SUI)
        .await?;

    let amounts = [MIST_PER_SUI, 2 * MIST_PER_SUI, 3 * MIST_PER_SUI];

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let recipient = builder.pure(&Address::ZERO);

    for amount in &amounts {
        let bal = builder.intent(Balance::sui(*amount));
        let coin = balance_to_coin(&mut builder, bal);
        builder.transfer_objects(vec![coin], recipient);
    }

    let transaction = builder.build(&mut sui.client).await?;

    assert!(has_funds_withdrawal(&transaction));
    assert!(!has_coin_objects(&transaction));

    execute(&mut sui.client, &private_key, transaction).await?;

    let balances = owned_sui_coins(&mut sui.client, Address::ZERO).await?;
    let mut expected = amounts.to_vec();
    expected.sort();
    assert_eq!(balances, expected);

    Ok(())
}

/// Balance intent with only AB, gas coin enabled -- still takes path 1 since
/// all intents are Balance and AB is sufficient.
#[tokio::test]
async fn balance_gas_coin_path_only_ab() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();

    sui.deposit_to_address_balance(sender, 10 * MIST_PER_SUI)
        .await?;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let bal = builder.intent(Balance::sui(3 * MIST_PER_SUI));
    let coin = balance_to_coin(&mut builder, bal);
    let recipient = builder.pure(&Address::ZERO);
    builder.transfer_objects(vec![coin], recipient);

    let transaction = builder.build(&mut sui.client).await?;

    // All Balance intents + AB sufficient = path 1.
    assert!(has_funds_withdrawal(&transaction));
    assert!(!has_coin_objects(&transaction));
    assert!(has_move_call(
        &transaction,
        Address::TWO,
        "balance",
        "redeem_funds"
    ));

    execute(&mut sui.client, &private_key, transaction).await?;

    let balances = owned_sui_coins(&mut sui.client, Address::ZERO).await?;
    assert_eq!(balances, [3 * MIST_PER_SUI]);

    Ok(())
}

// ===========================================================================
// Balance intent tests -- path 2 (merge and split)
// ===========================================================================

/// Balance intent uses the gas coin path (path 2) when AB is insufficient.
#[tokio::test]
async fn balance_gas_coin_fallback() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();

    // Fund with coin objects, no AB.
    sui.fund(&[(sender, 5 * MIST_PER_SUI)]).await?;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let bal = builder.intent(Balance::sui(MIST_PER_SUI));
    let coin = balance_to_coin(&mut builder, bal);
    let recipient = builder.pure(&Address::ZERO);
    builder.transfer_objects(vec![coin], recipient);

    let transaction = builder.build(&mut sui.client).await?;

    assert!(has_coin_objects(&transaction));
    assert!(has_move_call(
        &transaction,
        Address::TWO,
        "coin",
        "into_balance"
    ));

    execute(&mut sui.client, &private_key, transaction).await?;

    let balances = owned_sui_coins(&mut sui.client, Address::ZERO).await?;
    assert_eq!(balances, [MIST_PER_SUI]);

    Ok(())
}

/// Balance intent with gas coin + AB fallback (path 2). AB must be less than
/// total requested to force path 2.
#[tokio::test]
async fn balance_gas_coin_with_ab_fallback() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();

    // 5 SUI in coins, 3 SUI in AB. Request 7 SUI (AB < 7, forces path 2).
    sui.fund(&[(sender, 5 * MIST_PER_SUI)]).await?;
    sui.deposit_to_address_balance(sender, 3 * MIST_PER_SUI)
        .await?;

    let request_amount = 7 * MIST_PER_SUI;
    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let bal = builder.intent(Balance::sui(request_amount));
    let coin = balance_to_coin(&mut builder, bal);
    let recipient = builder.pure(&Address::ZERO);
    builder.transfer_objects(vec![coin], recipient);

    let transaction = builder.build(&mut sui.client).await?;

    assert!(has_coin_objects(&transaction));
    assert!(has_funds_withdrawal(&transaction));
    assert!(has_move_call(
        &transaction,
        Address::TWO,
        "coin",
        "into_balance"
    ));

    execute(&mut sui.client, &private_key, transaction).await?;

    let balances = owned_sui_coins(&mut sui.client, Address::ZERO).await?;
    assert_eq!(balances, [request_amount]);

    Ok(())
}

/// Balance intent with use_gas_coin(false) forces the non-gas coin path
/// (path 2). AB < total forces coin usage.
#[tokio::test]
async fn balance_non_gas_coin_fallback() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();

    // 5 SUI in coins, 3 SUI in AB. sum=4 (2 Balance + 2 Coin), AB < 4
    // so Path 2.
    sui.fund(&[(sender, 5 * MIST_PER_SUI)]).await?;
    sui.deposit_to_address_balance(sender, 3 * MIST_PER_SUI)
        .await?;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let bal = builder.intent(Balance::sui(2 * MIST_PER_SUI).with_use_gas_coin(false));
    let coin_intent = builder.intent(Coin::sui(2 * MIST_PER_SUI).with_use_gas_coin(false));
    let bal_coin = balance_to_coin(&mut builder, bal);
    let recipient = builder.pure(&Address::ZERO);
    builder.transfer_objects(vec![bal_coin, coin_intent], recipient);

    let transaction = builder.build(&mut sui.client).await?;

    assert!(has_coin_objects(&transaction));
    assert!(has_move_call(
        &transaction,
        Address::TWO,
        "coin",
        "into_balance"
    ));

    execute(&mut sui.client, &private_key, transaction).await?;

    let balances = owned_sui_coins(&mut sui.client, Address::ZERO).await?;
    assert_eq!(balances, [2 * MIST_PER_SUI, 2 * MIST_PER_SUI]);

    Ok(())
}

/// Balance intent with coins + AB fallback (non-gas path, path 2). AB must
/// be less than total requested to avoid path 1.
#[tokio::test]
async fn balance_non_gas_coin_with_ab_fallback() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();

    // 5 SUI in coins, 3 SUI in AB. Request 6 SUI (AB < 6, so path 2).
    sui.fund(&[(sender, 5 * MIST_PER_SUI)]).await?;
    sui.deposit_to_address_balance(sender, 3 * MIST_PER_SUI)
        .await?;

    let request_amount = 6 * MIST_PER_SUI;
    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let bal = builder.intent(Balance::sui(request_amount).with_use_gas_coin(false));
    let coin = balance_to_coin(&mut builder, bal);
    let recipient = builder.pure(&Address::ZERO);
    builder.transfer_objects(vec![coin], recipient);

    let transaction = builder.build(&mut sui.client).await?;

    assert!(has_funds_withdrawal(&transaction));
    assert!(has_coin_objects(&transaction));
    assert!(has_move_call(
        &transaction,
        Address::TWO,
        "coin",
        "into_balance"
    ));

    execute(&mut sui.client, &private_key, transaction).await?;

    let balances = owned_sui_coins(&mut sui.client, Address::ZERO).await?;
    assert_eq!(balances, [request_amount]);

    Ok(())
}

// ===========================================================================
// Balance intent -- remainder handling
// ===========================================================================

/// Non-gas coin path with Balance intents should send remainder to AB via
/// coin::send_funds.
#[tokio::test]
async fn balance_remainder_sent_to_ab() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();

    // Fund with more coins than needed so there's a surplus after splitting.
    // AB is insufficient so that path 2 is taken.
    sui.fund(&[(sender, 10 * MIST_PER_SUI)]).await?;
    sui.deposit_to_address_balance(sender, 5 * MIST_PER_SUI)
        .await?;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    // AB (5) < request (8) so path 2 is taken; coins are surplus.
    let bal = builder.intent(Balance::sui(8 * MIST_PER_SUI).with_use_gas_coin(false));
    let coin = balance_to_coin(&mut builder, bal);
    let recipient = builder.pure(&Address::ZERO);
    builder.transfer_objects(vec![coin], recipient);

    let transaction = builder.build(&mut sui.client).await?;

    assert!(has_move_call(
        &transaction,
        Address::TWO,
        "coin",
        "send_funds"
    ));
    assert!(has_move_call(
        &transaction,
        Address::TWO,
        "coin",
        "into_balance"
    ));

    execute(&mut sui.client, &private_key, transaction).await?;

    Ok(())
}

// ===========================================================================
// Mixed Coin + Balance intents (always path 2)
// ===========================================================================

/// Mix of Coin and Balance intents for SUI in a single transaction.
#[tokio::test]
async fn mixed_coin_and_balance_intents() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();

    sui.fund(&[(sender, 10 * MIST_PER_SUI)]).await?;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let recipient = builder.pure(&Address::ZERO);

    let coin = builder.intent(Coin::sui(MIST_PER_SUI));
    builder.transfer_objects(vec![coin], recipient);

    let bal = builder.intent(Balance::sui(2 * MIST_PER_SUI));
    let bal_coin = balance_to_coin(&mut builder, bal);
    builder.transfer_objects(vec![bal_coin], recipient);

    let transaction = builder.build(&mut sui.client).await?;

    assert!(has_move_call(
        &transaction,
        Address::TWO,
        "coin",
        "into_balance"
    ));

    execute(&mut sui.client, &private_key, transaction).await?;

    let balances = owned_sui_coins(&mut sui.client, Address::ZERO).await?;
    assert_eq!(balances, [MIST_PER_SUI, 2 * MIST_PER_SUI]);

    Ok(())
}

/// Mix of Coin and Balance intents with AB fallback.
#[tokio::test]
async fn mixed_coin_and_balance_with_ab_fallback() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();

    // 5 SUI in coins, 3 SUI in AB. sum=7 (3 Coin + 4 Balance), AB < 7.
    sui.fund(&[(sender, 5 * MIST_PER_SUI)]).await?;
    sui.deposit_to_address_balance(sender, 3 * MIST_PER_SUI)
        .await?;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let recipient = builder.pure(&Address::ZERO);

    let coin = builder.intent(Coin::sui(3 * MIST_PER_SUI));
    builder.transfer_objects(vec![coin], recipient);

    let bal = builder.intent(Balance::sui(4 * MIST_PER_SUI));
    let bal_coin = balance_to_coin(&mut builder, bal);
    builder.transfer_objects(vec![bal_coin], recipient);

    let transaction = builder.build(&mut sui.client).await?;

    assert!(has_funds_withdrawal(&transaction));
    assert!(has_coin_objects(&transaction));
    assert!(has_move_call(
        &transaction,
        Address::TWO,
        "coin",
        "into_balance"
    ));

    execute(&mut sui.client, &private_key, transaction).await?;

    let balances = owned_sui_coins(&mut sui.client, Address::ZERO).await?;
    assert_eq!(balances, [3 * MIST_PER_SUI, 4 * MIST_PER_SUI]);

    Ok(())
}

// ===========================================================================
// Zero-balance intents
// ===========================================================================

/// Zero-balance Balance intent uses `balance::zero`.
#[tokio::test]
async fn balance_zero_value() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let private_key = sui.user_keys.first().unwrap();
    let sender = private_key.public_key().derive_address();

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let bal = builder.intent(Balance::sui(0));
    let coin = balance_to_coin(&mut builder, bal);
    let recipient = builder.pure(&Address::ZERO);
    builder.transfer_objects(vec![coin], recipient);

    let transaction = builder.build(&mut sui.client).await?;

    assert!(has_move_call(&transaction, Address::TWO, "balance", "zero"));
    assert!(!has_move_call(&transaction, Address::TWO, "coin", "zero"));

    execute(&mut sui.client, private_key, transaction).await?;

    let balances = owned_sui_coins(&mut sui.client, Address::ZERO).await?;
    assert_eq!(balances, [0]);

    Ok(())
}

/// Mixed zero and non-zero Balance intents.
#[tokio::test]
async fn balance_mixed_zero_and_nonzero() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (private_key, sender) = fresh_account();

    sui.deposit_to_address_balance(sender, 10 * MIST_PER_SUI)
        .await?;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let recipient = builder.pure(&Address::ZERO);

    let zero_bal = builder.intent(Balance::sui(0));
    let zero_coin = balance_to_coin(&mut builder, zero_bal);
    builder.transfer_objects(vec![zero_coin], recipient);

    let nonzero_bal = builder.intent(Balance::sui(MIST_PER_SUI));
    let nonzero_coin = balance_to_coin(&mut builder, nonzero_bal);
    builder.transfer_objects(vec![nonzero_coin], recipient);

    let transaction = builder.build(&mut sui.client).await?;

    assert!(has_move_call(&transaction, Address::TWO, "balance", "zero"));
    assert!(has_move_call(
        &transaction,
        Address::TWO,
        "balance",
        "redeem_funds"
    ));

    execute(&mut sui.client, &private_key, transaction).await?;

    let balances = owned_sui_coins(&mut sui.client, Address::ZERO).await?;
    assert_eq!(balances, [0, MIST_PER_SUI]);

    Ok(())
}

// ===========================================================================
// Balance intent -- error cases
// ===========================================================================

#[tokio::test]
async fn balance_insufficient_balance() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (_, sender) = fresh_account();

    sui.fund(&[(sender, 2 * MIST_PER_SUI)]).await?;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let bal = builder.intent(Balance::sui(100 * MIST_PER_SUI));
    let coin = balance_to_coin(&mut builder, bal);
    let recipient = builder.pure(&Address::ZERO);
    builder.transfer_objects(vec![coin], recipient);

    let err = builder.build(&mut sui.client).await.unwrap_err();
    assert!(
        matches!(&err, Error::Input(msg) if msg.contains("does not have sufficient balance")),
        "expected insufficient balance error, got: {err}"
    );

    Ok(())
}

#[tokio::test]
async fn balance_missing_sender() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;

    let mut builder = TransactionBuilder::new();
    let bal = builder.intent(Balance::sui(MIST_PER_SUI));
    let coin = balance_to_coin(&mut builder, bal);
    let recipient = builder.pure(&Address::ZERO);
    builder.transfer_objects(vec![coin], recipient);

    let err = builder.build(&mut sui.client).await.unwrap_err();
    assert!(
        matches!(err, Error::MissingSender),
        "expected MissingSender error, got: {err}"
    );

    Ok(())
}

#[tokio::test]
async fn balance_zero_balance_account() -> Result<()> {
    let mut sui = SuiNetworkBuilder::default().build().await?;
    let (_, sender) = fresh_account();

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let bal = builder.intent(Balance::sui(MIST_PER_SUI));
    let coin = balance_to_coin(&mut builder, bal);
    let recipient = builder.pure(&Address::ZERO);
    builder.transfer_objects(vec![coin], recipient);

    let err = builder.build(&mut sui.client).await.unwrap_err();
    assert!(
        matches!(&err, Error::Input(msg) if msg.contains("does not have sufficient balance")),
        "expected insufficient balance error, got: {err}"
    );

    Ok(())
}

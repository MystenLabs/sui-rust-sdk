(function() {
    var implementors = Object.fromEntries([["sui_graphql_client",[["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.ActiveValidatorsQuery.html\" title=\"struct sui_graphql_client::query_types::ActiveValidatorsQuery\">ActiveValidatorsQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.Balance.html\" title=\"struct sui_graphql_client::query_types::Balance\">Balance</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.BalanceQuery.html\" title=\"struct sui_graphql_client::query_types::BalanceQuery\">BalanceQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.ChainIdentifierQuery.html\" title=\"struct sui_graphql_client::query_types::ChainIdentifierQuery\">ChainIdentifierQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.CheckpointQuery.html\" title=\"struct sui_graphql_client::query_types::CheckpointQuery\">CheckpointQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.CheckpointTotalTxQuery.html\" title=\"struct sui_graphql_client::query_types::CheckpointTotalTxQuery\">CheckpointTotalTxQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.CheckpointsQuery.html\" title=\"struct sui_graphql_client::query_types::CheckpointsQuery\">CheckpointsQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.CoinMetadata.html\" title=\"struct sui_graphql_client::query_types::CoinMetadata\">CoinMetadata</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.CoinMetadataQuery.html\" title=\"struct sui_graphql_client::query_types::CoinMetadataQuery\">CoinMetadataQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.DefaultSuinsNameQuery.html\" title=\"struct sui_graphql_client::query_types::DefaultSuinsNameQuery\">DefaultSuinsNameQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.DryRunQuery.html\" title=\"struct sui_graphql_client::query_types::DryRunQuery\">DryRunQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.DryRunResult.html\" title=\"struct sui_graphql_client::query_types::DryRunResult\">DryRunResult</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.DynamicFieldQuery.html\" title=\"struct sui_graphql_client::query_types::DynamicFieldQuery\">DynamicFieldQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.DynamicFieldsOwnerQuery.html\" title=\"struct sui_graphql_client::query_types::DynamicFieldsOwnerQuery\">DynamicFieldsOwnerQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.DynamicObjectFieldQuery.html\" title=\"struct sui_graphql_client::query_types::DynamicObjectFieldQuery\">DynamicObjectFieldQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.Epoch.html\" title=\"struct sui_graphql_client::query_types::Epoch\">Epoch</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.EpochSummaryQuery.html\" title=\"struct sui_graphql_client::query_types::EpochSummaryQuery\">EpochSummaryQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.EpochValidator.html\" title=\"struct sui_graphql_client::query_types::EpochValidator\">EpochValidator</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.Event.html\" title=\"struct sui_graphql_client::query_types::Event\">Event</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.EventConnection.html\" title=\"struct sui_graphql_client::query_types::EventConnection\">EventConnection</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.EventsQuery.html\" title=\"struct sui_graphql_client::query_types::EventsQuery\">EventsQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.ExecuteTransactionQuery.html\" title=\"struct sui_graphql_client::query_types::ExecuteTransactionQuery\">ExecuteTransactionQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.ExecutionResult.html\" title=\"struct sui_graphql_client::query_types::ExecutionResult\">ExecutionResult</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.GQLAddress.html\" title=\"struct sui_graphql_client::query_types::GQLAddress\">GQLAddress</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.LatestPackageQuery.html\" title=\"struct sui_graphql_client::query_types::LatestPackageQuery\">LatestPackageQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.MoveFunction.html\" title=\"struct sui_graphql_client::query_types::MoveFunction\">MoveFunction</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.MoveFunctionTypeParameter.html\" title=\"struct sui_graphql_client::query_types::MoveFunctionTypeParameter\">MoveFunctionTypeParameter</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.MoveModule.html\" title=\"struct sui_graphql_client::query_types::MoveModule\">MoveModule</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.MoveObject.html\" title=\"struct sui_graphql_client::query_types::MoveObject\">MoveObject</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.MoveObjectContents.html\" title=\"struct sui_graphql_client::query_types::MoveObjectContents\">MoveObjectContents</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.MovePackage.html\" title=\"struct sui_graphql_client::query_types::MovePackage\">MovePackage</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.MoveType.html\" title=\"struct sui_graphql_client::query_types::MoveType\">MoveType</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.MoveValue.html\" title=\"struct sui_graphql_client::query_types::MoveValue\">MoveValue</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.NormalizedMoveFunctionQuery.html\" title=\"struct sui_graphql_client::query_types::NormalizedMoveFunctionQuery\">NormalizedMoveFunctionQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.NormalizedMoveModuleQuery.html\" title=\"struct sui_graphql_client::query_types::NormalizedMoveModuleQuery\">NormalizedMoveModuleQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.ObjectQuery.html\" title=\"struct sui_graphql_client::query_types::ObjectQuery\">ObjectQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.ObjectsQuery.html\" title=\"struct sui_graphql_client::query_types::ObjectsQuery\">ObjectsQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.OpenMoveType.html\" title=\"struct sui_graphql_client::query_types::OpenMoveType\">OpenMoveType</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.Owner.html\" title=\"struct sui_graphql_client::query_types::Owner\">Owner</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.PackageByNameQuery.html\" title=\"struct sui_graphql_client::query_types::PackageByNameQuery\">PackageByNameQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.PackageQuery.html\" title=\"struct sui_graphql_client::query_types::PackageQuery\">PackageQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.PackageVersionsQuery.html\" title=\"struct sui_graphql_client::query_types::PackageVersionsQuery\">PackageVersionsQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.PackagesQuery.html\" title=\"struct sui_graphql_client::query_types::PackagesQuery\">PackagesQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.PageInfo.html\" title=\"struct sui_graphql_client::query_types::PageInfo\">PageInfo</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.ProtocolConfigQuery.html\" title=\"struct sui_graphql_client::query_types::ProtocolConfigQuery\">ProtocolConfigQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.ProtocolConfigs.html\" title=\"struct sui_graphql_client::query_types::ProtocolConfigs\">ProtocolConfigs</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.ResolveSuinsQuery.html\" title=\"struct sui_graphql_client::query_types::ResolveSuinsQuery\">ResolveSuinsQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.ServiceConfig.html\" title=\"struct sui_graphql_client::query_types::ServiceConfig\">ServiceConfig</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.ServiceConfigQuery.html\" title=\"struct sui_graphql_client::query_types::ServiceConfigQuery\">ServiceConfigQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.TransactionBlock.html\" title=\"struct sui_graphql_client::query_types::TransactionBlock\">TransactionBlock</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.TransactionBlockEffectsQuery.html\" title=\"struct sui_graphql_client::query_types::TransactionBlockEffectsQuery\">TransactionBlockEffectsQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.TransactionBlockQuery.html\" title=\"struct sui_graphql_client::query_types::TransactionBlockQuery\">TransactionBlockQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.TransactionBlocksEffectsQuery.html\" title=\"struct sui_graphql_client::query_types::TransactionBlocksEffectsQuery\">TransactionBlocksEffectsQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.TransactionBlocksQuery.html\" title=\"struct sui_graphql_client::query_types::TransactionBlocksQuery\">TransactionBlocksQuery</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.Validator.html\" title=\"struct sui_graphql_client::query_types::Validator\">Validator</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.ValidatorConnection.html\" title=\"struct sui_graphql_client::query_types::ValidatorConnection\">ValidatorConnection</a>"],["impl QueryFragment for <a class=\"struct\" href=\"sui_graphql_client/query_types/struct.ValidatorSet.html\" title=\"struct sui_graphql_client::query_types::ValidatorSet\">ValidatorSet</a>"]]]]);
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()
//{"start":57,"fragment_lengths":[11574]}
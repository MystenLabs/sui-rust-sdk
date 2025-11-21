use super::*;
use crate::proto::TryFromProtoError;
use tap::Pipe;

//
// ExecutionStatus
//

impl From<sui_sdk_types::ExecutionStatus> for ExecutionStatus {
    fn from(value: sui_sdk_types::ExecutionStatus) -> Self {
        match value {
            sui_sdk_types::ExecutionStatus::Success => Self {
                success: Some(true),
                error: None,
            },
            sui_sdk_types::ExecutionStatus::Failure { error, command } => {
                let mut error_message = ExecutionError::from(error);
                error_message.command = command;
                Self {
                    success: Some(false),
                    error: Some(error_message),
                }
            }
        }
    }
}

impl TryFrom<&ExecutionStatus> for sui_sdk_types::ExecutionStatus {
    type Error = TryFromProtoError;

    fn try_from(value: &ExecutionStatus) -> Result<Self, Self::Error> {
        let success = value
            .success
            .ok_or_else(|| TryFromProtoError::missing(ExecutionStatus::SUCCESS_FIELD))?;
        match (success, &value.error) {
            (true, None) => Self::Success,
            (false, Some(error)) => Self::Failure {
                error: sui_sdk_types::ExecutionError::try_from(error)
                    .map_err(|e| e.nested(ExecutionStatus::ERROR_FIELD))?,
                command: error.command,
            },
            (true, Some(_)) | (false, None) => {
                return Err(TryFromProtoError::invalid(
                    ExecutionStatus::ERROR_FIELD,
                    "invalid execution status",
                ));
            }
        }
        .pipe(Ok)
    }
}

//
// ExecutionError
//

impl From<sui_sdk_types::ExecutionError> for ExecutionError {
    fn from(value: sui_sdk_types::ExecutionError) -> Self {
        use execution_error::ErrorDetails;
        use execution_error::ExecutionErrorKind;
        use sui_sdk_types::ExecutionError as E;

        let mut message = Self::default();

        let kind = match value {
            E::InsufficientGas => ExecutionErrorKind::InsufficientGas,
            E::InvalidGasObject => ExecutionErrorKind::InvalidGasObject,
            E::InvariantViolation => ExecutionErrorKind::InvariantViolation,
            E::FeatureNotYetSupported => ExecutionErrorKind::FeatureNotYetSupported,
            E::ObjectTooBig {
                object_size,
                max_object_size,
            } => {
                message.error_details = Some(ErrorDetails::SizeError(SizeError {
                    size: Some(object_size),
                    max_size: Some(max_object_size),
                }));
                ExecutionErrorKind::ObjectTooBig
            }
            E::PackageTooBig {
                object_size,
                max_object_size,
            } => {
                message.error_details = Some(ErrorDetails::SizeError(SizeError {
                    size: Some(object_size),
                    max_size: Some(max_object_size),
                }));
                ExecutionErrorKind::PackageTooBig
            }
            E::CircularObjectOwnership { object } => {
                message.error_details = Some(ErrorDetails::ObjectId(object.to_string()));
                ExecutionErrorKind::CircularObjectOwnership
            }
            E::InsufficientCoinBalance => ExecutionErrorKind::InsufficientCoinBalance,
            E::CoinBalanceOverflow => ExecutionErrorKind::CoinBalanceOverflow,
            E::PublishErrorNonZeroAddress => ExecutionErrorKind::PublishErrorNonZeroAddress,
            E::SuiMoveVerificationError => ExecutionErrorKind::SuiMoveVerificationError,
            E::MovePrimitiveRuntimeError { location } => {
                message.error_details = location.map(|l| {
                    ErrorDetails::Abort(MoveAbort {
                        location: Some(l.into()),
                        ..Default::default()
                    })
                });
                ExecutionErrorKind::MovePrimitiveRuntimeError
            }
            E::MoveAbort { location, code } => {
                message.error_details = Some(ErrorDetails::Abort(MoveAbort {
                    abort_code: Some(code),
                    location: Some(location.into()),
                    clever_error: None,
                }));
                ExecutionErrorKind::MoveAbort
            }
            E::VmVerificationOrDeserializationError => {
                ExecutionErrorKind::VmVerificationOrDeserializationError
            }
            E::VmInvariantViolation => ExecutionErrorKind::VmInvariantViolation,
            E::FunctionNotFound => ExecutionErrorKind::FunctionNotFound,
            E::ArityMismatch => ExecutionErrorKind::ArityMismatch,
            E::TypeArityMismatch => ExecutionErrorKind::TypeArityMismatch,
            E::NonEntryFunctionInvoked => ExecutionErrorKind::NonEntryFunctionInvoked,
            E::CommandArgumentError { argument, kind } => {
                let mut command_argument_error = CommandArgumentError::from(kind);
                command_argument_error.argument = Some(argument.into());
                message.error_details =
                    Some(ErrorDetails::CommandArgumentError(command_argument_error));
                ExecutionErrorKind::CommandArgumentError
            }
            E::TypeArgumentError {
                type_argument,
                kind,
            } => {
                let type_argument_error = TypeArgumentError {
                    type_argument: Some(type_argument.into()),
                    kind: Some(type_argument_error::TypeArgumentErrorKind::from(kind).into()),
                };
                message.error_details = Some(ErrorDetails::TypeArgumentError(type_argument_error));
                ExecutionErrorKind::TypeArgumentError
            }
            E::UnusedValueWithoutDrop { result, subresult } => {
                message.error_details = Some(ErrorDetails::IndexError(IndexError {
                    index: Some(result.into()),
                    subresult: Some(subresult.into()),
                }));
                ExecutionErrorKind::UnusedValueWithoutDrop
            }
            E::InvalidPublicFunctionReturnType { index } => {
                message.error_details = Some(ErrorDetails::IndexError(IndexError {
                    index: Some(index.into()),
                    subresult: None,
                }));
                ExecutionErrorKind::InvalidPublicFunctionReturnType
            }
            E::InvalidTransferObject => ExecutionErrorKind::InvalidTransferObject,
            E::EffectsTooLarge {
                current_size,
                max_size,
            } => {
                message.error_details = Some(ErrorDetails::SizeError(SizeError {
                    size: Some(current_size),
                    max_size: Some(max_size),
                }));
                ExecutionErrorKind::EffectsTooLarge
            }
            E::PublishUpgradeMissingDependency => {
                ExecutionErrorKind::PublishUpgradeMissingDependency
            }
            E::PublishUpgradeDependencyDowngrade => {
                ExecutionErrorKind::PublishUpgradeDependencyDowngrade
            }
            E::PackageUpgradeError { kind } => {
                message.error_details = Some(ErrorDetails::PackageUpgradeError(kind.into()));
                ExecutionErrorKind::PackageUpgradeError
            }
            E::WrittenObjectsTooLarge {
                object_size,
                max_object_size,
            } => {
                message.error_details = Some(ErrorDetails::SizeError(SizeError {
                    size: Some(object_size),
                    max_size: Some(max_object_size),
                }));

                ExecutionErrorKind::WrittenObjectsTooLarge
            }
            E::CertificateDenied => ExecutionErrorKind::CertificateDenied,
            E::SuiMoveVerificationTimedout => ExecutionErrorKind::SuiMoveVerificationTimedout,
            E::ConsensusObjectOperationNotAllowed => {
                ExecutionErrorKind::ConsensusObjectOperationNotAllowed
            }
            E::InputObjectDeleted => ExecutionErrorKind::InputObjectDeleted,
            E::ExecutionCanceledDueToConsensusObjectCongestion { congested_objects } => {
                message.error_details = Some(ErrorDetails::CongestedObjects(CongestedObjects {
                    objects: congested_objects.iter().map(ToString::to_string).collect(),
                }));

                ExecutionErrorKind::ExecutionCanceledDueToConsensusObjectCongestion
            }
            E::AddressDeniedForCoin { address, coin_type } => {
                message.error_details = Some(ErrorDetails::CoinDenyListError(CoinDenyListError {
                    address: Some(address.to_string()),
                    coin_type: Some(coin_type),
                }));
                ExecutionErrorKind::AddressDeniedForCoin
            }
            E::CoinTypeGlobalPause { coin_type } => {
                message.error_details = Some(ErrorDetails::CoinDenyListError(CoinDenyListError {
                    address: None,
                    coin_type: Some(coin_type),
                }));
                ExecutionErrorKind::CoinTypeGlobalPause
            }
            E::ExecutionCanceledDueToRandomnessUnavailable => {
                ExecutionErrorKind::ExecutionCanceledDueToRandomnessUnavailable
            }
            E::MoveVectorElemTooBig {
                value_size,
                max_scaled_size,
            } => {
                message.error_details = Some(ErrorDetails::SizeError(SizeError {
                    size: Some(value_size),
                    max_size: Some(max_scaled_size),
                }));

                ExecutionErrorKind::MoveVectorElemTooBig
            }
            E::MoveRawValueTooBig {
                value_size,
                max_scaled_size,
            } => {
                message.error_details = Some(ErrorDetails::SizeError(SizeError {
                    size: Some(value_size),
                    max_size: Some(max_scaled_size),
                }));
                ExecutionErrorKind::MoveRawValueTooBig
            }
            E::InvalidLinkage => ExecutionErrorKind::InvalidLinkage,
            _ => ExecutionErrorKind::Unknown,
        };

        message.set_kind(kind);
        message
    }
}

impl TryFrom<&ExecutionError> for sui_sdk_types::ExecutionError {
    type Error = TryFromProtoError;

    fn try_from(value: &ExecutionError) -> Result<Self, Self::Error> {
        use execution_error::ErrorDetails;
        use execution_error::ExecutionErrorKind as K;

        match value.kind() {
            K::Unknown => return Err(TryFromProtoError::invalid(ExecutionError::KIND_FIELD, "unknown ExecutionErrorKind")),
            K::InsufficientGas => Self::InsufficientGas,
            K::InvalidGasObject => Self::InvalidGasObject,
            K::InvariantViolation => Self::InvariantViolation,
            K::FeatureNotYetSupported => Self::FeatureNotYetSupported,
            K::ObjectTooBig => {
                let Some(ErrorDetails::SizeError(SizeError { size, max_size })) =
                    &value.error_details
                else {
                    return Err(TryFromProtoError::missing(ExecutionError::SIZE_ERROR_FIELD));
                };
                Self::ObjectTooBig {
                    object_size: size.ok_or_else(|| TryFromProtoError::missing(SizeError::SIZE_FIELD))?,
                    max_object_size: max_size
                        .ok_or_else(|| TryFromProtoError::missing(SizeError::MAX_SIZE_FIELD))?,
                }
            }
            K::PackageTooBig => {
                let Some(ErrorDetails::SizeError(SizeError { size, max_size })) =
                    &value.error_details
                else {
                    return Err(TryFromProtoError::missing(ExecutionError::SIZE_ERROR_FIELD));
                };
                Self::PackageTooBig {
                    object_size: size.ok_or_else(|| TryFromProtoError::missing(SizeError::SIZE_FIELD))?,
                    max_object_size: max_size
                        .ok_or_else(|| TryFromProtoError::missing(SizeError::MAX_SIZE_FIELD))?,
                }
            }
            K::CircularObjectOwnership => {
                let Some(ErrorDetails::ObjectId(object_id)) = &value.error_details else {
                    return Err(TryFromProtoError::missing(ExecutionError::OBJECT_ID_FIELD));
                };
                Self::CircularObjectOwnership {
                    object: object_id.parse().map_err(|e| TryFromProtoError::invalid(ExecutionError::OBJECT_ID_FIELD, e))?,
                }
            }
            K::InsufficientCoinBalance => Self::InsufficientCoinBalance,
            K::CoinBalanceOverflow => Self::CoinBalanceOverflow,
            K::PublishErrorNonZeroAddress => Self::PublishErrorNonZeroAddress,
            K::SuiMoveVerificationError => Self::SuiMoveVerificationError,
            K::MovePrimitiveRuntimeError => {
                let location = if let Some(ErrorDetails::Abort(abort)) = &value.error_details {
                    abort.location.as_ref().map(TryInto::try_into).transpose()?
                } else {
                    None
                };
                Self::MovePrimitiveRuntimeError { location }
            }
            K::MoveAbort => {
                let Some(ErrorDetails::Abort(abort)) = &value.error_details else {
                    return Err(TryFromProtoError::missing("abort"));
                };
                Self::MoveAbort {
                    location: abort
                        .location
                        .as_ref()
                        .ok_or_else(|| TryFromProtoError::missing("location"))?
                        .try_into()?,
                    code: abort
                        .abort_code
                        .ok_or_else(|| TryFromProtoError::missing("abort_code"))?,
                }
            }
            K::VmVerificationOrDeserializationError => Self::VmVerificationOrDeserializationError,
            K::VmInvariantViolation => Self::VmInvariantViolation,
            K::FunctionNotFound => Self::FunctionNotFound,
            K::ArityMismatch => Self::ArityMismatch,
            K::TypeArityMismatch => Self::TypeArityMismatch,
            K::NonEntryFunctionInvoked => Self::NonEntryFunctionInvoked,
            K::CommandArgumentError => {
                let Some(ErrorDetails::CommandArgumentError(command_argument_error)) =
                    &value.error_details
                else {
                    return Err(TryFromProtoError::missing("command_argument_error"));
                };
                Self::CommandArgumentError {
                    argument: command_argument_error
                        .argument
                        .ok_or_else(|| TryFromProtoError::missing("argument"))?
                        .try_into()
                        .map_err(|e| TryFromProtoError::invalid(CommandArgumentError::ARGUMENT_FIELD, e))?,
                    kind: command_argument_error.try_into()?,
                }
            }
            K::TypeArgumentError => {
                let Some(ErrorDetails::TypeArgumentError(type_argument_error)) =
                    &value.error_details
                else {
                    return Err(TryFromProtoError::missing("type_argument_error"));
                };
                Self::TypeArgumentError {
                    type_argument: type_argument_error
                        .type_argument
                        .ok_or_else(|| TryFromProtoError::missing("type_argument"))?
                        .try_into()
                        .map_err(|e| TryFromProtoError::invalid(TypeArgumentError::TYPE_ARGUMENT_FIELD, e))?,
                    kind: type_argument_error.kind().try_into()?,
                }
            }
            K::UnusedValueWithoutDrop => {
                let Some(ErrorDetails::IndexError(IndexError { index, subresult })) =
                    &value.error_details
                else {
                    return Err(TryFromProtoError::missing("index_error"));
                };
                Self::UnusedValueWithoutDrop {
                    result: index
                        .ok_or_else(|| TryFromProtoError::missing("result"))?
                        .try_into()
                        .map_err(|e| TryFromProtoError::invalid(IndexError::INDEX_FIELD, e))?,
                    subresult: subresult
                        .ok_or_else(|| TryFromProtoError::missing("subresult"))?
                        .try_into()
                        .map_err(|e| TryFromProtoError::invalid(IndexError::SUBRESULT_FIELD, e))?,
                }
            }
            K::InvalidPublicFunctionReturnType => {
                let Some(ErrorDetails::IndexError(IndexError { index, .. })) =
                    &value.error_details
                else {
                    return Err(TryFromProtoError::missing("index_error"));
                };
                Self::InvalidPublicFunctionReturnType {
                    index: index
                        .ok_or_else(|| TryFromProtoError::missing("index"))?
                        .try_into()
                        .map_err(|e| TryFromProtoError::invalid(IndexError::INDEX_FIELD, e))?,
                }
            }
            K::InvalidTransferObject => Self::InvalidTransferObject,
            K::EffectsTooLarge => {
                let Some(ErrorDetails::SizeError(SizeError { size, max_size })) =
                    &value.error_details
                else {
                    return Err(TryFromProtoError::missing(ExecutionError::SIZE_ERROR_FIELD));
                };
                Self::EffectsTooLarge {
                    current_size: size.ok_or_else(|| TryFromProtoError::missing(SizeError::SIZE_FIELD))?,
                    max_size: max_size.ok_or_else(|| TryFromProtoError::missing(SizeError::MAX_SIZE_FIELD))?,
                }
            }
            K::PublishUpgradeMissingDependency => Self::PublishUpgradeMissingDependency,
            K::PublishUpgradeDependencyDowngrade => Self::PublishUpgradeDependencyDowngrade,
            K::PackageUpgradeError => {
                let Some(ErrorDetails::PackageUpgradeError(package_upgrade_error)) =
                    &value.error_details
                else {
                    return Err(TryFromProtoError::missing("package_upgrade_error"));
                };
                Self::PackageUpgradeError {
                    kind: package_upgrade_error.try_into()?,
                }
            }
            K::WrittenObjectsTooLarge => {
                let Some(ErrorDetails::SizeError(SizeError { size, max_size })) =
                    &value.error_details
                else {
                    return Err(TryFromProtoError::missing(ExecutionError::SIZE_ERROR_FIELD));
                };

                Self::WrittenObjectsTooLarge {
                    object_size: size.ok_or_else(|| TryFromProtoError::missing(SizeError::SIZE_FIELD))?,
                    max_object_size: max_size
                        .ok_or_else(|| TryFromProtoError::missing(SizeError::MAX_SIZE_FIELD))?,
                }
            }
            K::CertificateDenied => Self::CertificateDenied,
            K::SuiMoveVerificationTimedout => Self::SuiMoveVerificationTimedout,
            K::ConsensusObjectOperationNotAllowed => Self::ConsensusObjectOperationNotAllowed,
            K::InputObjectDeleted => Self::InputObjectDeleted,
            K::ExecutionCanceledDueToConsensusObjectCongestion => {
                let Some(ErrorDetails::CongestedObjects(CongestedObjects { objects })) =
                    &value.error_details
                else {
                    return Err(TryFromProtoError::missing("congested_objects"));
                };

                Self::ExecutionCanceledDueToConsensusObjectCongestion {
                    congested_objects: objects
                        .iter()
                        .map(|s| s.parse())
                        .collect::<Result<_, _>>()
                        .map_err(|e| TryFromProtoError::invalid(CongestedObjects::OBJECTS_FIELD, e))?,
                }
            }
            K::AddressDeniedForCoin => {
                let Some(ErrorDetails::CoinDenyListError(CoinDenyListError {
                    address,
                    coin_type,
                })) = &value.error_details
                else {
                    return Err(TryFromProtoError::missing("coin_deny_list_error"));
                };
                Self::AddressDeniedForCoin {
                    address: address
                        .as_ref()
                        .ok_or_else(|| TryFromProtoError::missing("address"))?
                        .parse()
                        .map_err(|e| TryFromProtoError::invalid(CoinDenyListError::ADDRESS_FIELD, e))?,
                    coin_type: coin_type
                        .as_ref()
                        .ok_or_else(|| TryFromProtoError::missing("coin_type"))?
                        .to_owned(),
                }
            }
            K::CoinTypeGlobalPause => {
                let Some(ErrorDetails::CoinDenyListError(CoinDenyListError {
                    coin_type,
                    ..
                })) = &value.error_details
                else {
                    return Err(TryFromProtoError::missing("coin_deny_list_error"));
                };
                Self::CoinTypeGlobalPause {
                    coin_type: coin_type
                        .as_ref()
                        .ok_or_else(|| TryFromProtoError::missing("coin_type"))?
                        .to_owned(),
                }
            }
            K::ExecutionCanceledDueToRandomnessUnavailable => {
                Self::ExecutionCanceledDueToRandomnessUnavailable
            }
            K::MoveVectorElemTooBig => {
                let Some(ErrorDetails::SizeError(SizeError { size, max_size })) =
                    &value.error_details
                else {
                    return Err(TryFromProtoError::missing(ExecutionError::SIZE_ERROR_FIELD));
                };

                Self::MoveVectorElemTooBig {
                    value_size: size.ok_or_else(|| TryFromProtoError::missing(SizeError::SIZE_FIELD))?,
                    max_scaled_size: max_size
                        .ok_or_else(|| TryFromProtoError::missing(SizeError::MAX_SIZE_FIELD))?,
                }
            }
            K::MoveRawValueTooBig => {
                let Some(ErrorDetails::SizeError(SizeError { size, max_size })) =
                    &value.error_details
                else {
                    return Err(TryFromProtoError::missing(ExecutionError::SIZE_ERROR_FIELD));
                };

                Self::MoveRawValueTooBig {
                    value_size: size.ok_or_else(|| TryFromProtoError::missing(SizeError::SIZE_FIELD))?,
                    max_scaled_size: max_size
                        .ok_or_else(|| TryFromProtoError::missing(SizeError::MAX_SIZE_FIELD))?,
                }
            }
            K::InvalidLinkage => Self::InvalidLinkage,
        }
        .pipe(Ok)
    }
}

//
// CommandArgumentError
//

impl From<sui_sdk_types::CommandArgumentError> for CommandArgumentError {
    fn from(value: sui_sdk_types::CommandArgumentError) -> Self {
        use command_argument_error::CommandArgumentErrorKind;
        use sui_sdk_types::CommandArgumentError as E;

        let mut message = Self::default();

        let kind = match value {
            E::TypeMismatch => CommandArgumentErrorKind::TypeMismatch,
            E::InvalidBcsBytes => CommandArgumentErrorKind::InvalidBcsBytes,
            E::InvalidUsageOfPureArgument => CommandArgumentErrorKind::InvalidUsageOfPureArgument,
            E::InvalidArgumentToPrivateEntryFunction => {
                CommandArgumentErrorKind::InvalidArgumentToPrivateEntryFunction
            }
            E::IndexOutOfBounds { index } => {
                message.index_error = Some(IndexError {
                    index: Some(index.into()),
                    subresult: None,
                });
                CommandArgumentErrorKind::IndexOutOfBounds
            }
            E::SecondaryIndexOutOfBounds { result, subresult } => {
                message.index_error = Some(IndexError {
                    index: Some(result.into()),
                    subresult: Some(subresult.into()),
                });
                CommandArgumentErrorKind::SecondaryIndexOutOfBounds
            }
            E::InvalidResultArity { result } => {
                message.index_error = Some(IndexError {
                    index: Some(result.into()),
                    subresult: None,
                });
                CommandArgumentErrorKind::InvalidResultArity
            }
            E::InvalidGasCoinUsage => CommandArgumentErrorKind::InvalidGasCoinUsage,
            E::InvalidValueUsage => CommandArgumentErrorKind::InvalidValueUsage,
            E::InvalidObjectByValue => CommandArgumentErrorKind::InvalidObjectByValue,
            E::InvalidObjectByMutRef => CommandArgumentErrorKind::InvalidObjectByMutRef,
            E::ConsensusObjectOperationNotAllowed => {
                CommandArgumentErrorKind::ConsensusObjectOperationNotAllowed
            }
            E::InvalidArgumentArity => CommandArgumentErrorKind::InvalidArgumentArity,
            _ => CommandArgumentErrorKind::Unknown,
        };

        message.set_kind(kind);
        message
    }
}

impl TryFrom<&CommandArgumentError> for sui_sdk_types::CommandArgumentError {
    type Error = TryFromProtoError;

    fn try_from(value: &CommandArgumentError) -> Result<Self, Self::Error> {
        use command_argument_error::CommandArgumentErrorKind as K;

        match value.kind() {
            K::Unknown => {
                return Err(TryFromProtoError::invalid(
                    CommandArgumentError::KIND_FIELD,
                    "unknown CommandArgumentErrorKind",
                ));
            }
            K::TypeMismatch => Self::TypeMismatch,
            K::InvalidBcsBytes => Self::InvalidBcsBytes,
            K::InvalidUsageOfPureArgument => Self::InvalidUsageOfPureArgument,
            K::InvalidArgumentToPrivateEntryFunction => Self::InvalidArgumentToPrivateEntryFunction,
            K::IndexOutOfBounds => Self::IndexOutOfBounds {
                index: value
                    .index_error
                    .ok_or_else(|| TryFromProtoError::missing("index_error"))?
                    .index
                    .ok_or_else(|| TryFromProtoError::missing("index"))?
                    .try_into()
                    .map_err(|e| TryFromProtoError::invalid(IndexError::INDEX_FIELD, e))?,
            },
            K::SecondaryIndexOutOfBounds => {
                let index_error = value
                    .index_error
                    .ok_or_else(|| TryFromProtoError::missing("index_error"))?;
                Self::SecondaryIndexOutOfBounds {
                    result: index_error
                        .index
                        .ok_or_else(|| TryFromProtoError::missing("index"))?
                        .try_into()
                        .map_err(|e| TryFromProtoError::invalid(IndexError::INDEX_FIELD, e))?,
                    subresult: index_error
                        .subresult
                        .ok_or_else(|| TryFromProtoError::missing("subresult"))?
                        .try_into()
                        .map_err(|e| TryFromProtoError::invalid(IndexError::SUBRESULT_FIELD, e))?,
                }
            }
            K::InvalidResultArity => Self::InvalidResultArity {
                result: value
                    .index_error
                    .ok_or_else(|| TryFromProtoError::missing("index_error"))?
                    .index
                    .ok_or_else(|| TryFromProtoError::missing("index"))?
                    .try_into()
                    .map_err(|e| TryFromProtoError::invalid(IndexError::INDEX_FIELD, e))?,
            },
            K::InvalidGasCoinUsage => Self::InvalidGasCoinUsage,
            K::InvalidValueUsage => Self::InvalidValueUsage,
            K::InvalidObjectByValue => Self::InvalidObjectByValue,
            K::InvalidObjectByMutRef => Self::InvalidObjectByMutRef,
            K::ConsensusObjectOperationNotAllowed => Self::ConsensusObjectOperationNotAllowed,
            K::InvalidArgumentArity => Self::InvalidArgumentArity,
            K::InvalidTransferObject => Self::InvalidTransferObject,
            K::InvalidMakeMoveVecNonObjectArgument => Self::InvalidMakeMoveVecNonObjectArgument,
            K::ArgumentWithoutValue => Self::ArgumentWithoutValue,
            K::CannotMoveBorrowedValue => Self::CannotMoveBorrowedValue,
            K::CannotWriteToExtendedReference => Self::CannotWriteToExtendedReference,
            K::InvalidReferenceArgument => Self::InvalidReferenceArgument,
        }
        .pipe(Ok)
    }
}

//
// TypeArgumentError
//

impl From<sui_sdk_types::TypeArgumentError> for type_argument_error::TypeArgumentErrorKind {
    fn from(value: sui_sdk_types::TypeArgumentError) -> Self {
        use sui_sdk_types::TypeArgumentError::*;

        match value {
            TypeNotFound => Self::TypeNotFound,
            ConstraintNotSatisfied => Self::ConstraintNotSatisfied,
            _ => Self::Unknown,
        }
    }
}

impl TryFrom<type_argument_error::TypeArgumentErrorKind> for sui_sdk_types::TypeArgumentError {
    type Error = TryFromProtoError;

    fn try_from(value: type_argument_error::TypeArgumentErrorKind) -> Result<Self, Self::Error> {
        use type_argument_error::TypeArgumentErrorKind as K;

        match value {
            K::Unknown => {
                return Err(TryFromProtoError::invalid(
                    "kind",
                    "unknown TypeArgumentErrorKind",
                ));
            }
            K::TypeNotFound => Self::TypeNotFound,
            K::ConstraintNotSatisfied => Self::ConstraintNotSatisfied,
        }
        .pipe(Ok)
    }
}

//
// PackageUpgradeError
//

impl From<sui_sdk_types::PackageUpgradeError> for PackageUpgradeError {
    fn from(value: sui_sdk_types::PackageUpgradeError) -> Self {
        use package_upgrade_error::PackageUpgradeErrorKind;
        use sui_sdk_types::PackageUpgradeError as E;

        let mut message = Self::default();

        let kind = match value {
            E::UnableToFetchPackage { package_id } => {
                message.package_id = Some(package_id.to_string());
                PackageUpgradeErrorKind::UnableToFetchPackage
            }
            E::NotAPackage { object_id } => {
                message.package_id = Some(object_id.to_string());
                PackageUpgradeErrorKind::NotAPackage
            }
            E::IncompatibleUpgrade => PackageUpgradeErrorKind::IncompatibleUpgrade,
            E::DigestDoesNotMatch { digest } => {
                message.digest = Some(digest.to_string());
                PackageUpgradeErrorKind::DigestDoesNotMatch
            }
            E::UnknownUpgradePolicy { policy } => {
                message.policy = Some(policy.into());
                PackageUpgradeErrorKind::UnknownUpgradePolicy
            }
            E::PackageIdDoesNotMatch {
                package_id,
                ticket_id,
            } => {
                message.package_id = Some(package_id.to_string());
                message.ticket_id = Some(ticket_id.to_string());
                PackageUpgradeErrorKind::PackageIdDoesNotMatch
            }
            _ => PackageUpgradeErrorKind::Unknown,
        };

        message.set_kind(kind);
        message
    }
}

impl TryFrom<&PackageUpgradeError> for sui_sdk_types::PackageUpgradeError {
    type Error = TryFromProtoError;

    fn try_from(value: &PackageUpgradeError) -> Result<Self, Self::Error> {
        use package_upgrade_error::PackageUpgradeErrorKind as K;

        match value.kind() {
            K::Unknown => {
                return Err(TryFromProtoError::invalid(
                    PackageUpgradeError::KIND_FIELD,
                    "unknown PackageUpgradeErrorKind",
                ));
            }
            K::UnableToFetchPackage => Self::UnableToFetchPackage {
                package_id: value
                    .package_id
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("package_id"))?
                    .parse()
                    .map_err(|e| {
                        TryFromProtoError::invalid(PackageUpgradeError::PACKAGE_ID_FIELD, e)
                    })?,
            },
            K::NotAPackage => Self::NotAPackage {
                object_id: value
                    .package_id
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("package_id"))?
                    .parse()
                    .map_err(|e| {
                        TryFromProtoError::invalid(PackageUpgradeError::PACKAGE_ID_FIELD, e)
                    })?,
            },
            K::IncompatibleUpgrade => Self::IncompatibleUpgrade,
            K::DigestDoesNotMatch => Self::DigestDoesNotMatch {
                digest: value
                    .digest
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("digest"))?
                    .parse()
                    .map_err(|e| {
                        TryFromProtoError::invalid(PackageUpgradeError::DIGEST_FIELD, e)
                    })?,
            },
            K::UnknownUpgradePolicy => Self::UnknownUpgradePolicy {
                policy: value
                    .policy
                    .ok_or_else(|| TryFromProtoError::missing("policy"))?
                    .try_into()
                    .map_err(|e| {
                        TryFromProtoError::invalid(PackageUpgradeError::POLICY_FIELD, e)
                    })?,
            },
            K::PackageIdDoesNotMatch => Self::PackageIdDoesNotMatch {
                package_id: value
                    .package_id
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("package_id"))?
                    .parse()
                    .map_err(|e| {
                        TryFromProtoError::invalid(PackageUpgradeError::PACKAGE_ID_FIELD, e)
                    })?,
                ticket_id: value
                    .ticket_id
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("ticket_id"))?
                    .parse()
                    .map_err(|e| {
                        TryFromProtoError::invalid(PackageUpgradeError::TICKET_ID_FIELD, e)
                    })?,
            },
        }
        .pipe(Ok)
    }
}

//
// MoveLocation
//

impl From<sui_sdk_types::MoveLocation> for MoveLocation {
    fn from(value: sui_sdk_types::MoveLocation) -> Self {
        Self {
            package: Some(value.package.to_string()),
            module: Some(value.module.to_string()),
            function: Some(value.function.into()),
            instruction: Some(value.instruction.into()),
            function_name: value.function_name.map(|name| name.to_string()),
        }
    }
}

impl TryFrom<&MoveLocation> for sui_sdk_types::MoveLocation {
    type Error = TryFromProtoError;

    fn try_from(value: &MoveLocation) -> Result<Self, Self::Error> {
        let package = value
            .package
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("package"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(MoveLocation::PACKAGE_FIELD, e))?;
        let module = value
            .module
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("module"))?
            .parse()
            .map_err(|e| TryFromProtoError::invalid(MoveLocation::MODULE_FIELD, e))?;
        let function = value
            .function
            .ok_or_else(|| TryFromProtoError::missing("function"))?
            .try_into()
            .map_err(|e| TryFromProtoError::invalid(MoveLocation::FUNCTION_FIELD, e))?;
        let instruction = value
            .instruction
            .ok_or_else(|| TryFromProtoError::missing("instruction"))?
            .try_into()
            .map_err(|e| TryFromProtoError::invalid(MoveLocation::INSTRUCTION_FIELD, e))?;
        let function_name = value
            .function_name
            .as_ref()
            .map(|name| {
                name.parse()
                    .map_err(|e| TryFromProtoError::invalid(MoveLocation::FUNCTION_NAME_FIELD, e))
            })
            .transpose()?;

        Ok(Self {
            package,
            module,
            function,
            instruction,
            function_name,
        })
    }
}

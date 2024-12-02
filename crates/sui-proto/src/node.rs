#[path = "generated/sui.node.v2.rs"]
mod generated;
pub use generated::*;
use tap::Pipe;

use crate::TryFromProtoError;

//
// I128
//

impl From<i128> for I128 {
    fn from(value: i128) -> Self {
        Self {
            bytes: value.to_le_bytes().to_vec().into(),
        }
    }
}

impl TryFrom<&I128> for i128 {
    type Error = std::array::TryFromSliceError;

    fn try_from(value: &I128) -> Result<Self, Self::Error> {
        Ok(i128::from_le_bytes(value.bytes.as_ref().try_into()?))
    }
}

//
// BalanceChange
//

impl From<sui_sdk_types::types::BalanceChange> for BalanceChange {
    fn from(value: sui_sdk_types::types::BalanceChange) -> Self {
        Self {
            address: Some(value.address.into()),
            coin_type: Some(value.coin_type.into()),
            amount: Some(value.amount.into()),
        }
    }
}

impl TryFrom<&BalanceChange> for sui_sdk_types::types::BalanceChange {
    type Error = TryFromProtoError;

    fn try_from(value: &BalanceChange) -> Result<Self, Self::Error> {
        let address = value
            .address
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("address"))?
            .pipe(TryInto::try_into)?;
        let coin_type = value
            .coin_type
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("coin_type"))?
            .pipe(TryInto::try_into)?;
        let amount = value
            .amount
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("amount"))?
            .pipe(TryInto::try_into)?;
        Ok(Self {
            address,
            coin_type,
            amount,
        })
    }
}

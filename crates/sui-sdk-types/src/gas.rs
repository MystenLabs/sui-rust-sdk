/// Summary of gas charges.
///
/// Storage is charged independently of computation.
/// There are 3 parts to the storage charges:
/// `storage_cost`: it is the charge of storage at the time the transaction is executed.
///                 The cost of storage is the number of bytes of the objects being mutated
///                 multiplied by a variable storage cost per byte
/// `storage_rebate`: this is the amount a user gets back when manipulating an object.
///                   The `storage_rebate` is the `storage_cost` for an object minus fees.
/// `non_refundable_storage_fee`: not all the value of the object storage cost is
///                               given back to user and there is a small fraction that
///                               is kept by the system. This value tracks that charge.
///
/// When looking at a gas cost summary the amount charged to the user is
/// `computation_cost + storage_cost - storage_rebate`
/// and that is the amount that is deducted from the gas coins.
/// `non_refundable_storage_fee` is collected from the objects being mutated/deleted
/// and it is tracked by the system in storage funds.
///
/// Objects deleted, including the older versions of objects mutated, have the storage field
/// on the objects added up to a pool of "potential rebate". This rebate then is reduced
/// by the "nonrefundable rate" such that:
/// `potential_rebate(storage cost of deleted/mutated objects) =
/// storage_rebate + non_refundable_storage_fee`
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// gas-cost-summary = u64 ; computation-cost
///                    u64 ; storage-cost
///                    u64 ; storage-rebate
///                    u64 ; non-refundable-storage-fee
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct GasCostSummary {
    /// Cost of computation/execution
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub computation_cost: u64,

    /// Storage cost, it's the sum of all storage cost for all objects created or mutated.
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub storage_cost: u64,

    /// The amount of storage cost refunded to the user for all objects deleted or mutated in the
    /// transaction.
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub storage_rebate: u64,

    /// The fee for the rebate. The portion of the storage rebate kept by the system.
    #[cfg_attr(feature = "serde", serde(with = "crate::_serde::ReadableDisplay"))]
    pub non_refundable_storage_fee: u64,
}

impl GasCostSummary {
    /// Create a new gas cost summary.
    ///
    /// # Arguments
    /// * `computation_cost` - Cost of computation cost/execution.
    /// * `storage_cost` - Storage cost, it's the sum of all storage cost for all objects created or mutated.
    /// * `storage_rebate` - The amount of storage cost refunded to the user for all objects deleted or mutated in the transaction.
    /// * `non_refundable_storage_fee` - The fee for the rebate. The portion of the storage rebate kept by the system.
    pub fn new(
        computation_cost: u64,
        storage_cost: u64,
        storage_rebate: u64,
        non_refundable_storage_fee: u64,
    ) -> GasCostSummary {
        GasCostSummary {
            computation_cost,
            storage_cost,
            storage_rebate,
            non_refundable_storage_fee,
        }
    }

    /// The total gas used, which is the sum of computation and storage costs.
    pub fn gas_used(&self) -> u64 {
        self.computation_cost + self.storage_cost
    }

    /// The net gas usage, which is the total gas used minus the storage rebate.
    /// A positive number means used gas; negative number means refund.
    pub fn net_gas_usage(&self) -> i64 {
        self.gas_used() as i64 - self.storage_rebate as i64
    }
}

impl std::fmt::Display for GasCostSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "computation_cost: {}, ", self.computation_cost)?;
        write!(f, "storage_cost: {}, ", self.storage_cost)?;
        write!(f, "storage_rebate: {}, ", self.storage_rebate)?;
        write!(
            f,
            "non_refundable_storage_fee: {}",
            self.non_refundable_storage_fee
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[test]
    #[cfg(feature = "serde")]
    fn formats() {
        let actual = GasCostSummary {
            computation_cost: 42,
            storage_cost: u64::MAX,
            storage_rebate: 0,
            non_refundable_storage_fee: 9,
        };

        println!("{}", serde_json::to_string(&actual).unwrap());
        println!("{:?}", bcs::to_bytes(&actual).unwrap());
    }
}

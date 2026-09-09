//! GraphQL scalar types for Sui.

use std::borrow::Cow;

use serde::Deserialize;

/// `SuiAddress` scalar in Sui GraphQL schema. 32-byte hex-encoded address with `0x` prefix.
pub use sui_sdk_types::Address;

/// Useful for digest fields (Base58 string). Not a scalar in Sui GraphQL schema.
pub use sui_sdk_types::Digest;

/// `BigInt` scalar in Sui GraphQL schema.
///
/// Represented as a string because JSON numbers cannot reliably represent large integers.
/// The underlying type defaults to `u64` and can be any signed or unsigned primitive integer
/// type, or [`sui_sdk_types::U256`].
pub struct BigInt<T: sealed::Numeric = u64>(pub T);

impl<'de, T: sealed::Numeric> Deserialize<'de> for BigInt<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <Cow<'_, str>>::deserialize(deserializer)?;
        let value = s.parse().map_err(serde::de::Error::custom)?;
        Ok(BigInt(value))
    }
}

/// Restrict the types that can be used inside `BigInt`.
mod sealed {
    use std::fmt::Display;
    use std::str::FromStr;

    pub trait Numeric: FromStr<Err: Display> {}

    impl Numeric for i8 {}
    impl Numeric for i16 {}
    impl Numeric for i32 {}
    impl Numeric for i64 {}
    impl Numeric for i128 {}
    impl Numeric for isize {}
    impl Numeric for u8 {}
    impl Numeric for u16 {}
    impl Numeric for u32 {}
    impl Numeric for u64 {}
    impl Numeric for u128 {}
    impl Numeric for usize {}
    impl Numeric for sui_sdk_types::U256 {}
}

/// `DateTime` scalar in Sui GraphQL schema.
/// ISO-8601 Date and Time in UTC.
pub type DateTime = chrono::DateTime<chrono::Utc>;

#[cfg(test)]
mod tests {
    use super::BigInt;
    use sui_sdk_types::U256;

    #[test]
    fn defaults_to_u64() {
        let value: BigInt = serde_json::from_str(r#""18446744073709551615""#).unwrap();
        assert_eq!(value.0, u64::MAX);
    }

    #[test]
    fn supports_all_primitive_integer_types() {
        macro_rules! check_types {
            ($($type:ty),+ $(,)?) => {
                $(
                    let value: BigInt<$type> = serde_json::from_str(r#""1""#).unwrap();
                    assert_eq!(value.0, 1);
                )+
            };
        }

        check_types!(
            i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
        );
    }

    #[test]
    fn supports_larger_integer_types() {
        let value: BigInt<u128> =
            serde_json::from_str(r#""340282366920938463463374607431768211455""#).unwrap();
        assert_eq!(value.0, u128::MAX);

        let value: BigInt<U256> = serde_json::from_str(
            r#""115792089237316195423570985008687907853269984665640564039457584007913129639935""#,
        )
        .unwrap();
        assert_eq!(value.0, U256::from_digits([u8::MAX; 32]));
    }
}

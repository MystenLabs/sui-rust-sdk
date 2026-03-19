#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct U256(bnum::BUintD8<32>);

impl U256 {
    pub const ZERO: Self = Self(bnum::BUintD8::<32>::ZERO);
    pub const ONE: Self = Self(bnum::BUintD8::<32>::ONE);

    pub const fn from_digits(digits: [u8; 32]) -> Self {
        Self(bnum::BUintD8::<32>::from_digits(digits))
    }

    pub const fn digits(&self) -> &[u8; 32] {
        self.0.digits()
    }

    pub const fn from_le(value: Self) -> Self {
        Self(bnum::BUintD8::<32>::from_le(value.0))
    }

    pub const fn from_be(value: Self) -> Self {
        Self(bnum::BUintD8::<32>::from_be(value.0))
    }

    pub const fn to_le(self) -> Self {
        Self(self.0.to_le())
    }

    pub const fn to_be(self) -> Self {
        Self(self.0.to_be())
    }

    pub const fn from_str_radix(s: &str, radix: u32) -> Result<Self, bnum::errors::ParseIntError> {
        match bnum::BUintD8::<32>::from_str_radix(s, radix) {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(e),
        }
    }

    pub fn to_str_radix(&self, radix: u32) -> String {
        self.0.to_str_radix(radix)
    }

    pub fn from_radix_be(buf: &[u8], radix: u32) -> Option<Self> {
        bnum::BUintD8::<32>::from_radix_be(buf, radix).map(Self)
    }

    pub fn from_le_slice(slice: &[u8]) -> Option<Self> {
        bnum::BUintD8::<32>::from_le_slice(slice).map(Self)
    }

    pub fn to_radix_be(&self, radix: u32) -> Vec<u8> {
        self.0.to_radix_be(radix)
    }
}

impl From<u8> for U256 {
    fn from(value: u8) -> Self {
        Self(bnum::BUintD8::<32>::from(value))
    }
}

impl From<u64> for U256 {
    fn from(value: u64) -> Self {
        Self(bnum::BUintD8::<32>::from(value))
    }
}

impl std::str::FromStr for U256 {
    type Err = bnum::errors::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        bnum::BUintD8::<32>::from_str_radix(s, 10).map(Self)
    }
}

impl std::fmt::Display for U256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_str_radix(10))
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl serde::Serialize for U256 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(self.digits(), serializer)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl<'de> serde::Deserialize<'de> for U256 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let digits: [u8; 32] = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_digits(digits))
    }
}

// This is a constant time assert to ensure that the backing storage for U256 is 32 bytes long
#[allow(unused)]
const ASSERT_32_BYTES: () = {
    let u256 = U256::ZERO;

    let _digits: &[u8; 32] = u256.digits();
};

// This is a constant time assert to ensure endianness of the underlying storage is as expected
#[allow(unused)]
const ASSERT_ENDIANNESS: () = {
    const fn const_bytes_equal(lhs: &[u8], rhs: &[u8]) -> bool {
        if lhs.len() != rhs.len() {
            return false;
        }
        let mut i = 0;
        while i < lhs.len() {
            if lhs[i] != rhs[i] {
                return false;
            }
            i += 1;
        }
        true
    }

    let one_platform = U256::ONE;
    let one_le = {
        let mut buf = [0; 32];
        buf[0] = 1;
        buf
    };

    let one_be = {
        let mut buf = [0; 32];
        buf[31] = 1;
        buf
    };

    // To little endian
    let le = one_platform.to_le();
    assert!(const_bytes_equal(&one_le, le.digits().as_slice()));

    // To big endian
    let be = one_platform.to_be();
    assert!(const_bytes_equal(&one_be, be.digits().as_slice()));

    // From little endian
    assert!(const_bytes_equal(
        one_platform.digits().as_slice(),
        U256::from_le(U256::from_digits(one_le)).digits().as_slice()
    ));

    // From big endian
    assert!(const_bytes_equal(
        one_platform.digits().as_slice(),
        U256::from_be(U256::from_digits(one_be)).digits().as_slice()
    ));
};

#[cfg(test)]
mod test {
    use super::*;
    use num_bigint::BigUint;
    use proptest::prelude::*;
    use std::str::FromStr;
    use test_strategy::proptest;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[test]
    fn endianness() {
        let one_platform = U256::ONE;
        let one_le = {
            let mut buf = [0; 32];
            buf[0] = 1;
            buf
        };

        let one_be = {
            let mut buf = [0; 32];
            buf[31] = 1;
            buf
        };

        // To little endian
        let le = one_platform.to_le();
        assert_eq!(one_le, *le.digits());

        // To big endian
        let be = one_platform.to_be();
        assert_eq!(one_be, *be.digits());

        // From little endian
        assert_eq!(one_platform, U256::from_le(U256::from_digits(one_le)));
        // From big endian
        assert_eq!(one_platform, U256::from_be(U256::from_digits(one_be)));
    }

    #[proptest]
    fn dont_crash_on_large_inputs(
        #[strategy(proptest::collection::vec(any::<u8>(), 33..1024))] bytes: Vec<u8>,
    ) {
        let big_int = BigUint::from_bytes_be(&bytes);
        let radix10 = big_int.to_str_radix(10);

        // doesn't crash
        let _ = U256::from_str_radix(&radix10, 10);
    }

    #[proptest]
    fn valid_u256_strings(
        #[strategy(proptest::collection::vec(any::<u8>(), 1..=32))] bytes: Vec<u8>,
    ) {
        let big_int = BigUint::from_bytes_be(&bytes);
        let radix10 = big_int.to_str_radix(10);

        let u256 = U256::from_str_radix(&radix10, 10).unwrap();

        assert_eq!(radix10, u256.to_str_radix(10));

        let from_str = U256::from_str(&radix10).unwrap();
        assert_eq!(from_str, u256);
        assert_eq!(radix10, from_str.to_string());
    }
}

// Before we can expose this in the public interface it likely needs to be wrapped so that the type
// from our dependency doesn't leak
pub(crate) type U256 = bnum::BUintD8<32>;

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

    proptest! {
        #[test]
        fn dont_crash_on_large_inputs(
            bytes in proptest::collection::vec(any::<u8>(), 33..1024)
        ) {
            let big_int = BigUint::from_bytes_be(&bytes);
            let radix10 = big_int.to_str_radix(10);

            // doesn't crash
            let _ = U256::from_str_radix(&radix10, 10);
        }

        #[test]
        fn valid_u256_strings(
            bytes in proptest::collection::vec(any::<u8>(), 1..=32)
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
}

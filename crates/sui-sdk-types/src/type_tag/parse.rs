use super::Address;
use super::Identifier;
use super::StructTag;
use super::TypeTag;

use winnow::ModalParser;
use winnow::ModalResult;
use winnow::Parser;
use winnow::ascii::space0;
use winnow::combinator::alt;
use winnow::combinator::delimited;
use winnow::combinator::eof;
use winnow::combinator::opt;
use winnow::combinator::separated;
use winnow::stream::AsChar;
use winnow::token::one_of;
use winnow::token::take_while;

// static ALLOWED_IDENTIFIERS: &str = r"(?:[a-zA-Z][a-zA-Z0-9_]*)|(?:_[a-zA-Z0-9_]+)";
static MAX_IDENTIFIER_LENGTH: usize = 128;

/// Maximum nesting depth for type tags. Each `vector<...>` or struct generic
/// type parameter counts as one level.
static MAX_TYPE_TAG_DEPTH: usize = 128;

pub(super) fn parse_identifier(mut input: &str) -> ModalResult<&str> {
    (identifier, eof).take().parse_next(&mut input)
}

fn identifier<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    alt((
        (one_of(|c: char| c.is_alpha()), valid_remainder(0)),
        ('_', valid_remainder(1)),
    ))
    .take()
    .parse_next(input)
}

fn valid_remainder<'a>(
    minimum: usize,
) -> impl ModalParser<&'a str, &'a str, winnow::error::ContextError> {
    move |input: &mut &'a str| {
        take_while(
            // Use .. instead of ..= since we've already processed a single character
            minimum..MAX_IDENTIFIER_LENGTH,
            (b'_', b'a'..=b'z', b'A'..=b'Z', b'0'..=b'9'),
        )
        .parse_next(input)
    }
}

fn parse_address<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    ("0x", take_while(1..=64, AsChar::is_hex_digit))
        .take()
        .parse_next(input)
}

pub(super) fn parse_type_tag(mut input: &str) -> Result<TypeTag, Option<String>> {
    let tag = type_tag(&mut input, 0).map_err(extract_message)?;
    eof.parse_next(&mut input).map_err(extract_message)?;
    Ok(tag)
}

/// Extract a human-readable message from a winnow error, if one was attached
/// via `StrContext::Label`.
fn extract_message(err: winnow::error::ErrMode<winnow::error::ContextError>) -> Option<String> {
    use winnow::error::StrContext;

    let inner = match err {
        winnow::error::ErrMode::Backtrack(e) | winnow::error::ErrMode::Cut(e) => e,
        winnow::error::ErrMode::Incomplete(_) => return None,
    };

    inner.context().find_map(|ctx| match ctx {
        StrContext::Label(msg) => Some(msg.to_string()),
        _ => None,
    })
}

fn type_tag(input: &mut &str, depth: usize) -> ModalResult<TypeTag> {
    if depth > MAX_TYPE_TAG_DEPTH {
        let mut err = winnow::error::ContextError::new();
        err.push(winnow::error::StrContext::Label(
            "type exceeds the maximum nesting depth of 128",
        ));
        return Err(winnow::error::ErrMode::Cut(err));
    }

    alt((
        "u8".value(TypeTag::U8),
        "u16".value(TypeTag::U16),
        "u32".value(TypeTag::U32),
        "u64".value(TypeTag::U64),
        "u128".value(TypeTag::U128),
        "u256".value(TypeTag::U256),
        "bool".value(TypeTag::Bool),
        "address".value(TypeTag::Address),
        "signer".value(TypeTag::Signer),
        |input: &mut &str| {
            "vector<".parse_next(input)?;
            let ty = type_tag(input, depth + 1)?;
            ">".parse_next(input)?;
            Ok(TypeTag::Vector(Box::new(ty)))
        },
        |input: &mut &str| struct_tag(input, depth).map(|s| TypeTag::Struct(Box::new(s))),
    ))
    .parse_next(input)
}

pub(super) fn parse_struct_tag(mut input: &str) -> Result<StructTag, Option<String>> {
    let tag = struct_tag(&mut input, 0).map_err(extract_message)?;
    eof.parse_next(&mut input).map_err(extract_message)?;
    Ok(tag)
}

fn struct_tag(input: &mut &str, depth: usize) -> ModalResult<StructTag> {
    let (address, _, module, _, name) = (
        parse_address.try_map(|s| s.parse::<Address>()),
        "::",
        identifier.map(|ident| Identifier(ident.into())),
        "::",
        identifier.map(|ident| Identifier(ident.into())),
    )
        .parse_next(input)?;

    // optional generic
    let generics = opt(|input: &mut &str| {
        "<".parse_next(input)?;
        let result = generics_with_depth(input, depth)?;
        ">".parse_next(input)?;
        Ok(result)
    })
    .parse_next(input)?
    .unwrap_or_default();

    Ok(StructTag {
        address,
        module,
        name,
        type_params: generics,
    })
}

fn generics_with_depth(input: &mut &str, depth: usize) -> ModalResult<Vec<TypeTag>> {
    separated(
        1..,
        delimited(
            space0,
            |input: &mut &str| type_tag(input, depth + 1),
            space0,
        ),
        ",",
    )
    .parse_next(input)
}

//
// const identifier validity check
//
pub(super) const fn is_valid_identifier(s: &str) -> bool {
    const fn is_valid_remainder_byte(b: u8) -> bool {
        matches!(b, b'_' | b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9')
    }

    let bytes = s.as_bytes();
    if bytes.is_empty() || bytes.len() > MAX_IDENTIFIER_LENGTH {
        return false;
    }

    // Check that the identifier either starts with an alpha or an underscore + at least one more
    // byte
    let remainder = match bytes {
        [b'a'..=b'z', remainder @ ..] | [b'A'..=b'Z', remainder @ ..] => remainder,
        [b'_', remainder @ ..] if !remainder.is_empty() => remainder,
        _ => return false,
    };

    let mut i = 0;
    while i < remainder.len() {
        if !is_valid_remainder_byte(remainder[i]) {
            return false;
        }
        i += 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    use proptest::prelude::*;
    use test_strategy::proptest;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[test]
    fn test_type_tag() {
        for s in &[
            "u64",
            "bool",
            "vector<u8>",
            "vector<vector<u64>>",
            "vector<u16>",
            "vector<vector<u16>>",
            "vector<u32>",
            "vector<vector<u32>>",
            "vector<u128>",
            "vector<vector<u128>>",
            "vector<u256>",
            "vector<vector<u256>>",
            "signer",
            "0x1::M::S",
            "0x2::M::S_",
            "0x3::M_::S",
            "0x4::M_::S_",
            "0x00000000004::M::S",
            "0x1::M::S<u64>",
            "0x1::M::S<u16>",
            "0x1::M::S<u32>",
            "0x1::M::S<u256>",
            "0x1::M::S<0x2::P::Q>",
            "vector<0x1::M::S>",
            "vector<0x1::M_::S_>",
            "vector<vector<0x1::M_::S_>>",
            "0x1::M::S<vector<u8>>",
            "0x1::M::S<vector<u16>>",
            "0x1::M::S<vector<u32>>",
            "0x1::M::S<vector<u64>>",
            "0x1::M::S<vector<u128>>",
            "0x1::M::S<vector<u256>>",
            "0x1::_bar::_BAR",
            "0x1::__::__",
            "0x1::_bar::_BAR<0x2::_____::______fooo______>",
            "0x1::__::__<0x2::_____::______fooo______, 0xff::Bar____::_______foo>",
            "0x5d32d749705c5f07c741f1818df3db466128bf01677611a959b03040ac5dc774::slippage::HopSwapEvent<0x2::sui::SUI, 0x3c86bba6a3d3ce958615ae51cc5604f58956b1583323f664cf5f048da0fcbb19::_spd::_SPD>",
        ] {
            assert!(parse_type_tag(s).is_ok(), "Failed to parse tag {s}");
        }
    }

    #[test]
    fn test_parse_valid_struct_type() {
        let valid = vec![
            "0x1::Foo::Foo",
            "0x1::Foo_Type::Foo",
            "0x1::Foo_::Foo",
            "0x1::X_123::X32_",
            "0x1::Foo::Foo_Type",
            "0x1::Foo::Foo<0x1::ABC::ABC>",
            "0x1::Foo::Foo<0x1::ABC::ABC_Type>",
            "0x1::Foo::Foo<u8>",
            "0x1::Foo::Foo<u16>",
            "0x1::Foo::Foo<u32>",
            "0x1::Foo::Foo<u64>",
            "0x1::Foo::Foo<u128>",
            "0x1::Foo::Foo<u256>",
            "0x1::Foo::Foo<bool>",
            "0x1::Foo::Foo<address>",
            "0x1::Foo::Foo<signer>",
            "0x1::Foo::Foo<vector<0x1::ABC::ABC>>",
            "0x1::Foo::Foo<u8,bool>",
            "0x1::Foo::Foo<u8,   bool>",
            "0x1::Foo::Foo<u8  ,bool>",
            "0x1::Foo::Foo<u8 , bool  ,    vector<u8>,address,signer>",
            "0x1::Foo::Foo<vector<0x1::Foo::Struct<0x1::XYZ::XYZ>>>",
            "0x1::Foo::Foo<0x1::Foo::Struct<vector<0x1::XYZ::XYZ>, 0x1::Foo::Foo<vector<0x1::Foo::Struct<0x1::XYZ::XYZ>>>>>",
            "0x1::_bar::_BAR",
            "0x1::__::__",
            "0x1::_bar::_BAR<0x2::_____::______fooo______>",
            "0x1::__::__<0x2::_____::______fooo______, 0xff::Bar____::_______foo>",
            "0x5d32d749705c5f07c741f1818df3db466128bf01677611a959b03040ac5dc774::slippage::HopSwapEvent<0x2::sui::SUI, 0x3c86bba6a3d3ce958615ae51cc5604f58956b1583323f664cf5f048da0fcbb19::_spd::_SPD>",
        ];
        for s in valid {
            assert!(
                dbg!(parse_type_tag(s)).is_ok(),
                "Failed to parse struct {s}",
            );
        }
    }

    #[test]
    fn test_parse_struct_tag_with_type_names() {
        let names = vec![
            "address", "vector", "u128", "u256", "u64", "u32", "u16", "u8", "bool", "signer",
        ];

        let mut tests = vec![];
        for name in &names {
            for name_type in &names {
                tests.push(format!("0x1::{name}::{name_type}"))
            }
        }

        let mut instantiations = vec![];
        for ty in &tests {
            for other_ty in &tests {
                instantiations.push(format!("{ty}<{other_ty}>"))
            }
        }

        for text in tests.iter().chain(instantiations.iter()) {
            let st = parse_struct_tag(text).expect("valid StructTag");
            assert_eq!(
                st.to_string().replace(' ', ""),
                text.replace(' ', "")
                    .replace("0x1", &Address::from_str("0x1").unwrap().to_string()),
                "text: {text:?}, StructTag: {st:?}"
            );
        }
    }

    #[proptest]
    fn test_identifier_parsing_matches(s: String) {
        match Identifier::new(&s) {
            Ok(_) => assert!(is_valid_identifier(&s)),
            Err(_) => assert!(!is_valid_identifier(&s)),
        }
    }

    #[test]
    fn deeply_nested_type_tag_returns_error() {
        // Build a deeply nested type string: 0x1::m::S<0x1::m::S<0x1::m::S<...>>>
        // This must return a parse error, not overflow the stack.
        let depth = 1000;
        let mut tag = String::from("0x1::m::S");
        for _ in 0..depth {
            tag = format!("0x1::m::S<{tag}>");
        }
        let err = TypeTag::from_str(&tag).unwrap_err();
        assert!(
            err.to_string().contains("nesting depth"),
            "error should mention nesting depth, got: {err}"
        );
    }

    #[test]
    fn deeply_nested_vector_returns_error() {
        // Build a deeply nested vector type: vector<vector<vector<...u8...>>>
        let depth = 1000;
        let mut tag = String::from("u8");
        for _ in 0..depth {
            tag = format!("vector<{tag}>");
        }
        assert!(
            TypeTag::from_str(&tag).is_err(),
            "deeply nested vector type should be rejected"
        );
    }

    #[test]
    fn type_tag_at_max_depth_is_rejected() {
        // A type at exactly MAX_TYPE_TAG_DEPTH + 1 nesting levels must fail.
        let mut tag = String::from("u8");
        for _ in 0..=MAX_TYPE_TAG_DEPTH {
            tag = format!("vector<{tag}>");
        }
        assert!(
            TypeTag::from_str(&tag).is_err(),
            "type tag exceeding max depth should be rejected"
        );
    }

    #[test]
    fn type_tag_within_max_depth_is_accepted() {
        // A type with moderate nesting should still parse fine.
        let mut tag = String::from("u8");
        for _ in 0..16 {
            tag = format!("vector<{tag}>");
        }
        assert!(
            TypeTag::from_str(&tag).is_ok(),
            "moderately nested type tag should parse successfully"
        );
    }

    // Custom strategies that exercise the full TypeTag grammar, including vectors and nested
    // generics. The existing `Arbitrary` derives intentionally skip these (Vector has weight 0,
    // type_params is always empty) to keep BCS serialization fuzz tests fast.

    /// Strategy producing valid Move identifiers, including both
    /// letter-prefixed (`Foo`, `a1`) and underscore-prefixed (`_bar`, `__x1`).
    fn arb_identifier() -> impl Strategy<Value = Identifier> {
        prop_oneof!["[a-zA-Z][a-zA-Z0-9_]{0,127}", "_[a-zA-Z0-9_]{1,127}",]
            .prop_map(|s| Identifier::new(&s).unwrap())
    }

    /// Strategy producing a StructTag with empty type_params.
    fn arb_struct_tag_base() -> impl Strategy<Value = StructTag> {
        (any::<Address>(), arb_identifier(), arb_identifier())
            .prop_map(|(addr, module, name)| StructTag::new(addr, module, name, Vec::new()))
    }

    /// Strategy producing valid TypeTags including vectors, structs with generics,
    /// and nested combinations. Recursion is bounded to depth 4.
    fn arb_type_tag() -> impl Strategy<Value = TypeTag> {
        let leaf = prop_oneof![
            Just(TypeTag::U8),
            Just(TypeTag::U16),
            Just(TypeTag::U32),
            Just(TypeTag::U64),
            Just(TypeTag::U128),
            Just(TypeTag::U256),
            Just(TypeTag::Bool),
            Just(TypeTag::Address),
            Just(TypeTag::Signer),
        ];

        leaf.prop_recursive(
            4,  // max depth of recursion
            64, // desired_size (target number of nodes)
            3,  // expected_branch_size per recursive step
            |inner| {
                let vector_strat = inner.clone().prop_map(|t| TypeTag::Vector(Box::new(t)));

                let struct_no_params =
                    arb_struct_tag_base().prop_map(|s| TypeTag::Struct(Box::new(s)));

                let struct_with_params = (
                    any::<Address>(),
                    arb_identifier(),
                    arb_identifier(),
                    proptest::collection::vec(inner, 1..=3),
                )
                    .prop_map(|(addr, module, name, params)| {
                        TypeTag::Struct(Box::new(StructTag::new(addr, module, name, params)))
                    });

                prop_oneof![vector_strat, struct_no_params, struct_with_params,]
            },
        )
    }

    /// Reduced set of primitive TypeTag leaves for depth-focused tests.
    fn arb_simple_leaf() -> impl Strategy<Value = TypeTag> {
        prop_oneof![Just(TypeTag::U8), Just(TypeTag::U64), Just(TypeTag::Bool)]
    }

    #[cfg_attr(target_arch = "wasm32", proptest(cases = 50))]
    #[proptest]
    fn type_tag_roundtrip(#[strategy(arb_type_tag())] type_tag: TypeTag) {
        let s = type_tag.to_string();
        let parsed = s.parse::<TypeTag>().unwrap();
        assert_eq!(type_tag, parsed);
    }

    #[proptest]
    fn identifier_roundtrip(#[strategy(arb_identifier())] ident: Identifier) {
        let s = ident.to_string();
        let parsed = s.parse::<Identifier>().unwrap();
        assert_eq!(ident, parsed);
    }

    #[proptest]
    fn nested_vector_parsing(
        #[strategy(0u32..=8)] depth: u32,
        #[strategy(arb_simple_leaf())] leaf: TypeTag,
    ) {
        let mut ty = leaf;
        for _ in 0..depth {
            ty = TypeTag::Vector(Box::new(ty));
        }
        let s = ty.to_string();
        let parsed = s.parse::<TypeTag>().unwrap();
        assert_eq!(ty, parsed);
    }
}

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

pub(super) fn parse_type_tag(mut input: &str) -> ModalResult<TypeTag> {
    (type_tag, eof).parse_next(&mut input).map(|(t, _)| t)
}

fn type_tag(input: &mut &str) -> ModalResult<TypeTag> {
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
        delimited("vector<", type_tag, ">").map(|ty| TypeTag::Vector(Box::new(ty))),
        struct_tag.map(|s| TypeTag::Struct(Box::new(s))),
    ))
    .parse_next(input)
}

pub(super) fn parse_struct_tag(mut input: &str) -> ModalResult<StructTag> {
    (struct_tag, eof).parse_next(&mut input).map(|(s, _)| s)
}

fn struct_tag(input: &mut &str) -> ModalResult<StructTag> {
    let (address, _, module, _, name) = (
        parse_address.try_map(|s| s.parse::<Address>()),
        "::",
        identifier.map(|ident| Identifier(ident.into())),
        "::",
        identifier.map(|ident| Identifier(ident.into())),
    )
        .parse_next(input)?;

    // optional generic
    let generics = opt(delimited("<", generics, ">"))
        .parse_next(input)?
        .unwrap_or_default();

    Ok(StructTag {
        address,
        module,
        name,
        type_params: generics,
    })
}

fn generics(input: &mut &str) -> ModalResult<Vec<TypeTag>> {
    separated(1.., delimited(space0, type_tag, space0), ",").parse_next(input)
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

//TODO add proptests
#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

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
            let mut input = s;
            assert!(
                dbg!((type_tag, eof).parse_next(&mut input)).is_ok(),
                "Failed to parse struct {s}, remainder {input}",
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

    #[test_strategy::proptest]
    fn test_identifier_parsing_matches(s: String) {
        match Identifier::new(&s) {
            Ok(_) => assert!(is_valid_identifier(&s)),
            Err(_) => assert!(!is_valid_identifier(&s)),
        }
    }
}

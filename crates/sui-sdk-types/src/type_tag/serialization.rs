use std::cell::Cell;

use super::*;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::DeserializeSeed;
use serde::de::Visitor;
use serde_with::DeserializeAs;
use serde_with::SerializeAs;

use super::parse::MAX_TYPE_NODE_COUNT;
use super::parse::MAX_TYPE_TAG_DEPTH;

/// Increment a shared `Cell<u64>` node counter and reject when the running
/// total exceeds `MAX_TYPE_NODE_COUNT`. The counter is allocated on the
/// stack at the top-level entry point and shared by reference through
/// every recursive seed; `Cell` provides single-threaded interior
/// mutability without the memory-barrier overhead of an atomic, and
/// references are `Copy` so seeds carry it without any cloning.
fn account_node<E: serde::de::Error>(nodes: &Cell<u64>) -> Result<(), E> {
    let next = nodes.get().saturating_add(1);
    if next > MAX_TYPE_NODE_COUNT {
        return Err(serde::de::Error::custom(format!(
            "type tag exceeds max node count of {MAX_TYPE_NODE_COUNT}"
        )));
    }
    nodes.set(next);
    Ok(())
}

/// Seed that threads the current 0-indexed nesting depth and a borrowed
/// node counter through the recursive BCS `TypeTag` deserialization graph.
/// Each successive `TypeTag` boundary (inside `Vector(inner)` or
/// `StructTag.type_params[i]`) receives a seed with `depth + 1` carrying
/// the same counter reference. Rejects on
/// `depth >= MAX_TYPE_TAG_DEPTH` (catches deep nesting) or
/// `node count > MAX_TYPE_NODE_COUNT` (catches wide types), matching the
/// bounds enforced by the string parser at `parse::type_tag`.
struct TypeTagSeed<'a> {
    depth: usize,
    nodes: &'a Cell<u64>,
}

impl<'de, 'a> DeserializeSeed<'de> for TypeTagSeed<'a> {
    type Value = TypeTag;

    fn deserialize<D>(self, deserializer: D) -> Result<TypeTag, D::Error>
    where
        D: Deserializer<'de>,
    {
        account_node(self.nodes)?;
        if self.depth >= MAX_TYPE_TAG_DEPTH {
            return Err(serde::de::Error::custom(format!(
                "type tag exceeds max nesting depth of {MAX_TYPE_TAG_DEPTH}"
            )));
        }
        let variants = &[
            "bool", "u8", "u64", "u128", "address", "signer", "vector", "struct", "u16", "u32",
            "u256",
        ];
        deserializer.deserialize_enum(
            "TypeTag",
            variants,
            TypeTagSeedVisitor {
                depth: self.depth,
                nodes: self.nodes,
            },
        )
    }
}

/// Seed that deserializes a `StructTag` while preserving the nesting depth
/// and shared node counter for its `type_params` elements. Used both as
/// the recursive landing pad from `TypeTag::Struct(s)` and as the root of
/// external `bcs::from_bytes::<StructTag>(...)` decoding.
struct StructTagSeed<'a> {
    /// Depth that should be passed to each `type_params[i]`.
    type_param_depth: usize,
    nodes: &'a Cell<u64>,
}

impl<'de, 'a> DeserializeSeed<'de> for StructTagSeed<'a> {
    type Value = StructTag;

    fn deserialize<D>(self, deserializer: D) -> Result<StructTag, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "BinaryStructTag",
            &["address", "module", "name", "type_params"],
            StructTagSeedVisitor {
                type_param_depth: self.type_param_depth,
                nodes: self.nodes,
            },
        )
    }
}

/// Seed that deserializes a `Vec<TypeTag>` such that every element receives
/// a `TypeTagSeed` carrying the inherited `depth` and the same borrowed
/// node-counter reference.
struct TypeTagVecSeed<'a> {
    depth: usize,
    nodes: &'a Cell<u64>,
}

impl<'de, 'a> DeserializeSeed<'de> for TypeTagVecSeed<'a> {
    type Value = Vec<TypeTag>;

    fn deserialize<D>(self, deserializer: D) -> Result<Vec<TypeTag>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(TypeTagVecVisitor {
            depth: self.depth,
            nodes: self.nodes,
        })
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_with::DisplayFromStr::deserialize_as(deserializer)
    }
}

#[repr(u32)]
enum SerializedTypeTagVariant {
    Bool = 0,
    U8 = 1,
    U64 = 2,
    U128 = 3,
    Address = 4,
    Signer = 5,
    Vector = 6,
    Struct = 7,
    U16 = 8,
    U32 = 9,
    U256 = 10,
}

impl SerializedTypeTagVariant {
    fn new(variant: u32) -> Result<Self, u32> {
        Ok(match variant {
            0 => Self::Bool,
            1 => Self::U8,
            2 => Self::U64,
            3 => Self::U128,
            4 => Self::Address,
            5 => Self::Signer,
            6 => Self::Vector,
            7 => Self::Struct,
            8 => Self::U16,
            9 => Self::U32,
            10 => Self::U256,
            unknown => return Err(unknown),
        })
    }
}

impl Serialize for TypeTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serde_with::DisplayFromStr::serialize_as(self, serializer)
        } else {
            match self {
                TypeTag::U8 => serializer.serialize_unit_variant(
                    "TypeTag",
                    SerializedTypeTagVariant::U8 as u32,
                    "u8",
                ),
                TypeTag::U16 => serializer.serialize_unit_variant(
                    "TypeTag",
                    SerializedTypeTagVariant::U16 as u32,
                    "u16",
                ),
                TypeTag::U32 => serializer.serialize_unit_variant(
                    "TypeTag",
                    SerializedTypeTagVariant::U32 as u32,
                    "u32",
                ),
                TypeTag::U64 => serializer.serialize_unit_variant(
                    "TypeTag",
                    SerializedTypeTagVariant::U64 as u32,
                    "u64",
                ),
                TypeTag::U128 => serializer.serialize_unit_variant(
                    "TypeTag",
                    SerializedTypeTagVariant::U128 as u32,
                    "u128",
                ),
                TypeTag::U256 => serializer.serialize_unit_variant(
                    "TypeTag",
                    SerializedTypeTagVariant::U256 as u32,
                    "u256",
                ),
                TypeTag::Bool => serializer.serialize_unit_variant(
                    "TypeTag",
                    SerializedTypeTagVariant::Bool as u32,
                    "bool",
                ),
                TypeTag::Address => serializer.serialize_unit_variant(
                    "TypeTag",
                    SerializedTypeTagVariant::Address as u32,
                    "address",
                ),
                TypeTag::Signer => serializer.serialize_unit_variant(
                    "TypeTag",
                    SerializedTypeTagVariant::Signer as u32,
                    "signer",
                ),
                TypeTag::Vector(v) => serializer.serialize_newtype_variant(
                    "TypeTag",
                    SerializedTypeTagVariant::Vector as u32,
                    "vector",
                    v,
                ),
                TypeTag::Struct(s) => serializer.serialize_newtype_variant(
                    "TypeTag",
                    SerializedTypeTagVariant::Struct as u32,
                    "struct",
                    s,
                ),
            }
        }
    }
}

impl<'de> Deserialize<'de> for TypeTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            serde_with::DisplayFromStr::deserialize_as(deserializer)
        } else {
            // Top-level entry into the seed graph at depth 0 with a fresh
            // node counter, matching the string parser which calls
            // `type_tag(input, 0, &Cell::new(0))`.
            let nodes = Cell::new(0);
            TypeTagSeed {
                depth: 0,
                nodes: &nodes,
            }
            .deserialize(deserializer)
        }
    }
}

struct TypeTagSeedVisitor<'a> {
    depth: usize,
    nodes: &'a Cell<u64>,
}

impl<'de, 'a> Visitor<'de> for TypeTagSeedVisitor<'a> {
    type Value = TypeTag;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("TypeTag")
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        use serde::de::VariantAccess;

        let (variant, deserializer) = data.variant::<u32>()?;
        match SerializedTypeTagVariant::new(variant)
            .map_err(|e| serde::de::Error::custom(format!("unknown variant {e}")))?
        {
            SerializedTypeTagVariant::Bool => deserializer.unit_variant().map(|_| TypeTag::Bool),
            SerializedTypeTagVariant::U8 => deserializer.unit_variant().map(|_| TypeTag::U8),
            SerializedTypeTagVariant::U64 => deserializer.unit_variant().map(|_| TypeTag::U64),
            SerializedTypeTagVariant::U128 => deserializer.unit_variant().map(|_| TypeTag::U128),
            SerializedTypeTagVariant::Address => {
                deserializer.unit_variant().map(|_| TypeTag::Address)
            }
            SerializedTypeTagVariant::Signer => {
                deserializer.unit_variant().map(|_| TypeTag::Signer)
            }
            SerializedTypeTagVariant::Vector => deserializer
                .newtype_variant_seed(TypeTagSeed {
                    depth: self.depth + 1,
                    nodes: self.nodes,
                })
                .map(|t| TypeTag::Vector(Box::new(t))),
            SerializedTypeTagVariant::Struct => deserializer
                .newtype_variant_seed(StructTagSeed {
                    type_param_depth: self.depth + 1,
                    nodes: self.nodes,
                })
                .map(|s| TypeTag::Struct(Box::new(s))),
            SerializedTypeTagVariant::U16 => deserializer.unit_variant().map(|_| TypeTag::U16),
            SerializedTypeTagVariant::U32 => deserializer.unit_variant().map(|_| TypeTag::U32),
            SerializedTypeTagVariant::U256 => deserializer.unit_variant().map(|_| TypeTag::U256),
        }
    }
}

struct StructTagSeedVisitor<'a> {
    type_param_depth: usize,
    nodes: &'a Cell<u64>,
}

impl<'de, 'a> Visitor<'de> for StructTagSeedVisitor<'a> {
    type Value = StructTag;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a StructTag")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let address: Address = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::custom("missing StructTag address"))?;
        let module: Identifier = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::custom("missing StructTag module"))?;
        let name: Identifier = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::custom("missing StructTag name"))?;
        let type_params: Vec<TypeTag> = seq
            .next_element_seed(TypeTagVecSeed {
                depth: self.type_param_depth,
                nodes: self.nodes,
            })?
            .ok_or_else(|| serde::de::Error::custom("missing StructTag type_params"))?;

        Ok(StructTag {
            address,
            module,
            name,
            type_params,
        })
    }
}

struct TypeTagVecVisitor<'a> {
    depth: usize,
    nodes: &'a Cell<u64>,
}

impl<'de, 'a> Visitor<'de> for TypeTagVecVisitor<'a> {
    type Value = Vec<TypeTag>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a sequence of TypeTags")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        // Clamp the wire-supplied length hint by the remaining node budget
        // so an attacker-supplied uleb128 sequence length cannot drive
        // `Vec::with_capacity` past the node-count bound the deserializer
        // is already prepared to enforce. Without this clamp, the bcs
        // crate's `SeqAccess::size_hint` propagates the parsed uleb32
        // length (up to ~2 billion) and `Vec::<TypeTag>::with_capacity`
        // attempts a multi-GB allocation before any element is parsed.
        let remaining = MAX_TYPE_NODE_COUNT.saturating_sub(self.nodes.get()) as usize;
        let cap = seq.size_hint().unwrap_or(0).min(remaining);
        let mut out = Vec::with_capacity(cap);
        while let Some(t) = seq.next_element_seed(TypeTagSeed {
            depth: self.depth,
            nodes: self.nodes,
        })? {
            out.push(t);
        }
        Ok(out)
    }
}

#[derive(serde_derive::Serialize)]
struct BinaryStructTagRef<'a> {
    address: &'a Address,
    module: &'a Identifier,
    name: &'a Identifier,
    type_params: &'a [TypeTag],
}

impl Serialize for StructTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serde_with::DisplayFromStr::serialize_as(self, serializer)
        } else {
            BinaryStructTagRef {
                address: &self.address,
                module: &self.module,
                name: &self.name,
                type_params: &self.type_params,
            }
            .serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for StructTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            serde_with::DisplayFromStr::deserialize_as(deserializer)
        } else {
            // Treat the `StructTag` as if it were the body of a TypeTag at
            // depth 0, so its `type_params` start at depth 1 — matching
            // `parse_struct_tag`, which calls `struct_tag(depth=0)` →
            // `generics_with_depth(depth=0)` → `type_tag(depth+1=1)`.
            // Also count the outer StructTag as one node so the
            // `bcs::from_bytes::<StructTag>` boundary matches
            // `bcs::from_bytes::<TypeTag>` for the same shape.
            let nodes = Cell::new(0);
            account_node(&nodes)?;
            StructTagSeed {
                type_param_depth: 1,
                nodes: &nodes,
            }
            .deserialize(deserializer)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::str::FromStr;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[test]
    fn type_tag_fixture() {
        let expected = TypeTag::Struct(Box::new(StructTag {
            address: Address::from_str("0x1").unwrap(),
            module: Identifier("Foo".into()),
            name: Identifier("Bar".into()),
            type_params: vec![
                TypeTag::Bool,
                TypeTag::U8,
                TypeTag::U64,
                TypeTag::U128,
                TypeTag::Address,
                TypeTag::Signer,
                TypeTag::U16,
                TypeTag::U32,
                TypeTag::U256,
                TypeTag::Vector(Box::new(TypeTag::Address)),
            ],
        }));

        let display = "0x0000000000000000000000000000000000000000000000000000000000000001::Foo::Bar<bool,u8,u64,u128,address,signer,u16,u32,u256,vector<address>>";
        let bcs_fixture: &[u8] = &[
            7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 3, 70, 111, 111, 3, 66, 97, 114, 10, 0, 1, 2, 3, 4, 5, 8, 9, 10, 6, 4,
        ];

        let type_from_string = TypeTag::from_str(display).unwrap();
        assert_eq!(type_from_string, expected);
        let type_from_bcs: TypeTag = bcs::from_bytes(bcs_fixture).unwrap();
        assert_eq!(type_from_bcs, expected);

        assert_eq!(type_from_string, type_from_bcs);
        assert_eq!(bcs_fixture, bcs::to_bytes(&type_from_bcs).unwrap());
        assert_eq!(bcs_fixture, bcs::to_bytes(&type_from_string).unwrap());
        assert_eq!(display, type_from_string.to_string().replace(' ', ""));
    }

    // Regression test: the BCS deserialization path used to inherit only
    // bcs's 500-frame container-depth limit, while the equivalent string
    // parser rejected anything beyond `MAX_TYPE_TAG_DEPTH`. Both paths now
    // share the same bound and reject `depth >= MAX_TYPE_TAG_DEPTH`.
    #[test]
    fn bcs_deserialize_at_max_depth_is_rejected() {
        // BCS encoding: each `Vector(inner)` is variant 6 (uleb128 = 0x06)
        // followed by `inner`. `MAX_TYPE_TAG_DEPTH` vector wrappers around
        // a `Bool` (variant 0 = 0x00) places the `Bool` at depth
        // `MAX_TYPE_TAG_DEPTH` — one past the deepest allowed level.
        let mut bytes = vec![SerializedTypeTagVariant::Bool as u8];
        for _ in 0..MAX_TYPE_TAG_DEPTH {
            bytes.insert(0, SerializedTypeTagVariant::Vector as u8);
        }
        let err = bcs::from_bytes::<TypeTag>(&bytes).unwrap_err();
        assert!(
            err.to_string().contains("max nesting depth"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn bcs_deserialize_within_max_depth_is_accepted() {
        let mut bytes = vec![SerializedTypeTagVariant::Bool as u8];
        // 16 nested vectors round-trip cleanly — well below the depth bound
        // and below bcs's own 500-frame limit.
        for _ in 0..16 {
            bytes.insert(0, SerializedTypeTagVariant::Vector as u8);
        }
        let parsed: TypeTag = bcs::from_bytes(&bytes).unwrap();
        assert_eq!(bcs::to_bytes(&parsed).unwrap(), bytes);
    }

    // Boundary case: `MAX_TYPE_TAG_DEPTH - 1` vector wrappers around a
    // primitive places the primitive at depth `MAX_TYPE_TAG_DEPTH - 1` —
    // the deepest still-legal level — for a total of `MAX_TYPE_TAG_DEPTH`
    // levels. This is accepted by both the string parser and the BCS
    // path.
    #[test]
    fn bcs_deserialize_at_exact_max_depth_is_accepted() {
        let mut bytes = vec![SerializedTypeTagVariant::Bool as u8];
        for _ in 0..(MAX_TYPE_TAG_DEPTH - 1) {
            bytes.insert(0, SerializedTypeTagVariant::Vector as u8);
        }
        let parsed: TypeTag = bcs::from_bytes(&bytes).unwrap();
        assert_eq!(bcs::to_bytes(&parsed).unwrap(), bytes);
    }

    // Regression test: when a `StructTag` is the BCS root (rather than a
    // `TypeTag::Struct(s)` recursion), its `type_params` must start at
    // depth 1 — matching `parse_struct_tag` — not depth 0. With the old
    // derive-based path, an external `StructTag` accepted one level of
    // nesting more than the string form.
    #[test]
    fn bcs_struct_tag_external_entry_at_max_depth_is_rejected() {
        // type_params element at depth 1, with N vector wraps around
        // a Bool -> Bool at depth `1 + N`. Reject when `1 + N >= MAX`,
        // i.e., `N >= MAX - 1`. Build the just-too-deep case.
        let mut type_param = TypeTag::Bool;
        for _ in 0..(MAX_TYPE_TAG_DEPTH - 1) {
            type_param = TypeTag::Vector(Box::new(type_param));
        }
        let st = StructTag {
            address: Address::from_str("0x1").unwrap(),
            module: Identifier("M".into()),
            name: Identifier("N".into()),
            type_params: vec![type_param],
        };
        let bytes = bcs::to_bytes(&st).unwrap();
        let err = bcs::from_bytes::<StructTag>(&bytes).unwrap_err();
        assert!(
            err.to_string().contains("max nesting depth"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn bcs_struct_tag_external_entry_at_exact_max_depth_is_accepted() {
        // Deepest still-legal type_param: `N = MAX - 2` vectors, putting
        // the inner Bool at depth `MAX - 1`.
        let mut type_param = TypeTag::Bool;
        for _ in 0..(MAX_TYPE_TAG_DEPTH - 2) {
            type_param = TypeTag::Vector(Box::new(type_param));
        }
        let st = StructTag {
            address: Address::from_str("0x1").unwrap(),
            module: Identifier("M".into()),
            name: Identifier("N".into()),
            type_params: vec![type_param],
        };
        let bytes = bcs::to_bytes(&st).unwrap();
        let parsed: StructTag = bcs::from_bytes(&bytes).unwrap();
        assert_eq!(parsed, st);
    }

    // Wide-type regression: a `TypeTag::Struct` with `MAX_TYPE_NODE_COUNT`
    // primitive type_params totals `MAX_TYPE_NODE_COUNT + 1` nodes (one for
    // the outer TypeTag::Struct itself), exceeding the bound.
    #[test]
    fn bcs_type_tag_at_max_node_count_is_rejected() {
        let st = StructTag {
            address: Address::from_str("0x1").unwrap(),
            module: Identifier("M".into()),
            name: Identifier("F".into()),
            type_params: vec![TypeTag::U8; MAX_TYPE_NODE_COUNT as usize],
        };
        let tt = TypeTag::Struct(Box::new(st));
        let bytes = bcs::to_bytes(&tt).unwrap();
        let err = bcs::from_bytes::<TypeTag>(&bytes).unwrap_err();
        assert!(
            err.to_string().contains("node count"),
            "expected node-count error, got: {err}"
        );
    }

    #[test]
    fn bcs_type_tag_at_exact_max_node_count_is_accepted() {
        // `MAX - 1` primitives + outer Struct = exactly MAX nodes.
        let st = StructTag {
            address: Address::from_str("0x1").unwrap(),
            module: Identifier("M".into()),
            name: Identifier("F".into()),
            type_params: vec![TypeTag::U8; (MAX_TYPE_NODE_COUNT - 1) as usize],
        };
        let tt = TypeTag::Struct(Box::new(st));
        let bytes = bcs::to_bytes(&tt).unwrap();
        let parsed: TypeTag = bcs::from_bytes(&bytes).unwrap();
        assert_eq!(parsed, tt);
    }

    // `StructTag::Deserialize` counts the outer StructTag as one node so
    // its boundary matches `TypeTag::Deserialize` for the same shape.
    #[test]
    fn bcs_struct_tag_at_max_node_count_is_rejected() {
        let st = StructTag {
            address: Address::from_str("0x1").unwrap(),
            module: Identifier("M".into()),
            name: Identifier("F".into()),
            type_params: vec![TypeTag::U8; MAX_TYPE_NODE_COUNT as usize],
        };
        let bytes = bcs::to_bytes(&st).unwrap();
        let err = bcs::from_bytes::<StructTag>(&bytes).unwrap_err();
        assert!(
            err.to_string().contains("node count"),
            "expected node-count error, got: {err}"
        );
    }

    #[test]
    fn bcs_struct_tag_at_exact_max_node_count_is_accepted() {
        // `MAX - 1` u8 type_params + the outer StructTag = exactly MAX
        // nodes.
        let st = StructTag {
            address: Address::from_str("0x1").unwrap(),
            module: Identifier("M".into()),
            name: Identifier("F".into()),
            type_params: vec![TypeTag::U8; (MAX_TYPE_NODE_COUNT - 1) as usize],
        };
        let bytes = bcs::to_bytes(&st).unwrap();
        let parsed: StructTag = bcs::from_bytes(&bytes).unwrap();
        assert_eq!(parsed, st);
    }

    // Regression test: the visitor used to call
    // `Vec::with_capacity(seq.size_hint().unwrap_or(0))` before any
    // per-element bound was enforced, so an attacker-supplied uleb32
    // type_params length of `0x7fffffff` could request a ~32 GB
    // allocation from a 42-byte BCS payload. The visitor must now clamp
    // the hint by the remaining node-count budget so this payload fails
    // through normal short-input handling instead of aborting on
    // allocation failure. If the regression returns, this test will
    // OOM-abort the process before it can fail cleanly.
    #[test]
    fn bcs_struct_tag_huge_type_params_length_does_not_overallocate() {
        // Variant tag 0x07 (Struct), 32-byte zero address, "a" as
        // module, "b" as name, then a uleb128 of 0x7fffffff for the
        // length of `type_params`.
        let mut bytes = vec![SerializedTypeTagVariant::Struct as u8];
        bytes.extend_from_slice(&[0u8; Address::LENGTH]);
        bytes.extend_from_slice(&[0x01, b'a']);
        bytes.extend_from_slice(&[0x01, b'b']);
        bytes.extend_from_slice(&[0xff, 0xff, 0xff, 0xff, 0x07]);

        assert!(bcs::from_bytes::<TypeTag>(&bytes).is_err());
    }
}

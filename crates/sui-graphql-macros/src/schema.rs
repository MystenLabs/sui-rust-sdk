//! GraphQL schema parsing and type lookup.

use graphql_parser::schema::Definition;
use graphql_parser::schema::TypeDefinition;
use graphql_parser::schema::{self as gql};
use std::collections::HashMap;
use std::sync::LazyLock;

/// The embedded Sui GraphQL schema.
const SCHEMA_SDL: &str = include_str!("../schema/sui.graphql");

/// Parsed and indexed schema, cached for reuse across macro invocations.
static SCHEMA: LazyLock<Result<Schema, String>> =
    LazyLock::new(|| Schema::parse(SCHEMA_SDL).map_err(|e| format!("Failed to parse schema: {e}")));

/// A parsed GraphQL schema with type lookup.
#[derive(Debug)]
pub struct Schema {
    types: HashMap<String, TypeInfo>,
}

/// Information about a GraphQL type.
#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub name: String,
    pub fields: HashMap<String, FieldInfo>,
}

/// Information about a field on a type.
#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub type_name: String,
    pub is_list: bool,
}

impl Schema {
    /// Load the embedded schema.
    pub fn load() -> Result<&'static Schema, syn::Error> {
        SCHEMA
            .as_ref()
            .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e.clone()))
    }

    /// Parse a GraphQL SDL schema.
    fn parse(sdl: &str) -> Result<Self, graphql_parser::schema::ParseError> {
        let doc = gql::parse_schema::<String>(sdl)?;
        let mut types = HashMap::new();

        for def in doc.definitions {
            if let Definition::TypeDefinition(type_def) = def {
                let type_info = Self::parse_type_definition(type_def);
                types.insert(type_info.name.clone(), type_info);
            }
        }

        Ok(Schema { types })
    }

    fn parse_type_definition(def: TypeDefinition<String>) -> TypeInfo {
        match def {
            TypeDefinition::Object(obj) => TypeInfo {
                name: obj.name.clone(),
                fields: obj
                    .fields
                    .iter()
                    .map(|f| {
                        let info = Self::parse_field(f);
                        (info.name.clone(), info)
                    })
                    .collect(),
            },
            TypeDefinition::Interface(i) => TypeInfo {
                name: i.name.clone(),
                fields: i
                    .fields
                    .iter()
                    .map(|f| {
                        let info = Self::parse_field(f);
                        (info.name.clone(), info)
                    })
                    .collect(),
            },
            TypeDefinition::Scalar(s) => TypeInfo {
                name: s.name.clone(),
                fields: HashMap::new(),
            },
            TypeDefinition::Enum(e) => TypeInfo {
                name: e.name.clone(),
                fields: HashMap::new(),
            },
            TypeDefinition::InputObject(io) => TypeInfo {
                name: io.name.clone(),
                fields: HashMap::new(),
            },
            TypeDefinition::Union(u) => TypeInfo {
                name: u.name.clone(),
                fields: HashMap::new(),
            },
        }
    }

    fn parse_field(field: &gql::Field<String>) -> FieldInfo {
        let (type_name, is_list) = Self::parse_type(&field.field_type);
        FieldInfo {
            name: field.name.clone(),
            type_name,
            is_list,
        }
    }

    /// Parse a GraphQL type, extracting the base type name and list status.
    fn parse_type(ty: &gql::Type<String>) -> (String, bool) {
        match ty {
            gql::Type::NamedType(name) => (name.clone(), false),
            gql::Type::NonNullType(inner) => Self::parse_type(inner),
            gql::Type::ListType(inner) => {
                let (name, _) = Self::parse_type(inner);
                (name, true)
            }
        }
    }

    /// Look up a field on a type.
    pub fn get_field(&self, type_name: &str, field_name: &str) -> Option<&FieldInfo> {
        self.types.get(type_name)?.fields.get(field_name)
    }
}

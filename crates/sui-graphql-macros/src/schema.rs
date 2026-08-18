//! GraphQL schema parsing and type lookup.

use graphql_parser::schema as gql;
use graphql_parser::schema::Definition;
use graphql_parser::schema::TypeDefinition;
use std::collections::HashMap;
use std::sync::LazyLock;

/// The embedded Sui GraphQL schema.
pub(crate) const SCHEMA_SDL: &str = include_str!("../schema/sui.graphql");

/// Parsed and indexed schema, cached for reuse across macro invocations.
static SCHEMA: LazyLock<Result<Schema, String>> =
    LazyLock::new(|| Schema::parse(SCHEMA_SDL).map_err(|e| format!("Failed to parse schema: {e}")));

/// A parsed GraphQL schema with type lookup.
#[derive(Debug)]
pub struct Schema {
    types: HashMap<String, TypeInfo>,
}

/// Information about a GraphQL type.
#[derive(Debug)]
pub struct TypeInfo {
    pub name: String,
    pub fields: HashMap<String, FieldInfo>,
    /// For union types, the member type names. `None` for non-union types.
    pub union_types: Option<Vec<String>>,
    /// The names of the interfaces this type declares it implements.
    pub interfaces: Vec<String>,
}

/// Information about a field on a type.
#[derive(Debug)]
pub struct FieldInfo {
    pub name: String,
    pub type_name: String,
    pub is_list: bool,
}

impl Schema {
    /// Load the embedded Sui GraphQL schema.
    ///
    /// Returns an `Arc<Schema>` for cheap sharing without deep copying.
    pub fn load() -> Result<&'static Schema, syn::Error> {
        SCHEMA
            .as_ref()
            .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e.clone()))
    }

    /// Load a custom schema from SDL content.
    ///
    /// Used when `#[response(schema = "...")]` specifies a custom schema.
    pub fn from_sdl(sdl: &str) -> Result<Self, syn::Error> {
        Self::parse(sdl).map_err(|e| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Failed to parse custom schema: {e}"),
            )
        })
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
                interfaces: obj.implements_interfaces.clone(),
                fields: obj
                    .fields
                    .iter()
                    .map(|f| {
                        let info = Self::parse_field(f);
                        (info.name.clone(), info)
                    })
                    .collect(),
                union_types: None,
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
                interfaces: Vec::new(),
                union_types: None,
            },
            TypeDefinition::Scalar(s) => TypeInfo {
                name: s.name.clone(),
                fields: HashMap::new(),
                interfaces: Vec::new(),
                union_types: None,
            },
            TypeDefinition::Enum(e) => TypeInfo {
                name: e.name.clone(),
                fields: HashMap::new(),
                interfaces: Vec::new(),
                union_types: None,
            },
            TypeDefinition::InputObject(io) => TypeInfo {
                name: io.name.clone(),
                fields: HashMap::new(),
                interfaces: Vec::new(),
                union_types: None,
            },
            TypeDefinition::Union(u) => TypeInfo {
                name: u.name.clone(),
                fields: HashMap::new(),
                interfaces: Vec::new(),
                union_types: Some(u.types),
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
    pub fn field(&self, type_name: &str, field_name: &str) -> Option<&FieldInfo> {
        self.types.get(type_name)?.fields.get(field_name)
    }

    /// Get all field names for a type.
    pub fn field_names(&self, type_name: &str) -> Vec<&str> {
        self.types
            .get(type_name)
            .map(|t| t.fields.keys().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Check if a type exists in the schema.
    pub fn has_type(&self, type_name: &str) -> bool {
        self.types.contains_key(type_name)
    }

    /// Get all type names in the schema.
    pub fn type_names(&self) -> Vec<&str> {
        self.types.keys().map(|s| s.as_str()).collect()
    }

    /// Check if a type is a union type.
    pub fn is_union(&self, type_name: &str) -> bool {
        self.types
            .get(type_name)
            .is_some_and(|t| t.union_types.is_some())
    }

    /// The root types a projection may declare in order to be flattened into
    /// `type_name`, sorted and deduplicated.
    ///
    /// A flattened projection is extracted unconditionally, so its root must hold for
    /// every concrete type `type_name` could be: the type itself, any interface it
    /// implements, and any union it belongs to.
    pub fn flatten_roots(&self, type_name: &str) -> Vec<&str> {
        let Some(info) = self.types.get(type_name) else {
            return Vec::new();
        };

        let mut targets = vec![info.name.as_str()];
        targets.extend(info.interfaces.iter().map(String::as_str));
        targets.extend(self.types.values().filter_map(|t| {
            let members = t.union_types.as_ref()?;
            members
                .iter()
                .any(|m| m == type_name)
                .then_some(t.name.as_str())
        }));
        targets.sort_unstable();
        targets.dedup();
        targets
    }

    /// Get the member type names of a union.
    pub fn union_types(&self, type_name: &str) -> Vec<&str> {
        self.types
            .get(type_name)
            .and_then(|t| t.union_types.as_ref())
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_type() {
        let schema = Schema::load().unwrap();
        // Standard GraphQL root types
        assert!(schema.has_type("Query"));
        assert!(schema.has_type("Mutation"));
        // Common Sui types
        assert!(schema.has_type("Object"));
        assert!(schema.has_type("DynamicField"));
        assert!(schema.has_type("MoveObject"));
        // Non-existent types
        assert!(!schema.has_type("NonExistent"));
        assert!(!schema.has_type("query")); // Case-sensitive
    }

    #[test]
    fn test_type_names() {
        let schema = Schema::load().unwrap();
        let type_names = schema.type_names();
        // Should contain standard types
        assert!(type_names.contains(&"Query"));
        assert!(type_names.contains(&"Mutation"));
        assert!(type_names.contains(&"Object"));
    }

    fn test_schema() -> Schema {
        let sdl = include_str!("../tests/test_schema.graphql");
        Schema::from_sdl(sdl).unwrap()
    }

    #[test]
    fn test_flatten_roots_object_includes_its_interfaces() {
        let schema = Schema::load().unwrap();

        // type Object implements Node & IAddressable & IObject
        assert_eq!(
            schema.flatten_roots("Object"),
            vec!["IAddressable", "IObject", "Node", "Object"]
        );

        // type DynamicField implements Node & IAddressable & IMoveObject & IObject
        assert_eq!(
            schema.flatten_roots("DynamicField"),
            vec![
                "DynamicField",
                "IAddressable",
                "IMoveObject",
                "IObject",
                "Node"
            ]
        );
    }

    #[test]
    fn test_flatten_roots_includes_containing_unions() {
        let schema = Schema::load().unwrap();

        // union DynamicFieldValue = MoveObject | MoveValue
        let roots = schema.flatten_roots("MoveObject");
        assert!(roots.contains(&"DynamicFieldValue"));
        assert!(roots.contains(&"MoveObject"));
        assert!(roots.contains(&"IObject"));

        // A type in no union has no union roots.
        assert!(!schema.flatten_roots("Epoch").contains(&"DynamicFieldValue"));
    }

    #[test]
    fn test_flatten_roots_of_root_and_interface_types() {
        let schema = Schema::load().unwrap();

        // Query implements nothing and belongs to no union.
        assert_eq!(schema.flatten_roots("Query"), vec!["Query"]);

        // An interface is only a root for itself: no Sui interface implements another,
        // and a union cannot have an interface as a member.
        assert_eq!(schema.flatten_roots("IObject"), vec!["IObject"]);
    }

    #[test]
    fn test_flatten_roots_is_sorted_and_deduplicated() {
        let schema = Schema::load().unwrap();

        for type_name in ["Object", "MoveObject", "DynamicField", "Query"] {
            let roots = schema.flatten_roots(type_name);
            let mut expected = roots.clone();
            expected.sort_unstable();
            expected.dedup();
            assert_eq!(
                roots, expected,
                "flatten_roots({type_name}) is not normalized"
            );
        }
    }

    #[test]
    fn test_flatten_roots_of_unknown_type_is_empty() {
        let schema = Schema::load().unwrap();
        assert!(schema.flatten_roots("NonExistent").is_empty());
    }

    #[test]
    fn test_union_types() {
        let schema = test_schema();

        assert!(schema.is_union("DynamicFieldValue"));
        let members = schema.union_types("DynamicFieldValue");
        assert!(members.contains(&"MoveObject"));
        assert!(members.contains(&"MoveValue"));

        // Non-union types
        assert!(!schema.is_union("Object"));
        assert!(schema.union_types("Object").is_empty());

        // Non-existent type
        assert!(!schema.is_union("NonExistent"));
        assert!(schema.union_types("NonExistent").is_empty());
    }
}

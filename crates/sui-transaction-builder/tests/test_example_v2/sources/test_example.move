module test_example::test_example {
use std::string::{Self, String};

    public struct Object has key {
        id: UID, // required
        name: String,
    }

    /// Creates a new Object with a Unique ID
    public fun new(name: String, ctx: &mut TxContext): Object {
        Object {
            id: object::new(ctx), // creates a new UID
            name,
        }
    }

    public fun default_name(ctx: &mut TxContext): Object {
        let default: String = string::utf8(b"default");
        Object {
            id: object::new(ctx),
            name: default,
        }
    }
}


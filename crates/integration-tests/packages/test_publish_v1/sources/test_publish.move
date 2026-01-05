module test_publish::test_publish;

use std::string::String;

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

// public fun default_name(ctx: &mut TxContext): Object {
//     let default: String = string::utf8(b"default");
//     Object {
//         id: object::new(ctx),
//         name: default,
//     }
// }

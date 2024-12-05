module test_example::test_example {
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
}


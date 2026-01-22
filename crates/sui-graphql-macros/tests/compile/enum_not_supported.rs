use sui_graphql_macros::Response;

#[derive(Response)]
enum BadEnum {
    Variant1,
    Variant2,
}

fn main() {}

use sui_sdk_types::Address;

use super::*;

impl LookupNameRequest {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            name: Some(name.into()),
        }
    }
}

impl LookupNameResponse {
    pub fn new(record: NameRecord) -> Self {
        Self {
            record: Some(record),
        }
    }
}

impl ReverseLookupNameRequest {
    pub fn new(address: &Address) -> Self {
        Self {
            address: Some(address.to_string()),
        }
    }
}

impl ReverseLookupNameResponse {
    pub fn new(record: NameRecord) -> Self {
        Self {
            record: Some(record),
        }
    }
}

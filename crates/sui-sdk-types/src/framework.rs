//! Rust definitions of move/sui framework types.

use super::Address;
use super::Object;
use super::TypeTag;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Coin<'a> {
    coin_type: Cow<'a, TypeTag>,
    id: Address,
    balance: u64,
}

impl<'a> Coin<'a> {
    pub fn coin_type(&self) -> &TypeTag {
        &self.coin_type
    }

    pub fn id(&self) -> &Address {
        &self.id
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }

    pub fn try_from_object(object: &'a Object) -> Option<Self> {
        match &object.data {
            super::ObjectData::Struct(move_struct) => {
                let coin_type = move_struct.type_.is_coin()?;

                let contents = &move_struct.contents;
                if contents.len() != Address::LENGTH + std::mem::size_of::<u64>() {
                    return None;
                }

                let id = Address::new((&contents[..Address::LENGTH]).try_into().unwrap());
                let balance =
                    u64::from_le_bytes((&contents[Address::LENGTH..]).try_into().unwrap());

                Some(Self {
                    coin_type: Cow::Borrowed(coin_type),
                    id,
                    balance,
                })
            }
            _ => None, // package
        }
    }

    pub fn into_owned(self) -> Coin<'static> {
        Coin {
            coin_type: Cow::Owned(self.coin_type.into_owned()),
            id: self.id,
            balance: self.balance,
        }
    }
}

//! Rust definitions of move/sui framework types.

use std::borrow::Cow;

use super::Object;
use super::ObjectId;
use super::TypeTag;

#[derive(Debug, Clone)]
pub struct Coin {
    coin_type: Cow<'static, TypeTag>,
    id: ObjectId,
    balance: u64,
}

impl Coin {
    pub fn coin_type(&self) -> &TypeTag {
        &self.coin_type
    }

    pub fn id(&self) -> &ObjectId {
        &self.id
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }

    pub fn try_from_object(object: &Object) -> Option<Self> {
        match &object.data {
            super::ObjectData::Struct(move_struct) => {
                let coin_type = move_struct.type_.is_coin()?;

                let contents = &move_struct.contents;
                if contents.len() != ObjectId::LENGTH + std::mem::size_of::<u64>() {
                    return None;
                }

                let id = ObjectId::new((&contents[..ObjectId::LENGTH]).try_into().unwrap());
                let balance =
                    u64::from_le_bytes((&contents[ObjectId::LENGTH..]).try_into().unwrap());

                Some(Self {
                    coin_type: Cow::Owned(coin_type.clone()),
                    id,
                    balance,
                })
            }
            _ => None, // package
        }
    }
}

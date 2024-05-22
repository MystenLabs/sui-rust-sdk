//! Rust definitions of move/sui framework types.

use super::{Object, ObjectId, TypeTag};

#[derive(Debug, Clone)]
pub struct Coin<'a> {
    coin_type: &'a TypeTag,
    id: ObjectId,
    balance: u64,
}

impl<'a> Coin<'a> {
    pub fn coin_type(&self) -> &'a TypeTag {
        self.coin_type
    }

    pub fn id(&self) -> &ObjectId {
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
                if contents.len() != ObjectId::LENGTH + std::mem::size_of::<u64>() {
                    return None;
                }

                let id = ObjectId::new((&contents[..ObjectId::LENGTH]).try_into().unwrap());
                let balance =
                    u64::from_le_bytes((&contents[ObjectId::LENGTH..]).try_into().unwrap());

                Some(Self {
                    coin_type,
                    id,
                    balance,
                })
            }
            _ => None, // package
        }
    }
}

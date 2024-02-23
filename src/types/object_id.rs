use super::Address;

#[derive(Eq, PartialEq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct ObjectId(Address);

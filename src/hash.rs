use crate::types::Digest;

use blake2::Digest as DigestTrait;

type Blake2b256 = blake2::Blake2b<blake2::digest::consts::U32>;

#[derive(Debug, Default)]
pub struct Hasher(Blake2b256);

impl Hasher {
    pub fn new() -> Self {
        Self(Blake2b256::new())
    }

    pub fn update<T: AsRef<[u8]>>(&mut self, data: T) {
        self.0.update(data)
    }

    /// Retrieve result and consume hasher instance.
    pub fn finalize(self) -> Digest {
        let mut buf = [0; Digest::LENGTH];
        let result = self.0.finalize();

        buf.copy_from_slice(result.as_slice());

        Digest::new(buf)
    }

    pub fn digest<T: AsRef<[u8]>>(data: T) -> Digest {
        let mut hasher = Self::new();
        hasher.update(data);
        hasher.finalize()
    }
}

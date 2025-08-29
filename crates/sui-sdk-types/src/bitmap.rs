/// A compressed bitmap using the [Roaring bitmap compression scheme](https://roaringbitmap.org/).
///
/// # BCS
///
/// The BCS serialized form for this type is defined by the following ABNF:
///
/// ```text
/// roaring-bitmap = bytes  ; where the contents of the bytes are valid
///                         ; according to the serialized spec for
///                         ; roaring bitmaps
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Bitmap(roaring::RoaringBitmap);

impl Bitmap {
    /// Creates an empty `Bitmap`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sui_sdk_types::Bitmap;
    /// let bitmap = Bitmap::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of distinct integers added to the set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sui_sdk_types::Bitmap;
    ///
    /// let mut bitmap = Bitmap::new();
    /// assert_eq!(bitmap.len(), 0);
    ///
    /// bitmap.insert(3);
    /// assert_eq!(bitmap.len(), 1);
    ///
    /// bitmap.insert(3);
    /// bitmap.insert(4);
    /// assert_eq!(bitmap.len(), 2);
    /// ```
    pub fn len(&self) -> u64 {
        self.0.len()
    }

    /// Clears all integers in this set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sui_sdk_types::Bitmap;
    ///
    /// let mut bitmap = Bitmap::new();
    /// bitmap.insert(1);
    /// assert_eq!(bitmap.contains(1), true);
    /// bitmap.clear();
    /// assert_eq!(bitmap.contains(1), false);
    /// ```
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Returns `true` if there are no integers in this set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sui_sdk_types::Bitmap;
    ///
    /// let mut bitmap = Bitmap::new();
    /// assert_eq!(bitmap.is_empty(), true);
    ///
    /// bitmap.insert(3);
    /// assert_eq!(bitmap.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Adds a value to the set.
    ///
    /// Returns whether the value was absent from the set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sui_sdk_types::Bitmap;
    ///
    /// let mut bitmap = Bitmap::new();
    /// assert_eq!(bitmap.insert(3), true);
    /// assert_eq!(bitmap.insert(3), false);
    /// assert_eq!(bitmap.contains(3), true);
    /// ```
    pub fn insert(&mut self, value: u32) -> bool {
        self.0.insert(value)
    }

    /// Inserts a range of values.
    /// Returns the number of inserted values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sui_sdk_types::Bitmap;
    ///
    /// let mut bitmap = Bitmap::new();
    /// bitmap.insert_range(2..4);
    /// assert!(bitmap.contains(2));
    /// assert!(bitmap.contains(3));
    /// assert!(!bitmap.contains(4));
    /// ```
    pub fn insert_range<R>(&mut self, range: R) -> u64
    where
        R: std::ops::RangeBounds<u32>,
    {
        self.0.insert_range(range)
    }

    /// Removes a value from the set. Returns `true` if the value was present in the set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sui_sdk_types::Bitmap;
    ///
    /// let mut bitmap = Bitmap::new();
    /// bitmap.insert(3);
    /// assert_eq!(bitmap.remove(3), true);
    /// assert_eq!(bitmap.remove(3), false);
    /// assert_eq!(bitmap.contains(3), false);
    /// ```
    pub fn remove(&mut self, value: u32) -> bool {
        self.0.remove(value)
    }

    /// Returns `true` if this set contains the specified integer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sui_sdk_types::Bitmap;
    ///
    /// let mut bitmap = Bitmap::new();
    /// bitmap.insert(1);
    /// assert_eq!(bitmap.contains(0), false);
    /// assert_eq!(bitmap.contains(1), true);
    /// assert_eq!(bitmap.contains(100), false);
    /// ```
    pub fn contains(&self, value: u32) -> bool {
        self.0.contains(value)
    }

    /// Iterator over each value stored in the Bitmap, guarantees values are ordered by value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use core::iter::FromIterator;
    /// use sui_sdk_types::Bitmap;
    ///
    /// let bitmap = (1..3).collect::<Bitmap>();
    /// let mut iter = bitmap.iter();
    ///
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = u32> + use<'_> {
        self.0.iter()
    }

    /// Deserialize a bitmap into memory from [the standard Roaring on-disk
    /// format][format]. This is compatible with the official C/C++, Java and
    /// Go implementations. This method checks that all of the internal values
    /// are valid.
    ///
    /// [format]: https://github.com/RoaringBitmap/RoaringFormatSpec
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sui_sdk_types::Bitmap;
    ///
    /// let bitmap1: Bitmap = (1..4).collect();
    /// let mut bytes = vec![];
    /// bitmap1.serialize_into(&mut bytes).unwrap();
    /// let bitmap2 = Bitmap::deserialize_from(&bytes[..]).unwrap();
    ///
    /// assert_eq!(bitmap1, bitmap2);
    /// ```
    #[cfg(feature = "serde")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
    pub fn deserialize_from<R: std::io::Read>(reader: R) -> std::io::Result<Self> {
        let rb = roaring::RoaringBitmap::deserialize_from(reader)?;

        // Need to explicilty build by iterating the read bitmap to enforce uniquness since the
        // deserialization path doesn't guarantee it.
        let mut bitmap = Self::new();
        bitmap.extend(rb);
        Ok(bitmap)
    }

    /// Serialize this bitmap into [the standard Roaring on-disk format][format].
    /// This is compatible with the official C/C++, Java and Go implementations.
    ///
    /// [format]: https://github.com/RoaringBitmap/RoaringFormatSpec
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sui_sdk_types::Bitmap;
    ///
    /// let bitmap1: Bitmap = (1..4).collect();
    /// let mut bytes = vec![];
    /// bitmap1.serialize_into(&mut bytes).unwrap();
    /// let bitmap2 = Bitmap::deserialize_from(&bytes[..]).unwrap();
    ///
    /// assert_eq!(bitmap1, bitmap2);
    /// ```
    #[cfg(feature = "serde")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
    pub fn serialize_into<W: std::io::Write>(&self, writer: W) -> std::io::Result<()> {
        self.0.serialize_into(writer)
    }
}

impl FromIterator<u32> for Bitmap {
    fn from_iter<I: IntoIterator<Item = u32>>(iterator: I) -> Bitmap {
        let mut bitmap = Self::new();
        bitmap.extend(iterator);
        bitmap
    }
}

impl<'a> FromIterator<&'a u32> for Bitmap {
    fn from_iter<I: IntoIterator<Item = &'a u32>>(iterator: I) -> Bitmap {
        let mut bitmap = Self::new();
        bitmap.extend(iterator);
        bitmap
    }
}

impl Extend<u32> for Bitmap {
    fn extend<I: IntoIterator<Item = u32>>(&mut self, values: I) {
        self.0.extend(values)
    }
}

impl<'a> Extend<&'a u32> for Bitmap {
    fn extend<I: IntoIterator<Item = &'a u32>>(&mut self, values: I) {
        self.extend(values.into_iter().copied());
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl serde::Serialize for Bitmap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde_with::SerializeAs;

        let mut bytes = vec![];

        self.serialize_into(&mut bytes)
            .map_err(serde::ser::Error::custom)?;

        if serializer.is_human_readable() {
            let b64 = <base64ct::Base64 as base64ct::Encoding>::encode_string(&bytes);
            serde::Serialize::serialize(&b64, serializer)
        } else {
            serde_with::Bytes::serialize_as(&bytes, serializer)
        }
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
impl<'de> serde::Deserialize<'de> for Bitmap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde_with::DeserializeAs;

        if deserializer.is_human_readable() {
            let b64: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
            let bytes = <base64ct::Base64 as base64ct::Encoding>::decode_vec(&b64)
                .map_err(serde::de::Error::custom)?;
            Self::deserialize_from(&bytes[..]).map_err(serde::de::Error::custom)
        } else {
            let bytes: std::borrow::Cow<'de, [u8]> =
                serde_with::Bytes::deserialize_as(deserializer)?;
            Self::deserialize_from(&bytes[..]).map_err(serde::de::Error::custom)
        }
    }
}

#[cfg(feature = "proptest")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "proptest")))]
impl proptest::arbitrary::Arbitrary for Bitmap {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::collection::vec;
        use proptest::prelude::*;

        vec(any::<u32>(), 0..32).prop_map(Self::from_iter).boxed()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use base64ct::Encoding;

    #[test]
    fn test_unique_deserialize() {
        let raw = "OjAAAAoAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWAAAAFoAAABcAAAAXgAAAGAAAABiAAAAZAAAAGYAAABoAAAAagAAAAEAAQABAAEAAQABAAEAAQABAAEA";
        let bytes = base64ct::Base64::decode_vec(raw).unwrap();

        let rb = roaring::RoaringBitmap::deserialize_from(&bytes[..]).unwrap();
        assert_eq!(rb.len(), 10);
        let bitmap_values: Vec<u32> = rb.iter().collect();
        assert_eq!(bitmap_values, vec![1; 10]);

        let bitmap = Bitmap::deserialize_from(&bytes[..]).unwrap();
        assert_eq!(bitmap.len(), 1);
        let bitmap_values: Vec<u32> = bitmap.iter().collect();
        assert_eq!(bitmap_values, vec![1]);
    }
}

/// [External doc](https://v2.docs.influxdata.com/v2.0/reference/syntax/line-protocol/#tag-set)
use super::{
    error::Error,
    escape,
    name_restriction::{prevent_key, prevent_tag_value},
};
use derive_more::{Deref, Display};

use std::borrow::Borrow;
use std::convert::{TryFrom, TryInto};

#[cfg(feature = "serde")]
use serde1::{de::Error as DeserializeError, Deserialize, Deserializer, Serialize, Serializer};

/// The key part of the key-value pair that makes up a tag.
///
/// Tag keys are strings and they store metadata.
/// Tag keys are indexed so queries on tag keys are performant.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Display, Deref)]
pub struct TagKey(String);

impl TagKey {
    /// Create Tag with check for correctness.
    pub fn new(s: impl Into<String>) -> Result<Self, Error> {
        let s = s.into();
        prevent_key(&s)?;
        Ok(Self(s))
    }
}

#[cfg(feature = "serde")]
impl Serialize for TagKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for TagKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        TagKey::new(s).map_err(D::Error::custom)
    }
}

impl TryFrom<String> for TagKey {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for TagKey {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Borrow<str> for TagKey {
    #[inline]
    fn borrow(&self) -> &str {
        self.0.borrow()
    }
}

impl AsRef<str> for TagKey {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Display, Deref)]
pub struct TagValue(String);

#[cfg(feature = "serde")]
impl Serialize for TagValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for TagValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        TagValue::new(s).map_err(D::Error::custom)
    }
}

impl TagValue {
    pub fn new(s: impl Into<String>) -> Result<Self, Error> {
        let s = s.into();
        prevent_tag_value(&s)?;
        Ok(Self(s))
    }
}

impl TryFrom<String> for TagValue {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for TagValue {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

///The key-value pair in the InfluxDB data structure that records metadata.
///
/// Tags are an optional part of the data structure, but they are useful for storing commonly-queried metadata;
/// tags are indexed so queries on tags are performant.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tag {
    key: TagKey,
    value: TagValue,
}

impl<K, V> TryFrom<(K, V)> for Tag
where
    K: TryInto<TagKey>,
    K::Error: Into<Error>,
    V: TryInto<TagValue>,
    V::Error: Into<Error>,
{
    type Error = Error;
    fn try_from(v: (K, V)) -> Result<Self, Self::Error> {
        let (into_key, into_value) = v;
        let key = into_key.try_into().map_err(|x| x.into())?;
        let value = into_value.try_into().map_err(|x| x.into())?;
        Ok(Self { key, value })
    }
}

impl Tag {
    pub fn new(key: String, value: String) -> Result<Self, Error> {
        //TODO check key, value are correct
        Ok(Self {
            key: TryFrom::try_from(key)?,
            value: TryFrom::try_from(value)?,
        })
    }

    pub(crate) fn to_text(&self) -> String {
        let escaped_key = escape::tag_key(&self.key);
        let escaped_value = escape::tag_value(&self.value);
        format!("{}={}", escaped_key, escaped_value)
    }
}

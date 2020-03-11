/// [External doc](https://v2.docs.influxdata.com/v2.0/reference/syntax/line-protocol/#tag-set)
use super::{
    error::Error,
    escape,
    name_restriction::{prevent_key, prevent_tag_value},
};
use derive_more::{Deref, Display};

use std::borrow::Borrow;
use std::convert::TryFrom;

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

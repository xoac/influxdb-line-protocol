use super::{
    error::Error,
    escape,
    name_restriction::{prevent_filed_value_string, prevent_key},
};
use derive_more::{Deref, Display, From};
use ordered_float::NotNan;

use std::{
    borrow::Borrow,
    convert::{TryFrom, TryInto},
};

#[cfg(feature = "serde")]
use serde1::{
    de::{self, Error as DeserializeError},
    Deserialize, Deserializer, Serialize, Serializer,
};

///The key part of the key-value pair that makes up a field.
///
///Field keys are strings and they store metadata.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Display, Deref)]
pub struct FieldKey(String);

#[cfg(feature = "serde")]
impl Serialize for FieldKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for FieldKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FieldKey::new(s).map_err(D::Error::custom)
    }
}

impl FieldKey {
    /// Create FiledKey with check for correctness.
    pub fn new(s: impl Into<String>) -> Result<Self, Error> {
        let s = s.into();
        prevent_key(&s)?;
        Ok(Self(s))
    }
}

impl TryFrom<String> for FieldKey {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for FieldKey {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Borrow<str> for FieldKey {
    #[inline]
    fn borrow(&self) -> &str {
        self.0.borrow()
    }
}

impl AsRef<str> for FieldKey {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

///The value part of the key-value pair that makes up a [`Field`].
///
///Field values are the actual data; they can be strings, floats, integers, or booleans.
#[derive(Debug, PartialEq, Eq, Clone, From)]
pub enum FieldValue {
    #[from(ignore)]
    String(String), //FIXME we shouldn't allow values bigger than 64KB
    UInteger(u64),
    Integer(i64),
    Float(NotNan<f64>), // float in influxdb can't be NaN
    Boolean(bool),
}

#[cfg(feature = "serde")]
impl Serialize for FieldValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            FieldValue::String(s) => s.serialize(serializer),
            FieldValue::UInteger(v) => v.serialize(serializer),
            FieldValue::Integer(v) => v.serialize(serializer),
            FieldValue::Float(v) => v.serialize(serializer),
            FieldValue::Boolean(v) => v.serialize(serializer),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for FieldValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::fmt;

        struct FieldVisitor;
        impl<'de> de::Visitor<'de> for FieldVisitor {
            type Value = FieldValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("influxDB field value (enum FieldValue)")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(FieldValue::from(value))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                FieldValue::try_from(v).map_err(E::custom)
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(FieldValue::Integer(v))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                NotNan::try_from(v).map_err(E::custom).map(FieldValue::from)
            }
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(FieldValue::Boolean(v))
            }
        }

        deserializer.deserialize_any(FieldVisitor)
    }
}

impl TryFrom<String> for FieldValue {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        prevent_filed_value_string(&value)?;
        Ok(FieldValue::String(value))
    }
}

impl TryFrom<&str> for FieldValue {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        FieldValue::try_from(String::from(value))
    }
}

impl TryFrom<f64> for FieldValue {
    type Error = Error;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        NotNan::new(value)
            .map(FieldValue::from)
            .map_err(Error::from)
    }
}

impl TryFrom<f32> for FieldValue {
    type Error = Error;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        f64::from(value).try_into()
    }
}

impl FieldValue {
    // convert self to string according to docs: https://v2.docs.influxdata.com/v2.0/reference/syntax/line-protocol/
    fn to_text(&self) -> String {
        match self {
            FieldValue::String(s) => format!(r#""{}""#, escape::field_value(&s)),
            FieldValue::UInteger(i) => i.to_string() + "u",
            FieldValue::Integer(i) => i.to_string() + "i",
            FieldValue::Float(f) => f.to_string(),
            FieldValue::Boolean(b) => b.to_string(),
        }
    }
}

///The key-value pair in an InfluxDB data structure that records metadata and the actual data value.
///
///Fields are required in InfluxDB data structures and they are not indexed - queries on field values
///scan all points that match the specified time range and, as a result, are not performant relative to [`tags'].
///
///[`tags`]:super::Tag
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Field {
    key: FieldKey,
    value: FieldValue,
}

//TODO implement Serialize / Deserialize

impl<K, V> TryFrom<(K, V)> for Field
where
    K: TryInto<FieldKey>,
    K::Error: Into<Error>,
    V: TryInto<FieldValue>,
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

impl Field {
    pub fn new<V>(key: impl Into<String>, value: V) -> Result<Self, Error>
    where
        V: TryInto<FieldValue>,
        V::Error: Into<Error>,
    {
        let value: Result<FieldValue, Error> = value.try_into().map_err(|e| e.into());
        let value = value?;
        Ok(Self {
            key: FieldKey::new(key)?,
            value,
        })
    }

    pub(crate) fn to_text(&self) -> String {
        let key = escape::field_key(&self.key);
        format!("{}={}", key, self.value.to_text())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_field_value_string() {
        let fv1 = FieldValue::try_from("FieldValue").unwrap();
        assert_eq!(fv1.to_text(), r#""FieldValue""#);

        let fv2 = FieldValue::try_from("Contains=EqualSign").unwrap();
        assert_eq!(fv2.to_text(), r#""Contains=EqualSign""#);

        let fv3 = FieldValue::try_from(r#"This value contains spaces and " quote"#).unwrap();
        assert_eq!(
            fv3.to_text(),
            r#""This value contains spaces and \" quote""#
        );

        let fv4 = FieldValue::try_from(r#"All = " \ , escaped characters"#).unwrap();
        assert_eq!(fv4.to_text(), r#""All = \" \\ , escaped characters""#);
    }

    #[test]
    fn escape_field_value() {
        let fv: FieldValue = 64i64.into();
        assert_eq!(fv.to_text(), r#"64i"#);

        let fv: FieldValue = 64u64.into();
        assert_eq!(fv.to_text(), r#"64u"#);

        let fl = 64.4f64;
        let fv: FieldValue = NotNan::new(fl).unwrap().into();
        assert_eq!(fv.to_text(), fl.to_string());

        let fv: FieldValue = true.into();
        assert_eq!(fv.to_text(), r#"true"#);
    }

    #[test]
    fn escape_filed_set() {
        let fv = FieldValue::try_from(String::from(r#""\"#)).unwrap();
        let key = String::from(r#"" =,"#);
        let fs = Field::new(key, fv).unwrap();
        assert_eq!(fs.to_text(), r#""\ \=\,="\"\\""#);
    }

    #[test]
    fn try_into_for_field() {
        let _: Field = ("Some", "Value").try_into().unwrap();
        let _: Field = ("U", 45u64).try_into().unwrap();
        let _: Field = ("I", 45i64).try_into().unwrap();
        let _: Field = ("F", 44f64).try_into().unwrap();
        let _: Field = ("bool", true).try_into().unwrap();
    }

    #[test]
    fn try_from_for_field() {
        let _ = Field::try_from(("Some", "Value")).unwrap();
        let _ = Field::try_from(("U", 45u64)).unwrap();
        let _ = Field::try_from(("I", 45i64)).unwrap();
        let _ = Field::try_from(("F", 44f64)).unwrap();
        let _ = Field::try_from(("bool", true)).unwrap();
    }
}

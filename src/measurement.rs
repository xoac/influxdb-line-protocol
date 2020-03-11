use super::{error::Error, name_restriction::check_measurement};
use derive_more::{Deref, Display};
use std::{borrow::Borrow, convert::TryFrom};

///The part of the InfluxDB data structure that describes the data stored in the associated fields.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Display, Deref)]
pub struct Measurement(String);

impl Measurement {
    pub fn new(measurement: impl Into<String>) -> Result<Self, Error> {
        let measurement = measurement.into();
        check_measurement(&measurement)?;
        Ok(Measurement(measurement))
    }
}

impl TryFrom<String> for Measurement {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for Measurement {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Borrow<str> for Measurement {
    #[inline]
    fn borrow(&self) -> &str {
        self.0.borrow()
    }
}

impl AsRef<str> for Measurement {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

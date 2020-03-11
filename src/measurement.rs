use super::{error::Error, name_restriction::check_measurement};
use derive_more::{Deref, Display};
use std::borrow::Borrow;

///The part of the InfluxDB data structure that describes the data stored in the associated fields.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Display, Deref)]
pub struct Measurement(String);

impl Measurement {
    pub fn new(measurement: String) -> Result<Self, Error> {
        check_measurement(&measurement)?;
        Ok(Measurement(measurement))
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

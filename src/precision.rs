use serde1::{Serialize, Serializer};
use std::str::FromStr;

pub struct ParsePrecisionErr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Precision {
    Secs,
    Milli,
    Micro,
    Nanos,
}

#[cfg(feature = "serde")]
impl Serialize for Precision {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let as_str = self.to_string();
        serializer.serialize_str(&as_str)
    }
}

impl Default for Precision {
    fn default() -> Self {
        Self::Nanos
    }
}

impl ToString for Precision {
    fn to_string(&self) -> String {
        match self {
            Precision::Milli => "ms",
            Precision::Secs => "s",
            Precision::Micro => "us",
            Precision::Nanos => "ns",
        }
        .to_string()
    }
}

impl FromStr for Precision {
    type Err = ParsePrecisionErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "ms" => Precision::Milli,
            "s" => Precision::Secs,
            "us" => Precision::Micro,
            "ns" => Precision::Nanos,
            _ => return Err(ParsePrecisionErr),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precision_ord_test() {
        assert!(Precision::Nanos > Precision::Secs);
        assert!(Precision::Nanos > Precision::Milli);
        assert!(Precision::Nanos > Precision::Micro);
        assert!(Precision::Nanos == Precision::Nanos);
    }

    #[test]
    fn precision_ord_option_test() {
        assert!(Some(Precision::Nanos) > Some(Precision::Secs));
        assert!(Some(Precision::Secs) > None);
        assert!(!(Some(Precision::Secs) < None));
    }
}

use crate::Precision;

/// InfluxDB Timestamp
///
/// Values are counted from UNIX_EPOCH.
///
///[external source](https://v2.docs.influxdata.com/v2.0/write-data/#timestamp-precision)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Timestamp {
    Now,
    Nanos(i64),
    Micro(i64),
    Milli(i64),
    Secs(i64),
}

impl Timestamp {
    pub fn precision(self) -> Option<Precision> {
        match self {
            Self::Now => None,
            Self::Nanos(_) => Some(Precision::Nanos),
            Self::Micro(_) => Some(Precision::Micro),
            Self::Milli(_) => Some(Precision::Milli),
            Self::Secs(_) => Some(Precision::Secs),
        }
    }

    pub fn timestamp_precision_lossy(self, precision: Precision) -> Self {
        match precision {
            Precision::Secs => self.to_secs_lossy(),
            Precision::Milli => self.to_milli_lossy(),
            Precision::Micro => self.to_micro_lossy(),
            Precision::Nanos => self.to_nanos(),
        }
    }

    pub fn to_secs_lossy(self) -> Self {
        let r_type = Self::Secs;
        match self {
            Self::Now => self,
            Self::Nanos(v) => r_type(v / 10i64.pow(9)),
            Self::Micro(v) => r_type(v / 10i64.pow(6)),
            Self::Milli(v) => r_type(v / 10i64.pow(3)),
            Self::Secs(_) => self,
        }
    }

    pub fn to_milli_lossy(self) -> Self {
        let r_type = Self::Milli;
        match self {
            Self::Now => self,
            Self::Nanos(v) => r_type(v / 10i64.pow(6)),
            Self::Micro(v) => r_type(v / 10i64.pow(3)),
            Self::Milli(v) => r_type(v),
            Self::Secs(v) => r_type(v * 10i64.pow(3)),
        }
    }

    pub fn to_micro_lossy(self) -> Self {
        let r_type = Self::Micro;
        match self {
            Self::Now => self,
            Self::Nanos(v) => r_type(v / 10i64.pow(3)),
            Self::Micro(v) => r_type(v),
            Self::Milli(v) => r_type(v * 10i64.pow(3)),
            Self::Secs(v) => r_type(v * 10i64.pow(6)),
        }
    }

    pub fn to_nanos(self) -> Self {
        let r_type = Self::Nanos;
        match self {
            Self::Now => self,
            Self::Nanos(v) => r_type(v),
            Self::Micro(v) => r_type(v * 10i64.pow(3)),
            Self::Milli(v) => r_type(v * 10i64.pow(6)),
            Self::Secs(v) => r_type(v * 10i64.pow(9)),
        }
    }

    pub fn timestamp_nanos(self) -> Option<i64> {
        match self {
            Self::Now => None,
            Self::Nanos(v) => Some(v),
            Timestamp::Micro(v) => v.checked_mul(10i64.pow(3)),
            Timestamp::Milli(v) => v.checked_mul(10i64.pow(6)),
            Timestamp::Secs(v) => v.checked_mul(10i64.pow(9)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamp_as_nanos_test() {
        assert_eq!(Timestamp::Secs(1).timestamp_nanos(), Some(10i64.pow(9)));
        assert_eq!(
            Timestamp::Milli(10).timestamp_nanos(),
            Some(10 * 10i64.pow(6))
        );
        assert_eq!(
            Timestamp::Micro(10).timestamp_nanos(),
            Some(10 * 10i64.pow(3))
        );
        assert_eq!(
            Timestamp::Nanos(10).timestamp_nanos(),
            Some(10 * 10i64.pow(0))
        );
    }

    #[test]
    fn secons_as_nanos() {
        assert_eq!(
            Timestamp::Secs(1).to_nanos(),
            Timestamp::Nanos(10i64.pow(9))
        );
    }
}

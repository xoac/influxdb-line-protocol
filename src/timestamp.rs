use std::{convert::TryFrom, time::SystemTime};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Timestamp {
    Now,
    Nanos(i64),
}

impl Timestamp {
    pub fn timestamp_nanos(self) -> Option<i64> {
        match self {
            Self::Now => None,
            Self::Nanos(v) => Some(v),
        }
    }
}

impl From<SystemTime> for Timestamp {
    fn from(v: SystemTime) -> Self {
        let nanos =
            i64::try_from(v.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos()).unwrap();
        Timestamp::Nanos(nanos)
    }
}

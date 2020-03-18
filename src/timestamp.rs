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

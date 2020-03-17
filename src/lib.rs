#![cfg_attr(feature = "nightly", feature(test))]

//! This crate aims to handle [influx db line protocol reference]. There is also
//! [another documentation in second version of InfluxDB](https://v2.docs.influxdata.com/v2.0/reference/syntax/line-protocol/).
//!
//! This crate aims as much as possible using the same terminology as influx db use.
//! If you have found any variance - it's bug. [InfuxDB glossary] can be use as good reference.
//!
//!
//! [InfuxDB glossary]:https://docs.influxdata.com/influxdb/v1.7/concepts/glossary/
//! [influx db line protocol reference]:https://docs.influxdata.com/influxdb/v1.7/concepts/glossary/
//!

mod escape;

mod batch;
pub mod error;
pub mod field;
mod measurement;
mod name_restriction;
mod point;
pub mod tag;
mod timestamp;

pub use batch::Batch;
pub use field::{Field, FieldKey, FieldValue};
pub use measurement::Measurement;
pub use point::Point;
pub use tag::{Tag, TagKey, TagValue};
pub use timestamp::Timestamp;

pub type FiledSet = Vec<Field>;
pub type TagSet = Vec<Tag>;

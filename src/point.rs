use super::{error::Error, escape, Field, FiledSet, Measurement, Tag, TagSet, Timestamp};

/// Represents a single data record
///
/// Each point:
/// - has a measurement, a tag set, a field key, a field value, and a timestamp;
/// - is uniquely identified by its series and timestamp.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Point {
    measurment: Measurement,
    tag_set: Vec<Tag>,
    field_set: Vec<Field>,
    timestamp: Timestamp,
}

impl Point {
    pub fn builder(measurment: impl Into<String>) -> PointBuilder {
        PointBuilder::new(measurment)
    }

    pub(crate) fn to_text(&self) -> String {
        let mut line = escape::measurement(&self.measurment);
        for tag_set in &self.tag_set {
            line += &format!(",{}", tag_set.to_text());
        }

        let mut first_iter = true;
        for field_set in &self.field_set {
            if first_iter {
                first_iter = false;
                line += &format!(" {}", field_set.to_text());
            } else {
                line += &format!(", {}", field_set.to_text());
            }
        }

        if let Some(ts) = self.timestamp.timestamp_nanos() {
            line += " ";
            line += &ts.to_string();
        }

        line
    }
}

/// Builder for [`Point`]
///
/// [`Point`]:Point
pub struct PointBuilder {
    measurment: String,
    tag_set: TagSet,
    field_set: FiledSet,
    timestamp: Timestamp,
}

impl PointBuilder {
    pub fn new(measurment: impl Into<String>) -> Self {
        Self {
            measurment: measurment.into(),
            tag_set: Default::default(),
            field_set: Default::default(),
            timestamp: Timestamp::Now,
        }
    }

    pub fn add_tag(mut self, tag_set: Tag) -> Self {
        self.tag_set.push(tag_set);
        self
    }

    pub fn add_field(mut self, field: Field) -> Self {
        self.field_set.push(field);
        self
    }

    pub fn timestamp(mut self, timestamp: impl Into<Timestamp>) -> Self {
        self.timestamp = timestamp.into();
        self
    }

    pub fn build(self) -> Result<Point, Error> {
        if self.field_set.is_empty() {
            panic!("At least one field value is required!");
        }

        //TODO change return Error
        Ok(Point {
            measurment: Measurement::new(self.measurment)?,
            tag_set: self.tag_set,
            field_set: self.field_set,
            timestamp: self.timestamp,
        })
    }
}

use super::{error::Error, escape, Field, FiledSet, Measurement, Tag, TagSet, Timestamp};
use std::{convert::TryInto, iter::FromIterator};

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

    pub fn add_tags<I>(mut self, tags: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Tag>,
    {
        self.tag_set = self
            .tag_set
            .into_iter()
            .chain(tags.into_iter().map(|x| x.into()))
            .collect::<TagSet>();
        self
    }

    pub fn try_add_tag<I, E>(self, tag: I) -> Result<Self, E>
    where
        I: TryInto<Tag>,
        E: From<I::Error>,
    {
        let tag = tag.try_into()?;
        Ok(self.add_tag(tag))
    }

    pub fn try_add_tags<I, E>(self, iter: I) -> Result<Self, E>
    where
        I: IntoIterator,
        I::Item: TryInto<Tag>,
        E: From<<I::Item as TryInto<Tag>>::Error>,
    {
        let tags_iter = iter.into_iter().map(|x| x.try_into());
        let res_tags = Result::from_iter(tags_iter.collect::<Vec<Result<Tag, _>>>());
        let v: Vec<Tag> = res_tags?;

        Ok(self.add_tags(v))
    }

    pub fn add_field(mut self, field: Field) -> Self {
        self.field_set.push(field);
        self
    }

    pub fn add_fields<I>(mut self, fields: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Field>,
    {
        self.field_set = self
            .field_set
            .into_iter()
            .chain(fields.into_iter().map(|x| x.into()))
            .collect::<Vec<_>>();
        self
    }

    pub fn try_add_fields<I, E>(self, iter: I) -> Result<Self, E>
    where
        I: IntoIterator,
        I::Item: TryInto<Field>,
        E: From<<I::Item as TryInto<Field>>::Error>,
    {
        let tags_iter = iter.into_iter().map(|x| x.try_into());
        let res_tags = Result::from_iter(tags_iter.collect::<Vec<Result<Field, _>>>());
        let v: Vec<Field> = res_tags?;

        Ok(self.add_fields(v))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vec_of_fields_to_builder() {
        let a = Field::new("a", "b").unwrap();
        let b = Field::new("c", 6i64).unwrap();

        let _point = Point::builder("test")
            .add_fields(vec![a, b])
            .build()
            .unwrap();
    }

    #[test]
    fn try_add_tags_to_builder() {
        let v = vec![("field1", "value1"), ("field2", "value2")];
        let _point = Point::builder("test")
            .try_add_fields::<_, Error>(v)
            .unwrap()
            .build()
            .unwrap();
    }
}

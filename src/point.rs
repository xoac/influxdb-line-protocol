use super::{error::Error, escape, Field, Measurement, Tag, TagSet, Timestamp};
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
    pub fn builder(measurment: impl Into<String>) -> Result<PointBuilder, Error> {
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
#[derive(Debug, Clone)]
pub struct PointBuilder {
    point: Point,
    errors: Vec<Error>,
}

impl PointBuilder {
    pub fn new(measurment: impl Into<String>) -> Result<Self, Error> {
        let measurment = Measurement::new(measurment)?;
        let point = Point {
            measurment,
            tag_set: Default::default(),
            field_set: Default::default(),
            timestamp: Timestamp::Now,
        };

        Ok(Self {
            point,
            errors: vec![],
        })
    }

    pub fn add_tag(mut self, tag_set: Tag) -> Self {
        self.point.tag_set.push(tag_set);
        self
    }

    pub fn add_tags<I>(mut self, tags: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Tag>,
    {
        self.point.tag_set = self
            .point
            .tag_set
            .into_iter()
            .chain(tags.into_iter().map(|x| x.into()))
            .collect::<TagSet>();
        self
    }

    pub fn try_add_tag<I>(mut self, tag: I) -> Self
    where
        I: TryInto<Tag>,
        I::Error: Into<Error>,
    {
        match tag.try_into() {
            Ok(tag) => self.add_tag(tag),
            Err(err) => {
                self.errors.push(err.into());
                self
            }
        }
    }

    pub fn try_add_tags<I>(mut self, iter: I) -> Self
    where
        I: IntoIterator,
        I::Item: TryInto<Tag>,
        <I::Item as TryInto<Tag>>::Error: Into<Error>,
    {
        let tags_iter = iter.into_iter().map(|x| x.try_into());
        let res_tags: Result<Vec<Tag>, _> =
            Result::from_iter(tags_iter.collect::<Vec<Result<Tag, _>>>());
        match res_tags {
            Ok(v) => self.add_tags(v),
            Err(err) => {
                self.errors.push(err.into());
                self
            }
        }
    }

    pub fn add_field(mut self, field: Field) -> Self {
        self.point.field_set.push(field);
        self
    }

    pub fn add_fields<I>(mut self, fields: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Field>,
    {
        self.point.field_set = self
            .point
            .field_set
            .into_iter()
            .chain(fields.into_iter().map(|x| x.into()))
            .collect::<Vec<_>>();
        self
    }

    pub fn try_add_fields<I>(mut self, iter: I) -> Self
    where
        I: IntoIterator,
        I::Item: TryInto<Field>,
        <I::Item as TryInto<Field>>::Error: Into<Error>,
    {
        let tags_iter = iter.into_iter().map(|x| x.try_into());
        let res_tags: Result<Vec<Field>, _> =
            Result::from_iter(tags_iter.collect::<Vec<Result<Field, _>>>());
        match res_tags {
            Ok(v) => self.add_fields(v),
            Err(err) => {
                self.errors.push(err.into());
                self
            }
        }
    }

    pub fn timestamp(mut self, timestamp: impl Into<Timestamp>) -> Self {
        self.point.timestamp = timestamp.into();
        self
    }

    pub fn errors(&self) -> &Vec<Error> {
        &self.errors
    }

    pub fn build(mut self) -> Result<Point, Error> {
        if self.point.field_set.is_empty() {
            panic!("At least one field value is required!");
        }

        if let Some(err) = self.errors.drain(..).next() {
            Err(err)
        } else {
            Ok(self.point)
        }
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
            .unwrap()
            .add_fields(vec![a, b])
            .build()
            .unwrap();
    }

    #[test]
    fn try_add_tags_to_builder() {
        let v = vec![("field1", "value1"), ("field2", "value2")];
        let _point = Point::builder("test")
            .unwrap()
            .try_add_fields(v)
            .build()
            .unwrap();
    }
}

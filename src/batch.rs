use super::Point;

/// A collection of data [`Points`] in InfluxDB line protocol format.
///
/// [`Points`]:Point
#[derive(Debug, Clone)]
pub struct Batch(Vec<Point>);

impl<V> From<V> for Batch
where
    V: Into<Point>,
{
    fn from(v: V) -> Self {
        let point = vec![v.into()];
        point.into()
    }
}

impl<V> From<Vec<V>> for Batch
where
    V: Into<Point>,
{
    fn from(v: Vec<V>) -> Self {
        let point_vec = v.into_iter().map(|v| v.into()).collect();

        Self(point_vec)
    }
}

impl Batch {
    pub fn to_line_protocol(&self) -> String {
        self.0
            .iter()
            .map(|point| point.to_text())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

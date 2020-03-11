use super::Point;

/// A collection of data [`Points`] in InfluxDB line protocol format.
///
/// [`Points`]:Point
#[derive(Debug, Clone)]
pub struct Batch(Vec<Point>);

impl From<Point> for Batch {
    fn from(point: Point) -> Self {
        Self(vec![point])
    }
}

impl From<Vec<Point>> for Batch {
    fn from(inner: Vec<Point>) -> Self {
        Self(inner)
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

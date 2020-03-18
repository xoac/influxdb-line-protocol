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
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn to_line_protocol(&self) -> String {
        self.0
            .iter()
            .map(|point| point.to_text())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn clone_and_clear(&mut self) -> Self {
        let mut new_v = Vec::with_capacity(self.len());
        std::mem::swap(&mut self.0, &mut new_v);
        Self(new_v)
    }

    pub fn push_point(&mut self, p: impl Into<Point>) {
        self.0.push(p.into())
    }

    pub fn push_points(&mut self, mut p: Vec<Point>) {
        self.0.append(&mut p)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

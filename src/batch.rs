use super::{Point, Precision};

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

    pub fn precision(&self) -> Option<Precision> {
        debug_assert!(Precision::Nanos > Precision::Secs);
        self.0.iter().map(|p| p.precision()).fold(None, |p, acc| {
            if p.map(|p| p.is_most_precise()).unwrap_or(false) {
                // this is early ending -- returns from precision
                return p;
            }
            match (p, acc) {
                (Some(p), Some(acc)) => {
                    if p > acc {
                        Some(p)
                    } else {
                        Some(acc)
                    }
                }
                (Some(p), None) => Some(p),
                (None, Some(acc)) => Some(acc),
                (None, None) => None,
            }
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Timestamp;

    #[test]
    fn precision_test() {
        let b_a1 = Point::builder("a")
            .unwrap()
            .try_add_field(("a", "a"))
            .timestamp(Timestamp::Nanos(1));
        let b_a2 = b_a1.clone().timestamp(Timestamp::Milli(2));
        assert_eq!(
            Batch::from(vec![b_a1.build().unwrap(), b_a2.build().unwrap()]).precision(),
            Some(Precision::Nanos)
        );
    }
}

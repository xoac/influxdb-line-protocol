use super::{Point, Precision};

/// A collection of data [`Points`] in InfluxDB line protocol format.
///
/// [`Points`]:Point
#[derive(Debug, Clone)]
pub struct Batch {
    inner: Vec<Point>,
    precision: Option<Precision>,
}

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
        let point_vec = v.into_iter().map(|v| v.into()).collect::<Vec<_>>();

        let mut b = Self::with_capacity(point_vec.len());
        b.push_points(point_vec);
        b
    }
}

impl Batch {
    fn update_precision(&mut self, precision: Option<Precision>) {
        if self.precision < precision {
            self.precision = precision;
        }
    }
}

impl Batch {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
            precision: None,
        }
    }

    pub fn to_line_protocol(&self) -> String {
        self.inner
            .iter()
            .map(|point| point.to_text_with_precision(self.precision))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn clone_and_clear(&mut self) -> Self {
        let mut new_v = Vec::with_capacity(self.len());
        std::mem::swap(&mut self.inner, &mut new_v);
        let r = Self {
            inner: new_v,
            precision: self.precision,
        };
        self.precision = None;
        r
    }

    pub fn push_point(&mut self, p: impl Into<Point>) {
        let point = p.into();
        self.update_precision(point.precision());
        self.inner.push(point)
    }

    pub fn push_points(&mut self, p_vec: Vec<Point>) {
        for p in p_vec {
            self.push_point(p)
        }
    }

    pub fn is_empty(&self) -> bool {
        debug_assert!(self.precision.is_none());
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
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
            Batch::from(vec![b_a1.build().unwrap(), b_a2.build().unwrap()]).precision,
            Some(Precision::Nanos)
        );
    }
}

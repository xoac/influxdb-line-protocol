use super::Point;

/// Conver into [`Point`]
///
/// [`Point`]:Point
pub trait IntoPoint {
    fn into_point(self) -> Point;
}

impl IntoPoint for Point {
    fn into_point(self) -> Point {
        self
    }
}

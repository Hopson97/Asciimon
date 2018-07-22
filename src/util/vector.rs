
use std::ops::Add;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T
}

impl<T> Vector2D<T> {
    pub fn new(x: T, y: T) -> Vector2D<T> {
        Vector2D {
            x, y
        }
    }
}

impl<T: Add<Output = T>> Add for Vector2D<T> {
    type Output = Vector2D<T>;

    fn add (self, other: Vector2D<T>) -> Vector2D<T> {
        Vector2D::new(self.x + other.x, self.y + other.y)
    }
}

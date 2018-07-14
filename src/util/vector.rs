struct Vector2D<T> {
    x: T,
    y: T
}

impl<T> Vector2D<T> {
    pub fn new(x: T, y: T) -> Vector2D<T> {
        Vector2D {
            x, y
        }
    }
}
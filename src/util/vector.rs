use std::fmt;
use std::ops::*;

pub const UP: Vector2D<i32> = Vector2D { x: 0, y: -1 };
pub const RIGHT: Vector2D<i32> = Vector2D { x: 1, y: 0 };
pub const DOWN: Vector2D<i32> = Vector2D { x: 0, y: 1 };
pub const LEFT: Vector2D<i32> = Vector2D { x: -1, y: 0 };

pub const ZERO: Vector2D<u32> = Vector2D { x: 0, y: 0 };
pub const ONE: Vector2D<u32> = Vector2D { x: 1, y: 1 };

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Vector2D { x, y }
    }

    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Vector2D<U> {
        Vector2D::new(f(self.x), f(self.y))
    }

    pub fn combine_with<U, V, F: Fn(T, U) -> V>(self, other: Vector2D<U>, f: F) -> Vector2D<V> {
        Vector2D::new(f(self.x, other.x), f(self.y, other.y))
    }
}

impl Vector2D<u32> {
    pub fn to_i32(self) -> Vector2D<i32> {
        Vector2D::new(self.x as i32, self.y as i32)
    }

    pub fn add_direction(self, direction: Vector2D<i32>) -> Self {
        self.combine_with(direction, |n, dir| {
            if dir < 0 {
                n - dir.abs() as u32
            } else if dir > 0 {
                n + dir as u32
            } else {
                n
            }
        })
    }
}

impl<T: fmt::Display> fmt::Display for Vector2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// vector + vector
impl<T: Add<Output = T>> Add for Vector2D<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector2D::new(self.x + other.x, self.y + other.y)
    }
}

// vector - vector
impl<T: Sub<Output = T>> Sub for Vector2D<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vector2D::new(self.x - other.x, self.y - other.y)
    }
}

// vector * scalar
impl<T: Mul<Output = T> + Copy> Mul<T> for Vector2D<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Vector2D::new(self.x * other, self.y * other)
    }
}

// vector * vector
impl<T: Mul<Output = T>> Mul for Vector2D<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Vector2D::new(self.x * other.x, self.y * other.y)
    }
}

// vector / scalar
impl<T: Div<Output = T> + Copy> Div<T> for Vector2D<T> {
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        Vector2D::new(self.x / other, self.y / other)
    }
}

// vector / vector
impl<T: Div<Output = T>> Div for Vector2D<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Vector2D::new(self.x / other.x, self.y / other.y)
    }
}

// vector += vector
impl<T: AddAssign> AddAssign for Vector2D<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

// vector -= vector
impl<T: SubAssign> SubAssign for Vector2D<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

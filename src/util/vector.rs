use std::fmt;
use std::ops::*;

pub const UP: Vector2D<i32> = Vector2D { x: 0, y: -1 };
pub const RIGHT: Vector2D<i32> = Vector2D { x: 1, y: 0 };
pub const DOWN: Vector2D<i32> = Vector2D { x: 0, y: 1 };
pub const LEFT: Vector2D<i32> = Vector2D { x: -1, y: 0 };

pub const ONE: Vector2D<i32> = Vector2D { x: 1, y: 1 };
pub const ZERO: Vector2D<i32> = Vector2D { x: 0, y: 0 };

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Vector2D { x, y }
    }

    pub fn cast<U: From<T>>(self) -> Vector2D<U> {
        Vector2D::new(U::from(self.x), U::from(self.y))
    }
}

impl Vector2D<f32> {
    pub fn length(&self) -> f32 {
        self.sqr_length().sqrt()
    }

    pub fn sqr_length(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn normalize(self) -> Self {
        if self.sqr_length() > 0.0 {
            self / self.length()
        } else {
            Vector2D::new(0.0, 0.0)
        }
    }
}

impl<T: fmt::Display> fmt::Display for Vector2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Add<Output = T>> Add for Vector2D<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector2D::new(self.x + other.x, self.y + other.y)
    }
}

impl<T: Sub<Output = T>> Sub for Vector2D<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vector2D::new(self.x - other.x, self.y - other.y)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector2D<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Vector2D::new(self.x * other, self.y * other)
    }
}

impl<T: Mul<Output = T>> Mul for Vector2D<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Vector2D::new(self.x * other.x, self.y * other.y)
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vector2D<T> {
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        Vector2D::new(self.x / other, self.y / other)
    }
}

impl<T: Div<Output = T>> Div for Vector2D<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Vector2D::new(self.x / other.x, self.y / other.y)
    }
}

impl<T: Neg<Output = T>> Neg for Vector2D<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector2D::new(-self.x, -self.y)
    }
}

impl<T: AddAssign> AddAssign for Vector2D<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: SubAssign> SubAssign for Vector2D<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector2D<T> {
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Vector2D<T> {
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
    }
}

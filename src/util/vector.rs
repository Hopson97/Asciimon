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

//-
impl<T: Sub<Output = T>> Sub for Vector2D<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vector2D::new(self.x - other.x, self.y - other.y)
    }
}

//*
impl<T: Mul<Output = T> + Copy> Mul<T> for Vector2D<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Vector2D::new(self.x * other, self.y * other)
    }
}

//*
impl<T: Mul<Output = T>> Mul for Vector2D<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Vector2D::new(self.x * other.x, self.y * other.y)
    }
}

//+=
impl<T: AddAssign> AddAssign for Vector2D<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

//-=
impl<T: SubAssign> SubAssign for Vector2D<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

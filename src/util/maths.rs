use std::cmp;
use std::ops::{Add, Rem, Sub};

pub fn clamp<T: Ord>(value: T, low: T, high: T) -> T {
    cmp::min(cmp::max(value, low), high)
}

pub fn repeat<T>(value: T, low: T, high: T) -> T
where
    T: Copy + PartialOrd<T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T>,
{
    let result = (value - low) % (high - low);
    // HACK: since we can't use constants with generic maths in Rust, here's a
    //       workaround: instead of comparing with 0, add `low` to both sides
    if low + result < low {
        result + high
    } else {
        result + low
    }
}

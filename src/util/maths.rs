use std::cmp;

pub fn clamp<T: Ord>(value: T, low: T, high: T) -> T {
    cmp::min(cmp::max(value, low), high)
}

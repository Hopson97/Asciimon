use std::cmp;

pub fn clamp<T>(value: T, low: T, high: T) -> T
where
    T: Ord,
{
    cmp::min(cmp::max(value, low), high)
}

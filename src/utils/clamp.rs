use std::cmp;

pub fn clamp<T: Ord + std::fmt::Debug>(min: T, val: T, max: T) -> T {
    assert!(
        min <= max,
        "Minimum clamp value is larger than maximum clamp value: {:?} > {:?}",
        min,
        max
    );
    cmp::min(cmp::max(min, val), max)
}

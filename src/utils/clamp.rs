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

pub fn clampf<T: PartialOrd + std::fmt::Debug>(min: T, val: T, max: T) -> T {
    assert!(
        min <= max,
        "Minimum clamp value is larger than maximum clamp value: {:?} > {:?}",
        min,
        max
    );
    if min > val {
        min
    } else if max < val {
        max
    } else {
        val
    }
}

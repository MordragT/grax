use std::cmp::{max_by, min_by, Ordering};

pub trait TotalOrd {
    fn total_ord(&self, other: &Self) -> Ordering;

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        min_by(self, other, Self::total_ord)
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        max_by(self, other, Self::total_ord)
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        assert!(min <= max);
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

impl TotalOrd for f32 {
    fn total_ord(&self, other: &Self) -> Ordering {
        f32::total_cmp(self, other)
    }
}

impl TotalOrd for f64 {
    fn total_ord(&self, other: &Self) -> Ordering {
        f64::total_cmp(self, other)
    }
}

pub trait Bounded {
    const MAX: Self;
    const MIN: Self;
}

impl Bounded for f64 {
    const MAX: Self = f64::INFINITY;
    const MIN: Self = f64::MIN;
}

impl Bounded for f32 {
    const MAX: Self = f32::INFINITY;
    const MIN: Self = f32::MIN;
}

impl Bounded for u32 {
    const MAX: Self = u32::MAX;
    const MIN: Self = u32::MIN;
}

impl Bounded for usize {
    const MAX: Self = usize::MAX;
    const MIN: Self = usize::MIN;
}

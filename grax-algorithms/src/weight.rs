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

macro_rules! impl_total_ord(
    ( $( $t:ident ),* )=> {
        $(
            impl TotalOrd for $t {
                fn total_ord(&self, other: &Self) -> Ordering {
                    Self::cmp(self, other)
                }
            }
        )*
    }
);

impl_total_ord!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

pub trait Bounded {
    const MAX: Self;
    const MIN: Self;
}

macro_rules! impl_bounded(
    ( $( $t:ident ),* )=> {
        $(
            impl Bounded for $t {
                const MAX: Self = $t::MAX;
                const MIN: Self = $t::MIN;
            }

        )*
    }
);

impl_bounded!(f32, f64, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

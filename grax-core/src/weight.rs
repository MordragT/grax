use std::{
    cmp::{max_by, min_by, Ordering},
    fmt::Debug,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

pub trait Sortable: PartialOrd + Sized {
    fn sort(&self, other: &Self) -> Ordering;

    fn min(self, other: Self) -> Self {
        min_by(self, other, Self::sort)
    }

    fn max(self, other: Self) -> Self {
        max_by(self, other, Self::sort)
    }
}

default impl<T: PartialOrd> Sortable for T {
    default fn sort(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl Sortable for usize {
    fn sort(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl Sortable for f64 {
    fn sort(&self, other: &Self) -> Ordering {
        self.total_cmp(other)
    }
}

impl Sortable for f32 {
    fn sort(&self, other: &Self) -> Ordering {
        self.total_cmp(other)
    }
}

pub trait Maximum {
    const MAX: Self;
}

impl Maximum for f64 {
    const MAX: Self = f64::INFINITY;
}

impl Maximum for f32 {
    const MAX: Self = f32::INFINITY;
}

impl Maximum for u32 {
    const MAX: Self = u32::MAX;
}

impl Maximum for usize {
    const MAX: Self = usize::MAX;
}

pub trait Numeric:
    PartialEq + Add<Self, Output = Self> + Sub<Self, Output = Self> + AddAssign + SubAssign + Copy
{
}

impl<
        T: PartialEq
            + Add<Self, Output = Self>
            + Sub<Self, Output = Self>
            + AddAssign
            + SubAssign
            + Copy,
    > Numeric for T
{
}

pub trait Nominal: Debug + Clone {}

impl<T: Debug + Clone> Nominal for T {}

pub trait Ordinal: Nominal + Sortable {}

impl<T: Nominal + Sortable> Ordinal for T {}

pub trait Interval: Ordinal + Numeric + Neg {}

impl<T: Ordinal + Numeric + Neg> Interval for T {}

pub trait Ratio: Ordinal + Numeric + Default {}

impl<T: Ordinal + Numeric + Default> Ratio for T {}

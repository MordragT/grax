use std::{
    cmp::{max_by, min_by, Ordering},
    fmt::Debug,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

// Rename to total ord ?

pub trait Sortable: Sized + PartialEq + PartialOrd {
    fn sort(&self, other: &Self) -> Ordering;

    fn equal(&self, other: &Self) -> bool {
        self.sort(other).is_eq()
    }

    fn min(self, other: Self) -> Self {
        min_by(self, other, Self::sort)
    }

    fn max(self, other: Self) -> Self {
        max_by(self, other, Self::sort)
    }
}

impl<T: PartialOrd> Sortable for T {
    default fn sort(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
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
pub struct Sorted<T: Sortable>(pub T);

impl<T: Sortable> PartialEq for Sorted<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.equal(&other.0)
    }
}

impl<T: Sortable> Eq for Sorted<T> {}

impl<T: Sortable> PartialOrd for Sorted<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.sort(&other.0))
    }
}

impl<T: Sortable> Ord for Sorted<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.sort(&other.0)
    }
}

pub struct RevSorted<T: Sortable>(pub T);

impl<T: Sortable> PartialEq for RevSorted<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.equal(&other.0)
    }
}

impl<T: Sortable> Eq for RevSorted<T> {}

impl<T: Sortable> PartialOrd for RevSorted<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.0.sort(&self.0))
    }
}

impl<T: Sortable> Ord for RevSorted<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.sort(&self.0)
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

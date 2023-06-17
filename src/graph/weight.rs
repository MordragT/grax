use std::{
    cmp::Ordering,
    fmt::Debug,
    ops::{Add, AddAssign, Sub, SubAssign},
};

pub trait Sortable: PartialOrd {
    fn sort(&self, other: &Self) -> Ordering;
}

default impl<T: PartialOrd> Sortable for T {
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

pub trait Maximum {
    fn max() -> Self;
}

impl Maximum for f64 {
    fn max() -> Self {
        f64::INFINITY
    }
}

impl Maximum for f32 {
    fn max() -> Self {
        f32::INFINITY
    }
}

impl Maximum for u32 {
    fn max() -> Self {
        u32::MAX
    }
}

pub trait Cost:
    Sortable
    + Maximum
    + Default
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + AddAssign
    + SubAssign
    + Copy
    + Debug
{
}

impl<
        T: Sortable
            + Maximum
            + Default
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + AddAssign
            + SubAssign
            + Copy
            + Debug,
    > Cost for T
{
}

pub trait WeightCapacity {
    type Capacity;

    fn capacity(&self) -> &Self::Capacity;
    fn capacity_mut(&mut self) -> &mut Self::Capacity;
}

pub trait WeightCost {
    type Cost;

    fn cost(&self) -> &Self::Cost;
    fn cost_mut(&mut self) -> &mut Self::Cost;
}

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct FlowWeight<W> {
    pub capacity: W,
    pub cost: W,
}

impl<W> FlowWeight<W> {
    pub fn new(capacity: W, cost: W) -> Self {
        Self { capacity, cost }
    }
}

impl<W> WeightCapacity for FlowWeight<W> {
    type Capacity = W;

    fn capacity(&self) -> &Self::Capacity {
        &self.capacity
    }

    fn capacity_mut(&mut self) -> &mut Self::Capacity {
        &mut self.capacity
    }
}

impl<W> WeightCost for FlowWeight<W> {
    type Cost = W;

    fn cost(&self) -> &Self::Cost {
        &self.cost
    }

    fn cost_mut(&mut self) -> &mut Self::Cost {
        &mut self.cost
    }
}

impl WeightCost for f32 {
    type Cost = f32;

    fn cost(&self) -> &Self::Cost {
        &self
    }

    fn cost_mut(&mut self) -> &mut Self::Cost {
        self
    }
}

impl WeightCost for f64 {
    type Cost = f64;

    fn cost(&self) -> &Self::Cost {
        &self
    }

    fn cost_mut(&mut self) -> &mut Self::Cost {
        self
    }
}

pub trait Weight: WeightCost<Cost: Cost> + Copy + Debug {}

impl<T: WeightCost<Cost: Cost> + Copy + Debug> Weight for T {}

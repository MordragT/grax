use std::{
    cmp::Ordering,
    fmt::Debug,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
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
    + Neg
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
            + Neg
            + Copy
            + Debug,
    > Cost for T
{
}

// TODO split into mutable and non mutable
// so that cost weights can just return the maximum capacity
pub trait EdgeCapacity {
    type Capacity;

    fn capacity(&self) -> &Self::Capacity;
    fn capacity_mut(&mut self) -> &mut Self::Capacity;
}

pub trait EdgeCost {
    type Cost;

    fn cost(&self) -> &Self::Cost;
    fn cost_mut(&mut self) -> &mut Self::Cost;
}

pub trait EdgeDirection {
    fn is_reverse(&self) -> bool;
    fn reverse(&mut self);
}

pub trait EdgeFlow {
    type Flow;

    fn flow(&self) -> &Self::Flow;
    fn flow_mut(&mut self) -> &mut Self::Flow;
}

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct FlowWeight<W> {
    pub flow: W,
    pub capacity: W,
    pub cost: W,
    pub rev: bool,
}

impl<W> FlowWeight<W> {
    pub fn new(capacity: W, cost: W, flow: W) -> Self {
        Self {
            capacity,
            cost,
            rev: false,
            flow,
        }
    }

    pub fn rev(capacity: W, cost: W, flow: W) -> Self {
        Self {
            capacity,
            cost,
            rev: true,
            flow,
        }
    }
}

impl<W> EdgeFlow for FlowWeight<W> {
    type Flow = W;

    fn flow(&self) -> &Self::Flow {
        &self.flow
    }

    fn flow_mut(&mut self) -> &mut Self::Flow {
        &mut self.flow
    }
}

impl<W> EdgeDirection for FlowWeight<W> {
    fn is_reverse(&self) -> bool {
        self.rev
    }

    fn reverse(&mut self) {
        self.rev = !self.rev;
    }
}

impl<W> EdgeCapacity for FlowWeight<W> {
    type Capacity = W;

    fn capacity(&self) -> &Self::Capacity {
        &self.capacity
    }

    fn capacity_mut(&mut self) -> &mut Self::Capacity {
        &mut self.capacity
    }
}

impl<W> EdgeCost for FlowWeight<W> {
    type Cost = W;

    fn cost(&self) -> &Self::Cost {
        &self.cost
    }

    fn cost_mut(&mut self) -> &mut Self::Cost {
        &mut self.cost
    }
}

impl EdgeCost for f32 {
    type Cost = f32;

    fn cost(&self) -> &Self::Cost {
        &self
    }

    fn cost_mut(&mut self) -> &mut Self::Cost {
        self
    }
}

impl EdgeCost for f64 {
    type Cost = f64;

    fn cost(&self) -> &Self::Cost {
        &self
    }

    fn cost_mut(&mut self) -> &mut Self::Cost {
        self
    }
}

impl EdgeCapacity for f32 {
    type Capacity = f32;

    fn capacity(&self) -> &Self::Capacity {
        &f32::MAX
    }

    fn capacity_mut(&mut self) -> &mut Self::Capacity {
        panic!("Cannot mutate capacity of cost only weight")
    }
}

impl EdgeCapacity for f64 {
    type Capacity = f64;

    fn capacity(&self) -> &Self::Capacity {
        &f64::MAX
    }

    fn capacity_mut(&mut self) -> &mut Self::Capacity {
        panic!("Cannot mutate capacity of cost only weight")
    }
}

impl EdgeFlow for f32 {
    type Flow = f32;

    fn flow(&self) -> &Self::Flow {
        &0.0
    }

    fn flow_mut(&mut self) -> &mut Self::Flow {
        panic!("Cannot mutate flow of cost only weight")
    }
}

impl EdgeFlow for f64 {
    type Flow = f64;

    fn flow(&self) -> &Self::Flow {
        &0.0
    }

    fn flow_mut(&mut self) -> &mut Self::Flow {
        panic!("Cannot mutate flow of cost only weight")
    }
}

pub trait Weight: EdgeCost<Cost: Cost> + Copy + Debug {}

impl<T: EdgeCost<Cost: Cost> + Copy + Debug> Weight for T {}

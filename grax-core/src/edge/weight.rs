use std::ops::{Neg, Sub};

pub trait Flow<T> {
    fn flow(&self) -> &T;
    fn flow_mut(&mut self) -> &mut T;
}

pub trait Capacity<T> {
    fn capacity(&self) -> &T;
    fn capacity_mut(&mut self) -> &mut T;
}

pub trait Cost<T> {
    fn cost(&self) -> &T;
    fn cost_mut(&mut self) -> &mut T;
}

pub trait Reverse {
    fn reverse(&self) -> Self;
    fn is_reverse(&self) -> bool;
}

pub trait ResidualCapacity<T>: Flow<T> + Capacity<T>
where
    T: Copy + Sub<T, Output = T>,
{
    fn residual_capacity(&self) -> T {
        *self.capacity() - *self.flow()
    }
}

impl<T, C> ResidualCapacity<C> for T
where
    C: Copy + Sub<C, Output = C>,
    T: Flow<C> + Capacity<C>,
{
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct FlowBundle<T> {
    pub flow: T,
    pub capacity: T,
    pub reverse: bool,
}

impl<T> Flow<T> for FlowBundle<T> {
    fn flow(&self) -> &T {
        &self.flow
    }

    fn flow_mut(&mut self) -> &mut T {
        &mut self.flow
    }
}

impl<T> Capacity<T> for FlowBundle<T> {
    fn capacity(&self) -> &T {
        &self.capacity
    }

    fn capacity_mut(&mut self) -> &mut T {
        &mut self.capacity
    }
}

impl<T> Reverse for FlowBundle<T>
where
    T: Clone + Sub<T, Output = T>,
{
    fn is_reverse(&self) -> bool {
        self.reverse
    }

    fn reverse(&self) -> Self {
        let Self {
            capacity,
            flow,
            reverse,
        } = self;

        Self {
            flow: capacity.clone() - flow.clone(),
            capacity: capacity.clone(),
            reverse: !reverse,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct FlowCostBundle<T> {
    pub cost: T,
    pub flow: T,
    pub capacity: T,
    pub reverse: bool,
}

impl<T> Cost<T> for FlowCostBundle<T> {
    fn cost(&self) -> &T {
        &self.cost
    }

    fn cost_mut(&mut self) -> &mut T {
        &mut self.cost
    }
}

impl<T> Flow<T> for FlowCostBundle<T> {
    fn flow(&self) -> &T {
        &self.flow
    }

    fn flow_mut(&mut self) -> &mut T {
        &mut self.flow
    }
}

impl<T> Capacity<T> for FlowCostBundle<T> {
    fn capacity(&self) -> &T {
        &self.capacity
    }

    fn capacity_mut(&mut self) -> &mut T {
        &mut self.capacity
    }
}

impl<T> Reverse for FlowCostBundle<T>
where
    T: Clone + Sub<T, Output = T> + Neg<Output = T>,
{
    fn is_reverse(&self) -> bool {
        self.reverse
    }

    fn reverse(&self) -> Self {
        let Self {
            cost,
            capacity,
            flow,
            reverse,
        } = self;

        Self {
            cost: -cost.clone(),
            flow: capacity.clone() - flow.clone(),
            capacity: capacity.clone(),
            reverse: !reverse,
        }
    }
}

macro_rules! impl_cost(
    ( $( $t:ident ),* )=> {
        $(
            impl Cost<$t> for $t {
                fn cost(&self) -> &Self {
                    self
                }
                fn cost_mut(&mut self) -> &mut Self {
                    self
                }
            }

        )*
    }
);

impl_cost!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

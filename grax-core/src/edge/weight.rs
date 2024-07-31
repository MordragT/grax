use std::ops::Sub;

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

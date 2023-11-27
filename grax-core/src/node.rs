use std::fmt::Debug;

pub trait NodeBalance {
    type Balance;

    fn balance(&self) -> &Self::Balance;
    fn balance_mut(&mut self) -> &mut Self::Balance;
}

pub trait Node: Default + PartialEq + Clone + Debug {}

impl<T: Default + PartialEq + Clone + Debug> Node for T {}

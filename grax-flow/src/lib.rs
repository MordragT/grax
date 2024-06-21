use std::fmt::Debug;

pub use adaptor::*;

mod adaptor;

pub trait EdgeFlow: Clone + Debug {
    type Flow;

    fn capacity(&self) -> &Self::Flow;
    fn capacity_mut(&mut self) -> &mut Self::Flow;
    fn flow(&self) -> &Self::Flow;
    fn flow_mut(&mut self) -> &mut Self::Flow;
    fn is_reverse(&self) -> bool;
    fn rev(self) -> Self;
}

pub trait NodeBalance {
    type Balance;

    fn balance(&self) -> &Self::Balance;
    fn balance_mut(&mut self) -> &mut Self::Balance;
}

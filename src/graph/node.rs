use std::fmt::Debug;

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct BalancedNode<N, W> {
    pub node: N,
    pub balance: W,
}

impl<N, W> BalancedNode<N, W> {
    pub fn new(node: N, balance: W) -> Self {
        Self { node, balance }
    }
}

pub trait Node: Default + PartialEq + Clone + Debug {}

impl<T: Default + PartialEq + Clone + Debug> Node for T {}

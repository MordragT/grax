use std::{fmt::Debug, hash::Hash};

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

pub trait Node: Default + PartialEq + Clone {}

impl<T: Default + PartialEq + Clone> Node for T {}

pub trait NodeIdentifier: Hash + Eq + Copy + Debug {
    fn as_usize(&self) -> usize;
}

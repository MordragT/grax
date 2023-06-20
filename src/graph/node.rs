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

impl<N, W> NodeBalance for BalancedNode<N, W> {
    type Balance = W;

    fn balance(&self) -> &Self::Balance {
        &self.balance
    }

    fn balance_mut(&mut self) -> &mut Self::Balance {
        &mut self.balance
    }
}

pub trait NodeBalance {
    type Balance;

    fn balance(&self) -> &Self::Balance;
    fn balance_mut(&mut self) -> &mut Self::Balance;
}

pub trait Node: Default + PartialEq + Clone + Debug {}

impl<T: Default + PartialEq + Clone + Debug> Node for T {}

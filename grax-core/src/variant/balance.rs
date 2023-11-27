use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{
    node::NodeBalance,
    traits::{Balance, Base, Viewable},
    view::AttrMap,
};

pub struct BalancedNode<N, B> {
    node: N,
    balance: B,
}

impl<N, B> BalancedNode<N, B> {
    pub fn new(node: N, balance: B) -> Self {
        Self { node, balance }
    }
}

impl<N, B> NodeBalance for BalancedNode<N, B> {
    type Balance = B;

    fn balance(&self) -> &Self::Balance {
        &self.balance
    }

    fn balance_mut(&mut self) -> &mut Self::Balance {
        &mut self.balance
    }
}

pub struct BalanceGraph<G: Viewable, B: Clone + Debug> {
    graph: G,
    balance: G::NodeMap<Option<B>>,
}

impl<G, B> Base for BalanceGraph<G, B>
where
    G: Viewable,
    B: Clone + Debug + Default,
{
    type Id = G::Id;
    type Node = G::Node;
    type Weight = G::Weight;
}

impl<G, B> Balance<B::Balance> for BalanceGraph<G, B>
where
    G: Viewable,
    B: Clone + Debug + Default + NodeBalance,
{
    type NodeBalance = B;

    fn balance(&self, node_id: crate::prelude::NodeId<Self::Id>) -> Option<&Self::NodeBalance> {
        self.balance.get(node_id).as_ref()
    }

    fn balance_mut(
        &mut self,
        node_id: crate::prelude::NodeId<Self::Id>,
    ) -> Option<&mut Self::NodeBalance> {
        self.balance.get_mut(node_id).as_mut()
    }
}

impl<G, B> Deref for BalanceGraph<G, B>
where
    G: Viewable,
    B: Clone + Debug + Default,
{
    type Target = G;
    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}

impl<G, B> DerefMut for BalanceGraph<G, B>
where
    G: Viewable,
    B: Clone + Debug + Default,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.graph
    }
}

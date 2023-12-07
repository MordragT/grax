use std::fmt::Debug;

use crate::{
    collections::EdgeCollection,
    edge::{Edge, EdgeCost, EdgeFlow},
    graph::AdaptEdge,
    node::NodeBalance,
    weight::Maximum,
};

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
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

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct FlowBundle<W, C> {
    pub weight: W,
    pub cost: C,
    pub capacity: C,
    pub flow: C,
    pub reverse: bool,
}

impl<W: Clone + Debug, C: Clone + Debug> EdgeCost for FlowBundle<W, C> {
    type Cost = C;

    fn cost(&self) -> &Self::Cost {
        &self.cost
    }

    fn cost_mut(&mut self) -> &mut Self::Cost {
        &mut self.cost
    }
}

impl<W: Clone + Debug, C: Clone + Debug> EdgeFlow for FlowBundle<W, C> {
    type Flow = C;

    fn flow(&self) -> &Self::Flow {
        &self.flow
    }

    fn flow_mut(&mut self) -> &mut Self::Flow {
        &mut self.flow
    }

    fn capacity(&self) -> &Self::Flow {
        &self.capacity
    }

    fn capacity_mut(&mut self) -> &mut Self::Flow {
        &mut self.capacity
    }

    fn is_reverse(&self) -> bool {
        self.reverse
    }

    fn reverse(&mut self) {
        self.reverse = true;
    }
}

pub fn flow_adaptor<G1, G2, W1>(graph: G1) -> G2
where
    W1: Clone + Maximum + Default,
    G1: EdgeCollection<EdgeWeight = W1> + AdaptEdge<G2, FlowBundle<W1, W1>>,
    G2: EdgeCollection<EdgeWeight = FlowBundle<W1, W1>>,
{
    graph.map_edge(|edge| {
        let Edge { edge_id, weight } = edge;

        let bundle = FlowBundle {
            cost: weight.clone(),
            weight,
            capacity: Maximum::MAX,
            flow: Default::default(),
            reverse: false,
        };

        Edge::new(edge_id, bundle)
    })
}

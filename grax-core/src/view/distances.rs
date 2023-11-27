use std::fmt::Debug;

use crate::{
    prelude::NodeId,
    traits::{Cost, Viewable},
};

use super::{AttrMap, Parents};

#[derive(Debug, Clone)]
pub struct Distances<C: Clone + Debug, G: Cost<C> + Viewable> {
    pub distances: G::NodeMap<Option<C>>,
    pub parents: Parents<G>,
}

impl<C: Clone + Debug, G: Cost<C> + Viewable> Distances<C, G> {
    pub(crate) fn new(distances: G::NodeMap<Option<C>>, parents: Parents<G>) -> Self {
        Self { distances, parents }
    }

    pub fn insert(&mut self, from: NodeId<G::Id>, to: NodeId<G::Id>, cost: C) {
        self.parents.insert(from, to);
        self.distances.insert(to, Some(cost));
    }

    pub fn update_cost(&mut self, to: NodeId<G::Id>, cost: C) -> Option<C> {
        self.distances.replace(to, Some(cost))
    }

    pub fn distance(&self, node: NodeId<G::Id>) -> Option<&C> {
        self.distances.get(node).as_ref()
    }
}

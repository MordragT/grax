use crate::graph::Base;
use crate::prelude::EdgeIdentifier;

#[derive(Debug)]
pub struct Route<G: Base>(Vec<G::NodeId>);

impl<G: Base> From<Vec<G::NodeId>> for Route<G> {
    fn from(value: Vec<G::NodeId>) -> Self {
        Self(value)
    }
}

impl<G: Base> Into<Vec<G::NodeId>> for Route<G> {
    fn into(self) -> Vec<G::NodeId> {
        self.0
    }
}

impl<G: Base> Route<G> {
    pub fn new(route: Vec<G::NodeId>) -> Self {
        Self(route)
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn first(&self) -> Option<G::NodeId> {
        self.0.first().cloned()
    }

    pub fn last(&self) -> Option<G::NodeId> {
        self.0.last().cloned()
    }

    pub fn node_ids(&self) -> &Vec<G::NodeId> {
        &self.0
    }

    pub fn back_edge(&self) -> Option<G::EdgeId> {
        match (self.last(), self.first()) {
            (Some(last), Some(first)) if last != first => Some(G::EdgeId::between(last, first)),
            _ => None,
        }
    }

    pub fn node_id_cycle(&self) -> impl Iterator<Item = G::NodeId> + '_ {
        self.0.iter().cloned().chain(self.first().into_iter())
    }

    pub fn edge_ids(&self) -> impl Iterator<Item = G::EdgeId> + '_ {
        self.0
            .array_windows::<2>()
            .map(|[from, to]| G::EdgeId::between(*from, *to))
    }

    pub fn edge_id_cycle(&self) -> impl Iterator<Item = G::EdgeId> + '_ {
        self.edge_ids().chain(self.back_edge().into_iter())
    }

    pub fn into_raw(self) -> Vec<G::NodeId> {
        self.0
    }
}

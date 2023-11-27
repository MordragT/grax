use crate::{
    prelude::{EdgeId, NodeId},
    traits::Base,
};

#[derive(Debug)]
pub struct Route<G: Base>(Vec<NodeId<G::Id>>);

impl<G: Base> From<Vec<NodeId<G::Id>>> for Route<G> {
    fn from(value: Vec<NodeId<G::Id>>) -> Self {
        Self(value)
    }
}

impl<G: Base> Into<Vec<NodeId<G::Id>>> for Route<G> {
    fn into(self) -> Vec<NodeId<G::Id>> {
        self.0
    }
}

impl<G: Base> Route<G> {
    pub fn new(route: Vec<NodeId<G::Id>>) -> Self {
        Self(route)
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn first(&self) -> Option<NodeId<G::Id>> {
        self.0.first().cloned()
    }

    pub fn last(&self) -> Option<NodeId<G::Id>> {
        self.0.last().cloned()
    }

    pub fn node_ids(&self) -> &Vec<NodeId<G::Id>> {
        &self.0
    }

    pub fn back_edge(&self) -> Option<EdgeId<G::Id>> {
        match (self.last(), self.first()) {
            (Some(last), Some(first)) if last != first => Some(EdgeId::new_unchecked(last, first)),
            _ => None,
        }
    }

    pub fn node_id_cycle(&self) -> impl Iterator<Item = NodeId<G::Id>> + '_ {
        self.0.iter().cloned().chain(self.first().into_iter())
    }

    pub fn edge_ids(&self) -> impl Iterator<Item = EdgeId<G::Id>> + '_ {
        self.0
            .array_windows::<2>()
            .map(|[from, to]| EdgeId::new_unchecked(*from, *to))
    }

    pub fn edge_id_cycle(&self) -> impl Iterator<Item = EdgeId<G::Id>> + '_ {
        self.edge_ids().chain(self.back_edge().into_iter())
    }

    pub fn into_raw(self) -> Vec<NodeId<G::Id>> {
        self.0
    }
}

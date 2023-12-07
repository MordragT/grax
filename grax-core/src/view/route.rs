use crate::{
    collections::Keyed,
    prelude::{EdgeId, NodeId},
};

#[derive(Debug)]
pub struct Route<G: Keyed>(Vec<NodeId<G::Key>>);

impl<G: Keyed> From<Vec<NodeId<G::Key>>> for Route<G> {
    fn from(value: Vec<NodeId<G::Key>>) -> Self {
        Self(value)
    }
}

impl<G: Keyed> Into<Vec<NodeId<G::Key>>> for Route<G> {
    fn into(self) -> Vec<NodeId<G::Key>> {
        self.0
    }
}

impl<G: Keyed> Route<G> {
    pub fn new(route: Vec<NodeId<G::Key>>) -> Self {
        Self(route)
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn first(&self) -> Option<NodeId<G::Key>> {
        self.0.first().cloned()
    }

    pub fn last(&self) -> Option<NodeId<G::Key>> {
        self.0.last().cloned()
    }

    pub fn node_ids(&self) -> &Vec<NodeId<G::Key>> {
        &self.0
    }

    pub fn back_edge(&self) -> Option<EdgeId<G::Key>> {
        match (self.last(), self.first()) {
            (Some(last), Some(first)) if last != first => Some(EdgeId::new_unchecked(last, first)),
            _ => None,
        }
    }

    pub fn node_id_cycle(&self) -> impl Iterator<Item = NodeId<G::Key>> + '_ {
        self.0.iter().cloned().chain(self.first().into_iter())
    }

    pub fn edge_ids(&self) -> impl Iterator<Item = EdgeId<G::Key>> + '_ {
        self.0
            .array_windows::<2>()
            .map(|[from, to]| EdgeId::new_unchecked(*from, *to))
    }

    pub fn edge_id_cycle(&self) -> impl Iterator<Item = EdgeId<G::Key>> + '_ {
        self.edge_ids().chain(self.back_edge().into_iter())
    }

    pub fn into_raw(self) -> Vec<NodeId<G::Key>> {
        self.0
    }
}

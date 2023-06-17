use std::{fmt::Debug, hash::Hash};

pub trait NodeIdentifier: From<usize> + Into<usize> + Hash + Eq + Copy + Debug {
    fn as_usize(&self) -> usize {
        (*self).into()
    }
}

impl<T: From<usize> + Into<usize> + Hash + Eq + Copy + Debug> NodeIdentifier for T {}

pub trait EdgeIdentifier: Hash + Eq + Copy + Debug {
    type NodeId: NodeIdentifier;

    fn between(from: Self::NodeId, to: Self::NodeId) -> Self;

    /// Reveres the edge index
    fn rev(&self) -> Self;
    fn to(&self) -> Self::NodeId;
    fn from(&self) -> Self::NodeId;
    fn contains(&self, node_id: Self::NodeId) -> bool;
    fn as_usize(&self) -> (usize, usize);
}

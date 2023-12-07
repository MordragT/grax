use std::ops::Rem;

use bitvec::vec::BitVec;
use grax_core::{
    collections::{EdgeCollection, EdgeCount, GetEdge, GetEdgeMut, Keyed, VisitEdgeMap},
    edge::{EdgeMut, EdgeRef},
    index::{EdgeId, NodeId},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EdgeBoolVec {
    vec: BitVec,
    node_count: usize,
}

impl EdgeBoolVec {
    pub fn new(vec: BitVec, node_count: usize) -> Self {
        Self { vec, node_count }
    }
}

impl Keyed for EdgeBoolVec {
    type Key = usize;
}

impl EdgeCollection for EdgeBoolVec {
    type EdgeWeight = bool;

    fn edges_capacity(&self) -> usize {
        self.vec.capacity()
    }
}

impl EdgeCount for EdgeBoolVec {
    fn edge_count(&self) -> usize {
        self.vec.len()
    }
}

// impl GetEdge for EdgeBoolVec {
//     fn edge(&self, edge_id: EdgeId<Self::Key>) -> Option<EdgeRef<Self::Key, Self::EdgeWeight>> {
//         self.vec
//             .get(*edge_id.from() * self.node_count + *edge_id.to())
//             .map(|weight| EdgeRef::new(edge_id, weight.as_ref()))
//     }
// }

// impl GetEdgeMut for EdgeBoolVec {
//     fn edge_mut(
//         &mut self,
//         edge_id: EdgeId<Self::Key>,
//     ) -> Option<grax_core::prelude::EdgeMut<Self::Key, Self::EdgeWeight>> {
//         self.vec
//             .get_mut(*edge_id.from() * self.node_count + *edge_id.to())
//             .map(|mut weight| EdgeMut::new(edge_id, weight.as_mut()))
//     }
// }

// impl EdgeIter for EdgeBoolVec {
//     type EdgeIds<'a> = impl Iterator<Item = EdgeId<Self::Key>> + 'a where Self: 'a;
//     type Edges<'a> = impl Iterator<Item = EdgeRef<'a, Self::Key, Self::EdgeWeight>> + 'a where Self: 'a;

//     fn iter_edges(&self) -> Self::Edges<'_> {
//         self.vec.iter().enumerate().map(|(key, weight)| {
//             let from = NodeId::new_unchecked(key.div_floor(self.node_count));
//             let to = NodeId::new_unchecked(key.rem(self.node_count));
//             let edge_id = EdgeId::new_unchecked(from, to);
//             EdgeRef::new(edge_id, weight.as_ref())
//         })
//     }

//     fn edge_ids(&self) -> Self::EdgeIds<'_> {
//         (0..self.vec.len()).map(|key| {
//             let from = NodeId::new_unchecked(key.div_floor(self.node_count));
//             let to = NodeId::new_unchecked(key.rem(self.node_count));
//             EdgeId::new_unchecked(from, to)
//         })
//     }
// }

// impl FixedEdgeMap<usize, bool> for EdgeBoolVec {}

impl VisitEdgeMap<usize> for EdgeBoolVec {
    type IterUnvisited<'a> = impl Iterator<Item = EdgeId<usize>> + 'a where Self: 'a;
    type IterVisited<'a> = impl Iterator<Item = EdgeId<usize>> + 'a where Self: 'a;

    fn is_visited(&self, edge_id: EdgeId<Self::Key>) -> bool {
        if let Some(weight) = self
            .vec
            .get(*edge_id.from() * self.node_count + *edge_id.to())
        {
            *weight
        } else {
            false
        }
    }

    fn visit(&mut self, edge_id: EdgeId<Self::Key>) {
        self.vec
            .set(*edge_id.from() * self.node_count + *edge_id.to(), true)
    }

    fn unvisit(&mut self, edge_id: EdgeId<Self::Key>) {
        self.vec
            .set(*edge_id.from() * self.node_count + *edge_id.to(), false)
    }

    fn all_visited(&self) -> bool {
        self.vec.all()
    }

    fn iter_visited(&self) -> Self::IterVisited<'_> {
        self.vec.iter_ones().map(|key| {
            let from = NodeId::new_unchecked(key.div_floor(self.node_count));
            let to = NodeId::new_unchecked(key.rem(self.node_count));
            EdgeId::new_unchecked(from, to)
        })
    }

    fn iter_unvisited(&self) -> Self::IterUnvisited<'_> {
        self.vec.iter_zeros().map(|key| {
            let from = NodeId::new_unchecked(key.div_floor(self.node_count));
            let to = NodeId::new_unchecked(key.rem(self.node_count));
            EdgeId::new_unchecked(from, to)
        })
    }
}

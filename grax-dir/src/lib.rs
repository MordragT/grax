use std::{
    fmt::Debug,
    io::Write,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use cap_std::fs::Dir;
use grax_core::{
    collections::{
        EdgeCollection, EdgeCount, EdgeIter, EdgeIterMut, GetEdge, GetEdgeMut, GetNode, GetNodeMut,
        InsertEdge, InsertNode, Keyed, NodeCollection, NodeCount, NodeIter, NodeIterMut,
        RemoveEdge, RemoveNode, VisitEdgeMap, VisitNodeMap,
    },
    index::NodeId,
};

pub enum DirNodeKind {
    File,
    Dir,
}

pub trait DirNode: Debug {
    fn kind(&self) -> DirNodeKind;
    fn name(&self) -> &str;
    fn contents(&self) -> &[u8];
}

#[derive(Debug)]
pub struct DirGraph<'id, N, W> {
    dir: Dir,
    node_weight: PhantomData<N>,
    edge_weight: PhantomData<W>,
    index: PhantomData<&'id Self>,
}

impl<'id, N: DirNode, W: Debug> Keyed for DirGraph<'id, N, W> {
    type Key = &'id Path;
}

impl<'id, N: DirNode, W: Debug> NodeCollection for DirGraph<'id, N, W> {
    type NodeWeight = N;

    fn nodes_capacity(&self) -> usize {
        usize::MAX
    }
}

impl<'id, N: DirNode, W: Debug> EdgeCollection for DirGraph<'id, N, W> {
    type EdgeWeight = W;

    fn edges_capacity(&self) -> usize {
        usize::MAX
    }
}

impl<'id, N: DirNode, W: Debug> NodeCount for DirGraph<'id, N, W> {
    fn node_count(&self) -> usize {
        todo!()
    }
}

impl<'id, N: DirNode, W: Debug> EdgeCount for DirGraph<'id, N, W> {
    fn edge_count(&self) -> usize {
        todo!()
    }
}

impl<'id, N: DirNode, W: Debug> InsertNode for DirGraph<'id, N, W> {
    // TODO use atomic write
    fn insert_node(&mut self, weight: Self::NodeWeight) -> NodeId<Self::Key> {
        match weight.kind() {
            DirNodeKind::File => {
                let mut file = self.dir.create(weight.name()).unwrap();
                file.write_all(weight.contents()).unwrap();
            }
            DirNodeKind::Dir => {
                let dir = self.dir.create_dir(weight.name()).unwrap();
            }
        }

        NodeId::new_unchecked(&self.dir.canonicalize(weight.name()).unwrap())
    }

    fn reserve_nodes(&mut self, _: usize) {}
}

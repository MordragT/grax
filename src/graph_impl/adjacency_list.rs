use super::{BaseGraph, EdgeIndex, NodeIndex};
use crate::{
    edge_list::EdgeList,
    graph::{Clear, Extend, Get, GetMut, Graph, IndexAdjacent, Insert, Remove},
    prelude::{
        Base, Capacity, Count, Create, Directed, EdgeRef, Index, IterEdges, IterNodes,
        IterNodesMut, Reserve,
    },
};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct AdjacencyList<Node, Weight, const Di: bool = false> {
    pub(crate) base: BaseGraph<Node, Weight, Di>,
    pub(crate) adjacencies: Vec<Vec<NodeIndex>>,
}

impl<Node, Weight, const Di: bool> AdjacencyList<Node, Weight, Di> {
    pub fn new() -> Self {
        Self::with_capacity(0, 0)
    }
}

impl<Weight: Copy, const Di: bool> From<EdgeList<usize, Weight, Di>>
    for AdjacencyList<usize, Weight, Di>
{
    fn from(edge_list: EdgeList<usize, Weight, Di>) -> Self {
        let EdgeList {
            parents,
            children,
            weights,
            node_count,
        } = edge_list;

        let mut adj_list = Self::with_capacity(node_count, parents.len());

        for ((from, to), weight) in parents
            .into_iter()
            .zip(children.into_iter())
            .zip(weights.into_iter())
        {
            adj_list.base.nodes[from] = from;
            adj_list.base.nodes[to] = to;

            let edge_id = EdgeIndex::new(NodeIndex(from), NodeIndex(to));

            if !Di {
                adj_list.insert_edge(edge_id.rev(), weight);
            }

            adj_list.insert_edge(edge_id, weight);
        }

        adj_list
    }
}

impl<Node, Weight, const Di: bool> Base for AdjacencyList<Node, Weight, Di> {
    type EdgeId = EdgeIndex;
    type NodeId = NodeIndex;
}

impl<Node, Weight, const Di: bool> Capacity for AdjacencyList<Node, Weight, Di> {
    fn edges_capacity(&self) -> usize {
        self.base.edges_capacity()
    }

    fn nodes_capacity(&self) -> usize {
        self.base.nodes_capacity()
    }
}

impl<Node, Weight, const Di: bool> Clear for AdjacencyList<Node, Weight, Di> {
    fn clear(&mut self) {
        self.base.clear();
        self.adjacencies.clear();
    }
}

impl<Node, Weight, const Di: bool> Count for AdjacencyList<Node, Weight, Di> {
    fn edge_count(&self) -> usize {
        self.base.edge_count()
    }

    fn node_count(&self) -> usize {
        self.base.node_count()
    }
}

impl<Node, Weight, const Di: bool> Create<Node> for AdjacencyList<Node, Weight, Di> {
    fn with_capacity(nodes: usize, edges: usize) -> Self {
        let base = BaseGraph::with_capacity(nodes, edges);
        let adjacencies = Vec::with_capacity(nodes);

        Self { base, adjacencies }
    }

    fn with_nodes(nodes: impl Iterator<Item = Node>) -> Self {
        let base = BaseGraph::with_nodes(nodes);
        let adjacencies = vec![Vec::new(); base.node_count()];

        Self { base, adjacencies }
    }
}

impl<Node, Weight, const Di: bool> Directed for AdjacencyList<Node, Weight, Di> {
    fn directed(&self) -> bool {
        Di
    }
}

impl<Node, Weight, const Di: bool> Extend<Node, Weight> for AdjacencyList<Node, Weight, Di> {
    fn extend_edges(&mut self, edges: impl Iterator<Item = (Self::EdgeId, Weight)>) {
        for (edge_id, weight) in edges {
            self.insert_edge(edge_id, weight);
        }
    }

    fn extend_nodes(&mut self, nodes: impl Iterator<Item = Node>) {
        for node in nodes {
            self.add_node(node);
        }
    }
}

impl<Node, Weight, const Di: bool> Get<Node, Weight> for AdjacencyList<Node, Weight, Di> {
    fn node(&self, node_id: Self::NodeId) -> Option<&Node> {
        self.base.node(node_id)
    }
    fn weight(&self, edge_id: Self::EdgeId) -> Option<&Weight> {
        self.base.weight(edge_id)
    }
}

impl<Node, Weight, const Di: bool> GetMut<Node, Weight> for AdjacencyList<Node, Weight, Di> {
    fn node_mut(&mut self, node_id: Self::NodeId) -> Option<&mut Node> {
        self.base.node_mut(node_id)
    }
    fn weight_mut(&mut self, edge_id: Self::EdgeId) -> Option<&mut Weight> {
        self.base.weight_mut(edge_id)
    }
}

impl<Node, Weight, const Di: bool> Index for AdjacencyList<Node, Weight, Di> {
    type EdgeIndices<'a> = impl Iterator<Item = EdgeIndex> + 'a
    where Self: 'a;
    type NodeIndices<'a> = impl Iterator<Item = NodeIndex> + 'a
    where Self: 'a;

    fn edge_indices<'a>(&'a self) -> Self::EdgeIndices<'a> {
        self.base.edge_indices()
    }

    fn node_indices<'a>(&'a self) -> Self::NodeIndices<'a> {
        self.base.node_indices()
    }
}

impl<Node, Weight, const Di: bool> IndexAdjacent for AdjacencyList<Node, Weight, Di> {
    type AdjacentEdgeIndices<'a> = impl Iterator<Item = EdgeIndex> + 'a
    where Self: 'a;
    type AdjacentNodeIndices<'a> = impl Iterator<Item = NodeIndex> + 'a
    where Self: 'a;

    fn adjacent_edge_indices<'a>(&'a self, node_id: Self::NodeId) -> Self::AdjacentEdgeIndices<'a> {
        self.adjacent_node_indices(node_id)
            .map(move |to| EdgeIndex::new(node_id, to))
    }
    fn adjacent_node_indices<'a>(&'a self, node_id: Self::NodeId) -> Self::AdjacentNodeIndices<'a> {
        self.adjacencies[node_id.0].iter().cloned()
    }
}

impl<Node, Weight, const Di: bool> Insert<Node, Weight> for AdjacencyList<Node, Weight, Di> {
    fn add_node(&mut self, node: Node) -> Self::NodeId {
        let index = self.base.add_node(node);
        self.adjacencies.push(Vec::new());
        index
    }
    fn insert_edge(&mut self, edge_id: Self::EdgeId, weight: Weight) -> Option<Weight> {
        self.adjacencies[edge_id.from.0].push(edge_id.to);
        self.base.insert_edge(edge_id, weight)
    }
}

impl<Node, Weight, const Di: bool> IterEdges<Weight> for AdjacencyList<Node, Weight, Di> {
    type Edges<'a> = impl Iterator<Item = EdgeRef<'a, EdgeIndex, Weight>> + 'a
    where
        Weight: 'a,
        Self: 'a;

    fn iter_edges<'a>(&'a self) -> Self::Edges<'a> {
        self.base.iter_edges()
    }
}

impl<Node, Weight, const Di: bool> IterNodes<Node> for AdjacencyList<Node, Weight, Di> {
    type Nodes<'a> = impl Iterator<Item = &'a Node> + 'a
    where
        Node: 'a,
        Self: 'a;

    fn iter_nodes<'a>(&'a self) -> Self::Nodes<'a> {
        self.base.iter_nodes()
    }
}

impl<Node, Weight, const Di: bool> IterNodesMut<Node> for AdjacencyList<Node, Weight, Di> {
    type NodesMut<'a> = impl Iterator<Item = &'a mut Node> + 'a
    where
        Node: 'a,
        Self: 'a;

    fn iter_nodes_mut<'a>(&'a mut self) -> Self::NodesMut<'a> {
        self.base.iter_nodes_mut()
    }
}

impl<Node, Weight, const Di: bool> Remove<Node, Weight> for AdjacencyList<Node, Weight, Di> {
    fn remove_node(&mut self, node_id: Self::NodeId) -> Option<Node> {
        todo!()
    }

    fn remove_edge(&mut self, edge_id: Self::EdgeId) -> Option<Weight> {
        todo!()
    }
}

impl<Node, Weight, const Di: bool> Reserve for AdjacencyList<Node, Weight, Di> {
    fn reserve_edges(&mut self, additional: usize) {
        self.base.reserve_edges(additional)
    }

    fn reserve_nodes(&mut self, additional: usize) {
        self.base.reserve_nodes(additional)
    }
}

impl<Node: crate::graph::Node, Weight: crate::graph::Weight, const Di: bool> Graph<Node, Weight>
    for AdjacencyList<Node, Weight, Di>
{
}

// #[cfg(test)]
// mod test {
//     extern crate test;

//     use crate::prelude::*;
//     use std::str::FromStr;

//     #[test]
//     fn add_node() {
//         let mut graph = AdjacencyList::<u32, ()>::new();
//         let _idx1 = graph.add_node(1);
//         let _idx2 = graph.add_node(2);
//         let _idx3 = graph.add_node(3);

//         graph.contains_node(&1).unwrap();
//         graph.contains_node(&2).unwrap();
//         graph.contains_node(&3).unwrap();

//         assert!(graph.contains_node(&100).is_none());
//     }

//     #[test]
//     fn update_node() {
//         let mut graph = AdjacencyList::<u32, ()>::new();
//         let idx1 = graph.add_node(1);

//         assert_eq!(graph.update_node(idx1, 5), 1);

//         graph.contains_node(&5).unwrap();
//         assert!(graph.contains_node(&1).is_none());
//     }

//     #[test]
//     fn add_edge() {
//         let mut graph = AdjacencyList::<u32, ()>::new();
//         let idx1 = graph.add_node(1);
//         let idx2 = graph.add_node(2);
//         let _idx3 = graph.add_node(3);

//         let _ = graph.add_edge(idx1, idx2, ()).unwrap();

//         graph.contains_edge(idx1, idx2).unwrap();
//         //graph.contains_edge(idx2, idx1).unwrap();

//         assert!(graph.contains_edge(idx2, idx1).is_none());
//     }

//     #[test]
//     fn update_edge() {
//         let mut graph = AdjacencyList::<u32, u32>::new();
//         let idx1 = graph.add_node(1);
//         let idx2 = graph.add_node(2);

//         let edge = graph.add_edge(idx1, idx2, 2).unwrap();

//         assert_eq!(graph.update_edge(edge, 5), 2);
//         assert_eq!(graph.weight(edge), &5);
//     }

//     #[test]
//     fn from_edge_list() {
//         let edge_list = "4
//         0 2
//         1 2
//         2 3
//         3 1";
//         let edge_list = EdgeList::from_str(&edge_list).unwrap();
//         let graph = AdjacencyList::<usize, ()>::try_from(edge_list).unwrap();

//         assert_eq!(graph.node_count(), 4);

//         let idx0 = graph.contains_node(&0).unwrap();
//         let idx1 = graph.contains_node(&1).unwrap();
//         let idx2 = graph.contains_node(&2).unwrap();
//         let idx3 = graph.contains_node(&3).unwrap();

//         graph.contains_edge(idx0, idx2).unwrap();
//         graph.contains_edge(idx1, idx2).unwrap();
//         graph.contains_edge(idx2, idx3).unwrap();
//         graph.contains_edge(idx3, idx1).unwrap();

//         graph.contains_edge(idx2, idx0).unwrap();
//         graph.contains_edge(idx2, idx1).unwrap();
//         graph.contains_edge(idx3, idx2).unwrap();
//         graph.contains_edge(idx1, idx3).unwrap();

//         assert!(graph.contains_edge(idx1, idx0).is_none());
//     }

//     #[test]
//     fn djikstra() {
//         let edge_list = EdgeList::with(
//             [
//                 (0, 1, 1.0),
//                 (0, 2, 3.0),
//                 (1, 2, 1.0),
//                 (2, 3, 4.0),
//                 (3, 0, 1.5),
//             ]
//             .into_iter(),
//             4,
//         );

//         let graph = AdjacencyList::<usize, f32>::try_from(edge_list).unwrap();
//         let dist = graph.dijkstra_between(NodeIndex(0), NodeIndex(2));

//         assert_eq!(dist, Some(2.0));
//     }
// }

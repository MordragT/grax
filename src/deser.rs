use crate::{
    error::GraphError,
    structure::{AdjacencyList, GraphDataProvider},
    Direction, GraphKind, NodeIndex,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct EdgeListOptions {
    pub weighted: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EdgeList {
    list: String,
    options: EdgeListOptions,
}

impl EdgeList {
    pub fn new(list: &str, options: EdgeListOptions) -> Self {
        Self {
            list: list.to_owned(),
            options,
        }
    }
}

impl<const KIND: GraphKind> TryFrom<EdgeList> for AdjacencyList<KIND, usize, f64> {
    type Error = GraphError;

    fn try_from(edge_list: EdgeList) -> Result<Self, Self::Error> {
        if !edge_list.options.weighted {
            return Err(GraphError::BadEdgeListFormat);
        }

        let mut lines = edge_list.list.lines();

        let nodes_len = lines.next().ok_or(GraphError::BadEdgeListFormat)?;
        let nodes_len = usize::from_str_radix(nodes_len, 10)?;

        let mut graph = Self {
            nodes: vec![0; nodes_len],
            adjacencies: vec![HashSet::new(); nodes_len],
            edges: HashMap::new(),
        };

        for line in lines {
            let mut split = line.split_whitespace();
            let left = split.next().ok_or(GraphError::BadEdgeListFormat)?;
            let right = split.next().ok_or(GraphError::BadEdgeListFormat)?;
            let weight = split.next().ok_or(GraphError::BadEdgeListFormat)?;

            let left = usize::from_str_radix(left, 10)?;
            let right = usize::from_str_radix(right, 10)?;
            let left_idx = NodeIndex(left);
            let right_idx = NodeIndex(right);

            let weight = weight.parse()?;

            // panics if out of range
            graph.nodes[left] = left;
            graph.nodes[right] = right;

            match KIND {
                GraphKind::Directed => {
                    graph.add_edge(left_idx, right_idx, weight, Direction::Outgoing)?;
                }
                GraphKind::Undirected => {
                    graph.add_edge(left_idx, right_idx, weight, Direction::Outgoing)?;
                    graph.add_edge(left_idx, right_idx, weight, Direction::Incoming)?;
                }
            }
        }

        Ok(graph)
    }
}

impl<const KIND: GraphKind> TryFrom<EdgeList> for AdjacencyList<KIND, usize, ()> {
    type Error = GraphError;

    fn try_from(edge_list: EdgeList) -> Result<Self, Self::Error> {
        if edge_list.options.weighted {
            return Err(GraphError::BadEdgeListFormat);
        }

        let mut lines = edge_list.list.lines();

        let nodes_len = lines.next().ok_or(GraphError::BadEdgeListFormat)?;
        let nodes_len = usize::from_str_radix(nodes_len, 10)?;

        let mut graph = Self {
            nodes: vec![0; nodes_len],
            adjacencies: vec![HashSet::new(); nodes_len],
            edges: HashMap::new(),
        };

        for line in lines {
            let mut split = line.split_whitespace();
            let left = split.next().ok_or(GraphError::BadEdgeListFormat)?;
            let right = split.next().ok_or(GraphError::BadEdgeListFormat)?;

            let left = usize::from_str_radix(left, 10)?;
            let right = usize::from_str_radix(right, 10)?;
            let left_idx = NodeIndex(left);
            let right_idx = NodeIndex(right);

            // panics if out of range
            graph.nodes[left] = left;
            graph.nodes[right] = right;

            match KIND {
                GraphKind::Directed => {
                    graph.add_edge(left_idx, right_idx, (), Direction::Outgoing)?;
                }
                GraphKind::Undirected => {
                    graph.add_edge(left_idx, right_idx, (), Direction::Outgoing)?;
                    graph.add_edge(left_idx, right_idx, (), Direction::Incoming)?;
                }
            }
        }

        Ok(graph)
    }
}

#[cfg(test)]
mod test {
    use super::{EdgeList, EdgeListOptions};
    use crate::{structure::AdjacencyList, GraphKind};
    use std::fs;

    #[test]
    fn unweighted() {
        let edge_list = fs::read_to_string("data/Graph_gross.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: false });
        let _adj_list =
            AdjacencyList::<{ GraphKind::Directed }, usize, ()>::try_from(edge_list.clone())
                .unwrap();
        let _adj_list =
            AdjacencyList::<{ GraphKind::Undirected }, usize, ()>::try_from(edge_list).unwrap();
    }

    #[test]
    fn weighted() {
        let edge_list = fs::read_to_string("data/G_1_200.txt").unwrap();
        let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: true });
        let _adj_list =
            AdjacencyList::<{ GraphKind::Directed }, usize, f64>::try_from(edge_list.clone())
                .unwrap();
        let _adj_list =
            AdjacencyList::<{ GraphKind::Undirected }, usize, f64>::try_from(edge_list).unwrap();
    }
}

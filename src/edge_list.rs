use crate::{
    adjacency_list::AdjacencyList, error::GraphError, graph::data_provider::GraphDataProvider,
    Direction, GraphKind, NodeIndex,
};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EdgeList<N, W> {
    parents: Vec<N>,
    children: Vec<N>,
    weights: Vec<W>,
    node_count: usize,
}

impl<N, W> EdgeList<N, W> {
    pub fn with(list: impl Iterator<Item = (N, N, W)>, node_count: usize) -> Self {
        let ((parents, children), weights) =
            list.map(|(from, to, weight)| ((from, to), weight)).unzip();

        Self {
            parents,
            children,
            weights,
            node_count,
        }
    }
}

impl FromStr for EdgeList<usize, ()> {
    type Err = GraphError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let node_count = lines.next().ok_or(GraphError::BadEdgeListFormat)?;
        let node_count = usize::from_str_radix(node_count, 10)?;

        let edge_list = lines
            .map(|line| -> Result<(usize, usize, ()), Self::Err> {
                let mut split = line.split_whitespace();
                let from = split.next().ok_or(GraphError::BadEdgeListFormat)?;
                let to = split.next().ok_or(GraphError::BadEdgeListFormat)?;

                let from = from.parse::<usize>()?;
                let to = to.parse::<usize>()?;
                Ok((from, to, ()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::with(edge_list.into_iter(), node_count))
    }
}

impl FromStr for EdgeList<usize, f64> {
    type Err = GraphError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let node_count = lines.next().ok_or(GraphError::BadEdgeListFormat)?;
        let node_count = usize::from_str_radix(node_count, 10)?;

        let edge_list = lines
            .map(|line| -> Result<(usize, usize, f64), Self::Err> {
                let mut split = line.split_whitespace();
                let from = split.next().ok_or(GraphError::BadEdgeListFormat)?;
                let to = split.next().ok_or(GraphError::BadEdgeListFormat)?;
                let weight = split.next().ok_or(GraphError::BadEdgeListFormat)?;

                let from = from.parse::<usize>()?;
                let to = to.parse::<usize>()?;
                let weight = weight.parse::<f64>()?;
                Ok((from, to, weight))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::with(edge_list.into_iter(), node_count))
    }
}

impl<const KIND: GraphKind, W: Default + Copy> TryFrom<EdgeList<usize, W>>
    for AdjacencyList<KIND, usize, W>
{
    type Error = GraphError;

    fn try_from(edge_list: EdgeList<usize, W>) -> Result<Self, Self::Error> {
        let EdgeList {
            parents,
            children,
            weights,
            node_count,
        } = edge_list;

        let mut adj_list = AdjacencyList::with_nodes(vec![0; node_count]);

        for ((from, to), weight) in parents
            .into_iter()
            .zip(children.into_iter())
            .zip(weights.into_iter())
        {
            adj_list.nodes[from] = from;
            adj_list.nodes[to] = to;

            let from_idx = NodeIndex(from);
            let to_idx = NodeIndex(to);

            if KIND == GraphKind::Undirected {
                adj_list.add_edge(from_idx, to_idx, weight, Direction::Incoming)?;
            }
            adj_list.add_edge(from_idx, to_idx, weight, Direction::Outgoing)?;
        }

        Ok(adj_list)
    }
}

#[cfg(test)]
mod test {
    use super::EdgeList;
    use crate::{adjacency_list::AdjacencyList, GraphKind};
    use std::{fs, str::FromStr};

    #[test]
    fn unweighted() {
        let edge_list = fs::read_to_string("data/Graph_gross.txt").unwrap();
        let edge_list = EdgeList::from_str(&edge_list).unwrap();
        let _adj_list =
            AdjacencyList::<{ GraphKind::Directed }, usize, ()>::try_from(edge_list.clone())
                .unwrap();
        let _adj_list =
            AdjacencyList::<{ GraphKind::Undirected }, usize, ()>::try_from(edge_list).unwrap();
    }

    #[test]
    fn weighted() {
        let edge_list = fs::read_to_string("data/G_1_200.txt").unwrap();
        let edge_list = EdgeList::from_str(&edge_list).unwrap();
        let _adj_list =
            AdjacencyList::<{ GraphKind::Directed }, usize, f64>::try_from(edge_list.clone())
                .unwrap();
        let _adj_list =
            AdjacencyList::<{ GraphKind::Undirected }, usize, f64>::try_from(edge_list).unwrap();
    }
}

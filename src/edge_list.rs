use crate::{
    error::GraphError,
    graph::{BalancedNode, FlowWeight},
    structures::SparseMatrix,
};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EdgeList<N, W, const DI: bool = false> {
    pub(crate) nodes: Vec<N>,
    pub(crate) edges: SparseMatrix<W>,
    pub(crate) node_count: usize,
}

impl<W, const DI: bool> EdgeList<usize, W, DI> {
    pub fn with(list: impl Iterator<Item = (usize, usize, W)>, node_count: usize) -> Self {
        let nodes = (0..node_count).collect();
        let mut edges = SparseMatrix::with_capacity(node_count, node_count);

        for (parent, child, weight) in list {
            edges.insert(parent, child, weight);
        }

        Self {
            nodes,
            edges,
            node_count,
        }
    }
}

impl<const DI: bool> FromStr for EdgeList<BalancedNode<usize, f64>, FlowWeight<f64>, DI> {
    type Err = GraphError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let node_count = lines.next().ok_or(GraphError::BadEdgeListFormat)?;
        let node_count = usize::from_str_radix(node_count, 10)?;

        let nodes = (0..node_count)
            .map(|node_id| -> Result<BalancedNode<usize, f64>, GraphError> {
                let balance_str = lines.next().ok_or(GraphError::BadEdgeListFormat)?;
                let balance = balance_str.parse::<f64>()?;
                let node = BalancedNode::new(node_id, balance);
                Ok(node)
            })
            .collect::<Result<_, _>>()?;

        let mut edges = SparseMatrix::with_capacity(node_count, node_count);

        for line in lines {
            let mut split = line.split_whitespace();
            let from = split.next().ok_or(GraphError::BadEdgeListFormat)?;
            let to = split.next().ok_or(GraphError::BadEdgeListFormat)?;
            let cost = split.next().ok_or(GraphError::BadEdgeListFormat)?;
            let capacity = split.next().ok_or(GraphError::BadEdgeListFormat)?;

            let from = from.parse::<usize>()?;
            let to = to.parse::<usize>()?;
            let cost = cost.parse::<f64>()?;
            let capacity = capacity.parse::<f64>()?;

            edges.insert(from, to, FlowWeight::new(capacity, cost, 0.0));
        }

        Ok(Self {
            nodes,
            edges,
            node_count,
        })
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

impl<const DI: bool> FromStr for EdgeList<usize, f64, DI> {
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
                let weight: f64 = weight.parse::<f64>()?;
                Ok((from, to, weight))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::with(edge_list.into_iter(), node_count))
    }
}

impl<const DI: bool> FromStr for EdgeList<usize, f32, DI> {
    type Err = GraphError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let node_count = lines.next().ok_or(GraphError::BadEdgeListFormat)?;
        let node_count = usize::from_str_radix(node_count, 10)?;

        let edge_list = lines
            .map(|line| -> Result<(usize, usize, f32), Self::Err> {
                let mut split = line.split_whitespace();
                let from = split.next().ok_or(GraphError::BadEdgeListFormat)?;
                let to = split.next().ok_or(GraphError::BadEdgeListFormat)?;
                let weight = split.next().ok_or(GraphError::BadEdgeListFormat)?;

                let from = from.parse::<usize>()?;
                let to = to.parse::<usize>()?;
                let weight = weight.parse::<f32>()?;
                Ok((from, to, weight))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::with(edge_list.into_iter(), node_count))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        graph::{BalancedNode, FlowWeight},
        prelude::AdjacencyList,
    };

    use super::EdgeList;
    use std::{fs, str::FromStr};

    #[test]
    fn unweighted() {
        let edge_list = fs::read_to_string("data/Graph_gross.txt").unwrap();
        let edge_list = EdgeList::from_str(&edge_list).unwrap();
        let _adj_list = AdjacencyList::<usize, ()>::try_from(edge_list).unwrap();
    }

    #[test]
    fn weighted() {
        let edge_list = fs::read_to_string("data/G_1_200.txt").unwrap();
        let edge_list = EdgeList::from_str(&edge_list).unwrap();
        let _adj_list = AdjacencyList::<usize, f64>::try_from(edge_list).unwrap();
    }
    #[test]
    fn directed() {
        let edge_list = fs::read_to_string("data/G_1_200.txt").unwrap();
        let edge_list = EdgeList::from_str(&edge_list).unwrap();
        let _adj_list = AdjacencyList::<usize, f64, true>::try_from(edge_list).unwrap();
    }

    #[test]
    fn balanced() {
        let edge_list = fs::read_to_string("data/Kostenminimal1.txt").unwrap();
        let edge_list = EdgeList::from_str(&edge_list).unwrap();
        let _adj_list =
            AdjacencyList::<BalancedNode<usize, f64>, FlowWeight<f64>, true>::try_from(edge_list)
                .unwrap();
    }
}

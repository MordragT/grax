use crate::error::GraphError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EdgeList<N, W, const DIRECTED: bool = false> {
    pub(crate) parents: Vec<N>,
    pub(crate) children: Vec<N>,
    pub(crate) weights: Vec<W>,
    pub(crate) node_count: usize,
}

impl<N, W, const DIRECTED: bool> EdgeList<N, W, DIRECTED> {
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

impl<const DIRECTED: bool> FromStr for EdgeList<usize, f64, DIRECTED> {
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

impl<const DIRECTED: bool> FromStr for EdgeList<usize, f32, DIRECTED> {
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
    use crate::prelude::AdjacencyList;

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
}

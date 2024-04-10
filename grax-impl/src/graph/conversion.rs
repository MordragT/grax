use crate::{edges::EdgeStorage, error::GraphError, nodes::NodeStorage, Graph};
use grax_core::index::NodeId;
use grax_flow::{BalancedNode, FlowBundle};
use std::{marker::PhantomData, str::FromStr};

// impl<S, N, W, const DI: bool> From<Graph<S, N, W, DI>> for StableGraph<S, N, W, DI>
// where
//     N: Clone,
//     S: StableEdgeStorage<W>,
// {
//     fn from(graph: Graph<S, N, W, DI>) -> Self {
//         let Graph {
//             nodes,
//             edges,
//             weight,
//         } = graph;

//         let nodes = StableVec::from(nodes);

//         Self {
//             nodes,
//             edges,
//             weight,
//         }
//     }
// }

impl<NS, ES, const DI: bool> FromStr
    for Graph<NS, ES, BalancedNode<usize, f64>, FlowBundle<f64, f64>, DI>
where
    NS: NodeStorage<usize, BalancedNode<usize, f64>>,
    ES: EdgeStorage<usize, FlowBundle<f64, f64>>,
{
    type Err = GraphError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let node_count = lines.next().ok_or(GraphError::BadEdgeListFormat)?;
        let node_count = usize::from_str_radix(node_count, 10)?;

        let nodes = (0..node_count)
            .map(|node_id| -> Result<BalancedNode<usize, f64>, GraphError> {
                let balance_str = lines.next().ok_or(GraphError::BadEdgeListFormat)?;
                let balance = balance_str.parse::<f64>()?;
                let node_weight = BalancedNode::new(node_id, balance);
                Ok(node_weight)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let nodes = NS::with_nodes(node_count, nodes);

        let edges = lines
            .map(|line| -> Result<(NodeId<_>, NodeId<_>, _), Self::Err> {
                let mut split = line.split_whitespace();
                let from = split.next().ok_or(GraphError::BadEdgeListFormat)?;
                let to = split.next().ok_or(GraphError::BadEdgeListFormat)?;
                let cost = split.next().ok_or(GraphError::BadEdgeListFormat)?;
                let capacity = split.next().ok_or(GraphError::BadEdgeListFormat)?;

                let from = NodeId::new_unchecked(from.parse::<usize>()?);
                let to = NodeId::new_unchecked(to.parse::<usize>()?);
                let cost = cost.parse::<f64>()?;
                let capacity = capacity.parse::<f64>()?;

                let bundle = FlowBundle {
                    weight: cost,
                    capacity,
                    cost,
                    flow: 0.0,
                    reverse: false,
                };

                Ok((from, to, bundle))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let edges = if DI {
            ES::with_edges(node_count, edges.len(), edges)
        } else {
            ES::with_edges(
                node_count,
                edges.len(),
                edges
                    .into_iter()
                    .flat_map(|(from, to, weight)| [(from, to, weight), (to, from, weight)]),
            )
        };

        Ok(Self {
            nodes,
            edges,
            edge_weight: PhantomData,
            node_weight: PhantomData,
        })
    }
}

impl<NS, ES, const DI: bool> FromStr for Graph<NS, ES, usize, (), DI>
where
    NS: NodeStorage<usize, usize>,
    ES: EdgeStorage<usize, ()>,
{
    type Err = GraphError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let node_count = lines.next().ok_or(GraphError::BadEdgeListFormat)?;
        let node_count = usize::from_str_radix(node_count, 10)?;

        let nodes = NS::with_nodes(node_count, 0..node_count);

        let edges = lines
            .map(|line| -> Result<(NodeId<_>, NodeId<_>, ()), Self::Err> {
                let mut split = line.split_whitespace();
                let from = split.next().ok_or(GraphError::BadEdgeListFormat)?;
                let to = split.next().ok_or(GraphError::BadEdgeListFormat)?;

                let from = NodeId::new_unchecked(from.parse::<usize>()?);
                let to = NodeId::new_unchecked(to.parse::<usize>()?);

                Ok((from, to, ()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let edges = if DI {
            ES::with_edges(node_count, edges.len(), edges)
        } else {
            ES::with_edges(
                node_count,
                edges.len(),
                edges
                    .into_iter()
                    .flat_map(|(from, to, weight)| [(from, to, weight), (to, from, weight)]),
            )
        };

        Ok(Self {
            nodes,
            edges,
            edge_weight: PhantomData,
            node_weight: PhantomData,
        })
    }
}

impl<NS, ES, const DI: bool> FromStr for Graph<NS, ES, usize, f64, DI>
where
    NS: NodeStorage<usize, usize>,
    ES: EdgeStorage<usize, f64>,
{
    type Err = GraphError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let node_count = lines.next().ok_or(GraphError::BadEdgeListFormat)?;
        let node_count = usize::from_str_radix(node_count, 10)?;

        let nodes = NS::with_nodes(node_count, 0..node_count);

        let edges = lines
            .map(|line| -> Result<(NodeId<_>, NodeId<_>, f64), Self::Err> {
                let mut split = line.split_whitespace();
                let from = split.next().ok_or(GraphError::BadEdgeListFormat)?;
                let to = split.next().ok_or(GraphError::BadEdgeListFormat)?;
                let weight = split.next().ok_or(GraphError::BadEdgeListFormat)?;

                let from = NodeId::new_unchecked(from.parse::<usize>()?);
                let to = NodeId::new_unchecked(to.parse::<usize>()?);
                let weight = weight.parse::<f64>()?;

                Ok((from, to, weight))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let edges = if DI {
            ES::with_edges(node_count, edges.len(), edges)
        } else {
            ES::with_edges(
                node_count,
                edges.len(),
                edges
                    .into_iter()
                    .flat_map(|(from, to, weight)| [(from, to, weight), (to, from, weight)]),
            )
        };

        Ok(Self {
            nodes,
            edges,
            edge_weight: PhantomData,
            node_weight: PhantomData,
        })
    }
}

impl<NS, ES, const DI: bool> FromStr for Graph<NS, ES, usize, f32, DI>
where
    NS: NodeStorage<usize, usize>,
    ES: EdgeStorage<usize, f32>,
{
    type Err = GraphError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let node_count = lines.next().ok_or(GraphError::BadEdgeListFormat)?;
        let node_count = usize::from_str_radix(node_count, 10)?;

        let nodes = NS::with_nodes(node_count, 0..node_count);

        let edges = lines
            .map(|line| -> Result<(NodeId<_>, NodeId<_>, f32), Self::Err> {
                let mut split = line.split_whitespace();
                let from = split.next().ok_or(GraphError::BadEdgeListFormat)?;
                let to = split.next().ok_or(GraphError::BadEdgeListFormat)?;
                let weight = split.next().ok_or(GraphError::BadEdgeListFormat)?;

                let from = NodeId::new_unchecked(from.parse::<usize>()?);
                let to = NodeId::new_unchecked(to.parse::<usize>()?);
                let weight = weight.parse::<f32>()?;

                Ok((from, to, weight))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let edges = if DI {
            ES::with_edges(node_count, edges.len(), edges)
        } else {
            ES::with_edges(
                node_count,
                edges.len(),
                edges
                    .into_iter()
                    .flat_map(|(from, to, weight)| [(from, to, weight), (to, from, weight)]),
            )
        };

        Ok(Self {
            nodes,
            edges,
            edge_weight: PhantomData,
            node_weight: PhantomData,
        })
    }
}

#[cfg(test)]
mod test {
    use grax_flow::{BalancedNode, FlowBundle};

    use crate::AdjGraph;
    use std::{fs, str::FromStr};

    #[test]
    fn unweighted_adj_graph() {
        let edge_list = fs::read_to_string("../data/Graph_gross.txt").unwrap();
        let _adj_list = AdjGraph::<usize, ()>::from_str(&edge_list).unwrap();
    }

    #[test]
    fn weighted_adj_graph() {
        let edge_list = fs::read_to_string("../data/G_1_200.txt").unwrap();
        let _adj_list = AdjGraph::<usize, f64>::from_str(&edge_list).unwrap();
    }

    #[test]
    fn directed_adj_graph() {
        let edge_list = fs::read_to_string("../data/G_1_200.txt").unwrap();
        let _adj_list = AdjGraph::<usize, f64, true>::from_str(&edge_list).unwrap();
    }

    #[test]
    fn balanced() {
        let edge_list = fs::read_to_string("../data/Kostenminimal1.txt").unwrap();
        let _adj_list =
            AdjGraph::<BalancedNode<usize, f64>, FlowBundle<f64, f64>, true>::from_str(&edge_list)
                .unwrap();
    }
}

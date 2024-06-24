use crate::{edges::EdgeStorage, error::GraphError, nodes::NodeStorage, Graph};
use grax_core::{
    edge::weight::{FlowCostBundle, Reverse},
    index::NodeId,
};
use std::{marker::PhantomData, str::FromStr};

// pub trait ParseWeight: Sized {
//     const SIZE: usize;

//     fn parse_weight<'a>(values: &'a [&'a str]) -> Result<Self, GraphError>;
// }

// impl<T> ParseWeight for FlowCostBundle<T>
// where
//     T: FromStr + Default,
//     GraphError: From<T::Err>,
// {
//     const SIZE: usize = 2;

//     fn parse_weight<'a>(values: &'a [&'a str]) -> Result<Self, GraphError> {
//         let [cost, capacity, ..] = values else {
//             return Err(GraphError::BadEdgeListFormat);
//         };

//         let cost = cost.parse::<T>()?;
//         let capacity = capacity.parse::<T>()?;

//         Ok(Self {
//             capacity,
//             cost,
//             flow: T::default(),
//             reverse: false,
//         })
//     }
// }

// impl<N, W, NS, ES, const DI: bool> FromStr for Graph<NS, ES, N, W, DI>
// where
//     N: ParseWeight + Debug,
//     W: ParseWeight + Debug,
//     NS: NodeStorage<usize, N>,
//     ES: EdgeStorage<usize, W>,
// {
//     type Err = GraphError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let mut splitted = s.split_whitespace();

//         let node_count = splitted.next().ok_or(GraphError::BadEdgeListFormat)?;
//         let node_count = usize::from_str_radix(node_count, 10)?;

//         let nodes = splitted
//             .by_ref()
//             .take(node_count * N::SIZE)
//             .array_chunks::<{ N::SIZE }>()
//             .map(N::parse_weight)
//             .collect::<Result<Vec<_>, _>>()?;
//         let nodes = NS::with_nodes(node_count, nodes);

//         let edges = splitted
//             .array_chunks::<{ 2 + W::SIZE }>()
//             .map(
//                 |[from, to, weight @ ..]| -> Result<(NodeId<_>, NodeId<_>, _), Self::Err> {
//                     let from = NodeId::new_unchecked(from.parse::<usize>()?);
//                     let to = NodeId::new_unchecked(to.parse::<usize>()?);
//                     let weight = W::parse_weight(&weight)?;

//                     Ok((from, to, weight))
//                 },
//             )
//             .collect::<Result<Vec<_>, _>>()?;

//         let edges = if DI {
//             ES::with_edges(node_count, edges.len(), edges)
//         } else {
//             ES::with_edges(
//                 node_count,
//                 edges.len(),
//                 edges.into_iter().flat_map(|(from, to, weight)| {
//                     [(from, to, weight.clone()), (to, from, weight.reverse())]
//                 }),
//             )
//         };

//         Ok(Self {
//             nodes,
//             edges,
//             edge_weight: PhantomData,
//             node_weight: PhantomData,
//         })
//     }
// }

impl<NS, ES, const DI: bool> FromStr for Graph<NS, ES, f64, FlowCostBundle<f64>, DI>
where
    NS: NodeStorage<usize, f64>,
    ES: EdgeStorage<usize, FlowCostBundle<f64>>,
{
    type Err = GraphError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split_whitespace();

        let node_count = splitted.next().ok_or(GraphError::BadEdgeListFormat)?;
        let node_count = usize::from_str_radix(node_count, 10)?;

        let nodes = splitted
            .by_ref()
            .take(node_count)
            .map(f64::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        let nodes = NS::with_nodes(node_count, nodes);

        let edges = splitted
            .array_chunks::<4>()
            .map(
                |[from, to, cost, capacity]| -> Result<(NodeId<_>, NodeId<_>, _), Self::Err> {
                    let from = NodeId::new_unchecked(from.parse::<usize>()?);
                    let to = NodeId::new_unchecked(to.parse::<usize>()?);
                    let cost = cost.parse::<f64>()?;
                    let capacity = capacity.parse::<f64>()?;

                    let bundle = FlowCostBundle {
                        capacity,
                        cost,
                        flow: 0.0,
                        reverse: false,
                    };

                    Ok((from, to, bundle))
                },
            )
            .collect::<Result<Vec<_>, _>>()?;

        let edges = if DI {
            ES::with_edges(node_count, edges.len(), edges)
        } else {
            ES::with_edges(
                node_count,
                edges.len(),
                edges.into_iter().flat_map(|(from, to, weight)| {
                    [(from, to, weight.clone()), (to, from, weight.reverse())]
                }),
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
        let mut splitted = s.split_whitespace();

        let node_count = splitted.next().ok_or(GraphError::BadEdgeListFormat)?;
        let node_count = usize::from_str_radix(node_count, 10)?;

        let nodes = NS::with_nodes(node_count, 0..node_count);

        let edges = splitted
            .array_chunks::<2>()
            .map(
                |[from, to]| -> Result<(NodeId<_>, NodeId<_>, ()), Self::Err> {
                    let from = NodeId::new_unchecked(from.parse::<usize>()?);
                    let to = NodeId::new_unchecked(to.parse::<usize>()?);

                    Ok((from, to, ()))
                },
            )
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
        let mut splitted = s.split_whitespace();

        let node_count = splitted.next().ok_or(GraphError::BadEdgeListFormat)?;
        let node_count = usize::from_str_radix(node_count, 10)?;

        let nodes = NS::with_nodes(node_count, 0..node_count);

        let edges = splitted
            .array_chunks::<3>()
            .map(
                |[from, to, weight]| -> Result<(NodeId<_>, NodeId<_>, f64), Self::Err> {
                    let from = NodeId::new_unchecked(from.parse::<usize>()?);
                    let to = NodeId::new_unchecked(to.parse::<usize>()?);
                    let weight = weight.parse::<f64>()?;

                    Ok((from, to, weight))
                },
            )
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
        let mut splitted = s.split_whitespace();

        let node_count = splitted.next().ok_or(GraphError::BadEdgeListFormat)?;
        let node_count = usize::from_str_radix(node_count, 10)?;

        let nodes = NS::with_nodes(node_count, 0..node_count);

        let edges = splitted
            .array_chunks::<3>()
            .map(
                |[from, to, weight]| -> Result<(NodeId<_>, NodeId<_>, f32), Self::Err> {
                    let from = NodeId::new_unchecked(from.parse::<usize>()?);
                    let to = NodeId::new_unchecked(to.parse::<usize>()?);
                    let weight = weight.parse::<f32>()?;

                    Ok((from, to, weight))
                },
            )
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

    extern crate test;

    use grax_core::edge::weight::FlowCostBundle;
    use test::Bencher;

    use crate::AdjGraph;
    use std::{fs, str::FromStr};

    #[bench]
    fn read_graph_1_2_adj_list(b: &mut Bencher) {
        let edge_list = fs::read_to_string("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            AdjGraph::<usize, f64>::from_str(&edge_list).unwrap();
        })
    }

    #[bench]
    fn read_graph_1_20_adj_list(b: &mut Bencher) {
        let edge_list = fs::read_to_string("../data/G_1_20.txt").unwrap();

        b.iter(|| {
            AdjGraph::<usize, f64>::from_str(&edge_list).unwrap();
        })
    }

    #[bench]
    fn read_graph_1_200_adj_list(b: &mut Bencher) {
        let edge_list = fs::read_to_string("../data/G_1_200.txt").unwrap();

        b.iter(|| {
            AdjGraph::<usize, f64>::from_str(&edge_list).unwrap();
        })
    }

    #[bench]
    fn read_graph_10_20_adj_list(b: &mut Bencher) {
        let edge_list = fs::read_to_string("../data/G_10_20.txt").unwrap();

        b.iter(|| {
            AdjGraph::<usize, f64>::from_str(&edge_list).unwrap();
        })
    }

    #[bench]
    fn read_graph_10_200_adj_list(b: &mut Bencher) {
        let edge_list = fs::read_to_string("../data/G_10_200.txt").unwrap();

        b.iter(|| {
            AdjGraph::<usize, f64>::from_str(&edge_list).unwrap();
        })
    }

    #[bench]
    fn read_graph_100_200_adj_list(b: &mut Bencher) {
        let edge_list = fs::read_to_string("../data/G_100_200.txt").unwrap();

        b.iter(|| {
            AdjGraph::<usize, f64>::from_str(&edge_list).unwrap();
        })
    }

    #[bench]
    fn read_digraph_1_2_adj_list(b: &mut Bencher) {
        let edge_list = fs::read_to_string("../data/G_1_2.txt").unwrap();

        b.iter(|| {
            AdjGraph::<usize, f64, true>::from_str(&edge_list).unwrap();
        })
    }

    #[bench]
    fn read_digraph_1_20_adj_list(b: &mut Bencher) {
        let edge_list = fs::read_to_string("../data/G_1_20.txt").unwrap();

        b.iter(|| {
            AdjGraph::<usize, f64, true>::from_str(&edge_list).unwrap();
        })
    }

    #[bench]
    fn read_digraph_1_200_adj_list(b: &mut Bencher) {
        let edge_list = fs::read_to_string("../data/G_1_200.txt").unwrap();

        b.iter(|| {
            AdjGraph::<usize, f64, true>::from_str(&edge_list).unwrap();
        })
    }

    #[bench]
    fn read_digraph_10_20_adj_list(b: &mut Bencher) {
        let edge_list = fs::read_to_string("../data/G_10_20.txt").unwrap();

        b.iter(|| {
            AdjGraph::<usize, f64, true>::from_str(&edge_list).unwrap();
        })
    }

    #[bench]
    fn read_digraph_10_200_adj_list(b: &mut Bencher) {
        let edge_list = fs::read_to_string("../data/G_10_200.txt").unwrap();

        b.iter(|| {
            AdjGraph::<usize, f64, true>::from_str(&edge_list).unwrap();
        })
    }

    #[bench]
    fn read_digraph_100_200_adj_list(b: &mut Bencher) {
        let edge_list = fs::read_to_string("../data/G_100_200.txt").unwrap();

        b.iter(|| {
            AdjGraph::<usize, f64, true>::from_str(&edge_list).unwrap();
        })
    }

    #[bench]
    fn read_kostenminimal1_adj_list(b: &mut Bencher) {
        let edge_list = fs::read_to_string("../data/Kostenminimal1.txt").unwrap();

        b.iter(|| {
            AdjGraph::<f64, FlowCostBundle<f64>, true>::from_str(&edge_list).unwrap();
        })
    }

    #[bench]
    fn read_kostenminimal2_adj_list(b: &mut Bencher) {
        let edge_list = fs::read_to_string("../data/Kostenminimal2.txt").unwrap();

        b.iter(|| {
            AdjGraph::<f64, FlowCostBundle<f64>, true>::from_str(&edge_list).unwrap();
        })
    }

    #[bench]
    fn read_kostenminimal3_adj_list(b: &mut Bencher) {
        let edge_list = fs::read_to_string("../data/Kostenminimal3.txt").unwrap();

        b.iter(|| {
            AdjGraph::<f64, FlowCostBundle<f64>, true>::from_str(&edge_list).unwrap();
        })
    }

    #[bench]
    fn read_kostenminimal4_adj_list(b: &mut Bencher) {
        let edge_list = fs::read_to_string("../data/Kostenminimal4.txt").unwrap();

        b.iter(|| {
            AdjGraph::<f64, FlowCostBundle<f64>, true>::from_str(&edge_list).unwrap();
        })
    }

    #[bench]
    fn read_kostenminimal_gross1_adj_list(b: &mut Bencher) {
        let edge_list = fs::read_to_string("../data/Kostenminimal_gross1.txt").unwrap();

        b.iter(|| {
            AdjGraph::<f64, FlowCostBundle<f64>, true>::from_str(&edge_list).unwrap();
        })
    }

    #[bench]
    fn read_kostenminimal_gross2_adj_list(b: &mut Bencher) {
        let edge_list = fs::read_to_string("../data/Kostenminimal_gross2.txt").unwrap();

        b.iter(|| {
            AdjGraph::<f64, FlowCostBundle<f64>, true>::from_str(&edge_list).unwrap();
        })
    }

    #[bench]
    fn read_kostenminimal_gross3_adj_list(b: &mut Bencher) {
        let edge_list = fs::read_to_string("../data/Kostenminimal_gross3.txt").unwrap();

        b.iter(|| {
            AdjGraph::<f64, FlowCostBundle<f64>, true>::from_str(&edge_list).unwrap();
        })
    }

    #[bench]
    fn read_weightless_graph_gross_adj_graph(b: &mut Bencher) {
        let edge_list = fs::read_to_string("../data/Graph_gross.txt").unwrap();

        b.iter(|| {
            AdjGraph::<usize, ()>::from_str(&edge_list).unwrap();
        })
    }
}

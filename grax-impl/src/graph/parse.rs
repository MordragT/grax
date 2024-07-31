use crate::{edges::EdgeStorage, nodes::NodeStorage, Graph};
use grax_core::{
    index::NodeId,
    parse::{ParseError, ParseGrax, ParseWeight},
};
use itertools::Itertools;
use std::{fmt::Debug, marker::PhantomData};

impl<NS, ES, N, W, const DI: bool> ParseGrax for Graph<NS, ES, N, W, DI>
where
    NS: NodeStorage<usize, N>,
    ES: EdgeStorage<usize, W>,
    N: ParseWeight + Debug + Default + Clone,
    W: ParseWeight + Debug,
{
    fn parse_grax<F>(s: &str, back_edge: F) -> Result<Self, ParseError>
    where
        F: Fn(
            NodeId<Self::Key>,
            NodeId<Self::Key>,
            Self::EdgeWeight,
        ) -> [(NodeId<Self::Key>, NodeId<Self::Key>, Self::EdgeWeight); 2],
    {
        let mut splitted = s.split_whitespace();

        let node_count = splitted.next().ok_or(ParseError::BadEdgeListFormat)?;
        let node_count = usize::from_str_radix(node_count, 10)?;

        let nodes = if N::LENGTH > 0 {
            splitted
                .by_ref()
                .take(node_count * N::LENGTH)
                .chunks(N::LENGTH)
                .into_iter()
                .map(N::parse_weight)
                .collect::<Result<Vec<_>, _>>()?
        } else {
            vec![N::default(); node_count]
        };
        let nodes = NS::with_nodes(node_count, nodes);

        let edges = splitted
            .chunks(W::LENGTH + 2)
            .into_iter()
            .map(
                |mut chunk| -> Result<(NodeId<_>, NodeId<_>, _), ParseError> {
                    let from = NodeId::new_unchecked(
                        chunk
                            .next()
                            .ok_or(ParseError::BadEdgeListFormat)?
                            .parse::<usize>()?,
                    );
                    let to = NodeId::new_unchecked(
                        chunk
                            .next()
                            .ok_or(ParseError::BadEdgeListFormat)?
                            .parse::<usize>()?,
                    );
                    let weight = W::parse_weight(chunk)?;

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
                    .flat_map(|(from, to, weight)| back_edge(from, to, weight)),
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

use std::cmp::min_by;

use num_traits::{Float, Pow};

use grax_core::prelude::*;
use grax_core::traits::*;

pub fn capcity_scaling<N, W, C, G>(graph: &G) -> Option<C>
where
    C: Float,
    W: EdgeCapacity<Capacity = C>,
    G: Iter + Base<Node = N, Weight = W>,
{
    let delta = match graph
        .iter_edges()
        .map(|edge| edge.weight.capacity())
        .cloned()
        .reduce(C::max)
    {
        Some(max) => {
            let mut delta = C::one();
            while delta.powi(2) < max {
                delta = delta.powi(2)
            }
            delta
        }
        None => return None,
    };

    // TODO find augmenting paths where all edge capacities are >= delta
    // if no more paths are found: delta = delta / 2 and repeat
    // find bottleneck of that path
    // apply bottleneck
    // when delta == 0 terminate

    todo!()
}

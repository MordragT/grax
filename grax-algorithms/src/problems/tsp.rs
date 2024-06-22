use grax_core::graph::NodeAttribute;

use crate::util::Cycle;

pub trait TspSolver<C, G>
where
    G: NodeAttribute,
{
    /// Returns depending on the implementation an exact or approximate shortest route
    /// Returns none if such cycle cannot be found
    fn solve(graph: &G) -> Option<TspCycle<C, G>>;
}

pub struct TspCycle<C, G>
where
    G: NodeAttribute,
{
    pub cost: C,
    pub cycle: Cycle<G>,
}

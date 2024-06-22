pub trait McfSolver<C, G> {
    fn solve(graph: &G) -> Option<C>;
}

use crate::graph::{Base, Get};

// TODO replace occurences with Parent when possible
#[derive(Debug)]
pub struct Tour<NodeId, Weight> {
    pub route: Vec<NodeId>,
    pub weight: Weight,
}

impl<NodeId, Weight> Tour<NodeId, Weight> {
    pub fn new(route: Vec<NodeId>, weight: Weight) -> Self {
        Self { route, weight }
    }

    pub fn edges(&self) -> impl Iterator<Item = (&NodeId, &NodeId)> {
        self.route.array_windows::<2>().map(|[from, to]| (from, to))
    }

    pub fn map<F, T>(self, mut f: F) -> Tour<NodeId, T>
    where
        F: FnMut(Weight) -> T,
    {
        let Tour { route, weight } = self;
        let weight = f(weight);
        Tour { route, weight }
    }
}

impl<NodeId: Copy, Weight> Tour<NodeId, Weight> {
    pub fn source(&self) -> Option<NodeId> {
        self.route.get(0).cloned()
    }

    pub fn sink(&self) -> Option<NodeId> {
        self.route.last().cloned()
    }

    pub fn nodes<'a, N, G>(&'a self, graph: &'a G) -> impl Iterator<Item = &'a N> + 'a
    where
        G: Get<N, Weight> + Base<NodeId = NodeId>,
    {
        self.route.iter().map(|index| graph.node(*index).unwrap())
    }
}

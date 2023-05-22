#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge<Id, Weight> {
    pub edge_id: Id,
    pub weight: Weight,
}

impl<Id, Weight> Edge<Id, Weight> {
    pub fn new(edge_id: Id, weight: Weight) -> Self {
        Self { edge_id, weight }
    }
}

impl<Id: Eq, Weight: Ord> Ord for Edge<Id, Weight> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<Id: Eq, Weight: PartialOrd> PartialOrd for Edge<Id, Weight> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EdgeRef<'a, Id, Weight> {
    pub edge_id: Id,
    pub weight: &'a Weight,
}

impl<'a, Id, Weight> EdgeRef<'a, Id, Weight> {
    pub fn new(edge_id: Id, weight: &'a Weight) -> Self {
        Self { edge_id, weight }
    }
}

impl<'a, Id: Clone, Weight: Clone> EdgeRef<'a, Id, Weight> {
    pub fn to_owned(&self) -> Edge<Id, Weight> {
        Edge::new(self.edge_id.clone(), self.weight.clone())
    }
}

impl<'a, Id: Eq, Weight: Ord> Ord for EdgeRef<'a, Id, Weight> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(other.weight)
    }
}

impl<'a, Id: Eq, Weight: PartialOrd> PartialOrd for EdgeRef<'a, Id, Weight> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(other.weight)
    }
}

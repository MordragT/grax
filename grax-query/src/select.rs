use grax_core::prelude::{EdgeRef, Identifier, NodeId};

pub struct Select<Id: Identifier> {
    pub(crate) ids: Vec<NodeId<Id>>,
}

impl<Id: Identifier> Select<Id> {
    pub fn join(mut self, id: NodeId<Id>) -> Self {
        self.ids.push(id);
        self
    }

    pub fn nodes<N>(self) -> SelectNodes<Id, N> {
        SelectNodes {
            ids: self.ids,
            filters: Vec::new(),
        }
    }

    pub fn edges<W: Send>(self) -> SelectEdges<Id, W> {
        SelectEdges {
            ids: self.ids,
            filters: Vec::new(),
        }
    }
}

pub struct SelectNodes<Id: Identifier, N> {
    ids: Vec<NodeId<Id>>,
    filters: Vec<Box<dyn Fn(&N) -> bool>>,
}

impl<Id: Identifier, N> SelectNodes<Id, N> {
    pub fn filter<F>(mut self, f: F) -> Self
    where
        F: Fn(&N) -> bool + 'static,
    {
        self.filters.push(Box::new(f));
        self
    }
}

pub struct SelectEdges<Id: Identifier, W: Send> {
    ids: Vec<NodeId<Id>>,
    filters: Vec<Box<dyn Fn(&EdgeRef<Id, W>) -> bool>>,
}

impl<Id: Identifier, W: Send> SelectEdges<Id, W> {
    pub fn filter<F>(mut self, f: F) -> Self
    where
        F: Fn(&EdgeRef<Id, W>) -> bool + 'static,
    {
        self.filters.push(Box::new(f));
        self
    }
}

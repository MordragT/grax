use std::{
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, Index, IndexMut},
};

pub trait Identifier: Hash + PartialEq + Eq + PartialOrd + Ord + Copy + Clone + Debug {}

impl<T: Hash + PartialEq + Eq + PartialOrd + Ord + Copy + Clone + Debug> Identifier for T {}

// pub struct UncheckedNodeId<Id: Identifier> {
//     id: Id,
// }

// pub struct CheckedNodeId<'a, Id: Identifier, G> {
//     id: Id,
//     lifetime: PhantomData<&'a G>,
// }

// impl<'a, Id: Identifier, G> CheckedNodeId<'a, Id, G> {
//     pub fn downcast(self) -> UncheckedNodeId<Id> {
//         UncheckedNodeId { id: self.id }
//     }
// }

// Idea 1

// maybe restrict checked NodeId not by Graph itself but by marker inside Graph
// Marker in Graph is only mutable borrowed for remove_node, insert_node and other operations potentially invalidating indices
// then hopefully mutable graph actons which dont invalidated indices will preserve the index

// Idea 2

// not be so strict but restrict Ids by generic G graph parameter so that indices of differing graphs cannot be used interchangeable.
// Also depending on the operation take ownership of indices or only borrow them and disallow copying or cloning of indices.

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct NodeId<Id: Identifier>(Id);

impl<Id: Identifier> NodeId<Id> {
    pub fn new_unchecked(id: Id) -> Self {
        Self(id)
    }

    // pub fn raw(&self) -> Id {
    //     self.0
    // }
}

// impl<Id: Identifier> Index for NodeId<Id> {
//     type Output = ;

//     fn index(&self, index: Idx) -> &Self::Output {

//     }
// }

impl<Id: Identifier> Deref for NodeId<Id> {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct EdgeId<Id: Identifier> {
    from: NodeId<Id>,
    to: NodeId<Id>,
}

impl<Id: Identifier> EdgeId<Id> {
    pub fn new_unchecked(from: NodeId<Id>, to: NodeId<Id>) -> Self {
        Self { from, to }
    }

    pub fn contains(&self, node_id: NodeId<Id>) -> bool {
        self.from == node_id || self.to == node_id
    }

    pub fn from(&self) -> NodeId<Id> {
        self.from
    }

    pub fn to(&self) -> NodeId<Id> {
        self.to
    }

    pub fn raw(&self) -> (Id, Id) {
        (*self.from, *self.to)
    }

    pub fn rev(&self) -> EdgeId<Id> {
        let Self { from, to } = self;
        Self {
            from: *to,
            to: *from,
        }
    }
}

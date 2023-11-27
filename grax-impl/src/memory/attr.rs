use std::{collections::HashMap, fmt::Debug, hash::Hash};

use grax_core::{
    prelude::NodeId,
    view::{AttrMap, VisitMap},
};

#[derive(Clone, Debug)]
pub struct AttrVec<Attr>(pub(crate) Vec<Attr>);

impl VisitMap<NodeId<usize>> for AttrVec<bool> {
    fn visit(&mut self, id: NodeId<usize>) {
        self.0[id.raw()] = true;
    }

    fn unvisit(&mut self, id: NodeId<usize>) {
        self.0[id.raw()] = false;
    }

    fn is_visited(&self, id: NodeId<usize>) -> bool {
        self.0[id.raw()]
    }

    fn all(&self) -> bool {
        self.0.iter().all(|v| *v)
    }
}

impl<Attr: Clone + Debug + Default> AttrMap<NodeId<usize>, Attr> for AttrVec<Attr> {
    type Iter<'a> = impl Iterator<Item = (NodeId<usize>, &'a Attr)>
    where
        Attr: 'a,
        Self: 'a;

    fn replace(&mut self, id: NodeId<usize>, attr: Attr) -> Attr {
        std::mem::replace(&mut self.0[id.raw()], attr)
    }

    fn insert(&mut self, id: NodeId<usize>, attr: Attr) {
        if self.0.len() <= id.raw() {
            self.0.resize(id.raw(), Attr::default());
        }
        self.replace(id, attr);
    }

    fn get(&self, id: NodeId<usize>) -> &Attr {
        &self.0[id.raw()]
    }

    fn get_mut(&mut self, id: NodeId<usize>) -> &mut Attr {
        &mut self.0[id.raw()]
    }

    fn iter<'a>(&'a self) -> Self::Iter<'a> {
        self.0.iter().enumerate().map(|(node_id, attr)| {
            let node_id = NodeId::new_unchecked(node_id);
            (node_id, attr)
        })
    }

    fn clear(&mut self) {
        self.0.clear()
    }

    fn count(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone, Debug)]
pub struct AttrHashMap<Id, Attr>(pub(crate) HashMap<Id, Attr>);

impl<Id: Hash + Eq + Copy + Debug, Attr: Clone + Debug> AttrMap<Id, Attr>
    for AttrHashMap<Id, Attr>
{
    type Iter<'a> = impl Iterator<Item = (Id, &'a Attr)>
    where
        Id: 'a,
        Attr: 'a,
        Self: 'a;

    fn replace(&mut self, id: Id, attr: Attr) -> Attr {
        self.0.insert(id, attr).unwrap()
    }

    fn insert(&mut self, id: Id, attr: Attr) {
        self.0.insert(id, attr);
    }

    fn get(&self, id: Id) -> &Attr {
        self.0.get(&id).unwrap()
    }

    fn get_mut(&mut self, id: Id) -> &mut Attr {
        self.0.get_mut(&id).unwrap()
    }

    fn iter<'a>(&'a self) -> Self::Iter<'a> {
        self.0.iter().map(|(id, attr)| (*id, attr))
    }

    fn clear(&mut self) {
        self.0.clear()
    }

    fn count(&self) -> usize {
        self.0.len()
    }
}

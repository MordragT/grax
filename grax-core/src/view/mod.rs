//! A view refers to customized or filtered perspectives of a graph.
//! These views are created to present specific subsets of the graph
//! or to apply specific filters, transformations, or analyses to the graph's data.
//! Therefore usually the incoroprate functions in them which can be used by the graph to alter
//! its functionality on the fly

// TODO elements in view module are not really views

// pub struct NodeViewGraph<V: NodeView, G> {
//     view: V,
//     graph: G,
// }

// pub trait NodeView {

// fn filter
// fn complement
// fn intersect
// fn

//     fn process() {
//         filter_map nodes

//         with data from node view impl
//     }
//     }

use std::fmt::Debug;

pub use distances::Distances;
pub use filter::*;
pub use graph::ViewGraph;
pub use parents::*;
pub use route::Route;
pub use union_find::UnionFind;

mod distances;
mod filter;
mod graph;
mod parents;
mod route;
mod union_find;

pub trait VisitMap<Id>: Clone + Debug {
    fn visit(&mut self, id: Id);
    fn unvisit(&mut self, id: Id);
    fn is_visited(&self, id: Id) -> bool;
    fn all(&self) -> bool;
}

pub trait AttrMap<Id, Attr>: Clone + Debug {
    type Iter<'a>: Iterator<Item = (Id, &'a Attr)>
    where
        Id: 'a,
        Attr: 'a,
        Self: 'a;

    fn replace(&mut self, id: Id, attr: Attr) -> Attr;
    fn insert(&mut self, id: Id, attr: Attr);
    fn get(&self, id: Id) -> &Attr;
    fn get_mut(&mut self, id: Id) -> &mut Attr;
    fn iter<'a>(&'a self) -> Self::Iter<'a>;
    fn clear(&mut self);
    fn count(&self) -> usize;
}

pub trait View {}

pub trait ViewAdaptor<G> {
    fn adapt(&self, graph: &mut G);
}

use std::borrow::BorrowMut;

use ghost_cell::{GhostCell, GhostToken};
use static_rc::StaticRc;

pub type GhostNode<'brand, T> = GhostCell<'brand, SoftNode<'brand, T>>;
pub type HalfNodePtr<'brand, T> = StaticRc<GhostNode<'brand, T>, 1, 2>;
pub type FullNodePtr<'brand, T> = StaticRc<GhostNode<'brand, T>, 2, 2>;

#[derive(Default)]
pub struct SoftNode<'brand, T> {
    // upper bound of keys contained in elements
    current_key: usize,
    rank: usize,
    size: usize,
    // number of elements is roughly size
    elements: Vec<SoftElement<T>>,
    // must be rank - 1 if existent
    left: Option<HalfNodePtr<'brand, T>>,
    // must be rank - 1 if existent
    right: Option<HalfNodePtr<'brand, T>>,
}

impl<'brand, T: Default> SoftNode<'brand, T> {
    pub fn new() -> Self {
        Self {
            current_key: 0,
            rank: 0,
            size: 0,
            elements: Vec::new(),
            left: None,
            right: None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn insert(&mut self, key: usize, value: T) {
        self.elements.push((key, value));
        self.current_key = self.current_key.max(key);
    }

    pub fn pop(&mut self) -> Option<SoftElement<T>> {
        self.elements.pop()
    }

    pub fn sift(&mut self, token: &mut GhostToken<'brand>) {
        if !self.is_leaf() && self.elements.len() < self.size / 2 {
            if self.left.is_none() {
                let right = std::mem::replace(&mut self.left, self.right.take());
                self.right = right;
            }
            // left is now some

            // TODO mutations only allowed for full ptrs
            // TODO need lift feature of staticrc ?

            let left = self.left.as_ref().unwrap().get_ref();

            self.elements.append(elements);
            self.current_key = left.borrow(token).current_key;

            if !left.borrow(token).is_leaf() {
                self.left = None;
            } else {
            }
        }
    }
}

pub type SoftElement<T> = (usize, T);

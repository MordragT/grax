use ghost_cell::{GhostCell, GhostToken};

use super::tree::{FullTreePtr, GhostTree, SoftTree};

pub struct SoftHeap<'brand, T> {
    tree_head: FullTreePtr<'brand, T>,
    rank: usize,
    empty: bool,
}

use ghost_cell::GhostCell;
use static_rc::StaticRc;

use super::node::FullNodePtr;

pub type GhostTree<'brand, T> = GhostCell<'brand, SoftTree<'brand, T>>;

pub type ThirdTreePtr<'brand, T> = StaticRc<GhostTree<'brand, T>, 1, 3>;
// pub type TwoThirdTreePtr<'brand, T> = StaticRc<GhostTree<'brand, T>, 2, 3>;
pub type FullTreePtr<'brand, T> = StaticRc<GhostTree<'brand, T>, 3, 3>;

pub struct SoftTree<'brand, T> {
    // rank of tree determined by its root
    root: FullNodePtr<'brand, T>,
    next: ThirdTreePtr<'brand, T>,
    prev: ThirdTreePtr<'brand, T>,
    suff_min: ThirdTreePtr<'brand, T>,
}

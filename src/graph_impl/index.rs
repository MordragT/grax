#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct NodeIndex(pub(crate) usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EdgeIndex {
    pub(crate) from: NodeIndex,
    pub(crate) to: NodeIndex,
}

impl EdgeIndex {
    pub(crate) fn new(from: NodeIndex, to: NodeIndex) -> Self {
        Self { from, to }
    }

    pub fn contains(&self, index: NodeIndex) -> bool {
        self.from == index || self.to == index
    }

    pub fn rev(&self) -> Self {
        let Self { from, to } = self;

        Self {
            from: *to,
            to: *from,
        }
    }
}

use grax_core::prelude::{Identifier, NodeId};

pub struct Update<Id: Identifier> {
    pub(crate) id: NodeId<Id>,
}

use grax_core::prelude::{Identifier, NodeId};

pub struct Delete<Id: Identifier> {
    pub(crate) id: NodeId<Id>,
}

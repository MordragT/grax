use grax_core::prelude::{Identifier, NodeId};

pub struct Insert<Id: Identifier> {
    pub(crate) id: NodeId<Id>,
}

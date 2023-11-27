use grax_core::prelude::{Identifier, NodeId};
use statement::*;

pub mod delete;
pub mod insert;
pub mod select;
pub mod update;

pub fn node<Id: Identifier>(id: NodeId<Id>) -> Statement<Id> {
    Statement { id }
}

pub mod statement {
    use super::{delete::*, insert::*, select::*, update::*};
    use grax_core::prelude::{Identifier, NodeId};

    pub struct Statement<Id: Identifier> {
        pub(crate) id: NodeId<Id>,
    }

    impl<Id: Identifier> Statement<Id> {
        pub fn select(self) -> Select<Id> {
            Select { ids: vec![self.id] }
        }

        pub fn update(self) -> Update<Id> {
            Update { id: self.id }
        }

        pub fn insert(self) -> Insert<Id> {
            Insert { id: self.id }
        }

        pub fn delete(self) -> Delete<Id> {
            Delete { id: self.id }
        }
    }
}

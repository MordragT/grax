use serde::{Deserialize, Serialize};
use std::{fmt::Debug, path::PathBuf};

pub trait NodeDestination {
    fn destination(&self) -> PathBuf;
}

// #[derive(Hash, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
// pub enum NodeOperation {
//     #[default]
//     None,
//     Remove,
//     Insert(Vec<u8>),
//     Update(Vec<u8>),
// }

use std::num::ParseIntError;
use thiserror::Error;

use crate::NodeIndex;

pub type GraphResult<T> = Result<T, GraphError>;

#[derive(Debug, Error)]
pub enum GraphError {
    #[error("The edge between {left:?} and {right:?} already exists.")]
    EdgeAlreadyExists { left: NodeIndex, right: NodeIndex },
    #[error("Two sided edge forbidden between {left:?} and {right:?} in directed graph.")]
    TwoSidedEdgeForbidden { left: NodeIndex, right: NodeIndex },
    #[error("The given edge list has a bad format")]
    BadEdgeListFormat,
    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] ParseIntError),
}

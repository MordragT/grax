use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

use crate::NodeIndex;

pub type GraphResult<T> = Result<T, GraphError>;

#[derive(Debug, Error)]
pub enum GraphError {
    #[error("The edge between {from:?} and {to:?} already exists.")]
    EdgeAlreadyExists { from: NodeIndex, to: NodeIndex },
    #[error("Two sided edge forbidden between {from:?} and {to:?} in directed graph.")]
    TwoSidedEdgeForbidden { from: NodeIndex, to: NodeIndex },
    #[error("The given edge list has a bad format")]
    BadEdgeListFormat,
    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("ParseFloatError: {0}")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("Could not find tsp solution with nearest neighbor")]
    NNAbort,
}

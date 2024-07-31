use std::{
    num::{ParseFloatError, ParseIntError},
    str::FromStr,
};

use thiserror::Error;

use crate::{
    collections::{EdgeCollection, Keyed, NodeCollection},
    prelude::NodeId,
};

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("The given edge list has a bad format")]
    BadEdgeListFormat,
    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("ParseFloatError: {0}")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("IoError: {0}")]
    Io(#[from] std::io::Error),
}

pub type ParseResult<T> = Result<T, ParseError>;

pub trait ParseGrax: NodeCollection + EdgeCollection + Keyed + Sized {
    fn parse_grax<F>(s: &str, back_edge: F) -> ParseResult<Self>
    where
        F: Fn(
            NodeId<Self::Key>,
            NodeId<Self::Key>,
            Self::EdgeWeight,
        ) -> [(NodeId<Self::Key>, NodeId<Self::Key>, Self::EdgeWeight); 2];
}

pub trait ParseWeight: Sized {
    const LENGTH: usize;

    fn parse_weight<'a>(chunk: impl Iterator<Item = &'a str>) -> ParseResult<Self>;
}

macro_rules! impl_parse_weight(
    ( $( $t:ident ),* )=> {
        $(
            impl ParseWeight for $t {
                const LENGTH: usize = 1;

                fn parse_weight<'a>(mut chunk: impl Iterator<Item = &'a str>) -> ParseResult<Self> {
                    let value = Self::from_str(chunk.next().ok_or(ParseError::BadEdgeListFormat)?)?;
                    Ok(value)
                }
            }

        )*
    }
);

impl_parse_weight!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

impl ParseWeight for () {
    const LENGTH: usize = 0;

    fn parse_weight<'a>(_chunk: impl Iterator<Item = &'a str>) -> ParseResult<Self> {
        Ok(())
    }
}

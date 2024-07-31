use std::{
    ops::{Neg, Sub},
    str::FromStr,
};

use grax_core::{
    edge::weight::{Capacity, Cost, Flow, Reverse},
    parse::{ParseError, ParseWeight},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct FlowBundle<T> {
    pub flow: T,
    pub capacity: T,
    pub reverse: bool,
}

impl<T> Flow<T> for FlowBundle<T> {
    fn flow(&self) -> &T {
        &self.flow
    }

    fn flow_mut(&mut self) -> &mut T {
        &mut self.flow
    }
}

impl<T> Capacity<T> for FlowBundle<T> {
    fn capacity(&self) -> &T {
        &self.capacity
    }

    fn capacity_mut(&mut self) -> &mut T {
        &mut self.capacity
    }
}

impl<T> Reverse for FlowBundle<T>
where
    T: Clone + Sub<T, Output = T>,
{
    fn is_reverse(&self) -> bool {
        self.reverse
    }

    fn reverse(&self) -> Self {
        let Self {
            capacity,
            flow,
            reverse,
        } = self;

        Self {
            flow: capacity.clone() - flow.clone(),
            capacity: capacity.clone(),
            reverse: !reverse,
        }
    }
}

impl<T> ParseWeight for FlowBundle<T>
where
    T: FromStr<Err: Into<ParseError>> + Default,
{
    const LENGTH: usize = 1;

    fn parse_weight<'a>(mut chunk: impl Iterator<Item = &'a str>) -> Result<Self, ParseError> {
        let token = chunk.next().ok_or(ParseError::BadEdgeListFormat)?;
        let capacity = token.parse::<T>().map_err(Into::into)?;

        Ok(Self {
            capacity,
            flow: T::default(),
            reverse: false,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct FlowCostBundle<T> {
    pub cost: T,
    pub flow: T,
    pub capacity: T,
    pub reverse: bool,
}

impl<T> Cost<T> for FlowCostBundle<T> {
    fn cost(&self) -> &T {
        &self.cost
    }

    fn cost_mut(&mut self) -> &mut T {
        &mut self.cost
    }
}

impl<T> Flow<T> for FlowCostBundle<T> {
    fn flow(&self) -> &T {
        &self.flow
    }

    fn flow_mut(&mut self) -> &mut T {
        &mut self.flow
    }
}

impl<T> Capacity<T> for FlowCostBundle<T> {
    fn capacity(&self) -> &T {
        &self.capacity
    }

    fn capacity_mut(&mut self) -> &mut T {
        &mut self.capacity
    }
}

impl<T> Reverse for FlowCostBundle<T>
where
    T: Clone + Sub<T, Output = T> + Neg<Output = T>,
{
    fn is_reverse(&self) -> bool {
        self.reverse
    }

    fn reverse(&self) -> Self {
        let Self {
            cost,
            capacity,
            flow,
            reverse,
        } = self;

        Self {
            cost: -cost.clone(),
            flow: capacity.clone() - flow.clone(),
            capacity: capacity.clone(),
            reverse: !reverse,
        }
    }
}

impl<T> ParseWeight for FlowCostBundle<T>
where
    T: FromStr<Err: Into<ParseError>> + Default,
{
    const LENGTH: usize = 2;

    fn parse_weight<'a>(mut chunk: impl Iterator<Item = &'a str>) -> Result<Self, ParseError> {
        let cost = chunk
            .next()
            .ok_or(ParseError::BadEdgeListFormat)?
            .parse::<T>()
            .map_err(Into::into)?;
        let capacity = chunk
            .next()
            .ok_or(ParseError::BadEdgeListFormat)?
            .parse::<T>()
            .map_err(Into::into)?;

        Ok(Self {
            capacity,
            cost,
            flow: T::default(),
            reverse: false,
        })
    }
}

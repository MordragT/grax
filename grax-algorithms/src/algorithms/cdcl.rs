use std::ops::{BitAnd, Not};

// for package management
// create mst from dependency graph, then dfs over it and install the packages visited

// 1. CandidateProvider: Complete Graph of all candiates, where each node weight implements
// candidate/ list of candidates and creation of graph inside alg
// 2. mutating graph by incrementally adding edges in form of terms by observing the requirements
// and choosing a candidate which also matches the current term (so requirements for discovery and term for checking if correct)
// 3. If no candidate is available which matches the requirement, negate the requirement and add it to the current term.
// Then backtrack until another node can be selected while also removing Requirements which were taken for the nodes along the previsouly
// chosen path.
// 4. If candidates are available but they do not satisfy the current term, identify the subterm which triggered the conflict (and therefore
// the node which triggered it) and combine it with the subterm of the current node
// ({A, B} u {C, not B} = {A, C} where A, C are nodes and B, not B are requirements meaning that A is not compatible with C)
// and then backtrack to the node which triggered the conflicting subterm (node before C) and select another candidate if available else 3

// Order Candidates so that e.g. higher version are prioritized
pub trait Candidate: Ord {}

pub trait Requirement {
    type Candidate: Candidate;

    fn matches(&self, candidate: Self::Candidate) -> bool;

    fn negate(&self) -> Self;
}

// term = distance (parents with cost(subterm) associated ?)
pub trait Term {
    type Literal: Requirement;

    fn requires(&self) -> &[Self::Literal];
    fn forbids(&self) -> &[Self::Literal];

    // required or forbidden
    // is satisfied by something
}

// incompatiblities
// set of terms that cannot be satisfied at once
// use incompatibilities to represent dependencies between nodes
// to avoid redoing the same work over and over again

// 1. phase unit propagation:
// create incompatibilities from the selected nodes

// 2. decision making
// iterate over dependencies and choose some dependent version
// wich matches the term, then add incompatibilities lazily.
// if conflict arises find incompatilbity that triggered the conflict,
// then select other version that matches term or return the root cause,
// why it is failing

// Nodes

// 0 rust 1.5: (llvm > 1.0, libc < 3.5)(Term)
// 1 llvm 1.2: libc > 3.2
// 2 llvm 1.5: libc > 3.5
// 3 libc 3.3
// 4 libc 3.4

// Create conflict graph from it

// 0 -(llvm <= 1.0)(incompatiblitiy)> 2
// 0 -(libc >= 3.5)(incompat)> 4
// 2 -(libc <= 3.5)>4
// conflict in set of incompats
// go from outer most to root
//


#[cfg(test)]
mod tests;

pub mod deterministic;
pub mod compute;
pub mod errors;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct State(pub String);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol(pub String);
#[derive(Debug, PartialEq, Eq)]
pub struct Transition(pub usize, pub usize, pub i32);

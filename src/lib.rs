#![feature(test)]

mod binary_search;
mod linear_search;

pub use binary_search::binary_search;
pub use linear_search::linear_search;

#[cfg(test)]
const LIST_SIZE: i32 = 100_000;

// Change the seed to test with different targets. 
// We need the target to be the same for both functions. 
// At the same time, we need to be able to test with different targets. 
// This is where being able to change the seed comes in handy.
#[cfg(test)]
const SEED: u64 = 1;

//! A Rust implementation of topics within Dempster-Shafer Theory.
#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]
pub mod approx;
pub mod comb;
pub mod dst;
mod product;
pub mod set;

mod hash;

//! Graph generators
//!
//! Generate graphs with different properties.

pub mod cycle;
pub mod linear;

pub use {
    cycle::Cycle,
    linear::Linear,
};

//! Graph generators
//!
//! Generate graphs with different properties.

pub mod cycle;
pub mod linear;
pub mod linear_const;

pub use {
    cycle::Cycle,
    linear::Linear,
    linear_const::LinearConst,
};

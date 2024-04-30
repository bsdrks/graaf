//! Graph generators
//!
//! Generate graphs with different properties.

pub mod cycle;
pub mod cycle_const;
pub mod linear;
pub mod linear_const;

pub use {
    cycle::Cycle,
    cycle_const::CycleConst,
    linear::Linear,
    linear_const::LinearConst,
};

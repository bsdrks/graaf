//! Graph generators
//!
//! Generate parametrics graphs.

pub mod cycle;
pub mod cycle_const;
pub mod linear;
pub mod linear_const;
pub mod star;

pub use {
    cycle::Cycle,
    cycle_const::CycleConst,
    linear::Linear,
    linear_const::LinearConst,
    star::Star,
};

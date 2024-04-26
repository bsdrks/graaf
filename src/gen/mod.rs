//! Graph generators
//!
//! Generate graphs with different properties.

pub mod empty;
pub mod linear;

pub use {
    empty::Empty,
    linear::Linear,
};

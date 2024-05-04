//! Graph generators
//!
//! Generate parametrics graphs.
//!
//! # Examples
//!
//! ```
//! use graaf::gen::{
//!     Cycle,
//!     Linear,
//!     Star,
//! };
//!
//! assert_eq!(
//!     Vec::<Vec<usize>>::cycle(4),
//!     vec![vec![1], vec![2], vec![3], vec![0]]
//! );
//!
//! assert_eq!(
//!     Vec::<Vec<usize>>::linear(4),
//!     vec![vec![1], vec![2], vec![3], Vec::new()]
//! );
//!
//! assert_eq!(
//!     Vec::<Vec<usize>>::star(5),
//!     vec![vec![1, 2, 3, 4], vec![0], vec![0], vec![0], vec![0]]
//! );
//! ```

pub mod cycle;
pub mod cycle_const;
pub mod linear;
pub mod linear_const;
pub mod star;
pub mod star_const;

pub use {
    cycle::Cycle,
    cycle_const::CycleConst,
    linear::Linear,
    linear_const::LinearConst,
    star::Star,
    star_const::StarConst,
};

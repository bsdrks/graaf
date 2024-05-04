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
//!     <[Vec::<usize>; 4]>::cycle(),
//!     [vec![1], vec![2], vec![3], vec![0]]
//! );
//!
//! assert_eq!(
//!     <[Vec::<usize>; 4]>::linear(),
//!     [vec![1], vec![2], vec![3], Vec::new()]
//! );
//!
//! assert_eq!(
//!     <[Vec::<usize>; 5]>::star(),
//!     [vec![1, 2, 3, 4], vec![0], vec![0], vec![0], vec![0]]
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

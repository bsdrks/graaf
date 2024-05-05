//! Graph generators
//!
//! Generate parametrics digraphs.
//!
//! # Examples
//!
//! ```
//! use graaf::gen::Cycle;
//!
//! assert_eq!(
//!     Vec::<Vec<usize>>::cycle(4),
//!     vec![vec![1], vec![2], vec![3], vec![0]]
//! );
//! ```

pub mod cycle;
pub mod cycle_const;

pub use {
    cycle::Cycle,
    cycle_const::CycleConst,
};

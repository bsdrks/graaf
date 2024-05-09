//! Graph generators
//!
//! Generate parametrics directed graphs.
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
//!
//! ```
//! extern crate alloc;
//!
//! use {
//!     alloc::collections::BTreeSet,
//!     graaf::gen::CompleteConst,
//! };
//!
//! assert_eq!(
//!     <[BTreeSet::<usize>; 4]>::complete(),
//!     [
//!         BTreeSet::from([1, 2, 3]),
//!         BTreeSet::from([0, 2, 3]),
//!         BTreeSet::from([0, 1, 3]),
//!         BTreeSet::from([0, 1, 2]),
//!     ]
//! );
//! ```
//!
//! ```
//! extern crate alloc;
//!
//! use {
//!     alloc::collections::BTreeSet,
//!     graaf::gen::Empty,
//! };
//!
//! assert_eq!(Vec::<BTreeSet<usize>>::empty(3), vec![BTreeSet::new(); 3]);
//! ```

pub mod complete;
pub mod complete_const;
pub mod cycle;
pub mod cycle_const;
pub mod empty;
pub mod empty_const;

pub use {
    complete::Complete,
    complete_const::CompleteConst,
    cycle::Cycle,
    cycle_const::CycleConst,
    empty::Empty,
    empty_const::EmptyConst,
};

//! Cross-module properties and strategies.
//!
//! # Examples
//!
//! Properties test the combined behavior of multiple traits.
//!
//! ```
//! use {
//!     graaf::{
//!         gen::RandomTournament,
//!         prop::sum_indegrees_eq_sum_outdegrees,
//!     },
//!     std::collections::BTreeSet,
//! };
//!
//! for v in 1..10 {
//!     let digraph = Vec::<BTreeSet<usize>>::random_tournament(v);
//!
//!     assert!(sum_indegrees_eq_sum_outdegrees(&digraph));
//! }
//! ```

mod add_arc_has_arc;
mod add_arc_remove_arc;
mod add_weighted_arc_has_arc;
mod add_weighted_arc_remove_arc;
mod sum_indegrees_eq_sum_outdegrees;

pub mod strategy;

pub use {
    add_arc_has_arc::add_arc_has_arc,
    add_arc_remove_arc::add_arc_remove_arc,
    add_weighted_arc_has_arc::add_weighted_arc_has_arc,
    add_weighted_arc_remove_arc::add_weighted_arc_remove_arc,
    sum_indegrees_eq_sum_outdegrees::sum_indegrees_eq_sum_outdegrees,
};

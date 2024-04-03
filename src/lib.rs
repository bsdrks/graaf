//! # Graaf
//!
//! Functions and types for working with graphs
//!
//! ## Example
//!
//! ```
//! use graaf::{
//!     ops::{
//!         AddEdge,
//!         Indegree,
//!         Outdegree,
//!     },
//!     repr::AdjacencyMatrix,
//! };
//!
//! let mut adj = AdjacencyMatrix::<4>::new();
//!
//! adj.add_edge(0, 1);
//! adj.add_edge(0, 2);
//! adj.add_edge(1, 3);
//! adj.add_edge(2, 3);
//!
//! assert_eq!(adj.indegree(0), 0);
//! assert_eq!(adj.indegree(1), 1);
//! assert_eq!(adj.indegree(2), 1);
//! assert_eq!(adj.indegree(3), 2);
//!
//! assert_eq!(adj.outdegree(0), 2);
//! assert_eq!(adj.outdegree(1), 1);
//! assert_eq!(adj.outdegree(2), 1);
//! assert_eq!(adj.outdegree(3), 0);
//! ```

#![cfg(not(tarpaulin_include))]
// Groups
#![forbid(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic)]
// Restrictions
#![deny(
    clippy::alloc_instead_of_core,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::impl_trait_in_params,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::missing_assert_message,
    clippy::multiple_inherent_impl,
    clippy::panic_in_result_fn,
    clippy::pattern_type_mismatch,
    clippy::redundant_type_annotations,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::self_named_module_files,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::unwrap_in_result
)]
// Rustc
#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    variant_size_differences
)]
// Rustdoc
#![deny(rustdoc::missing_crate_level_docs)]
#![allow(incomplete_features)]
#![feature(assert_matches, generic_const_exprs)]

/// Graph algorithms
pub mod algo;

/// Operations on graphs
pub mod ops;

/// Types that represent graphs
pub mod repr;

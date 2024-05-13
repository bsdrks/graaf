//! # Graaf
//!
//! Functions and types for working with graphs
//!
//! ## Operations
//!
//! Build and query graphs made with standard collections, or implement the
//! operation traits for your types.
//!
//! ```rust
//! use {
//!     graaf::{
//!         gen::EmptyConst,
//!         op::*,
//!     },
//!     std::collections::BTreeSet,
//! };
//!
//! let mut graph = <[BTreeSet<usize>; 3]>::empty();
//!
//! graph.add_edge(0, 1);
//! graph.add_edge(0, 2);
//!
//! assert_eq!(graph.degree(0), 2);
//! assert_eq!(graph.degree(1), 1);
//! assert_eq!(graph.degree(2), 1);
//! ```
//!
//! ## Algorithms
//!
//! Search, traverse, and analyze graphs built from the types that implement the
//! operation traits.
//!
//! ```rust
//! use graaf::algo::bfs::single_pair_shortest_path as spsp;
//!
//! // 0  ←  1
//! // ↑     ↑
//! // 3  →  2
//!
//! let graph = [Vec::new(), vec![0], vec![1], vec![0, 2]];
//!
//! assert_eq!(spsp(&graph, 3, 0), Some(vec![3, 0]));
//! assert_eq!(spsp(&graph, 3, 1), Some(vec![3, 2, 1]));
//! assert_eq!(spsp(&graph, 3, 2), Some(vec![3, 2]));
//! assert_eq!(spsp(&graph, 0, 3), None);
//! ```
//!
//! ## Representations
//!
//! An adjacency matrix representation is available with the `adjacency_matrix`
//! feature.
//!
//! ```rust
//! use graaf::{
//!     op::*,
//!     repr::AdjacencyMatrix,
//! };
//!
//! let mut graph = AdjacencyMatrix::<3>::new();
//!
//! graph.add_edge(0, 1);
//! graph.add_edge(1, 1);
//!
//! assert!(!graph.is_simple());
//! ```
//!
//! ## Generators
//!
//! Generate parameterized graphs.
//!
//! ```rust
//! use graaf::gen::*;
//!
//! assert_eq!(Vec::<Vec<usize>>::empty(2), vec![Vec::new(), Vec::new()]);
//! assert_eq!(Vec::<Vec<usize>>::cycle(3), vec![vec![1], vec![2], vec![0]]);
//!
//! assert_eq!(
//!     <[Vec::<usize>; 3]>::complete(),
//!     [vec![1, 2], vec![0, 2], vec![0, 1]]
//! );
//! ```

// Clippy lint groups
#![deny(clippy::all, clippy::cargo, clippy::pedantic, clippy::nursery)]
// Clippy restriction lints
#![deny(
    clippy::alloc_instead_of_core,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::impl_trait_in_params,
    clippy::missing_assert_message,
    clippy::multiple_inherent_impl,
    clippy::panic_in_result_fn,
    clippy::redundant_type_annotations,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::self_named_module_files,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::unwrap_in_result
)]
// Rustc lint groups
#![deny(rust_2018_idioms)]
// Rustc lints
#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    variant_size_differences
)]
// Rustdoc lints
#![deny(rustdoc::all)]
#![cfg_attr(feature = "adjacency_matrix", allow(incomplete_features))]
#![cfg_attr(feature = "adjacency_matrix", feature(generic_const_exprs))]

pub mod algo;
pub mod gen;
pub mod op;
pub mod prop;
pub mod repr;

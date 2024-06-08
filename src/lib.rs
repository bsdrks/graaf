//! # Graaf
//!
//! Build and query digraphs.
//!
//! ## Examples
//!
//! ```rust
//! use {
//!     graaf::{
//!         algo::bfs::single_pair_shortest_path as spsp,
//!         gen::*,
//!         op::*,
//!         repr::*,
//!     },
//!     std::collections::BTreeSet,
//! };
//!
//! let mut digraph = <[BTreeSet<usize>; 3]>::empty();
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//!
//! // 0 -> {1, 2}
//! // 1 -> {}
//! // 2 -> {}
//!
//! assert_eq!(digraph.degree(0), 2);
//! assert_eq!(digraph.degree(1), 1);
//! assert_eq!(digraph.degree(2), 1);
//!
//! // 0 -> {1}
//! // 1 -> {0}
//! // 2 -> {1}
//! // 3 -> {0, 2}
//!
//! let digraph = [Vec::new(), vec![0], vec![1], vec![0, 2]];
//!
//! assert_eq!(spsp(&digraph, 3, 0), Some(vec![3, 0]));
//! assert_eq!(spsp(&digraph, 3, 1), Some(vec![3, 2, 1]));
//!
//! // 0 -> {}
//! // 1 -> {}
//! // 2 -> {}
//!
//! assert!(Vec::<Vec<usize>>::empty(3)
//!     .iter()
//!     .eq(&[Vec::new(), Vec::new(), Vec::new()]));
//!
//! // 0 -> {1}
//! // 1 -> {2}
//! // 2 -> {0}
//!
//! assert!(Vec::<Vec<usize>>::cycle(3)
//!     .iter()
//!     .eq(&[vec![1], vec![2], vec![0]]));
//!
//! // 0 -> {1, 2}
//! // 1 -> {0, 2}
//! // 2 -> {0, 1}
//!
//! assert!(<[Vec::<usize>; 3]>::complete()
//!     .iter()
//!     .eq(&[vec![1, 2], vec![0, 2], vec![0, 1]]));
//!
//! let tournament = Vec::<BTreeSet<usize>>::random_tournament(4);
//!
//! assert_eq!(tournament.size(), 6);
//! assert_eq!(tournament.order(), 4);
//!
//! for s in tournament.iter_vertices() {
//!     assert_eq!(tournament.degree(s), 3);
//!     assert!((0..3).contains(&tournament.outdegree(s)));
//!     assert!((0..3).contains(&tournament.indegree(s)));
//! }
//!
//! let mut digraph = AdjacencyMatrix::<3>::new();
//!
//! // 0 -> {1}
//! // 1 -> {1}
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 1);
//!
//! assert!(!digraph.is_simple());
//! ```

// Clippy lint groups
#![deny(clippy::all, clippy::cargo, clippy::pedantic, clippy::nursery)]
// Clippy restriction lints
#![deny(
    clippy::std_instead_of_core,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::impl_trait_in_params,
    clippy::missing_assert_message,
    clippy::multiple_inherent_impl,
    clippy::panic_in_result_fn,
    clippy::redundant_type_annotations,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::self_named_module_files,
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

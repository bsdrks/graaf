//! # Graaf
//!
//! Functions and types for working with graphs
//!
//! ## Examples
//!
//! ```
//! use {
//!     graaf::op::{
//!         AddEdge,
//!         Indegree,
//!         Outdegree,
//!     },
//!     std::collections::HashSet,
//! };
//!
//! let mut graph = [HashSet::new(), HashSet::new(), HashSet::new()];
//!
//! graph.add_edge(0, 1);
//! graph.add_edge(0, 2);
//! graph.add_edge(1, 2);
//!
//! assert_eq!(graph.indegree(0), 0);
//! assert_eq!(graph.indegree(1), 1);
//! assert_eq!(graph.indegree(2), 2);
//!
//! assert_eq!(graph.outdegree(0), 2);
//! assert_eq!(graph.outdegree(1), 1);
//! assert_eq!(graph.outdegree(2), 0);
//! ```

// Clippy lint groups
#![deny(clippy::all, clippy::cargo, clippy::pedantic, clippy::nursery)]
// Clippy lints
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
#![cfg_attr(feature = "nightly", allow(incomplete_features))]
#![cfg_attr(feature = "nightly", feature(generic_const_exprs))]

/// Graph algorithms
pub mod algo;

/// Operations on graphs
pub mod op;

/// Types that represent graphs
pub mod repr;

#[cfg(feature = "adjacency-matrix")]
#[cfg(test)]
mod tests {
    use crate::{
        op::{
            AddEdge,
            Indegree,
            Outdegree,
        },
        repr::AdjacencyMatrix,
    };

    #[test]
    fn example() {
        let mut graph = AdjacencyMatrix::<3>::new();

        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);

        assert_eq!(graph.indegree(0), 0);
        assert_eq!(graph.indegree(1), 1);
        assert_eq!(graph.indegree(2), 2);

        assert_eq!(graph.outdegree(0), 2);
        assert_eq!(graph.outdegree(1), 1);
        assert_eq!(graph.outdegree(2), 0);
    }
}

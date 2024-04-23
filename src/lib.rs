//! # Graaf
//!
//! Functions and types for working with graphs
//!
//! ## Examples
//!
//! ```
//! use {
//!     graaf::{
//!         algo::bfs::single_pair_shortest_path,
//!         op::{
//!             AddEdge,
//!             Indegree,
//!         },
//!     },
//!     std::collections::HashSet,
//! };
//!
//! let mut graph: [HashSet<usize>; 4] = [
//!     HashSet::new(),
//!     HashSet::new(),
//!     HashSet::new(),
//!     HashSet::new(),
//! ];
//!
//! // ╭───╮       ╭───╮
//! // │ 0 │   →   │ 1 │
//! // ╰───╯       ╰───╯
//! //   ↑           ↓
//! // ╭───╮       ╭───╮
//! // │ 3 │       │ 2 │
//! // ╰───╯       ╰───╯
//!
//! graph.add_edge(3, 0);
//! graph.add_edge(0, 1);
//! graph.add_edge(1, 2);
//!
//! assert_eq!(graph.indegree(0), 1);
//! assert_eq!(graph.indegree(1), 1);
//! assert_eq!(graph.indegree(2), 1);
//! assert_eq!(graph.indegree(3), 0);
//!
//! let path = single_pair_shortest_path(&graph, 3, 2);
//!
//! assert_eq!(path, Some(vec![3, 0, 1, 2]));
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

pub mod algo;
pub mod gen;
pub mod op;
pub mod repr;

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        use {
            crate::{
                algo::bfs::single_pair_shortest_path,
                op::{
                    AddEdge,
                    Indegree,
                },
            },
            std::collections::HashSet,
        };

        let mut graph = [
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
        ];

        // ╭───╮       ╭───╮
        // │ 0 │   →   │ 1 │
        // ╰───╯       ╰───╯
        //   ↑           ↓
        // ╭───╮       ╭───╮
        // │ 3 │       │ 2 │
        // ╰───╯       ╰───╯

        graph.add_edge(3, 0);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        assert_eq!(graph.indegree(0), 1);
        assert_eq!(graph.indegree(1), 1);
        assert_eq!(graph.indegree(2), 1);
        assert_eq!(graph.indegree(3), 0);

        let path = single_pair_shortest_path(&graph, 3, 2);

        assert_eq!(path, Some(vec![3, 0, 1, 2]));
    }
}

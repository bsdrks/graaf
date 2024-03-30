//! # Graaf
//!
//! Functions and types for working with graphs
//!
//! ## Example
//!
//! ```
//! use graaf::{
//!     AddEdge,
//!     AdjacencyMatrix,
//!     Indegree,
//!     Outdegree,
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
#![deny(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic)]
#![deny(
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    rustdoc::missing_crate_level_docs,
    rustdoc::missing_doc_code_examples,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    variant_size_differences
)]
#![allow(incomplete_features)]
#![feature(assert_matches, generic_const_exprs, rustdoc_missing_doc_code_examples)]

mod algo;
mod ops;
mod repr;

pub use {
    algo::{
        DijkstraUnweighted,
        DijkstraWeighted,
    },
    ops::{
        AddEdge,
        AddWeightedEdge,
        CountAllEdges,
        CountAllVertices,
        EdgeWeight,
        Indegree,
        IsEdge,
        IterAllEdges,
        IterAllWeightedEdges,
        IterEdges,
        IterVertices,
        IterWeightedEdges,
        Outdegree,
        RemoveEdge,
        VertexWeight,
    },
    repr::AdjacencyMatrix,
};

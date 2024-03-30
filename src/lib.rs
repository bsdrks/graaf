//! # Graph
//!
//! A simple graph library.
#![deny(
    clippy::all,
    clippy::nursery,
    clippy::pedantic,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    missing_abi,
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
#![allow(incomplete_features)]
#![feature(assert_matches, generic_const_exprs)]

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

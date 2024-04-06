//! # Graaf
//!
//! Functions and types for working with graphs
//!
//! ## Examples
//!
//! ```
//! use graaf::{
//!     op::{
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

// Rustdoc
#![allow(incomplete_features)]
#![feature(assert_matches, generic_const_exprs)]

/// Graph algorithms
pub mod algo;

/// Operations on graphs
pub mod op;

/// Types that represent graphs
pub mod repr;

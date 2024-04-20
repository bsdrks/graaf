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
//! let mut graph = AdjacencyMatrix::<3>::new();
//!
//! adj.add_edge(0, 1);
//! adj.add_edge(0, 2);
//! adj.add_edge(1, 2);
//!
//! assert_eq!(adj.indegree(0), 0);
//! assert_eq!(adj.indegree(1), 1);
//! assert_eq!(adj.indegree(2), 2);
//!
//! assert_eq!(adj.outdegree(0), 2);
//! assert_eq!(adj.outdegree(1), 1);
//! assert_eq!(adj.outdegree(2), 0);
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

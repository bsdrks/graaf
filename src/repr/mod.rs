//! Custom graph representations
//!
//! See the documentation for [`op`](crate::op) for a list of common
//! representations built from standard library types.

#[cfg(feature = "adjacency_matrix")]
pub mod adjacency_matrix;

#[cfg(feature = "adjacency_matrix")]
pub use adjacency_matrix::AdjacencyMatrix;

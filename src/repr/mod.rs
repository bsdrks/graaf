//! Types that represent graphs

/// An adjacency matrix representation of an unweighted digraph stored as a bit
/// array
#[cfg(feature = "adjacency_matrix")]
pub mod adjacency_matrix;

#[cfg(feature = "adjacency_matrix")]
pub use adjacency_matrix::AdjacencyMatrix;

/// An adjacency matrix representation of an unweighted directed graph stored as
/// a bit array
#[cfg(feature = "adjacency_matrix")]
pub mod adjacency_matrix;

#[cfg(feature = "adjacency_matrix")]
pub use adjacency_matrix::AdjacencyMatrix;

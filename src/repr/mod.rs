//! Custom digraph representations
//!
//! See the documentation for [`op`] for a list of common representations built
//! from standard library types.
//!
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     op::*,
//!     repr::AdjacencyMatrix,
//! };
//!
//! let mut digraph = AdjacencyMatrix::<3>::new();
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 1);
//!
//! assert!(!digraph.is_simple());
//! ```
//!
//! [`op`]: crate::op

#[cfg(feature = "adjacency_matrix")]
pub mod adjacency_matrix;

#[cfg(feature = "adjacency_matrix")]
pub use adjacency_matrix::AdjacencyMatrix;

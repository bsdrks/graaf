//! Check whether an edge exists in a digraph.
//!
//! To determine whether an arc exists from `u` to `v`, see [`HasArc`].
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Circuit,
//!     Empty,
//!     HasEdge,
//! };
//!
//! let digraph = AdjacencyList::circuit(2);
//!
//! assert!(digraph.has_edge(0, 1));
//! assert!(digraph.has_edge(1, 0));
//!
//! let mut digraph = AdjacencyList::empty(2);
//!
//! digraph.add_arc(0, 1);
//!
//! assert!(!digraph.has_edge(0, 1));
//! assert!(!digraph.has_edge(1, 0));
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(!digraph.has_edge(0, 1));
//! assert!(digraph.has_edge(0, 2));
//! assert!(!digraph.has_edge(1, 0));
//! assert!(!digraph.has_edge(1, 2));
//! assert!(digraph.has_edge(2, 0));
//! assert!(!digraph.has_edge(2, 1));
//! ```
//!
//! [`HasArc`]: crate::HasArc

/// Check whether an edge exists in a digraph.
pub trait HasEdge {
    /// Check whether an edge exists in the digraph.
    ///
    /// # Arguments
    ///
    /// * `u`: The tail vertex.
    /// * `v`: The head vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Circuit,
    ///     Empty,
    ///     HasEdge,
    /// };
    ///
    /// let digraph = AdjacencyList::circuit(2);
    ///
    /// assert!(digraph.has_edge(0, 1));
    /// assert!(digraph.has_edge(1, 0));
    ///
    /// let mut digraph = AdjacencyList::empty(2);
    ///
    /// digraph.add_arc(0, 1);
    ///
    /// assert!(!digraph.has_edge(0, 1));
    /// assert!(!digraph.has_edge(1, 0));
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(2, 0);
    ///
    /// assert!(!digraph.has_edge(0, 1));
    /// assert!(digraph.has_edge(0, 2));
    /// assert!(!digraph.has_edge(1, 0));
    /// assert!(!digraph.has_edge(1, 2));
    /// assert!(digraph.has_edge(2, 0));
    /// assert!(!digraph.has_edge(2, 1));
    /// ```
    #[must_use]
    fn has_edge(&self, u: usize, v: usize) -> bool;
}

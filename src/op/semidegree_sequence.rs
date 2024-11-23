//! Iterate a digraph's semidegree sequence.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     SemidegreeSequence,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(digraph.semidegree_sequence().eq([(1, 2), (1, 1), (2, 1)]));
//! ```

use crate::{
    Indegree,
    Outdegree,
    Vertices,
};

/// Digraph semidegree sequence
pub trait SemidegreeSequence {
    /// Iterate the semidegree sequence of a digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     SemidegreeSequence,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(2, 0);
    ///
    /// assert!(digraph.semidegree_sequence().eq([(1, 2), (1, 1), (2, 1)]));
    /// ```
    #[must_use]
    fn semidegree_sequence(&self) -> impl Iterator<Item = (usize, usize)>;
}

impl<D> SemidegreeSequence for D
where
    D: Indegree + Outdegree + Vertices,
{
    fn semidegree_sequence(&self) -> impl Iterator<Item = (usize, usize)> {
        self.vertices()
            .map(|u| (self.indegree(u), self.outdegree(u)))
    }
}

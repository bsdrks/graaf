//! Return a digraph's semidegree sequence.
//!
//! The semidegree sequence iterates over a digraph's indegree and outdegree
//! pairs.
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

/// Return a digraph's semidegree sequence.
///
/// # Implementing [`SemidegreeSequence`] for a custom type
///
/// Provide an implementation of
/// [`semidegree_sequence`](SemidegreeSequence::semidegree_sequence) that
/// returns a digraph's semidegree sequence OR implement [`Indegree`],
/// [`Outdegree`], and [`Vertices`].
///
/// ```
/// use {
///     graaf::{
///         Indegree,
///         Outdegree,
///         SemidegreeSequence,
///         Vertices,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Indegree for AdjacencyList {
///     fn indegree(&self, v: usize) -> usize {
///         self.arcs.iter().filter(|set| set.contains(&v)).count()
///     }
/// }
///
/// impl Outdegree for AdjacencyList {
///     fn outdegree(&self, u: usize) -> usize {
///         self.arcs[u].len()
///     }
/// }
///
/// impl Vertices for AdjacencyList {
///     fn vertices(&self) -> impl Iterator<Item = usize> {
///         0..self.arcs.len()
///     }
/// }
///
/// let digraph = AdjacencyList {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([2]),
///         BTreeSet::from([0]),
///     ],
/// };
///
/// assert!(digraph.semidegree_sequence().eq([(1, 2), (1, 1), (2, 1)]));
/// ```
pub trait SemidegreeSequence {
    /// Return the semidegree sequence of a digraph.
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

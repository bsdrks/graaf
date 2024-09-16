//! Return a digraph's outdegree sequence.
//!
//! The outdegree sequence iterates over a digraph's outdegrees.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     OutdegreeSequence,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(digraph.outdegree_sequence().eq([2, 1, 1]));
//! ```

use crate::{
    Outdegree,
    Vertices,
};

/// Return a digraph's outdegree sequence.
///
/// # Implementing [`OutdegreeSequence`] for a custom type
///
/// Provide an implementation of
/// [`outdegree_sequence`](OutdegreeSequence::outdegree_sequence) that returns
/// the digraph's outdegree sequence OR implement [`Outdegree`] and
/// [`Vertices`].
///
/// ```
/// use {
///     graaf::{
///         Outdegree,
///         OutdegreeSequence,
///         Vertices,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
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
/// assert!(digraph.outdegree_sequence().eq([2, 1, 1]));
/// ```
pub trait OutdegreeSequence {
    /// Return a digraph's outdegree sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     OutdegreeSequence,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(2, 0);
    ///
    /// assert!(digraph.outdegree_sequence().eq([2, 1, 1]));
    /// ```
    #[must_use]
    fn outdegree_sequence(&self) -> impl Iterator<Item = usize>;
}

impl<D> OutdegreeSequence for D
where
    D: Outdegree + Vertices,
{
    fn outdegree_sequence(&self) -> impl Iterator<Item = usize> {
        self.vertices().map(move |v| self.outdegree(v))
    }
}

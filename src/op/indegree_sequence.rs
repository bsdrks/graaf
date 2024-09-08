//! Return a digraph's indegree sequence.
//!
//! The indegree sequence is an iterator over the indegrees of the vertices of
//! a digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IndegreeSequence,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(digraph.indegree_sequence().eq([1, 1, 2]));
//! ```

use crate::{
    Indegree,
    Vertices,
};

/// Return a digraph's indegree sequence.
///
/// # Implementing [`IndegreeSequence`] for a custom type
///
/// Provide an implementation of
/// [`indegree_sequence`](IndegreeSequence::indegree_sequence) that returns the
/// indegree sequence of the digraph OR implement `Indegree` and `Vertices`.
///
/// ```
/// use {
///     graaf::{
///         Indegree,
///         IndegreeSequence,
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
/// impl Vertices for AdjacencyList {
///     fn vertices(&self) -> impl Iterator<Item = usize> {
///         0..self.arcs.len()
///     }
/// }
///
/// let mut digraph = AdjacencyList {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([2]),
///         BTreeSet::from([0]),
///     ],
/// };
///
/// assert!(digraph.indegree_sequence().eq([1, 1, 2]));
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     AddArc,
///     AdjacencyList,
///     Empty,
///     IndegreeSequence,
/// };
///
/// let mut digraph = AdjacencyList::empty(3);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(0, 2);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(2, 0);
///
/// assert!(digraph.indegree_sequence().eq([1, 1, 2]));
/// ```
pub trait IndegreeSequence {
    /// Return the indegree sequence of the digraph.
    #[must_use]
    fn indegree_sequence(&self) -> impl Iterator<Item = usize>;
}

impl<D> IndegreeSequence for D
where
    D: Indegree + Vertices,
{
    fn indegree_sequence(&self) -> impl Iterator<Item = usize> {
        self.vertices().map(move |v| self.indegree(v))
    }
}

//! Return a digraph's outdegree sequence.
//!
//! The outdegree sequence is an iterator over the outdegrees of the vertices
//! of a digraph.
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
/// # Implementing `OutdegreeSequence`
///
/// Provide an implementation of `OutdegreeSequence` that returns the outdegree
/// sequence of the digraph OR implement `Outdegree` and `Vertices`.
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
///         
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
pub trait OutdegreeSequence {
    /// Returns the outdegree sequence of the digraph.
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

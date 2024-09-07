//! Return a digraph's indegree sequence.
//!
//! The indegree sequence is an iterator over the indegrees of the vertices of
//! a digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         IndegreeSequence,
//!     },
//! };
//!
//! let mut digraph = Digraph::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(digraph.indegree_sequence().eq([1, 1, 2]));
//! ```

use super::{
    Indegree,
    Vertices,
};

/// Return a digraph's indegree sequence.
///
/// # Implementing `IndegreeSequence`
///
/// Provide an implementation of `IndegreeSequence` that returns the indegree
/// sequence of the digraph OR implement `Indegree` and `Vertices`.
///
/// ```
/// use {
///     graaf::op::{
///         Indegree,
///         IndegreeSequence,
///         Vertices,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Indegree for Digraph {
///     fn indegree(&self, v: usize) -> usize {
///         self.arcs.iter().filter(|set| set.contains(&v)).count()
///     }
/// }
///
/// impl Vertices for Digraph {
///     fn vertices(&self) -> impl Iterator<Item = usize> {
///         0..self.arcs.len()
///     }
/// }
///
/// let mut digraph = Digraph {
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
///     adjacency_list::Digraph,
///     gen::Empty,
///     op::{
///         AddArc,
///         IndegreeSequence,
///     },
/// };
///
/// let mut digraph = Digraph::empty(3);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(0, 2);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(2, 0);
///
/// assert!(digraph.indegree_sequence().eq([1, 1, 2]));
/// ```
pub trait IndegreeSequence {
    /// Returns the indegree sequence of the digraph.
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

//! Return the semidegree sequence of a digraph.
//!
//! The semidegree sequence is an iterator over the indegree and outdegree pairs
//! of the vertices of a digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         SemidegreeSequence,
//!     },
//! };
//!
//! // 0 -> {1, 2}
//! // 1 -> {2}
//! // 2 -> {0}
//!
//! let mut digraph = Digraph::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(digraph.semidegree_sequence().eq([(1, 2), (1, 1), (2, 1)]));
//! ```

use super::{
    Indegree,
    Outdegree,
    Vertices,
};

/// Return the degree sequence of a digraph.
///
/// # How can I implement `SemidegreeSequence`?
///
/// Provide an implementation of `SemidegreeSequence` that returns the degree
/// sequence of the digraph OR implement `Indegree`, `Outdegree`, and
/// `Vertices`.
///
/// ```
/// use {
///     graaf::op::{
///         Indegree,
///         Outdegree,
///         SemidegreeSequence,
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
/// impl Outdegree for Digraph {
///     fn outdegree(&self, u: usize) -> usize {
///         self.arcs[u].len()
///     }
/// }
///
/// impl Vertices for Digraph {
///     fn vertices(&self) -> impl Iterator<Item = usize> {
///         0..self.arcs.len()
///     }
/// }
///
/// // 0 -> {1, 2}
/// // 1 -> {2}
/// // 2 -> {0}
///
/// let digraph = Digraph {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([2]),
///         BTreeSet::from([0]),
///     ],
/// };
///
/// assert!(digraph.semidegree_sequence().eq([(1, 2), (1, 1), (2, 1)]));
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
///         SemidegreeSequence,
///     },
/// };
///
/// // 0 -> {1, 2}
/// // 1 -> {2}
/// // 2 -> {0}
///
/// let mut digraph = Digraph::empty(3);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(0, 2);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(2, 0);
///
/// assert!(digraph.semidegree_sequence().eq([(1, 2), (1, 1), (2, 1)]));
/// ```
pub trait SemidegreeSequence {
    /// Returns the semidegree sequence of a digraph.
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

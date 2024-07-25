//! Return the outdegree sequence of a digraph.
//!
//! The outdegree sequence is an iterator over the outdegrees of the vertices
//! of a digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         OutdegreeSequence,
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
//! assert!(digraph.outdegree_sequence().eq([2, 1, 1]));
//! ```

use super::{
    Outdegree,
    Vertices,
};

/// Return the outdegree sequence of the digraph.
///
/// # How can I implement `OutdegreeSequence`?
///
/// Provide an implementation of `OutdegreeSequence` that returns the outdegree
/// sequence of the digraph OR implement `Outdegree` and `Vertices`.
///
/// ```
/// use {
///     graaf::op::{
///         Outdegree,
///         OutdegreeSequence,
///         Vertices,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
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
/// assert!(digraph.outdegree_sequence().eq([2, 1, 1]));
/// ```
///         
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     gen::Empty,
///     op::{
///         AddArc,
///         OutdegreeSequence,
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

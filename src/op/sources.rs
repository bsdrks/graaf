//! Return a digraph's sources.
//!
//! A source is a vertex with no in-neighbors.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         Sources,
//!     },
//! };
//!
//! // 0 -> {1, 2}
//! // 1 -> {2}
//! // 2 -> {}
//! // 3 -> {}
//!
//! let mut digraph = Digraph::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//!
//! assert!(digraph.sources().eq([0, 3]));
//! ```

use super::{
    Indegree,
    Vertices,
};

/// Return a digraph's sources.
///
/// # Implementing `Sources`
///
/// Provide an implementation of `sources` that returns an iterator over the
/// sources in the digraph OR implement `Indegree` and `Vertices`.
///
/// ```
/// use {
///     graaf::op::{
///         Indegree,
///         Sources,
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
///     fn indegree(&self, u: usize) -> usize {
///         self.arcs.iter().filter(|set| set.contains(&u)).count()
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
/// // 2 -> {}
/// // 3 -> {}
///
/// let mut digraph = Digraph {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([2]),
///         BTreeSet::new(),
///         BTreeSet::new(),
///     ],
/// };
///
/// assert!(digraph.sources().eq([0, 3]));
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
///         Sources,
///     },
/// };
///
/// // 0 -> {1, 2}
/// // 1 -> {2}
/// // 2 -> {}
/// // 3 -> {}
///
/// let mut digraph = Digraph::empty(4);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(0, 2);
/// digraph.add_arc(1, 2);
///
/// assert!(digraph.sources().eq([0, 3]));
/// ```
pub trait Sources {
    /// Returns an iterator over the sources in the digraph.
    fn sources(&self) -> impl Iterator<Item = usize>;
}

impl<D> Sources for D
where
    D: Indegree + Vertices,
{
    fn sources(&self) -> impl Iterator<Item = usize> {
        self.vertices().filter(move |&u| self.is_source(u))
    }
}

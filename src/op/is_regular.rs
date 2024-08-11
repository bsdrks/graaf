//! Check whether a digraph is regular.
//!
//! A digraph is regular if all vertices have the same indegree and
//! outdegree.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Circuit,
//!     op::{
//!         IsRegular,
//!         RemoveArc,
//!     },
//! };
//!
//! // 0 -> {1}
//! // 1 -> {2}
//! // 2 -> {3}
//! // 3 -> {4}
//! // 4 -> {5}
//! // 5 -> {6}
//! // 6 -> {0}
//!
//! let mut digraph = Digraph::circuit(7);
//!
//! assert!(digraph.is_regular());
//!
//! // 0 -> {1}
//! // 1 -> {2}
//! // 2 -> {3}
//! // 3 -> {4}
//! // 4 -> {5}
//! // 5 -> {6}
//! // 6 -> {}
//!
//! digraph.remove_arc(6, 0);
//!
//! assert!(!digraph.is_regular());
//! ```

use super::{
    Indegree,
    Outdegree,
    Vertices,
};

/// Check whether a digraph is regular.
///
/// # How can I implement `IsRegular`?
///
/// Provide an implementation of `is_regular` that returns whether the digraph
/// is regular OR implement `Indegree`, `Vertices`, and `Outdegree`.
///
/// ```
/// use {
///     graaf::op::{
///         Indegree,
///         IsRegular,
///         Outdegree,
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
/// impl Outdegree for Digraph {
///     fn outdegree(&self, u: usize) -> usize {
///         self.arcs[u].len()
///     }
/// }
///
/// // 0 -> {1, 2}
/// // 1 -> {2, 0}
/// // 2 -> {0, 1}
///
/// let digraph = Digraph {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([2, 0]),
///         BTreeSet::from([0, 1]),
///     ],
/// };
///
/// assert!(digraph.is_regular());
///
/// // 0 -> {1, 2}
/// // 1 -> {0, 2}
/// // 2 -> {0}
///
/// let digraph = Digraph {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([0, 2]),
///         BTreeSet::from([0]),
///     ],
/// };
///
/// assert!(!digraph.is_regular());
/// ```
pub trait IsRegular {
    /// Checks whether the digraph is regular.
    #[must_use]
    fn is_regular(&self) -> bool;
}

impl<D> IsRegular for D
where
    D: Indegree + Outdegree + Vertices,
{
    /// # Panics
    ///
    /// Panics if the digraph has no vertices.
    fn is_regular(&self) -> bool {
        let mut vertices = self.vertices();

        let order = vertices
            .next()
            .expect("a digraph must have at least one vertex");

        let indegree = self.indegree(order);
        let outdegree = self.outdegree(order);

        indegree == outdegree
            && vertices.all(|u| {
                self.indegree(u) == indegree && self.outdegree(u) == outdegree
            })
    }
}

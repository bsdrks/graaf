//! Check whether a digraph is regular.
//!
//! A digraph is regular if all vertices have the same indegree and
//! outdegree.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Circuit,
//!     IsRegular,
//!     RemoveArc,
//! };
//!
//! let mut digraph = AdjacencyList::circuit(7);
//!
//! assert!(digraph.is_regular());
//!
//! digraph.remove_arc(6, 0);
//!
//! assert!(!digraph.is_regular());
//! ```

use crate::{
    Indegree,
    Outdegree,
    Vertices,
};

/// Check whether a digraph is regular.
///
/// # Implementing `IsRegular`
///
/// Provide an implementation of `is_regular` that returns whether the digraph
/// is regular OR implement `Indegree`, `Vertices`, and `Outdegree`.
///
/// ```
/// use {
///     graaf::{
///         Indegree,
///         IsRegular,
///         Outdegree,
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
/// impl Outdegree for AdjacencyList {
///     fn outdegree(&self, u: usize) -> usize {
///         self.arcs[u].len()
///     }
/// }
///
/// let digraph = AdjacencyList {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([2, 0]),
///         BTreeSet::from([0, 1]),
///     ],
/// };
///
/// assert!(digraph.is_regular());
///
/// let digraph = AdjacencyList {
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

        let order =
            vertices.next().expect("a digraph has at least one vertex");

        let indegree = self.indegree(order);
        let outdegree = self.outdegree(order);

        indegree == outdegree
            && vertices.all(|u| {
                self.indegree(u) == indegree && self.outdegree(u) == outdegree
            })
    }
}

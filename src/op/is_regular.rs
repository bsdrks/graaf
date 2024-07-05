//! Determine whether a digraph is regular.
//!
//! A digraph is regular if all vertices have the same indegree and
//! outdegree.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Cycle,
//!     op::{
//!         IsRegular,
//!         RemoveArc,
//!     },
//! };
//!
//! let mut digraph = Digraph::cycle(7);
//!
//! assert!(digraph.is_regular());
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

/// Determine whether a digraph is regular.
///
/// # How can I implement `IsRegular`?
///
/// Provide an implementation of `is_regular` that returns `true` if the digraph
/// is regular and `false` otherwise OR implement `Indegree`, `Vertices`,
/// and `Outdegree`.
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
    /// Returns whether the digraph is regular.
    fn is_regular(&self) -> bool;
}

impl<T> IsRegular for T
where
    T: Indegree + Outdegree + Vertices,
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

        vertices.all(|u| self.indegree(u) == indegree && self.outdegree(u) == outdegree)
    }
}

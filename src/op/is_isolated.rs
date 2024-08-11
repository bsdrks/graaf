//! Check whether a vertex is isolated.
//!
//! A vertex is isolated if it has no incoming or outgoing arcs.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         IsIsolated,
//!     },
//! };
//!
//! // 0 -> {1, 2}
//! // 1 -> {0}
//! // 2 -> {}
//! // 3 -> {}
//!
//! let mut digraph = Digraph::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//!
//! assert!(!digraph.is_isolated(0));
//! assert!(!digraph.is_isolated(1));
//! assert!(!digraph.is_isolated(2));
//! assert!(digraph.is_isolated(3));
//! ```

use super::{
    Indegree,
    Outdegree,
};

/// Determine whether a vertex is isolated.
///
/// # How can I implement `IsIsolated`?
///
/// Provide an implementation of `is_isolated` that returns whether the vertex
/// is isolated OR implement `Indegree` and `Outdegree`.
///
/// ```
/// use {
///     graaf::op::{
///         Indegree,
///         IsIsolated,
///         Outdegree,
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
/// impl Outdegree for Digraph {
///     fn outdegree(&self, u: usize) -> usize {
///         self.arcs[u].len()
///     }
/// }
///
/// // 0 -> {1, 2}
/// // 1 -> {0}
/// // 2 -> {}
/// // 3 -> {}
///
/// let digraph = Digraph {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([0]),
///         BTreeSet::new(),
///         BTreeSet::new(),
///     ],
/// };
///
/// assert!(!digraph.is_isolated(0));
/// assert!(!digraph.is_isolated(1));
/// assert!(!digraph.is_isolated(2));
/// assert!(digraph.is_isolated(3));
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
///         IsIsolated,
///     },
/// };
///
/// // 0 -> {1}
/// // 1 -> {0}
/// // 2 -> {}
///
/// let mut digraph = Digraph::empty(3);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 0);
///
/// assert!(!digraph.is_isolated(0));
/// assert!(!digraph.is_isolated(1));
/// assert!(digraph.is_isolated(2));
/// ```
pub trait IsIsolated {
    /// Checks whether the vertex is isolated in the digraph.
    ///
    /// # Arguments
    ///
    /// * `u`: The vertex.
    #[must_use]
    fn is_isolated(&self, u: usize) -> bool;
}

impl<D> IsIsolated for D
where
    D: Indegree + Outdegree,
{
    fn is_isolated(&self, u: usize) -> bool {
        self.indegree(u) == 0 && self.outdegree(u) == 0
    }
}

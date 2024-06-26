//! Determine whether a vertex in a digraph is isolated.
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
/// Provide an implementation of `is_isolated` that returns `true` if the vertex
/// is isolated and `false` otherwise OR implement `Indegree` and `Outdegree`.
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
///     fn indegree(&self, s: usize) -> usize {
///         self.arcs.iter().filter(|set| set.contains(&s)).count()
///     }
/// }
///
/// impl Outdegree for Digraph {
///     fn outdegree(&self, s: usize) -> usize {
///         self.arcs[s].len()
///     }
/// }
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
    /// Returns whether the vertex is isolated in the digraph.
    fn is_isolated(&self, s: usize) -> bool;
}

impl<T> IsIsolated for T
where
    T: Indegree + Outdegree,
{
    fn is_isolated(&self, s: usize) -> bool {
        self.indegree(s) == 0 && self.outdegree(s) == 0
    }
}

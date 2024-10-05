//! Check whether a vertex is isolated.
//!
//! A vertex is isolated if it has no incoming or outgoing arcs.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IsIsolated,
//! };
//!
//! let mut digraph = AdjacencyList::empty(4);
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

use crate::{
    Indegree,
    Outdegree,
};

/// Check whether a vertex is isolated.
///
/// # Implementing [`IsIsolated`] for a custom type
///
/// Provide an implementation of [`is_isolated`](IsIsolated::is_isolated) that
/// returns whether the vertex is isolated OR implement [`Indegree`] and
/// [`Outdegree`].
///
/// ```
/// use {
///     graaf::{
///         Indegree,
///         IsIsolated,
///         Outdegree,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Indegree for AdjacencyList {
///     fn indegree(&self, u: usize) -> usize {
///         self.arcs.iter().filter(|set| set.contains(&u)).count()
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
pub trait IsIsolated {
    /// Check whether the vertex is isolated in the digraph.
    ///
    /// # Arguments
    ///
    /// * `u`: The vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     IsIsolated,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(1, 0);
    ///
    /// assert!(!digraph.is_isolated(0));
    /// assert!(!digraph.is_isolated(1));
    /// assert!(digraph.is_isolated(2));
    /// ```
    #[must_use]
    fn is_isolated(&self, u: usize) -> bool;
}

impl<D> IsIsolated for D
where
    D: Indegree + Outdegree,
{
    fn is_isolated(&self, u: usize) -> bool {
        self.is_sink(u) && self.is_source(u)
    }
}

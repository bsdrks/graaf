//! Check whether a digraph contains an edge.
//!
//! To determine whether an arc exists from `u` to `v`, see [`HasArc`].
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Circuit,
//!     Empty,
//!     HasEdge,
//! };
//!
//! let digraph = AdjacencyList::circuit(2);
//!
//! assert!(digraph.has_edge(0, 1));
//! assert!(digraph.has_edge(1, 0));
//!
//! let mut digraph = AdjacencyList::empty(2);
//!
//! digraph.add_arc(0, 1);
//!
//! assert!(!digraph.has_edge(0, 1));
//! assert!(!digraph.has_edge(1, 0));
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(!digraph.has_edge(0, 1));
//! assert!(digraph.has_edge(0, 2));
//! assert!(!digraph.has_edge(1, 0));
//! assert!(!digraph.has_edge(1, 2));
//! assert!(digraph.has_edge(2, 0));
//! assert!(!digraph.has_edge(2, 1));
//! ```
//!
//! [`HasArc`]: crate::HasArc

use crate::HasArc;

/// Check whether a digraph contains an edge.
///
/// # Implementing [`HasEdge`] for a custom type
///
/// Provide an implementation of [`has_edge`](HasEdge::has_edge) that returns
/// `true` if the digraph contains the arc OR implement [`HasArc`].
///
/// ```
/// use {
///     graaf::{
///         HasArc,
///         HasEdge,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl HasArc for AdjacencyList {
///     fn has_arc(&self, u: usize, v: usize) -> bool {
///         self.arcs.get(u).map_or(false, |set| set.contains(&v))
///     }
/// }
///
/// let digraph = AdjacencyList {
///     arcs: vec![BTreeSet::from([1]), BTreeSet::from([0])],
/// };
///
/// assert!(digraph.has_edge(0, 1));
/// assert!(digraph.has_edge(1, 0));
/// ```
pub trait HasEdge {
    /// Check whether the digraph has an arc from `u` to `v` AND from `v`
    /// to `u`.
    ///
    /// # Arguments
    ///
    /// * `u`: The tail vertex.
    /// * `v`: The head vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Circuit,
    ///     Empty,
    ///     HasEdge,
    /// };
    ///
    /// let digraph = AdjacencyList::circuit(2);
    ///
    /// assert!(digraph.has_edge(0, 1));
    /// assert!(digraph.has_edge(1, 0));
    ///
    /// let mut digraph = AdjacencyList::empty(2);
    ///
    /// digraph.add_arc(0, 1);
    ///
    /// assert!(!digraph.has_edge(0, 1));
    /// assert!(!digraph.has_edge(1, 0));
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(2, 0);
    ///
    /// assert!(!digraph.has_edge(0, 1));
    /// assert!(digraph.has_edge(0, 2));
    /// assert!(!digraph.has_edge(1, 0));
    /// assert!(!digraph.has_edge(1, 2));
    /// assert!(digraph.has_edge(2, 0));
    /// assert!(!digraph.has_edge(2, 1));
    /// ```
    #[must_use]
    fn has_edge(&self, u: usize, v: usize) -> bool;
}

impl<D> HasEdge for D
where
    D: HasArc,
{
    fn has_edge(&self, u: usize, v: usize) -> bool {
        self.has_arc(u, v) && self.has_arc(v, u)
    }
}

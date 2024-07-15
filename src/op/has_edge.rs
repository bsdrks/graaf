//! Determine whether an edge exists between two vertices.
//!
//! To determine whether an arc exists from `u` to `v`, see [`HasArc`].
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::{
//!         Cycle,
//!         Empty,
//!     },
//!     op::{
//!         AddArc,
//!         HasEdge,
//!     },
//! };
//!
//! let digraph = Digraph::cycle(2);
//!
//! assert!(digraph.has_edge(0, 1));
//! assert!(digraph.has_edge(1, 0));
//!
//! let mut digraph = Digraph::empty(2);
//!
//! digraph.add_arc(0, 1);
//!
//! assert!(!digraph.has_edge(0, 1));
//! assert!(!digraph.has_edge(1, 0));
//!
//! let mut digraph = Digraph::empty(3);
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
//! [`HasArc`]: crate::op::HasArc

use super::HasArc;

/// Determine whether an edge exists between two vertices.
///
/// # How can I implement `HasEdge`?
///
/// Provide an implementation of `has_edge` that returns `true` if the
/// digraph contains the arc OR implement `HasArc`.
///
/// ```
/// use {
///     graaf::op::{
///         HasArc,
///         HasEdge,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl HasArc for Digraph {
///     fn has_arc(&self, u: usize, v: usize) -> bool {
///         self.arcs.get(u).map_or(false, |set| set.contains(&v))
///     }
/// }
///
/// let digraph = Digraph {
///     arcs: vec![BTreeSet::from([1]), BTreeSet::from([0])],
/// };
///
/// assert!(digraph.has_edge(0, 1));
/// assert!(digraph.has_edge(1, 0));
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     gen::{
///         Cycle,
///         Empty,
///     },
///     op::{
///         AddArc,
///         HasEdge,
///     },
/// };
///
/// let digraph = Digraph::cycle(2);
///
/// assert!(digraph.has_edge(0, 1));
/// assert!(digraph.has_edge(1, 0));
///
/// let mut digraph = Digraph::empty(2);
///
/// digraph.add_arc(0, 1);
///
/// assert!(!digraph.has_edge(0, 1));
/// assert!(!digraph.has_edge(1, 0));
///
/// let mut digraph = Digraph::empty(3);
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
pub trait HasEdge {
    /// Determines whether the digraph has an arc from `u` to `v` AND from `v`
    /// to `u`.
    ///
    /// # Arguments
    ///
    /// * `u`: The tail vertex.
    /// * `v`: The head vertex.
    #[must_use]
    fn has_edge(&self, u: usize, v: usize) -> bool;
}

impl<T> HasEdge for T
where
    T: HasArc,
{
    fn has_edge(&self, u: usize, v: usize) -> bool {
        self.has_arc(u, v) && self.has_arc(v, u)
    }
}

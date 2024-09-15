//! Iterate a digraph's sinks.
//!
//! A sink is a vertex with no out-neighbors.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     Sinks,
//! };
//!
//! let mut digraph = AdjacencyList::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//!
//! assert!(digraph.sinks().eq([2, 3]));
//! ```

use crate::{
    Outdegree,
    Vertices,
};

/// Iterate a digraph's sinks.
///
/// # Implementing [`Sinks`] for a custom type
///
/// Provide an implementation of [`sinks`](Sinks::sinks) that returns an
/// iterator over the sinks in the digraph OR implement [`Outdegree`] and
/// [`Vertices`].
///
/// ```
/// use {
///     graaf::{
///         Outdegree,
///         Sinks,
///         Vertices,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Outdegree for AdjacencyList {
///     fn outdegree(&self, u: usize) -> usize {
///         self.arcs[u].len()
///     }
/// }
///
/// impl Vertices for AdjacencyList {
///     fn vertices(&self) -> impl Iterator<Item = usize> {
///         0..self.arcs.len()
///     }
/// }
///
/// let mut digraph = AdjacencyList {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([2]),
///         BTreeSet::new(),
///         BTreeSet::new(),
///     ],
/// };
///
/// assert!(digraph.sinks().eq([2, 3]));
/// ```
pub trait Sinks {
    /// Return an iterator over the sinks in the digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     Sinks,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(4);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 2);
    ///
    /// assert!(digraph.sinks().eq([2, 3]));
    /// ```
    fn sinks(&self) -> impl Iterator<Item = usize>;
}

impl<D> Sinks for D
where
    D: Outdegree + Vertices,
{
    fn sinks(&self) -> impl Iterator<Item = usize> {
        self.vertices().filter(move |&u| self.is_sink(u))
    }
}

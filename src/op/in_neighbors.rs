//! Return a digraph's in-neighbors.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     InNeighbors,
//! };
//!
//! let mut digraph = AdjacencyList::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//! digraph.add_arc(2, 0);
//! digraph.add_arc(2, 1);
//! digraph.add_arc(2, 3);
//! digraph.add_arc(3, 0);
//!
//! assert!(digraph.in_neighbors(0).eq([1, 2, 3]));
//! assert!(digraph.in_neighbors(1).eq([0, 2]));
//! assert!(digraph.in_neighbors(2).eq([0]));
//! assert!(digraph.in_neighbors(3).eq([2]));
//! ```
#![doc(alias = "iter_in_neighbours")]

use crate::Arcs;

/// Return a digraph's in-neighbors.
///
/// # Implementing [`InNeighbors`] for a custom type
///
/// Provide an implementation of [`in_neighbors`](InNeighbors::in_neighbors)
/// that returns an iterator over a vertex's in-neighbors OR implement
/// [`Arcs`].
///
/// ```
/// use {
///     graaf::InNeighbors,
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl InNeighbors for AdjacencyList {
///     fn in_neighbors(&self, v: usize) -> impl Iterator<Item = usize> {
///         self.arcs.iter().enumerate().filter_map(move |(u, set)| {
///             set.iter().find(|&&y| y == v).map(move |_| u)
///         })
///     }
/// }
///
/// let digraph = AdjacencyList {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([0]),
///         BTreeSet::from([0, 1, 3]),
///         BTreeSet::from([0]),
///     ],
/// };
///
/// assert!(digraph.in_neighbors(0).eq([1, 2, 3]));
/// assert!(digraph.in_neighbors(1).eq([0, 2]));
/// assert!(digraph.in_neighbors(2).eq([0]));
/// assert!(digraph.in_neighbors(3).eq([2]));
/// ```
#[doc(alias = "InNeighbours")]
pub trait InNeighbors {
    /// Return an iterator over a vertex's in-neighbors.
    ///
    /// # Arguments
    ///
    /// * `v`: The vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     InNeighbors,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(4);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 0);
    /// digraph.add_arc(2, 0);
    /// digraph.add_arc(2, 1);
    /// digraph.add_arc(2, 3);
    /// digraph.add_arc(3, 0);
    ///
    /// assert!(digraph.in_neighbors(0).eq([1, 2, 3]));
    /// assert!(digraph.in_neighbors(1).eq([0, 2]));
    /// assert!(digraph.in_neighbors(2).eq([0]));
    /// assert!(digraph.in_neighbors(3).eq([2]));
    /// ```
    #[doc(alias = "in_neighbours")]
    #[must_use]
    fn in_neighbors(&self, v: usize) -> impl Iterator<Item = usize>;
}

impl<D> InNeighbors for D
where
    D: Arcs,
{
    fn in_neighbors(&self, v: usize) -> impl Iterator<Item = usize> {
        self.arcs().filter_map(move |(x, y)| (v == y).then_some(x))
    }
}

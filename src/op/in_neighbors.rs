//! Return a digraph's in-neighbors.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         InNeighbors,
//!     },
//! };
//!
//! let mut digraph = Digraph::empty(4);
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

use super::Arcs;

/// Returns a digraph's in-neighbors.
///
/// # Implementing `InNeighbors`
///
/// Provide an implementation of `in_neighbors` that returns an iterator
/// over the in-neighbors of a vertex in the digraph OR implement `Arcs`.
///
/// ```
/// use {
///     graaf::op::InNeighbors,
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl InNeighbors for Digraph {
///     fn in_neighbors(&self, v: usize) -> impl Iterator<Item = usize> {
///         self.arcs.iter().enumerate().filter_map(move |(u, set)| {
///             set.iter().find(|&&v_| v_ == v).map(move |_| u)
///         })
///     }
/// }
///
/// let digraph = Digraph {
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
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     gen::Empty,
///     op::{
///         AddArc,
///         InNeighbors,
///     },
/// };
///
/// let mut digraph = Digraph::empty(4);
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
#[doc(alias = "InNeighbours")]
pub trait InNeighbors {
    /// Returns an iterator over the in-neighbors of a vertex in the digraph.
    ///
    /// # Arguments
    ///
    /// * `v`: The vertex.
    #[doc(alias = "in_neighbours")]
    #[must_use]
    fn in_neighbors(&self, v: usize) -> impl Iterator<Item = usize>;
}

impl<D> InNeighbors for D
where
    D: Arcs,
{
    fn in_neighbors(&self, v: usize) -> impl Iterator<Item = usize> {
        self.arcs()
            .filter_map(move |(u, v_)| (v == v_).then_some(u))
    }
}

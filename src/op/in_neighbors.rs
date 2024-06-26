//! Iterate over the in-neighbors of a vertex in a digraph.
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

/// Iterate over the in-neighbors of a vertex in a digraph.
///
/// # How can I implement `InNeighbors`?
///
/// Provide an implementation of `in_neighbors` that returns an iterator
/// over the in-neighbors of a vertex OR implement `Arcs`.
///
/// ```
/// use graaf::op::InNeighbors;
///
/// struct Digraph {
///     arcs: Vec<Vec<usize>>,
/// }
///
/// impl InNeighbors for Digraph {
///     fn in_neighbors(&self, t: usize) -> impl Iterator<Item = usize> {
///         self.arcs
///             .iter()
///             .enumerate()
///             .filter_map(move |(s, ts)| ts.iter().find(|&t_| *t_ == t).map(move |_| s))
///     }
/// }
///
/// let digraph = Digraph {
///     arcs: vec![vec![1, 2], vec![0], vec![0, 1, 3], vec![0]],
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
    /// * `t`: The head vertex.
    fn in_neighbors(&self, t: usize) -> impl Iterator<Item = usize>;
}

impl<D> InNeighbors for D
where
    D: Arcs,
{
    fn in_neighbors(&self, t: usize) -> impl Iterator<Item = usize> {
        self.arcs()
            .filter_map(move |(s, t_)| (t == t_).then_some(s))
    }
}

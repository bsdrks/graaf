//! Return a vertex's out-neighbors.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     OutNeighbors,
//! };
//!
//! let mut digraph = AdjacencyList::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(1, 3);
//! digraph.add_arc(2, 0);
//! digraph.add_arc(2, 1);
//! digraph.add_arc(2, 3);
//! digraph.add_arc(3, 1);
//! digraph.add_arc(3, 2);
//!
//! assert!(digraph.out_neighbors(0).eq([1, 2]));
//! assert!(digraph.out_neighbors(1).eq([0, 2, 3]));
//! assert!(digraph.out_neighbors(2).eq([0, 1, 3]));
//! assert!(digraph.out_neighbors(3).eq([1, 2]));
//! ```
#![doc(alias = "out_neighbours")]

/// Return a vertex's out-neighbors.
///
/// # Implementing [`OutNeighbors`] for a custom type
///
/// Provide an implementation of [`out_neighbors`](OutNeighbors::out_neighbors)
/// that returns an iterator over a vertex's out-neighbors.
///
/// ```
/// use {
///     graaf::OutNeighbors,
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl OutNeighbors for AdjacencyList {
///     fn out_neighbors(&self, u: usize) -> impl Iterator<Item = usize> {
///         self.arcs[u].iter().copied()
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
/// assert!(digraph.out_neighbors(0).eq([1, 2]));
/// assert!(digraph.out_neighbors(1).eq([0]));
/// assert!(digraph.out_neighbors(2).eq([0, 1, 3]));
/// assert!(digraph.out_neighbors(3).eq([0]));
/// ```
#[doc(alias = "IterOutNeighbours")]
pub trait OutNeighbors {
    /// Return an iterator over a vertex's out-neighbors.
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
    ///     OutNeighbors,
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
    /// assert!(digraph.out_neighbors(0).eq([1, 2]));
    /// assert!(digraph.out_neighbors(1).eq([0]));
    /// assert!(digraph.out_neighbors(2).eq([0, 1, 3]));
    /// assert!(digraph.out_neighbors(3).eq([0]));
    /// ```
    #[doc(alias = "out_neighbours")]
    #[must_use]
    fn out_neighbors(&self, u: usize) -> impl Iterator<Item = usize>;
}

//! Iterate a vertex's in-neighbors.
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

/// Iterate a vertex's in-neighbors.
#[doc(alias = "InNeighbours")]
pub trait InNeighbors {
    /// Iterate the vertex's in-neighbors.
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

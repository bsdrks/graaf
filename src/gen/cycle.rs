//! Generate cycle digraphs.
//!
//! A cycle is a digraph with a single bidirectional cycle.
//!
//! # Examples
//!
//! ## Order 2
//!
//! Generate a cycle digraph of order `2`.
//!
//! ![A cycle digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/cycle_2.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Cycle,
//! };
//!
//! assert!(AdjacencyList::cycle(2).arcs().eq([(0, 1), (1, 0)]));
//! ```
//!
//! ## Order 3
//!
//! Generate a cycle digraph of order `3`.
//!
//! ![A cycle digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/cycle_3.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Cycle,
//! };
//!
//! assert!(AdjacencyList::cycle(3).arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (1, 0),
//!     (1, 2),
//!     (2, 0),
//!     (2, 1)
//! ]));
//! ```
//!
//! ## Order 4
//!
//! Generate a cycle digraph of order `4`.
//!
//! ![A cycle digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/cycle_4.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Cycle,
//! };
//!
//! assert!(AdjacencyList::cycle(4).arcs().eq([
//!     (0, 1),
//!     (0, 3),
//!     (1, 0),
//!     (1, 2),
//!     (2, 1),
//!     (2, 3),
//!     (3, 0),
//!     (3, 2)
//! ]));
//! ```

/// Cycle digraphs
pub trait Cycle {
    /// Generate a cycle digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ## Order 2
    ///
    /// Generate a cycle digraph of order `2`.
    ///
    /// ![A cycle digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/cycle_2.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Cycle,
    /// };
    ///
    /// assert!(AdjacencyList::cycle(2).arcs().eq([(0, 1), (1, 0)]));
    /// ```
    ///
    /// ## Order 3
    ///
    /// Generate a cycle digraph of order `3`.
    ///
    /// ![A cycle digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/cycle_3.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Cycle,
    /// };
    ///
    /// assert!(AdjacencyList::cycle(3).arcs().eq([
    ///     (0, 1),
    ///     (0, 2),
    ///     (1, 0),
    ///     (1, 2),
    ///     (2, 0),
    ///     (2, 1)
    /// ]));
    /// ```
    ///
    /// ## Order 4
    ///
    /// Generate a cycle digraph of order `4`.
    ///
    /// ![A cycle digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/cycle_4.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Cycle,
    /// };
    ///
    /// assert!(AdjacencyList::cycle(4).arcs().eq([
    ///     (0, 1),
    ///     (0, 3),
    ///     (1, 0),
    ///     (1, 2),
    ///     (2, 1),
    ///     (2, 3),
    ///     (3, 0),
    ///     (3, 2)
    /// ]));
    /// ```
    #[must_use]
    fn cycle(order: usize) -> Self;
}

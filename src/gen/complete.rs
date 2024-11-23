//! Generate complete digraphs.
//!
//! In a complete digraph, an arc connects every ordered vertex pair.
//!
//! # Examples
//!
//! ## Order 2
//!
//! Generate a complete digraph of order `2`.
//!
//! ![A complete digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/complete_2.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Complete,
//! };
//!
//! assert!(AdjacencyList::complete(2).arcs().eq([(0, 1), (1, 0)]));
//! ```
//!
//! ## Order 3
//!
//! Generate a complete digraph of order `3`.
//!
//! ![A complete digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/complete_3.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Complete,
//! };
//!
//! assert!(AdjacencyList::complete(3).arcs().eq([
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
//! Generate a complete digraph of order `4`.
//!
//! ![A complete digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/complete_4.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Complete,
//! };
//!
//! assert!(AdjacencyList::complete(4).arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (0, 3),
//!     (1, 0),
//!     (1, 2),
//!     (1, 3),
//!     (2, 0),
//!     (2, 1),
//!     (2, 3),
//!     (3, 0),
//!     (3, 1),
//!     (3, 2)
//! ]));
//! ```

/// Complete digraphs
pub trait Complete {
    /// Generate a complete digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ## Order 2
    ///
    /// Generate a complete digraph of order `2`.
    ///
    /// ![A complete digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/complete_2.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Complete,
    /// };
    ///
    /// assert!(AdjacencyList::complete(2).arcs().eq([(0, 1), (1, 0)]));
    /// ```
    ///
    /// ## Order 3
    ///
    /// Generate a complete digraph of order `3`.
    ///
    /// ![A complete digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/complete_3.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Complete,
    /// };
    ///
    /// assert!(AdjacencyList::complete(3).arcs().eq([
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
    /// Generate a complete digraph of order `4`.
    ///
    /// ![A complete digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/complete_4.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Complete,
    /// };
    ///
    /// assert!(AdjacencyList::complete(4).arcs().eq([
    ///     (0, 1),
    ///     (0, 2),
    ///     (0, 3),
    ///     (1, 0),
    ///     (1, 2),
    ///     (1, 3),
    ///     (2, 0),
    ///     (2, 1),
    ///     (2, 3),
    ///     (3, 0),
    ///     (3, 1),
    ///     (3, 2)
    /// ]));
    /// ```
    #[must_use]
    fn complete(order: usize) -> Self;
}

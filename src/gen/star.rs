//! Generate star digraphs.
//!
//! A star digraph is a digraph with a single vertex connected to all others.
//!
//! # Examples
//!
//! ## Order 2
//!
//! Generate a star digraph of order `2`.
//!
//! ![A star digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_2.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Star,
//! };
//!
//! assert!(AdjacencyList::star(2).arcs().eq([(0, 1), (1, 0)]));
//! ```
//!
//! ## Order 3
//!
//! Generate a star digraph of order `3`.
//!
//! ![A star digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_3.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Star,
//! };
//!
//! assert!(AdjacencyList::star(3).arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (1, 0),
//!     (2, 0)
//! ]));
//! ```
//!
//! ## Order 4
//!
//! Generate a star digraph of order `4`.
//!
//! ![A star digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_4.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Star,
//! };
//!
//! assert!(AdjacencyList::star(4).arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (0, 3),
//!     (1, 0),
//!     (2, 0),
//!     (3, 0)
//! ]));
//! ```

/// Star digraphs
pub trait Star {
    /// Generate a star digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ## Order 2
    ///
    /// Generate a star digraph of order `2`.
    ///
    /// ![A star digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_2.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Star,
    /// };
    ///
    /// assert!(AdjacencyList::star(2).arcs().eq([(0, 1), (1, 0)]));
    /// ```
    ///
    /// ## Order 3
    ///
    /// Generate a star digraph of order `3`.
    ///
    /// ![A star digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_3.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Star,
    /// };
    ///
    /// assert!(AdjacencyList::star(3).arcs().eq([
    ///     (0, 1),
    ///     (0, 2),
    ///     (1, 0),
    ///     (2, 0)
    /// ]));
    /// ```
    ///
    /// ## Order 4
    ///
    /// Generate a star digraph of order `4`.
    ///
    /// ![A star digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_4.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Star,
    /// };
    ///
    /// assert!(AdjacencyList::star(4).arcs().eq([
    ///     (0, 1),
    ///     (0, 2),
    ///     (0, 3),
    ///     (1, 0),
    ///     (2, 0),
    ///     (3, 0)
    /// ]));
    /// ```
    #[must_use]
    fn star(order: usize) -> Self;
}

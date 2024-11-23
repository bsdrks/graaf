//! Generate path digraphs.
//!
//! A path digraph is an arc chain that connects vertices in a linear sequence.
//!
//! # Examples
//!
//! ## Order 2
//!
//! Generate a path digraph of order `2`.
//!
//! ![A path digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_2.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Path,
//! };
//!
//! assert!(AdjacencyList::path(2).arcs().eq([(0, 1)]));
//! ```
//!
//! ## Order 3
//!
//! Generate a path digraph of order `3`.
//!
//! ![A path digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_3.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Path,
//! };
//!
//! assert!(AdjacencyList::path(3).arcs().eq([(0, 1), (1, 2)]));
//! ```
//!
//! ## Order 4
//!
//! Generate a path digraph of order `4`.
//!
//! ![A path digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_4.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Path,
//! };
//!
//! assert!(AdjacencyList::path(4).arcs().eq([(0, 1), (1, 2), (2, 3)]));
//! ```

/// Path digraphs
pub trait Path {
    /// Generate a path digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ## Order 2
    ///
    /// Generate a path digraph of order `2`.
    ///
    /// ![A path digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_2.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Path,
    /// };
    ///
    /// assert!(AdjacencyList::path(2).arcs().eq([(0, 1)]));
    /// ```
    ///
    /// ## Order 3
    ///
    /// Generate a path digraph of order `3`.
    ///
    /// ![A path digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_3.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Path,
    /// };
    ///
    /// assert!(AdjacencyList::path(3).arcs().eq([(0, 1), (1, 2)]));
    /// ```
    ///
    /// ## Order 4
    ///
    /// Generate a path digraph of order `4`.
    ///
    /// ![A path digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_4.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Path,
    /// };
    ///
    /// assert!(AdjacencyList::path(4).arcs().eq([(0, 1), (1, 2), (2, 3)]));
    /// ```
    #[must_use]
    fn path(order: usize) -> Self;
}

//! Generate wheel digraphs.
//!
//! A wheel digraph is a circuit digraph with an additional vertex connected to
//! all others. A wheel digraph has `4` or more vertices.
//!
//! # Examples
//!
//! ## Order < 4
//!
//! A wheel digraph has at least four vertices.
//!
//! ```should_panic
//! use graaf::{
//!     AdjacencyList,
//!     Wheel,
//! };
//!
//! let _ = AdjacencyList::wheel(3);
//! ```
//!
//! ## Order 4
//!
//! Generate a wheel digraph of order `4`.
//!
//! ![A wheel digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/wheel_4-0.87.5.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Wheel,
//! };
//!
//! assert!(AdjacencyList::wheel(4).arcs().eq([
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
//!     (3, 2),
//! ]));
//! ```
//!
//! ## Order 5
//!
//! Generate a wheel digraph of order `5`.
//!
//! ![A wheel digraph of order `5`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/wheel_5-0.87.5.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Wheel,
//! };
//!
//! assert!(AdjacencyList::wheel(5).arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (0, 3),
//!     (0, 4),
//!     (1, 0),
//!     (1, 2),
//!     (1, 4),
//!     (2, 0),
//!     (2, 1),
//!     (2, 3),
//!     (3, 0),
//!     (3, 2),
//!     (3, 4),
//!     (4, 0),
//!     (4, 1),
//!     (4, 3),
//! ]));
//! ```
//!
//! ## Order 6
//!
//! Generate a wheel digraph of order `6`.
//!
//! ![A wheel digraph of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/wheel_6-0.87.5.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Wheel,
//! };
//!
//! assert!(AdjacencyList::wheel(6).arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (0, 3),
//!     (0, 4),
//!     (0, 5),
//!     (1, 0),
//!     (1, 2),
//!     (1, 5),
//!     (2, 0),
//!     (2, 1),
//!     (2, 3),
//!     (3, 0),
//!     (3, 2),
//!     (3, 4),
//!     (4, 0),
//!     (4, 3),
//!     (4, 5),
//!     (5, 0),
//!     (5, 1),
//!     (5, 4),
//! ]));
//! ```

/// Wheel digraphs
pub trait Wheel {
    /// Generate a wheel digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ## Order 4
    ///
    /// Generate a wheel digraph of order `4`.
    ///
    /// ![A wheel digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/wheel_4-0.87.5.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Wheel,
    /// };
    ///
    /// assert!(AdjacencyList::wheel(4).arcs().eq([
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
    ///     (3, 2),
    /// ]));
    /// ```
    ///
    /// ## Order 5
    ///
    /// Generate a wheel digraph of order `5`.
    ///
    /// ![A wheel digraph of order `5`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/wheel_5-0.87.5.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Wheel,
    /// };
    ///
    /// assert!(AdjacencyList::wheel(5).arcs().eq([
    ///     (0, 1),
    ///     (0, 2),
    ///     (0, 3),
    ///     (0, 4),
    ///     (1, 0),
    ///     (1, 2),
    ///     (1, 4),
    ///     (2, 0),
    ///     (2, 1),
    ///     (2, 3),
    ///     (3, 0),
    ///     (3, 2),
    ///     (3, 4),
    ///     (4, 0),
    ///     (4, 1),
    ///     (4, 3),
    /// ]));
    /// ```
    ///
    /// ## Order 6
    ///
    /// Generate a wheel digraph of order `6`.
    ///
    /// ![A wheel digraph of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/wheel_6-0.87.5.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Wheel,
    /// };
    ///
    /// assert!(AdjacencyList::wheel(6).arcs().eq([
    ///     (0, 1),
    ///     (0, 2),
    ///     (0, 3),
    ///     (0, 4),
    ///     (0, 5),
    ///     (1, 0),
    ///     (1, 2),
    ///     (1, 5),
    ///     (2, 0),
    ///     (2, 1),
    ///     (2, 3),
    ///     (3, 0),
    ///     (3, 2),
    ///     (3, 4),
    ///     (4, 0),
    ///     (4, 3),
    ///     (4, 5),
    ///     (5, 0),
    ///     (5, 1),
    ///     (5, 4),
    /// ]));
    /// ```
    #[must_use]
    fn wheel(order: usize) -> Self;
}

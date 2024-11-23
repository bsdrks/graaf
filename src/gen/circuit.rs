//! Generate circuit digraphs.
//!
//! A circuit is an oriented cycle.
//!
//! # Examples
//!
//! ## Order 2
//!
//! Generate a circuit digraph of order `2`.
//!
//! ![A circuit digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/circuit_2.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Circuit,
//! };
//!
//! assert!(AdjacencyList::circuit(2).arcs().eq([(0, 1), (1, 0)]));
//! ```
//!
//! ## Order 3
//!
//! Generate a circuit digraph of order `3`.
//!
//! ![A circuit digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/circuit_3.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Circuit,
//! };
//!
//! assert!(AdjacencyList::circuit(3)
//!     .arcs()
//!     .eq([(0, 1), (1, 2), (2, 0)]));
//! ```
//!
//! ## Order 4
//!
//! Generate a circuit digraph of order `4`.
//!
//! ![A circuit digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/circuit_4.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Circuit,
//! };
//!
//! assert!(AdjacencyList::circuit(4).arcs().eq([
//!     (0, 1),
//!     (1, 2),
//!     (2, 3),
//!     (3, 0)
//! ]));
//! ```

/// Circuit digraphs
pub trait Circuit {
    /// Generate a circuit digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ## Order 2
    ///
    /// Generate a circuit digraph of order `2`.
    ///
    /// ![A circuit digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/circuit_2.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Circuit,
    /// };
    ///
    /// assert!(AdjacencyList::circuit(2).arcs().eq([(0, 1), (1, 0)]));
    /// ```
    ///
    /// ## Order 3
    ///
    /// Generate a circuit digraph of order `3`.
    ///
    /// ![A circuit digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/circuit_3.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Circuit,
    /// };
    ///
    /// assert!(AdjacencyList::circuit(3)
    ///     .arcs()
    ///     .eq([(0, 1), (1, 2), (2, 0)]));
    /// ```
    ///
    /// ## Order 4
    ///
    /// Generate a circuit digraph of order `4`.
    ///
    /// ![A circuit digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/circuit_4.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Circuit,
    /// };
    ///
    /// assert!(AdjacencyList::circuit(4).arcs().eq([
    ///     (0, 1),
    ///     (1, 2),
    ///     (2, 3),
    ///     (3, 0)
    /// ]));
    /// ```
    #[must_use]
    fn circuit(order: usize) -> Self;
}

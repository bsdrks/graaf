//! Generate growing network.
//!
//! A growing network is a digraph that starts with a single vertex and adds a
//! new vertex with an arc to an existing vertex at each step.
//!
//! # Examples
//!
//! Generate a growing network of order `6`.
//!
//! ![A growing network of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/growing_network_1-0.89.3.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     GrowingNetwork,
//! };
//!
//! assert!(AdjacencyList::growing_network(6, 0).arcs().eq([
//!     (1, 0),
//!     (2, 0),
//!     (3, 1),
//!     (4, 0),
//!     (5, 2)
//! ]));
//! ```

/// Growing networks
pub trait GrowingNetwork {
    /// Generate a growing network.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    /// * `seed` - The seed for the random number generator.
    ///
    /// # Panics
    ///
    /// * Panics if `order` is zero.
    ///
    /// # Examples
    ///
    /// Generate a growing network of order `6`.
    ///
    /// ![A growing network of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/growing_network_1-0.89.3.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     GrowingNetwork,
    /// };
    ///
    /// assert!(AdjacencyList::growing_network(6, 0).arcs().eq([
    ///     (1, 0),
    ///     (2, 0),
    ///     (3, 1),
    ///     (4, 0),
    ///     (5, 2)
    /// ]));
    /// ```
    #[must_use]
    fn growing_network(order: usize, seed: u64) -> Self;
}

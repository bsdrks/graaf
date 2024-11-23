//! Generate Erdős-Rényi digraphs.
//!
//! The Erdős-Rényi model generates a random digraph with a given number of
//! vertices.
//!
//! Runs in **O(v²)** time, where **v** is the number of vertices.
//!
//! # Examples
//!
//! Generate a random Erdős-Rényi digraph of order `6` with a probability of
//! `0.5`.
//!
//! ![A random Erdős-Rényi digraph of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/erdos_renyi_1-0.89.2.svg?)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     ErdosRenyi,
//! };
//!
//! let digraph = AdjacencyList::erdos_renyi(6, 0.5, 0);
//!
//! assert!(digraph.arcs().eq([
//!     (0, 4),
//!     (0, 5),
//!     (1, 2),
//!     (1, 3),
//!     (1, 4),
//!     (2, 0),
//!     (2, 1),
//!     (2, 4),
//!     (3, 1),
//!     (4, 0),
//!     (4, 1),
//!     (4, 2),
//!     (5, 1),
//!     (5, 3)
//! ]));
//! ```

/// Erdős-Rényi digraphs
pub trait ErdosRenyi {
    /// Generate an Erdős-Rényi digraph.
    ///
    /// The Erdős-Rényi model generates a random digraph with a given number of
    /// vertices.
    ///
    /// Runs in **O(v²)** time, where **v** is the number of vertices.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    /// * `p` - The probability of an arc between two vertices.
    ///
    /// # Examples
    ///
    /// Generate a random Erdős-Rényi digraph of order `6` with a probability
    /// of `0.5`.
    ///
    /// ![A random Erdős-Rényi digraph of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/erdos_renyi_1-0.89.2.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     ErdosRenyi,
    /// };
    ///
    /// let digraph = AdjacencyList::erdos_renyi(6, 0.5, 0);
    ///
    /// assert!(digraph.arcs().eq([
    ///     (0, 4),
    ///     (0, 5),
    ///     (1, 2),
    ///     (1, 3),
    ///     (1, 4),
    ///     (2, 0),
    ///     (2, 1),
    ///     (2, 4),
    ///     (3, 1),
    ///     (4, 0),
    ///     (4, 1),
    ///     (4, 2),
    ///     (5, 1),
    ///     (5, 3)
    /// ]));
    /// ```
    #[must_use]
    fn erdos_renyi(order: usize, p: f64, seed: u64) -> Self;
}

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
//! ![Random Erdős-Rényi digraph of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/erdos_renyi_1-0.89.2.svg?)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     ErdosRenyi,
//!     Order,
//!     Size,
//! };
//!
//! let digraph = AdjacencyList::erdos_renyi(6, 0.5, 0);
//!
//! assert!(digraph.arcs().eq([
//!     (0, 3),
//!     (0, 4),
//!     (1, 2),
//!     (2, 0),
//!     (2, 1),
//!     (2, 3),
//!     (2, 5),
//!     (3, 0),
//!     (3, 2),
//!     (3, 5),
//!     (4, 0),
//!     (4, 1),
//!     (4, 2),
//!     (5, 0)
//! ]));
//! ```

use crate::{
    gen::prng::Xoshiro256StarStar,
    AddArc,
    Empty,
};

/// Generate Erdős-Rényi digraphs.
///
/// The Erdős-Rényi model generates a random digraph with a given number of
/// vertices.
///
/// Runs in **O(v²)** time, where **v** is the number of vertices.
///
/// # Implementing [`ErdosRenyi`] for a custom type
///
/// Provide an implementation of [`erdos_renyi`](ErdosRenyi::erdos_renyi) that
/// generates an Erdős-Rényi digraph of a given `order` and `p` OR implement
/// `AddArc` and `Empty`.
///
/// ```
/// use {
///     graaf::{
///         gen::prng::Xoshiro256StarStar,
///         Empty,
///         ErdosRenyi,
///     },
///     std::collections::BTreeSet,
/// };
///
/// pub struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl ErdosRenyi for AdjacencyList {
///     /// # Panics
///     ///
///     /// * Panics if `p` is negative.
///     /// * Panics if `p` is greater than 1.0.
///     fn erdos_renyi(order: usize, p: f64, seed: u64) -> Self {
///         assert!(p >= 0.0, "p = {p} must be non-negative");
///         assert!(p <= 1.0, "p = {p} must be at most 1.0");
///
///         let mut digraph = Self {
///             arcs: vec![BTreeSet::new(); order],
///         };
///
///         let mut rng = Xoshiro256StarStar::new(seed);
///
///         for u in 0..order {
///             for v in (0..order).filter(|&v| v != u) {
///                 if rng.next_f64() < p {
///                     digraph.arcs[u].insert(v);
///                 }
///             }
///         }
///
///         digraph
///     }
/// }
///
/// let digraph = AdjacencyList::erdos_renyi(4, 0.5, 0);
///
/// assert!(digraph.arcs.len() <= 12);
/// ```
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
    /// ![Random Erdős-Rényi digraph of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/erdos_renyi_1-0.89.2.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     ErdosRenyi,
    ///     Order,
    ///     Size,
    /// };
    ///
    /// let digraph = AdjacencyList::erdos_renyi(6, 0.5, 0);
    ///
    /// assert!(digraph.arcs().eq([
    ///     (0, 3),
    ///     (0, 4),
    ///     (1, 2),
    ///     (2, 0),
    ///     (2, 1),
    ///     (2, 3),
    ///     (2, 5),
    ///     (3, 0),
    ///     (3, 2),
    ///     (3, 5),
    ///     (4, 0),
    ///     (4, 1),
    ///     (4, 2),
    ///     (5, 0)
    /// ]));
    /// ```
    #[must_use]
    fn erdos_renyi(order: usize, p: f64, seed: u64) -> Self;
}

impl<D> ErdosRenyi for D
where
    D: AddArc + Empty,
{
    /// # Panics
    ///
    /// * Panics if `p` is negative.
    /// * Panics if `p` is greater than 1.0.
    fn erdos_renyi(order: usize, p: f64, seed: u64) -> Self {
        let mut digraph = Self::empty(order);
        let mut rng = Xoshiro256StarStar::new(seed);

        for u in 0..order {
            for v in (0..order).filter(|&v| v != u) {
                if rng.next_f64() < p {
                    digraph.add_arc(u, v);
                }
            }
        }

        digraph
    }
}

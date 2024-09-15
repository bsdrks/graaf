//! Generate growing network.
//!
//! A growing network is a digraph that starts with a single vertex and adds a
//! new vertex with a directed edge to an existing vertex at each step.
//!
//! # Examples
//!
//! Generate a growing network of order `6`.
//!
//! ![Growing network of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/growing_network_1-0.89.3.svg)
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

use {
    super::prng::Xoshiro256StarStar,
    crate::{
        AddArc,
        Empty,
    },
};

/// Generate growing network.
///
/// A growing network is a digraph that starts with a single vertex and adds a
/// new vertex with a directed edge to an existing vertex at each step.
///
/// # Implementing [`GrowingNetwork`] for a custom type
///
/// Provide an implementation of
/// [`growing_network`](GrowingNetwork::growing_network) that generates a
/// growing network of a given `order` from a given `seed` OR implement
/// [`AddArc`] and [`Empty`].
///
/// ```
/// use {
///     graaf::{
///         AddArc,
///         Empty,
///         GrowingNetwork,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl AddArc for AdjacencyList {
///     fn add_arc(&mut self, u: usize, v: usize) {
///         self.arcs[u].insert(v);
///     }
/// }
///
/// impl Empty for AdjacencyList {
///     fn empty(order: usize) -> Self {
///         Self {
///             arcs: vec![BTreeSet::new(); order],
///         }
///     }
/// }
///
/// let digraph = AdjacencyList::growing_network(6, 0);
///
/// assert!(digraph.arcs.iter().eq(&[
///     BTreeSet::new(),
///     BTreeSet::from([0]),
///     BTreeSet::from([0]),
///     BTreeSet::from([1]),
///     BTreeSet::from([0]),
///     BTreeSet::from([2]),
/// ]));
/// ```
pub trait GrowingNetwork {
    /// Generate a growing network.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    /// * `seed` - The seed for the random number generator.
    ///
    /// # Examples
    ///
    /// Generate a growing network of order `6`.
    ///
    /// ![Growing network of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/growing_network_1-0.89.3.svg)
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

impl<D> GrowingNetwork for D
where
    D: AddArc + Empty,
{
    /// # Panics
    ///
    /// * Panics if `order` is zero.
    /// * Panics if the random number generator fails.
    /// * Panics if conversion from `u64` to `usize` fails.
    fn growing_network(order: usize, seed: u64) -> Self {
        let mut digraph = D::empty(order);
        let mut rng = Xoshiro256StarStar::new(seed);

        for u in 1..order {
            let v = rng.next().expect("RNG failed");
            let v = usize::try_from(v).expect("conversion failed") % u;

            digraph.add_arc(u, v);
        }

        digraph
    }
}

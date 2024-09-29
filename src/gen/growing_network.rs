//! Generate growing network.
//!
//! A growing network is a digraph that starts with a single vertex and adds a
//! new vertex with an arc to an existing vertex at each step.
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

/// Generate growing network.
///
/// A growing network is a digraph that starts with a single vertex and adds a
/// new vertex with an arc to an existing vertex at each step.
///
/// # Implementing [`GrowingNetwork`] for a custom type
///
/// Provide an implementation of
/// [`growing_network`](GrowingNetwork::growing_network) that generates a
/// growing network of a given `order` from a given `seed`.
///
/// ```
/// use {
///     graaf::{
///         gen::prng::Xoshiro256StarStar,
///         GrowingNetwork,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl GrowingNetwork for AdjacencyList {
///     fn growing_network(order: usize, seed: u64) -> Self {
///         let mut arcs = Vec::with_capacity(order);
///         let mut rng = Xoshiro256StarStar::new(seed);
///
///         arcs.push(BTreeSet::new());
///
///         for (u, v) in (1..order).zip(rng) {
///             arcs.push(BTreeSet::from([usize::try_from(v)
///                 .expect("conversion failed")
///                 % u]));
///         }
///
///         Self { arcs }
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
///         
/// Implementations can be built with the [`AddArc`] and [`Empty`] traits.
///
/// ```
/// use {
///     graaf::{
///         gen::prng::Xoshiro256StarStar,
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
/// impl GrowingNetwork for AdjacencyList {
///     fn growing_network(order: usize, seed: u64) -> Self {
///         let mut digraph = Self::empty(order);
///         let rng = Xoshiro256StarStar::new(seed);
///
///         for (u, v) in (1..order).zip(rng) {
///             digraph.add_arc(
///                 u,
///                 usize::try_from(v).expect("conversion failed") % u,
///             );
///         }
///
///         digraph
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

//! Generate Erdős-Rényi digraphs.
//!
//! The Erdős-Rényi model generates a random digraph with a given number of
//! vertices. The time complexity is *O*(*v*²).
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::ErdosRenyi,
//!     op::{
//!         Order,
//!         Size,
//!     },
//! };
//!
//! let tournament = Digraph::erdos_renyi(4, 0.5);
//!
//! assert_eq!(tournament.order(), 4);
//! ```

use {
    super::prng::Xoshiro256StarStar,
    crate::{
        gen::Empty,
        op::AddArc,
    },
};

/// Generate Erdős-Rényi digraphs.
///
/// # How can I implement `ErdosRenyi`?
///
/// Provide an implementation of `random_tournament` that generates a random
///
/// ```
/// use {
///     graaf::gen::{
///         prng::Xoshiro256StarStar,
///         Empty,
///         ErdosRenyi,
///     },
///     std::collections::BTreeSet,
/// };
///
/// pub struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl ErdosRenyi for Digraph {
///     /// # Panics
///     ///
///     /// * Panics if `p` is negative.
///     /// * Panics if `p` is greater than 1.0.
///     fn erdos_renyi(order: usize, p: f64) -> Self {
///         assert!(p >= 0.0, "p = {p} must be non-negative");
///         assert!(p <= 1.0, "p = {p} must be at most 1.0");
///
///         let mut digraph = Self {
///             arcs: vec![BTreeSet::new(); order],
///         };
///
///         let mut rng = Xoshiro256StarStar::new(order as u64);
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
/// let tournament = Digraph::erdos_renyi(4, 0.5);
///
/// assert!((0..=12).contains(&tournament.arcs.len()));
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     gen::ErdosRenyi,
///     op::{
///         Degree,
///         Indegree,
///         Order,
///         Outdegree,
///         Size,
///         Vertices,
///     },
/// };
///
/// let digraph = Digraph::erdos_renyi(4, 0.5);
///
/// assert!(digraph.size() <= 12);
/// assert_eq!(digraph.order(), 4);
///
/// for s in digraph.vertices() {
///     assert!((0..=6).contains(&digraph.degree(s)));
///     assert!((0..=3).contains(&digraph.indegree(s)));
///     assert!((0..=3).contains(&digraph.outdegree(s)));
/// }
/// ```
pub trait ErdosRenyi {
    /// Generates a random Erdős-Rényi digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    /// * `p` - The probability of an arc between two vertices.
    #[must_use]
    fn erdos_renyi(order: usize, p: f64) -> Self;
}

impl<D> ErdosRenyi for D
where
    D: AddArc + Empty,
{
    /// # Panics
    ///
    /// * Panics if `p` is negative.
    /// * Panics if `p` is greater than 1.0.
    fn erdos_renyi(order: usize, p: f64) -> Self {
        let mut digraph = Self::empty(order);
        let mut rng = Xoshiro256StarStar::new(order as u64);

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

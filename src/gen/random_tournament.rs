//! Generate random tournaments.
//!
//! A tournament is a digraph in which an arc connects every unordered pair of
//! distinct vertices.
//!
//! # Examples
//!
//! Generate a random tournament of order `6`.
//!
//! ![Random tournament of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/random_tournament_1-0.89.2.svg?)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     RandomTournament,
//! };
//!
//! let tournament = AdjacencyList::random_tournament(6, 0);
//!
//! assert!(tournament.arcs().eq([
//!     (0, 5),
//!     (1, 0),
//!     (1, 4),
//!     (1, 5),
//!     (2, 0),
//!     (2, 1),
//!     (2, 3),
//!     (2, 5),
//!     (3, 0),
//!     (3, 1),
//!     (3, 5),
//!     (4, 0),
//!     (4, 2),
//!     (4, 3),
//!     (5, 4)
//! ]));
//! ```

/// Generate random tournaments.
///
/// A tournament is a digraph in which an arc connects every unordered pair of
/// distinct vertices.
///
/// # Implementing [`RandomTournament`] for a custom type
///
/// Provide an implementation of
/// [`random_tournament`](RandomTournament::random_tournament) that generates a
/// random tournament of a given `order`.
///
/// ```
/// use {
///     graaf::{
///         gen::prng::Xoshiro256StarStar,
///         RandomTournament,
///     },
///     std::collections::BTreeSet,
/// };
///
/// pub struct Tournament {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl RandomTournament for Tournament {
///     fn random_tournament(order: usize, seed: u64) -> Self {
///         let mut rng = Xoshiro256StarStar::new(seed);
///
///         let mut tournament = Self {
///             arcs: vec![BTreeSet::new(); order],
///         };
///
///         for u in 0..order {
///             for v in (u + 1)..order {
///                 if rng.next_bool() {
///                     tournament.arcs[u].insert(v);
///                 } else {
///                     tournament.arcs[v].insert(u);
///                 }
///             }
///         }
///
///         tournament
///     }
/// }
///
/// let tournament = Tournament::random_tournament(4, 0);
///
/// assert_eq!(tournament.arcs.len(), 4);
///
/// assert_eq!(
///     tournament.arcs.iter().map(|set| set.len()).sum::<usize>(),
///     6
/// );
///
/// for u in 0..tournament.arcs.len() {
///     assert!((0..3).contains(&tournament.arcs[u].len()));
/// }
/// ```
///
/// Implementations can be built with the [`AddArc`](crate::AddArc) and
/// [`Empty`](crate::Empty) traits.
///
/// ```
/// use {
///     graaf::{
///         gen::prng::Xoshiro256StarStar,
///         AddArc,
///         Empty,
///         RandomTournament,
///     },
///     std::collections::BTreeSet,
/// };
///
/// pub struct Tournament {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Empty for Tournament {
///     fn empty(order: usize) -> Self {
///         Self {
///             arcs: vec![BTreeSet::new(); order],
///         }
///     }
/// }
///
/// impl AddArc for Tournament {
///     fn add_arc(&mut self, u: usize, v: usize) {
///         self.arcs[u].insert(v);
///     }
/// }
///
/// impl RandomTournament for Tournament {
///     fn random_tournament(order: usize, seed: u64) -> Self {
///         let mut rng = Xoshiro256StarStar::new(seed);
///
///         let mut tournament = Self::empty(order);
///
///         for u in 0..order {
///             for v in (u + 1)..order {
///                 if rng.next_bool() {
///                     tournament.add_arc(u, v);
///                 } else {
///                     tournament.add_arc(v, u);
///                 }
///             }
///         }
///
///         tournament
///     }
/// }
///
/// let tournament = Tournament::random_tournament(4, 0);
///
/// assert_eq!(tournament.arcs.len(), 4);
///
/// assert_eq!(
///     tournament.arcs.iter().map(|set| set.len()).sum::<usize>(),
///     6
/// );
///
/// for u in 0..tournament.arcs.len() {
///     assert!((0..3).contains(&tournament.arcs[u].len()));
/// }
/// ```
pub trait RandomTournament {
    /// Generate a random tournament.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the tournament.
    /// * `seed` - The seed for the random number generator.
    ///
    /// # Examples
    ///
    /// Generate a random tournament of order `6`.
    ///
    /// ![Random tournament of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/random_tournament_1-0.89.2.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     RandomTournament,
    /// };
    ///
    /// let tournament = AdjacencyList::random_tournament(6, 0);
    ///
    /// assert!(tournament.arcs().eq([
    ///     (0, 5),
    ///     (1, 0),
    ///     (1, 4),
    ///     (1, 5),
    ///     (2, 0),
    ///     (2, 1),
    ///     (2, 3),
    ///     (2, 5),
    ///     (3, 0),
    ///     (3, 1),
    ///     (3, 5),
    ///     (4, 0),
    ///     (4, 2),
    ///     (4, 3),
    ///     (5, 4)
    /// ]));
    /// ```
    #[must_use]
    fn random_tournament(order: usize, seed: u64) -> Self;
}

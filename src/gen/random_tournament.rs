//! Generate random tournaments.
//!
//! A tournament is a digraph in which an arc connects every unordered pair of
//! distinct vertices.
//!
//! # Examples
//!
//! Generate a random tournament of order `6`.
//!
//! ![A random tournament of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/random_tournament_1-0.89.2.svg?)
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

/// Random tournaments
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
    /// ![A random tournament of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/random_tournament_1-0.89.2.svg?)
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

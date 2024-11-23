//! Check whether a digraph is a tournament.
//!
//! A tournament is a digraph in which there is one arc between every unordered
//! pair of distinct vertices.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Circuit,
//!     Complete,
//!     Empty,
//!     IsTournament,
//!     RandomTournament,
//! };
//!
//! assert!(!AdjacencyList::empty(3).is_tournament());
//! assert!(!AdjacencyList::complete(3).is_tournament());
//! assert!(AdjacencyList::circuit(3).is_tournament());
//! assert!(AdjacencyList::random_tournament(3, 0).is_tournament());
//! ```

/// Check whether a digraph is a tournament.
pub trait IsTournament {
    /// Check whether the digraph is a tournament.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Circuit,
    ///     Complete,
    ///     Empty,
    ///     IsTournament,
    ///     RandomTournament,
    /// };
    ///
    /// assert!(!AdjacencyList::empty(3).is_tournament());
    /// assert!(!AdjacencyList::complete(3).is_tournament());
    /// assert!(AdjacencyList::circuit(3).is_tournament());
    /// assert!(AdjacencyList::random_tournament(3, 0).is_tournament());
    /// ```
    #[must_use]
    fn is_tournament(&self) -> bool;
}

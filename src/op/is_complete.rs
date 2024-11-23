//! Check whether a digraph is complete.
//!
//! A digraph is complete if, for every pair `u`, `v` of distinct vertices,
//! there is an arc from `u` to `v` and an arc from `v` to `u`.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Circuit,
//!     Complete,
//!     Empty,
//!     IsComplete,
//!     RandomTournament,
//! };
//!
//! assert!(AdjacencyList::complete(3).is_complete());
//! assert!(!AdjacencyList::circuit(3).is_complete());
//! assert!(!AdjacencyList::empty(3).is_complete());
//! assert!(!AdjacencyList::random_tournament(3, 0).is_complete());
//! ```

/// Check whether a digraph is complete.
pub trait IsComplete {
    /// Check whether the digraph is complete.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Circuit,
    ///     Complete,
    ///     Empty,
    ///     IsComplete,
    ///     RandomTournament,
    /// };
    ///
    /// assert!(AdjacencyList::complete(3).is_complete());
    /// assert!(!AdjacencyList::circuit(3).is_complete());
    /// assert!(!AdjacencyList::empty(3).is_complete());
    /// assert!(!AdjacencyList::random_tournament(3, 0).is_complete());
    /// ```
    #[must_use]
    fn is_complete(&self) -> bool;
}

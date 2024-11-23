//! Check whether a digraph is semicomplete.
//!
//! A digraph is semicomplete if there is an arc between every unordered pair
//! of distinct vertices.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Circuit,
//!     Complete,
//!     Empty,
//!     IsSemicomplete,
//!     RandomTournament,
//! };
//!
//! assert!(!AdjacencyList::empty(3).is_semicomplete());
//! assert!(AdjacencyList::complete(3).is_semicomplete());
//! assert!(AdjacencyList::circuit(3).is_semicomplete());
//! assert!(AdjacencyList::random_tournament(3, 0).is_semicomplete());
//! ```

/// Check whether a digraph is semicomplete.
pub trait IsSemicomplete {
    /// Check whether the digraph is semicomplete.
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Circuit,
    ///     Complete,
    ///     Empty,
    ///     IsSemicomplete,
    ///     RandomTournament,
    /// };
    ///
    /// assert!(!AdjacencyList::empty(3).is_semicomplete());
    /// assert!(AdjacencyList::complete(3).is_semicomplete());
    /// assert!(AdjacencyList::circuit(3).is_semicomplete());
    /// assert!(AdjacencyList::random_tournament(3, 0).is_semicomplete());
    /// ```
    #[must_use]
    fn is_semicomplete(&self) -> bool;
}

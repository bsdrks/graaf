//! Check whether a digraph is another digraph's superdigraph.
//!
//! If digraph `H` is a subdigraph of digraph `D`, then `D` is a superdigraph
//! of `H`; the vertex set of `H` is a subset of the vertex set of `D` and the
//! arc set of `H` is a subset of the arc set of `D`. Additionally, the
//! end-vertices of each arc in `H` must be vertices in `H`.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Circuit,
//!     Empty,
//!     IsSuperdigraph,
//! };
//!
//! let mut h = AdjacencyList::empty(3);
//!
//! h.add_arc(0, 1);
//!
//! let d = AdjacencyList::circuit(3);
//!
//! assert!(d.is_superdigraph(&h));
//!
//! h.add_arc(0, 2);
//!
//! assert!(!d.is_superdigraph(&h));
//! ```
//!
//! Every digraph is a superdigraph of itself.
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     IsSuperdigraph,
//!     RandomTournament,
//! };
//!
//! let tournament = AdjacencyList::random_tournament(4, 0);
//!
//! assert!(tournament.is_superdigraph(&tournament));
//! ```

use crate::IsSubdigraph;

/// Check whether a digraph is another digraph's superdigraph.
pub trait IsSuperdigraph {
    /// Check whether the digraph is another digraph's superdigraph.
    ///
    /// # Arguments
    ///
    /// * `d`: The digraph to compare against.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Circuit,
    ///     Empty,
    ///     IsSuperdigraph,
    /// };
    ///
    /// let mut h = AdjacencyList::empty(3);
    ///
    /// h.add_arc(0, 1);
    ///
    /// let d = AdjacencyList::circuit(4);
    ///
    /// assert!(d.is_superdigraph(&h));
    ///
    /// h.add_arc(0, 2);
    ///
    /// assert!(!d.is_superdigraph(&h));
    /// ```
    ///
    /// Every digraph is a superdigraph of itself.
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     IsSuperdigraph,
    ///     RandomTournament,
    /// };
    ///
    /// let tournament = AdjacencyList::random_tournament(4, 0);
    ///
    /// assert!(tournament.is_superdigraph(&tournament));
    /// ```
    #[must_use]
    fn is_superdigraph(&self, d: &Self) -> bool;
}

impl<D> IsSuperdigraph for D
where
    D: IsSubdigraph,
{
    fn is_superdigraph(&self, d: &Self) -> bool {
        d.is_subdigraph(self)
    }
}

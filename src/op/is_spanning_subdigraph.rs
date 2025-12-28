//! Check whether a digraph is another digraph's spanning subdigraph.
//!
//! A digraph `H` is a spanning subdigraph of a digraph `D` if the vertex set
//! of `H` equals the vertex set of `D` and the arc set of `H` is a subset of
//! the arc set of `D`.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Circuit,
//!     Empty,
//!     IsSpanningSubdigraph,
//! };
//!
//! let mut h = AdjacencyList::empty(3);
//!
//! h.add_arc(0, 1);
//!
//! let d = AdjacencyList::circuit(3);
//!
//! assert!(h.is_spanning_subdigraph(&d));
//! ```
//!
//! Every digraph is its own spanning subdigraph.
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     IsSpanningSubdigraph,
//!     RandomTournament,
//! };
//!
//! let tournament = AdjacencyList::random_tournament(4, 0);
//!
//! assert!(tournament.is_spanning_subdigraph(&tournament));
//! ```
//!
//! A digraph `H` with arcs not in the arc set of a digraph `D` isn't a
//! spanning subdigraph of `D`.
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IsSpanningSubdigraph,
//! };
//!
//! let mut h = AdjacencyList::empty(2);
//!
//! h.add_arc(0, 1);
//! h.add_arc(1, 0);
//!
//! let mut d = AdjacencyList::empty(2);
//!
//! d.add_arc(0, 1);
//!
//! assert!(!h.is_spanning_subdigraph(&d));
//! ```
//!
//! A digraph `H` with vertices not in the vertex set of a digraph `D` isn't a
//! spanning subdigraph of `D`.
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IsSpanningSubdigraph,
//! };
//!
//! let mut h = AdjacencyList::empty(2);
//!
//! h.add_arc(0, 1);
//!
//! let d = AdjacencyList::empty(2);
//!
//! assert!(!h.is_spanning_subdigraph(&d));
//! ```
//!
//! A digraph `H` with arcs whose end-vertices aren't in the vertex set of `H`
//! isn't a spanning subdigraph of a digraph `D`.
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IsSpanningSubdigraph,
//! };
//!
//! // The arc (0, 2) has end-vertex `2` which isn't in the vertex set of `H`.
//!
//! let mut h = AdjacencyList::empty(3);
//!
//! h.add_arc(0, 1);
//! h.add_arc(0, 2);
//! h.add_arc(1, 0);
//!
//! let mut d = AdjacencyList::empty(3);
//!
//! d.add_arc(0, 1);
//! d.add_arc(1, 0);
//!
//! assert!(!h.is_spanning_subdigraph(&d));
//! ```
//!
//! A digraph `H` isn't a spanning subdigraph of a digraph `D` if the vertex
//! set of `H` isn't equal to the vertex set of `D`.
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IsSpanningSubdigraph,
//! };
//!
//! let h = AdjacencyList::empty(2);
//! let d = AdjacencyList::empty(3);
//!
//! assert!(!h.is_spanning_subdigraph(&d));
//! ```

use crate::{
    Arcs,
    HasArc,
    Vertices,
};

/// Check whether a digraph is another digraph's spanning subdigraph.
pub trait IsSpanningSubdigraph {
    /// Check whether the digraph is another digraph's spanning subdigraph.
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
    ///     IsSpanningSubdigraph,
    /// };
    ///
    /// let mut h = AdjacencyList::empty(3);
    ///
    /// h.add_arc(0, 1);
    ///
    /// let d = AdjacencyList::circuit(3);
    ///
    /// assert!(h.is_spanning_subdigraph(&d));
    /// ```
    ///
    /// Every digraph is a spanning subdigraph of itself.
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     IsSpanningSubdigraph,
    ///     RandomTournament,
    /// };
    ///
    /// let tournament = AdjacencyList::random_tournament(4, 0);
    ///
    /// assert!(tournament.is_spanning_subdigraph(&tournament));
    /// ```
    ///
    /// A digraph `H` with arcs not in the arc set of a digraph `D` isn't a
    /// spanning subdigraph of `D`.
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     IsSpanningSubdigraph,
    /// };
    ///
    /// let mut h = AdjacencyList::empty(2);
    ///
    /// h.add_arc(0, 1);
    /// h.add_arc(1, 0);
    ///
    /// let mut d = AdjacencyList::empty(2);
    ///
    /// d.add_arc(0, 1);
    ///
    /// assert!(!h.is_spanning_subdigraph(&d));
    /// ```
    ///
    /// A digraph `H` with vertices not in the vertex set of a digraph `D` is
    /// not a spanning subdigraph of `D`.
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     IsSpanningSubdigraph,
    /// };
    ///
    /// let mut h = AdjacencyList::empty(2);
    ///
    /// h.add_arc(0, 1);
    ///
    /// let d = AdjacencyList::empty(2);
    ///
    /// assert!(!h.is_spanning_subdigraph(&d));
    /// ```
    ///
    /// A digraph `H` with arcs whose end-vertices aren't in the vertex set of
    /// `H` isn't a spanning subdigraph of a digraph `D`.
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     IsSpanningSubdigraph,
    /// };
    ///
    /// // The arc (0, 2) has end-vertex `2` which isn't in the vertex set
    /// // of `H`.
    ///
    /// let mut h = AdjacencyList::empty(3);
    ///
    /// h.add_arc(0, 1);
    /// h.add_arc(0, 2);
    /// h.add_arc(1, 0);
    ///
    /// let mut d = AdjacencyList::empty(3);
    ///
    /// d.add_arc(0, 1);
    /// d.add_arc(1, 0);
    ///
    /// assert!(!h.is_spanning_subdigraph(&d));
    /// ```
    ///
    /// A digraph `H` isn't a spanning subdigraph of a digraph `D` if the
    /// vertex set of `H` isn't equal to the vertex set of `D`.
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     IsSpanningSubdigraph,
    /// };
    ///
    /// let h = AdjacencyList::empty(2);
    /// let d = AdjacencyList::empty(3);
    ///
    /// assert!(!h.is_spanning_subdigraph(&d));
    /// ```
    #[must_use]
    fn is_spanning_subdigraph(&self, d: &Self) -> bool;
}

impl<D> IsSpanningSubdigraph for D
where
    D: Arcs + HasArc + Vertices,
{
    fn is_spanning_subdigraph(&self, d: &Self) -> bool {
        self.vertices().eq(d.vertices())
            && self.arcs().all(|(u, v)| d.has_arc(u, v))
    }
}

//! Check whether a digraph is another digraph's subdigraph.
//!
//! A digraph `H` is a subdigraph of a digraph `D` if the vertex set of `H` is
//! a subset of the vertex set of `D` and the arc set of `H` is a subset of the
//! arc set of `D`.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Circuit,
//!     Empty,
//!     IsSubdigraph,
//! };
//!
//! let mut h = AdjacencyList::empty(3);
//!
//! h.add_arc(0, 1);
//!
//! let d = AdjacencyList::circuit(3);
//!
//! assert!(h.is_subdigraph(&d));
//! ```
//!
//! Every digraph is a subdigraph of itself.
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     IsSubdigraph,
//!     RandomTournament,
//! };
//!
//! let tournament = AdjacencyList::random_tournament(4, 0);
//!
//! assert!(tournament.is_subdigraph(&tournament));
//! ```
//!
//! A digraph `H` with arcs not in the arc set of a digraph `D` isn't a
//! subdigraph of `D`.
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IsSubdigraph,
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
//! assert!(!h.is_subdigraph(&d));
//! ```
//!
//! A digraph `H` with vertices not in the vertex set of a digraph `D` isn't a
//! subdigraph of `D`.
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IsSubdigraph,
//! };
//!
//! let mut h = AdjacencyList::empty(2);
//!
//! h.add_arc(0, 1);
//!
//! let d = AdjacencyList::empty(2);
//!
//! assert!(!h.is_subdigraph(&d));
//! ```
//!
//! A digraph `H` with arcs whose end-vertices aren't in the vertex set of `H`
//! isn't a subdigraph of a digraph `D`.
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IsSubdigraph,
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
//! assert!(!h.is_subdigraph(&d));
//! ```

use {
    crate::{
        Arcs,
        HasArc,
        Vertices,
    },
    std::collections::BTreeSet,
};

/// Check whether a digraph is another digraph's subdigraph.
pub trait IsSubdigraph {
    /// Check whether the digraph is another digraph's subdigraph.
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
    ///     IsSubdigraph,
    /// };
    ///
    /// let mut h = AdjacencyList::empty(3);
    ///
    /// h.add_arc(0, 1);
    ///
    /// let d = AdjacencyList::circuit(3);
    ///
    /// assert!(h.is_subdigraph(&d));
    /// ```
    ///
    /// Every digraph is a subdigraph of itself.
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     IsSubdigraph,
    ///     RandomTournament,
    /// };
    ///
    /// let tournament = AdjacencyList::random_tournament(4, 0);
    ///
    /// assert!(tournament.is_subdigraph(&tournament));
    /// ```
    ///
    /// A digraph `H` with arcs not in the arc set of a digraph `D` isn't a
    /// subdigraph of `D`.
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     IsSubdigraph,
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
    /// assert!(!h.is_subdigraph(&d));
    /// ```
    ///
    /// A digraph `H` with vertices not in the vertex set of a digraph `D` is
    /// not a subdigraph of `D`.
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     IsSubdigraph,
    /// };
    ///
    /// let mut h = AdjacencyList::empty(2);
    ///
    /// h.add_arc(0, 1);
    ///
    /// let d = AdjacencyList::empty(2);
    ///
    /// assert!(!h.is_subdigraph(&d));
    /// ```
    ///
    /// A digraph `H` with arcs whose end-vertices aren't in the vertex set of
    /// `H` isn't a subdigraph of a digraph `D`.
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     IsSubdigraph,
    /// };
    ///
    /// // The arc (0, 2) has end-vertex `2` which isn't in the vertex set of `H`.
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
    /// assert!(!h.is_subdigraph(&d));
    /// ```
    #[must_use]
    fn is_subdigraph(&self, d: &Self) -> bool;
}

impl<D> IsSubdigraph for D
where
    D: Arcs + HasArc + Vertices,
{
    fn is_subdigraph(&self, d: &Self) -> bool {
        let hv = self.vertices().collect::<BTreeSet<_>>();
        let dv = d.vertices().collect::<BTreeSet<_>>();

        self.arcs().all(|(u, v)| {
            d.has_arc(u, v) && hv.contains(&u) && hv.contains(&v)
        }) && hv.iter().all(|u| dv.contains(u))
    }
}

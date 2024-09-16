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
//! A digraph `H` with arcs not in the arc set of a digraph `D` is not a
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
//! A digraph `H` with vertices not in the vertex set of a digraph `D` is not a
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
//! A digraph `H` with arcs whose end-vertices are not in the vertex set of `H`
//! is not a spanning subdigraph of a digraph `D`.
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IsSpanningSubdigraph,
//! };
//!
//! // The arc (0, 2) has end-vertex `2` which is not in the vertex set of `H`.
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
//! A digraph `H` is not a spanning subdigraph of a digraph `D` if the vertex
//! set of `H` is not equal to the vertex set of `D`.
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
///
/// # Implementing [`IsSpanningSubdigraph`] for a custom type
///
/// Provide an implementation of
/// [`is_spanning_subdigraph`](IsSpanningSubdigraph::is_spanning_subdigraph)
/// that returns whether the digraph is the given digraph's spanning subdigraph
/// OR implement [`HasArc`], [`Arcs`], and [`Vertices`].
///
/// ```
/// use {
///     graaf::{
///         Arcs,
///         HasArc,
///         IsSpanningSubdigraph,
///         Vertices,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Arcs for AdjacencyList {
///     fn arcs(&self) -> impl Iterator<Item = (usize, usize)> {
///         self.arcs
///             .iter()
///             .enumerate()
///             .flat_map(|(u, set)| set.iter().map(move |&v| (u, v)))
///     }
/// }
///
/// impl HasArc for AdjacencyList {
///     fn has_arc(&self, u: usize, v: usize) -> bool {
///         self.arcs[u].contains(&v)
///     }
/// }
///
/// impl Vertices for AdjacencyList {
///     fn vertices(&self) -> impl Iterator<Item = usize> {
///         0..self.arcs.len()
///     }
/// }
///
/// let h = AdjacencyList {
///     arcs: vec![BTreeSet::from([1]), BTreeSet::new(), BTreeSet::new()],
/// };
///
/// let d = AdjacencyList {
///     arcs: vec![
///         BTreeSet::from([1]),
///         BTreeSet::from([2]),
///         BTreeSet::from([0]),
///     ],
/// };
///
/// assert!(h.is_spanning_subdigraph(&d));
/// ```
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
    /// A digraph `H` with arcs not in the arc set of a digraph `D` is not a
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
    /// A digraph `H` with arcs whose end-vertices are not in the vertex set of
    /// `H` is not a spanning subdigraph of a digraph `D`.
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     IsSpanningSubdigraph,
    /// };
    ///
    /// // The arc (0, 2) has end-vertex `2` which is not in the vertex set of `H`.
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
    /// A digraph `H` is not a spanning subdigraph of a digraph `D` if the
    /// vertex set of `H` is not equal to the vertex set of `D`.
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

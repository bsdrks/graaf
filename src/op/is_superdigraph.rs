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
///
/// If digraph `H` is a subdigraph of digraph `D`, then `D` is a superdigraph
/// of `H`; the vertex set of `H` is a subset of the vertex set of `D` and the
/// arc set of `H` is a subset of the arc set of `D`. Additionally, the
/// end-vertices of each arc in `H` must be vertices in `H`.
///
/// # Implementing [`IsSuperdigraph`] for a custom type
///
/// Provide an implementation of
/// [`is_superdigraph`](IsSuperdigraph::is_superdigraph) that returns whether
/// the digraph is a superdigraph of the given digraph OR implement
/// [`IsSubdigraph`].
///
/// ```
/// use {
///     graaf::{
///         Arcs,
///         Circuit,
///         HasArc,
///         IsSuperdigraph,
///         Vertices,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl HasArc for AdjacencyList {
///     fn has_arc(&self, u: usize, v: usize) -> bool {
///         self.arcs[u].contains(&v)
///     }
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
/// impl Vertices for AdjacencyList {
///     fn vertices(&self) -> impl Iterator<Item = usize> {
///         0..self.arcs.len()
///     }
/// }
///
/// let mut h = AdjacencyList {
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
/// assert!(d.is_superdigraph(&h));
///
/// h.arcs[0].insert(2);
///
/// assert!(!d.is_superdigraph(&h));
/// ```
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

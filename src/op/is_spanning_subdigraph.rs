//! Return whether a digraph is a spanning subdigraph of another digraph.
//!
//! A digraph `H` is a spanning subdigraph of a digraph `D` if the vertex set
//! of `H` equals the vertex set of `D` and the arc set of `H` is a subset of
//! the arc set of `D`.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::{
//!         Circuit,
//!         Empty,
//!     },
//!     op::{
//!         AddArc,
//!         IsSpanningSubdigraph,
//!     },
//! };
//!
//! let mut h = Digraph::empty(3);
//!
//! h.add_arc(0, 1);
//!
//! let d = Digraph::circuit(3);
//!
//! assert!(h.is_spanning_subdigraph(&d));
//! ```
//!
//! Every digraph is a spanning subdigraph of itself.
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::RandomTournament,
//!     op::IsSpanningSubdigraph,
//! };
//!
//! let tournament = Digraph::random_tournament(4, 0);
//!
//! assert!(tournament.is_spanning_subdigraph(&tournament));
//! ```
//!
//! A digraph `H` with arcs not in the arc set of a digraph `D` is not a
//! spanning subdigraph of `D`.
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         IsSpanningSubdigraph,
//!     },
//! };
//!
//! let mut h = Digraph::empty(2);
//!
//! h.add_arc(0, 1);
//! h.add_arc(1, 0);
//!
//! let mut d = Digraph::empty(2);
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
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         IsSpanningSubdigraph,
//!     },
//! };
//!
//! let mut h = Digraph::empty(2);
//!
//! h.add_arc(0, 1);
//!
//! let d = Digraph::empty(2);
//!
//! assert!(!h.is_spanning_subdigraph(&d));
//! ```
//!
//! A digraph `H` with arcs whose end-vertices are not in the vertex set of `H`
//! is not a spanning subdigraph of a digraph `D`.
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         IsSpanningSubdigraph,
//!     },
//! };
//!
//! // The arc (0, 2) has end-vertex `2` which is not in the vertex set of `H`.
//!
//! let mut h = Digraph::empty(3);
//!
//! h.add_arc(0, 1);
//! h.add_arc(0, 2);
//! h.add_arc(1, 0);
//!
//! let mut d = Digraph::empty(3);
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
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         IsSpanningSubdigraph,
//!     },
//! };
//!
//! let h = Digraph::empty(2);
//! let d = Digraph::empty(3);
//!
//! assert!(!h.is_spanning_subdigraph(&d));
//! ```

use super::{
    Arcs,
    HasArc,
    Vertices,
};

/// Return whether a digraph is a spanning subdigraph of another digraph.
///
/// # How can I implement `IsSpanningSubdigraph`?
///
/// Provide an implementation of `is_spanning_subdigraph` that returns whether
/// the digraph is a spanning subdigraph of the given digraph OR implement
/// `HasArc`, `Arcs`, and `Vertices`.
///
/// ```
/// use {
///     graaf::op::{
///         Arcs,
///         HasArc,
///         IsSpanningSubdigraph,
///         Vertices,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Arcs for Digraph {
///     fn arcs(&self) -> impl Iterator<Item = (usize, usize)> {
///         self.arcs
///             .iter()
///             .enumerate()
///             .flat_map(|(u, set)| set.iter().map(move |&v| (u, v)))
///     }
/// }
///
/// impl HasArc for Digraph {
///     fn has_arc(&self, u: usize, v: usize) -> bool {
///         self.arcs[u].contains(&v)
///     }
/// }
///
/// impl Vertices for Digraph {
///     fn vertices(&self) -> impl Iterator<Item = usize> {
///         0..self.arcs.len()
///     }
/// }
///
/// let h = Digraph {
///     arcs: vec![BTreeSet::from([1]), BTreeSet::new(), BTreeSet::new()],
/// };
///
/// let d = Digraph {
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
    /// Returns whether the digraph is a spanning subdigraph of another
    /// digraph.
    ///
    /// # Arguments
    ///
    /// * `d`: The digraph to compare against.
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

//! Determine whether a digraph is a subdigraph of another digraph.
//!
//! A digraph `H` is a subdigraph of a digraph `D` if the vertex set of `H` is a
//! subset of the vertex set of `D` and the arc set of `H` is a subset of the
//! arc set of `D`.
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
//!         IsSubdigraph,
//!     },
//! };
//!
//! let mut h = Digraph::empty(3);
//!
//! h.add_arc(0, 1);
//!
//! let d = Digraph::circuit(3);
//!
//! assert!(h.is_subdigraph(&d));
//! ```
//!
//! Every digraph is a subdigraph of itself.
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::RandomTournament,
//!     op::IsSubdigraph,
//! };
//!
//! let tournament = Digraph::random_tournament(4);
//!
//! assert!(tournament.is_subdigraph(&tournament));
//! ```
//!
//! A digraph `H` with arcs not in the arc set of a digraph `D` is not a
//! subdigraph of `D`.
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         IsSubdigraph,
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
//! assert!(!h.is_subdigraph(&d));
//! ```
//!
//! A digraph `H` with vertices not in the vertex set of a digraph `D` is not a
//! subdigraph of `D`.
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         IsSubdigraph,
//!     },
//! };
//!
//! let mut h = Digraph::empty(2);
//!
//! h.add_arc(0, 1);
//!
//! let d = Digraph::empty(2);
//!
//! assert!(!h.is_subdigraph(&d));
//! ```
//!
//! A digraph `H` with arcs whose end-vertices are not in the vertex set of `H`
//! is not a subdigraph of a digraph `D`.
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         IsSubdigraph,
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
//! assert!(!h.is_subdigraph(&d));
//! ```

use {
    super::{
        Arcs,
        HasArc,
        Vertices,
    },
    std::collections::BTreeSet,
};

/// Determine whether a digraph is a subdigraph of another digraph.
///
/// # How can I implement `IsSubdigraph`?
///
/// Provide an implementation of `is_subdigraph` that returns whether the
/// digraph is a subdigraph of the given digraph OR implement `HasArc`, `Arcs`,
/// and `Vertices`.
///
/// ```
/// use {
///     graaf::op::{
///         Arcs,
///         HasArc,
///         IsSubdigraph,
///         Vertices,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl HasArc for Digraph {
///     fn has_arc(&self, u: usize, v: usize) -> bool {
///         self.arcs[u].contains(&v)
///     }
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
/// impl Vertices for Digraph {
///     fn vertices(&self) -> impl Iterator<Item = usize> {
///         0..self.arcs.len()
///     }
/// }
///
/// let h = Digraph {
///     arcs: vec![BTreeSet::from([1]), BTreeSet::new()],
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
/// assert!(h.is_subdigraph(&d));
/// ```
pub trait IsSubdigraph {
    /// Returns whether the digraph is a subdigraph of another digraph.
    ///
    /// # Arguments
    ///
    /// * `d`: The digraph to compare against.
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

        self.arcs()
            .all(|(u, v)| d.has_arc(u, v) && hv.contains(&u) && hv.contains(&v))
            && hv.iter().all(|u| dv.contains(u))
    }
}

//! Determine whether a digraph is a superdigraph of another digraph.
//!
//! If digraph `H` is a subdigraph of digraph `D`, then `D` is a superdigraph of
//! `H`; the vertex set of `H` is a subset of the vertex set of `D` and the arc
//! set of `H` is a subset of the arc set of `D`. Additionally, the end-vertices
//! of each arc in `H` must be vertices in `H`.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::{
//!         Cycle,
//!         Empty,
//!     },
//!     op::{
//!         AddArc,
//!         IsSuperdigraph,
//!     },
//! };
//!
//! let mut h = Digraph::empty(3);
//!
//! h.add_arc(0, 1);
//!
//! let d = Digraph::cycle(3);
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
//!     adjacency_list::Digraph,
//!     gen::RandomTournament,
//!     op::IsSuperdigraph,
//! };
//!
//! let tournament = Digraph::random_tournament(4);
//!
//! assert!(tournament.is_superdigraph(&tournament));
//! ```

use crate::op::IsSubdigraph;

/// Determine whether a digraph is a superdigraph of another digraph.
///
/// If digraph `H` is a subdigraph of digraph `D`, then `D` is a superdigraph of
/// `H`; the vertex set of `H` is a subset of the vertex set of `D` and the arc
/// set of `H` is a subset of the arc set of `D`. Additionally, the end-vertices
/// of each arc in `H` must be vertices in `H`.
///
/// # How can I implement `IsSuperdigraph`?
///
/// Provide an implementation of `is_subdigraph` that returns `true` if the
/// digraph is a superdigraph of the digraph `d` and `false` otherwise OR
/// implement `IsSubdigraph`.
///
/// ```
/// use {
///     graaf::{
///         gen::Cycle,
///         op::{
///             Arcs,
///             HasArc,
///             IsSuperdigraph,
///             Vertices,
///         },
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
///             .flat_map(|(u, vs)| vs.iter().map(move |v| (u, *v)))
///     }
/// }
///
/// impl Vertices for Digraph {
///     fn vertices(&self) -> impl Iterator<Item = usize> {
///         (0..self.arcs.len())
///     }
/// }
///
/// let mut h = Digraph {
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
/// assert!(d.is_superdigraph(&h));
///
/// h.arcs[0].insert(2);
///
/// assert!(!d.is_superdigraph(&h));
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     gen::{
///         Cycle,
///         Empty,
///     },
///     op::{
///         AddArc,
///         IsSuperdigraph,
///     },
/// };
///
/// let mut h = Digraph::empty(2);
///
/// h.add_arc(0, 1);
///
/// let d = Digraph::cycle(3);
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
///     adjacency_list::Digraph,
///     gen::RandomTournament,
///     op::IsSuperdigraph,
/// };
///
/// let tournament = Digraph::random_tournament(4);
///
/// assert!(tournament.is_superdigraph(&tournament));
/// ```
pub trait IsSuperdigraph {
    /// Determines whether the digraph is a superdigraph of another digraph.
    fn is_superdigraph(&self, d: &Self) -> bool;
}

impl<T> IsSuperdigraph for T
where
    T: IsSubdigraph,
{
    fn is_superdigraph(&self, d: &Self) -> bool {
        d.is_subdigraph(self)
    }
}

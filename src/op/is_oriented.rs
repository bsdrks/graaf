//! Check whether a digraph is oriented.
//!
//! An oriented graph is a digraph with no cycle of length 2.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Circuit,
//!     IsOriented,
//! };
//!
//! assert!(!AdjacencyList::circuit(2).is_oriented());
//! assert!(AdjacencyList::circuit(3).is_oriented());
//! ```

use crate::{
    Arcs,
    HasArc,
};

/// Check whether a digraph is oriented.
///
/// # Implementing [`IsOriented`] for a custom type
///
/// Provide an implementation of [`is_oriented`](IsOriented::is_oriented) that
/// returns whether the digraph is oriented OR implement `Arcs` and `HasArc`.
///
/// ```
/// use {
///     graaf::{
///         Arcs,
///         Circuit,
///         HasArc,
///         IsOriented,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     pub arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Arcs for AdjacencyList {
///     fn arcs(&self) -> impl Iterator<Item = (usize, usize)> {
///         (0..self.arcs.len())
///             .flat_map(move |u| self.arcs[u].iter().map(move |&v| (u, v)))
///     }
/// }
///
/// impl HasArc for AdjacencyList {
///     fn has_arc(&self, u: usize, v: usize) -> bool {
///         self.arcs[u].contains(&v)
///     }
/// }
///
/// let digraph = AdjacencyList {
///     arcs: vec![BTreeSet::from([1]), BTreeSet::from([0])],
/// };
///
/// assert!(!digraph.is_oriented());
///
/// let digraph = AdjacencyList {
///     arcs: vec![
///         BTreeSet::from([1]),
///         BTreeSet::from([2]),
///         BTreeSet::from([0]),
///     ],
/// };
///
/// assert!(digraph.is_oriented());
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     AdjacencyList,
///     Circuit,
///     IsOriented,
/// };
///
/// assert!(!AdjacencyList::circuit(2).is_oriented());
/// assert!(AdjacencyList::circuit(3).is_oriented());
/// ```
pub trait IsOriented {
    /// Check whether the digraph is oriented.
    #[must_use]
    fn is_oriented(&self) -> bool;
}

impl<D> IsOriented for D
where
    D: Arcs + HasArc,
{
    fn is_oriented(&self) -> bool {
        self.arcs().all(|(u, v)| !self.has_arc(v, u))
    }
}

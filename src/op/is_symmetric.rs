//! Check whether a digraph is symmetric.
//!
//! A digraph is symmetric if for every arc `(u, v)` there is an arc
//! `(v, u)`.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IsSymmetric,
//!     RemoveArc,
//! };
//!
//! let mut digraph = AdjacencyList::empty(2);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 0);
//!
//! assert!(digraph.is_symmetric());
//!
//! digraph.remove_arc(1, 0);
//!
//! assert!(!digraph.is_symmetric());
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(!digraph.is_symmetric());
//! ```

use crate::{
    Arcs,
    HasArc,
};

/// Check whether a digraph is symmetric.
///
/// # Implementing [`IsSymmetric`] for a custom type
///
/// Provide an implementation of [`is_symmetric`](IsSymmetric::is_symmetric)
/// that returns whether the digraph is symmetric OR implement `Arcs` and
/// `HasArc`.
///
/// ```
/// use {
///     graaf::{
///         Arcs,
///         HasArc,
///         IsSymmetric,
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
/// let digraph = AdjacencyList {
///     arcs: vec![BTreeSet::from([1]), BTreeSet::from([0])],
/// };
///
/// assert!(digraph.is_symmetric());
///
/// let digraph = AdjacencyList {
///     arcs: vec![BTreeSet::from([1]), BTreeSet::new()],
/// };
///
/// assert!(!digraph.is_symmetric());
/// ```
pub trait IsSymmetric {
    /// Check whether the digraph is symmetric.
    ///
    /// # Examples
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     IsSymmetric,
    ///     RemoveArc,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(2);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(1, 0);
    ///
    /// assert!(digraph.is_symmetric());
    ///
    /// digraph.remove_arc(1, 0);
    ///
    /// assert!(!digraph.is_symmetric());
    /// ```
    #[must_use]
    fn is_symmetric(&self) -> bool;
}

impl<D> IsSymmetric for D
where
    D: Arcs + HasArc,
{
    fn is_symmetric(&self) -> bool {
        self.arcs().all(|(u, v)| self.has_arc(v, u))
    }
}

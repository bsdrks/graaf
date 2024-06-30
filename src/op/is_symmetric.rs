//! Determine whether a digraph is symmetric.
//!
//! A digraph is symmetric if for every arc `(s, t)` there is an arc
//! `(t, s)`.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         IsSymmetric,
//!         RemoveArc,
//!     },
//! };
//!
//! let mut digraph = Digraph::empty(2);
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
//! let mut digraph = Digraph::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(!digraph.is_symmetric());
//! ```

use super::{
    HasArc,
    Arcs,
};

/// Determine whether a digraph is symmetric.
///
/// # How can I implement `IsSymmetric`?
///
/// Provide an implementation of `is_symmetric` that returns `true` if the
/// digraph is symmetric and `false` otherwise.
///
/// ```
/// use {
///     graaf::op::{
///         HasArc,
///         IsSymmetric,
///         Arcs,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl HasArc for Digraph {
///     fn has_arc(&self, s: usize, t: usize) -> bool {
///         self.arcs[s].contains(&t)
///     }
/// }
///
/// impl Arcs for Digraph {
///     fn arcs(&self) -> impl Iterator<Item = (usize, usize)> {
///         self.arcs
///             .iter()
///             .enumerate()
///             .flat_map(|(s, set)| set.iter().map(move |&t| (s, t)))
///     }
/// }
///
/// let digraph = Digraph {
///     arcs: vec![BTreeSet::from([1]), BTreeSet::from([0])],
/// };
///
/// assert!(digraph.is_symmetric());
///
/// let digraph = Digraph {
///     arcs: vec![BTreeSet::from([1]), BTreeSet::new()],
/// };
///
/// assert!(!digraph.is_symmetric());
/// ```
///
/// # Examples
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     gen::Empty,
///     op::{
///         AddArc,
///         IsSymmetric,
///         RemoveArc,
///     },
/// };
///
/// let mut digraph = Digraph::empty(2);
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
pub trait IsSymmetric {
    /// Returns whether the digraph is symmetric.
    fn is_symmetric(&self) -> bool;
}

impl<T> IsSymmetric for T
where
    T: HasArc + Arcs,
{
    fn is_symmetric(&self) -> bool {
        self.arcs().all(|(s, t)| self.has_arc(t, s))
    }
}

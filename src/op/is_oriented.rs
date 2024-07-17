//! Determine whether a digraph is oriented.
//!
//! An oriented graph is a digraph with no cycle of length 2.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Circuit,
//!     op::IsOriented,
//! };
//!
//! assert!(!Digraph::circuit(2).is_oriented());
//! assert!(Digraph::circuit(3).is_oriented());
//! ```

use super::{
    Arcs,
    HasArc,
};

/// Determine whether a digraph is oriented.
///
/// # How can I implement `IsOriented`?
///
/// Provide an implementation of `is_oriented` that returns whether the digraph
/// is oriented OR implement `Arcs` and `HasArc`.
///
/// ```
/// use {
///     graaf::{
///         gen::Circuit,
///         op::{
///             Arcs,
///             HasArc,
///             IsOriented,
///         },
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     pub arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Arcs for Digraph {
///     fn arcs(&self) -> impl Iterator<Item = (usize, usize)> {
///         (0..self.arcs.len()).flat_map(move |u| self.arcs[u].iter().map(move |&v| (u, v)))
///     }
/// }
///
/// impl HasArc for Digraph {
///     fn has_arc(&self, u: usize, v: usize) -> bool {
///         self.arcs[u].contains(&v)
///     }
/// }
///
/// let digraph = Digraph {
///     arcs: vec![BTreeSet::from([1]), BTreeSet::from([0])],
/// };
///
/// assert!(!digraph.is_oriented());
///
/// let digraph = Digraph {
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
///     adjacency_list::Digraph,
///     gen::Circuit,
///     op::IsOriented,
/// };
///
/// assert!(!Digraph::circuit(2).is_oriented());
/// assert!(Digraph::circuit(3).is_oriented());
/// ```
pub trait IsOriented {
    /// Returns whether the digraph is oriented.
    #[must_use]
    fn is_oriented(&self) -> bool;
}

impl<T> IsOriented for T
where
    T: Arcs + HasArc,
{
    fn is_oriented(&self) -> bool {
        self.arcs().all(|(u, v)| !self.has_arc(v, u))
    }
}

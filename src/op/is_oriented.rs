//! Determine whether a digraph is oriented.
//!
//! An oriented graph is a digraph with no cycle of length 2.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Cycle,
//!     op::IsOriented,
//! };
//!
//! assert!(!Digraph::cycle(2).is_oriented());
//! assert!(Digraph::cycle(3).is_oriented());
//! ```

use super::{
    Arcs,
    HasArc,
};

/// Determine whether a digraph is oriented.
///
/// # How can I implement `IsOriented`?
///
/// Provide an implementation of `is_oriented` that returns `true` if the
/// digraph is oriented and `false` otherwise OR implement `HasArc` and
/// `Arcs`.
///
/// ```
/// use {
///     graaf::{
///         gen::Cycle,
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
///         (0..self.arcs.len()).flat_map(move |s| self.arcs[s].iter().map(move |&t| (s, t)))
///     }
/// }
///
/// impl HasArc for Digraph {
///     fn has_arc(&self, s: usize, t: usize) -> bool {
///         self.arcs[s].contains(&t)
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
///     gen::Cycle,
///     op::IsOriented,
/// };
///
/// assert!(!Digraph::cycle(2).is_oriented());
/// assert!(Digraph::cycle(3).is_oriented());
/// ```
pub trait IsOriented {
    /// Returns whether the digraph is oriented.
    fn is_oriented(&self) -> bool;
}

impl<T> IsOriented for T
where
    T: Arcs + HasArc,
{
    fn is_oriented(&self) -> bool {
        self.arcs().all(|(s, t)| !self.has_arc(t, s))
    }
}

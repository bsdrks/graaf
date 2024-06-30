//! Check whether a sequence of vertices is a walk in a digraph.
//!
//! A sequence of vertices is a walk in a digraph if each pair of consecutive
//! vertices in the sequence is an arc in the digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Cycle,
//!     op::IsWalk,
//! };
//!
//! let digraph = Digraph::cycle(2);
//!
//! assert!(digraph.is_walk(&[0, 1]));
//! assert!(digraph.is_walk(&[1, 0]));
//!
//! assert!(!digraph.is_walk(&[0]));
//! assert!(!digraph.is_walk(&[1]));
//! assert!(!digraph.is_walk(&[2]));
//! assert!(!digraph.is_walk(&[0, 0]));
//! assert!(!digraph.is_walk(&[1, 1]));
//! assert!(!digraph.is_walk(&[0, 2]));
//! ```

use crate::op::HasArc;

/// Check whether a sequence of vertices is a walk in a digraph.
///
/// # How do I implement `IsWalk`?
///
/// Provide an implementation of `is_walk` that returns `true` if each pair of
/// consecutive vertices in the sequence is an arc in the digraph and `false`
/// otherwise.
///
/// ```
/// use {
///     graaf::{
///         gen::Cycle,
///         op::IsWalk,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     pub arcs: BTreeSet<(usize, usize)>,
/// }
///
/// impl IsWalk for Digraph {
///     fn is_walk(&self, walk: &[usize]) -> bool {
///         let mut arcs = walk.iter().zip(walk.iter().skip(1));
///
///         arcs.clone().count() > 0 && arcs.all(|(s, t)| self.arcs.contains(&(*s, *t)))
///     }
/// }
///
/// let digraph = Digraph {
///     arcs: BTreeSet::from([(0, 1), (1, 0)]),
/// };
///
/// assert!(digraph.is_walk(&[0, 1]));
/// assert!(digraph.is_walk(&[1, 0]));
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     gen::Cycle,
///     op::IsWalk,
/// };
///
/// let digraph = Digraph::cycle(2);
///
/// assert!(digraph.is_walk(&[0, 1]));
/// assert!(digraph.is_walk(&[1, 0]));
///
/// assert!(!digraph.is_walk(&[0]));
/// assert!(!digraph.is_walk(&[1]));
/// assert!(!digraph.is_walk(&[2]));
/// assert!(!digraph.is_walk(&[0, 0]));
/// assert!(!digraph.is_walk(&[1, 1]));
/// assert!(!digraph.is_walk(&[0, 2]));
/// ```
pub trait IsWalk {
    /// Returns whether the sequence of vertices is a walk in the digraph.
    fn is_walk(&self, walk: &[usize]) -> bool;
}

impl<T> IsWalk for T
where
    T: HasArc,
{
    fn is_walk(&self, walk: &[usize]) -> bool {
        let mut arcs = walk.iter().zip(walk.iter().skip(1));

        arcs.clone().count() > 0 && arcs.all(|(s, t)| self.has_arc(*s, *t))
    }
}

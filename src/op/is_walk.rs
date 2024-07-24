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
//!     gen::Circuit,
//!     op::IsWalk,
//! };
//!
//! let digraph = Digraph::circuit(2);
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

use super::HasArc;

/// Check whether a sequence of vertices is a walk in a digraph.
///
/// # How do I implement `IsWalk`?
///
/// Provide an implementation of `is_walk` that returns whether the sequence is
/// a walk in the digraph OR implement `HasArc`.
///
/// ```
/// use {
///     graaf::{
///         gen::Circuit,
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
///         arcs.clone().count() > 0 && arcs.all(|(&u, &v)| self.arcs.contains(&(u, v)))
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
///     gen::Circuit,
///     op::IsWalk,
/// };
///
/// let digraph = Digraph::circuit(2);
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
    ///
    /// # Arguments
    ///
    /// * `walk`: A sequence of vertices.
    #[must_use]
    fn is_walk(&self, walk: &[usize]) -> bool;
}

impl<D> IsWalk for D
where
    D: HasArc,
{
    fn is_walk(&self, walk: &[usize]) -> bool {
        let mut arcs = walk.iter().zip(walk.iter().skip(1));

        arcs.clone().count() > 0 && arcs.all(|(&u, &v)| self.has_arc(u, v))
    }
}

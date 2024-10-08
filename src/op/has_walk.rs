//! Check whether a digraph contains a walk.
//!
//! A sequence of vertices is a walk in a digraph if each pair of consecutive
//! vertices in the sequence is an arc in the digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Circuit,
//!     HasWalk,
//! };
//!
//! let digraph = AdjacencyList::circuit(2);
//!
//! assert!(digraph.has_walk(&[0, 1]));
//! assert!(digraph.has_walk(&[1, 0]));
//!
//! assert!(!digraph.has_walk(&[0]));
//! assert!(!digraph.has_walk(&[1]));
//! assert!(!digraph.has_walk(&[2]));
//! assert!(!digraph.has_walk(&[0, 0]));
//! assert!(!digraph.has_walk(&[1, 1]));
//! assert!(!digraph.has_walk(&[0, 2]));
//! ```

use crate::HasArc;

/// Check whether a digraph contains a walk.
///
/// # Implementing [`HasWalk`] for a custom type
///
/// Provide an implementation of [`has_walk`](HasWalk::has_walk) that returns
/// whether the sequence is a walk in the digraph OR implement [`HasArc`].
///
/// ```
/// use {
///     graaf::{
///         Circuit,
///         HasWalk,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     pub arcs: BTreeSet<(usize, usize)>,
/// }
///
/// impl HasWalk for AdjacencyList {
///     fn has_walk(&self, walk: &[usize]) -> bool {
///         let mut arcs = walk.iter().zip(walk.iter().skip(1));
///
///         arcs.clone().count() > 0
///             && arcs.all(|(&u, &v)| self.arcs.contains(&(u, v)))
///     }
/// }
///
/// let digraph = AdjacencyList {
///     arcs: BTreeSet::from([(0, 1), (1, 0)]),
/// };
///
/// assert!(digraph.has_walk(&[0, 1]));
/// assert!(digraph.has_walk(&[1, 0]));
/// ```
pub trait HasWalk {
    /// Check whether the sequence is a walk in the digraph.
    ///
    /// # Arguments
    ///
    /// * `walk`: A sequence of vertices.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Circuit,
    ///     HasWalk,
    /// };
    ///
    /// let digraph = AdjacencyList::circuit(2);
    ///
    /// assert!(digraph.has_walk(&[0, 1]));
    /// assert!(digraph.has_walk(&[1, 0]));
    ///
    /// assert!(!digraph.has_walk(&[0]));
    /// assert!(!digraph.has_walk(&[1]));
    /// assert!(!digraph.has_walk(&[2]));
    /// assert!(!digraph.has_walk(&[0, 0]));
    /// assert!(!digraph.has_walk(&[1, 1]));
    /// assert!(!digraph.has_walk(&[0, 2]));
    /// ```
    #[must_use]
    fn has_walk(&self, walk: &[usize]) -> bool;
}

impl<D> HasWalk for D
where
    D: HasArc,
{
    fn has_walk(&self, walk: &[usize]) -> bool {
        walk.len() > 1
            && walk
                .iter()
                .zip(walk.iter().skip(1))
                .all(|(&u, &v)| self.has_arc(u, v))
    }
}

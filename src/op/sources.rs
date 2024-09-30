//! Iterate over a digraph's sources.
//!
//! A source is a vertex without in-neighbors.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     Sources,
//! };
//!
//! let mut digraph = AdjacencyList::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//!
//! assert!(digraph.sources().eq([0, 3]));
//! ```

use crate::{
    Indegree,
    Vertices,
};

/// Iterate over a digraph's sources.
///
/// # Implementing [`Sources`] for a custom type
///
/// Provide an implementation of [`sources`](Sources::sources) that returns an
/// iterator over a digraph's sources OR implement [`Indegree`] and
/// [`Vertices`].
///
/// ```
/// use {
///     graaf::{
///         Indegree,
///         Sources,
///         Vertices,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Indegree for AdjacencyList {
///     fn indegree(&self, u: usize) -> usize {
///         self.arcs.iter().filter(|set| set.contains(&u)).count()
///     }
/// }
///
/// impl Vertices for AdjacencyList {
///     fn vertices(&self) -> impl Iterator<Item = usize> {
///         0..self.arcs.len()
///     }
/// }
///
/// let mut digraph = AdjacencyList {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([2]),
///         BTreeSet::new(),
///         BTreeSet::new(),
///     ],
/// };
///
/// assert!(digraph.sources().eq([0, 3]));
/// ```
pub trait Sources {
    /// Return an iterator over the sources in the digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     Sources,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(4);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 2);
    ///
    /// assert!(digraph.sources().eq([0, 3]));
    /// ```
    #[must_use]
    fn sources(&self) -> impl Iterator<Item = usize>;
}

impl<D> Sources for D
where
    D: Indegree + Vertices,
{
    fn sources(&self) -> impl Iterator<Item = usize> {
        self.vertices().filter(move |&u| self.is_source(u))
    }
}

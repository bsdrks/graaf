//! Return a vertex's degree.
//!
//! The degree of a vertex is the sum of its indegree and outdegree.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Degree,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert_eq!(digraph.degree(0), 3);
//! assert_eq!(digraph.degree(1), 2);
//! assert_eq!(digraph.degree(2), 3);
//! ```
#![doc(alias = "valence")]
#![doc(alias = "valency")]

use crate::{
    Indegree,
    Outdegree,
};

/// Return a vertex's degree.
///
/// # Implementing `Degree`
///
/// Provide an implementation of `Degree` that returns the degree of the vertex
/// OR implement `Indegree` and `Outdegree`.
///
/// ```
/// use {
///     graaf::{
///         Degree,
///         Indegree,
///         Outdegree,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Indegree for AdjacencyList {
///     fn indegree(&self, v: usize) -> usize {
///         self.arcs.iter().filter(|set| set.contains(&v)).count()
///     }
/// }
///
/// impl Outdegree for AdjacencyList {
///     fn outdegree(&self, u: usize) -> usize {
///         self.arcs[u].len()
///     }
/// }
///
/// let digraph = AdjacencyList {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([2]),
///         BTreeSet::from([0]),
///         BTreeSet::new(),
///     ],
/// };
///
/// assert_eq!(digraph.degree(0), 3);
/// assert_eq!(digraph.degree(1), 2);
/// assert_eq!(digraph.degree(2), 3);
/// assert_eq!(digraph.degree(3), 0);
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     AddArc,
///     AdjacencyList,
///     Degree,
///     Empty,
/// };
///
/// let mut digraph = AdjacencyList::empty(3);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(0, 2);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(2, 0);
///
/// assert_eq!(digraph.degree(0), 3);
/// assert_eq!(digraph.degree(1), 2);
/// assert_eq!(digraph.degree(2), 3);
/// ```
#[doc(alias = "Valence")]
#[doc(alias = "Valency")]
pub trait Degree {
    /// Returns a vertex's degree.
    ///
    /// # Arguments
    ///
    /// * `u`: The vertex.
    #[doc(alias = "valence")]
    #[doc(alias = "valency")]
    #[must_use]
    fn degree(&self, u: usize) -> usize;
}

impl<D> Degree for D
where
    D: Indegree + Outdegree,
{
    fn degree(&self, u: usize) -> usize {
        self.indegree(u) + self.outdegree(u)
    }
}

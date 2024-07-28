//! Return the degree of a vertex.
//!
//! The degree of a vertex is the sum of its indegree and outdegree.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         Degree,
//!     },
//! };
//!
//! // 0 -> {1, 2}
//! // 1 -> {2}
//! // 2 -> {0}
//!
//! let mut digraph = Digraph::empty(3);
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

use super::{
    Indegree,
    Outdegree,
};

/// Return the degree of a vertex.
///
/// # How can I implement `Degree`?
///
/// Provide an implementation of `Degree` that returns the degree of the vertex
/// OR implement `Indegree` and `Outdegree`.
///
/// ```
/// use {
///     graaf::op::{
///         Degree,
///         Indegree,
///         Outdegree,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Indegree for Digraph {
///     fn indegree(&self, v: usize) -> usize {
///         self.arcs.iter().filter(|set| set.contains(&v)).count()
///     }
/// }
///
/// impl Outdegree for Digraph {
///     fn outdegree(&self, u: usize) -> usize {
///         self.arcs[u].len()
///     }
/// }
///
/// // 0 -> {1, 2}
/// // 1 -> {2}
/// // 2 -> {0}
/// // 3 -> {}
///
/// let digraph = Digraph {
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
///     adjacency_list::Digraph,
///     gen::Empty,
///     op::{
///         AddArc,
///         Degree,
///     },
/// };
///
/// // 0 -> {1, 2}
/// // 1 -> {2}
/// // 2 -> {0}
///
/// let mut digraph = Digraph::empty(3);
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
    /// Returns the degree of a vertex in the digraph.
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

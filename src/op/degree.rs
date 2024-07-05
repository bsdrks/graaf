#![doc(alias = "valency")]
//! Get the degree of a vertex.
//!
//! For digraphs, the degree is the sum of the indegree and outdegree.
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

use super::{
    Indegree,
    Outdegree,
};

/// Get the degree of a vertex.
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
pub trait Degree {
    /// Returns the degree of a vertex in the digraph.
    ///
    /// # Arguments
    ///
    /// * `u`: The vertex.
    fn degree(&self, u: usize) -> usize;
}

impl<T> Degree for T
where
    T: Indegree + Outdegree,
{
    fn degree(&self, u: usize) -> usize {
        self.indegree(u) + self.outdegree(u)
    }
}

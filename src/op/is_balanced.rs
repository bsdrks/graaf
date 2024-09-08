//! Check whether a digraph is balanced.
//!
//! A digraph is balanced if the indegree of each vertex equals its
//! outdegree.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IsBalanced,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(!digraph.is_balanced());
//!
//! digraph.add_arc(2, 1);
//!
//! assert!(digraph.is_balanced());
//! ```
#![doc(alias = "isograph")]
#![doc(alias = "pseudosymmetric")]

use crate::{
    Indegree,
    Outdegree,
    Vertices,
};

/// Check whether a digraph is balanced.
///
/// # Implementing `IsBalanced`
///
/// Provide an implementation of `is_balanced` that returns whether the digraph
/// is balanced OR implement `Indegree`, `Outdegree`, and `Vertices`.
///
/// ```
/// use {
///     graaf::{
///         Indegree,
///         IsBalanced,
///         Outdegree,
///         Vertices,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     pub arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Indegree for AdjacencyList {
///     fn indegree(&self, v: usize) -> usize {
///         self.arcs.iter().filter(|set| set.contains(&v)).count()
///     }
/// }
///
/// impl Vertices for AdjacencyList {
///     fn vertices(&self) -> impl Iterator<Item = usize> {
///         0..self.arcs.len()
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
///         BTreeSet::from([0, 2]),
///         BTreeSet::from([0, 1]),
///     ],
/// };
///
/// assert!(digraph.is_balanced());
///
/// let digraph = AdjacencyList {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([0, 2]),
///         BTreeSet::from([0]),
///     ],
/// };
///
/// assert!(!digraph.is_balanced());
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     AddArc,
///     AdjacencyList,
///     Empty,
///     IsBalanced,
/// };
///
/// let mut digraph = AdjacencyList::empty(3);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(0, 2);
/// digraph.add_arc(1, 0);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(2, 0);
///
/// assert!(!digraph.is_balanced());
///
/// digraph.add_arc(2, 1);
///
/// assert!(digraph.is_balanced());
/// ```
#[doc(alias = "Isograph")]
#[doc(alias = "Pseudosymmetric")]
pub trait IsBalanced {
    /// Checks whether the digraph is balanced.
    #[doc(alias = "isograph")]
    #[doc(alias = "pseudosymmetric")]
    #[must_use]
    fn is_balanced(&self) -> bool;
}

impl<D> IsBalanced for D
where
    D: Indegree + Outdegree + Vertices,
{
    fn is_balanced(&self) -> bool {
        self.vertices()
            .all(|u| self.indegree(u) == self.outdegree(u))
    }
}

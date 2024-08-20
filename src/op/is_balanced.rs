//! Check whether a digraph is balanced.
//!
//! A digraph is balanced if the indegree of each vertex equals its
//! outdegree.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         IsBalanced,
//!     },
//! };
//!
//! // 0 -> {1, 2}
//! // 1 -> {0, 2}
//! // 2 -> {0}
//!
//! let mut digraph = Digraph::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(!digraph.is_balanced());
//!
//! // 0 -> {1, 2}
//! // 1 -> {0, 2}
//! // 2 -> {0, 1}
//!
//! digraph.add_arc(2, 1);
//!
//! assert!(digraph.is_balanced());
//! ```
#![doc(alias = "isograph")]
#![doc(alias = "pseudosymmetric")]

use super::{
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
///     graaf::op::{
///         Indegree,
///         IsBalanced,
///         Outdegree,
///         Vertices,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     pub arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Indegree for Digraph {
///     fn indegree(&self, v: usize) -> usize {
///         self.arcs.iter().filter(|set| set.contains(&v)).count()
///     }
/// }
///
/// impl Vertices for Digraph {
///     fn vertices(&self) -> impl Iterator<Item = usize> {
///         0..self.arcs.len()
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
/// // 1 -> {0, 2}
/// // 2 -> {0, 1}
///
/// let digraph = Digraph {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([0, 2]),
///         BTreeSet::from([0, 1]),
///     ],
/// };
///
/// assert!(digraph.is_balanced());
///
/// // 0 -> {1, 2}
/// // 1 -> {0, 2}
/// // 2 -> {0}
///
/// let digraph = Digraph {
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
///     adjacency_list::Digraph,
///     gen::Empty,
///     op::{
///         AddArc,
///         IsBalanced,
///     },
/// };
///
/// // 0 -> {1, 2}
/// // 1 -> {0, 2}
/// // 2 -> {0}
///
/// let mut digraph = Digraph::empty(3);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(0, 2);
/// digraph.add_arc(1, 0);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(2, 0);
///
/// assert!(!digraph.is_balanced());
///
/// // 0 -> {1, 2}
/// // 1 -> {0, 2}
/// // 2 -> {0, 1}
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

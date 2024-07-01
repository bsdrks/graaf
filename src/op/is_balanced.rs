#![doc(alias = "isograph")]
#![doc(alias = "pseudosymmetric")]
//! Determine whether a digraph is balanced.
//!
//! A digraph is balanced if the indegree of each vertex is equal to its
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
//! digraph.add_arc(2, 1);
//!
//! assert!(digraph.is_balanced());
//! ```

use super::{
    Indegree,
    Outdegree,
    Vertices,
};

/// Determine whether a digraph is balanced.
///
/// # How can I implement `IsBalanced`?
///
/// Provide an implementation of `is_balanced` that returns `true` if the
/// digraph is balanced and `false` otherwise OR implement `Indegree`,
/// `Vertices`, and `Outdegree`.
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
///     fn indegree(&self, s: usize) -> usize {
///         self.arcs.iter().filter(|set| set.contains(&s)).count()
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
///     fn outdegree(&self, s: usize) -> usize {
///         self.arcs[s].len()
///     }
/// }
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
/// digraph.add_arc(2, 1);
///
/// assert!(digraph.is_balanced());
/// ```
pub trait IsBalanced {
    /// Returns whether the digraph is balanced.
    fn is_balanced(&self) -> bool;
}

impl<T> IsBalanced for T
where
    T: Indegree + Outdegree + Vertices,
{
    fn is_balanced(&self) -> bool {
        self.vertices()
            .all(|s| self.indegree(s) == self.outdegree(s))
    }
}

//! Determine whether a vertex in a digraph is a pendant
//! vertex.
//!
//! A pendant vertex has a degree of one.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         IsPendant,
//!     },
//! };
//!
//! let mut digraph = Digraph::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//! digraph.add_arc(3, 0);
//!
//! assert!(!digraph.is_pendant(0));
//! assert!(!digraph.is_pendant(1));
//! assert!(digraph.is_pendant(2));
//! assert!(digraph.is_pendant(3));
//! ```

use super::Degree;

/// Determine whether a vertex is a pendant vertex.
///
/// # How can I implement `IsPendant`?
///
/// Provide an implementation of `is_pendant` that returns whether the vertex
/// is pendant OR implement `Degree`.
///
/// ```
/// use {
///     graaf::op::{
///         Indegree,
///         IsPendant,
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
///     fn indegree(&self, u: usize) -> usize {
///         self.arcs.iter().filter(|set| set.contains(&u)).count()
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
///         BTreeSet::from([0]),
///         BTreeSet::new(),
///         BTreeSet::from([0]),
///     ],
/// };
///
/// assert!(!digraph.is_pendant(0));
/// assert!(!digraph.is_pendant(1));
/// assert!(digraph.is_pendant(2));
/// assert!(digraph.is_pendant(3));
/// ```
pub trait IsPendant {
    /// Returns whether a vertex is a pendant vertex.
    ///
    /// # Arguments
    ///
    /// * `u`: The vertex.
    #[must_use]
    fn is_pendant(&self, u: usize) -> bool;
}

impl<D> IsPendant for D
where
    D: Degree,
{
    fn is_pendant(&self, u: usize) -> bool {
        self.degree(u) == 1
    }
}

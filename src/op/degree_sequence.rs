//! Get the degree sequence of a digraph.
//!
//! For digraphs, the degree sequence is the list of its indegree and outdegree
//! pairs.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         DegreeSequence,
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
//! assert!(digraph
//!     .degree_sequence()
//!     .iter()
//!     .eq(&[(1, 2), (1, 1), (2, 1)]));
//! ```

use super::{
    Indegree,
    Outdegree,
    Vertices,
};

/// Get the outdegree sequence of a digraph.
///
/// # How can I implement `DegreeSequence`?
///
/// Provide an implementation of `DegreeSequence` that returns the degree
/// sequence of the digraph OR implement `Indegree` and `Outdegree`.
///
/// ```
/// use {
///     graaf::op::{
///         DegreeSequence,
///         Indegree,
///         Outdegree,
///         Vertices,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Indegree for Digraph {
///     fn indegree(&self, s: usize) -> usize {
///         self.arcs.iter().filter(|set| set.contains(&s)).count()
///     }
/// }
///
/// impl Outdegree for Digraph {
///     fn outdegree(&self, s: usize) -> usize {
///         self.arcs[s].len()
///     }
/// }
///
/// impl Vertices for Digraph {
///     fn vertices(&self) -> impl Iterator<Item = usize> {
///         0..self.arcs.len()
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
/// assert!(digraph
///     .degree_sequence()
///     .iter()
///     .eq(&[(1, 2), (1, 1), (2, 1), (0, 0)]));
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
///         DegreeSequence,
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
/// assert!(digraph
///     .degree_sequence()
///     .iter()
///     .eq(&[(1, 2), (1, 1), (2, 1)]));
/// ```
pub trait DegreeSequence {
    /// Returns the degree sequence of a digraph.
    fn degree_sequence(&self) -> Vec<(usize, usize)>;
}

impl<T> DegreeSequence for T
where
    T: Indegree + Outdegree + Vertices,
{
    fn degree_sequence(&self) -> Vec<(usize, usize)> {
        self.vertices()
            .map(|u| (self.indegree(u), self.outdegree(u)))
            .collect()
    }
}

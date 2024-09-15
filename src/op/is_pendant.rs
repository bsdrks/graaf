//! Check whether a vertex is pendant.
//!
//! A pendant vertex has a degree of one.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IsPendant,
//! };
//!
//! let mut digraph = AdjacencyList::empty(4);
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

use crate::Degree;

/// Check whether a vertex is Pendant.
///
/// # Implementing [`IsPendant`] for a custom type
///
/// Provide an implementation of [`is_pendant`](IsPendant::is_pendant) that
/// returns whether the vertex is pendant OR implement [`Degree`].
///
/// ```
/// use {
///     graaf::{
///         Indegree,
///         IsPendant,
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
///     fn indegree(&self, u: usize) -> usize {
///         self.arcs.iter().filter(|set| set.contains(&u)).count()
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
    /// Check whether a vertex is a pendant vertex.
    ///
    /// # Arguments
    ///
    /// * `u`: The vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     IsPendant,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(4);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 0);
    /// digraph.add_arc(3, 0);
    ///
    /// assert!(!digraph.is_pendant(0));
    /// assert!(!digraph.is_pendant(1));
    /// assert!(digraph.is_pendant(2));
    /// assert!(digraph.is_pendant(3));
    /// ```
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

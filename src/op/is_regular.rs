//! Check whether a digraph is regular.
//!
//! A digraph is regular if all vertices have the same indegree and
//! outdegree.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Circuit,
//!     IsRegular,
//!     RemoveArc,
//! };
//!
//! let mut digraph = AdjacencyList::circuit(7);
//!
//! assert!(digraph.is_regular());
//!
//! digraph.remove_arc(6, 0);
//!
//! assert!(!digraph.is_regular());
//! ```

/// Check whether a digraph is regular.
///
/// # Implementing [`IsRegular`] for a custom type
///
/// Provide an implementation of [`is_regular`](IsRegular::is_regular) that
/// returns whether the digraph is regular.
///
/// ```
/// use {
///     graaf::IsRegular,
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl IsRegular for AdjacencyList {
///     fn is_regular(&self) -> bool {
///         (0..self.arcs.len()).all(|u| {
///             self.arcs[u].len()
///                 == (0..self.arcs.len())
///                     .filter(|v| self.arcs[*v].contains(&u))
///                     .count()
///         })
///     }
/// }
///
/// assert!(AdjacencyList {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([2, 0]),
///         BTreeSet::from([0, 1]),
///     ]
/// }
/// .is_regular());
///
/// assert!(!AdjacencyList {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([0, 2]),
///         BTreeSet::from([0]),
///     ]
/// }
/// .is_regular());
/// ```
///
/// Implementations can be built with the [`Indegree`](crate::Indegree),
/// [`Outdegree`](crate::Outdegree), and [`Vertices`](crate::Vertices) traits.
///
/// ```
/// use {
///     graaf::{
///         Indegree,
///         IsRegular,
///         Outdegree,
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
/// impl IsRegular for AdjacencyList {
///     fn is_regular(&self) -> bool {
///         self.vertices()
///             .all(|u| self.indegree(u) == self.outdegree(u))
///     }
/// }
///
/// assert!(AdjacencyList {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([2, 0]),
///         BTreeSet::from([0, 1]),
///     ]
/// }
/// .is_regular());
///
/// assert!(!AdjacencyList {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([0, 2]),
///         BTreeSet::from([0]),
///     ]
/// }
/// .is_regular());
/// ```
pub trait IsRegular {
    /// Check whether the digraph is regular.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Circuit,
    ///     IsRegular,
    ///     RemoveArc,
    /// };
    ///
    /// let mut digraph = AdjacencyList::circuit(7);
    ///
    /// assert!(digraph.is_regular());
    ///
    /// digraph.remove_arc(6, 0);
    ///
    /// assert!(!digraph.is_regular());
    /// ```
    #[must_use]
    fn is_regular(&self) -> bool;
}

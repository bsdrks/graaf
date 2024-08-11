//! Return a vertex's indegree.
//!
//! The indegree is the number of arcs incident into a vertex.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         Indegree,
//!     },
//! };
//!
//! // 0 -> {1, 2}
//! // 1 -> {2}
//! // 2 -> {}
//!
//! let mut digraph = Digraph::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//!
//! assert_eq!(digraph.indegree(0), 0);
//! assert_eq!(digraph.indegree(1), 1);
//! assert_eq!(digraph.indegree(2), 2);
//! ```
#![doc(alias = "in_degree")]

/// Return a vertex's indegree.
///
/// # How can I implement `Indegree`?
///
/// Provide an implementation of `indegree` that returns the indegree of the
/// vertex.
///
/// ```
/// use {
///     graaf::op::Indegree,
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
/// // 0 -> {1, 2}
/// // 1 -> {2}
/// // 2 -> {}
///
/// let digraph = Digraph {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([2]),
///         BTreeSet::new(),
///     ],
/// };
///
/// assert_eq!(digraph.indegree(0), 0);
/// assert_eq!(digraph.indegree(1), 1);
/// assert_eq!(digraph.indegree(2), 2);
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
///         Indegree,
///     },
/// };
///
/// // 0 -> {1, 2}
/// // 1 -> {2}
/// // 2 -> {}
///
/// let mut digraph = Digraph::empty(3);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(0, 2);
/// digraph.add_arc(1, 2);
///
/// assert_eq!(digraph.indegree(0), 0);
/// assert_eq!(digraph.indegree(1), 1);
/// assert_eq!(digraph.indegree(2), 2);
/// ```
#[doc(alias = "InDegree")]
pub trait Indegree {
    /// Returns the indegree of a vertex in the digraph.
    ///
    /// # Arguments
    ///
    /// * `v`: The vertex.
    #[doc(alias = "in_degree")]
    #[must_use]
    fn indegree(&self, v: usize) -> usize;

    /// Returns whether a vertex is a source of the digraph.
    ///
    /// A source is a vertex with an indegree of 0.
    ///
    /// # Arguments
    ///
    /// * `v`: The vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Empty,
    ///     op::{
    ///         AddArc,
    ///         Indegree,
    ///     },
    /// };
    ///
    /// // 0 -> {1, 2}
    /// // 1 -> {2}
    /// // 2 -> {}
    ///
    /// let mut digraph = Digraph::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 2);
    ///
    /// assert!(digraph.is_source(0));
    /// assert!(!digraph.is_source(1));
    /// assert!(!digraph.is_source(2));
    /// ```
    #[must_use]
    fn is_source(&self, v: usize) -> bool {
        self.indegree(v) == 0
    }
}

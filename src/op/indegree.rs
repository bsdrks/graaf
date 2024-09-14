//! Return a vertex's indegree.
//!
//! The indegree is the number of arcs incident into a vertex.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     Indegree,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
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

use crate::Vertices;

/// Return a vertex's indegree.
///
/// # Implementing [`Indegree`] for a custom type
///
/// Provide an implementation of [`indegree`](Indegree::indegree) that returns
/// the indegree of the vertex.
///
/// ```
/// use {
///     graaf::Indegree,
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
/// let digraph = AdjacencyList {
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
#[doc(alias = "InDegree")]
pub trait Indegree {
    /// Return the indegree of a vertex in the digraph.
    ///
    /// # Arguments
    ///
    /// * `v`: The vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     Indegree,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 2);
    ///
    /// assert_eq!(digraph.indegree(0), 0);
    /// assert_eq!(digraph.indegree(1), 1);
    /// assert_eq!(digraph.indegree(2), 2);
    /// ```
    #[doc(alias = "in_degree")]
    #[must_use]
    fn indegree(&self, v: usize) -> usize;

    /// Check whether a vertex is a source of the digraph.
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
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     Indegree,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
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

    /// Return the maximum indegree of the digraph.
    ///
    /// # Examples
    ///
    /// The maximum indegree of this digraph is `3`. The vertex with the
    /// maximum indegree is red.
    ///
    /// ![Maximum indegree](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/max_indegree-0.88.5.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     Indegree,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(4);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(0, 3);
    /// digraph.add_arc(1, 0);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(2, 0);
    /// digraph.add_arc(3, 0);
    ///
    /// assert_eq!(digraph.max_indegree(), 3);
    /// ```
    #[must_use]
    fn max_indegree(&self) -> usize
    where
        Self: Vertices,
    {
        self.vertices().map(|u| self.indegree(u)).max().unwrap_or(0)
    }

    /// Return the minimum indegree of the digraph.
    ///
    /// # Examples
    ///
    /// The minimum indegree of this digraph is `1`. The vertices with the
    /// minimum indegree are red.
    ///
    /// ![Minimum indegree](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/min_indegree-0.88.5.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     Indegree,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(4);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(0, 3);
    /// digraph.add_arc(1, 0);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(2, 0);
    /// digraph.add_arc(3, 0);
    ///
    /// assert_eq!(digraph.min_indegree(), 1);
    /// ```
    #[must_use]
    fn min_indegree(&self) -> usize
    where
        Self: Vertices,
    {
        self.vertices().map(|u| self.indegree(u)).min().unwrap_or(0)
    }
}

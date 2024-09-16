//! Return a vertex's degree.
//!
//! A vertex's degree is the sum of its indegree and outdegree.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Degree,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
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
#![doc(alias = "semidegree")]
#![doc(alias = "valence")]
#![doc(alias = "valency")]

use crate::{
    Indegree,
    Outdegree,
    Vertices,
};

/// Return a vertex's degree.
///
/// # Implementing [`Degree`] for a custom type
///
/// Provide an implementation of [`degree`](Degree::degree) that returns a
/// vertex's degree OR implement [`Indegree`] and [`Outdegree`].
///
/// ```
/// use {
///     graaf::{
///         Degree,
///         Indegree,
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
///     fn indegree(&self, v: usize) -> usize {
///         self.arcs.iter().filter(|set| set.contains(&v)).count()
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
#[doc(alias = "Semidegree")]
#[doc(alias = "Valence")]
#[doc(alias = "Valency")]
pub trait Degree {
    /// Return a vertex's degree.
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
    ///     Degree,
    ///     Empty,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
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
    #[doc(alias = "semidegree")]
    #[doc(alias = "valence")]
    #[doc(alias = "valency")]
    #[must_use]
    fn degree(&self, u: usize) -> usize;

    /// Return a digraph's maximum degree.
    ///
    /// # Examples
    ///
    /// The maximum degree of this digraph is `6`. The vertex with the maximum
    /// degree is red.
    ///
    /// ![Maximum degree](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/max_degree-0.88.4.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Degree,
    ///     Empty,
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
    /// assert_eq!(digraph.max_degree(), 6);
    /// ```
    #[must_use]
    fn max_degree(&self) -> usize
    where
        Self: Vertices,
    {
        self.vertices().map(|u| self.degree(u)).max().unwrap_or(0)
    }

    /// Return a digraph's minimum degree.
    ///
    /// # Examples
    ///
    /// The minimum degree of this digraph is `2`. The vertex with the minimum
    /// degree is red.
    ///
    /// ![Minimum degree](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/min_degree-0.88.4.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Degree,
    ///     Empty,
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
    /// assert_eq!(digraph.min_degree(), 2);
    /// ```
    #[must_use]
    fn min_degree(&self) -> usize
    where
        Self: Vertices,
    {
        self.vertices().map(|u| self.degree(u)).min().unwrap_or(0)
    }
}

impl<D> Degree for D
where
    D: Indegree + Outdegree,
{
    fn degree(&self, u: usize) -> usize {
        self.indegree(u) + self.outdegree(u)
    }
}

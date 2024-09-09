//! Return a vertex's outdegree.
//!
//! The outdegree is the number of arcs incident out of a vertex.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     Outdegree,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//!
//! assert_eq!(digraph.outdegree(0), 2);
//! assert_eq!(digraph.outdegree(1), 1);
//! assert_eq!(digraph.outdegree(2), 0);
//! ```
#![doc(alias = "out_degree")]

use crate::Vertices;

/// Return a vertex's outdegree.
///
/// # Implementing [`Outdegree`] for a custom type
///
/// Provide an implementation of [`outdegree`](Outdegree::outdegree) that
/// returns the outdegree of the vertex.
///
/// ```
/// use {
///     graaf::Outdegree,
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Outdegree for AdjacencyList {
///     fn outdegree(&self, u: usize) -> usize {
///         self.arcs.get(u).map_or(0, BTreeSet::len)
///     }
/// }
///
/// let digraph = AdjacencyList {
///     arcs: vec![
///         BTreeset::from([1, 2]),
///         BTreeset::from([0]),
///         BTreeset::from([1]),
///     ],
/// };
///
/// assert_eq!(digraph.outdegree(0), 2);
/// assert_eq!(digraph.outdegree(1), 1);
/// assert_eq!(digraph.outdegree(2), 1);
/// ```
pub trait Outdegree {
    /// Return the outdegree of a vertex in the digraph.
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
    ///     Outdegree,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 0);
    /// digraph.add_arc(2, 1);
    ///
    /// assert_eq!(digraph.outdegree(0), 2);
    /// assert_eq!(digraph.outdegree(1), 1);
    /// assert_eq!(digraph.outdegree(2), 1);
    /// ```
    #[doc(alias = "out_degree")]
    #[must_use]
    fn outdegree(&self, u: usize) -> usize;

    /// Check whether a vertex is a sink of the digraph.
    ///
    /// A sink is a vertex with no out-neighbors.
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
    ///     Outdegree,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 0);
    ///
    /// assert!(!digraph.is_sink(0));
    /// assert!(!digraph.is_sink(1));
    /// assert!(digraph.is_sink(2));
    /// ```
    #[must_use]
    fn is_sink(&self, u: usize) -> bool {
        self.outdegree(u) == 0
    }

    /// Return the maximum outdegree of the digraph.
    ///
    /// # Examples
    ///
    /// The maximum outdegree of the following digraph is `3`. The vertex with
    /// the maximum outdegree is red.
    ///
    /// ![Maximum outdegree](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/max_outdegree-0.88.5.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     Outdegree,
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
    /// assert_eq!(digraph.max_outdegree(), 3);
    /// ```
    #[must_use]
    fn max_outdegree(&self) -> usize
    where
        Self: Vertices,
    {
        self.vertices()
            .map(|u| self.outdegree(u))
            .max()
            .unwrap_or(0)
    }

    /// Return the minimum outdegree of the digraph.
    ///
    /// # Examples
    ///
    /// The minimum outdegree of the following digraph is `1`. The vertices
    /// with the minimum outdegree are red.
    ///
    /// ![Minimum outdegree](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/min_outdegree-0.88.5.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     Outdegree,
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
    /// assert_eq!(digraph.min_outdegree(), 1);
    /// ```
    #[must_use]
    fn min_outdegree(&self) -> usize
    where
        Self: Vertices,
    {
        self.vertices()
            .map(|u| self.outdegree(u))
            .min()
            .unwrap_or(0)
    }
}

//! Return a vertex's outdegree.
//!
//! The outdegree is the digraph's size incident out of a vertex.
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

/// Vertex outdegree
pub trait Outdegree {
    /// Return the vertex's outdegree.
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

    /// Check whether a vertex is a digraph's sink.
    ///
    /// A sink is a vertex without out-neighbors.
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

    /// Return the digraph's maximum outdegree.
    ///
    /// # Examples
    ///
    /// The maximum outdegree of this digraph is `3`. The vertex with the
    /// maximum outdegree is red.
    ///
    /// ![A digraph and its maximum outdegree](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/max_outdegree-0.88.5.svg?)
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

    /// Return the digraph's minimum outdegree.
    ///
    /// # Examples
    ///
    /// The minimum outdegree of this digraph is `1`. The vertices with the
    /// minimum outdegree are red.
    ///
    /// ![A digraph and its minimum outdegree](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/min_outdegree-0.88.5.svg?)
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

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::collections::BTreeSet,
    };

    #[test]
    fn is_sink() {
        struct AdjacencyList {
            arcs: Vec<BTreeSet<usize>>,
        }

        impl Outdegree for AdjacencyList {
            fn outdegree(&self, u: usize) -> usize {
                self.arcs.get(u).map_or(0, BTreeSet::len)
            }
        }

        let digraph = AdjacencyList {
            arcs: vec![
                BTreeSet::from([1, 2]),
                BTreeSet::new(),
                BTreeSet::new(),
                BTreeSet::from([0]),
            ],
        };

        assert!(!digraph.is_sink(0));
        assert!(digraph.is_sink(1));
        assert!(digraph.is_sink(2));
        assert!(!digraph.is_sink(3));
    }
}

//! Return a vertex's indegree.
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

/// Vertex indegree
#[doc(alias = "InDegree")]
pub trait Indegree {
    /// Return the vertex's indegree.
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

    /// Check whether a vertex is a digraph's source.
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

    /// Return a digraph's maximum indegree.
    ///
    /// # Examples
    ///
    /// The maximum indegree of this digraph is `3`. The vertex with the
    /// maximum indegree is red.
    ///
    /// ![A digraph and its maximum indegree](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/max_indegree-0.88.5.svg?)
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

    /// Return a digraph's minimum indegree.
    ///
    /// # Examples
    ///
    /// The minimum indegree of this digraph is `1`. The vertices with the
    /// minimum indegree are red.
    ///
    /// ![A digraph and its minimum indegree](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/min_indegree-0.88.5.svg?)
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

        impl Indegree for AdjacencyList {
            fn indegree(&self, v: usize) -> usize {
                self.arcs.iter().filter(|set| set.contains(&v)).count()
            }
        }

        let digraph = AdjacencyList {
            arcs: vec![
                BTreeSet::from([1, 2]),
                BTreeSet::new(),
                BTreeSet::new(),
                BTreeSet::from([1]),
            ],
        };

        assert!(digraph.is_source(0));
        assert!(!digraph.is_source(1));
        assert!(!digraph.is_source(2));
        assert!(digraph.is_source(3));
    }
}

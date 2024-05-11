#![doc(alias = "valency")]
//! A trait to get the degree of a given vertex
//!
//! The degree of a vertex is the number of edges incident on it. For directed
//! graphs, the degree is the sum of the indegree and outdegree.
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::Degree,
//!     std::collections::HashSet,
//! };
//!
//! let graph = vec![
//!     HashSet::from([1, 2]),
//!     HashSet::from([2]),
//!     HashSet::from([0]),
//! ];
//!
//! assert_eq!(graph.degree(0), 3);
//! assert_eq!(graph.degree(1), 2);
//! assert_eq!(graph.degree(2), 3);
//! ```

extern crate alloc;

use {
    crate::op::{
        Indegree,
        Outdegree,
    },
    alloc::collections::{
        BTreeMap,
        BTreeSet,
    },
    core::hash::BuildHasher,
    std::collections::{
        HashMap,
        HashSet,
    },
};

/// A trait to get the degree of a given vertex
///
/// # How can I implement `Degree`?
///
/// Provide an implementation of `Degree` that returns the degree of the vertex.
///
/// ```
/// use {
///     graaf::op::{
///         Degree,
///         Indegree,
///         Outdegree,
///     },
///     std::collections::HashSet,
/// };
///
/// struct Graph {
///     edges: Vec<HashSet<usize>>,
/// }
///
/// impl Degree for Graph {
///     fn degree(&self, s: usize) -> usize {
///         self.edges.indegree(s) + self.edges.outdegree(s)
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use {
///     graaf::op::Degree,
///     std::collections::HashSet,
/// };
///
/// let graph = vec![
///     HashSet::from([1, 2]),
///     HashSet::from([2]),
///     HashSet::from([0]),
/// ];
///
/// assert_eq!(graph.degree(0), 3);
/// assert_eq!(graph.degree(1), 2);
/// assert_eq!(graph.degree(2), 3);
/// ```
pub trait Degree {
    /// Returns the degree of a vertex.
    ///
    /// # Arguments
    ///
    /// * `s`: The vertex.
    fn degree(&self, s: usize) -> usize;
}

impl Degree for Vec<BTreeSet<usize>> {
    fn degree(&self, s: usize) -> usize {
        self.indegree(s) + self.outdegree(s)
    }
}

impl<H> Degree for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    fn degree(&self, s: usize) -> usize {
        self.indegree(s) + self.outdegree(s)
    }
}

impl Degree for [BTreeSet<usize>] {
    fn degree(&self, s: usize) -> usize {
        self.indegree(s) + self.outdegree(s)
    }
}

impl<H> Degree for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    fn degree(&self, s: usize) -> usize {
        self.indegree(s) + self.outdegree(s)
    }
}

impl<const V: usize> Degree for [BTreeSet<usize>; V] {
    fn degree(&self, s: usize) -> usize {
        self.indegree(s) + self.outdegree(s)
    }
}

impl<const V: usize, H> Degree for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    fn degree(&self, s: usize) -> usize {
        self.indegree(s) + self.outdegree(s)
    }
}

impl Degree for BTreeMap<usize, BTreeSet<usize>> {
    fn degree(&self, s: usize) -> usize {
        self.indegree(s) + self.outdegree(s)
    }
}

impl<H> Degree for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    fn degree(&self, s: usize) -> usize {
        self.indegree(s) + self.outdegree(s)
    }
}

impl<W> Degree for Vec<BTreeMap<usize, W>> {
    fn degree(&self, s: usize) -> usize {
        self.indegree(s) + self.outdegree(s)
    }
}

impl<W, H> Degree for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn degree(&self, s: usize) -> usize {
        self.indegree(s) + self.outdegree(s)
    }
}

impl<W> Degree for [BTreeMap<usize, W>] {
    fn degree(&self, s: usize) -> usize {
        self.indegree(s) + self.outdegree(s)
    }
}

impl<W, H> Degree for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    fn degree(&self, s: usize) -> usize {
        self.indegree(s) + self.outdegree(s)
    }
}

impl<const V: usize, W> Degree for [BTreeMap<usize, W>; V] {
    fn degree(&self, s: usize) -> usize {
        self.indegree(s) + self.outdegree(s)
    }
}

impl<const V: usize, W, H> Degree for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn degree(&self, s: usize) -> usize {
        self.indegree(s) + self.outdegree(s)
    }
}

impl<W> Degree for BTreeMap<usize, BTreeMap<usize, W>> {
    fn degree(&self, s: usize) -> usize {
        self.indegree(s) + self.outdegree(s)
    }
}

impl<W, H> Degree for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    fn degree(&self, s: usize) -> usize {
        self.indegree(s) + self.outdegree(s)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            gen::{
                Empty,
                EmptyConst,
            },
            op::{
                AddEdge,
                AddWeightedEdge,
            },
        },
    };

    macro_rules! test_degree {
        ($graph:expr) => {
            assert_eq!($graph.degree(0), 3);
            assert_eq!($graph.degree(1), 2);
            assert_eq!($graph.degree(2), 3);
        };
    }

    macro_rules! test_degree_unweighted {
        ($graph:expr) => {
            $graph.add_edge(0, 1);
            $graph.add_edge(0, 2);
            $graph.add_edge(1, 2);
            $graph.add_edge(2, 0);

            test_degree!($graph);
        };
    }

    macro_rules! test_degree_weighted {
        ($graph:expr) => {
            $graph.add_weighted_edge(0, 1, 1);
            $graph.add_weighted_edge(0, 2, 2);
            $graph.add_weighted_edge(1, 2, 3);
            $graph.add_weighted_edge(2, 0, 2);

            test_degree!($graph);
        };
    }

    #[test]
    fn vec_btree_set() {
        let graph = &mut <Vec<BTreeSet<usize>>>::empty(3);

        test_degree_unweighted!(graph);
    }

    #[test]
    fn vec_hash_set() {
        let graph = &mut <Vec<HashSet<usize>>>::empty(3);

        test_degree_unweighted!(graph);
    }

    #[test]
    fn slice_btree_set() {
        let graph: &mut [BTreeSet<usize>] = &mut Vec::<BTreeSet<usize>>::empty(3);

        test_degree_unweighted!(graph);
    }

    #[test]
    fn slice_hash_set() {
        let graph: &mut [HashSet<usize>] = &mut Vec::<HashSet<usize>>::empty(3);

        test_degree_unweighted!(graph);
    }

    #[test]
    fn arr_btree_set() {
        let graph = &mut <[BTreeSet<usize>; 3]>::empty();

        test_degree_unweighted!(graph);
    }

    #[test]
    fn arr_hash_set() {
        let graph = &mut <[HashSet<usize>; 3]>::empty();

        test_degree_unweighted!(graph);
    }

    #[test]
    fn vec_btree_map() {
        let graph = &mut <Vec<BTreeMap<usize, usize>>>::empty(3);

        test_degree_weighted!(graph);
    }

    #[test]
    fn vec_hash_map() {
        let graph = &mut <Vec<HashMap<usize, usize>>>::empty(3);

        test_degree_weighted!(graph);
    }

    #[test]
    fn slice_btree_map() {
        let graph: &mut [BTreeMap<usize, usize>] = &mut Vec::<BTreeMap<usize, usize>>::empty(3);

        test_degree_weighted!(graph);
    }

    #[test]
    fn slice_hash_map() {
        let graph: &mut [HashMap<usize, usize>] = &mut Vec::<HashMap<usize, usize>>::empty(3);

        test_degree_weighted!(graph);
    }

    #[test]
    fn arr_btree_map() {
        let graph = &mut <[BTreeMap<usize, usize>; 3]>::empty();

        test_degree_weighted!(graph);
    }

    #[test]
    fn arr_hash_map() {
        let graph = &mut <[HashMap<usize, usize>; 3]>::empty();

        test_degree_weighted!(graph);
    }

    #[test]
    fn btree_map_btree_set() {
        let graph = &mut BTreeMap::<usize, BTreeSet<usize>>::empty(3);

        test_degree_unweighted!(graph);
    }

    #[test]
    fn btree_map_btree_map() {
        let graph = &mut BTreeMap::<usize, BTreeMap<usize, usize>>::empty(3);

        test_degree_weighted!(graph);
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = &mut HashMap::<usize, HashSet<usize>>::empty(3);

        test_degree_unweighted!(graph);
    }

    #[test]
    fn hash_map_hash_map() {
        let graph = &mut HashMap::<usize, HashMap<usize, usize>>::empty(3);

        test_degree_weighted!(graph);
    }
}

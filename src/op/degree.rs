#![doc(alias = "valency")]
//! A trait to get the degree of a given vertex
//!
//! For digraphs, the degree is the sum of the indegree and outdegree.
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::Degree,
//!     std::collections::HashSet,
//! };
//!
//! let digraph = vec![
//!     HashSet::from([1, 2]),
//!     HashSet::from([2]),
//!     HashSet::from([0]),
//! ];
//!
//! assert_eq!(digraph.degree(0), 3);
//! assert_eq!(digraph.degree(1), 2);
//! assert_eq!(digraph.degree(2), 3);
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
/// struct Digraph {
///     arcs: Vec<HashSet<usize>>,
/// }
///
/// impl Degree for Digraph {
///     fn degree(&self, s: usize) -> usize {
///         self.arcs.indegree(s) + self.arcs.outdegree(s)
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
/// let digraph = vec![
///     HashSet::from([1, 2]),
///     HashSet::from([2]),
///     HashSet::from([0]),
/// ];
///
/// assert_eq!(digraph.degree(0), 3);
/// assert_eq!(digraph.degree(1), 2);
/// assert_eq!(digraph.degree(2), 3);
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
                AddArc,
                AddWeightedArc,
            },
        },
    };

    macro_rules! test_degree {
        ($digraph:expr) => {
            assert_eq!($digraph.degree(0), 3);
            assert_eq!($digraph.degree(1), 2);
            assert_eq!($digraph.degree(2), 3);
        };
    }

    macro_rules! test_degree_unweighted {
        ($digraph:expr) => {
            $digraph.add_arc(0, 1);
            $digraph.add_arc(0, 2);
            $digraph.add_arc(1, 2);
            $digraph.add_arc(2, 0);

            test_degree!($digraph);
        };
    }

    macro_rules! test_degree_weighted {
        ($digraph:expr) => {
            $digraph.add_weighted_arc(0, 1, 1);
            $digraph.add_weighted_arc(0, 2, 2);
            $digraph.add_weighted_arc(1, 2, 3);
            $digraph.add_weighted_arc(2, 0, 2);

            test_degree!($digraph);
        };
    }

    #[test]
    fn vec_btree_set() {
        let digraph = &mut <Vec<BTreeSet<usize>>>::empty(3);

        test_degree_unweighted!(digraph);
    }

    #[test]
    fn vec_hash_set() {
        let digraph = &mut <Vec<HashSet<usize>>>::empty(3);

        test_degree_unweighted!(digraph);
    }

    #[test]
    fn slice_btree_set() {
        let digraph: &mut [BTreeSet<usize>] = &mut Vec::<BTreeSet<usize>>::empty(3);

        test_degree_unweighted!(digraph);
    }

    #[test]
    fn slice_hash_set() {
        let digraph: &mut [HashSet<usize>] = &mut Vec::<HashSet<usize>>::empty(3);

        test_degree_unweighted!(digraph);
    }

    #[test]
    fn arr_btree_set() {
        let digraph = &mut <[BTreeSet<usize>; 3]>::empty();

        test_degree_unweighted!(digraph);
    }

    #[test]
    fn arr_hash_set() {
        let digraph = &mut <[HashSet<usize>; 3]>::empty();

        test_degree_unweighted!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let digraph = &mut BTreeMap::<usize, BTreeSet<usize>>::empty(3);

        test_degree_unweighted!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let digraph = &mut HashMap::<usize, HashSet<usize>>::empty(3);

        test_degree_unweighted!(digraph);
    }

    #[test]
    fn vec_btree_map() {
        let digraph = &mut <Vec<BTreeMap<usize, usize>>>::empty(3);

        test_degree_weighted!(digraph);
    }

    #[test]
    fn vec_hash_map() {
        let digraph = &mut <Vec<HashMap<usize, usize>>>::empty(3);

        test_degree_weighted!(digraph);
    }

    #[test]
    fn slice_btree_map() {
        let digraph: &mut [BTreeMap<usize, usize>] = &mut Vec::<BTreeMap<usize, usize>>::empty(3);

        test_degree_weighted!(digraph);
    }

    #[test]
    fn slice_hash_map() {
        let digraph: &mut [HashMap<usize, usize>] = &mut Vec::<HashMap<usize, usize>>::empty(3);

        test_degree_weighted!(digraph);
    }

    #[test]
    fn arr_btree_map() {
        let digraph = &mut <[BTreeMap<usize, usize>; 3]>::empty();

        test_degree_weighted!(digraph);
    }

    #[test]
    fn arr_hash_map() {
        let digraph = &mut <[HashMap<usize, usize>; 3]>::empty();

        test_degree_weighted!(digraph);
    }

    #[test]
    fn btree_map_btree_map() {
        let digraph = &mut BTreeMap::<usize, BTreeMap<usize, usize>>::empty(3);

        test_degree_weighted!(digraph);
    }

    #[test]
    fn hash_map_hash_map() {
        let digraph = &mut HashMap::<usize, HashMap<usize, usize>>::empty(3);

        test_degree_weighted!(digraph);
    }
}

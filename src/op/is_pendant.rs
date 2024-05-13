//! A trait to determine whether a vertex in a directed graph is a pendant
//! vertex
//!
//! A vertex is a pendant vertex if it has a degree of one.
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::IsPendant,
//!     std::collections::HashSet,
//! };
//!
//! let graph = vec![
//!     HashSet::from([1, 2]),
//!     HashSet::from([0]),
//!     HashSet::new(),
//!     HashSet::from([0]),
//! ];
//!
//! assert!(!graph.is_pendant(0));
//! assert!(!graph.is_pendant(1));
//! assert!(graph.is_pendant(2));
//! assert!(graph.is_pendant(3));
//! assert!(!graph.is_pendant(4));
//! ```

extern crate alloc;

use {
    crate::op::Degree,
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

/// A trait to determine whether a vertex is a pendant vertex
///
/// # How can I implement `IsPendant`?
///
/// Provide an implementation of `is_pendant` that returns `true` if the vertex
/// is a pendant vertex and `false` otherwise.
///
/// ```
/// use {
///     graaf::op::{
///         Degree,
///         IsPendant,
///     },
///     std::collections::HashSet,
/// };
///
/// struct Graph {
///     edges: Vec<HashSet<usize>>,
/// }
///
/// impl IsPendant for Graph {
///     fn is_pendant(&self, s: usize) -> bool {
///         self.edges.degree(s) == 1
///     }
/// }
///
/// let graph = Graph {
///     edges: vec![
///         HashSet::from([1, 2]),
///         HashSet::from([0]),
///         HashSet::new(),
///         HashSet::from([0]),
///     ],
/// };
///
/// assert!(!graph.is_pendant(0));
/// assert!(!graph.is_pendant(1));
/// assert!(graph.is_pendant(2));
/// assert!(graph.is_pendant(3));
/// assert!(!graph.is_pendant(4));
/// ```
pub trait IsPendant {
    /// Returns `true` if the vertex is a pendant vertex and `false` otherwise
    fn is_pendant(&self, s: usize) -> bool;
}

impl IsPendant for Vec<BTreeSet<usize>> {
    fn is_pendant(&self, s: usize) -> bool {
        self.degree(s) == 1
    }
}

impl<H> IsPendant for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    fn is_pendant(&self, s: usize) -> bool {
        self.degree(s) == 1
    }
}

impl IsPendant for [BTreeSet<usize>] {
    fn is_pendant(&self, s: usize) -> bool {
        self.degree(s) == 1
    }
}

impl<H> IsPendant for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    fn is_pendant(&self, s: usize) -> bool {
        self.degree(s) == 1
    }
}

impl<const V: usize> IsPendant for [BTreeSet<usize>; V] {
    fn is_pendant(&self, s: usize) -> bool {
        self.degree(s) == 1
    }
}

impl<const V: usize, H> IsPendant for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    fn is_pendant(&self, s: usize) -> bool {
        self.degree(s) == 1
    }
}

impl IsPendant for BTreeMap<usize, BTreeSet<usize>> {
    fn is_pendant(&self, s: usize) -> bool {
        self.degree(s) == 1
    }
}

impl<H> IsPendant for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    fn is_pendant(&self, s: usize) -> bool {
        self.degree(s) == 1
    }
}

impl<W> IsPendant for Vec<BTreeMap<usize, W>> {
    fn is_pendant(&self, s: usize) -> bool {
        self.degree(s) == 1
    }
}

impl<W, H> IsPendant for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn is_pendant(&self, s: usize) -> bool {
        self.degree(s) == 1
    }
}

impl<W> IsPendant for [BTreeMap<usize, W>] {
    fn is_pendant(&self, s: usize) -> bool {
        self.degree(s) == 1
    }
}

impl<W, H> IsPendant for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    fn is_pendant(&self, s: usize) -> bool {
        self.degree(s) == 1
    }
}

impl<const V: usize, W> IsPendant for [BTreeMap<usize, W>; V] {
    fn is_pendant(&self, s: usize) -> bool {
        self.degree(s) == 1
    }
}

impl<const V: usize, W, H> IsPendant for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn is_pendant(&self, s: usize) -> bool {
        self.degree(s) == 1
    }
}

impl<W> IsPendant for BTreeMap<usize, BTreeMap<usize, W>> {
    fn is_pendant(&self, s: usize) -> bool {
        self.degree(s) == 1
    }
}

impl<W, H> IsPendant for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    fn is_pendant(&self, s: usize) -> bool {
        self.degree(s) == 1
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

    macro_rules! test_is_pendant {
        ($graph:expr) => {
            assert!(!$graph.is_pendant(0));
            assert!(!$graph.is_pendant(1));
            assert!($graph.is_pendant(2));
            assert!($graph.is_pendant(3));
            assert!(!$graph.is_pendant(4));
        };
    }

    macro_rules! test_is_pendant_unweighted {
        ($graph:expr) => {
            $graph.add_edge(0, 1);
            $graph.add_edge(0, 2);
            $graph.add_edge(1, 0);
            $graph.add_edge(3, 0);

            test_is_pendant!($graph);
        };
    }

    macro_rules! test_is_pendant_weighted {
        ($graph:expr) => {
            $graph.add_weighted_edge(0, 1, 1);
            $graph.add_weighted_edge(0, 2, 1);
            $graph.add_weighted_edge(1, 0, 1);
            $graph.add_weighted_edge(3, 0, 1);

            test_is_pendant!($graph);
        };
    }

    #[test]
    fn vec_btree_set() {
        let graph = &mut <Vec<BTreeSet<usize>>>::empty(4);

        test_is_pendant_unweighted!(graph);
    }

    #[test]
    fn vec_hash_set() {
        let graph = &mut <Vec<HashSet<usize>>>::empty(4);

        test_is_pendant_unweighted!(graph);
    }

    #[test]
    fn slice_btree_set() {
        let graph: &mut [BTreeSet<usize>] = &mut Vec::<BTreeSet<usize>>::empty(4);

        test_is_pendant_unweighted!(graph);
    }

    #[test]
    fn slice_hash_set() {
        let graph: &mut [HashSet<usize>] = &mut Vec::<HashSet<usize>>::empty(4);

        test_is_pendant_unweighted!(graph);
    }

    #[test]
    fn arr_btree_set() {
        let graph = &mut <[BTreeSet<usize>; 4]>::empty();

        test_is_pendant_unweighted!(graph);
    }

    #[test]
    fn arr_hash_set() {
        let graph = &mut <[HashSet<usize>; 4]>::empty();

        test_is_pendant_unweighted!(graph);
    }

    #[test]
    fn vec_btree_map() {
        let graph = &mut <Vec<BTreeMap<usize, usize>>>::empty(4);

        test_is_pendant_weighted!(graph);
    }

    #[test]
    fn vec_hash_map() {
        let graph = &mut <Vec<HashMap<usize, usize>>>::empty(4);

        test_is_pendant_weighted!(graph);
    }

    #[test]
    fn slice_btree_map() {
        let graph: &mut [BTreeMap<usize, usize>] = &mut Vec::<BTreeMap<usize, usize>>::empty(4);

        test_is_pendant_weighted!(graph);
    }

    #[test]
    fn slice_hash_map() {
        let graph: &mut [HashMap<usize, usize>] = &mut Vec::<HashMap<usize, usize>>::empty(4);

        test_is_pendant_weighted!(graph);
    }

    #[test]
    fn arr_btree_map() {
        let graph = &mut <[BTreeMap<usize, usize>; 4]>::empty();

        test_is_pendant_weighted!(graph);
    }

    #[test]
    fn arr_hash_map() {
        let graph = &mut <[HashMap<usize, usize>; 4]>::empty();

        test_is_pendant_weighted!(graph);
    }

    #[test]
    fn btree_map_btree_set() {
        let graph = &mut BTreeMap::<usize, BTreeSet<usize>>::empty(4);

        test_is_pendant_unweighted!(graph);
    }

    #[test]
    fn btree_map_btree_map() {
        let graph = &mut BTreeMap::<usize, BTreeMap<usize, usize>>::empty(4);

        test_is_pendant_weighted!(graph);
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = &mut HashMap::<usize, HashSet<usize>>::empty(4);

        test_is_pendant_unweighted!(graph);
    }

    #[test]
    fn hash_map_hash_map() {
        let graph = &mut HashMap::<usize, HashMap<usize, usize>>::empty(4);

        test_is_pendant_weighted!(graph);
    }
}

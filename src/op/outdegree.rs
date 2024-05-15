#![doc(alias = "out_valence")]
#![doc(alias = "outward_demidegree")]
//! A trait to get the outdegree of a given vertex in a directed graph
//!
//! The outdegree is the number of arcs incident out of a vertex.
//!
//! # Examples
//!
//! ```
//! use graaf::op::Outdegree;
//!
//! let graph = vec![vec![1, 2], vec![0], vec![1]];
//!
//! assert_eq!(graph.outdegree(0), 2);
//! assert_eq!(graph.outdegree(1), 1);
//! assert_eq!(graph.outdegree(2), 1);
//! ```

extern crate alloc;

use {
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

/// A trait to get the outdegree of a given vertex in a directed graph
///
/// # How can I implement `Outdegree`?
///
/// Provide an implementation of `outdegree` that returns the outdegree of the
/// target vertex. The implementation should not panic if the vertex is not in
/// the graph.
///
/// ```
/// use graaf::op::Outdegree;
///
/// struct Graph {
///     arcs: Vec<Vec<usize>>,
/// }
///
/// impl Outdegree for Graph {
///     fn outdegree(&self, s: usize) -> usize {
///         self.arcs.get(s).map_or(0, Vec::len)
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::Outdegree;
///
/// let graph = vec![vec![1, 2], vec![0], vec![1]];
///
/// assert_eq!(graph.outdegree(0), 2);
/// assert_eq!(graph.outdegree(1), 1);
/// assert_eq!(graph.outdegree(2), 1);
/// assert_eq!(graph.outdegree(3), 0);
/// ```
pub trait Outdegree {
    /// Returns the outdegree of a vertex.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    fn outdegree(&self, s: usize) -> usize;
}

impl Outdegree for Vec<Vec<usize>> {
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, Vec::len)
    }
}

impl<W> Outdegree for Vec<Vec<(usize, W)>> {
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, Vec::len)
    }
}

impl Outdegree for Vec<BTreeSet<usize>> {
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, BTreeSet::len)
    }
}

impl<W> Outdegree for Vec<BTreeSet<(usize, W)>> {
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, BTreeSet::len)
    }
}

impl<H> Outdegree for Vec<HashSet<usize, H>> {
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, HashSet::len)
    }
}

impl<W, H> Outdegree for Vec<HashSet<(usize, W), H>> {
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, HashSet::len)
    }
}

impl<W> Outdegree for Vec<BTreeMap<usize, W>> {
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, BTreeMap::len)
    }
}

impl<W, H> Outdegree for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, HashMap::len)
    }
}

impl Outdegree for [Vec<usize>] {
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, Vec::len)
    }
}

impl<W> Outdegree for [Vec<(usize, W)>] {
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, Vec::len)
    }
}

impl Outdegree for [BTreeSet<usize>] {
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, BTreeSet::len)
    }
}

impl<W> Outdegree for [BTreeSet<(usize, W)>] {
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, BTreeSet::len)
    }
}

impl<H> Outdegree for [HashSet<usize, H>] {
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, HashSet::len)
    }
}

impl<W, H> Outdegree for [HashSet<(usize, W), H>] {
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, HashSet::len)
    }
}

impl<W> Outdegree for [BTreeMap<usize, W>] {
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, BTreeMap::len)
    }
}

impl<W, H> Outdegree for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, HashMap::len)
    }
}

impl<const V: usize> Outdegree for [Vec<usize>; V] {
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, Vec::len)
    }
}

impl<const V: usize, W> Outdegree for [Vec<(usize, W)>; V] {
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, Vec::len)
    }
}

impl<const V: usize> Outdegree for [BTreeSet<usize>; V] {
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, BTreeSet::len)
    }
}

impl<const V: usize, W> Outdegree for [BTreeSet<(usize, W)>; V] {
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, BTreeSet::len)
    }
}

impl<const V: usize, H> Outdegree for [HashSet<usize, H>; V] {
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, HashSet::len)
    }
}

impl<const V: usize, W, H> Outdegree for [HashSet<(usize, W), H>; V] {
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, HashSet::len)
    }
}

impl<const V: usize, W> Outdegree for [BTreeMap<usize, W>; V] {
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, BTreeMap::len)
    }
}

impl<const V: usize, W, H> Outdegree for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn outdegree(&self, s: usize) -> usize {
        self.get(s).map_or(0, HashMap::len)
    }
}

impl Outdegree for BTreeMap<usize, Vec<usize>> {
    fn outdegree(&self, s: usize) -> usize {
        self.get(&s).map_or(0, Vec::len)
    }
}

impl<H> Outdegree for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    fn outdegree(&self, s: usize) -> usize {
        self.get(&s).map_or(0, Vec::len)
    }
}

impl Outdegree for BTreeMap<usize, BTreeSet<usize>> {
    fn outdegree(&self, s: usize) -> usize {
        self.get(&s).map_or(0, BTreeSet::len)
    }
}

impl<H> Outdegree for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    fn outdegree(&self, s: usize) -> usize {
        self.get(&s).map_or(0, HashSet::len)
    }
}

impl<W> Outdegree for BTreeMap<usize, BTreeMap<usize, W>> {
    fn outdegree(&self, s: usize) -> usize {
        self.get(&s).map_or(0, BTreeMap::len)
    }
}

impl<W, H> Outdegree for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    fn outdegree(&self, s: usize) -> usize {
        self.get(&s).map_or(0, HashMap::len)
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

    macro_rules! test_outdegree {
        ($graph:expr) => {
            assert_eq!($graph.outdegree(0), 2);
            assert_eq!($graph.outdegree(1), 1);
            assert_eq!($graph.outdegree(2), 1);
            assert_eq!($graph.outdegree(3), 0);
        };
    }

    macro_rules! test_outdegree_unweighted {
        ($graph:expr) => {
            $graph.add_arc(0, 1);
            $graph.add_arc(0, 2);
            $graph.add_arc(1, 0);
            $graph.add_arc(2, 1);

            test_outdegree!($graph);
        };
    }

    macro_rules! test_outdegree_weighted {
        ($graph:expr) => {
            $graph.add_weighted_arc(0, 1, 1);
            $graph.add_weighted_arc(0, 2, 2);
            $graph.add_weighted_arc(1, 0, 3);
            $graph.add_weighted_arc(2, 1, 4);

            test_outdegree!($graph);
        };
    }

    #[test]
    fn vec_vec_unweighted() {
        let graph = &mut Vec::<Vec<usize>>::empty(4);

        test_outdegree_unweighted!(graph);
    }

    #[test]
    fn vec_btree_set_unweighted() {
        let graph = &mut Vec::<BTreeSet<usize>>::empty(4);

        test_outdegree_unweighted!(graph);
    }

    #[test]
    fn vec_hash_set_unweighted() {
        let graph = &mut Vec::<HashSet<usize>>::empty(4);

        test_outdegree_unweighted!(graph);
    }

    #[test]
    fn slice_vec_unweighted() {
        let graph: &mut [Vec<usize>] = &mut Vec::<Vec<usize>>::empty(4);

        test_outdegree_unweighted!(graph);
    }

    #[test]
    fn slice_btree_set_unweighted() {
        let graph: &mut [BTreeSet<usize>] = &mut Vec::<BTreeSet<usize>>::empty(4);

        test_outdegree_unweighted!(graph);
    }

    #[test]
    fn slice_hash_set_unweighted() {
        let graph: &mut [HashSet<usize>] = &mut Vec::<HashSet<usize>>::empty(4);

        test_outdegree_unweighted!(graph);
    }

    #[test]
    fn arr_vec_unweighted() {
        let graph = &mut <[Vec<usize>; 4]>::empty();

        test_outdegree_unweighted!(graph);
    }

    #[test]
    fn arr_btree_set_unweighted() {
        let graph = &mut <[BTreeSet<usize>; 4]>::empty();

        test_outdegree_unweighted!(graph);
    }

    #[test]
    fn arr_hash_set_unweighted() {
        let graph = &mut <[HashSet<usize>; 4]>::empty();

        test_outdegree_unweighted!(graph);
    }

    #[test]
    fn btree_map_vec() {
        let graph = &mut BTreeMap::<usize, Vec<usize>>::empty(4);

        test_outdegree_unweighted!(graph);
    }

    #[test]
    fn hash_map_vec() {
        let graph = &mut HashMap::<usize, Vec<usize>>::empty(4);

        test_outdegree_unweighted!(graph);
    }

    #[test]
    fn btree_map_btree_set() {
        let graph = &mut BTreeMap::<usize, BTreeSet<usize>>::empty(4);

        test_outdegree_unweighted!(graph);
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = &mut HashMap::<usize, HashSet<usize>>::empty(4);

        test_outdegree_unweighted!(graph);
    }

    #[test]
    fn vec_vec_weighted() {
        let graph = &mut Vec::<Vec<(usize, i32)>>::empty(4);

        test_outdegree_weighted!(graph);
    }

    #[test]
    fn vec_btree_set_weighted() {
        let graph = &mut Vec::<BTreeSet<(usize, i32)>>::empty(4);

        test_outdegree_weighted!(graph);
    }

    #[test]
    fn vec_hash_set_weighted() {
        let graph = &mut Vec::<HashSet<(usize, i32)>>::empty(4);

        test_outdegree_weighted!(graph);
    }

    #[test]
    fn vec_btree_map() {
        let graph = &mut Vec::<BTreeMap<usize, i32>>::empty(4);

        test_outdegree_weighted!(graph);
    }

    #[test]
    fn vec_hash_map() {
        let graph = &mut Vec::<HashMap<usize, i32>>::empty(4);

        test_outdegree_weighted!(graph);
    }

    #[test]
    fn slice_vec_weighted() {
        let graph: &mut [Vec<(usize, i32)>] = &mut Vec::<Vec<(usize, i32)>>::empty(4);

        test_outdegree_weighted!(graph);
    }

    #[test]
    fn slice_btree_set_weighted() {
        let graph: &mut [BTreeSet<(usize, i32)>] = &mut Vec::<BTreeSet<(usize, i32)>>::empty(4);

        test_outdegree_weighted!(graph);
    }

    #[test]
    fn slice_hash_set_weighted() {
        let graph: &mut [HashSet<(usize, i32)>] = &mut Vec::<HashSet<(usize, i32)>>::empty(4);

        test_outdegree_weighted!(graph);
    }

    #[test]
    fn slice_btree_map() {
        let graph: &mut [BTreeMap<usize, i32>] = &mut Vec::<BTreeMap<usize, i32>>::empty(4);

        test_outdegree_weighted!(graph);
    }

    #[test]
    fn slice_hash_map() {
        let graph: &mut [HashMap<usize, i32>] = &mut Vec::<HashMap<usize, i32>>::empty(4);

        test_outdegree_weighted!(graph);
    }

    #[test]
    fn arr_vec_weighted() {
        let graph = &mut <[Vec<(usize, i32)>; 4]>::empty();

        test_outdegree_weighted!(graph);
    }

    #[test]
    fn arr_btree_set_weighted() {
        let graph = &mut <[BTreeSet<(usize, i32)>; 4]>::empty();

        test_outdegree_weighted!(graph);
    }

    #[test]
    fn arr_hash_set_weighted() {
        let graph = &mut <[HashSet<(usize, i32)>; 4]>::empty();

        test_outdegree_weighted!(graph);
    }

    #[test]
    fn arr_btree_map() {
        let graph = &mut <[BTreeMap<usize, i32>; 4]>::empty();

        test_outdegree_weighted!(graph);
    }

    #[test]
    fn arr_hash_map() {
        let graph = &mut <[HashMap<usize, i32>; 4]>::empty();

        test_outdegree_weighted!(graph);
    }

    #[test]
    fn btree_map_btree_map() {
        let graph = &mut BTreeMap::<usize, BTreeMap<usize, i32>>::empty(4);

        test_outdegree_weighted!(graph);
    }

    #[test]
    fn hash_map_hash_map() {
        let graph = &mut HashMap::<usize, HashMap<usize, i32>>::empty(4);

        test_outdegree_weighted!(graph);
    }
}

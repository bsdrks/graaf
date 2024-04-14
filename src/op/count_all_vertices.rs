//! A trait to count all vertices in a graph
//!
//! # Example
//!
//! ```
//! use graaf::op::CountAllVertices;
//!
//! let graph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
//!
//! assert_eq!(graph.count_all_vertices(), 4);
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

/// A trait to count all vertices in a graph
///
/// # How can I implement `CountAllVertices`?
///
/// Provide an implementation of `count_all_vertices` that returns the number of
/// vertices in the graph.
///
/// ```
/// use graaf::op::CountAllVertices;
///
/// struct Graph {
///     vertices: Vec<usize>,
/// }
///
/// impl CountAllVertices for Graph {
///     fn count_all_vertices(&self) -> usize {
///         self.vertices.len()
///     }
/// }
/// ```
///
/// # Example
///
/// ```
/// use graaf::op::CountAllVertices;
///
/// let graph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
///
/// assert_eq!(graph.count_all_vertices(), 4);
/// ```
pub trait CountAllVertices {
    /// Count all vertices.
    fn count_all_vertices(&self) -> usize;
}

macro_rules! impl_by_len {
    () => {
        fn count_all_vertices(&self) -> usize {
            self.len()
        }
    };
}

macro_rules! impl_by_const {
    () => {
        fn count_all_vertices(&self) -> usize {
            V
        }
    };
}

impl CountAllVertices for Vec<Vec<usize>> {
    impl_by_len!();
}

impl<W> CountAllVertices for Vec<Vec<(usize, W)>> {
    impl_by_len!();
}

impl CountAllVertices for Vec<BTreeSet<usize>> {
    impl_by_len!();
}

impl<W> CountAllVertices for Vec<BTreeMap<usize, W>> {
    impl_by_len!();
}

impl<H> CountAllVertices for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    impl_by_len!();
}

impl<W, H> CountAllVertices for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    impl_by_len!();
}

impl CountAllVertices for [Vec<usize>] {
    impl_by_len!();
}

impl<W> CountAllVertices for [Vec<(usize, W)>] {
    impl_by_len!();
}

impl CountAllVertices for [BTreeSet<usize>] {
    impl_by_len!();
}

impl<H> CountAllVertices for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    impl_by_len!();
}

impl<W> CountAllVertices for [BTreeMap<usize, W>] {
    impl_by_len!();
}

impl<W, H> CountAllVertices for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    impl_by_len!();
}

impl<const V: usize> CountAllVertices for [Vec<usize>; V] {
    impl_by_const!();
}

impl<const V: usize, W> CountAllVertices for [Vec<(usize, W)>; V] {
    impl_by_const!();
}

impl<const V: usize> CountAllVertices for [BTreeSet<usize>; V] {
    impl_by_const!();
}

impl<const V: usize, W> CountAllVertices for [BTreeMap<usize, W>; V] {
    impl_by_const!();
}

impl<const V: usize, H> CountAllVertices for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    impl_by_const!();
}

impl<const V: usize, W, H> CountAllVertices for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    impl_by_const!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_vec_unweighted() {
        let graph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        assert_eq!(graph.count_all_vertices(), 4);
    }

    #[test]
    fn vec_vec_weighted() {
        let graph = vec![vec![(1, 2), (2, 3)], vec![(0, 4)], vec![(0, 7), (1, 8)]];

        assert_eq!(graph.count_all_vertices(), 3);
    }

    #[test]
    fn vec_btree_set() {
        let graph = vec![
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
        ];

        assert_eq!(graph.count_all_vertices(), 3);
    }

    #[test]
    fn vec_btree_map() {
        let graph = vec![
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 4)]),
            BTreeMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.count_all_vertices(), 3);
    }

    #[test]
    fn vec_hash_set() {
        let graph = vec![
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        assert_eq!(graph.count_all_vertices(), 4);
    }

    #[test]
    fn vec_hash_map() {
        let graph = vec![
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.count_all_vertices(), 3);
    }

    #[test]
    fn slice_vec_unweighted() {
        let graph: &[Vec<usize>] = &[vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        assert_eq!(graph.count_all_vertices(), 4);
    }

    #[test]
    fn slice_vec_weighted() {
        let graph: &[Vec<(usize, i32)>] =
            &[vec![(1, 2), (2, 3)], vec![(0, 4)], vec![(0, 7), (1, 8)]];

        assert_eq!(graph.count_all_vertices(), 3);
    }

    #[test]
    fn slice_btree_set() {
        let graph: &[BTreeSet<usize>] = &[
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
        ];

        assert_eq!(graph.count_all_vertices(), 3);
    }

    #[test]
    fn slice_btree_map() {
        let graph: &[BTreeMap<usize, i32>] = &[
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 4)]),
            BTreeMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.count_all_vertices(), 3);
    }

    #[test]
    fn slice_hash_set() {
        let graph: &[HashSet<usize>] = &[
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        assert_eq!(graph.count_all_vertices(), 4);
    }

    #[test]
    fn slice_hash_map() {
        let graph: &[HashMap<usize, i32>] = &[
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.count_all_vertices(), 3);
    }

    #[test]
    fn arr_vec_unweighted() {
        let graph = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        assert_eq!(graph.count_all_vertices(), 4);
    }

    #[test]
    fn arr_vec_weighted() {
        let graph = [vec![(1, 2), (2, 3)], vec![(0, 4)], vec![(0, 7), (1, 8)]];

        assert_eq!(graph.count_all_vertices(), 3);
    }

    #[test]
    fn arr_btree_set() {
        let graph = [
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
        ];

        assert_eq!(graph.count_all_vertices(), 3);
    }

    #[test]
    fn arr_btree_map() {
        let graph = [
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 4)]),
            BTreeMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.count_all_vertices(), 3);
    }

    #[test]
    fn arr_hash_set() {
        let graph = [
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
        ];

        assert_eq!(graph.count_all_vertices(), 3);
    }

    #[test]
    fn arr_hash_map() {
        let graph = [
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.count_all_vertices(), 3);
    }
}

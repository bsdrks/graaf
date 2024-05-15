//! A trait to count the number of vertices in a graph
//!
//! # Example
//!
//! ```
//! use graaf::op::Order;
//!
//! let graph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
//!
//! assert_eq!(graph.order(), 4);
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

/// A trait to count the number of vertices in a graph
///
/// # How can I implement `Order`?
///
/// Provides an implementation of `order` that returns the number
/// of vertices in the graph.
///
/// ```
/// use graaf::op::Order;
///
/// struct Graph {
///     vertices: Vec<usize>,
/// }
///
/// impl Order for Graph {
///     fn order(&self) -> usize {
///         self.vertices.len()
///     }
/// }
/// ```
///
/// # Example
///
/// ```
/// use graaf::op::Order;
///
/// let graph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
///
/// assert_eq!(graph.order(), 4);
/// ```
pub trait Order {
    /// Count all vertices.
    fn order(&self) -> usize;
}

impl Order for Vec<Vec<usize>> {
    fn order(&self) -> usize {
        self.len()
    }
}

impl<W> Order for Vec<Vec<(usize, W)>> {
    fn order(&self) -> usize {
        self.len()
    }
}

impl Order for Vec<BTreeSet<usize>> {
    fn order(&self) -> usize {
        self.len()
    }
}

impl<W> Order for Vec<BTreeSet<(usize, W)>> {
    fn order(&self) -> usize {
        self.len()
    }
}

impl<H> Order for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    fn order(&self) -> usize {
        self.len()
    }
}

impl<H, W> Order for Vec<HashSet<(usize, W), H>>
where
    H: BuildHasher,
{
    fn order(&self) -> usize {
        self.len()
    }
}

impl<W> Order for Vec<BTreeMap<usize, W>> {
    fn order(&self) -> usize {
        self.len()
    }
}

impl<W, H> Order for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn order(&self) -> usize {
        self.len()
    }
}

impl Order for [Vec<usize>] {
    fn order(&self) -> usize {
        self.len()
    }
}

impl<W> Order for [Vec<(usize, W)>] {
    fn order(&self) -> usize {
        self.len()
    }
}

impl Order for [BTreeSet<usize>] {
    fn order(&self) -> usize {
        self.len()
    }
}

impl<W> Order for [BTreeSet<(usize, W)>] {
    fn order(&self) -> usize {
        self.len()
    }
}

impl<H> Order for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    fn order(&self) -> usize {
        self.len()
    }
}

impl<H, W> Order for [HashSet<(usize, W), H>]
where
    H: BuildHasher,
{
    fn order(&self) -> usize {
        self.len()
    }
}

impl<W> Order for [BTreeMap<usize, W>] {
    fn order(&self) -> usize {
        self.len()
    }
}

impl<W, H> Order for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    fn order(&self) -> usize {
        self.len()
    }
}

impl<const V: usize> Order for [Vec<usize>; V] {
    fn order(&self) -> usize {
        V
    }
}

impl<const V: usize, W> Order for [Vec<(usize, W)>; V] {
    fn order(&self) -> usize {
        V
    }
}

impl<const V: usize> Order for [BTreeSet<usize>; V] {
    fn order(&self) -> usize {
        V
    }
}

impl<const V: usize, W> Order for [BTreeSet<(usize, W)>; V] {
    fn order(&self) -> usize {
        V
    }
}

impl<const V: usize, H> Order for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    fn order(&self) -> usize {
        V
    }
}

impl<const V: usize, W, H> Order for [HashSet<(usize, W), H>; V]
where
    H: BuildHasher,
{
    fn order(&self) -> usize {
        V
    }
}

impl<const V: usize, W> Order for [BTreeMap<usize, W>; V] {
    fn order(&self) -> usize {
        V
    }
}

impl<const V: usize, W, H> Order for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn order(&self) -> usize {
        V
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_vec_unweighted() {
        let graph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        assert_eq!(graph.order(), 4);
    }

    #[test]
    fn vec_vec_weighted() {
        let graph = vec![vec![(1, 2), (2, 3)], vec![(0, 4)], vec![(0, 7), (1, 8)]];

        assert_eq!(graph.order(), 3);
    }

    #[test]
    fn vec_btree_set_unweighted() {
        let graph = vec![
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
        ];

        assert_eq!(graph.order(), 3);
    }

    #[test]
    fn vec_btree_set_weighted() {
        let graph = vec![
            BTreeSet::from([(1, 2), (2, 3)]),
            BTreeSet::from([(0, 4)]),
            BTreeSet::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.order(), 3);
    }

    #[test]
    fn vec_hash_set_unweighted() {
        let graph = vec![
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        assert_eq!(graph.order(), 4);
    }

    #[test]
    fn vec_hash_set_weighted() {
        let graph = vec![
            HashSet::from([(1, 2), (2, 3)]),
            HashSet::from([(0, 4)]),
            HashSet::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.order(), 3);
    }

    #[test]
    fn vec_btree_map() {
        let graph = vec![
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 4)]),
            BTreeMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.order(), 3);
    }

    #[test]
    fn vec_hash_map() {
        let graph = vec![
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.order(), 3);
    }

    #[test]
    fn slice_vec_unweighted() {
        let graph: &[Vec<usize>] = &[vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        assert_eq!(graph.order(), 4);
    }

    #[test]
    fn slice_vec_weighted() {
        let graph: &[Vec<(usize, i32)>] =
            &[vec![(1, 2), (2, 3)], vec![(0, 4)], vec![(0, 7), (1, 8)]];

        assert_eq!(graph.order(), 3);
    }

    #[test]
    fn slice_btree_set_unweighted() {
        let graph: &[BTreeSet<usize>] = &[
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
        ];

        assert_eq!(graph.order(), 3);
    }

    #[test]
    fn slice_btree_set_weighted() {
        let graph: &[BTreeSet<(usize, i32)>] = &[
            BTreeSet::from([(1, 2), (2, 3)]),
            BTreeSet::from([(0, 4)]),
            BTreeSet::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.order(), 3);
    }

    #[test]
    fn slice_hash_set_unweighted() {
        let graph: &[HashSet<usize>] = &[
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        assert_eq!(graph.order(), 4);
    }

    #[test]
    fn slice_hash_set_weighted() {
        let graph: &[HashSet<(usize, i32)>] = &[
            HashSet::from([(1, 2), (2, 3)]),
            HashSet::from([(0, 4)]),
            HashSet::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.order(), 3);
    }

    #[test]
    fn slice_btree_map() {
        let graph: &[BTreeMap<usize, i32>] = &[
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 4)]),
            BTreeMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.order(), 3);
    }

    #[test]
    fn slice_hash_map() {
        let graph: &[HashMap<usize, i32>] = &[
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.order(), 3);
    }

    #[test]
    fn arr_vec_unweighted() {
        let graph = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        assert_eq!(graph.order(), 4);
    }

    #[test]
    fn arr_vec_weighted() {
        let graph = [vec![(1, 2), (2, 3)], vec![(0, 4)], vec![(0, 7), (1, 8)]];

        assert_eq!(graph.order(), 3);
    }

    #[test]
    fn arr_btree_set_unweighted() {
        let graph = [
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
        ];

        assert_eq!(graph.order(), 3);
    }

    #[test]
    fn arr_btree_set_weighted() {
        let graph = [
            BTreeSet::from([(1, 2), (2, 3)]),
            BTreeSet::from([(0, 4)]),
            BTreeSet::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.order(), 3);
    }

    #[test]
    fn arr_hash_set_unweighted() {
        let graph = [
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
        ];

        assert_eq!(graph.order(), 3);
    }

    #[test]
    fn arr_hash_set_weighted() {
        let graph = [
            HashSet::from([(1, 2), (2, 3)]),
            HashSet::from([(0, 4)]),
            HashSet::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.order(), 3);
    }

    #[test]
    fn arr_btree_map() {
        let graph = [
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 4)]),
            BTreeMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.order(), 3);
    }

    #[test]
    fn arr_hash_map() {
        let graph = [
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.order(), 3);
    }
}

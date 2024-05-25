//! A trait to count the number of vertices in a digraph
//!
//! # Example
//!
//! ```
//! use graaf::op::Order;
//!
//! let digraph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
//!
//! assert_eq!(digraph.order(), 4);
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

/// A trait to count the number of vertices in a digraph
///
/// # How can I implement `Order`?
///
/// Provides an implementation of `order` that returns the number
/// of vertices in the digraph.
///
/// ```
/// use graaf::op::Order;
///
/// struct Digraph {
///     vertices: Vec<usize>,
/// }
///
/// impl Order for Digraph {
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
/// let digraph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
///
/// assert_eq!(digraph.order(), 4);
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

impl<W, H> Order for Vec<HashSet<(usize, W), H>>
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

impl<W, H> Order for [HashSet<(usize, W), H>]
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
        let digraph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        assert_eq!(digraph.order(), 4);
    }

    #[test]
    fn vec_vec_weighted() {
        let digraph = vec![vec![(1, 2), (2, 3)], vec![(0, 4)], vec![(0, 7), (1, 8)]];

        assert_eq!(digraph.order(), 3);
    }

    #[test]
    fn vec_btree_set_unweighted() {
        let digraph = vec![
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
        ];

        assert_eq!(digraph.order(), 3);
    }

    #[test]
    fn vec_btree_set_weighted() {
        let digraph = vec![
            BTreeSet::from([(1, 2), (2, 3)]),
            BTreeSet::from([(0, 4)]),
            BTreeSet::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(digraph.order(), 3);
    }

    #[test]
    fn vec_hash_set_unweighted() {
        let digraph = vec![
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        assert_eq!(digraph.order(), 4);
    }

    #[test]
    fn vec_hash_set_weighted() {
        let digraph = vec![
            HashSet::from([(1, 2), (2, 3)]),
            HashSet::from([(0, 4)]),
            HashSet::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(digraph.order(), 3);
    }

    #[test]
    fn vec_btree_map() {
        let digraph = vec![
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 4)]),
            BTreeMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(digraph.order(), 3);
    }

    #[test]
    fn vec_hash_map() {
        let digraph = vec![
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(digraph.order(), 3);
    }

    #[test]
    fn slice_vec_unweighted() {
        let digraph: &[Vec<usize>] = &[vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        assert_eq!(digraph.order(), 4);
    }

    #[test]
    fn slice_vec_weighted() {
        let digraph: &[Vec<(usize, i32)>] =
            &[vec![(1, 2), (2, 3)], vec![(0, 4)], vec![(0, 7), (1, 8)]];

        assert_eq!(digraph.order(), 3);
    }

    #[test]
    fn slice_btree_set_unweighted() {
        let digraph: &[BTreeSet<usize>] = &[
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
        ];

        assert_eq!(digraph.order(), 3);
    }

    #[test]
    fn slice_btree_set_weighted() {
        let digraph: &[BTreeSet<(usize, i32)>] = &[
            BTreeSet::from([(1, 2), (2, 3)]),
            BTreeSet::from([(0, 4)]),
            BTreeSet::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(digraph.order(), 3);
    }

    #[test]
    fn slice_hash_set_unweighted() {
        let digraph: &[HashSet<usize>] = &[
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        assert_eq!(digraph.order(), 4);
    }

    #[test]
    fn slice_hash_set_weighted() {
        let digraph: &[HashSet<(usize, i32)>] = &[
            HashSet::from([(1, 2), (2, 3)]),
            HashSet::from([(0, 4)]),
            HashSet::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(digraph.order(), 3);
    }

    #[test]
    fn slice_btree_map() {
        let digraph: &[BTreeMap<usize, i32>] = &[
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 4)]),
            BTreeMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(digraph.order(), 3);
    }

    #[test]
    fn slice_hash_map() {
        let digraph: &[HashMap<usize, i32>] = &[
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(digraph.order(), 3);
    }

    #[test]
    fn arr_vec_unweighted() {
        let digraph = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        assert_eq!(digraph.order(), 4);
    }

    #[test]
    fn arr_vec_weighted() {
        let digraph = [vec![(1, 2), (2, 3)], vec![(0, 4)], vec![(0, 7), (1, 8)]];

        assert_eq!(digraph.order(), 3);
    }

    #[test]
    fn arr_btree_set_unweighted() {
        let digraph = [
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
        ];

        assert_eq!(digraph.order(), 3);
    }

    #[test]
    fn arr_btree_set_weighted() {
        let digraph = [
            BTreeSet::from([(1, 2), (2, 3)]),
            BTreeSet::from([(0, 4)]),
            BTreeSet::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(digraph.order(), 3);
    }

    #[test]
    fn arr_hash_set_unweighted() {
        let digraph = [
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
        ];

        assert_eq!(digraph.order(), 3);
    }

    #[test]
    fn arr_hash_set_weighted() {
        let digraph = [
            HashSet::from([(1, 2), (2, 3)]),
            HashSet::from([(0, 4)]),
            HashSet::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(digraph.order(), 3);
    }

    #[test]
    fn arr_btree_map() {
        let digraph = [
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 4)]),
            BTreeMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(digraph.order(), 3);
    }

    #[test]
    fn arr_hash_map() {
        let digraph = [
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(digraph.order(), 3);
    }
}

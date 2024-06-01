//! Iterate over the vertices in a digraph.
//!
//! # Example
//!
//! ```
//! use graaf::op::IterVertices;
//!
//! let digraph = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
//!
//! assert!(digraph.iter_vertices().eq(0..4));
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

/// Iterate over the vertices in a digraph.
///
/// # How can I implement `IterVertices`?
///
/// Provide an implementation of `iter_vertices` that returns an iterator over
/// all vertices in the digraph.
///
/// ```
/// use graaf::op::IterVertices;
///
/// struct Digraph {
///     vertices: Vec<usize>,
/// }
///
/// impl IterVertices for Digraph {
///     fn iter_vertices(&self) -> impl Iterator<Item = usize> {
///         0..self.vertices.len()
///     }
/// }
/// ```
///
/// # Example
///
/// ```
/// use graaf::op::IterVertices;
///
/// let digraph = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
///
/// assert!(digraph.iter_vertices().eq(0..4));
/// ```
pub trait IterVertices {
    /// Returns an iterator over the vertices.
    fn iter_vertices(&self) -> impl Iterator<Item = usize>;
}

macro_rules! impl_len {
    () => {
        fn iter_vertices(&self) -> impl Iterator<Item = usize> {
            0..self.len()
        }
    };
}

macro_rules! impl_const {
    () => {
        fn iter_vertices(&self) -> impl Iterator<Item = usize> {
            0..V
        }
    };
}

macro_rules! impl_map_unweighted {
    ($ty:ident) => {
        fn iter_vertices(&self) -> impl Iterator<Item = usize> {
            self.keys()
                .copied()
                .chain(self.values().flat_map(|v| v.iter().copied()))
                .collect::<$ty<_>>()
                .into_iter()
        }
    };
}

macro_rules! impl_map_weighted {
    ($ty:ident) => {
        fn iter_vertices(&self) -> impl Iterator<Item = usize> {
            self.keys()
                .copied()
                .chain(self.values().flat_map(|v| v.iter().map(|(t, _)| *t)))
                .collect::<$ty<_>>()
                .into_iter()
        }
    };
}

impl IterVertices for Vec<Vec<usize>> {
    impl_len!();
}

impl IterVertices for Vec<BTreeSet<usize>> {
    impl_len!();
}

impl<H> IterVertices for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    impl_len!();
}

impl IterVertices for [Vec<usize>] {
    impl_len!();
}

impl IterVertices for [BTreeSet<usize>] {
    impl_len!();
}

impl<H> IterVertices for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    impl_len!();
}

impl<const V: usize> IterVertices for [Vec<usize>; V] {
    impl_const!();
}

impl<const V: usize> IterVertices for [BTreeSet<usize>; V] {
    impl_const!();
}

impl IterVertices for BTreeMap<usize, Vec<usize>> {
    impl_map_unweighted!(BTreeSet);
}

impl IterVertices for BTreeMap<usize, BTreeSet<usize>> {
    impl_map_unweighted!(BTreeSet);
}

impl<H> IterVertices for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    impl_map_unweighted!(HashSet);
}

impl<H> IterVertices for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    impl_map_unweighted!(HashSet);
}

impl<W> IterVertices for Vec<Vec<(usize, W)>> {
    impl_len!();
}

impl<W> IterVertices for Vec<BTreeSet<(usize, W)>> {
    impl_len!();
}

impl<W> IterVertices for Vec<BTreeMap<usize, W>> {
    impl_len!();
}

impl<W, H> IterVertices for Vec<HashSet<(usize, W), H>>
where
    H: BuildHasher,
{
    impl_len!();
}

impl<W, H> IterVertices for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    impl_len!();
}

impl<W> IterVertices for [Vec<(usize, W)>] {
    impl_len!();
}

impl<W> IterVertices for [BTreeSet<(usize, W)>] {
    impl_len!();
}

impl<W> IterVertices for [BTreeMap<usize, W>] {
    impl_len!();
}

impl<W, H> IterVertices for [HashSet<(usize, W), H>]
where
    H: BuildHasher,
{
    impl_len!();
}

impl<W, H> IterVertices for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    impl_len!();
}

impl<const V: usize, W> IterVertices for [Vec<(usize, W)>; V] {
    impl_const!();
}

impl<const V: usize, W> IterVertices for [BTreeSet<(usize, W)>; V] {
    impl_const!();
}

impl<const V: usize, W> IterVertices for [BTreeMap<usize, W>; V] {
    impl_const!();
}

impl<const V: usize, H> IterVertices for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    impl_const!();
}

impl<const V: usize, W, H> IterVertices for [HashSet<(usize, W), H>; V]
where
    H: BuildHasher,
{
    impl_const!();
}

impl<const V: usize, W, H> IterVertices for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    impl_const!();
}

impl<W> IterVertices for BTreeMap<usize, Vec<(usize, W)>> {
    impl_map_weighted!(BTreeSet);
}

impl<W> IterVertices for BTreeMap<usize, BTreeSet<(usize, W)>> {
    impl_map_weighted!(BTreeSet);
}

impl<W> IterVertices for BTreeMap<usize, BTreeMap<usize, W>> {
    impl_map_weighted!(BTreeSet);
}

impl<W, H> IterVertices for HashMap<usize, Vec<(usize, W)>, H>
where
    H: BuildHasher,
{
    impl_map_weighted!(HashSet);
}

impl<W, H> IterVertices for HashMap<usize, HashSet<(usize, W), H>, H>
where
    H: BuildHasher,
{
    impl_map_weighted!(HashSet);
}

impl<W, H> IterVertices for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    impl_map_weighted!(HashSet);
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

    macro_rules! setup_unweighted {
        ($digraph:expr) => {
            $digraph.add_arc(0, 1);
            $digraph.add_arc(0, 2);
            $digraph.add_arc(1, 0);
            $digraph.add_arc(1, 2);
            $digraph.add_arc(1, 3);
            $digraph.add_arc(2, 0);
            $digraph.add_arc(2, 1);
            $digraph.add_arc(2, 3);
            $digraph.add_arc(3, 1);
            $digraph.add_arc(3, 2);
        };
    }

    macro_rules! setup_weighted {
        ($digraph:expr) => {
            $digraph.add_weighted_arc(0, 1, 2);
            $digraph.add_weighted_arc(0, 2, 3);
            $digraph.add_weighted_arc(1, 0, 4);
            $digraph.add_weighted_arc(1, 2, 5);
            $digraph.add_weighted_arc(1, 3, 6);
            $digraph.add_weighted_arc(2, 0, 7);
            $digraph.add_weighted_arc(2, 1, 8);
            $digraph.add_weighted_arc(2, 3, 9);
            $digraph.add_weighted_arc(3, 1, 10);
            $digraph.add_weighted_arc(3, 2, 11);
        };
    }

    macro_rules! test_stable {
        ($digraph:expr) => {
            assert!($digraph.iter_vertices().eq(0..=3));
        };
    }

    macro_rules! test_unstable {
        ($digraph:expr) => {
            let mut iter = $digraph.iter_vertices();

            assert!(matches!(iter.next(), Some(0..=3)));
            assert!(matches!(iter.next(), Some(0..=3)));
            assert!(matches!(iter.next(), Some(0..=3)));
            assert!(matches!(iter.next(), Some(0..=3)));
            assert_eq!(iter.next(), None);
        };
    }

    #[test]
    fn vec_vec_unweighted() {
        let mut digraph = Vec::<Vec<usize>>::empty(4);

        setup_unweighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn vec_btree_set_unweighted() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(4);

        setup_unweighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn vec_hash_set_unweighted() {
        let mut digraph = Vec::<HashSet<usize>>::empty(4);

        setup_unweighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn slice_vec_unweighted() {
        let mut digraph = Vec::<Vec<usize>>::empty(4);

        setup_unweighted!(digraph);
        test_stable!(digraph.as_slice());
    }

    #[test]
    fn slice_btree_set_unweighted() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(4);

        setup_unweighted!(digraph);
        test_stable!(digraph.as_slice());
    }

    #[test]
    fn slice_hash_set_unweighted() {
        let mut digraph = Vec::<HashSet<usize>>::empty(4);

        setup_unweighted!(digraph);
        test_stable!(digraph.as_slice());
    }

    #[test]
    fn arr_vec_unweighted() {
        let mut digraph = <[Vec<usize>; 4]>::empty();

        setup_unweighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn arr_btree_set_unweighted() {
        let mut digraph = <[BTreeSet<usize>; 4]>::empty();

        setup_unweighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn arr_hash_set_unweighted() {
        let mut digraph = <[HashSet<usize>; 4]>::empty();

        setup_unweighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn btree_map_vec_unweighted() {
        let mut digraph = BTreeMap::<usize, Vec<usize>>::empty(4);

        setup_unweighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn btree_map_btree_set_unweighted() {
        let mut digraph = BTreeMap::<usize, BTreeSet<usize>>::empty(4);

        setup_unweighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn hash_map_vec_unweighted() {
        let mut digraph = HashMap::<usize, Vec<usize>>::empty(4);

        setup_unweighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn hash_map_hash_set_unweighted() {
        let mut digraph = HashMap::<usize, HashSet<usize>>::empty(4);

        setup_unweighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn vec_vec_weighted() {
        let mut digraph = Vec::<Vec<(usize, i32)>>::empty(4);

        setup_weighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn vec_btree_set_weighted() {
        let mut digraph = Vec::<BTreeSet<(usize, i32)>>::empty(4);

        setup_weighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn vec_hash_set_weighted() {
        let mut digraph = Vec::<HashSet<(usize, i32)>>::empty(4);

        setup_weighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn vec_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, i32>>::empty(4);

        setup_weighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn vec_hash_map() {
        let mut digraph = Vec::<HashMap<usize, i32>>::empty(4);

        setup_weighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn slice_vec_weighted() {
        let mut digraph = Vec::<Vec<(usize, i32)>>::empty(4);

        setup_weighted!(digraph);
        test_stable!(digraph.as_slice());
    }

    #[test]
    fn slice_btree_set_weighted() {
        let mut digraph = Vec::<BTreeSet<(usize, i32)>>::empty(4);

        setup_weighted!(digraph);
        test_stable!(digraph.as_slice());
    }

    #[test]
    fn slice_hash_set_weighted() {
        let mut digraph = Vec::<HashSet<(usize, i32)>>::empty(4);

        setup_weighted!(digraph);
        test_stable!(digraph.as_slice());
    }

    #[test]
    fn slice_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, i32>>::empty(4);

        setup_weighted!(digraph);
        test_stable!(digraph.as_slice());
    }

    #[test]
    fn slice_hash_map() {
        let mut digraph = Vec::<HashMap<usize, i32>>::empty(4);

        setup_weighted!(digraph);
        test_unstable!(digraph.as_slice());
    }

    #[test]
    fn arr_vec_weighted() {
        let mut digraph = <[Vec<(usize, i32)>; 4]>::empty();

        setup_weighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn arr_btree_set_weighted() {
        let mut digraph = <[BTreeSet<(usize, i32)>; 4]>::empty();

        setup_weighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn arr_hash_set_weighted() {
        let mut digraph = <[HashSet<(usize, i32)>; 4]>::empty();

        setup_weighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn arr_btree_map() {
        let mut digraph = <[BTreeMap<usize, i32>; 4]>::empty();

        setup_weighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn arr_hash_map() {
        let mut digraph = <[HashMap<usize, i32>; 4]>::empty();

        setup_weighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn btree_map_vec_weighted() {
        let mut digraph = BTreeMap::<usize, Vec<(usize, i32)>>::empty(4);

        setup_weighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn btree_map_btree_set_weighted() {
        let mut digraph = BTreeMap::<usize, BTreeSet<(usize, i32)>>::empty(4);

        setup_weighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn btree_map_btree_map() {
        let mut digraph = BTreeMap::<usize, BTreeMap<usize, i32>>::empty(4);

        setup_weighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn hash_map_vec_weighted() {
        let mut digraph = HashMap::<usize, Vec<(usize, i32)>>::empty(4);

        setup_weighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn hash_map_hash_set_weighted() {
        let mut digraph = HashMap::<usize, HashSet<(usize, i32)>>::empty(4);

        setup_weighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn hash_map_hash_map() {
        let mut digraph = HashMap::<usize, HashMap<usize, i32>>::empty(4);

        setup_weighted!(digraph);
        test_unstable!(digraph);
    }
}

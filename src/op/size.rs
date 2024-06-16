//! Count the arcs in a digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::op::Size;
//!
//! let digraph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
//!
//! assert_eq!(digraph.size(), 10);
//! ```
//!
//! # Benchmarks
//!
//! To run the benchmarks for this operation, use:
//!
//! ```sh
//! cargo bench --bench size
//! ```

use {
    core::hash::BuildHasher,
    std::collections::{
        BTreeMap,
        BTreeSet,
        HashMap,
        HashSet,
    },
};

/// Count the arcs in a digraph.
///
/// # How can I implement `Size`?
///
/// Provide an implementation of `size` that returns the number of
/// arcs in the digraph.
///
/// ```
/// use graaf::op::Size;
///
/// struct Digraph {
///     arcs: Vec<Vec<usize>>,
/// }
///
/// impl Size for Digraph {
///     fn size(&self) -> usize {
///         self.arcs.iter().map(Vec::len).sum()
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::Size;
///
/// let digraph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
///
/// assert_eq!(digraph.size(), 10);
/// ```
pub trait Size {
    /// Counts the arcs in the digraph.
    fn size(&self) -> usize;
}

macro_rules! impl_iter_sum {
    ($ty:ident) => {
        fn size(&self) -> usize {
            self.iter().map($ty::len).sum()
        }
    };
}

macro_rules! impl_values_sum {
    ($ty:ident) => {
        fn size(&self) -> usize {
            self.values().map($ty::len).sum()
        }
    };
}

macro_rules! impl_len {
    () => {
        fn size(&self) -> usize {
            self.len()
        }
    };
}

impl Size for Vec<Vec<usize>> {
    impl_iter_sum!(Vec);
}

impl Size for Vec<BTreeSet<usize>> {
    impl_iter_sum!(BTreeSet);
}

impl<H> Size for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    impl_iter_sum!(HashSet);
}

impl<W> Size for Vec<BTreeMap<usize, W>> {
    impl_iter_sum!(BTreeMap);
}

impl<W, H> Size for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    impl_iter_sum!(HashMap);
}

impl Size for [Vec<usize>] {
    impl_iter_sum!(Vec);
}

impl Size for [BTreeSet<usize>] {
    impl_iter_sum!(BTreeSet);
}

impl<H> Size for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    impl_iter_sum!(HashSet);
}

impl<W> Size for [BTreeMap<usize, W>] {
    impl_iter_sum!(BTreeMap);
}

impl<W, H> Size for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    impl_iter_sum!(HashMap);
}

impl<const V: usize> Size for [Vec<usize>; V] {
    impl_iter_sum!(Vec);
}

impl<const V: usize> Size for [BTreeSet<usize>; V] {
    impl_iter_sum!(BTreeSet);
}

impl<const V: usize, W> Size for [BTreeMap<usize, W>; V] {
    impl_iter_sum!(BTreeMap);
}

impl<const V: usize, H> Size for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    impl_iter_sum!(HashSet);
}

impl<const V: usize, W, H> Size for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    impl_iter_sum!(HashMap);
}

impl Size for BTreeMap<usize, Vec<usize>> {
    impl_values_sum!(Vec);
}

impl Size for BTreeMap<usize, BTreeSet<usize>> {
    impl_values_sum!(BTreeSet);
}

impl<W> Size for BTreeMap<usize, BTreeMap<usize, W>> {
    impl_values_sum!(BTreeMap);
}

impl<H> Size for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    impl_values_sum!(Vec);
}

impl<H> Size for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    impl_values_sum!(HashSet);
}

impl<W, H> Size for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    impl_values_sum!(HashMap);
}

impl Size for Vec<(usize, usize)> {
    impl_len!();
}

impl<W> Size for Vec<(usize, usize, W)> {
    impl_len!();
}

impl Size for [(usize, usize)] {
    impl_len!();
}

impl<W> Size for [(usize, usize, W)] {
    impl_len!();
}

impl Size for BTreeSet<(usize, usize)> {
    impl_len!();
}

impl<W> Size for BTreeSet<(usize, usize, W)> {
    impl_len!();
}

impl<H> Size for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    impl_len!();
}

impl<W, H> Size for HashSet<(usize, usize, W), H>
where
    H: BuildHasher,
{
    impl_len!();
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
            $digraph.add_arc(2, 1);
        };
    }

    macro_rules! setup_weighted {
        ($digraph:expr) => {
            $digraph.add_weighted_arc(0, 1, 2);
            $digraph.add_weighted_arc(0, 2, 3);
            $digraph.add_weighted_arc(1, 0, 4);
            $digraph.add_weighted_arc(2, 1, 5);
        };
    }

    #[test]
    fn vec_vec() {
        let mut digraph = Vec::<Vec<usize>>::empty(4);

        setup_unweighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn vec_btree_set() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(4);

        setup_unweighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn vec_hash_set() {
        let mut digraph = Vec::<HashSet<usize>>::empty(4);

        setup_unweighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn vec_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(4);

        setup_weighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn vec_hash_map() {
        let mut digraph = Vec::<HashMap<usize, usize>>::empty(4);

        setup_weighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn slice_vec() {
        let mut digraph = Vec::<Vec<usize>>::empty(4);

        setup_unweighted!(digraph);
        assert_eq!(digraph.as_mut_slice().size(), 4);
    }

    #[test]
    fn slice_btree_set() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(4);

        setup_unweighted!(digraph);
        assert_eq!(digraph.as_mut_slice().size(), 4);
    }

    #[test]
    fn slice_hash_set() {
        let mut digraph = Vec::<HashSet<usize>>::empty(4);

        setup_unweighted!(digraph);
        assert_eq!(digraph.as_mut_slice().size(), 4);
    }

    #[test]
    fn slice_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(4);

        setup_weighted!(digraph);
        assert_eq!(digraph.as_mut_slice().size(), 4);
    }

    #[test]
    fn slice_hash_map() {
        let mut digraph = Vec::<HashMap<usize, usize>>::empty(4);

        setup_weighted!(digraph);
        assert_eq!(digraph.as_mut_slice().size(), 4);
    }

    #[test]
    fn arr_vec() {
        let mut digraph = <[Vec<usize>; 4]>::empty();

        setup_unweighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn arr_btree_set() {
        let mut digraph = <[BTreeSet<usize>; 4]>::empty();

        setup_unweighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn arr_hash_set() {
        let mut digraph = <[HashSet<usize>; 4]>::empty();

        setup_unweighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn arr_btree_map() {
        let mut digraph = <[BTreeMap<usize, usize>; 4]>::empty();

        setup_weighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn arr_hash_map() {
        let mut digraph = <[HashMap<usize, usize>; 4]>::empty();

        setup_weighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn btree_map_vec() {
        let mut digraph = BTreeMap::<usize, Vec<usize>>::empty(4);

        setup_unweighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn btree_map_btree_set() {
        let mut digraph = BTreeMap::<usize, BTreeSet<usize>>::empty(4);

        setup_unweighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn btree_map_btree_map() {
        let mut digraph = BTreeMap::<usize, BTreeMap<usize, usize>>::empty(4);

        setup_weighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn hash_map_vec() {
        let mut digraph = HashMap::<usize, Vec<usize>>::empty(4);

        setup_unweighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn hash_map_hash_set() {
        let mut digraph = HashMap::<usize, HashSet<usize>>::empty(4);

        setup_unweighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn hash_map_hash_map() {
        let mut digraph = HashMap::<usize, HashMap<usize, usize>>::empty(4);

        setup_weighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn vec_tuple_unweighted() {
        let mut digraph = Vec::<(usize, usize)>::empty();

        setup_unweighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn vec_tuple_weighted() {
        let mut digraph = Vec::<(usize, usize, usize)>::empty();

        setup_weighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn slice_tuple_unweighted() {
        let mut digraph = Vec::<(usize, usize)>::empty();

        setup_unweighted!(digraph);
        assert_eq!(digraph.as_slice().size(), 4);
    }

    #[test]
    fn slice_tuple_weighted() {
        let mut digraph = Vec::<(usize, usize, usize)>::empty();

        setup_weighted!(digraph);
        assert_eq!(digraph.as_slice().size(), 4);
    }

    #[test]
    fn btree_set_tuple_unweighted() {
        let mut digraph = BTreeSet::<(usize, usize)>::empty();

        setup_unweighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn btree_set_tuple_weighted() {
        let mut digraph = BTreeSet::<(usize, usize, usize)>::empty();

        setup_weighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn hash_set_tuple_unweighted() {
        let mut digraph = HashSet::<(usize, usize)>::empty();

        setup_unweighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn hash_set_tuple_weighted() {
        let mut digraph = HashSet::<(usize, usize, usize)>::empty();

        setup_weighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }
}

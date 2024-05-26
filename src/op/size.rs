//! Count the number of arcs in a digraph
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

/// Count the number of arcs in a digraph
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
    /// Counts all arcs.
    fn size(&self) -> usize;
}

impl<T> Size for Vec<Vec<T>> {
    fn size(&self) -> usize {
        self.iter().map(Vec::len).sum()
    }
}

impl<T> Size for Vec<BTreeSet<T>> {
    fn size(&self) -> usize {
        self.iter().map(BTreeSet::len).sum()
    }
}

impl<T, H> Size for Vec<HashSet<T, H>>
where
    H: BuildHasher,
{
    fn size(&self) -> usize {
        self.iter().map(HashSet::len).sum()
    }
}

impl<K, W> Size for Vec<BTreeMap<K, W>> {
    fn size(&self) -> usize {
        self.iter().map(BTreeMap::len).sum()
    }
}

impl<K, W, H> Size for Vec<HashMap<K, W, H>>
where
    H: BuildHasher,
{
    fn size(&self) -> usize {
        self.iter().map(HashMap::len).sum()
    }
}

impl<T> Size for [Vec<T>] {
    fn size(&self) -> usize {
        self.iter().map(Vec::len).sum()
    }
}

impl<T> Size for [BTreeSet<T>] {
    fn size(&self) -> usize {
        self.iter().map(BTreeSet::len).sum()
    }
}

impl<T, H> Size for [HashSet<T, H>]
where
    H: BuildHasher,
{
    fn size(&self) -> usize {
        self.iter().map(HashSet::len).sum()
    }
}

impl<K, W> Size for [BTreeMap<K, W>] {
    fn size(&self) -> usize {
        self.iter().map(BTreeMap::len).sum()
    }
}

impl<K, W, H> Size for [HashMap<K, W, H>]
where
    H: BuildHasher,
{
    fn size(&self) -> usize {
        self.iter().map(HashMap::len).sum()
    }
}

impl<const V: usize, T> Size for [Vec<T>; V] {
    fn size(&self) -> usize {
        self.iter().map(Vec::len).sum()
    }
}

impl<const V: usize, T> Size for [BTreeSet<T>; V] {
    fn size(&self) -> usize {
        self.iter().map(BTreeSet::len).sum()
    }
}

impl<const V: usize, K, W> Size for [BTreeMap<K, W>; V] {
    fn size(&self) -> usize {
        self.iter().map(BTreeMap::len).sum()
    }
}

impl<const V: usize, T, H> Size for [HashSet<T, H>; V]
where
    H: BuildHasher,
{
    fn size(&self) -> usize {
        self.iter().map(HashSet::len).sum()
    }
}

impl<const V: usize, K, W, H> Size for [HashMap<K, W, H>; V]
where
    H: BuildHasher,
{
    fn size(&self) -> usize {
        self.iter().map(HashMap::len).sum()
    }
}

impl<K, T> Size for BTreeMap<K, Vec<T>> {
    fn size(&self) -> usize {
        self.values().map(Vec::len).sum()
    }
}

impl<K, T> Size for BTreeMap<K, BTreeSet<T>> {
    fn size(&self) -> usize {
        self.values().map(BTreeSet::len).sum()
    }
}

impl<K, W> Size for BTreeMap<K, BTreeMap<K, W>> {
    fn size(&self) -> usize {
        self.values().map(BTreeMap::len).sum()
    }
}

impl<K, T, H> Size for HashMap<K, Vec<T>, H>
where
    H: BuildHasher,
{
    fn size(&self) -> usize {
        self.values().map(Vec::len).sum()
    }
}

impl<K, T, H> Size for HashMap<K, HashSet<T, H>, H>
where
    H: BuildHasher,
{
    fn size(&self) -> usize {
        self.values().map(HashSet::len).sum()
    }
}

impl<K, W, H> Size for HashMap<K, HashMap<K, W, H>, H>
where
    H: BuildHasher,
{
    fn size(&self) -> usize {
        self.values().map(HashMap::len).sum()
    }
}

impl Size for Vec<(usize, usize)> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<W> Size for Vec<(usize, usize, W)> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl Size for [(usize, usize)] {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<W> Size for [(usize, usize, W)] {
    fn size(&self) -> usize {
        self.len()
    }
}

impl Size for BTreeSet<(usize, usize)> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<W> Size for BTreeSet<(usize, usize, W)> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<H> Size for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    fn size(&self) -> usize {
        self.len()
    }
}

impl<W, H> Size for HashSet<(usize, usize, W), H>
where
    H: BuildHasher,
{
    fn size(&self) -> usize {
        self.len()
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
        let mut digraph = Vec::<(usize, usize)>::empty(4);

        setup_unweighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn vec_tuple_weighted() {
        let mut digraph = Vec::<(usize, usize, usize)>::empty(4);

        setup_weighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn slice_tuple_unweighted() {
        let mut digraph = Vec::<(usize, usize)>::empty(4);

        setup_unweighted!(digraph);
        assert_eq!(digraph.as_slice().size(), 4);
    }

    #[test]
    fn slice_tuple_weighted() {
        let mut digraph = Vec::<(usize, usize, usize)>::empty(4);

        setup_weighted!(digraph);
        assert_eq!(digraph.as_slice().size(), 4);
    }

    #[test]
    fn btree_set_tuple_unweighted() {
        let mut digraph = BTreeSet::<(usize, usize)>::empty(4);

        setup_unweighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn btree_set_tuple_weighted() {
        let mut digraph = BTreeSet::<(usize, usize, usize)>::empty(4);

        setup_weighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn hash_set_tuple_unweighted() {
        let mut digraph = HashSet::<(usize, usize)>::empty(4);

        setup_unweighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }

    #[test]
    fn hash_set_tuple_weighted() {
        let mut digraph = HashSet::<(usize, usize, usize)>::empty(4);

        setup_weighted!(digraph);
        assert_eq!(digraph.size(), 4);
    }
}

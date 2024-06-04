#![doc(alias = "out_degree")]
//! Get the outdegree of a vertex in a digraph.
//!
//! The outdegree is the number of arcs incident out of a vertex.
//!
//! # Examples
//!
//! ```
//! use graaf::op::Outdegree;
//!
//! let digraph = vec![vec![1, 2], vec![0], vec![1]];
//!
//! assert_eq!(digraph.outdegree(0), 2);
//! assert_eq!(digraph.outdegree(1), 1);
//! assert_eq!(digraph.outdegree(2), 1);
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

/// Get the outdegree of a vertex in a digraph.
///
/// # How can I implement `Outdegree`?
///
/// Provide an implementation of `outdegree` that returns the outdegree of the
/// target vertex. The implementation should not panic if the vertex is not in
/// the digraph.
///
/// ```
/// use graaf::op::Outdegree;
///
/// struct Digraph {
///     arcs: Vec<Vec<usize>>,
/// }
///
/// impl Outdegree for Digraph {
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
/// let digraph = vec![vec![1, 2], vec![0], vec![1]];
///
/// assert_eq!(digraph.outdegree(0), 2);
/// assert_eq!(digraph.outdegree(1), 1);
/// assert_eq!(digraph.outdegree(2), 1);
/// assert_eq!(digraph.outdegree(3), 0);
/// ```
pub trait Outdegree {
    /// Returns the outdegree of a vertex in the digraph.
    ///
    /// # Arguments
    ///
    /// * `s`: The head vertex.
    fn outdegree(&self, s: usize) -> usize;

    /// Returns whether a vertex is a sink of the digraph.
    ///
    /// A sink is a vertex with an outdegree of 0.
    ///
    /// # Arguments
    ///
    /// * `s`: The vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::op::Outdegree;
    ///
    /// let digraph = vec![vec![1, 2], vec![0], Vec::new()];
    ///
    /// assert!(!digraph.is_sink(0));
    /// assert!(!digraph.is_sink(1));
    /// assert!(digraph.is_sink(2));
    /// ```
    fn is_sink(&self, s: usize) -> bool {
        self.outdegree(s) == 0
    }
}

macro_rules! impl_len {
    ($ty:ident) => {
        fn outdegree(&self, s: usize) -> usize {
            self.get(s).map_or(0, $ty::len)
        }
    };
}

macro_rules! impl_map_len {
    ($ty:ident) => {
        fn outdegree(&self, s: usize) -> usize {
            self.get(&s).map_or(0, $ty::len)
        }
    };
}

impl Outdegree for Vec<Vec<usize>> {
    impl_len!(Vec);
}

impl<W> Outdegree for Vec<Vec<(usize, W)>> {
    impl_len!(Vec);
}

impl Outdegree for Vec<BTreeSet<usize>> {
    impl_len!(BTreeSet);
}

impl<W> Outdegree for Vec<BTreeSet<(usize, W)>> {
    impl_len!(BTreeSet);
}

impl<H> Outdegree for Vec<HashSet<usize, H>> {
    impl_len!(HashSet);
}

impl<W, H> Outdegree for Vec<HashSet<(usize, W), H>> {
    impl_len!(HashSet);
}

impl<W> Outdegree for Vec<BTreeMap<usize, W>> {
    impl_len!(BTreeMap);
}

impl<W, H> Outdegree for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    impl_len!(HashMap);
}

impl Outdegree for [Vec<usize>] {
    impl_len!(Vec);
}

impl<W> Outdegree for [Vec<(usize, W)>] {
    impl_len!(Vec);
}

impl Outdegree for [BTreeSet<usize>] {
    impl_len!(BTreeSet);
}

impl<W> Outdegree for [BTreeSet<(usize, W)>] {
    impl_len!(BTreeSet);
}

impl<H> Outdegree for [HashSet<usize, H>] {
    impl_len!(HashSet);
}

impl<W, H> Outdegree for [HashSet<(usize, W), H>] {
    impl_len!(HashSet);
}

impl<W> Outdegree for [BTreeMap<usize, W>] {
    impl_len!(BTreeMap);
}

impl<W, H> Outdegree for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    impl_len!(HashMap);
}

impl<const V: usize> Outdegree for [Vec<usize>; V] {
    impl_len!(Vec);
}

impl<const V: usize, W> Outdegree for [Vec<(usize, W)>; V] {
    impl_len!(Vec);
}

impl<const V: usize> Outdegree for [BTreeSet<usize>; V] {
    impl_len!(BTreeSet);
}

impl<const V: usize, W> Outdegree for [BTreeSet<(usize, W)>; V] {
    impl_len!(BTreeSet);
}

impl<const V: usize, H> Outdegree for [HashSet<usize, H>; V] {
    impl_len!(HashSet);
}

impl<const V: usize, W, H> Outdegree for [HashSet<(usize, W), H>; V] {
    impl_len!(HashSet);
}

impl<const V: usize, W> Outdegree for [BTreeMap<usize, W>; V] {
    impl_len!(BTreeMap);
}

impl<const V: usize, W, H> Outdegree for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    impl_len!(HashMap);
}

impl Outdegree for BTreeMap<usize, Vec<usize>> {
    impl_map_len!(Vec);
}

impl<H> Outdegree for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    impl_map_len!(Vec);
}

impl Outdegree for BTreeMap<usize, BTreeSet<usize>> {
    impl_map_len!(BTreeSet);
}

impl<H> Outdegree for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    impl_map_len!(HashSet);
}

impl<W> Outdegree for BTreeMap<usize, BTreeMap<usize, W>> {
    impl_map_len!(BTreeMap);
}

impl<W, H> Outdegree for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    impl_map_len!(HashMap);
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
        ($digraph:expr) => {
            assert_eq!($digraph.outdegree(0), 2);
            assert_eq!($digraph.outdegree(1), 1);
            assert_eq!($digraph.outdegree(2), 1);
            assert_eq!($digraph.outdegree(3), 0);
        };
    }

    macro_rules! test_outdegree_unweighted {
        ($digraph:expr) => {
            $digraph.add_arc(0, 1);
            $digraph.add_arc(0, 2);
            $digraph.add_arc(1, 0);
            $digraph.add_arc(2, 1);

            test_outdegree!($digraph);
        };
    }

    macro_rules! test_outdegree_weighted {
        ($digraph:expr) => {
            $digraph.add_weighted_arc(0, 1, 1);
            $digraph.add_weighted_arc(0, 2, 2);
            $digraph.add_weighted_arc(1, 0, 3);
            $digraph.add_weighted_arc(2, 1, 4);

            test_outdegree!($digraph);
        };
    }

    macro_rules! test_is_sink {
        ($digraph:expr) => {
            assert!(!$digraph.is_sink(0));
            assert!($digraph.is_sink(1));
            assert!(!$digraph.is_sink(2));
            assert!($digraph.is_sink(3));
        };
    }

    macro_rules! test_is_sink_unweighted {
        ($digraph:expr) => {
            $digraph.add_arc(0, 1);
            $digraph.add_arc(0, 2);
            $digraph.add_arc(2, 1);

            test_is_sink!($digraph);
        };
    }

    macro_rules! test_is_sink_weighted {
        ($digraph:expr) => {
            $digraph.add_weighted_arc(0, 1, 1);
            $digraph.add_weighted_arc(0, 2, 2);
            $digraph.add_weighted_arc(2, 1, 4);

            test_is_sink!($digraph);
        };
    }

    #[test]
    fn vec_vec_unweighted() {
        let digraph = &mut Vec::<Vec<usize>>::empty(4);

        test_outdegree_unweighted!(digraph);
    }

    #[test]
    fn vec_btree_set_unweighted() {
        let digraph = &mut Vec::<BTreeSet<usize>>::empty(4);

        test_outdegree_unweighted!(digraph);
    }

    #[test]
    fn vec_hash_set_unweighted() {
        let digraph = &mut Vec::<HashSet<usize>>::empty(4);

        test_outdegree_unweighted!(digraph);
    }

    #[test]
    fn slice_vec_unweighted() {
        let digraph: &mut [Vec<usize>] = &mut Vec::<Vec<usize>>::empty(4);

        test_outdegree_unweighted!(digraph);
    }

    #[test]
    fn slice_btree_set_unweighted() {
        let digraph: &mut [BTreeSet<usize>] = &mut Vec::<BTreeSet<usize>>::empty(4);

        test_outdegree_unweighted!(digraph);
    }

    #[test]
    fn slice_hash_set_unweighted() {
        let digraph: &mut [HashSet<usize>] = &mut Vec::<HashSet<usize>>::empty(4);

        test_outdegree_unweighted!(digraph);
    }

    #[test]
    fn arr_vec_unweighted() {
        let digraph = &mut <[Vec<usize>; 4]>::empty();

        test_outdegree_unweighted!(digraph);
    }

    #[test]
    fn arr_btree_set_unweighted() {
        let digraph = &mut <[BTreeSet<usize>; 4]>::empty();

        test_outdegree_unweighted!(digraph);
    }

    #[test]
    fn arr_hash_set_unweighted() {
        let digraph = &mut <[HashSet<usize>; 4]>::empty();

        test_outdegree_unweighted!(digraph);
    }

    #[test]
    fn btree_map_vec() {
        let digraph = &mut BTreeMap::<usize, Vec<usize>>::empty(4);

        test_outdegree_unweighted!(digraph);
    }

    #[test]
    fn hash_map_vec() {
        let digraph = &mut HashMap::<usize, Vec<usize>>::empty(4);

        test_outdegree_unweighted!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let digraph = &mut BTreeMap::<usize, BTreeSet<usize>>::empty(4);

        test_outdegree_unweighted!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let digraph = &mut HashMap::<usize, HashSet<usize>>::empty(4);

        test_outdegree_unweighted!(digraph);
    }

    #[test]
    fn vec_vec_weighted() {
        let digraph = &mut Vec::<Vec<(usize, i32)>>::empty(4);

        test_outdegree_weighted!(digraph);
    }

    #[test]
    fn vec_btree_set_weighted() {
        let digraph = &mut Vec::<BTreeSet<(usize, i32)>>::empty(4);

        test_outdegree_weighted!(digraph);
    }

    #[test]
    fn vec_hash_set_weighted() {
        let digraph = &mut Vec::<HashSet<(usize, i32)>>::empty(4);

        test_outdegree_weighted!(digraph);
    }

    #[test]
    fn vec_btree_map() {
        let digraph = &mut Vec::<BTreeMap<usize, i32>>::empty(4);

        test_outdegree_weighted!(digraph);
    }

    #[test]
    fn vec_hash_map() {
        let digraph = &mut Vec::<HashMap<usize, i32>>::empty(4);

        test_outdegree_weighted!(digraph);
    }

    #[test]
    fn slice_vec_weighted() {
        let digraph: &mut [Vec<(usize, i32)>] = &mut Vec::<Vec<(usize, i32)>>::empty(4);

        test_outdegree_weighted!(digraph);
    }

    #[test]
    fn slice_btree_set_weighted() {
        let digraph: &mut [BTreeSet<(usize, i32)>] = &mut Vec::<BTreeSet<(usize, i32)>>::empty(4);

        test_outdegree_weighted!(digraph);
    }

    #[test]
    fn slice_hash_set_weighted() {
        let digraph: &mut [HashSet<(usize, i32)>] = &mut Vec::<HashSet<(usize, i32)>>::empty(4);

        test_outdegree_weighted!(digraph);
    }

    #[test]
    fn slice_btree_map() {
        let digraph: &mut [BTreeMap<usize, i32>] = &mut Vec::<BTreeMap<usize, i32>>::empty(4);

        test_outdegree_weighted!(digraph);
    }

    #[test]
    fn slice_hash_map() {
        let digraph: &mut [HashMap<usize, i32>] = &mut Vec::<HashMap<usize, i32>>::empty(4);

        test_outdegree_weighted!(digraph);
    }

    #[test]
    fn arr_vec_weighted() {
        let digraph = &mut <[Vec<(usize, i32)>; 4]>::empty();

        test_outdegree_weighted!(digraph);
    }

    #[test]
    fn arr_btree_set_weighted() {
        let digraph = &mut <[BTreeSet<(usize, i32)>; 4]>::empty();

        test_outdegree_weighted!(digraph);
    }

    #[test]
    fn arr_hash_set_weighted() {
        let digraph = &mut <[HashSet<(usize, i32)>; 4]>::empty();

        test_outdegree_weighted!(digraph);
    }

    #[test]
    fn arr_btree_map() {
        let digraph = &mut <[BTreeMap<usize, i32>; 4]>::empty();

        test_outdegree_weighted!(digraph);
    }

    #[test]
    fn arr_hash_map() {
        let digraph = &mut <[HashMap<usize, i32>; 4]>::empty();

        test_outdegree_weighted!(digraph);
    }

    #[test]
    fn btree_map_btree_map() {
        let digraph = &mut BTreeMap::<usize, BTreeMap<usize, i32>>::empty(4);

        test_outdegree_weighted!(digraph);
    }

    #[test]
    fn hash_map_hash_map() {
        let digraph = &mut HashMap::<usize, HashMap<usize, i32>>::empty(4);

        test_outdegree_weighted!(digraph);
    }

    #[test]
    fn is_sink_vec_vec_unweighted() {
        let digraph = &mut Vec::<Vec<usize>>::empty(4);

        test_is_sink_unweighted!(digraph);
    }

    #[test]
    fn is_sink_vec_btree_set_unweighted() {
        let digraph = &mut Vec::<BTreeSet<usize>>::empty(4);

        test_is_sink_unweighted!(digraph);
    }

    #[test]
    fn is_sink_vec_hash_set_unweighted() {
        let digraph = &mut Vec::<HashSet<usize>>::empty(4);

        test_is_sink_unweighted!(digraph);
    }

    #[test]
    fn is_sink_slice_vec_unweighted() {
        let digraph: &mut [Vec<usize>] = &mut Vec::<Vec<usize>>::empty(4);

        test_is_sink_unweighted!(digraph);
    }

    #[test]
    fn is_sink_slice_btree_set_unweighted() {
        let digraph: &mut [BTreeSet<usize>] = &mut Vec::<BTreeSet<usize>>::empty(4);

        test_is_sink_unweighted!(digraph);
    }

    #[test]
    fn is_sink_slice_hash_set_unweighted() {
        let digraph: &mut [HashSet<usize>] = &mut Vec::<HashSet<usize>>::empty(4);

        test_is_sink_unweighted!(digraph);
    }

    #[test]
    fn is_sink_arr_vec_unweighted() {
        let digraph = &mut <[Vec<usize>; 4]>::empty();

        test_is_sink_unweighted!(digraph);
    }

    #[test]
    fn is_sink_arr_btree_set_unweighted() {
        let digraph = &mut <[BTreeSet<usize>; 4]>::empty();

        test_is_sink_unweighted!(digraph);
    }

    #[test]
    fn is_sink_arr_hash_set_unweighted() {
        let digraph = &mut <[HashSet<usize>; 4]>::empty();

        test_is_sink_unweighted!(digraph);
    }

    #[test]
    fn is_sink_btree_map_vec() {
        let digraph = &mut BTreeMap::<usize, Vec<usize>>::empty(4);

        test_is_sink_unweighted!(digraph);
    }

    #[test]
    fn is_sink_hash_map_vec() {
        let digraph = &mut HashMap::<usize, Vec<usize>>::empty(4);

        test_is_sink_unweighted!(digraph);
    }

    #[test]
    fn is_sink_btree_map_btree_set() {
        let digraph = &mut BTreeMap::<usize, BTreeSet<usize>>::empty(4);

        test_is_sink_unweighted!(digraph);
    }

    #[test]
    fn is_sink_hash_map_hash_set() {
        let digraph = &mut HashMap::<usize, HashSet<usize>>::empty(4);

        test_is_sink_unweighted!(digraph);
    }

    #[test]
    fn is_sink_vec_vec_weighted() {
        let digraph = &mut Vec::<Vec<(usize, i32)>>::empty(4);

        test_is_sink_weighted!(digraph);
    }

    #[test]
    fn is_sink_vec_btree_set_weighted() {
        let digraph = &mut Vec::<BTreeSet<(usize, i32)>>::empty(4);

        test_is_sink_weighted!(digraph);
    }

    #[test]
    fn is_sink_vec_hash_set_weighted() {
        let digraph = &mut Vec::<HashSet<(usize, i32)>>::empty(4);

        test_is_sink_weighted!(digraph);
    }

    #[test]
    fn is_sink_vec_btree_map() {
        let digraph = &mut Vec::<BTreeMap<usize, i32>>::empty(4);

        test_is_sink_weighted!(digraph);
    }

    #[test]
    fn is_sink_vec_hash_map() {
        let digraph = &mut Vec::<HashMap<usize, i32>>::empty(4);

        test_is_sink_weighted!(digraph);
    }

    #[test]
    fn is_sink_slice_vec_weighted() {
        let digraph: &mut [Vec<(usize, i32)>] = &mut Vec::<Vec<(usize, i32)>>::empty(4);

        test_is_sink_weighted!(digraph);
    }

    #[test]
    fn is_sink_slice_btree_set_weighted() {
        let digraph: &mut [BTreeSet<(usize, i32)>] = &mut Vec::<BTreeSet<(usize, i32)>>::empty(4);

        test_is_sink_weighted!(digraph);
    }

    #[test]
    fn is_sink_slice_hash_set_weighted() {
        let digraph: &mut [HashSet<(usize, i32)>] = &mut Vec::<HashSet<(usize, i32)>>::empty(4);

        test_is_sink_weighted!(digraph);
    }

    #[test]
    fn is_sink_slice_btree_map() {
        let digraph: &mut [BTreeMap<usize, i32>] = &mut Vec::<BTreeMap<usize, i32>>::empty(4);

        test_is_sink_weighted!(digraph);
    }

    #[test]
    fn is_sink_slice_hash_map() {
        let digraph: &mut [HashMap<usize, i32>] = &mut Vec::<HashMap<usize, i32>>::empty(4);

        test_is_sink_weighted!(digraph);
    }

    #[test]
    fn is_sink_arr_vec_weighted() {
        let digraph = &mut <[Vec<(usize, i32)>; 4]>::empty();

        test_is_sink_weighted!(digraph);
    }

    #[test]
    fn is_sink_arr_btree_set_weighted() {
        let digraph = &mut <[BTreeSet<(usize, i32)>; 4]>::empty();

        test_is_sink_weighted!(digraph);
    }

    #[test]
    fn is_sink_arr_hash_set_weighted() {
        let digraph = &mut <[HashSet<(usize, i32)>; 4]>::empty();

        test_is_sink_weighted!(digraph);
    }

    #[test]
    fn is_sink_arr_btree_map() {
        let digraph = &mut <[BTreeMap<usize, i32>; 4]>::empty();

        test_is_sink_weighted!(digraph);
    }

    #[test]
    fn is_sink_arr_hash_map() {
        let digraph = &mut <[HashMap<usize, i32>; 4]>::empty();

        test_is_sink_weighted!(digraph);
    }

    #[test]
    fn is_sink_btree_map_btree_map() {
        let digraph = &mut BTreeMap::<usize, BTreeMap<usize, i32>>::empty(4);

        test_is_sink_weighted!(digraph);
    }

    #[test]
    fn is_sink_hash_map_hash_map() {
        let digraph = &mut HashMap::<usize, HashMap<usize, i32>>::empty(4);

        test_is_sink_weighted!(digraph);
    }
}

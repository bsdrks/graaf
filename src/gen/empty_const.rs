#![doc(alias = "edgeless")]
//! Generate empty constant-sized digraphs.
//!
//! Empty graphs are also known as edgeless graphs. To generate empty
//! variable-sized digraphs, see [`Empty`].
//!
//! # Examples
//!
//! ```
//! use graaf::gen::EmptyConst;
//!
//! // 0 -> {}
//!
//! assert_eq!(<[Vec::<usize>; 1]>::empty(), [Vec::new()]);
//!
//! // 0 -> {}
//! // 1 -> {}
//!
//! assert_eq!(<[Vec::<usize>; 2]>::empty(), [Vec::new(), Vec::new()]);
//!
//! // 0 -> {}
//! // 1 -> {}
//! // 2 -> {}
//!
//! assert_eq!(
//!     <[Vec::<usize>; 3]>::empty(),
//!     [Vec::new(), Vec::new(), Vec::new()]
//! );
//! ```
//!
//! [`Empty`]: crate::gen::Empty

use {
    core::{
        array::from_fn,
        hash::BuildHasher,
    },
    std::collections::{
        BTreeMap,
        BTreeSet,
        HashMap,
        HashSet,
    },
};

/// Generate empty constant-sized digraphs.
///
/// # How can I implement `EmptyConst`?
///
/// Provide an implementation of `empty` that generates an empty digraph with
/// `V` vertices.
///
/// ```
/// use {
///     core::{
///         array::from_fn,
///         hash::BuildHasher,
///     },
///     graaf::gen::EmptyConst,
///     std::collections::{
///         hash_map::RandomState,
///         HashSet,
///     },
/// };
///
/// struct Digraph<const V: usize, H>
/// where
///     H: BuildHasher,
/// {
///     arcs: [HashSet<usize, H>; V],
/// }
///
/// impl<const V: usize, H> EmptyConst for Digraph<V, H>
/// where
///     H: BuildHasher + Default,
/// {
///     /// # Panics
///     ///
///     /// Panics if `V` is zero.
///     fn empty() -> Self {
///         assert!(V > 0, "a graph must have at least one vertex");
///
///         Digraph {
///             arcs: from_fn(|_| HashSet::with_hasher(H::default())),
///         }
///     }
/// }
///
/// let digraph = Digraph::<3, RandomState>::empty();
///
/// assert_eq!(
///     digraph.arcs,
///     [HashSet::new(), HashSet::new(), HashSet::new()]
/// );
/// ```
///
/// # Examples
///
/// ```
/// use graaf::gen::EmptyConst;
///
/// // 0 -> {}
///
/// assert_eq!(<[Vec::<usize>; 1]>::empty(), [Vec::new()]);
///
/// // 0 -> {}
/// // 1 -> {}
///
/// assert_eq!(<[Vec::<usize>; 2]>::empty(), [Vec::new(), Vec::new()]);
///
/// // 0 -> {}
/// // 1 -> {}
/// // 2 -> {}
///
/// assert_eq!(
///     <[Vec::<usize>; 3]>::empty(),
///     [Vec::new(), Vec::new(), Vec::new()]
/// );
/// ```
pub trait EmptyConst {
    /// Generates an empty digraph.
    #[must_use]
    fn empty() -> Self;
}

macro_rules! impl_new {
    ($ty:ident) => {
        /// # Panics
        ///
        /// Panics if `V` is zero.
        fn empty() -> Self {
            assert!(V > 0, "a graph must have at least one vertex");

            from_fn(|_| $ty::new())
        }
    };
}

macro_rules! impl_with_hasher {
    ($ty:ident) => {
        /// # Panics
        ///
        /// Panics if `V` is zero.
        fn empty() -> Self {
            assert!(V > 0, "a graph must have at least one vertex");

            from_fn(|_| $ty::with_hasher(H::default()))
        }
    };
}

impl<const V: usize> EmptyConst for [Vec<usize>; V] {
    impl_new!(Vec);
}

impl<const V: usize> EmptyConst for [BTreeSet<usize>; V] {
    impl_new!(BTreeSet);
}

impl<const V: usize, H> EmptyConst for [HashSet<usize, H>; V]
where
    H: BuildHasher + Default,
{
    impl_with_hasher!(HashSet);
}

impl<const V: usize, W> EmptyConst for [Vec<(usize, W)>; V] {
    impl_new!(Vec);
}

impl<const V: usize, W> EmptyConst for [BTreeSet<(usize, W)>; V] {
    impl_new!(BTreeSet);
}

impl<const V: usize, W, H> EmptyConst for [HashSet<(usize, W), H>; V]
where
    H: BuildHasher + Default,
{
    impl_with_hasher!(HashSet);
}

impl<const V: usize, W> EmptyConst for [BTreeMap<usize, W>; V] {
    impl_new!(BTreeMap);
}

impl<const V: usize, W, H> EmptyConst for [HashMap<usize, W, H>; V]
where
    H: BuildHasher + Default,
{
    impl_with_hasher!(HashMap);
}

impl EmptyConst for Vec<(usize, usize)> {
    fn empty() -> Self {
        Self::new()
    }
}

impl EmptyConst for BTreeSet<(usize, usize)> {
    fn empty() -> Self {
        Self::new()
    }
}

impl<H> EmptyConst for HashSet<(usize, usize), H>
where
    H: BuildHasher + Default,
{
    fn empty() -> Self {
        Self::with_hasher(H::default())
    }
}

impl<W> EmptyConst for Vec<(usize, usize, W)> {
    fn empty() -> Self {
        Self::new()
    }
}

impl<W> EmptyConst for BTreeSet<(usize, usize, W)> {
    fn empty() -> Self {
        Self::new()
    }
}

impl<W, H> EmptyConst for HashSet<(usize, usize, W), H>
where
    H: BuildHasher + Default,
{
    fn empty() -> Self {
        Self::with_hasher(H::default())
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::op::{
            Order,
            Size,
        },
    };

    #[test]
    fn arr_vec_unweighted() {
        assert_eq!(<[Vec::<usize>; 1]>::empty(), [Vec::new()]);
        assert_eq!(<[Vec::<usize>; 2]>::empty(), [Vec::new(), Vec::new()]);

        assert_eq!(
            <[Vec::<usize>; 3]>::empty(),
            [Vec::new(), Vec::new(), Vec::new()]
        );
    }

    #[test]
    fn arr_btree_set_unweighted() {
        assert_eq!(<[BTreeSet::<usize>; 1]>::empty(), [BTreeSet::new()]);

        assert_eq!(
            <[BTreeSet::<usize>; 2]>::empty(),
            [BTreeSet::new(), BTreeSet::new()]
        );

        assert_eq!(
            <[BTreeSet::<usize>; 3]>::empty(),
            [BTreeSet::new(), BTreeSet::new(), BTreeSet::new()]
        );
    }

    #[test]
    fn arr_hash_set_unweighted() {
        assert_eq!(<[HashSet::<usize>; 1]>::empty(), [HashSet::new()]);

        assert_eq!(
            <[HashSet::<usize>; 2]>::empty(),
            [HashSet::new(), HashSet::new()]
        );

        assert_eq!(
            <[HashSet::<usize>; 3]>::empty(),
            [HashSet::new(), HashSet::new(), HashSet::new()]
        );
    }

    #[test]
    fn arr_vec_weighted() {
        assert_eq!(<[Vec<(usize, usize)>; 1]>::empty(), [Vec::new()]);

        assert_eq!(
            <[Vec<(usize, usize)>; 2]>::empty(),
            [Vec::new(), Vec::new()]
        );

        assert_eq!(
            <[Vec<(usize, usize)>; 3]>::empty(),
            [Vec::new(), Vec::new(), Vec::new()]
        );
    }

    #[test]
    fn arr_btree_set_weighted() {
        assert_eq!(<[BTreeSet<(usize, usize)>; 1]>::empty(), [BTreeSet::new()]);

        assert_eq!(
            <[BTreeSet<(usize, usize)>; 2]>::empty(),
            [BTreeSet::new(), BTreeSet::new()]
        );

        assert_eq!(
            <[BTreeSet<(usize, usize)>; 3]>::empty(),
            [BTreeSet::new(), BTreeSet::new(), BTreeSet::new()]
        );
    }

    #[test]
    fn arr_hash_set_weighted() {
        assert_eq!(<[HashSet<(usize, usize)>; 1]>::empty(), [HashSet::new()]);

        assert_eq!(
            <[HashSet<(usize, usize)>; 2]>::empty(),
            [HashSet::new(), HashSet::new()]
        );

        assert_eq!(
            <[HashSet<(usize, usize)>; 3]>::empty(),
            [HashSet::new(), HashSet::new(), HashSet::new()]
        );
    }

    #[test]
    fn arr_btree_map() {
        assert_eq!(<[BTreeMap<usize, usize>; 1]>::empty(), [BTreeMap::new()]);

        assert_eq!(
            <[BTreeMap<usize, usize>; 2]>::empty(),
            [BTreeMap::new(), BTreeMap::new()]
        );

        assert_eq!(
            <[BTreeMap<usize, usize>; 3]>::empty(),
            [BTreeMap::new(), BTreeMap::new(), BTreeMap::new()]
        );
    }

    #[test]
    fn arr_hash_map() {
        assert_eq!(<[HashMap<usize, usize>; 1]>::empty(), [HashMap::new()]);

        assert_eq!(
            <[HashMap<usize, usize>; 2]>::empty(),
            [HashMap::new(), HashMap::new()]
        );

        assert_eq!(
            <[HashMap<usize, usize>; 3]>::empty(),
            [HashMap::new(), HashMap::new(), HashMap::new()]
        );
    }

    #[test]
    fn size_arr_vec_unweighted() {
        assert_eq!(<[Vec<usize>; 1]>::empty().size(), 0);
        assert_eq!(<[Vec<usize>; 2]>::empty().size(), 0);
        assert_eq!(<[Vec<usize>; 3]>::empty().size(), 0);
    }

    #[test]
    fn size_arr_btree_set_unweighted() {
        assert_eq!(<[BTreeSet<usize>; 1]>::empty().size(), 0);
        assert_eq!(<[BTreeSet<usize>; 2]>::empty().size(), 0);
        assert_eq!(<[BTreeSet<usize>; 3]>::empty().size(), 0);
    }

    #[test]
    fn size_arr_hash_set_unweighted() {
        assert_eq!(<[HashSet<usize>; 1]>::empty().size(), 0);
        assert_eq!(<[HashSet<usize>; 2]>::empty().size(), 0);
        assert_eq!(<[HashSet<usize>; 3]>::empty().size(), 0);
    }

    #[test]
    fn size_arr_vec_weighted() {
        assert_eq!(<[Vec<(usize, usize)>; 1]>::empty().size(), 0);
        assert_eq!(<[Vec<(usize, usize)>; 2]>::empty().size(), 0);
        assert_eq!(<[Vec<(usize, usize)>; 3]>::empty().size(), 0);
    }

    #[test]
    fn size_arr_btree_set_weighted() {
        assert_eq!(<[BTreeSet<(usize, usize)>; 1]>::empty().size(), 0);
        assert_eq!(<[BTreeSet<(usize, usize)>; 2]>::empty().size(), 0);
        assert_eq!(<[BTreeSet<(usize, usize)>; 3]>::empty().size(), 0);
    }

    #[test]
    fn size_arr_hash_set_weighted() {
        assert_eq!(<[HashSet<(usize, usize)>; 1]>::empty().size(), 0);
        assert_eq!(<[HashSet<(usize, usize)>; 2]>::empty().size(), 0);
        assert_eq!(<[HashSet<(usize, usize)>; 3]>::empty().size(), 0);
    }

    #[test]
    fn size_arr_btree_map() {
        assert_eq!(<[BTreeMap<usize, usize>; 1]>::empty().size(), 0);
        assert_eq!(<[BTreeMap<usize, usize>; 2]>::empty().size(), 0);
        assert_eq!(<[BTreeMap<usize, usize>; 3]>::empty().size(), 0);
    }

    #[test]
    fn size_arr_hash_map() {
        assert_eq!(<[HashMap<usize, usize>; 1]>::empty().size(), 0);
        assert_eq!(<[HashMap<usize, usize>; 2]>::empty().size(), 0);
        assert_eq!(<[HashMap<usize, usize>; 3]>::empty().size(), 0);
    }

    #[test]
    fn order_arr_vec_unweighted() {
        assert_eq!(<[Vec<usize>; 1]>::empty().order(), 1);
        assert_eq!(<[Vec<usize>; 2]>::empty().order(), 2);
        assert_eq!(<[Vec<usize>; 3]>::empty().order(), 3);
    }

    #[test]
    fn order_arr_btree_set_unweighted() {
        assert_eq!(<[BTreeSet<usize>; 1]>::empty().order(), 1);
        assert_eq!(<[BTreeSet<usize>; 2]>::empty().order(), 2);
        assert_eq!(<[BTreeSet<usize>; 3]>::empty().order(), 3);
    }

    #[test]
    fn order_arr_hash_set_unweighted() {
        assert_eq!(<[HashSet<usize>; 1]>::empty().order(), 1);
        assert_eq!(<[HashSet<usize>; 2]>::empty().order(), 2);
        assert_eq!(<[HashSet<usize>; 3]>::empty().order(), 3);
    }

    #[test]
    fn order_arr_arr_weighted() {
        assert_eq!(<[Vec<(usize, usize)>; 1]>::empty().order(), 1);
        assert_eq!(<[Vec<(usize, usize)>; 2]>::empty().order(), 2);
        assert_eq!(<[Vec<(usize, usize)>; 3]>::empty().order(), 3);
    }

    #[test]
    fn order_arr_btree_set_weighted() {
        assert_eq!(<[BTreeSet<(usize, usize)>; 1]>::empty().order(), 1);
        assert_eq!(<[BTreeSet<(usize, usize)>; 2]>::empty().order(), 2);
        assert_eq!(<[BTreeSet<(usize, usize)>; 3]>::empty().order(), 3);
    }

    #[test]
    fn order_arr_hash_set_weighted() {
        assert_eq!(<[HashSet<(usize, usize)>; 1]>::empty().order(), 1);
        assert_eq!(<[HashSet<(usize, usize)>; 2]>::empty().order(), 2);
        assert_eq!(<[HashSet<(usize, usize)>; 3]>::empty().order(), 3);
    }

    #[test]
    fn order_arr_btree_map() {
        assert_eq!(<[BTreeMap<usize, usize>; 1]>::empty().order(), 1);
        assert_eq!(<[BTreeMap<usize, usize>; 2]>::empty().order(), 2);
        assert_eq!(<[BTreeMap<usize, usize>; 3]>::empty().order(), 3);
    }

    #[test]
    fn order_arr_hash_map() {
        assert_eq!(<[HashMap<usize, usize>; 1]>::empty().order(), 1);
        assert_eq!(<[HashMap<usize, usize>; 2]>::empty().order(), 2);
        assert_eq!(<[HashMap<usize, usize>; 3]>::empty().order(), 3);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn arr_vec_unweighted_panic() {
        let _ = <[Vec<usize>; 0]>::empty();
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn arr_btree_set_unweighted_panic() {
        let _ = <[BTreeSet<usize>; 0]>::empty();
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn arr_hash_set_unweighted_panic() {
        let _ = <[HashSet<usize>; 0]>::empty();
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn arr_vec_weighted_panic() {
        let _ = <[Vec<(usize, usize)>; 0]>::empty();
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn arr_btree_set_weighted_panic() {
        let _ = <[BTreeSet<(usize, usize)>; 0]>::empty();
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn arr_hash_set_weighted_panic() {
        let _ = <[HashSet<(usize, usize)>; 0]>::empty();
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn arr_btree_map_panic() {
        let _ = <[BTreeMap<usize, usize>; 0]>::empty();
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn arr_hash_map_panic() {
        let _ = <[HashMap<usize, usize>; 0]>::empty();
    }

    #[test]
    fn vec_tuple_unweighted() {
        assert_eq!(Vec::<(usize, usize)>::empty(), Vec::new());
    }

    #[test]
    fn btree_set_tuple_unweighted() {
        assert_eq!(BTreeSet::<(usize, usize)>::empty(), BTreeSet::new());
    }

    #[test]
    fn hash_set_tuple_unweighted() {
        assert_eq!(HashSet::<(usize, usize)>::empty(), HashSet::new());
    }

    #[test]
    fn vec_tuple_weighted() {
        assert_eq!(Vec::<(usize, usize, usize)>::empty(), Vec::new());
    }

    #[test]
    fn btree_set_tuple_weighted() {
        assert_eq!(BTreeSet::<(usize, usize, usize)>::empty(), BTreeSet::new());
    }

    #[test]
    fn hash_set_tuple_weighted() {
        assert_eq!(HashSet::<(usize, usize, usize)>::empty(), HashSet::new());
    }
}

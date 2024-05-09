#![doc(alias = "edgeless")]
//! A trait to generate empty const-sized directed graphs
//!
//! Empty graphs are also known as edgeless graphs. To generate empty
//! variable-sized graphs, see [`Empty`].
//!
//! # Examples
//!
//! ```
//! use graaf::gen::EmptyConst;
//!
//! assert!(<[Vec::<usize>; 0]>::empty().is_empty());
//! assert_eq!(<[Vec::<usize>; 1]>::empty(), [Vec::new()]);
//! assert_eq!(<[Vec::<usize>; 2]>::empty(), [Vec::new(), Vec::new()]);
//!
//! assert_eq!(
//!     <[Vec::<usize>; 3]>::empty(),
//!     [Vec::new(), Vec::new(), Vec::new()]
//! );
//! ```
//!
//! [`Empty`]: crate::gen::Empty

extern crate alloc;

use {
    alloc::collections::{
        BTreeMap,
        BTreeSet,
    },
    core::{
        array::from_fn,
        hash::BuildHasher,
    },
    std::collections::{
        HashMap,
        HashSet,
    },
};

/// A trait to generate empty const-sized directed graphs
///
/// # How can I implement `EmptyConst`?
///
/// Provide an implementation of `empty` that generates a empty graph with `V`
/// vertices.
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
/// struct Graph<const V: usize, H>
/// where
///     H: BuildHasher,
/// {
///     edges: [HashSet<usize, H>; V],
/// }
///
/// impl<const V: usize, H> EmptyConst for Graph<V, H>
/// where
///     H: BuildHasher + Default,
/// {
///     fn empty() -> Self {
///         Graph {
///             edges: from_fn(|_| HashSet::with_hasher(H::default())),
///         }
///     }
/// }
///
/// let graph = Graph::<3, RandomState>::empty();
///
/// assert_eq!(
///     graph.edges,
///     [HashSet::new(), HashSet::new(), HashSet::new()]
/// );
/// ```
#[doc(alias = "circular")]
pub trait EmptyConst {
    /// Generates a empty graph.
    fn empty() -> Self;
}

impl<const V: usize> EmptyConst for [Vec<usize>; V] {
    fn empty() -> Self {
        from_fn(|_| Vec::new())
    }
}

impl<const V: usize> EmptyConst for [BTreeSet<usize>; V] {
    fn empty() -> Self {
        from_fn(|_| BTreeSet::new())
    }
}

impl<const V: usize, H> EmptyConst for [HashSet<usize, H>; V]
where
    H: BuildHasher + Default,
{
    fn empty() -> Self {
        from_fn(|_| HashSet::with_hasher(H::default()))
    }
}

impl<const V: usize, W> EmptyConst for [Vec<(usize, W)>; V] {
    fn empty() -> Self {
        from_fn(|_| Vec::new())
    }
}

impl<const V: usize, W> EmptyConst for [BTreeSet<(usize, W)>; V] {
    fn empty() -> Self {
        from_fn(|_| BTreeSet::new())
    }
}

impl<const V: usize, W, H> EmptyConst for [HashSet<(usize, W), H>; V]
where
    W: Default,
    H: BuildHasher + Default,
{
    fn empty() -> Self {
        from_fn(|_| HashSet::with_hasher(H::default()))
    }
}

impl<const V: usize, W> EmptyConst for [BTreeMap<usize, W>; V]
where
    W: Default,
{
    fn empty() -> Self {
        from_fn(|_| BTreeMap::new())
    }
}

impl<const V: usize, W, H> EmptyConst for [HashMap<usize, W, H>; V]
where
    W: Default,
    H: BuildHasher + Default,
{
    fn empty() -> Self {
        from_fn(|_| HashMap::with_hasher(H::default()))
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::op::{
            CountAllEdges,
            CountAllVertices,
        },
    };

    #[test]
    fn arr_vec_unweighted() {
        assert!(<[Vec::<usize>; 0]>::empty().is_empty());
        assert_eq!(<[Vec::<usize>; 1]>::empty(), [Vec::new()]);
        assert_eq!(<[Vec::<usize>; 2]>::empty(), [Vec::new(), Vec::new()]);

        assert_eq!(
            <[Vec::<usize>; 3]>::empty(),
            [Vec::new(), Vec::new(), Vec::new()]
        );
    }

    #[test]
    fn arr_btree_set_unweighted() {
        assert!(<[BTreeSet::<usize>; 0]>::empty().is_empty());
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
        assert!(<[HashSet::<usize>; 0]>::empty().is_empty());
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
        assert!(<[Vec<(usize, usize)>; 0]>::empty().is_empty());
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
        assert!(<[BTreeSet<(usize, usize)>; 0]>::empty().is_empty());
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
        assert!(<[HashSet<(usize, usize)>; 0]>::empty().is_empty());
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
        assert!(<[BTreeMap<usize, usize>; 0]>::empty().is_empty());
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
        assert!(<[HashMap<usize, usize>; 0]>::empty().is_empty());
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
    fn count_all_edges_arr_vec_unweighted() {
        assert_eq!(<[Vec<usize>; 0]>::empty().count_all_edges(), 0);
        assert_eq!(<[Vec<usize>; 1]>::empty().count_all_edges(), 0);
        assert_eq!(<[Vec<usize>; 2]>::empty().count_all_edges(), 0);
        assert_eq!(<[Vec<usize>; 3]>::empty().count_all_edges(), 0);
    }

    #[test]
    fn count_all_edges_arr_btree_set_unweighted() {
        assert_eq!(<[BTreeSet<usize>; 0]>::empty().count_all_edges(), 0);
        assert_eq!(<[BTreeSet<usize>; 1]>::empty().count_all_edges(), 0);
        assert_eq!(<[BTreeSet<usize>; 2]>::empty().count_all_edges(), 0);
        assert_eq!(<[BTreeSet<usize>; 3]>::empty().count_all_edges(), 0);
    }

    #[test]
    fn count_all_edges_arr_hash_set_unweighted() {
        assert_eq!(<[HashSet<usize>; 0]>::empty().count_all_edges(), 0);
        assert_eq!(<[HashSet<usize>; 1]>::empty().count_all_edges(), 0);
        assert_eq!(<[HashSet<usize>; 2]>::empty().count_all_edges(), 0);
        assert_eq!(<[HashSet<usize>; 3]>::empty().count_all_edges(), 0);
    }

    #[test]
    fn count_all_edges_arr_vec_weighted() {
        assert_eq!(<[Vec<(usize, usize)>; 0]>::empty().count_all_edges(), 0);
        assert_eq!(<[Vec<(usize, usize)>; 1]>::empty().count_all_edges(), 0);
        assert_eq!(<[Vec<(usize, usize)>; 2]>::empty().count_all_edges(), 0);
        assert_eq!(<[Vec<(usize, usize)>; 3]>::empty().count_all_edges(), 0);
    }

    #[test]
    fn count_all_edges_arr_btree_set_weighted() {
        assert_eq!(
            <[BTreeSet<(usize, usize)>; 0]>::empty().count_all_edges(),
            0
        );
        assert_eq!(
            <[BTreeSet<(usize, usize)>; 1]>::empty().count_all_edges(),
            0
        );
        assert_eq!(
            <[BTreeSet<(usize, usize)>; 2]>::empty().count_all_edges(),
            0
        );
        assert_eq!(
            <[BTreeSet<(usize, usize)>; 3]>::empty().count_all_edges(),
            0
        );
    }

    #[test]
    fn count_all_edges_arr_hash_set_weighted() {
        assert_eq!(<[HashSet<(usize, usize)>; 0]>::empty().count_all_edges(), 0);
        assert_eq!(<[HashSet<(usize, usize)>; 1]>::empty().count_all_edges(), 0);
        assert_eq!(<[HashSet<(usize, usize)>; 2]>::empty().count_all_edges(), 0);
        assert_eq!(<[HashSet<(usize, usize)>; 3]>::empty().count_all_edges(), 0);
    }

    #[test]
    fn count_all_edges_arr_btree_map() {
        assert_eq!(<[BTreeMap<usize, usize>; 0]>::empty().count_all_edges(), 0);
        assert_eq!(<[BTreeMap<usize, usize>; 1]>::empty().count_all_edges(), 0);
        assert_eq!(<[BTreeMap<usize, usize>; 2]>::empty().count_all_edges(), 0);
        assert_eq!(<[BTreeMap<usize, usize>; 3]>::empty().count_all_edges(), 0);
    }

    #[test]
    fn count_all_edges_arr_hash_map() {
        assert_eq!(<[HashMap<usize, usize>; 0]>::empty().count_all_edges(), 0);
        assert_eq!(<[HashMap<usize, usize>; 1]>::empty().count_all_edges(), 0);
        assert_eq!(<[HashMap<usize, usize>; 2]>::empty().count_all_edges(), 0);
        assert_eq!(<[HashMap<usize, usize>; 3]>::empty().count_all_edges(), 0);
    }

    #[test]
    fn count_all_vertices_arr_vec_unweighted() {
        assert_eq!(<[Vec<usize>; 0]>::empty().count_all_vertices(), 0);
        assert_eq!(<[Vec<usize>; 1]>::empty().count_all_vertices(), 1);
        assert_eq!(<[Vec<usize>; 2]>::empty().count_all_vertices(), 2);
        assert_eq!(<[Vec<usize>; 3]>::empty().count_all_vertices(), 3);
    }

    #[test]
    fn count_all_vertices_arr_btree_set_unweighted() {
        assert_eq!(<[BTreeSet<usize>; 0]>::empty().count_all_vertices(), 0);
        assert_eq!(<[BTreeSet<usize>; 1]>::empty().count_all_vertices(), 1);
        assert_eq!(<[BTreeSet<usize>; 2]>::empty().count_all_vertices(), 2);
        assert_eq!(<[BTreeSet<usize>; 3]>::empty().count_all_vertices(), 3);
    }

    #[test]
    fn count_all_vertices_arr_hash_set_unweighted() {
        assert_eq!(<[HashSet<usize>; 0]>::empty().count_all_vertices(), 0);
        assert_eq!(<[HashSet<usize>; 1]>::empty().count_all_vertices(), 1);
        assert_eq!(<[HashSet<usize>; 2]>::empty().count_all_vertices(), 2);
        assert_eq!(<[HashSet<usize>; 3]>::empty().count_all_vertices(), 3);
    }

    #[test]
    fn count_all_vertices_arr_arr_weighted() {
        assert_eq!(<[Vec<(usize, usize)>; 0]>::empty().count_all_vertices(), 0);
        assert_eq!(<[Vec<(usize, usize)>; 1]>::empty().count_all_vertices(), 1);
        assert_eq!(<[Vec<(usize, usize)>; 2]>::empty().count_all_vertices(), 2);
        assert_eq!(<[Vec<(usize, usize)>; 3]>::empty().count_all_vertices(), 3);
    }

    #[test]
    fn count_all_vertices_arr_btree_set_weighted() {
        assert_eq!(
            <[BTreeSet<(usize, usize)>; 0]>::empty().count_all_vertices(),
            0
        );
        assert_eq!(
            <[BTreeSet<(usize, usize)>; 1]>::empty().count_all_vertices(),
            1
        );
        assert_eq!(
            <[BTreeSet<(usize, usize)>; 2]>::empty().count_all_vertices(),
            2
        );
        assert_eq!(
            <[BTreeSet<(usize, usize)>; 3]>::empty().count_all_vertices(),
            3
        );
    }

    #[test]
    fn count_all_vertices_arr_hash_set_weighted() {
        assert_eq!(
            <[HashSet<(usize, usize)>; 0]>::empty().count_all_vertices(),
            0
        );
        assert_eq!(
            <[HashSet<(usize, usize)>; 1]>::empty().count_all_vertices(),
            1
        );
        assert_eq!(
            <[HashSet<(usize, usize)>; 2]>::empty().count_all_vertices(),
            2
        );
        assert_eq!(
            <[HashSet<(usize, usize)>; 3]>::empty().count_all_vertices(),
            3
        );
    }

    #[test]
    fn count_all_vertices_arr_btree_map() {
        assert_eq!(
            <[BTreeMap<usize, usize>; 0]>::empty().count_all_vertices(),
            0
        );
        assert_eq!(
            <[BTreeMap<usize, usize>; 1]>::empty().count_all_vertices(),
            1
        );
        assert_eq!(
            <[BTreeMap<usize, usize>; 2]>::empty().count_all_vertices(),
            2
        );
        assert_eq!(
            <[BTreeMap<usize, usize>; 3]>::empty().count_all_vertices(),
            3
        );
    }

    #[test]
    fn count_all_vertices_arr_hash_map() {
        assert_eq!(
            <[HashMap<usize, usize>; 0]>::empty().count_all_vertices(),
            0
        );
        assert_eq!(
            <[HashMap<usize, usize>; 1]>::empty().count_all_vertices(),
            1
        );
        assert_eq!(
            <[HashMap<usize, usize>; 2]>::empty().count_all_vertices(),
            2
        );
        assert_eq!(
            <[HashMap<usize, usize>; 3]>::empty().count_all_vertices(),
            3
        );
    }
}
#![doc(alias = "edgeless")]
//! A trait to generate empty variable-sized directed graphs
//!
//! Empty graphs are also known as edgeless graphs. To generate empty
//! const-sized directed graphs, see [`EmptyConst`].
//!
//! # Examples
//!
//! ```
//! use graaf::gen::Empty;
//!
//! assert!(Vec::<Vec<usize>>::empty(0).is_empty());
//! assert_eq!(Vec::<Vec<usize>>::empty(1), vec![Vec::new()]);
//! assert_eq!(Vec::<Vec<usize>>::empty(2), vec![Vec::new(), Vec::new()]);
//!
//! assert_eq!(
//!     Vec::<Vec<usize>>::empty(3),
//!     vec![Vec::new(), Vec::new(), Vec::new()]
//! );
//! ```
//!
//! [`EmptyConst`]: crate::gen::EmptyConst

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

/// A trait to generate empty variable-sized directed graphs
///
/// # How can I implement `Empty`?
///
/// Provide an implementation of `empty` that generates an empty graph with `v`
/// vertices.
///
/// ```
/// use {
///     graaf::gen::Empty,
///     std::collections::HashSet,
/// };
///
/// #[derive(Debug, PartialEq)]
/// struct Graph {
///     edges: HashSet<(usize, usize)>,
/// }
///
/// impl Empty for Graph {
///     fn empty(v: usize) -> Self {
///         Graph {
///             edges: HashSet::new(),
///         }
///     }
/// }
///
/// assert_eq!(
///     Graph::empty(3),
///     Graph {
///         edges: HashSet::new()
///     }
/// );
/// ```
pub trait Empty {
    /// Generates an empty graph.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices in the graph
    fn empty(v: usize) -> Self;
}

impl Empty for Vec<Vec<usize>> {
    fn empty(v: usize) -> Self {
        vec![Vec::new(); v]
    }
}

impl Empty for Vec<BTreeSet<usize>> {
    fn empty(v: usize) -> Self {
        vec![BTreeSet::new(); v]
    }
}

impl<H> Empty for Vec<HashSet<usize, H>>
where
    H: BuildHasher + Default,
    HashSet<usize, H>: Clone,
{
    fn empty(v: usize) -> Self {
        vec![HashSet::with_hasher(H::default()); v]
    }
}

impl Empty for BTreeMap<usize, Vec<usize>> {
    fn empty(v: usize) -> Self {
        (0..v).map(|s| (s, Vec::new())).collect()
    }
}

impl Empty for BTreeMap<usize, BTreeSet<usize>> {
    fn empty(v: usize) -> Self {
        (0..v).map(|s| (s, BTreeSet::new())).collect()
    }
}

impl<H> Empty for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher + Default,
{
    fn empty(v: usize) -> Self {
        (0..v).map(|s| (s, Vec::new())).collect()
    }
}

impl<H> Empty for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher + Default,
{
    fn empty(v: usize) -> Self {
        (0..v)
            .map(|s| (s, HashSet::with_hasher(H::default())))
            .collect()
    }
}

impl Empty for Vec<(usize, usize)> {
    fn empty(_: usize) -> Self {
        Self::new()
    }
}

impl Empty for BTreeSet<(usize, usize)> {
    fn empty(_: usize) -> Self {
        Self::new()
    }
}

impl<H> Empty for HashSet<(usize, usize), H>
where
    H: BuildHasher + Default,
{
    fn empty(_: usize) -> Self {
        Self::with_hasher(H::default())
    }
}

impl<W> Empty for Vec<Vec<(usize, W)>>
where
    W: Clone,
{
    fn empty(v: usize) -> Self {
        vec![Vec::new(); v]
    }
}

impl<W> Empty for Vec<BTreeSet<(usize, W)>>
where
    W: Clone,
{
    fn empty(v: usize) -> Self {
        vec![BTreeSet::new(); v]
    }
}

impl<W, H> Empty for Vec<HashSet<(usize, W), H>>
where
    W: Clone,
    H: BuildHasher + Default,
    HashSet<(usize, W), H>: Clone,
{
    fn empty(v: usize) -> Self {
        vec![HashSet::with_hasher(H::default()); v]
    }
}

impl<W> Empty for Vec<BTreeMap<usize, W>>
where
    W: Clone,
{
    fn empty(v: usize) -> Self {
        vec![BTreeMap::new(); v]
    }
}

impl<W, H> Empty for Vec<HashMap<usize, W, H>>
where
    W: Clone,
    H: BuildHasher + Default,
    HashMap<usize, W, H>: Clone,
{
    fn empty(v: usize) -> Self {
        vec![HashMap::with_hasher(H::default()); v]
    }
}

impl<W> Empty for BTreeMap<usize, Vec<(usize, W)>>
where
    W: Clone,
{
    fn empty(v: usize) -> Self {
        (0..v).map(|s| (s, Vec::new())).collect()
    }
}

impl<W> Empty for BTreeMap<usize, BTreeSet<(usize, W)>>
where
    W: Clone,
{
    fn empty(v: usize) -> Self {
        (0..v).map(|s| (s, BTreeSet::new())).collect()
    }
}

impl<W> Empty for BTreeMap<usize, BTreeMap<usize, W>>
where
    W: Clone,
{
    fn empty(v: usize) -> Self {
        (0..v).map(|s| (s, BTreeMap::new())).collect()
    }
}

impl<W, H> Empty for HashMap<usize, Vec<(usize, W)>, H>
where
    W: Clone,
    H: BuildHasher + Default,
{
    fn empty(v: usize) -> Self {
        (0..v).map(|s| (s, Vec::new())).collect()
    }
}

impl<W, H> Empty for HashMap<usize, HashSet<(usize, W), H>, H>
where
    W: Clone,
    H: BuildHasher + Default,
{
    fn empty(v: usize) -> Self {
        (0..v)
            .map(|s| (s, HashSet::with_hasher(H::default())))
            .collect()
    }
}

impl<W, H> Empty for HashMap<usize, HashMap<usize, W, H>, H>
where
    W: Clone,
    H: BuildHasher + Default,
{
    fn empty(v: usize) -> Self {
        (0..v)
            .map(|s| (s, HashMap::with_hasher(H::default())))
            .collect()
    }
}

impl<W> Empty for Vec<(usize, usize, W)> {
    fn empty(_: usize) -> Self {
        Self::new()
    }
}

impl<W> Empty for BTreeSet<(usize, usize, W)> {
    fn empty(_: usize) -> Self {
        Self::new()
    }
}

impl<W, H> Empty for HashSet<(usize, usize, W), H>
where
    H: BuildHasher + Default,
{
    fn empty(_: usize) -> Self {
        Self::with_hasher(H::default())
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::op::{
            CountAllEdges,
            CountAllVertices,
            Indegree,
            IsSimple,
            Outdegree,
        },
        proptest::prelude::*,
    };

    fn prop_count_all_edges<T: CountAllEdges + Empty>(v: usize) {
        assert_eq!(T::empty(v).count_all_edges(), 0);
    }

    fn prop_count_all_vertices<T: CountAllVertices + Empty>(v: usize) {
        assert_eq!(T::empty(v).count_all_vertices(), v);
    }

    fn prop_indegree<T: Indegree + Empty>(v: usize) {
        let graph = T::empty(v);

        for s in 0..v {
            assert_eq!(graph.indegree(s), 0);
        }
    }

    fn prop_is_simple<T: IsSimple + Empty>(v: usize) {
        assert!(T::empty(v).is_simple());
    }

    fn prop_outdegree<T: Outdegree + Empty>(v: usize) {
        let graph = T::empty(v);

        for s in 0..v {
            assert_eq!(graph.outdegree(s), 0);
        }
    }

    proptest! {
        #[test]
        fn count_all_edges_vec_vec_unweighted(v in 0..100_usize) {
            prop_count_all_edges::<Vec<Vec<usize>>>(v);
        }

        #[test]
        fn count_all_edges_vec_btree_set_unweighted(v in 0..100_usize) {
            prop_count_all_edges::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn count_all_edges_vec_hash_set_unweighted(v in 0..100_usize) {
            prop_count_all_edges::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn count_all_edges_btree_map_vec_unweighted(v in 0..100_usize) {
            prop_count_all_edges::<BTreeMap<usize, Vec<usize>>>(v);
        }

        #[test]
        fn count_all_edges_btree_map_btree_set_unweighted(v in 0..100_usize) {
            prop_count_all_edges::<BTreeMap<usize, BTreeSet<usize>>>(v);
        }

        #[test]
        fn count_all_edges_hash_map_vec_unweighted(v in 0..100_usize) {
            prop_count_all_edges::<HashMap<usize, Vec<usize>>>(v);
        }

        #[test]
        fn count_all_edges_hash_map_hash_set_unweighted(v in 0..100_usize) {
            prop_count_all_edges::<HashMap<usize, HashSet<usize>>>(v);
        }

        #[test]
        fn count_all_edges_vec_tuple_unweighted(v in 0..100_usize) {
            prop_count_all_edges::<Vec<(usize, usize)>>(v);
        }

        #[test]
        fn count_all_edges_btree_set_tuple_unweighted(v in 0..100_usize) {
            prop_count_all_edges::<BTreeSet<(usize, usize)>>(v);
        }

        #[test]
        fn count_all_edges_hash_set_tuple_unweighted(v in 0..100_usize) {
            prop_count_all_edges::<HashSet<(usize, usize)>>(v);
        }

        #[test]
        fn count_all_edges_vec_vec_weighted(v in 0..100_usize) {
            prop_count_all_edges::<Vec<Vec<(usize, usize)>>>(v);
        }

        #[test]
        fn count_all_edges_vec_btree_set_weighted(v in 0..100_usize) {
            prop_count_all_edges::<Vec<BTreeSet<(usize, usize)>>>(v);
        }

        #[test]
        fn count_all_edges_vec_hash_set_weighted(v in 0..100_usize) {
            prop_count_all_edges::<Vec<HashSet<(usize, usize)>>>(v);
        }

        #[test]
        fn count_all_edges_vec_btree_map_weighted(v in 0..100_usize) {
            prop_count_all_edges::<Vec<BTreeMap<usize, usize>>>(v);
        }

        #[test]
        fn count_all_edges_vec_hash_map_weighted(v in 0..100_usize) {
            prop_count_all_edges::<Vec<HashMap<usize, usize>>>(v);
        }

        #[test]
        fn count_all_edges_btree_map_vec_weighted(v in 0..100_usize) {
            prop_count_all_edges::<BTreeMap<usize, Vec<(usize, usize)>>>(v);
        }

        #[test]
        fn count_all_edges_btree_map_btree_set_weighted(v in 0..100_usize) {
            prop_count_all_edges::<BTreeMap<usize, BTreeSet<(usize, usize)>>>(v);
        }

        #[test]
        fn count_all_edges_btree_map_btree_map(v in 0..100_usize) {
            prop_count_all_edges::<BTreeMap<usize, BTreeMap<usize, usize>>>(v);
        }

        #[test]
        fn count_all_edges_hash_map_vec_weighted(v in 0..100_usize) {
            prop_count_all_edges::<HashMap<usize, Vec<(usize, usize)>>>(v);
        }

        #[test]
        fn count_all_edges_hash_map_hash_set_weighted(v in 0..100_usize) {
            prop_count_all_edges::<HashMap<usize, HashSet<(usize, usize)>>>(v);
        }

        #[test]
        fn count_all_edges_hash_map_hash_map(v in 0..100_usize) {
            prop_count_all_edges::<HashMap<usize, HashMap<usize, usize>>>(v);
        }

        #[test]
        fn count_all_edges_vec_tuple_weighted(v in 0..100_usize) {
            prop_count_all_edges::<Vec<(usize, usize, usize)>>(v);
        }

        #[test]
        fn count_all_edges_btree_set_tuple_weighted(v in 0..100_usize) {
            prop_count_all_edges::<BTreeSet<(usize, usize, usize)>>(v);
        }

        #[test]
        fn count_all_edges_hash_set_tuple_weighted(v in 0..100_usize) {
            prop_count_all_edges::<HashSet<(usize, usize, usize)>>(v);
        }

        #[test]
        fn count_all_vertices_vec_vec_unweighted(v in 0..100_usize) {
            prop_count_all_vertices::<Vec<Vec<usize>>>(v);
        }

        #[test]
        fn count_all_vertices_vec_btree_set_unweighted(v in 0..100_usize) {
            prop_count_all_vertices::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn count_all_vertices_vec_hash_set_unweighted(v in 0..100_usize) {
            prop_count_all_vertices::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn count_all_vertices_vec_vec_weighted(v in 0..100_usize) {
            prop_count_all_vertices::<Vec<Vec<(usize, usize)>>>(v);
        }

        #[test]
        fn count_all_vertices_vec_btree_set_weighted(v in 0..100_usize) {
            prop_count_all_vertices::<Vec<BTreeSet<(usize, usize)>>>(v);
        }

        #[test]
        fn count_all_vertices_vec_hash_set_weighted(v in 0..100_usize) {
            prop_count_all_vertices::<Vec<HashSet<(usize, usize)>>>(v);
        }

        #[test]
        fn indegree_vec_btree_set_unweighted(v in 1..100_usize) {
            prop_indegree::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn indegree_vec_hash_set_unweighted(v in 1..100_usize) {
            prop_indegree::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn indegree_btree_map_btree_set_unweighted(v in 1..100_usize) {
            prop_indegree::<BTreeMap<usize, BTreeSet<usize>>>(v);
        }

        #[test]
        fn indegree_hash_map_hash_set_unweighted(v in 1..100_usize) {
            prop_indegree::<HashMap<usize, HashSet<usize>>>(v);
        }

        #[test]
        fn indegree_vec_btree_map_weighted(v in 1..100_usize) {
            prop_indegree::<Vec<BTreeMap<usize, usize>>>(v);
        }

        #[test]
        fn indegree_vec_hash_map_weighted(v in 1..100_usize) {
            prop_indegree::<Vec<HashMap<usize, usize>>>(v);
        }

        #[test]
        fn indegree_btree_map_btree_set(v in 1..100_usize) {
            prop_indegree::<BTreeMap<usize, BTreeSet<usize>>>(v);
        }

        #[test]
        fn indegree_hash_map_hash_set(v in 1..100_usize) {
            prop_indegree::<HashMap<usize, HashSet<usize>>>(v);
        }

        #[test]
        fn indegree_btree_map_btree_map(v in 1..100_usize) {
            prop_indegree::<BTreeMap<usize, BTreeMap<usize, usize>>>(v);
        }

        #[test]
        fn indegree_hash_map_hash_map(v in 1..100_usize) {
            prop_indegree::<HashMap<usize, HashMap<usize, usize>>>(v);
        }

        #[test]
        fn is_simple_vec_btree_set_unweighted(v in 0..100_usize) {
            prop_is_simple::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn is_simple_vec_hash_set_unweighted(v in 0..100_usize) {
            prop_is_simple::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn is_simple_vec_tuple_unweighted(v in 0..100_usize) {
            prop_is_simple::<Vec<(usize, usize)>>(v);
        }

        #[test]
        fn is_simple_btree_set_tuple_unweighted(v in 0..100_usize) {
            prop_is_simple::<BTreeSet<(usize, usize)>>(v);
        }

        #[test]
        fn is_simple_hash_set_tuple_unweighted(v in 0..100_usize) {
            prop_is_simple::<HashSet<(usize, usize)>>(v);
        }

        #[test]
        fn is_simple_vec_tuple_weighted(v in 0..100_usize) {
            prop_is_simple::<Vec<(usize, usize, usize)>>(v);
        }

        #[test]
        fn is_simple_btree_set_tuple_weighted(v in 0..100_usize) {
            prop_is_simple::<BTreeSet<(usize, usize, usize)>>(v);
        }

        #[test]
        fn is_simple_hash_set_tuple_weighted(v in 0..100_usize) {
            prop_is_simple::<HashSet<(usize, usize, usize)>>(v);
        }

        #[test]
        fn outdegree_vec_vec_unweighted(v in 1..100_usize) {
            prop_outdegree::<Vec<Vec<usize>>>(v);
        }

        #[test]
        fn outdegree_vec_btree_set_unweighted(v in 1..100_usize) {
            prop_outdegree::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn outdegree_vec_hash_set_unweighted(v in 1..100_usize) {
            prop_outdegree::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn outdegree_btree_map_vec_unweighted(v in 1..100_usize) {
            prop_outdegree::<BTreeMap<usize, Vec<usize>>>(v);
        }

        #[test]
        fn outdegree_btree_map_btree_set_unweighted(v in 1..100_usize) {
            prop_outdegree::<BTreeMap<usize, BTreeSet<usize>>>(v);
        }

        #[test]
        fn outdegree_hash_map_vec_unweighted(v in 1..100_usize) {
            prop_outdegree::<HashMap<usize, Vec<usize>>>(v);
        }

        #[test]
        fn outdegree_hash_map_hash_set_unweighted(v in 1..100_usize) {
            prop_outdegree::<HashMap<usize, HashSet<usize>>>(v);
        }

        #[test]
        fn outdegree_vec_vec_weighted(v in 1..100_usize) {
            prop_outdegree::<Vec<Vec<(usize, usize)>>>(v);
        }

        #[test]
        fn outdegree_vec_btree_set_weighted(v in 1..100_usize) {
            prop_outdegree::<Vec<BTreeSet<(usize, usize)>>>(v);
        }

        #[test]
        fn outdegree_vec_hash_set_weighted(v in 1..100_usize) {
            prop_outdegree::<Vec<HashSet<(usize, usize)>>>(v);
        }

        #[test]
        fn outdegree_vec_btree_map_weighted(v in 1..100_usize) {
            prop_outdegree::<Vec<BTreeMap<usize, usize>>>(v);
        }

        #[test]
        fn outdegree_vec_hash_map_weighted(v in 1..100_usize) {
            prop_outdegree::<Vec<HashMap<usize, usize>>>(v);
        }

        #[test]
        fn outdegree_btree_map_btree_map(v in 1..100_usize) {
            prop_outdegree::<BTreeMap<usize, BTreeMap<usize, usize>>>(v);
        }

        #[test]
        fn outdegree_hash_map_hash_map(v in 1..100_usize) {
            prop_outdegree::<HashMap<usize, HashMap<usize, usize>>>(v);
        }
    }

    #[test]
    fn vec_vec_unweighted() {
        for (v, g) in [
            Vec::new(),
            vec![Vec::new()],
            vec![Vec::new(), Vec::new()],
            vec![Vec::new(), Vec::new(), Vec::new()],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<Vec<usize>>::empty(v), g);
        }
    }

    #[test]
    fn vec_btree_set_unweighted() {
        for (v, g) in [
            Vec::new(),
            vec![BTreeSet::new()],
            vec![BTreeSet::new(), BTreeSet::new()],
            vec![BTreeSet::new(), BTreeSet::new(), BTreeSet::new()],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<BTreeSet<usize>>::empty(v), g);
        }
    }

    #[test]
    fn vec_hash_set_unweighted() {
        for (v, g) in [
            Vec::new(),
            vec![HashSet::new()],
            vec![HashSet::new(), HashSet::new()],
            vec![HashSet::new(), HashSet::new(), HashSet::new()],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<HashSet<usize>>::empty(v), g);
        }
    }

    #[test]
    fn btree_map_vec_unweighted() {
        for (v, g) in [
            BTreeMap::new(),
            BTreeMap::from([(0, Vec::new())]),
            BTreeMap::from([(0, Vec::new()), (1, Vec::new())]),
            BTreeMap::from([(0, Vec::new()), (1, Vec::new()), (2, Vec::new())]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&BTreeMap::<usize, Vec<usize>>::empty(v), g);
        }
    }

    #[test]
    fn btree_map_btree_set_unweighted() {
        for (v, g) in [
            BTreeMap::new(),
            BTreeMap::from([(0, BTreeSet::new())]),
            BTreeMap::from([(0, BTreeSet::new()), (1, BTreeSet::new())]),
            BTreeMap::from([
                (0, BTreeSet::new()),
                (1, BTreeSet::new()),
                (2, BTreeSet::new()),
            ]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&BTreeMap::<usize, BTreeSet<usize>>::empty(v), g);
        }
    }

    #[test]
    fn hash_map_vec_unweighted() {
        for (v, g) in [
            HashMap::new(),
            HashMap::from([(0, Vec::new())]),
            HashMap::from([(0, Vec::new()), (1, Vec::new())]),
            HashMap::from([(0, Vec::new()), (1, Vec::new()), (2, Vec::new())]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&HashMap::<usize, Vec<usize>>::empty(v), g);
        }
    }

    #[test]
    fn hash_map_hash_set_unweighted() {
        for (v, g) in [
            HashMap::new(),
            HashMap::from([(0, HashSet::new())]),
            HashMap::from([(0, HashSet::new()), (1, HashSet::new())]),
            HashMap::from([
                (0, HashSet::new()),
                (1, HashSet::new()),
                (2, HashSet::new()),
            ]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&HashMap::<usize, HashSet<usize>>::empty(v), g);
        }
    }

    #[test]
    fn vec_tuple_unweighted() {
        for (v, g) in [Vec::new(), Vec::new(), Vec::new(), Vec::new()]
            .iter()
            .enumerate()
        {
            assert_eq!(&Vec::<(usize, usize)>::empty(v), g);
        }
    }

    #[test]
    fn btree_set_tuple_unweighted() {
        for (v, g) in [
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeSet::new(),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&BTreeSet::<(usize, usize)>::empty(v), g);
        }
    }

    #[test]
    fn hash_set_tuple_unweighted() {
        for (v, g) in [
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&HashSet::<(usize, usize)>::empty(v), g);
        }
    }

    #[test]
    fn vec_vec_weighted() {
        for (v, g) in [
            Vec::new(),
            vec![Vec::new()],
            vec![Vec::new(), Vec::new()],
            vec![Vec::new(), Vec::new(), Vec::new()],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<Vec<(usize, usize)>>::empty(v), g);
        }
    }

    #[test]
    fn vec_btree_set_weighted() {
        for (v, g) in [
            Vec::new(),
            vec![BTreeSet::new()],
            vec![BTreeSet::new(), BTreeSet::new()],
            vec![BTreeSet::new(), BTreeSet::new(), BTreeSet::new()],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<BTreeSet<(usize, usize)>>::empty(v), g);
        }
    }

    #[test]
    fn vec_hash_set_weighted() {
        for (v, g) in [
            Vec::new(),
            vec![HashSet::new()],
            vec![HashSet::new(), HashSet::new()],
            vec![HashSet::new(), HashSet::new(), HashSet::new()],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<HashSet<(usize, usize)>>::empty(v), g);
        }
    }

    #[test]
    fn vec_btree_map_weighted() {
        for (v, g) in [
            Vec::new(),
            vec![BTreeMap::new()],
            vec![BTreeMap::new(), BTreeMap::new()],
            vec![BTreeMap::new(), BTreeMap::new(), BTreeMap::new()],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<BTreeMap<usize, usize>>::empty(v), g);
        }
    }

    #[test]
    fn vec_hash_map_weighted() {
        for (v, g) in [
            Vec::new(),
            vec![HashMap::new()],
            vec![HashMap::new(), HashMap::new()],
            vec![HashMap::new(), HashMap::new(), HashMap::new()],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<HashMap<usize, usize>>::empty(v), g);
        }
    }

    #[test]
    fn btree_map_vec_weighted() {
        for (v, g) in [
            BTreeMap::new(),
            BTreeMap::from([(0, Vec::new())]),
            BTreeMap::from([(0, Vec::new()), (1, Vec::new())]),
            BTreeMap::from([(0, Vec::new()), (1, Vec::new()), (2, Vec::new())]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&BTreeMap::<usize, Vec<(usize, usize)>>::empty(v), g);
        }
    }

    #[test]
    fn btree_map_btree_set_weighted() {
        for (v, g) in [
            BTreeMap::new(),
            BTreeMap::from([(0, BTreeSet::new())]),
            BTreeMap::from([(0, BTreeSet::new()), (1, BTreeSet::new())]),
            BTreeMap::from([
                (0, BTreeSet::new()),
                (1, BTreeSet::new()),
                (2, BTreeSet::new()),
            ]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&BTreeMap::<usize, BTreeSet<(usize, usize)>>::empty(v), g);
        }
    }

    #[test]
    fn btree_map_btree_map() {
        for (v, g) in [
            BTreeMap::new(),
            BTreeMap::from([(0, BTreeMap::new())]),
            BTreeMap::from([(0, BTreeMap::new()), (1, BTreeMap::new())]),
            BTreeMap::from([
                (0, BTreeMap::new()),
                (1, BTreeMap::new()),
                (2, BTreeMap::new()),
            ]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&BTreeMap::<usize, BTreeMap<usize, usize>>::empty(v), g);
        }
    }

    #[test]
    fn hash_map_vec_weighted() {
        for (v, g) in [
            HashMap::new(),
            HashMap::from([(0, Vec::new())]),
            HashMap::from([(0, Vec::new()), (1, Vec::new())]),
            HashMap::from([(0, Vec::new()), (1, Vec::new()), (2, Vec::new())]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&HashMap::<usize, Vec<(usize, usize)>>::empty(v), g);
        }
    }

    #[test]
    fn hash_map_hash_set_weighted() {
        for (v, g) in [
            HashMap::new(),
            HashMap::from([(0, HashSet::new())]),
            HashMap::from([(0, HashSet::new()), (1, HashSet::new())]),
            HashMap::from([
                (0, HashSet::new()),
                (1, HashSet::new()),
                (2, HashSet::new()),
            ]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&HashMap::<usize, HashSet<(usize, usize)>>::empty(v), g);
        }
    }

    #[test]
    fn hash_map_hash_map() {
        for (v, g) in [
            HashMap::new(),
            HashMap::from([(0, HashMap::new())]),
            HashMap::from([(0, HashMap::new()), (1, HashMap::new())]),
            HashMap::from([
                (0, HashMap::new()),
                (1, HashMap::new()),
                (2, HashMap::new()),
            ]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&HashMap::<usize, HashMap<usize, usize>>::empty(v), g);
        }
    }

    #[test]
    fn vec_tuple_weighted() {
        for (v, g) in [Vec::new(), Vec::new(), Vec::new(), Vec::new()]
            .iter()
            .enumerate()
        {
            assert_eq!(&Vec::<(usize, usize, usize)>::empty(v), g);
        }
    }

    #[test]
    fn btree_set_tuple_weighted() {
        for (v, g) in [
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeSet::new(),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&BTreeSet::<(usize, usize, usize)>::empty(v), g);
        }
    }

    #[test]
    fn hash_set_tuple_weighted() {
        for (v, g) in [
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&HashSet::<(usize, usize, usize)>::empty(v), g);
        }
    }
}

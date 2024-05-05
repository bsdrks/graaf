//! A trait to generate variable-sized complete directed graphs
//!
//! The generated graphs are simple; they contain no self-loops. To generate
//! const-sized complete graphs, see [`CompleteConst`].
//!
//! # Examples
//!
//! ```
//! use graaf::gen::Complete;
//!
//! assert!(Vec::<Vec<usize>>::complete(0).is_empty());
//! assert_eq!(Vec::<Vec<usize>>::complete(1), vec![Vec::new()]);
//! assert_eq!(Vec::<Vec<usize>>::complete(2), vec![vec![1], vec![0]]);
//!
//! assert_eq!(
//!     Vec::<Vec<usize>>::complete(3),
//!     vec![vec![1, 2], vec![0, 2], vec![0, 1]]
//! );
//! ```
//!
//! [`CompleteConst`]: crate::gen::CompleteConst

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

/// A trait to generate variable-size complete directed graphs
///
/// # How can I implement `Complete`?
///
/// Provide an implementation of `complete` that generates a complete graph with
/// `v` vertices.
///
/// ```
/// use {
///     graaf::gen::Complete,
///     std::collections::HashSet,
/// };
///
/// struct Graph {
///     edges: HashSet<(usize, usize)>,
/// }
///
/// impl Complete for Graph {
///     fn complete(v: usize) -> Self {
///         let mut edges = HashSet::new();
///
///         for s in 0..v {
///             for t in 0..v {
///                 if s != t {
///                     let _ = edges.insert((s, t));
///                     let _ = edges.insert((t, s));
///                 }
///             }
///         }
///
///         Graph { edges }
///     }
/// }
///
/// let graph = Graph::complete(3);
///
/// assert_eq!(
///     graph.edges,
///     HashSet::from([(0, 1), (1, 0), (0, 2), (2, 0), (1, 2), (2, 1)])
/// );
/// ```
#[doc(alias = "circular")]
pub trait Complete {
    /// Generates a complete graph.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices in the graph
    fn complete(v: usize) -> Self;
}

impl Complete for Vec<Vec<usize>> {
    fn complete(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        (0..v)
            .map(|s| (0..v).filter(|&t| s != t).collect())
            .collect()
    }
}

impl Complete for Vec<BTreeSet<usize>> {
    fn complete(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        (0..v)
            .map(|s| (0..v).filter(|&t| s != t).collect())
            .collect()
    }
}

impl<H> Complete for Vec<HashSet<usize, H>>
where
    H: BuildHasher + Default,
    HashSet<usize, H>: Clone,
{
    fn complete(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        (0..v)
            .map(|s| (0..v).filter(|&t| s != t).collect())
            .collect()
    }
}

impl Complete for BTreeMap<usize, Vec<usize>> {
    fn complete(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        (0..v)
            .map(|s| (s, (0..v).filter(|&t| s != t).collect()))
            .collect()
    }
}

impl Complete for BTreeMap<usize, BTreeSet<usize>> {
    fn complete(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        (0..v)
            .map(|s| (s, (0..v).filter(|&t| s != t).collect()))
            .collect()
    }
}

impl<H> Complete for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher + Default,
{
    fn complete(v: usize) -> Self {
        if v == 0 {
            return Self::with_hasher(H::default());
        }

        (0..v)
            .map(|s| (s, (0..v).filter(|&t| s != t).collect()))
            .collect()
    }
}

impl<H> Complete for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher + Default,
{
    fn complete(v: usize) -> Self {
        if v == 0 {
            return Self::with_hasher(H::default());
        }

        (0..v)
            .map(|s| (s, (0..v).filter(|&t| s != t).collect()))
            .collect()
    }
}

impl Complete for Vec<(usize, usize)> {
    fn complete(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        (0..v)
            .flat_map(|s| (0..v).filter(move |&t| s != t).map(move |t| (s, t)))
            .collect()
    }
}

impl Complete for BTreeSet<(usize, usize)> {
    fn complete(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        (0..v)
            .flat_map(|s| (0..v).filter(move |&t| s != t).map(move |t| (s, t)))
            .collect()
    }
}

impl<H> Complete for HashSet<(usize, usize), H>
where
    H: BuildHasher + Default,
{
    fn complete(v: usize) -> Self {
        if v == 0 {
            return Self::with_hasher(H::default());
        }

        (0..v)
            .flat_map(|s| (0..v).filter(move |&t| s != t).map(move |t| (s, t)))
            .collect()
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

    proptest! {
        #[test]
        fn count_all_edges_vec_vec(v in 1..100_usize) {
            assert_eq!(Vec::<Vec<usize>>::complete(v).count_all_edges(), v * (v - 1));
        }

        #[test]
        fn count_all_edges_vec_btree_set(v in 1..100_usize) {
            assert_eq!(Vec::<BTreeSet<usize>>::complete(v).count_all_edges(), v * (v - 1));
        }

        #[test]
        fn count_all_edges_vec_hash_set(v in 1..100_usize) {
            assert_eq!(Vec::<HashSet<usize>>::complete(v).count_all_edges(), v * (v - 1));
        }

        #[test]
        fn count_all_edges_btree_map_vec(v in 1..100_usize) {
            assert_eq!(BTreeMap::<usize, Vec<usize>>::complete(v).count_all_edges(), v * (v - 1));
        }

        #[test]
        fn count_all_edges_btree_map_btree_set(v in 1..100_usize) {
            assert_eq!(BTreeMap::<usize, BTreeSet<usize>>::complete(v).count_all_edges(), v * (v - 1));
        }

        #[test]
        fn count_all_edges_hash_map_vec(v in 1..100_usize) {
            assert_eq!(HashMap::<usize, Vec<usize>>::complete(v).count_all_edges(), v * (v - 1));
        }

        #[test]
        fn count_all_edges_hash_map_hash_set(v in 1..100_usize) {
            assert_eq!(HashMap::<usize, HashSet<usize>>::complete(v).count_all_edges(), v * (v - 1));
        }

        #[test]
        fn count_all_edges_vec_tuple(v in 1..100_usize) {
            assert_eq!(Vec::<(usize, usize)>::complete(v).count_all_edges(), v * (v - 1));
        }

        #[test]
        fn count_all_edges_btree_set_tuple(v in 1..100_usize) {
            assert_eq!(BTreeSet::<(usize, usize)>::complete(v).count_all_edges(), v * (v - 1));
        }

        #[test]
        fn count_all_edges_hash_set_tuple(v in 1..100_usize) {
            assert_eq!(HashSet::<(usize, usize)>::complete(v).count_all_edges(), v * (v - 1));
        }

        #[test]
        fn count_all_vertices_vec_vec(v in 0..100_usize) {
            assert_eq!(Vec::<Vec<usize>>::complete(v).count_all_vertices(), v);
        }

        #[test]
        fn count_all_vertices_vec_btree_set(v in 0..100_usize) {
            assert_eq!(Vec::<BTreeSet<usize>>::complete(v).count_all_vertices(), v);
        }

        #[test]
        fn count_all_vertices_vec_hash_set(v in 0..100_usize) {
            assert_eq!(Vec::<HashSet<usize>>::complete(v).count_all_vertices(), v);
        }

        #[test]
        fn indegree_vec_btree_set(v in 1..100_usize) {
            let graph = Vec::<BTreeSet<usize>>::complete(v);

            for s in 0..v {
                assert_eq!(graph.indegree(s), v - 1);
            }
        }

        #[test]
        fn indegree_vec_hash_set(v in 1..100_usize) {
            let graph = Vec::<HashSet<usize>>::complete(v);

            for s in 0..v {
                assert_eq!(graph.indegree(s), v - 1);
            }
        }

        #[test]
        fn indegree_btree_map_btree_set(v in 1..100_usize) {
            let graph = BTreeMap::<usize, BTreeSet<usize>>::complete(v);

            for s in 0..v {
                assert_eq!(graph.indegree(s), v - 1);
            }
        }

        #[test]
        fn indegree_hash_map_hash_set(v in 1..100_usize) {
            let graph = HashMap::<usize, HashSet<usize>>::complete(v);

            for s in 0..v {
                assert_eq!(graph.indegree(s), v - 1);
            }
        }

        #[test]
        fn is_simple_vec_btree_set(v in 0..100_usize) {
            assert!(Vec::<BTreeSet<usize>>::complete(v).is_simple());
        }

        #[test]
        fn is_simple_vec_hash_set(v in 0..100_usize) {
            assert!(Vec::<HashSet<usize>>::complete(v).is_simple());
        }

        #[test]
        fn is_simple_vec_tuple(v in 0..100_usize) {
            assert!(Vec::<(usize, usize)>::complete(v).is_simple());
        }

        #[test]
        fn is_simple_btree_set_tuple(v in 0..100_usize) {
            assert!(BTreeSet::<(usize, usize)>::complete(v).is_simple());
        }

        #[test]
        fn is_simple_hash_set_tuple(v in 0..100_usize) {
            assert!(HashSet::<(usize, usize)>::complete(v).is_simple());
        }

        #[test]
        fn outdegree_vec_vec(v in 1..100_usize) {
            let graph = Vec::<Vec<usize>>::complete(v);

            for s in 0..v {
                assert_eq!(graph.outdegree(s), v - 1);
            }
        }

        #[test]
        fn outdegree_vec_btree_set(v in 1..100_usize) {
            let graph = Vec::<BTreeSet<usize>>::complete(v);

            for s in 0..v {
                assert_eq!(graph.outdegree(s), v - 1);
            }
        }

        #[test]
        fn outdegree_vec_hash_set(v in 1..100_usize) {
            let graph = Vec::<HashSet<usize>>::complete(v);

            for s in 0..v {
                assert_eq!(graph.outdegree(s), v - 1);
            }
        }

        #[test]
        fn outdegree_btree_map_vec(v in 1..100_usize) {
            let graph = BTreeMap::<usize, Vec<usize>>::complete(v);

            for s in 0..v {
                assert_eq!(graph.outdegree(s), v - 1);
            }
        }

        #[test]
        fn outdegree_btree_map_btree_set(v in 1..100_usize) {
            let graph = BTreeMap::<usize, BTreeSet<usize>>::complete(v);

            for s in 0..v {
                assert_eq!(graph.outdegree(s), v - 1);
            }
        }

        #[test]
        fn outdegree_hash_map_vec(v in 1..100_usize) {
            let graph = HashMap::<usize, Vec<usize>>::complete(v);

            for s in 0..v {
                assert_eq!(graph.outdegree(s), v - 1);
            }
        }

        #[test]
        fn outdegree_hash_map_hash_set(v in 1..100_usize) {
            let graph = HashMap::<usize, HashSet<usize>>::complete(v);

            for s in 0..v {
                assert_eq!(graph.outdegree(s), v - 1);
            }
        }
    }

    #[test]
    fn vec_vec() {
        for (v, g) in [
            Vec::new(),
            vec![Vec::new()],
            vec![vec![1], vec![0]],
            vec![vec![1, 2], vec![0, 2], vec![0, 1]],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<Vec<usize>>::complete(v), g);
        }
    }

    #[test]
    fn vec_btree_set() {
        for (v, g) in [
            Vec::new(),
            vec![BTreeSet::new()],
            vec![BTreeSet::from([1]), BTreeSet::from([0])],
            vec![
                BTreeSet::from([1, 2]),
                BTreeSet::from([0, 2]),
                BTreeSet::from([0, 1]),
            ],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<BTreeSet<usize>>::complete(v), g);
        }
    }

    #[test]
    fn vec_hash_set() {
        for (v, g) in [
            Vec::new(),
            vec![HashSet::new()],
            vec![HashSet::from([1]), HashSet::from([0])],
            vec![
                HashSet::from([1, 2]),
                HashSet::from([0, 2]),
                HashSet::from([0, 1]),
            ],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<HashSet<usize>>::complete(v), g);
        }
    }

    #[test]
    fn btree_map_vec() {
        for (v, g) in [
            BTreeMap::new(),
            BTreeMap::from([(0, Vec::new())]),
            BTreeMap::from([(0, vec![1]), (1, vec![0])]),
            BTreeMap::from([(0, vec![1, 2]), (1, vec![0, 2]), (2, vec![0, 1])]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&BTreeMap::<usize, Vec<usize>>::complete(v), g);
        }
    }

    #[test]
    fn btree_map_btree_set() {
        for (v, g) in [
            BTreeMap::new(),
            BTreeMap::from([(0, BTreeSet::new())]),
            BTreeMap::from([(0, BTreeSet::from([1])), (1, BTreeSet::from([0]))]),
            BTreeMap::from([
                (0, BTreeSet::from([1, 2])),
                (1, BTreeSet::from([0, 2])),
                (2, BTreeSet::from([0, 1])),
            ]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&BTreeMap::<usize, BTreeSet<usize>>::complete(v), g);
        }
    }

    #[test]
    fn hash_map_vec() {
        for (v, g) in [
            HashMap::new(),
            HashMap::from([(0, Vec::new())]),
            HashMap::from([(0, vec![1]), (1, vec![0])]),
            HashMap::from([(0, vec![1, 2]), (1, vec![0, 2]), (2, vec![0, 1])]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&HashMap::<usize, Vec<usize>>::complete(v), g);
        }
    }

    #[test]
    fn hash_map_hash_set() {
        for (v, g) in [
            HashMap::new(),
            HashMap::from([(0, HashSet::new())]),
            HashMap::from([(0, HashSet::from([1])), (1, HashSet::from([0]))]),
            HashMap::from([
                (0, HashSet::from([1, 2])),
                (1, HashSet::from([0, 2])),
                (2, HashSet::from([0, 1])),
            ]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&HashMap::<usize, HashSet<usize>>::complete(v), g);
        }
    }

    #[test]
    fn vec_tuple() {
        for (v, g) in [
            Vec::new(),
            Vec::new(),
            vec![(0, 1), (1, 0)],
            vec![(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<(usize, usize)>::complete(v), g);
        }
    }

    #[test]
    fn btree_set_tuple() {
        for (v, g) in [
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeSet::from([(0, 1), (1, 0)]),
            BTreeSet::from([(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&BTreeSet::<(usize, usize)>::complete(v), g);
        }
    }

    #[test]
    fn hash_set_tuple() {
        for (v, g) in [
            HashSet::new(),
            HashSet::new(),
            HashSet::from([(0, 1), (1, 0)]),
            HashSet::from([(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&HashSet::<(usize, usize)>::complete(v), g);
        }
    }
}

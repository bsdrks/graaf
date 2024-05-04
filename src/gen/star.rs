//! A trait to generate variable-sized star graphs
//!
//! # Examples
//!
//! ```
//! use graaf::gen::Star;
//!
//! assert_eq!(Vec::<Vec<usize>>::star(0), Vec::<Vec<usize>>::new());
//! assert_eq!(Vec::<Vec<usize>>::star(1), vec![Vec::new()]);
//! assert_eq!(Vec::<Vec<usize>>::star(2), vec![vec![1], vec![0]]);
//!
//! assert_eq!(
//!     Vec::<Vec<usize>>::star(3),
//!     vec![vec![1, 2], vec![0], vec![0]]
//! );
//!
//! assert_eq!(
//!     Vec::<Vec<usize>>::star(4),
//!     vec![vec![1, 2, 3], vec![0], vec![0], vec![0]]
//! );
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

/// A trait to generate variable-sized star graphs
///
/// # How can I implement `Star`?
///
/// Provide an implementation of `star` that generates a star graph with `v`
/// vertices.
///
/// ```
/// use {
///     graaf::gen::Star,
///     std::collections::HashSet,
/// };
///
/// struct Graph {
///     edges: HashSet<(usize, usize)>,
/// }
///
/// impl Star for Graph {
///     fn star(v: usize) -> Self {
///         let mut edges = HashSet::new();
///
///         for s in 1..v {
///             let _ = edges.insert((0, s));
///             let _ = edges.insert((s, 0));
///         }
///
///         Graph { edges }
///     }
/// }
///
/// let graph = Graph::star(5);
///
/// assert_eq!(
///     graph.edges,
///     HashSet::from([
///         (0, 1),
///         (1, 0),
///         (0, 2),
///         (2, 0),
///         (0, 3),
///         (3, 0),
///         (0, 4),
///         (4, 0)
///     ])
/// );
/// ```
pub trait Star {
    /// Generates a variable-sized star graph.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices in the graph
    fn star(v: usize) -> Self;
}

impl Star for Vec<Vec<usize>> {
    fn star(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        let mut graph = vec![Vec::new(); v];

        for s in 1..v {
            graph[0].push(s);
            graph[s].push(0);
        }

        graph
    }
}

impl Star for Vec<BTreeSet<usize>> {
    fn star(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        let mut graph = vec![BTreeSet::new(); v];

        for s in 1..v {
            let _ = graph[0].insert(s);
            let _ = graph[s].insert(0);
        }

        graph
    }
}

impl<H> Star for Vec<HashSet<usize, H>>
where
    H: BuildHasher + Default,
    HashSet<usize, H>: Clone,
{
    fn star(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        let mut graph = vec![HashSet::with_hasher(H::default()); v];

        for s in 1..v {
            let _ = graph[0].insert(s);
            let _ = graph[s].insert(0);
        }

        graph
    }
}

impl Star for BTreeMap<usize, Vec<usize>> {
    fn star(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        let mut graph = Self::new();

        for s in 1..v {
            graph.entry(0).or_insert_with(Vec::new).push(s);
            graph.entry(s).or_insert_with(Vec::new).push(0);
        }

        graph
    }
}

impl Star for BTreeMap<usize, BTreeSet<usize>> {
    fn star(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        let mut graph = Self::new();

        for s in 1..v {
            let _ = graph.entry(0).or_insert_with(BTreeSet::new).insert(s);
            let _ = graph.entry(s).or_insert_with(BTreeSet::new).insert(0);
        }

        graph
    }
}

impl<H> Star for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher + Default,
    Vec<usize>: Clone,
{
    fn star(v: usize) -> Self {
        if v == 0 {
            return Self::with_hasher(H::default());
        }

        let mut graph = Self::with_hasher(H::default());

        for s in 1..v {
            graph.entry(0).or_insert_with(Vec::new).push(s);
            graph.entry(s).or_insert_with(Vec::new).push(0);
        }

        graph
    }
}

impl<H> Star for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher + Default,
    HashSet<usize, H>: Clone,
{
    fn star(v: usize) -> Self {
        if v == 0 {
            return Self::with_hasher(H::default());
        }

        let mut graph = Self::with_hasher(H::default());

        for s in 1..v {
            let _ = graph
                .entry(0)
                .or_insert_with(|| HashSet::with_hasher(H::default()))
                .insert(s);

            let _ = graph
                .entry(s)
                .or_insert_with(|| HashSet::with_hasher(H::default()))
                .insert(0);
        }

        graph
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
        fn count_all_edges_vec_vec(v in 0..100_usize) {
            assert_eq!(
                Vec::<Vec<usize>>::star(v).count_all_edges(),
                v.saturating_sub(1) * 2
            );
        }

        #[test]
        fn count_all_edges_vec_btree_set(v in 0..100_usize) {
            assert_eq!(
                Vec::<BTreeSet<usize>>::star(v).count_all_edges(),
                v.saturating_sub(1) * 2
            );
        }

        #[test]
        fn count_all_edges_vec_hash_set(v in 0..100_usize) {
            assert_eq!(
                Vec::<HashSet<usize>>::star(v).count_all_edges(),
                v.saturating_sub(1) * 2
            );
        }

        #[test]
        fn count_all_edges_btree_map_vec(v in 0..100_usize) {
            assert_eq!(
                BTreeMap::<usize, Vec<usize>>::star(v).count_all_edges(),
                v.saturating_sub(1) * 2
            );
        }

        #[test]
        fn count_all_edges_btree_map_btree_set(v in 0..100_usize) {
            assert_eq!(
                BTreeMap::<usize, BTreeSet<usize>>::star(v).count_all_edges(),
                v.saturating_sub(1) * 2
            );
        }

        #[test]
        fn count_all_edges_hash_map_vec(v in 0..100_usize) {
            assert_eq!(
                HashMap::<usize, Vec<usize>>::star(v).count_all_edges(),
                v.saturating_sub(1) * 2
            );
        }

        #[test]
        fn count_all_edges_hash_map_hash_set(v in 0..100_usize) {
            assert_eq!(
                HashMap::<usize, HashSet<usize>>::star(v).count_all_edges(),
                v.saturating_sub(1) * 2
            );
        }

        #[test]
        fn count_all_vertices_vec_vec(v in 0..100_usize) {
            assert_eq!(Vec::<Vec<usize>>::star(v).count_all_vertices(), v);
        }

        #[test]
        fn count_all_vertices_vec_btree_set(v in 0..100_usize) {
            assert_eq!(Vec::<BTreeSet<usize>>::star(v).count_all_vertices(), v);
        }

        #[test]
        fn count_all_vertices_vec_hash_set(v in 1..100_usize) {
            assert_eq!(Vec::<HashSet<usize>>::star(v).count_all_vertices(), v);
        }

        #[test]
        fn indegree_vec_btree_set(v in 2..100_usize) {
            let graph = Vec::<BTreeSet<usize>>::star(v);

            assert_eq!(graph.indegree(0), v.saturating_sub(1));

            for s in 1..v {
                assert_eq!(graph.indegree(s), 1);
            }
        }

        #[test]
        fn indegree_vec_hash_set(v in 2..100_usize) {
            let graph = Vec::<HashSet<usize>>::star(v);

            assert_eq!(graph.indegree(0), v.saturating_sub(1));

            for s in 1..v {
                assert_eq!(graph.indegree(s), 1);
            }
        }

        #[test]
        fn indegree_btree_map_btree_set(v in 2..100_usize) {
            let graph = BTreeMap::<usize, BTreeSet<usize>>::star(v);

            assert_eq!(graph.indegree(0), v.saturating_sub(1));

            for s in 1..v {
                assert_eq!(graph.indegree(s), 1);
            }
        }

        #[test]
        fn indegree_hash_map_hash_set(v in 2..100_usize) {
            let graph = HashMap::<usize, HashSet<usize>>::star(v);

            assert_eq!(graph.indegree(0), v.saturating_sub(1));

            for s in 1..v {
                assert_eq!(graph.indegree(s), 1);
            }
        }

        #[test]
        fn is_simple_vec_btree_set(v in 0..100_usize) {
            assert!(Vec::<BTreeSet<usize>>::star(v).is_simple());
        }

        #[test]
        fn is_simple_vec_hash_set(v in 0..100_usize) {
            assert!(Vec::<HashSet<usize>>::star(v).is_simple());
        }

        #[test]
        fn outdegree_vec_vec(v in 2..100_usize) {
            let graph = Vec::<Vec<usize>>::star(v);

            assert_eq!(graph.outdegree(0), v.saturating_sub(1));

            for s in 1..v {
                assert_eq!(graph.outdegree(s), 1);
            }
        }

        #[test]
        fn outdegree_vec_btree_set(v in 2..100_usize) {
            let graph = Vec::<BTreeSet<usize>>::star(v);

            assert_eq!(graph.outdegree(0), v.saturating_sub(1));

            for s in 1..v {
                assert_eq!(graph.outdegree(s), 1);
            }
        }

        #[test]
        fn outdegree_vec_hash_set(v in 2..100_usize) {
            let graph = Vec::<HashSet<usize>>::star(v);

            assert_eq!(graph.outdegree(0), v.saturating_sub(1));

            for s in 1..v {
                assert_eq!(graph.outdegree(s), 1);
            }
        }

        #[test]
        fn outdegree_btree_map_vec(v in 2..100_usize) {
            let graph = BTreeMap::<usize, Vec<usize>>::star(v);

            assert_eq!(graph.outdegree(0), v.saturating_sub(1));

            for s in 1..v {
                assert_eq!(graph.outdegree(s), 1);
            }
        }

        #[test]
        fn outdegree_btree_map_btree_set(v in 2..100_usize) {
            let graph = BTreeMap::<usize, BTreeSet<usize>>::star(v);

            assert_eq!(graph.outdegree(0), v.saturating_sub(1));

            for s in 1..v {
                assert_eq!(graph.outdegree(s), 1);
            }
        }

        #[test]
        fn outdegree_hash_map_vec(v in 2..100_usize) {
            let graph = HashMap::<usize, Vec<usize>>::star(v);

            assert_eq!(graph.outdegree(0), v.saturating_sub(1));

            for s in 1..v {
                assert_eq!(graph.outdegree(s), 1);
            }
        }

        #[test]
        fn outdegree_hash_map_hash_set(v in 2..100_usize) {
            let graph = HashMap::<usize, HashSet<usize>>::star(v);

            assert_eq!(graph.outdegree(0), v.saturating_sub(1));

            for s in 1..v {
                assert_eq!(graph.outdegree(s), 1);
            }
        }
    }

    #[test]
    fn vec_vec() {
        for (v, g) in [
            Vec::new(),
            vec![Vec::new()],
            vec![vec![1], vec![0]],
            vec![vec![1, 2], vec![0], vec![0]],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<Vec<usize>>::star(v), g);
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
                BTreeSet::from([0]),
                BTreeSet::from([0]),
            ],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<BTreeSet<usize>>::star(v), g);
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
                HashSet::from([0]),
                HashSet::from([0]),
            ],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<HashSet<usize>>::star(v), g);
        }
    }

    #[test]
    fn btree_map_vec() {
        for (v, g) in [
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::from([(0, vec![1]), (1, vec![0])]),
            BTreeMap::from([(0, vec![1, 2]), (1, vec![0]), (2, vec![0])]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&BTreeMap::<usize, Vec<usize>>::star(v), g);
        }
    }

    #[test]
    fn btree_map_btree_set() {
        for (v, g) in [
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::from([(0, BTreeSet::from([1])), (1, BTreeSet::from([0]))]),
            BTreeMap::from([
                (0, BTreeSet::from([1, 2])),
                (1, BTreeSet::from([0])),
                (2, BTreeSet::from([0])),
            ]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&BTreeMap::<usize, BTreeSet<usize>>::star(v), g);
        }
    }

    #[test]
    fn hash_map_vec() {
        for (v, g) in [
            HashMap::new(),
            HashMap::new(),
            HashMap::from([(0, vec![1]), (1, vec![0])]),
            HashMap::from([(0, vec![1, 2]), (1, vec![0]), (2, vec![0])]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&HashMap::<usize, Vec<usize>>::star(v), g);
        }
    }

    #[test]
    fn hash_map_hash_set() {
        for (v, g) in [
            HashMap::new(),
            HashMap::new(),
            HashMap::from([(0, HashSet::from([1])), (1, HashSet::from([0]))]),
            HashMap::from([
                (0, HashSet::from([1, 2])),
                (1, HashSet::from([0])),
                (2, HashSet::from([0])),
            ]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&HashMap::<usize, HashSet<usize>>::star(v), g);
        }
    }
}

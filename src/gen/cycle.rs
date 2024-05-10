#![doc(alias = "circular")]
//! A trait to generate variable-sized directed cycle graphs
//!
//! Cycle graphs are also known as circular graphs. To generate const-sized
//! cycle graphs, see [`CycleConst`].
//!
//! # Examples
//!
//! ```
//! use graaf::gen::Cycle;
//!
//! assert!(Vec::<Vec<usize>>::cycle(0).is_empty());
//! assert_eq!(Vec::<Vec<usize>>::cycle(1), vec![vec![0]]);
//! assert_eq!(Vec::<Vec<usize>>::cycle(2), vec![vec![1], vec![0]]);
//! assert_eq!(Vec::<Vec<usize>>::cycle(3), vec![vec![1], vec![2], vec![0]]);
//! ```
//!
//! [`CycleConst`]: crate::gen::CycleConst

extern crate alloc;

use {
    crate::gen::Empty,
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

/// A trait to generate variable-size directed cycle graphs
///
/// # How can I implement `Cycle`?
///
/// Provide an implementation of `cycle` that generates a cycle graph with `v`
/// vertices.
///
/// ```
/// use {
///     graaf::gen::Cycle,
///     std::collections::HashSet,
/// };
///
/// struct Graph {
///     edges: HashSet<(usize, usize)>,
/// }
///
/// impl Cycle for Graph {
///     fn cycle(v: usize) -> Self {
///         let mut edges = (0..v - 1).map(|s| (s, s + 1)).collect::<HashSet<_>>();
///
///         edges.insert((v - 1, 0));
///
///         Graph { edges }
///     }
/// }
///
/// let graph = Graph::cycle(3);
///
/// assert_eq!(graph.edges, HashSet::from([(0, 1), (1, 2), (2, 0)]));
/// ```
pub trait Cycle {
    /// Generates a cycle graph.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices in the graph
    fn cycle(v: usize) -> Self;
}

impl Cycle for Vec<Vec<usize>> {
    fn cycle(v: usize) -> Self {
        let mut graph = Self::empty(v);

        if v == 0 {
            return graph;
        }

        for (s, vec) in graph.iter_mut().enumerate().take(v - 1) {
            vec.push(s + 1);
        }

        graph[v - 1].push(0);

        graph
    }
}

impl Cycle for Vec<BTreeSet<usize>> {
    fn cycle(v: usize) -> Self {
        let mut graph = Self::empty(v);

        if v == 0 {
            return graph;
        }

        for (s, set) in graph.iter_mut().enumerate().take(v - 1) {
            let _ = set.insert(s + 1);
        }

        let _ = graph[v - 1].insert(0);

        graph
    }
}

impl<H> Cycle for Vec<HashSet<usize, H>>
where
    H: BuildHasher + Default,
    HashSet<usize, H>: Clone,
{
    fn cycle(v: usize) -> Self {
        let mut graph = Self::empty(v);

        if v == 0 {
            return graph;
        }

        for (s, set) in graph.iter_mut().enumerate().take(v - 1) {
            let _ = set.insert(s + 1);
        }

        let _ = graph[v - 1].insert(0);

        graph
    }
}

impl Cycle for BTreeMap<usize, Vec<usize>> {
    fn cycle(v: usize) -> Self {
        let mut graph = Self::empty(v);

        if v == 0 {
            return graph;
        }

        for s in 0..v - 1 {
            let _ = graph.insert(s, vec![s + 1]);
        }

        let _ = graph.insert(v - 1, vec![0]);

        graph
    }
}

impl Cycle for BTreeMap<usize, BTreeSet<usize>> {
    fn cycle(v: usize) -> Self {
        let mut graph = Self::empty(v);

        if v == 0 {
            return graph;
        }

        for s in 0..v - 1 {
            let _ = graph.insert(s, BTreeSet::from([s + 1]));
        }

        let _ = graph.insert(v - 1, BTreeSet::from([0]));

        graph
    }
}

impl<H> Cycle for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher + Default,
{
    fn cycle(v: usize) -> Self {
        let mut graph = Self::empty(v);

        if v == 0 {
            return graph;
        }

        for s in 0..v - 1 {
            let _ = graph.insert(s, vec![s + 1]);
        }

        let _ = graph.insert(v - 1, vec![0]);

        graph
    }
}

impl<H> Cycle for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher + Default,
{
    fn cycle(v: usize) -> Self {
        let mut graph = Self::empty(v);

        if v == 0 {
            return graph;
        }

        for s in 0..v - 1 {
            let mut set = HashSet::with_hasher(H::default());
            let _ = set.insert(s + 1);
            let _ = graph.insert(s, set);
        }

        let mut set = HashSet::with_hasher(H::default());
        let _ = set.insert(0);
        let _ = graph.insert(v - 1, set);

        graph
    }
}

impl Cycle for Vec<(usize, usize)> {
    fn cycle(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        (0..v).map(|s| (s, (s + 1) % v)).collect()
    }
}

impl Cycle for BTreeSet<(usize, usize)> {
    fn cycle(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        (0..v).map(|s| (s, (s + 1) % v)).collect()
    }
}

impl<H> Cycle for HashSet<(usize, usize), H>
where
    H: BuildHasher + Default,
{
    fn cycle(v: usize) -> Self {
        if v == 0 {
            return Self::empty(v);
        }

        (0..v).map(|s| (s, (s + 1) % v)).collect()
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

    fn prop_count_all_edges<T: Cycle + CountAllEdges>(v: usize) {
        assert_eq!(T::cycle(v).count_all_edges(), v);
    }

    fn prop_count_all_vertices<T: Cycle + CountAllVertices>(v: usize) {
        assert_eq!(T::cycle(v).count_all_vertices(), v);
    }

    fn prop_indegree<T: Cycle + Indegree>(v: usize) {
        let graph = T::cycle(v);

        for s in 0..v {
            assert_eq!(graph.indegree(s), 1);
        }
    }

    fn prop_is_simple<T: Cycle + IsSimple>(v: usize) {
        assert!(T::cycle(v).is_simple());
    }

    fn prop_outdegree<T: Cycle + Outdegree>(v: usize) {
        let graph = T::cycle(v);

        for s in 0..v {
            assert_eq!(graph.outdegree(s), 1);
        }
    }

    proptest! {
        #[test]
        fn count_all_edges_vec_vec(v in 0..100_usize) {
            prop_count_all_edges::<Vec<Vec<usize>>>(v);
        }

        #[test]
        fn count_all_edges_vec_btree_set(v in 0..100_usize) {
            prop_count_all_edges::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn count_all_edges_vec_hash_set(v in 0..100_usize) {
            prop_count_all_edges::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn count_all_edges_btree_map_vec(v in 0..100_usize) {
            prop_count_all_edges::<BTreeMap<usize, Vec<usize>>>(v);
        }

        #[test]
        fn count_all_edges_btree_map_btree_set(v in 0..100_usize) {
            prop_count_all_edges::<BTreeMap<usize, BTreeSet<usize>>>(v);
        }

        #[test]
        fn count_all_edges_hash_map_vec(v in 0..100_usize) {
            prop_count_all_edges::<HashMap<usize, Vec<usize>>>(v);
        }

        #[test]
        fn count_all_edges_hash_map_hash_set(v in 0..100_usize) {
            prop_count_all_edges::<HashMap<usize, HashSet<usize>>>(v);
        }

        #[test]
        fn count_all_edges_vec_tuple(v in 0..100_usize) {
            prop_count_all_edges::<Vec<(usize, usize)>>(v);
        }

        #[test]
        fn count_all_edges_btree_set_tuple(v in 0..100_usize) {
            prop_count_all_edges::<BTreeSet<(usize, usize)>>(v);
        }

        #[test]
        fn count_all_edges_hash_set_tuple(v in 0..100_usize) {
            prop_count_all_edges::<HashSet<(usize, usize)>>(v);
        }

        #[test]
        fn count_all_vertices_vec_vec(v in 0..100_usize) {
            prop_count_all_vertices::<Vec<Vec<usize>>>(v);
        }

        #[test]
        fn count_all_vertices_vec_btree_set(v in 0..100_usize) {
            prop_count_all_vertices::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn count_all_vertices_vec_hash_set(v in 0..100_usize) {
            prop_count_all_vertices::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn indegree_vec_btree_set(v in 1..100_usize) {
            prop_indegree::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn indegree_vec_hash_set(v in 1..100_usize) {
            prop_indegree::<Vec<HashSet<usize>>>(v);
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
        fn is_simple_vec_btree_set(v in 2..100_usize) {
            prop_is_simple::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn is_simple_vec_hash_set(v in 2..100_usize) {
            prop_is_simple::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn is_simple_vec_tuple(v in 2..100_usize) {
            prop_is_simple::<Vec<(usize, usize)>>(v);
        }

        #[test]
        fn is_simple_btree_set_tuple(v in 2..100_usize) {
            prop_is_simple::<BTreeSet<(usize, usize)>>(v);
        }

        #[test]
        fn is_simple_hash_set_tuple(v in 2..100_usize) {
            prop_is_simple::<HashSet<(usize, usize)>>(v);
        }

        #[test]
        fn outdegree_vec_vec(v in 1..100_usize) {
            prop_outdegree::<Vec<Vec<usize>>>(v);
        }

        #[test]
        fn outdegree_vec_btree_set(v in 1..100_usize) {
            prop_outdegree::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn outdegree_vec_hash_set(v in 1..100_usize) {
            prop_outdegree::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn outdegree_btree_map_vec(v in 1..100_usize) {
            prop_outdegree::<BTreeMap<usize, Vec<usize>>>(v);
        }

        #[test]
        fn outdegree_btree_map_btree_set(v in 1..100_usize) {
            prop_outdegree::<BTreeMap<usize, BTreeSet<usize>>>(v);
        }

        #[test]
        fn outdegree_hash_map_vec(v in 1..100_usize) {
            prop_outdegree::<HashMap<usize, Vec<usize>>>(v);
        }

        #[test]
        fn outdegree_hash_map_hash_set(v in 1..100_usize) {
            prop_outdegree::<HashMap<usize, HashSet<usize>>>(v);
        }
    }

    #[test]
    fn vec_vec() {
        for (v, g) in [
            Vec::new(),
            vec![vec![0]],
            vec![vec![1], vec![0]],
            vec![vec![1], vec![2], vec![0]],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<Vec<usize>>::cycle(v), g);
        }
    }

    #[test]
    fn vec_btree_set() {
        for (v, g) in [
            Vec::new(),
            vec![BTreeSet::from([0])],
            vec![BTreeSet::from([1]), BTreeSet::from([0])],
            vec![
                BTreeSet::from([1]),
                BTreeSet::from([2]),
                BTreeSet::from([0]),
            ],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<BTreeSet<usize>>::cycle(v), g);
        }
    }

    #[test]
    fn vec_hash_set() {
        for (v, g) in [
            Vec::new(),
            vec![HashSet::from([0])],
            vec![HashSet::from([1]), HashSet::from([0])],
            vec![HashSet::from([1]), HashSet::from([2]), HashSet::from([0])],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<HashSet<usize>>::cycle(v), g);
        }
    }

    #[test]
    fn btree_map_vec() {
        for (v, g) in [
            BTreeMap::new(),
            BTreeMap::from([(0, vec![0])]),
            BTreeMap::from([(0, vec![1]), (1, vec![0])]),
            BTreeMap::from([(0, vec![1]), (1, vec![2]), (2, vec![0])]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&BTreeMap::<usize, Vec<usize>>::cycle(v), g);
        }
    }

    #[test]
    fn btree_map_btree_set() {
        for (v, g) in [
            BTreeMap::new(),
            BTreeMap::from([(0, BTreeSet::from([0]))]),
            BTreeMap::from([(0, BTreeSet::from([1])), (1, BTreeSet::from([0]))]),
            BTreeMap::from([
                (0, BTreeSet::from([1])),
                (1, BTreeSet::from([2])),
                (2, BTreeSet::from([0])),
            ]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&BTreeMap::<usize, BTreeSet<usize>>::cycle(v), g);
        }
    }

    #[test]
    fn hash_map_vec() {
        for (v, g) in [
            HashMap::new(),
            HashMap::from([(0, vec![0])]),
            HashMap::from([(0, vec![1]), (1, vec![0])]),
            HashMap::from([(0, vec![1]), (1, vec![2]), (2, vec![0])]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&HashMap::<usize, Vec<usize>>::cycle(v), g);
        }
    }

    #[test]
    fn hash_map_hash_set() {
        for (v, g) in [
            HashMap::new(),
            HashMap::from([(0, HashSet::from([0]))]),
            HashMap::from([(0, HashSet::from([1])), (1, HashSet::from([0]))]),
            HashMap::from([
                (0, HashSet::from([1])),
                (1, HashSet::from([2])),
                (2, HashSet::from([0])),
            ]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&HashMap::<usize, HashSet<usize>>::cycle(v), g);
        }
    }

    #[test]
    fn vec_tuple() {
        for (v, g) in [
            Vec::new(),
            vec![(0, 0)],
            vec![(0, 1), (1, 0)],
            vec![(0, 1), (1, 2), (2, 0)],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<(usize, usize)>::cycle(v), g);
        }
    }

    #[test]
    fn btree_set_tuple() {
        for (v, g) in [
            BTreeSet::new(),
            BTreeSet::from([(0, 0)]),
            BTreeSet::from([(0, 1), (1, 0)]),
            BTreeSet::from([(0, 1), (1, 2), (2, 0)]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&BTreeSet::<(usize, usize)>::cycle(v), g);
        }
    }

    #[test]
    fn hash_set_tuple() {
        for (v, g) in [
            HashSet::new(),
            HashSet::from([(0, 0)]),
            HashSet::from([(0, 1), (1, 0)]),
            HashSet::from([(0, 1), (1, 2), (2, 0)]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&HashSet::<(usize, usize)>::cycle(v), g);
        }
    }

    #[test]
    fn is_simple_vec_btree_set_0() {
        assert!(Vec::<BTreeSet<usize>>::cycle(0).is_simple());
    }

    #[test]
    fn is_simple_vec_hash_set_0() {
        assert!(Vec::<HashSet<usize>>::cycle(0).is_simple());
    }

    #[test]
    fn is_simple_vec_tuple_0() {
        assert!(Vec::<(usize, usize)>::cycle(0).is_simple());
    }

    #[test]
    fn is_simple_btree_set_tuple_0() {
        assert!(BTreeSet::<(usize, usize)>::cycle(0).is_simple());
    }

    #[test]
    fn is_simple_hash_set_tuple_0() {
        assert!(HashSet::<(usize, usize)>::cycle(0).is_simple());
    }

    #[test]
    fn is_simple_vec_btree_set_1() {
        assert!(!Vec::<BTreeSet<usize>>::cycle(1).is_simple());
    }

    #[test]
    fn is_simple_vec_hash_set_1() {
        assert!(!Vec::<HashSet<usize>>::cycle(1).is_simple());
    }

    #[test]
    fn is_simple_vec_tuple_1() {
        assert!(!Vec::<(usize, usize)>::cycle(1).is_simple());
    }

    #[test]
    fn is_simple_btree_set_tuple_1() {
        assert!(!BTreeSet::<(usize, usize)>::cycle(1).is_simple());
    }

    #[test]
    fn is_simple_hash_set_tuple_1() {
        assert!(!HashSet::<(usize, usize)>::cycle(1).is_simple());
    }
}

//! A trait to determine whether a directed graph is regular
//!
//! A directed graph is regular if all vertices have the same indegree and
//! outdegree.
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::{
//!         gen::Cycle,
//!         op::{
//!             IsRegular,
//!             RemoveEdge,
//!         },
//!     },
//!     std::collections::BTreeSet,
//! };
//!
//! let mut graph = Vec::<BTreeSet<usize>>::cycle(7);
//!
//! assert!(graph.is_regular());
//!
//! graph.remove_edge(6, 0);
//!
//! assert!(!graph.is_regular());
//! ```

extern crate alloc;

use {
    crate::op::{
        Indegree,
        IterVertices,
        Outdegree,
    },
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

/// A trait to determine whether a directed graph is regular
///
/// # How can I implement `IsRegular`?
///
/// Provide an implementation of `is_regular` that returns `true` if the graph
/// is regular and `false` otherwise.
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::op::{
///         Indegree,
///         IsRegular,
///         IterVertices,
///         Outdegree,
///     },
/// };
///
/// struct Graph {
///     edges: Vec<BTreeSet<usize>>,
/// }
///
/// impl IsRegular for Graph {
///     /// # Panics
///     ///
///     /// Panics if the graph has no vertices.
///     fn is_regular(&self) -> bool {
///         let mut vertices = self.edges.iter_vertices();
///
///         let v = vertices
///             .next()
///             .expect("a graph must have at least one vertex");
///
///         let indegree = self.edges.indegree(v);
///         let outdegree = self.edges.outdegree(v);
///
///         vertices
///             .all(|v| self.edges.indegree(v) == indegree && self.edges.outdegree(v) == outdegree)
///     }
/// }
/// ```
pub trait IsRegular {
    /// Returns whether the graph is regular.
    fn is_regular(&self) -> bool;
}

impl IsRegular for Vec<BTreeSet<usize>> {
    /// # Panics
    ///
    /// Panics if the graph has no vertices.
    fn is_regular(&self) -> bool {
        let mut vertices = self.iter_vertices();

        let v = vertices
            .next()
            .expect("a graph must have at least one vertex");

        let indegree = self.indegree(v);
        let outdegree = self.outdegree(v);

        vertices.all(|v| self.indegree(v) == indegree && self.outdegree(v) == outdegree)
    }
}

impl<H> IsRegular for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if the graph has no vertices.
    fn is_regular(&self) -> bool {
        let mut vertices = self.iter_vertices();

        let v = vertices
            .next()
            .expect("a graph must have at least one vertex");

        let indegree = self.indegree(v);
        let outdegree = self.outdegree(v);

        vertices.all(|v| self.indegree(v) == indegree && self.outdegree(v) == outdegree)
    }
}

impl IsRegular for [BTreeSet<usize>] {
    /// # Panics
    ///
    /// Panics if the graph has no vertices.
    fn is_regular(&self) -> bool {
        let mut vertices = self.iter_vertices();

        let v = vertices
            .next()
            .expect("a graph must have at least one vertex");

        let indegree = self.indegree(v);
        let outdegree = self.outdegree(v);

        vertices.all(|v| self.indegree(v) == indegree && self.outdegree(v) == outdegree)
    }
}

impl<H> IsRegular for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if the graph has no vertices.
    fn is_regular(&self) -> bool {
        let mut vertices = self.iter_vertices();

        let v = vertices
            .next()
            .expect("a graph must have at least one vertex");

        let indegree = self.indegree(v);
        let outdegree = self.outdegree(v);

        vertices.all(|v| self.indegree(v) == indegree && self.outdegree(v) == outdegree)
    }
}

impl<const V: usize> IsRegular for [BTreeSet<usize>; V] {
    /// # Panics
    ///
    /// Panics if the graph has no vertices.
    fn is_regular(&self) -> bool {
        let mut vertices = self.iter_vertices();

        let v = vertices
            .next()
            .expect("a graph must have at least one vertex");

        let indegree = self.indegree(v);
        let outdegree = self.outdegree(v);

        vertices.all(|v| self.indegree(v) == indegree && self.outdegree(v) == outdegree)
    }
}

impl<const V: usize, H> IsRegular for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if the graph has no vertices.
    fn is_regular(&self) -> bool {
        let mut vertices = self.iter_vertices();

        let v = vertices
            .next()
            .expect("a graph must have at least one vertex");

        let indegree = self.indegree(v);
        let outdegree = self.outdegree(v);

        vertices.all(|v| self.indegree(v) == indegree && self.outdegree(v) == outdegree)
    }
}

impl IsRegular for BTreeMap<usize, BTreeSet<usize>> {
    /// # Panics
    ///
    /// Panics if the graph has no vertices.
    fn is_regular(&self) -> bool {
        let mut vertices = self.iter_vertices();

        let v = vertices
            .next()
            .expect("a graph must have at least one vertex");

        let indegree = self.indegree(v);
        let outdegree = self.outdegree(v);

        vertices.all(|v| self.indegree(v) == indegree && self.outdegree(v) == outdegree)
    }
}

impl<H> IsRegular for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if the graph has no vertices.
    fn is_regular(&self) -> bool {
        let mut vertices = self.iter_vertices();

        let v = vertices
            .next()
            .expect("a graph must have at least one vertex");

        let indegree = self.indegree(v);
        let outdegree = self.outdegree(v);

        vertices.all(|v| self.indegree(v) == indegree && self.outdegree(v) == outdegree)
    }
}

impl<W> IsRegular for Vec<BTreeMap<usize, W>> {
    /// # Panics
    ///
    /// Panics if the graph has no vertices.
    fn is_regular(&self) -> bool {
        let mut vertices = self.iter_vertices();

        let v = vertices
            .next()
            .expect("a graph must have at least one vertex");

        let indegree = self.indegree(v);
        let outdegree = self.outdegree(v);

        vertices.all(|v| self.indegree(v) == indegree && self.outdegree(v) == outdegree)
    }
}

impl<W, H> IsRegular for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if the graph has no vertices.
    fn is_regular(&self) -> bool {
        let mut vertices = self.iter_vertices();

        let v = vertices
            .next()
            .expect("a graph must have at least one vertex");

        let indegree = self.indegree(v);
        let outdegree = self.outdegree(v);

        vertices.all(|v| self.indegree(v) == indegree && self.outdegree(v) == outdegree)
    }
}

impl<W> IsRegular for [BTreeMap<usize, W>] {
    /// # Panics
    ///
    /// Panics if the graph has no vertices.
    fn is_regular(&self) -> bool {
        let mut vertices = self.iter_vertices();

        let v = vertices
            .next()
            .expect("a graph must have at least one vertex");

        let indegree = self.indegree(v);
        let outdegree = self.outdegree(v);

        vertices.all(|v| self.indegree(v) == indegree && self.outdegree(v) == outdegree)
    }
}

impl<W, H> IsRegular for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if the graph has no vertices.
    fn is_regular(&self) -> bool {
        let mut vertices = self.iter_vertices();

        let v = vertices
            .next()
            .expect("a graph must have at least one vertex");

        let indegree = self.indegree(v);
        let outdegree = self.outdegree(v);

        vertices.all(|v| self.indegree(v) == indegree && self.outdegree(v) == outdegree)
    }
}

impl<const V: usize, W> IsRegular for [BTreeMap<usize, W>; V] {
    /// # Panics
    ///
    /// Panics if the graph has no vertices.
    fn is_regular(&self) -> bool {
        let mut vertices = self.iter_vertices();

        let v = vertices
            .next()
            .expect("a graph must have at least one vertex");

        let indegree = self.indegree(v);
        let outdegree = self.outdegree(v);

        vertices.all(|v| self.indegree(v) == indegree && self.outdegree(v) == outdegree)
    }
}

impl<const V: usize, W, H> IsRegular for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if the graph has no vertices.
    fn is_regular(&self) -> bool {
        let mut vertices = self.iter_vertices();

        let v = vertices
            .next()
            .expect("a graph must have at least one vertex");

        let indegree = self.indegree(v);
        let outdegree = self.outdegree(v);

        vertices.all(|v| self.indegree(v) == indegree && self.outdegree(v) == outdegree)
    }
}

impl<W> IsRegular for BTreeMap<usize, BTreeMap<usize, W>> {
    /// # Panics
    ///
    /// Panics if the graph has no vertices.
    fn is_regular(&self) -> bool {
        let mut vertices = self.iter_vertices();

        let v = vertices
            .next()
            .expect("a graph must have at least one vertex");

        let indegree = self.indegree(v);
        let outdegree = self.outdegree(v);

        vertices.all(|v| self.indegree(v) == indegree && self.outdegree(v) == outdegree)
    }
}

impl<W, H> IsRegular for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if the graph has no vertices.
    fn is_regular(&self) -> bool {
        let mut vertices = self.iter_vertices();

        let v = vertices
            .next()
            .expect("a graph must have at least one vertex");

        let indegree = self.indegree(v);
        let outdegree = self.outdegree(v);

        vertices.all(|v| self.indegree(v) == indegree && self.outdegree(v) == outdegree)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            gen::{
                Cycle,
                CycleConst,
                Empty,
            },
            op::RemoveEdge,
        },
        proptest::proptest,
    };

    macro_rules! test_is_regular {
        ($graph:expr) => {
            assert!($graph.is_regular());

            let _ = $graph.remove_edge(2, 0);

            assert!(!$graph.is_regular());
        };
    }

    proptest! {
        #[test]
        fn empty_vec_btree_set_unweighted(v in 1..100_usize) {
            let graph = Vec::<BTreeSet<usize>>::empty(v);

            assert!(graph.is_regular());
        }

        #[test]
        fn empty_vec_hash_set_unweighted(v in 1..100_usize) {
            let graph = Vec::<HashSet<usize>>::empty(v);

            assert!(graph.is_regular());
        }

        #[test]
        fn empty_btree_map_btree_set_unweighted(v in 1..100_usize) {
            let graph = BTreeMap::<usize, BTreeSet<usize>>::empty(v);

            assert!(graph.is_regular());
        }

        #[test]
        fn empty_hash_map_hash_set_unweighted(v in 1..100_usize) {
            let graph = HashMap::<usize, HashSet<usize>>::empty(v);

            assert!(graph.is_regular());
        }

        #[test]
        fn empty_vec_btree_map(v in 1..100_usize) {
            let graph = Vec::<BTreeMap<usize, usize>>::empty(v);

            assert!(graph.is_regular());
        }

        #[test]
        fn empty_vec_hash_map(v in 1..100_usize) {
            let graph = Vec::<HashMap<usize, usize>>::empty(v);

            assert!(graph.is_regular());
        }

        #[test]
        fn empty_btree_map_btree_map(v in 1..100_usize) {
            let graph = BTreeMap::<usize, BTreeMap<usize, usize>>::empty(v);

            assert!(graph.is_regular());
        }

        #[test]
        fn empty_hash_map_hash_map(v in 1..100_usize) {
            let graph = HashMap::<usize, HashMap<usize, usize>>::empty(v);

            assert!(graph.is_regular());
        }
    }

    #[test]
    fn vec_btree_set_unweighted() {
        let graph = &mut <Vec<BTreeSet<usize>>>::cycle(3);

        test_is_regular!(graph);
    }

    #[test]
    fn vec_hash_set_unweighted() {
        let graph = &mut <Vec<HashSet<usize>>>::cycle(3);

        test_is_regular!(graph);
    }

    #[test]
    fn slice_btree_set_unweighted() {
        let graph: &mut [BTreeSet<usize>] = &mut Vec::<BTreeSet<usize>>::cycle(3);

        test_is_regular!(graph);
    }

    #[test]
    fn slice_hash_set_unweighted() {
        let graph: &mut [HashSet<usize>] = &mut Vec::<HashSet<usize>>::cycle(3);

        test_is_regular!(graph);
    }

    #[test]
    fn arr_btree_set_unweighted() {
        let graph = &mut <[BTreeSet<usize>; 3]>::cycle();

        test_is_regular!(graph);
    }

    #[test]
    fn arr_hash_set_unweighted() {
        let graph = &mut <[HashSet<usize>; 3]>::cycle();

        test_is_regular!(graph);
    }

    #[test]
    fn btree_map_btree_set() {
        let mut graph = BTreeMap::<usize, BTreeSet<usize>>::cycle(3);

        test_is_regular!(graph);
    }

    #[test]
    fn hash_map_hash_set() {
        let mut graph = HashMap::<usize, HashSet<usize>>::cycle(3);

        test_is_regular!(graph);
    }

    #[test]
    fn vec_btree_map() {
        let mut graph = vec![
            BTreeMap::from([(1, 4)]),
            BTreeMap::from([(2, 3)]),
            BTreeMap::from([(0, 3)]),
        ];

        test_is_regular!(graph);
    }

    #[test]
    fn vec_hash_map() {
        let mut graph = vec![
            HashMap::from([(1, 4)]),
            HashMap::from([(2, 3)]),
            HashMap::from([(0, 3)]),
        ];

        test_is_regular!(graph);
    }

    #[test]
    fn slice_btree_map() {
        let graph: &mut [BTreeMap<usize, usize>] = &mut [
            BTreeMap::from([(1, 4)]),
            BTreeMap::from([(2, 3)]),
            BTreeMap::from([(0, 3)]),
        ];

        test_is_regular!(graph);
    }

    #[test]
    fn slice_hash_map() {
        let graph: &mut [HashMap<usize, usize>] = &mut [
            HashMap::from([(1, 4)]),
            HashMap::from([(2, 3)]),
            HashMap::from([(0, 3)]),
        ];

        test_is_regular!(graph);
    }

    #[test]
    fn arr_btree_map() {
        let mut graph = [
            BTreeMap::from([(1, 4)]),
            BTreeMap::from([(2, 3)]),
            BTreeMap::from([(0, 3)]),
        ];

        test_is_regular!(graph);
    }

    #[test]
    fn arr_hash_map() {
        let mut graph = [
            HashMap::from([(1, 4)]),
            HashMap::from([(2, 3)]),
            HashMap::from([(0, 3)]),
        ];

        test_is_regular!(graph);
    }

    #[test]
    fn btree_map_btree_map() {
        let mut graph = BTreeMap::from([
            (0, BTreeMap::from([(1, 4)])),
            (1, BTreeMap::from([(2, 3)])),
            (2, BTreeMap::from([(0, 3)])),
        ]);

        test_is_regular!(graph);
    }

    #[test]
    fn hash_map_hash_map() {
        let mut graph = HashMap::from([
            (0, HashMap::from([(1, 4)])),
            (1, HashMap::from([(2, 3)])),
            (2, HashMap::from([(0, 3)])),
        ]);

        test_is_regular!(graph);
    }
}

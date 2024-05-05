//! A trait to generate const-sized complete directed graphs
//!
//! The generated graphs are simple; they contain no self-loops. To generate
//! variable-sized complete graphs, see [`Complete`].
//!
//! # Examples
//!
//! ```
//! use graaf::gen::CompleteConst;
//!
//! assert!(<[Vec::<usize>; 0]>::complete().is_empty());
//! assert_eq!(<[Vec::<usize>; 1]>::complete(), [Vec::new()]);
//! assert_eq!(<[Vec::<usize>; 2]>::complete(), [vec![1], vec![0]]);
//!
//! assert_eq!(
//!     <[Vec::<usize>; 3]>::complete(),
//!     [vec![1, 2], vec![0, 2], vec![0, 1]]
//! );
//! ```
//!
//! [`Complete`]: crate::gen::Complete

extern crate alloc;

use {
    alloc::collections::BTreeSet,
    core::{
        array::from_fn,
        hash::BuildHasher,
    },
    std::collections::HashSet,
};

/// A trait to generate const-sized complete directed graphs
///
/// # How can I implement `CompleteConst`?
///
/// Provide an implementation of `complete` that generates a complete graph.
///
/// ```
/// use {
///     core::{
///         array::from_fn,
///         hash::BuildHasher,
///     },
///     graaf::gen::CompleteConst,
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
/// impl<const V: usize, H> CompleteConst for Graph<V, H>
/// where
///     H: BuildHasher + Default,
/// {
///     fn complete() -> Self {
///         let mut edges = from_fn(|_| HashSet::with_hasher(H::default()));
///
///         if V == 0 {
///             return Graph { edges };
///         }
///
///         for (s, set) in edges.iter_mut().enumerate() {
///             for t in 0..V {
///                 if s != t {
///                     let _ = set.insert(t);
///                 }
///             }
///         }
///
///         Graph { edges }
///     }
/// }
///
/// let graph = Graph::<3, RandomState>::complete();
///
/// assert_eq!(
///     graph.edges,
///     [
///         HashSet::from([1, 2]),
///         HashSet::from([0, 2]),
///         HashSet::from([0, 1])
///     ]
/// );
/// ```
#[doc(alias = "circular")]
pub trait CompleteConst {
    /// Generates a complete graph.
    fn complete() -> Self;
}

impl<const V: usize> CompleteConst for [Vec<usize>; V] {
    fn complete() -> Self {
        let mut graph: [Vec<usize>; V] = from_fn(|_| Vec::new());

        if V == 0 {
            return graph;
        }

        for (s, vec) in graph.iter_mut().enumerate().take(V) {
            for t in 0..V {
                if s != t {
                    vec.push(t);
                }
            }
        }

        graph
    }
}

impl<const V: usize> CompleteConst for [BTreeSet<usize>; V] {
    fn complete() -> Self {
        let mut graph: [BTreeSet<usize>; V] = from_fn(|_| BTreeSet::new());

        if V == 0 {
            return graph;
        }

        for (s, set) in graph.iter_mut().enumerate().take(V) {
            for t in 0..V {
                if s != t {
                    let _ = set.insert(t);
                }
            }
        }

        graph
    }
}

impl<const V: usize, H> CompleteConst for [HashSet<usize, H>; V]
where
    H: BuildHasher + Default,
{
    fn complete() -> Self {
        let mut graph: [HashSet<usize, H>; V] = from_fn(|_| HashSet::with_hasher(H::default()));

        if V == 0 {
            return graph;
        }

        for (s, set) in graph.iter_mut().enumerate().take(V) {
            for t in 0..V {
                if s != t {
                    let _ = set.insert(t);
                }
            }
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
        },
    };

    #[test]
    fn arr_vec() {
        assert!(<[Vec::<usize>; 0]>::complete().is_empty());
        assert_eq!(<[Vec::<usize>; 1]>::complete(), [Vec::new()]);
        assert_eq!(<[Vec::<usize>; 2]>::complete(), [vec![1], vec![0]]);

        assert_eq!(
            <[Vec::<usize>; 3]>::complete(),
            [vec![1, 2], vec![0, 2], vec![0, 1]]
        );
    }

    #[test]
    fn arr_btree_set() {
        assert!(<[BTreeSet::<usize>; 0]>::complete().is_empty());
        assert_eq!(<[BTreeSet::<usize>; 1]>::complete(), [BTreeSet::new()]);

        assert_eq!(
            <[BTreeSet::<usize>; 2]>::complete(),
            [BTreeSet::from([1]), BTreeSet::from([0])]
        );

        assert_eq!(
            <[BTreeSet::<usize>; 3]>::complete(),
            [
                BTreeSet::from([1, 2]),
                BTreeSet::from([0, 2]),
                BTreeSet::from([0, 1])
            ]
        );
    }

    #[test]
    fn arr_hash_set() {
        assert!(<[HashSet::<usize>; 0]>::complete().is_empty());
        assert_eq!(<[HashSet::<usize>; 1]>::complete(), [HashSet::new()]);

        assert_eq!(
            <[HashSet::<usize>; 2]>::complete(),
            [HashSet::from([1]), HashSet::from([0])]
        );

        assert_eq!(
            <[HashSet::<usize>; 3]>::complete(),
            [
                HashSet::from([1, 2]),
                HashSet::from([0, 2]),
                HashSet::from([0, 1])
            ]
        );
    }

    #[test]
    fn count_all_edges_arr_vec() {
        assert_eq!(<[Vec<usize>; 0]>::complete().count_all_edges(), 0);
        assert_eq!(<[Vec<usize>; 1]>::complete().count_all_edges(), 0);
        assert_eq!(<[Vec<usize>; 2]>::complete().count_all_edges(), 2);
        assert_eq!(<[Vec<usize>; 3]>::complete().count_all_edges(), 6);
        assert_eq!(<[Vec<usize>; 4]>::complete().count_all_edges(), 12);
        assert_eq!(<[Vec<usize>; 5]>::complete().count_all_edges(), 20);
    }

    #[test]
    fn count_all_edges_arr_btree_set() {
        assert_eq!(<[BTreeSet<usize>; 0]>::complete().count_all_edges(), 0);
        assert_eq!(<[BTreeSet<usize>; 1]>::complete().count_all_edges(), 0);
        assert_eq!(<[BTreeSet<usize>; 2]>::complete().count_all_edges(), 2);
        assert_eq!(<[BTreeSet<usize>; 3]>::complete().count_all_edges(), 6);
        assert_eq!(<[BTreeSet<usize>; 4]>::complete().count_all_edges(), 12);
        assert_eq!(<[BTreeSet<usize>; 5]>::complete().count_all_edges(), 20);
    }

    #[test]
    fn count_all_edges_arr_hash_set() {
        assert_eq!(<[HashSet<usize>; 0]>::complete().count_all_edges(), 0);
        assert_eq!(<[HashSet<usize>; 1]>::complete().count_all_edges(), 0);
        assert_eq!(<[HashSet<usize>; 2]>::complete().count_all_edges(), 2);
        assert_eq!(<[HashSet<usize>; 3]>::complete().count_all_edges(), 6);
        assert_eq!(<[HashSet<usize>; 4]>::complete().count_all_edges(), 12);
        assert_eq!(<[HashSet<usize>; 5]>::complete().count_all_edges(), 20);
    }

    #[test]
    fn count_all_vertices_arr_vec() {
        assert_eq!(<[Vec<usize>; 0]>::complete().count_all_vertices(), 0);
        assert_eq!(<[Vec<usize>; 1]>::complete().count_all_vertices(), 1);
        assert_eq!(<[Vec<usize>; 2]>::complete().count_all_vertices(), 2);
        assert_eq!(<[Vec<usize>; 3]>::complete().count_all_vertices(), 3);
        assert_eq!(<[Vec<usize>; 4]>::complete().count_all_vertices(), 4);
        assert_eq!(<[Vec<usize>; 5]>::complete().count_all_vertices(), 5);
    }

    #[test]
    fn count_all_vertices_arr_btree_set() {
        assert_eq!(<[BTreeSet<usize>; 0]>::complete().count_all_vertices(), 0);
        assert_eq!(<[BTreeSet<usize>; 1]>::complete().count_all_vertices(), 1);
        assert_eq!(<[BTreeSet<usize>; 2]>::complete().count_all_vertices(), 2);
        assert_eq!(<[BTreeSet<usize>; 3]>::complete().count_all_vertices(), 3);
        assert_eq!(<[BTreeSet<usize>; 4]>::complete().count_all_vertices(), 4);
        assert_eq!(<[BTreeSet<usize>; 5]>::complete().count_all_vertices(), 5);
    }

    #[test]
    fn count_all_vertices_arr_hash_set() {
        assert_eq!(<[HashSet<usize>; 0]>::complete().count_all_vertices(), 0);
        assert_eq!(<[HashSet<usize>; 1]>::complete().count_all_vertices(), 1);
        assert_eq!(<[HashSet<usize>; 2]>::complete().count_all_vertices(), 2);
        assert_eq!(<[HashSet<usize>; 3]>::complete().count_all_vertices(), 3);
        assert_eq!(<[HashSet<usize>; 4]>::complete().count_all_vertices(), 4);
        assert_eq!(<[HashSet<usize>; 5]>::complete().count_all_vertices(), 5);
    }

    #[test]
    fn indegree_arr_btree_set() {
        let graph = <[BTreeSet<usize>; 1]>::complete();

        assert_eq!(graph.indegree(0), 0);

        let graph = <[BTreeSet<usize>; 2]>::complete();

        assert_eq!(graph.indegree(0), 1);
        assert_eq!(graph.indegree(1), 1);

        let graph = <[BTreeSet<usize>; 3]>::complete();

        assert_eq!(graph.indegree(0), 2);
        assert_eq!(graph.indegree(1), 2);
        assert_eq!(graph.indegree(2), 2);
    }

    #[test]
    fn indegree_arr_hash_set() {
        let graph = <[HashSet<usize>; 1]>::complete();

        assert_eq!(graph.indegree(0), 0);

        let graph = <[HashSet<usize>; 2]>::complete();

        assert_eq!(graph.indegree(0), 1);
        assert_eq!(graph.indegree(1), 1);

        let graph = <[HashSet<usize>; 3]>::complete();

        assert_eq!(graph.indegree(0), 2);
        assert_eq!(graph.indegree(1), 2);
        assert_eq!(graph.indegree(2), 2);
    }

    #[test]
    fn is_simple_arr_btree_set() {
        assert!(<[BTreeSet<usize>; 0]>::complete().is_simple());
        assert!(<[BTreeSet<usize>; 1]>::complete().is_simple());
        assert!(<[BTreeSet<usize>; 2]>::complete().is_simple());
        assert!(<[BTreeSet<usize>; 3]>::complete().is_simple());
        assert!(<[BTreeSet<usize>; 4]>::complete().is_simple());
        assert!(<[BTreeSet<usize>; 5]>::complete().is_simple());
        assert!(<[BTreeSet<usize>; 6]>::complete().is_simple());
    }

    #[test]
    fn is_simple_arr_hash_set() {
        assert!(<[HashSet<usize>; 0]>::complete().is_simple());
        assert!(<[HashSet<usize>; 1]>::complete().is_simple());
        assert!(<[HashSet<usize>; 2]>::complete().is_simple());
        assert!(<[HashSet<usize>; 3]>::complete().is_simple());
        assert!(<[HashSet<usize>; 4]>::complete().is_simple());
        assert!(<[HashSet<usize>; 5]>::complete().is_simple());
        assert!(<[HashSet<usize>; 6]>::complete().is_simple());
    }

    #[test]
    fn outdegree_arr_vec() {
        let graph = <[Vec<usize>; 1]>::complete();

        assert_eq!(graph[0].len(), 0);

        let graph = <[Vec<usize>; 2]>::complete();

        assert_eq!(graph[0].len(), 1);
        assert_eq!(graph[1].len(), 1);

        let graph = <[Vec<usize>; 3]>::complete();

        assert_eq!(graph[0].len(), 2);
        assert_eq!(graph[1].len(), 2);
        assert_eq!(graph[2].len(), 2);
    }

    #[test]
    fn outdegree_arr_btree_set() {
        let graph = <[BTreeSet<usize>; 1]>::complete();

        assert_eq!(graph[0].len(), 0);

        let graph = <[BTreeSet<usize>; 2]>::complete();

        assert_eq!(graph[0].len(), 1);
        assert_eq!(graph[1].len(), 1);

        let graph = <[BTreeSet<usize>; 3]>::complete();

        assert_eq!(graph[0].len(), 2);
        assert_eq!(graph[1].len(), 2);
        assert_eq!(graph[2].len(), 2);
    }

    #[test]
    fn outdegree_arr_hash_set() {
        let graph = <[HashSet<usize>; 1]>::complete();

        assert_eq!(graph[0].len(), 0);

        let graph = <[HashSet<usize>; 2]>::complete();

        assert_eq!(graph[0].len(), 1);
        assert_eq!(graph[1].len(), 1);

        let graph = <[HashSet<usize>; 3]>::complete();

        assert_eq!(graph[0].len(), 2);
        assert_eq!(graph[1].len(), 2);
        assert_eq!(graph[2].len(), 2);
    }
}

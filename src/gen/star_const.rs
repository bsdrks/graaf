//! A trait to generate undirected const-sized star graphs
//!
//! To generate undirected variable-sized star graphs, see [`Star`].
//!
//! # Examples
//!
//! ```
//! use graaf::gen::StarConst;
//!
//! assert!(<[Vec::<usize>; 0]>::star().is_empty());
//! assert_eq!(<[Vec::<usize>; 1]>::star(), [Vec::new()]);
//! assert_eq!(<[Vec::<usize>; 2]>::star(), [vec![1], vec![0]]);
//! assert_eq!(<[Vec::<usize>; 3]>::star(), [vec![1, 2], vec![0], vec![0]]);
//! ```
//!
//! [`Star`]: crate::gen::Star

extern crate alloc;

use {
    alloc::collections::BTreeSet,
    core::{
        array::from_fn,
        hash::BuildHasher,
    },
    std::collections::HashSet,
};

/// A trait to generate undirected const-sized star graphs
///
/// # How can I implement `StarConst`?
///
/// Provide an implementation of `star` that generates an undirected star graph.
///
/// ```
/// use {
///     core::{
///         array::from_fn,
///         hash::BuildHasher,
///     },
///     graaf::gen::StarConst,
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
/// impl<const V: usize, H> StarConst for Graph<V, H>
/// where
///     H: BuildHasher + Default,
/// {
///     fn star() -> Self {
///         let mut edges = from_fn(|_| HashSet::with_hasher(H::default()));
///
///         for s in 1..V {
///             edges[0].insert(s);
///             edges[s].insert(0);
///         }
///
///         Graph { edges }
///     }
/// }
///
/// let graph = Graph::<3, RandomState>::star();
///
/// assert_eq!(
///     graph.edges,
///     [
///         HashSet::from([1, 2]),
///         HashSet::from([0]),
///         HashSet::from([0])
///     ]
/// );
/// ```
pub trait StarConst {
    /// Generates a star graph.
    fn star() -> Self;
}

impl<const V: usize> StarConst for [Vec<usize>; V] {
    fn star() -> Self {
        let mut graph: [Vec<usize>; V] = from_fn(|_| Vec::new());

        if V == 0 {
            return graph;
        }

        for s in 1..V {
            graph[0].push(s);
            graph[s].push(0);
        }

        graph
    }
}

impl<const V: usize> StarConst for [BTreeSet<usize>; V] {
    fn star() -> Self {
        let mut graph: [BTreeSet<usize>; V] = from_fn(|_| BTreeSet::new());

        if V == 0 {
            return graph;
        }

        for s in 1..V {
            let _ = graph[0].insert(s);
            let _ = graph[s].insert(0);
        }

        graph
    }
}

impl<const V: usize, H> StarConst for [HashSet<usize, H>; V]
where
    H: BuildHasher + Default,
{
    fn star() -> Self {
        let mut graph: [HashSet<usize, H>; V] = from_fn(|_| HashSet::with_hasher(H::default()));

        if V == 0 {
            return graph;
        }

        for s in 1..V {
            let _ = graph[0].insert(s);
            let _ = graph[s].insert(0);
        }

        graph
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::op::IsSimple,
    };

    #[test]
    fn arr_vec() {
        assert!(<[Vec::<usize>; 0]>::star().is_empty());
        assert_eq!(<[Vec::<usize>; 1]>::star(), [Vec::new()]);
        assert_eq!(<[Vec::<usize>; 2]>::star(), [vec![1], vec![0]]);
        assert_eq!(<[Vec::<usize>; 3]>::star(), [vec![1, 2], vec![0], vec![0]]);
    }

    #[test]
    fn arr_btree_set() {
        assert!(<[BTreeSet::<usize>; 0]>::star().is_empty());
        assert_eq!(<[BTreeSet::<usize>; 1]>::star(), [BTreeSet::new()]);

        assert_eq!(
            <[BTreeSet::<usize>; 2]>::star(),
            [BTreeSet::from([1]), BTreeSet::from([0])]
        );

        assert_eq!(
            <[BTreeSet::<usize>; 3]>::star(),
            [
                BTreeSet::from([1, 2]),
                BTreeSet::from([0]),
                BTreeSet::from([0])
            ]
        );
    }

    #[test]
    fn arr_hash_set() {
        assert!(<[HashSet::<usize>; 0]>::star().is_empty());
        assert_eq!(<[HashSet::<usize>; 1]>::star(), [HashSet::new()]);

        assert_eq!(
            <[HashSet::<usize>; 2]>::star(),
            [HashSet::from([1]), HashSet::from([0])]
        );

        assert_eq!(
            <[HashSet::<usize>; 3]>::star(),
            [
                HashSet::from([1, 2]),
                HashSet::from([0]),
                HashSet::from([0])
            ]
        );
    }

    #[test]
    fn is_simple_arr_btree_set() {
        assert!(<[BTreeSet::<usize>; 0]>::star().is_simple());
        assert!(<[BTreeSet::<usize>; 1]>::star().is_simple());
        assert!(<[BTreeSet::<usize>; 2]>::star().is_simple());
        assert!(<[BTreeSet::<usize>; 3]>::star().is_simple());
        assert!(<[BTreeSet::<usize>; 4]>::star().is_simple());
        assert!(<[BTreeSet::<usize>; 5]>::star().is_simple());
    }

    #[test]
    fn is_simple_arr_hash_set() {
        assert!(<[HashSet::<usize>; 0]>::star().is_simple());
        assert!(<[HashSet::<usize>; 1]>::star().is_simple());
        assert!(<[HashSet::<usize>; 2]>::star().is_simple());
        assert!(<[HashSet::<usize>; 3]>::star().is_simple());
        assert!(<[HashSet::<usize>; 4]>::star().is_simple());
        assert!(<[HashSet::<usize>; 5]>::star().is_simple());
    }
}

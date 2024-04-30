//! A trait to generate linear graphs for const-sized graphs
//!
//! Linear graphs are also known as path graphs. To generate variable-sized
//! linear graphs, see the [`Linear`](crate::gen::Linear) trait.
//!
//! # Examples
//!
//! ```
//! use graaf::gen::LinearConst;
//!
//! //
//! assert!(<[Vec::<usize>; 0]>::linear().is_empty());
//!
//! // 0
//! assert_eq!(<[Vec::<usize>; 1]>::linear(), [Vec::new()]);
//!
//! // 0 → 1
//! assert_eq!(<[Vec::<usize>; 2]>::linear(), [vec![1], Vec::new()]);
//!
//! // 0 → 1 → 2
//! assert_eq!(
//!     <[Vec::<usize>; 3]>::linear(),
//!     [vec![1], vec![2], Vec::new()]
//! );
//! ```

extern crate alloc;

use {
    alloc::collections::BTreeSet,
    core::{
        array::from_fn,
        hash::BuildHasher,
    },
    std::collections::HashSet,
};

/// A trait to generate linear graphs for const-sized graphs
///
/// # How can I implement `LinearConst`?
///
/// Provide an implementation of `linear` that generates a linear graph.
///
/// ```
/// use {
///     core::{
///         array::from_fn,
///         hash::BuildHasher,
///     },
///     graaf::gen::LinearConst,
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
/// impl<const V: usize, H> LinearConst for Graph<V, H>
/// where
///     H: BuildHasher + Default,
/// {
///     fn linear() -> Self {
///         let mut edges = from_fn(|_| HashSet::with_hasher(H::default()));
///
///         for (s, set) in edges.iter_mut().enumerate().take(V - 1) {
///             let _ = set.insert(s + 1);
///         }
///
///         Graph { edges }
///     }
/// }
///
/// let graph = Graph::<3, RandomState>::linear();
///
/// assert_eq!(
///     graph.edges,
///     [HashSet::from([1]), HashSet::from([2]), HashSet::new()]
/// );
/// ```
pub trait LinearConst {
    /// Generate a linear graph.
    fn linear() -> Self;
}

impl<const V: usize> LinearConst for [Vec<usize>; V] {
    fn linear() -> Self {
        let mut graph: [Vec<usize>; V] = from_fn(|_| Vec::new());

        if V == 0 {
            return graph;
        }

        for (s, vec) in graph.iter_mut().enumerate().take(V - 1) {
            vec.push(s + 1);
        }

        graph
    }
}

impl<const V: usize> LinearConst for [BTreeSet<usize>; V] {
    fn linear() -> Self {
        let mut graph: [BTreeSet<usize>; V] = from_fn(|_| BTreeSet::new());

        if V == 0 {
            return graph;
        }

        for (s, set) in graph.iter_mut().enumerate().take(V - 1) {
            let _ = set.insert(s + 1);
        }

        graph
    }
}

impl<const V: usize, H> LinearConst for [HashSet<usize, H>; V]
where
    H: BuildHasher + Default,
{
    fn linear() -> Self {
        let mut graph: [HashSet<usize, H>; V] = from_fn(|_| HashSet::with_hasher(H::default()));

        if V == 0 {
            return graph;
        }

        for (s, set) in graph.iter_mut().enumerate().take(V - 1) {
            let _ = set.insert(s + 1);
        }

        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arr_vec() {
        //
        assert!(<[Vec::<usize>; 0]>::linear().is_empty());

        // 0
        assert_eq!(<[Vec::<usize>; 1]>::linear(), [Vec::new()]);

        // 0 → 1
        assert_eq!(<[Vec::<usize>; 2]>::linear(), [vec![1], Vec::new()]);

        // 0 → 1 → 2
        assert_eq!(
            <[Vec::<usize>; 3]>::linear(),
            [vec![1], vec![2], Vec::new()]
        );
    }

    #[test]
    fn arr_btree_set() {
        //
        assert!(<[BTreeSet::<usize>; 0]>::linear().is_empty());

        // 0
        assert_eq!(<[BTreeSet::<usize>; 1]>::linear(), [BTreeSet::new()]);

        // 0 → 1
        assert_eq!(
            <[BTreeSet::<usize>; 2]>::linear(),
            [BTreeSet::from([1]), BTreeSet::new()]
        );

        // 0 → 1 → 2
        assert_eq!(
            <[BTreeSet::<usize>; 3]>::linear(),
            [BTreeSet::from([1]), BTreeSet::from([2]), BTreeSet::new()]
        );
    }

    #[test]
    fn arr_hash_set() {
        //
        assert!(<[HashSet::<usize>; 0]>::linear().is_empty());

        // 0
        assert_eq!(<[HashSet::<usize>; 1]>::linear(), [HashSet::new()]);

        // 0 → 1
        assert_eq!(
            <[HashSet::<usize>; 2]>::linear(),
            [HashSet::from([1]), HashSet::new()]
        );

        // 0 → 1 → 2
        assert_eq!(
            <[HashSet::<usize>; 3]>::linear(),
            [HashSet::from([1]), HashSet::from([2]), HashSet::new()]
        );
    }
}

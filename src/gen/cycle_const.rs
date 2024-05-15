#![doc(alias = "circular")]
//! A trait to generate const-sized directed cycle graphs
//!
//! Cycle graphs are also known as circular graphs. To generate variable-sized
//! cycle graphs, see [`Cycle`].
//!
//! # Examples
//!
//! ```
//! use graaf::gen::CycleConst;
//!
//! assert_eq!(<[Vec::<usize>; 1]>::cycle(), [vec![0]]);
//! assert_eq!(<[Vec::<usize>; 2]>::cycle(), [vec![1], vec![0]]);
//! assert_eq!(<[Vec::<usize>; 3]>::cycle(), [vec![1], vec![2], vec![0]]);
//! ```
//!
//! [`Cycle`]: crate::gen::Cycle

extern crate alloc;

use {
    crate::gen::EmptyConst,
    alloc::collections::BTreeSet,
    core::{
        array::from_fn,
        hash::BuildHasher,
    },
    std::collections::HashSet,
};

/// A trait to generate const-sized directed cycle graphs
///
/// # How can I implement `CycleConst`?
///
/// Provide an implementation of `cycle` that generates a cycle graph with `V`
/// vertices.
///
/// ```
/// use {
///     core::{
///         array::from_fn,
///         hash::BuildHasher,
///     },
///     graaf::gen::CycleConst,
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
///     arcs: [HashSet<usize, H>; V],
/// }
///
/// impl<const V: usize, H> CycleConst for Graph<V, H>
/// where
///     H: BuildHasher + Default,
/// {
///     /// # Panics
///     ///
///     /// Panics if `V` is zero.
///     fn cycle() -> Self {
///         assert!(V > 0, "a graph must have at least one vertex");
///
///         let mut arcs = from_fn(|_| HashSet::with_hasher(H::default()));
///
///         for (s, set) in arcs.iter_mut().enumerate().take(V - 1) {
///             let _ = set.insert(s + 1);
///         }
///
///         let _ = arcs[V - 1].insert(0);
///
///         Graph { arcs }
///     }
/// }
///
/// let graph = Graph::<3, RandomState>::cycle();
///
/// assert_eq!(
///     graph.arcs,
///     [HashSet::from([1]), HashSet::from([2]), HashSet::from([0])]
/// );
/// ```
pub trait CycleConst {
    /// Generates a cycle graph.
    fn cycle() -> Self;
}

impl<const V: usize> CycleConst for [Vec<usize>; V] {
    /// # Panics
    ///
    /// Panics if `V` is zero.
    fn cycle() -> Self {
        assert!(V > 0, "a graph must have at least one vertex");

        let mut graph = Self::empty();

        for (s, vec) in graph.iter_mut().enumerate().take(V - 1) {
            vec.push(s + 1);
        }

        graph[V - 1].push(0);

        graph
    }
}

impl<const V: usize> CycleConst for [BTreeSet<usize>; V] {
    /// # Panics
    ///
    /// Panics if `V` is zero.
    fn cycle() -> Self {
        assert!(V > 0, "a graph must have at least one vertex");

        let mut graph = Self::empty();

        for (s, set) in graph.iter_mut().enumerate().take(V - 1) {
            let _ = set.insert(s + 1);
        }

        let _ = graph[V - 1].insert(0);

        graph
    }
}

impl<const V: usize, H> CycleConst for [HashSet<usize, H>; V]
where
    H: BuildHasher + Default,
{
    /// # Panics
    ///
    /// Panics if `V` is zero.
    fn cycle() -> Self {
        assert!(V > 0, "a graph must have at least one vertex");

        let mut graph = Self::empty();

        for (s, set) in graph.iter_mut().enumerate().take(V - 1) {
            let _ = set.insert(s + 1);
        }

        let _ = graph[V - 1].insert(0);

        graph
    }
}

impl<const V: usize> CycleConst for [(usize, usize); V] {
    /// # Panics
    ///
    /// Panics if `V` is zero.
    fn cycle() -> Self {
        assert!(V > 0, "a graph must have at least one vertex");

        from_fn(|i| (i, (i + 1) % V))
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
        assert_eq!(<[Vec::<usize>; 1]>::cycle(), [vec![0]]);
        assert_eq!(<[Vec::<usize>; 2]>::cycle(), [vec![1], vec![0]]);
        assert_eq!(<[Vec::<usize>; 3]>::cycle(), [vec![1], vec![2], vec![0]]);
    }

    #[test]
    fn arr_btree_set() {
        assert_eq!(<[BTreeSet::<usize>; 1]>::cycle(), [BTreeSet::from([0])]);

        assert_eq!(
            <[BTreeSet::<usize>; 2]>::cycle(),
            [BTreeSet::from([1]), BTreeSet::from([0])]
        );

        assert_eq!(
            <[BTreeSet::<usize>; 3]>::cycle(),
            [
                BTreeSet::from([1]),
                BTreeSet::from([2]),
                BTreeSet::from([0])
            ]
        );
    }

    #[test]
    fn arr_hash_set() {
        assert_eq!(<[HashSet::<usize>; 1]>::cycle(), [HashSet::from([0])]);

        assert_eq!(
            <[HashSet::<usize>; 2]>::cycle(),
            [HashSet::from([1]), HashSet::from([0])]
        );

        assert_eq!(
            <[HashSet::<usize>; 3]>::cycle(),
            [HashSet::from([1]), HashSet::from([2]), HashSet::from([0])]
        );
    }

    #[test]
    fn arr_tuple() {
        assert_eq!(<[(_, _); 1]>::cycle(), [(0, 0)]);
        assert_eq!(<[(_, _); 2]>::cycle(), [(0, 1), (1, 0)]);
        assert_eq!(<[(_, _); 3]>::cycle(), [(0, 1), (1, 2), (2, 0)]);
    }

    #[test]
    fn is_simple_arr_btree_set() {
        assert!(!<[BTreeSet<usize>; 1]>::cycle().is_simple());
        assert!(<[BTreeSet<usize>; 2]>::cycle().is_simple());
        assert!(<[BTreeSet<usize>; 3]>::cycle().is_simple());
        assert!(<[BTreeSet<usize>; 4]>::cycle().is_simple());
        assert!(<[BTreeSet<usize>; 5]>::cycle().is_simple());
        assert!(<[BTreeSet<usize>; 6]>::cycle().is_simple());
    }

    #[test]
    fn is_simple_arr_hash_set() {
        assert!(!<[HashSet<usize>; 1]>::cycle().is_simple());
        assert!(<[HashSet<usize>; 2]>::cycle().is_simple());
        assert!(<[HashSet<usize>; 3]>::cycle().is_simple());
        assert!(<[HashSet<usize>; 4]>::cycle().is_simple());
        assert!(<[HashSet<usize>; 5]>::cycle().is_simple());
        assert!(<[HashSet<usize>; 6]>::cycle().is_simple());
    }

    #[test]
    fn is_simple_arr_tuple() {
        assert!(!<[(_, _); 1]>::cycle().is_simple());
        assert!(<[(_, _); 2]>::cycle().is_simple());
        assert!(<[(_, _); 3]>::cycle().is_simple());
        assert!(<[(_, _); 4]>::cycle().is_simple());
        assert!(<[(_, _); 5]>::cycle().is_simple());
        assert!(<[(_, _); 6]>::cycle().is_simple());
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn arr_vec_panic() {
        let _ = <[Vec<usize>; 0]>::cycle();
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn arr_btree_set_panic() {
        let _ = <[BTreeSet<usize>; 0]>::cycle();
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn arr_hash_set_panic() {
        let _ = <[HashSet<usize>; 0]>::cycle();
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn arr_tuple_panic() {
        let _ = <[(usize, usize); 0]>::cycle();
    }
}

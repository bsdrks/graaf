//! A trait to generate random variable-sized tournaments
//!
//! A tournament is a directed graph in which for every pair of distinct
//! vertices `s` and `t`, exactly one of the arcs `(s, t)` and `(t, s)` is
//! present.
//!
//! # Examples
//!
//! ```
//! extern crate alloc;
//!
//! use {
//!     alloc::collections::BTreeSet,
//!     graaf::{
//!         gen::RandomTournament,
//!         op::{
//!             Degree,
//!             Indegree,
//!             IterVertices,
//!             Order,
//!             Outdegree,
//!             Size,
//!         },
//!     },
//! };
//!
//! let tournament = Vec::<BTreeSet<usize>>::random_tournament(4);
//!
//! assert_eq!(tournament.size(), 6);
//! assert_eq!(tournament.order(), 4);
//!
//! for s in tournament.iter_vertices() {
//!     assert_eq!(tournament.degree(s), 3);
//!     assert!((0..3).contains(&tournament.outdegree(s)));
//!     assert!((0..3).contains(&tournament.indegree(s)));
//! }
//! ```

extern crate alloc;

use {
    super::prng::Xoshiro256StarStar,
    crate::{
        gen::Empty,
        op::AddArc,
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

/// A trait to generate random variable-sized tournaments
///
/// # How can I implement `RandomTournament`?
///
/// Provide an implementation of `random_tournament` that returns a random
/// tournament.
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::{
///         gen::{
///             prng::Xoshiro256StarStar,
///             Empty,
///             RandomTournament,
///         },
///         op::{
///             AddArc,
///             Degree,
///             Indegree,
///             IterVertices,
///             Order,
///             Outdegree,
///             Size,
///         },
///     },
/// };
///
/// pub struct Tournament {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Empty for Tournament {
///     fn empty(v: usize) -> Self {
///         Tournament {
///             arcs: vec![BTreeSet::new(); v],
///         }
///     }
/// }
///
/// impl RandomTournament for Tournament {
///     fn random_tournament(v: usize) -> Self {
///         let mut rng = Xoshiro256StarStar::new(v as u64);
///         let mut tournament = Tournament::empty(v);
///
///         for s in 0..v {
///             for t in (s + 1)..v {
///                 if rng.next_bool() {
///                     tournament.arcs.add_arc(s, t);
///                 } else {
///                     tournament.arcs.add_arc(t, s);
///                 }
///             }
///         }
///
///         tournament
///     }
/// }
///
/// let tournament = Tournament::random_tournament(4);
///
/// assert_eq!(tournament.arcs.size(), 6);
/// assert_eq!(tournament.arcs.order(), 4);
///
/// for s in tournament.arcs.iter_vertices() {
///     assert_eq!(tournament.arcs.degree(s), 3);
///     assert!((0..3).contains(&tournament.arcs.outdegree(s)));
///     assert!((0..3).contains(&tournament.arcs.indegree(s)));
/// }
/// ```
///
/// # Examples
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::{
///         gen::RandomTournament,
///         op::{
///             Degree,
///             Indegree,
///             IterVertices,
///             Order,
///             Outdegree,
///             Size,
///         },
///     },
/// };
///
/// let tournament = Vec::<BTreeSet<usize>>::random_tournament(4);
///
/// assert_eq!(tournament.size(), 6);
/// assert_eq!(tournament.order(), 4);
///
/// for s in tournament.iter_vertices() {
///     assert_eq!(tournament.degree(s), 3);
///     assert!((0..3).contains(&tournament.outdegree(s)));
///     assert!((0..3).contains(&tournament.indegree(s)));
/// }
/// ```
pub trait RandomTournament {
    /// Returns a random tournament.
    #[must_use]
    fn random_tournament(v: usize) -> Self;
}

macro_rules! impl_random_tournament {
    () => {
        fn random_tournament(v: usize) -> Self {
            let mut rng = Xoshiro256StarStar::new(v as u64);
            let mut arcs = Self::empty(v);

            for s in 0..v {
                for t in (s + 1)..v {
                    if rng.next_bool() {
                        arcs.add_arc(s, t);
                    } else {
                        arcs.add_arc(t, s);
                    }
                }
            }

            arcs
        }
    };
}

impl RandomTournament for Vec<Vec<usize>> {
    impl_random_tournament!();
}

impl RandomTournament for Vec<BTreeSet<usize>> {
    impl_random_tournament!();
}

impl<H> RandomTournament for Vec<HashSet<usize, H>>
where
    H: BuildHasher + Default,
    HashSet<usize, H>: Clone,
{
    impl_random_tournament!();
}

impl RandomTournament for BTreeMap<usize, Vec<usize>> {
    impl_random_tournament!();
}

impl RandomTournament for BTreeMap<usize, BTreeSet<usize>> {
    impl_random_tournament!();
}

impl<H> RandomTournament for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher + Default,
{
    impl_random_tournament!();
}

impl<H> RandomTournament for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher + Default,
{
    impl_random_tournament!();
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::op::{
            Degree,
            Indegree,
            IterVertices,
            Order,
            Outdegree,
            Size,
        },
    };

    #[test]
    fn vec_vec() {
        let graph = <Vec<Vec<usize>>>::random_tournament(4);

        assert_eq!(graph.size(), 6);
        assert_eq!(graph.order(), 4);

        for s in graph.iter_vertices() {
            assert!((0..3).contains(&graph.outdegree(s)));
        }
    }

    #[test]
    fn vec_btree_set() {
        let graph = <Vec<BTreeSet<usize>>>::random_tournament(4);

        assert_eq!(graph.size(), 6);
        assert_eq!(graph.order(), 4);

        for s in graph.iter_vertices() {
            assert_eq!(graph.degree(s), 3);
            assert!((0..3).contains(&graph.outdegree(s)));
            assert!((0..3).contains(&graph.indegree(s)));
        }
    }

    #[test]
    fn vec_hash_set() {
        let graph = <Vec<HashSet<usize>>>::random_tournament(4);

        assert_eq!(graph.size(), 6);
        assert_eq!(graph.order(), 4);

        for s in graph.iter_vertices() {
            assert_eq!(graph.degree(s), 3);
            assert!((0..3).contains(&graph.outdegree(s)));
            assert!((0..3).contains(&graph.indegree(s)));
        }
    }

    #[test]
    fn btree_map_vec() {
        let graph = <BTreeMap<usize, Vec<usize>>>::random_tournament(4);

        assert_eq!(graph.size(), 6);

        for s in graph.iter_vertices() {
            assert!((0..3).contains(&graph.outdegree(s)));
        }
    }

    #[test]
    fn btree_map_btree_set() {
        let graph = <BTreeMap<usize, BTreeSet<usize>>>::random_tournament(4);

        assert_eq!(graph.size(), 6);

        for s in graph.iter_vertices() {
            assert_eq!(graph.degree(s), 3);
            assert!((0..3).contains(&graph.outdegree(s)));
            assert!((0..3).contains(&graph.indegree(s)));
        }
    }

    #[test]
    fn hash_map_vec() {
        let graph = <HashMap<usize, Vec<usize>>>::random_tournament(4);

        assert_eq!(graph.size(), 6);

        for s in graph.iter_vertices() {
            assert!((0..3).contains(&graph.outdegree(s)));
        }
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = <HashMap<usize, HashSet<usize>>>::random_tournament(4);

        assert_eq!(graph.size(), 6);

        for s in graph.iter_vertices() {
            assert_eq!(graph.degree(s), 3);
            assert!((0..3).contains(&graph.outdegree(s)));
            assert!((0..3).contains(&graph.indegree(s)));
        }
    }
}

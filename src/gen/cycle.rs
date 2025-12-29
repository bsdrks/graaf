//! Generate cycle digraphs.
//!
//! A cycle is a digraph with a single bidirectional cycle.
//!
//! # Examples
//!
//! ## Order 2
//!
//! Generate a cycle digraph of order `2`.
//!
//! ![A cycle digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/cycle_2.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Cycle,
//! };
//!
//! assert!(AdjacencyList::cycle(2).arcs().eq([(0, 1), (1, 0)]));
//! ```
//!
//! ## Order 3
//!
//! Generate a cycle digraph of order `3`.
//!
//! ![A cycle digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/cycle_3.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Cycle,
//! };
//!
//! assert!(AdjacencyList::cycle(3).arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (1, 0),
//!     (1, 2),
//!     (2, 0),
//!     (2, 1)
//! ]));
//! ```
//!
//! ## Order 4
//!
//! Generate a cycle digraph of order `4`.
//!
//! ![A cycle digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/cycle_4.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Cycle,
//! };
//!
//! assert!(AdjacencyList::cycle(4).arcs().eq([
//!     (0, 1),
//!     (0, 3),
//!     (1, 0),
//!     (1, 2),
//!     (2, 1),
//!     (2, 3),
//!     (3, 0),
//!     (3, 2)
//! ]));
//! ```

/// Cycle digraphs
pub trait Cycle {
    /// Generate a cycle digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ## Order 2
    ///
    /// Generate a cycle digraph of order `2`.
    ///
    /// ![A cycle digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/cycle_2.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Cycle,
    /// };
    ///
    /// assert!(AdjacencyList::cycle(2).arcs().eq([(0, 1), (1, 0)]));
    /// ```
    ///
    /// ## Order 3
    ///
    /// Generate a cycle digraph of order `3`.
    ///
    /// ![A cycle digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/cycle_3.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Cycle,
    /// };
    ///
    /// assert!(AdjacencyList::cycle(3).arcs().eq([
    ///     (0, 1),
    ///     (0, 2),
    ///     (1, 0),
    ///     (1, 2),
    ///     (2, 0),
    ///     (2, 1)
    /// ]));
    /// ```
    ///
    /// ## Order 4
    ///
    /// Generate a cycle digraph of order `4`.
    ///
    /// ![A cycle digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/cycle_4.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Cycle,
    /// };
    ///
    /// assert!(AdjacencyList::cycle(4).arcs().eq([
    ///     (0, 1),
    ///     (0, 3),
    ///     (1, 0),
    ///     (1, 2),
    ///     (2, 1),
    ///     (2, 3),
    ///     (3, 0),
    ///     (3, 2)
    /// ]));
    /// ```
    #[must_use]
    fn cycle(order: usize) -> Self;
}

/// `Cycle` tests
#[macro_export]
macro_rules! test_cycle {
    ($type:ty) => {
        #[test]
        #[should_panic(expected = "a digraph has at least one vertex")]
        fn cycle_0() {
            drop(<$type>::cycle(0));
        }

        #[test]
        fn cycle_1() {
            let digraph = <$type>::cycle(1);

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn cycle_1_complement() {
            let digraph = <$type>::cycle(1).complement();

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn cycle_2() {
            let digraph = <$type>::cycle(2);

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([(0, 1), (1, 0)]));
        }

        #[test]
        fn cycle_2_complement() {
            let digraph = <$type>::cycle(2).complement();

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn cycle_3() {
            let digraph = <$type>::cycle(3);

            assert_eq!(digraph.order(), 3);

            assert!(digraph.arcs().eq([
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 2),
                (2, 0),
                (2, 1)
            ]));
        }

        #[test]
        fn cycle_3_complement() {
            let digraph = <$type>::cycle(3).complement();

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([]));
        }
    };
}

/// `Cycle` proptests
#[macro_export]
macro_rules! proptest_cycle {
    ($type:ty) => {
        use {
            proptest::proptest,
            $crate::{
                Degree,
                IsBalanced,
                IsIsolated,
                IsOriented,
                IsPendant,
                IsSpanningSubdigraph,
                IsSubdigraph,
                IsSuperdigraph,
                IsSymmetric,
                OutdegreeSequence,
                Sinks,
                Sources,
            },
        };

        proptest! {
            #[test]
            fn cycle_complement_size(order in 1..5_usize) {
                assert_eq!(
                    <$type>::cycle(order).complement().size(),
                    order * order.saturating_sub(3)
                );
            }

            #[test]
            fn cycle_degree(order in 1..5_usize) {
                let digraph = <$type>::cycle(order);

                assert!(digraph.vertices().all(|u| {
                    digraph.degree(u)
                        == match order {
                            1 => 0,
                            2 => 2,
                            _ => 4,
                        }
                }));
            }

            #[test]
            fn cycle_degree_sequence(order in 1..5_usize) {
                assert!(<$type>::cycle(order)
                    .degree_sequence()
                    .all(|d| match order {
                        1 => d == 0,
                        2 => d == 2,
                        _ => d == 4,
                    }));
            }

            #[test]
            fn cycle_degree_sum_equals_2size(order in 1..5_usize) {
                let digraph = <$type>::cycle(order);

                assert_eq!(
                    digraph
                        .vertices()
                        .fold(0, |acc, u| acc + digraph.degree(u)),
                    2 * digraph.size()
                );
            }

            #[test]
            fn cycle_even_number_odd_degrees(order in 1..5_usize) {
                let digraph = <$type>::cycle(order);

                assert_eq!(
                    digraph
                        .vertices()
                        .filter(|&u| digraph.degree(u) % 2 == 1)
                        .count()
                        % 2,
                    0
                );
            }

            #[test]
            fn cycle_indegree(order in 1..5_usize) {
                let digraph = <$type>::cycle(order);

                assert!(digraph.vertices().all(|u| {
                    digraph.indegree(u)
                        == match order {
                            1 => 0,
                            2 => 1,
                            _ => 2,
                        }
                }));
            }

            #[test]
            fn cycle_indegree_sequence(order in 1..5_usize) {
                assert!(<$type>::cycle(order).indegree_sequence().all(|d| d
                    == match order {
                        1 => 0,
                        2 => 1,
                        _ => 2,
                    }));
            }

            #[test]
            fn cycle_is_balanced(order in 1..5_usize) {
                assert!(<$type>::cycle(order).is_balanced());
            }

            #[test]
            fn cycle_is_complete(order in 1..5_usize) {
                assert!((order < 4) == <$type>::cycle(order).is_complete());
            }

            #[test]
            fn cycle_is_isolated(order in 1..5_usize) {
                let digraph = <$type>::cycle(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_isolated(u)));
            }

            #[test]
            fn cycle_is_oriented(order in 1..5_usize) {
                assert!((order == 1) == <$type>::cycle(order).is_oriented());
            }

            #[test]
            fn cycle_is_pendant(order in 1..5_usize) {
                let digraph = <$type>::cycle(order);

                assert!(digraph.vertices().all(|u| !digraph.is_pendant(u)));
            }

            #[test]
            fn cycle_is_regular(order in 1..5_usize) {
                assert!(<$type>::cycle(order).is_regular());
            }

            #[test]
            fn cycle_is_semicomplete(order in 1..5_usize) {
                assert!((order < 4) == <$type>::cycle(order).is_semicomplete());
            }

            #[test]
            fn cycle_is_simple(order in 1..5_usize) {
                assert!(<$type>::cycle(order).is_simple());
            }

            #[test]
            fn cycle_is_sink(order in 1..5_usize) {
                let digraph = <$type>::cycle(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_sink(u)));
            }

            #[test]
            fn cycle_is_source(order in 1..5_usize) {
                let digraph = <$type>::cycle(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_source(u)));
            }

            #[test]
            fn cycle_is_spanning_subdigraph(order in 1..5_usize) {
                let digraph = <$type>::cycle(order);

                assert!(digraph.is_spanning_subdigraph(&digraph));
            }

            #[test]
            fn cycle_is_subdigraph(order in 1..5_usize) {
                let digraph = <$type>::cycle(order);

                assert!(digraph.is_subdigraph(&digraph));
            }

            #[test]
            fn cycle_is_superdigraph(order in 1..5_usize) {
                let digraph = <$type>::cycle(order);

                assert!(digraph.is_superdigraph(&digraph));
            }

            #[test]
            fn cycle_is_symmetric(order in 1..5_usize) {
                assert!(<$type>::cycle(order).is_symmetric());
            }

            #[test]
            fn cycle_is_tournament(order in 1..5_usize) {
                assert!((order == 1) == <$type>::cycle(order).is_tournament());
            }

            #[test]
            fn cycle_max_degree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::cycle(order).max_degree(),
                    match order {
                        1 => 0,
                        2 => 2,
                        _ => 4,
                    }
                );
            }

            #[test]
            fn cycle_max_indegree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::cycle(order).max_indegree(),
                    match order {
                        1 => 0,
                        2 => 1,
                        _ => 2,
                    }
                );
            }

            #[test]
            fn cycle_max_outdegree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::cycle(order).max_outdegree(),
                    match order {
                        1 => 0,
                        2 => 1,
                        _ => 2,
                    }
                );
            }

            #[test]
            fn cycle_min_degree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::cycle(order).min_degree(),
                    match order {
                        1 => 0,
                        2 => 2,
                        _ => 4,
                    }
                );
            }

            #[test]
            fn cycle_min_indegree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::cycle(order).min_indegree(),
                    match order {
                        1 => 0,
                        2 => 1,
                        _ => 2,
                    }
                );
            }

            #[test]
            fn cycle_min_outdegree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::cycle(order).min_outdegree(),
                    match order {
                        1 => 0,
                        2 => 1,
                        _ => 2,
                    }
                );
            }

            #[test]
            fn cycle_outdegree(order in 1..5_usize) {
                let digraph = <$type>::cycle(order);

                assert!(digraph.vertices().all(|u| {
                    digraph.outdegree(u)
                        == match order {
                            1 => 0,
                            2 => 1,
                            _ => 2,
                        }
                }));
            }

            #[test]
            fn cycle_outdegree_sequence(order in 1..5_usize) {
                assert!(<$type>::cycle(order).outdegree_sequence().all(|d| d
                    == match order {
                        1 => 0,
                        2 => 1,
                        _ => 2,
                    }));
            }

            #[test]
            fn cycle_semidegree_sequence(order in 1..5_usize) {
                assert!(<$type>::cycle(order).semidegree_sequence().all(|d| d
                    == match order {
                        1 => (0, 0),
                        2 => (1, 1),
                        _ => (2, 2),
                    }));
            }

            #[test]
            fn cycle_sinks(order in 1..5_usize) {
                assert!(if order == 1 {
                    <$type>::cycle(order).sinks().eq([0])
                } else {
                    <$type>::cycle(order).sinks().eq([])
                });
            }

            #[test]
            fn cycle_size(order in 1..5_usize) {
                assert_eq!(
                    <$type>::cycle(order).size(),
                    match order {
                        1 => 0,
                        2 => 2,
                        _ => order * 2
                    }
                );
            }

            #[test]
            fn cycle_sources(order in 1..5_usize) {
                assert!(if order == 1 {
                    <$type>::cycle(order).sources().eq([0])
                } else {
                    <$type>::cycle(order).sources().eq([])
                });
            }
        }
    };
}

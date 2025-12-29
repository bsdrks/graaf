//! Generate empty digraphs.
//!
//! An empty digraph has no arcs.
//!
//! # Examples
//!
//! ## Order 2
//!
//! Generate an empty digraph of order `2`.
//!
//! ![An empty digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/empty_2.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Empty,
//! };
//!
//! assert!(AdjacencyList::empty(2).arcs().eq([]));
//! ```
//!
//! ## Order 3
//!
//! Generate an empty digraph of order `3`.
//!
//! ![An empty digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/empty_3.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Empty,
//! };
//!
//! assert!(AdjacencyList::empty(3).arcs().eq([]));
//! ```
//!
//! ## Order 4
//!
//! Generate an empty digraph of order `4`.
//!
//! ![An empty digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/empty_4.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Empty,
//! };
//!
//! assert!(AdjacencyList::empty(4).arcs().eq([]));
//! ```
#![doc(alias = "edgeless")]

/// Empty digraphs
#[doc(alias = "Edgeless")]
pub trait Empty {
    /// Generate an empty digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ## Order 2
    ///
    /// Generate an empty digraph of order `2`.
    ///
    /// ![An empty digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/empty_2.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Empty,
    /// };
    ///
    /// assert!(AdjacencyList::empty(2).arcs().eq([]));
    /// ```
    ///
    /// ## Order 3
    ///
    /// Generate an empty digraph of order `3`.
    ///
    /// ![An empty digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/empty_3.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Empty,
    /// };
    ///
    /// assert!(AdjacencyList::empty(3).arcs().eq([]));
    /// ```
    ///
    /// ## Order 4
    ///
    /// Generate an empty digraph of order `4`.
    ///
    /// ![An empty digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/empty_4.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Empty,
    /// };
    ///
    /// assert!(AdjacencyList::empty(4).arcs().eq([]));
    /// ```
    #[doc(alias = "edgeless")]
    #[must_use]
    fn empty(order: usize) -> Self;

    /// Generate a trivial digraph.
    ///
    /// A trivial digraph has one vertex and no arcs.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Empty,
    /// };
    ///
    /// assert!(AdjacencyList::trivial().arcs().eq([]));
    /// ```
    #[doc(alias = "singleton")]
    #[must_use]
    fn trivial() -> Self
    where
        Self: Sized,
    {
        Self::empty(1)
    }
}

/// `Empty` tests
#[macro_export]
macro_rules! test_empty {
    ($type:ty) => {
        #[test]
        #[should_panic(expected = "a digraph has at least one vertex")]
        fn empty_0() {
            drop(<$type>::empty(0));
        }

        #[test]
        fn empty_1() {
            let digraph = <$type>::empty(1);

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn empty_1_complement() {
            let digraph = <$type>::empty(1).complement();

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn empty_2() {
            let digraph = <$type>::empty(2);

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn empty_2_complement() {
            let digraph = <$type>::empty(2).complement();

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([(0, 1), (1, 0)]));
        }

        #[test]
        fn empty_3() {
            let digraph = <$type>::empty(3);

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn empty_3_complement() {
            let digraph = <$type>::empty(3).complement();

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
        fn empty_trivial() {
            let digraph = <$type>::trivial();

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }
    };
}

/// `Empty` proptests with `Complement`
#[macro_export]
macro_rules! proptest_empty_complement {
    ($type:ty) => {
        use {
            proptest::proptest,
            $crate::Complement,
        };

        proptest! {
            #[test]
            fn empty_complement_size(order in 1..5_usize) {
                assert_eq!(
                    <$type>::empty(order).complement().size(),
                    order * (order - 1)
                );
            }
        }
    };
}

/// `Empty` proptests with `Complement` and `Complete`
#[macro_export]
macro_rules! proptest_empty_complete {
    ($type:ty) => {
        use {
            proptest::proptest,
            $crate::{
                Complement,
                Complete,
            },
        };

        proptest! {
            #[test]
            fn empty_complement_equals_complete(order in 1..5_usize) {
                assert_eq!(
                    <$type>::empty(order).complement(),
                    <$type>::complete(order)
                );
            }
        }
    };
}

/// `Empty` proptests
#[macro_export]
macro_rules! proptest_empty {
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
            fn empty_arcs(order in 1..5_usize) {
                assert!(<$type>::empty(order).arcs().eq([]));
            }

            #[test]
            fn empty_degree(order in 1..5_usize) {
                let digraph = <$type>::empty(order);

                assert!(digraph.vertices().all(|u| digraph.degree(u) == 0));
            }

            #[test]
            fn empty_degree_sequence(order in 1..5_usize) {
                assert!(<$type>::empty(order).degree_sequence().all(|d| d == 0));
            }

            #[test]
            fn empty_degree_sum_equals_2size(order in 1..5_usize) {
                let digraph = <$type>::empty(order);

                assert_eq!(
                    digraph
                        .vertices()
                        .fold(0, |acc, u| acc + digraph.degree(u)),
                    2 * digraph.size()
                );
            }

            #[test]
            fn empty_even_number_odd_degrees(order in 1..5_usize) {
                let digraph = <$type>::empty(order);

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
            fn empty_has_arc(order in 1..5_usize) {
                let digraph = <$type>::empty(order);

                assert!(digraph.vertices().all(|u| {
                    digraph.vertices().all(|v| !digraph.has_arc(u, v))
                }));
            }

            #[test]
            fn empty_has_edge(order in 1..5_usize) {
                let digraph = <$type>::empty(order);

                assert!(digraph.vertices().all(|u| {
                    digraph.vertices().all(|v| !digraph.has_edge(u, v))
                }));
            }

            #[test]
            fn empty_indegree(order in 1..5_usize) {
                let digraph = <$type>::empty(order);

                assert!(digraph.vertices().all(|u| digraph.indegree(u) == 0));
            }

            #[test]
            fn empty_indegree_sequence(order in 1..5_usize) {
                assert!(<$type>::empty(order)
                    .indegree_sequence()
                    .all(|d| d == 0));
            }

            #[test]
            fn empty_is_balanced(order in 1..5_usize) {
                assert!(<$type>::empty(order).is_balanced());
            }

            #[test]
            fn empty_is_complete(order in 1..5_usize) {
                assert!((order == 1) == <$type>::empty(order).is_complete());
            }

            #[test]
            fn empty_is_isolated(order in 1..5_usize) {
                let digraph = <$type>::empty(order);

                assert!(digraph.vertices().all(|u| digraph.is_isolated(u)));
            }

            #[test]
            fn empty_is_oriented(order in 1..5_usize) {
                assert!(<$type>::empty(order).is_oriented());
            }

            #[test]
            fn empty_is_pendant(order in 1..5_usize) {
                let digraph = <$type>::empty(order);

                assert!(digraph.vertices().all(|u| !digraph.is_pendant(u)));
            }

            #[test]
            fn empty_is_regular(order in 1..5_usize) {
                assert!(<$type>::empty(order).is_regular());
            }

            #[test]
            fn empty_is_semicomplete(order in 1..5_usize) {
                assert!((order == 1) == <$type>::empty(order).is_semicomplete());
            }

            #[test]
            fn empty_is_simple(order in 1..5_usize) {
                assert!(<$type>::empty(order).is_simple());
            }

            #[test]
            fn empty_is_sink(order in 1..5_usize) {
                let digraph = <$type>::empty(order);

                assert!(digraph.vertices().all(|u| digraph.is_sink(u)));
            }

            #[test]
            fn empty_is_source(order in 1..5_usize) {
                let digraph = <$type>::empty(order);

                assert!(digraph.vertices().all(|u| digraph.is_source(u)));
            }

            #[test]
            fn empty_is_spanning_subdigraph(order in 1..5_usize) {
                let digraph = <$type>::empty(order);

                assert!(digraph.is_spanning_subdigraph(&digraph));
            }

            #[test]
            fn empty_is_subdigraph(order in 1..5_usize) {
                let digraph = <$type>::empty(order);

                assert!(digraph.is_subdigraph(&digraph));
            }

            #[test]
            fn empty_is_superdigraph(order in 1..5_usize) {
                let digraph = <$type>::empty(order);

                assert!(digraph.is_superdigraph(&digraph));
            }

            #[test]
            fn empty_is_symmetric(order in 1..5_usize) {
                assert!(<$type>::empty(order).is_symmetric());
            }

            #[test]
            fn empty_is_tournament(order in 1..5_usize) {
                assert!((order == 1) == <$type>::empty(order).is_tournament());
            }

            #[test]
            fn empty_max_degree(order in 1..5_usize) {
                assert_eq!(<$type>::empty(order).max_degree(), 0);
            }

            #[test]
            fn empty_max_indegree(order in 1..5_usize) {
                assert_eq!(<$type>::empty(order).max_indegree(), 0);
            }

            #[test]
            fn empty_max_outdegree(order in 1..5_usize) {
                assert_eq!(<$type>::empty(order).max_outdegree(), 0);
            }

            #[test]
            fn empty_min_degree(order in 1..5_usize) {
                assert_eq!(<$type>::empty(order).min_degree(), 0);
            }

            #[test]
            fn empty_min_indegree(order in 1..5_usize) {
                assert_eq!(<$type>::empty(order).min_indegree(), 0);
            }

            #[test]
            fn empty_min_outdegree(order in 1..5_usize) {
                assert_eq!(<$type>::empty(order).min_outdegree(), 0);
            }

            #[test]
            fn empty_outdegree(order in 1..5_usize) {
                let digraph = <$type>::empty(order);

                assert!(digraph.vertices().all(|u| digraph.outdegree(u) == 0));
            }

            #[test]
            fn empty_outdegree_sequence(order in 1..5_usize) {
                assert!(<$type>::empty(order)
                    .outdegree_sequence()
                    .all(|d| d == 0));
            }

            #[test]
            fn empty_semidegree_sequence(order in 1..5_usize) {
                assert!(<$type>::empty(order)
                    .semidegree_sequence()
                    .all(|d| d == (0, 0)));
            }

            #[test]
            fn empty_sinks(order in 1..5_usize) {
                assert!(<$type>::empty(order).sinks().eq(0..order));
            }

            #[test]
            fn empty_size(order in 1..5_usize) {
                assert_eq!(<$type>::empty(order).size(), 0);
            }

            #[test]
            fn empty_sources(order in 1..5_usize) {
                assert!(<$type>::empty(order).sources().eq(0..order));
            }
        }
    };
}

//! Generate complete digraphs.
//!
//! In a complete digraph, an arc connects every ordered vertex pair.
//!
//! # Examples
//!
//! ## Order 2
//!
//! Generate a complete digraph of order `2`.
//!
//! ![A complete digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/complete_2.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Complete,
//! };
//!
//! assert!(AdjacencyList::complete(2).arcs().eq([(0, 1), (1, 0)]));
//! ```
//!
//! ## Order 3
//!
//! Generate a complete digraph of order `3`.
//!
//! ![A complete digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/complete_3.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Complete,
//! };
//!
//! assert!(AdjacencyList::complete(3).arcs().eq([
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
//! Generate a complete digraph of order `4`.
//!
//! ![A complete digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/complete_4.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Complete,
//! };
//!
//! assert!(AdjacencyList::complete(4).arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (0, 3),
//!     (1, 0),
//!     (1, 2),
//!     (1, 3),
//!     (2, 0),
//!     (2, 1),
//!     (2, 3),
//!     (3, 0),
//!     (3, 1),
//!     (3, 2)
//! ]));
//! ```

/// Complete digraphs
pub trait Complete {
    /// Generate a complete digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ## Order 2
    ///
    /// Generate a complete digraph of order `2`.
    ///
    /// ![A complete digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/complete_2.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Complete,
    /// };
    ///
    /// assert!(AdjacencyList::complete(2).arcs().eq([(0, 1), (1, 0)]));
    /// ```
    ///
    /// ## Order 3
    ///
    /// Generate a complete digraph of order `3`.
    ///
    /// ![A complete digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/complete_3.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Complete,
    /// };
    ///
    /// assert!(AdjacencyList::complete(3).arcs().eq([
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
    /// Generate a complete digraph of order `4`.
    ///
    /// ![A complete digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/complete_4.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Complete,
    /// };
    ///
    /// assert!(AdjacencyList::complete(4).arcs().eq([
    ///     (0, 1),
    ///     (0, 2),
    ///     (0, 3),
    ///     (1, 0),
    ///     (1, 2),
    ///     (1, 3),
    ///     (2, 0),
    ///     (2, 1),
    ///     (2, 3),
    ///     (3, 0),
    ///     (3, 1),
    ///     (3, 2)
    /// ]));
    /// ```
    #[must_use]
    fn complete(order: usize) -> Self;
}

/// `Complete` tests
#[macro_export]
macro_rules! test_complete {
    ($type:ty) => {
        #[test]
        #[should_panic(expected = "a digraph has at least one vertex")]
        fn complete_0() {
            let _ = <$type>::complete(0);
        }

        #[test]
        fn complete_1() {
            let digraph = <$type>::complete(1);

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn complete_1_complement() {
            let digraph = <$type>::complete(1).complement();

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn complete_2() {
            let digraph = <$type>::complete(2);

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([(0, 1), (1, 0)]));
        }

        #[test]
        fn complete_2_complement() {
            let digraph = <$type>::complete(2).complement();

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn complete_3() {
            let digraph = <$type>::complete(3);

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
        fn complete_3_complement() {
            let digraph = <$type>::complete(3).complement();

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([]));
        }
    };
}

/// `Complete` proptests
#[macro_export]
macro_rules! proptest_complete {
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
            fn complete_complement_equals_empty(order in 1..5_usize) {
                assert_eq!(
                    <$type>::complete(order).complement(),
                    <$type>::empty(order)
                );
            }

            #[test]
            fn complete_complement_size(order in 1..5_usize) {
                assert_eq!(<$type>::complete(order).complement().size(), 0);
            }

            #[test]
            fn complete_degree(order in 1..5_usize) {
                let digraph = <$type>::complete(order);
                let degree = order * 2 - 2;

                assert!(digraph
                    .vertices()
                    .all(|u| digraph.degree(u) == degree));
            }

            #[test]
            fn complete_degree_sequence(order in 1..5_usize) {
                let degree = order * 2 - 2;

                assert!(<$type>::complete(order)
                    .degree_sequence()
                    .all(|d| d == degree));
            }

            #[test]
            fn complete_degree_sum_equals_2size(order in 1..5_usize) {
                let digraph = <$type>::complete(order);

                assert_eq!(
                    digraph
                    .vertices()
                    .fold(0, |acc, u| acc + digraph.degree(u)),
                    2 * digraph.size()
                );
            }

            #[test]
            fn complete_even_number_odd_degrees(order in 1..5_usize) {
                let digraph = <$type>::complete(order);

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
            fn complete_has_edge(order in 1..5_usize) {
                let digraph = <$type>::complete(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (u + 1..order).all(|v| digraph.has_edge(u, v))));
            }

            #[test]
            fn complete_indegree(order in 1..5_usize) {
                let digraph = <$type>::complete(order);
                let indegree = order - 1;

                assert!(digraph
                    .vertices()
                    .all(|v| digraph.indegree(v) == indegree));
            }

            #[test]
            fn complete_indegree_sequence(order in 1..5_usize) {
                let indegree = order - 1;

                assert!(<$type>::complete(order)
                    .indegree_sequence()
                    .all(|d| d == indegree));
            }

            #[test]
            fn complete_is_balanced(order in 1..5_usize) {
                assert!(<$type>::complete(order).is_balanced());
            }

            #[test]
            fn complete_is_complete(order in 1..5_usize) {
                assert!(<$type>::complete(order).is_complete());
            }

            #[test]
            fn complete_is_isolated(order in 1..5_usize) {
                let digraph = <$type>::complete(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_isolated(u)));
            }

            #[test]
            fn complete_is_oriented(order in 1..5_usize) {
                assert!((order == 1) == <$type>::complete(order).is_oriented());
            }

            #[test]
            fn complete_is_pendant(order in 1..5_usize) {
                let digraph = <$type>::complete(order);

                assert!(digraph.vertices().all(|u| !digraph.is_pendant(u)));
            }

            #[test]
            fn complete_is_regular(order in 1..5_usize) {
                assert!(<$type>::complete(order).is_regular());
            }

            #[test]
            fn complete_is_semicomplete(order in 1..5_usize) {
                assert!(<$type>::complete(order).is_semicomplete());
            }

            #[test]
            fn complete_is_simple(order in 1..5_usize) {
                assert!(<$type>::complete(order).is_simple());
            }

            #[test]
            fn complete_is_spanning_subdigraph(order in 1..5_usize) {
                let digraph = <$type>::complete(order);

                assert!(digraph.is_spanning_subdigraph(&digraph));
            }

            #[test]
            fn complete_is_subdigraph(order in 1..5_usize) {
                let digraph = <$type>::complete(order);

                assert!(digraph.is_subdigraph(&digraph));
            }

            #[test]
            fn complete_is_superdigraph(order in 1..5_usize) {
                let digraph = <$type>::complete(order);

                assert!(digraph.is_superdigraph(&digraph));
            }

            #[test]
            fn complete_is_symmetric(order in 1..5_usize) {
                assert!(<$type>::complete(order).is_symmetric());
            }

            #[test]
            fn complete_is_tournament(order in 1..5_usize) {
                assert!(
                    (order == 1) == <$type>::complete(order).is_tournament()
                );
            }

            #[test]
            fn complete_max_degree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::complete(order).max_degree(),
                    if order == 1 { 0 } else { (order - 1) * 2 }
                );
            }

            #[test]
            fn complete_max_indegree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::complete(order).max_indegree(),
                    if order == 1 { 0 } else { order - 1 }
                );
            }

            #[test]
            fn complete_max_outdegree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::complete(order).max_outdegree(),
                    if order == 1 { 0 } else { order - 1 }
                );
            }

            #[test]
            fn complete_min_degree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::complete(order).min_degree(),
                    if order == 1 { 0 } else { (order - 1) * 2 }
                );
            }

            #[test]
            fn complete_min_indegree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::complete(order).min_indegree(),
                    if order == 1 { 0 } else { order - 1 }
                );
            }

            #[test]
            fn complete_min_outdegree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::complete(order).min_outdegree(),
                    if order == 1 { 0 } else { order - 1 }
                );
            }

            #[test]
            fn complete_order(order in 1..5_usize) {
                assert_eq!(<$type>::complete(order).order(), order);
            }

            #[test]
            fn complete_outdegree(order in 1..5_usize) {
                let digraph = <$type>::complete(order);
                let outdegree = order - 1;

                assert!(digraph
                    .vertices()
                    .all(|s| digraph.outdegree(s) == outdegree));
            }

            #[test]
            fn complete_outdegree_sequence(order in 1..5_usize) {
                let outdegree = order - 1;

                assert!(<$type>::complete(order)
                    .outdegree_sequence()
                    .all(|d| d == outdegree));
            }

            #[test]
            fn complete_semidegree_sequence(order in 1..5_usize) {
                let degree = order - 1;
                let pair = (degree, degree);

                assert!(<$type>::complete(order)
                    .semidegree_sequence()
                    .all(|d| d == pair));
            }

            #[test]
            fn complete_sinks(order in 1..5_usize) {
                assert!(if order == 1 {
                    <$type>::complete(order).sinks().eq([0])
                } else {
                    <$type>::complete(order).sinks().eq([])
                });
            }

            #[test]
            fn complete_size(order in 1..5_usize) {
                assert_eq!(<$type>::complete(order).size(), order * (order - 1));
            }

            #[test]
            fn complete_sources(order in 1..5_usize) {
                assert!(if order == 1 {
                    <$type>::complete(order).sources().eq([0])
                } else {
                    <$type>::complete(order).sources().eq([])
                });
            }
        }
    };
}

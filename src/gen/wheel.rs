//! Generate wheel digraphs.
//!
//! A wheel digraph is a circuit digraph with an additional vertex connected to
//! all others. A wheel digraph has `4` or more vertices.
//!
//! # Examples
//!
//! ## Order < 4
//!
//! A wheel digraph has at least four vertices.
//!
//! ```should_panic
//! use graaf::{
//!     AdjacencyList,
//!     Wheel,
//! };
//!
//! let _ = AdjacencyList::wheel(3);
//! ```
//!
//! ## Order 4
//!
//! Generate a wheel digraph of order `4`.
//!
//! ![A wheel digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/wheel_4-0.87.5.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Wheel,
//! };
//!
//! assert!(AdjacencyList::wheel(4).arcs().eq([
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
//!     (3, 2),
//! ]));
//! ```
//!
//! ## Order 5
//!
//! Generate a wheel digraph of order `5`.
//!
//! ![A wheel digraph of order `5`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/wheel_5-0.87.5.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Wheel,
//! };
//!
//! assert!(AdjacencyList::wheel(5).arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (0, 3),
//!     (0, 4),
//!     (1, 0),
//!     (1, 2),
//!     (1, 4),
//!     (2, 0),
//!     (2, 1),
//!     (2, 3),
//!     (3, 0),
//!     (3, 2),
//!     (3, 4),
//!     (4, 0),
//!     (4, 1),
//!     (4, 3),
//! ]));
//! ```
//!
//! ## Order 6
//!
//! Generate a wheel digraph of order `6`.
//!
//! ![A wheel digraph of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/wheel_6-0.87.5.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Wheel,
//! };
//!
//! assert!(AdjacencyList::wheel(6).arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (0, 3),
//!     (0, 4),
//!     (0, 5),
//!     (1, 0),
//!     (1, 2),
//!     (1, 5),
//!     (2, 0),
//!     (2, 1),
//!     (2, 3),
//!     (3, 0),
//!     (3, 2),
//!     (3, 4),
//!     (4, 0),
//!     (4, 3),
//!     (4, 5),
//!     (5, 0),
//!     (5, 1),
//!     (5, 4),
//! ]));
//! ```

/// Wheel digraphs
pub trait Wheel {
    /// Generate a wheel digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ## Order 4
    ///
    /// Generate a wheel digraph of order `4`.
    ///
    /// ![A wheel digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/wheel_4-0.87.5.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Wheel,
    /// };
    ///
    /// assert!(AdjacencyList::wheel(4).arcs().eq([
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
    ///     (3, 2),
    /// ]));
    /// ```
    ///
    /// ## Order 5
    ///
    /// Generate a wheel digraph of order `5`.
    ///
    /// ![A wheel digraph of order `5`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/wheel_5-0.87.5.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Wheel,
    /// };
    ///
    /// assert!(AdjacencyList::wheel(5).arcs().eq([
    ///     (0, 1),
    ///     (0, 2),
    ///     (0, 3),
    ///     (0, 4),
    ///     (1, 0),
    ///     (1, 2),
    ///     (1, 4),
    ///     (2, 0),
    ///     (2, 1),
    ///     (2, 3),
    ///     (3, 0),
    ///     (3, 2),
    ///     (3, 4),
    ///     (4, 0),
    ///     (4, 1),
    ///     (4, 3),
    /// ]));
    /// ```
    ///
    /// ## Order 6
    ///
    /// Generate a wheel digraph of order `6`.
    ///
    /// ![A wheel digraph of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/wheel_6-0.87.5.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Wheel,
    /// };
    ///
    /// assert!(AdjacencyList::wheel(6).arcs().eq([
    ///     (0, 1),
    ///     (0, 2),
    ///     (0, 3),
    ///     (0, 4),
    ///     (0, 5),
    ///     (1, 0),
    ///     (1, 2),
    ///     (1, 5),
    ///     (2, 0),
    ///     (2, 1),
    ///     (2, 3),
    ///     (3, 0),
    ///     (3, 2),
    ///     (3, 4),
    ///     (4, 0),
    ///     (4, 3),
    ///     (4, 5),
    ///     (5, 0),
    ///     (5, 1),
    ///     (5, 4),
    /// ]));
    /// ```
    #[must_use]
    fn wheel(order: usize) -> Self;
}

/// `Wheel` tests
#[macro_export]
macro_rules! test_wheel {
    ($type:ty) => {
        #[test]
        #[should_panic(
            expected = "a wheel digraph has at least four vertices"
        )]
        fn wheel_0() {
            let _ = <$type>::wheel(0);
        }

        #[test]
        #[should_panic(
            expected = "a wheel digraph has at least four vertices"
        )]
        fn wheel_1() {
            let _ = <$type>::wheel(1);
        }

        #[test]
        #[should_panic(
            expected = "a wheel digraph has at least four vertices"
        )]
        fn wheel_2() {
            let _ = <$type>::wheel(2);
        }

        #[test]
        #[should_panic(
            expected = "a wheel digraph has at least four vertices"
        )]
        fn wheel_3() {
            let _ = <$type>::wheel(3);
        }

        #[test]
        fn wheel_4() {
            let digraph = <$type>::wheel(4);

            assert_eq!(digraph.order(), 4);

            assert!(digraph.arcs().eq([
                (0, 1),
                (0, 2),
                (0, 3),
                (1, 0),
                (1, 2),
                (1, 3),
                (2, 0),
                (2, 1),
                (2, 3),
                (3, 0),
                (3, 1),
                (3, 2)
            ]));
        }

        #[test]
        fn wheel_4_complement() {
            let digraph = <$type>::wheel(4).complement();

            assert_eq!(digraph.order(), 4);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn wheel_5() {
            let digraph = <$type>::wheel(5);

            assert_eq!(digraph.order(), 5);

            assert!(digraph.arcs().eq([
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (1, 0),
                (1, 2),
                (1, 4),
                (2, 0),
                (2, 1),
                (2, 3),
                (3, 0),
                (3, 2),
                (3, 4),
                (4, 0),
                (4, 1),
                (4, 3)
            ]));
        }

        #[test]
        fn wheel_5_complement() {
            let digraph = <$type>::wheel(5).complement();

            assert_eq!(digraph.order(), 5);
            assert!(digraph.arcs().eq([(1, 3), (2, 4), (3, 1), (4, 2)]));
        }

        #[test]
        fn wheel_6() {
            let digraph = <$type>::wheel(6);

            assert_eq!(digraph.order(), 6);

            assert!(digraph.arcs().eq([
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (0, 5),
                (1, 0),
                (1, 2),
                (1, 5),
                (2, 0),
                (2, 1),
                (2, 3),
                (3, 0),
                (3, 2),
                (3, 4),
                (4, 0),
                (4, 3),
                (4, 5),
                (5, 0),
                (5, 1),
                (5, 4)
            ]));
        }

        #[test]
        fn wheel_6_complement() {
            let digraph = <$type>::wheel(6).complement();

            assert_eq!(digraph.order(), 6);

            assert!(digraph.arcs().eq([
                (1, 3),
                (1, 4),
                (2, 4),
                (2, 5),
                (3, 1),
                (3, 5),
                (4, 1),
                (4, 2),
                (5, 2),
                (5, 3)
            ]));
        }
    };
}

/// `Wheel` proptests
#[macro_export]
macro_rules! proptest_wheel {
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
            fn wheel_complement_size(order in 4..7_usize) {
                assert_eq!(
                    <$type>::wheel(order).complement().size(),
                    (order - 1) * (order - 4)
                );
            }

            #[test]
            fn wheel_degree(order in 4..7_usize) {
                let digraph = <$type>::wheel(order);

                assert_eq!(digraph.degree(0), (order - 1) * 2);
                assert!((1..order).all(|u| digraph.degree(u) == 6));
            }

            #[test]
            fn wheel_degree_sequence(order in 4..7_usize) {
                let digraph = <$type>::wheel(order);
                let degree_sequence = &mut digraph.degree_sequence();

                assert_eq!(degree_sequence.next(), Some((order - 1) * 2));
                assert!(degree_sequence.all(|d| d == 6));
            }

            #[test]
            fn wheel_degree_sum_equals_2size(order in 4..7_usize) {
                let digraph = <$type>::wheel(order);

                assert_eq!(
                    digraph
                        .vertices()
                        .fold(0, |acc, u| acc + digraph.degree(u)),
                    2 * digraph.size()
                );
            }

            #[test]
            fn wheel_even_number_odd_degrees(order in 4..7_usize) {
                let digraph = <$type>::wheel(order);

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
            fn wheel_has_edge(order in 4..7_usize) {
                let digraph = <$type>::wheel(order);

                assert!((1..order).all(|u| digraph.has_edge(0, u)
                    && digraph.has_edge(u, ((u + order + 1) % order))
                    && digraph.has_edge(u, ((u + order - 1) % order))));
            }

            #[test]
            fn wheel_indegree(order in 4..7_usize) {
                let digraph = <$type>::wheel(order);

                assert_eq!(digraph.indegree(0), order - 1);
                assert!((1..order).all(|u| digraph.indegree(u) == 3));
            }

            #[test]
            fn wheel_indegree_sequence(order in 4..7_usize) {
                let digraph = <$type>::wheel(order);
                let indegree_sequence = &mut digraph.indegree_sequence();

                assert_eq!(indegree_sequence.next(), Some(order - 1));
                assert!(indegree_sequence.all(|d| d == 3));
            }

            #[test]
            fn wheel_is_balanced(order in 4..7_usize) {
                assert!(<$type>::wheel(order).is_balanced());
            }

            #[test]
            fn wheel_is_complete(order in 4..7_usize) {
                assert!((order == 4) == <$type>::wheel(order).is_complete());
            }

            #[test]
            fn wheel_is_isolated(order in 4..7_usize) {
                let digraph = <$type>::wheel(order);

                assert!(digraph.vertices().all(|u| !digraph.is_isolated(u)));
            }

            #[test]
            fn wheel_is_oriented(order in 4..7_usize) {
                assert!(!<$type>::wheel(order).is_oriented());
            }

            #[test]
            fn wheel_is_pendant(order in 4..7_usize) {
                let digraph = <$type>::wheel(order);

                assert!(digraph.vertices().all(|u| !digraph.is_pendant(u)));
            }

            #[test]
            fn wheel_is_regular(order in 4..7_usize) {
                assert!((order == 4) == <$type>::wheel(order).is_regular());
            }

            #[test]
            fn wheel_is_semicomplete(order in 4..7_usize) {
                assert!((order == 4) == <$type>::wheel(order).is_semicomplete());
            }

            #[test]
            fn wheel_is_simple(order in 4..7_usize) {
                assert!(<$type>::wheel(order).is_simple());
            }

            #[test]
            fn wheel_is_sink(order in 4..7_usize) {
                let digraph = <$type>::wheel(order);

                assert!(digraph.vertices().all(|u| !digraph.is_sink(u)));
            }

            #[test]
            fn wheel_is_source(order in 4..7_usize) {
                let digraph = <$type>::wheel(order);

                assert!(digraph.vertices().all(|u| !digraph.is_source(u)));
            }

            #[test]
            fn wheel_is_spanning_subdigraph(order in 4..7_usize) {
                let digraph = <$type>::wheel(order);

                assert!(digraph.is_spanning_subdigraph(&digraph));
            }

            #[test]
            fn wheel_is_subdigraph(order in 4..7_usize) {
                let digraph = <$type>::wheel(order);

                assert!(digraph.is_subdigraph(&digraph));
            }

            #[test]
            fn wheel_is_superdigraph(order in 4..7_usize) {
                let digraph = <$type>::wheel(order);

                assert!(digraph.is_superdigraph(&digraph));
            }

            #[test]
            fn wheel_is_symmetric(order in 4..7_usize) {
                assert!(<$type>::wheel(order).is_symmetric());
            }

            #[test]
            fn wheel_is_tournament(order in 4..7_usize) {
                assert!(!<$type>::wheel(order).is_tournament());
            }

            #[test]
            fn wheel_max_degree(order in 4..7_usize) {
                assert_eq!(<$type>::wheel(order).max_degree(), (order - 1) * 2);
            }

            #[test]
            fn wheel_max_indegree(order in 4..7_usize) {
                assert_eq!(<$type>::wheel(order).max_indegree(), order - 1);
            }

            #[test]
            fn wheel_max_outdegree(order in 4..7_usize) {
                assert_eq!(<$type>::wheel(order).max_outdegree(), order - 1);
            }

            #[test]
            fn wheel_min_degree(order in 4..7_usize) {
                assert_eq!(<$type>::wheel(order).min_degree(), 6);
            }

            #[test]
            fn wheel_min_indegree(order in 4..7_usize) {
                assert_eq!(<$type>::wheel(order).min_indegree(), 3);
            }

            #[test]
            fn wheel_min_outdegree(order in 4..7_usize) {
                assert_eq!(<$type>::wheel(order).min_outdegree(), 3);
            }

            #[test]
            fn wheel_outdegree(order in 4..7_usize) {
                let digraph = <$type>::wheel(order);

                assert_eq!(digraph.outdegree(0), order - 1);
                assert!((1..order).all(|u| digraph.outdegree(u) == 3));
            }

            #[test]
            fn wheel_outdegree_sequence(order in 4..7_usize) {
                let digraph = <$type>::wheel(order);
                let outdegree_sequence = &mut digraph.outdegree_sequence();

                assert_eq!(outdegree_sequence.next(), Some(order - 1));
                assert!(outdegree_sequence.all(|d| d == 3));
            }

            #[test]
            fn wheel_semidegree_sequence(order in 4..7_usize) {
                let digraph = <$type>::wheel(order);
                let mut semidegree_sequence = digraph.semidegree_sequence();

                assert_eq!(
                    semidegree_sequence.next(),
                    Some((order - 1, order - 1))
                );

                assert!(semidegree_sequence.all(|d| d == (3, 3)));
            }

            #[test]
            fn wheel_sinks(order in 4..7_usize) {
                assert!(<$type>::wheel(order).sinks().eq([]));
            }

            #[test]
            fn wheel_size(order in 4..7_usize) {
                assert_eq!(<$type>::wheel(order).size(), (order - 1) * 4);
            }

            #[test]
            fn wheel_sources(order in 4..7_usize) {
                assert!(<$type>::wheel(order).sources().eq([]));
            }
        }
    }
}

//! Generate circuit digraphs.
//!
//! A circuit is an oriented cycle.
//!
//! # Examples
//!
//! ## Order 2
//!
//! Generate a circuit digraph of order `2`.
//!
//! ![A circuit digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/circuit_2.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Circuit,
//! };
//!
//! assert!(AdjacencyList::circuit(2).arcs().eq([(0, 1), (1, 0)]));
//! ```
//!
//! ## Order 3
//!
//! Generate a circuit digraph of order `3`.
//!
//! ![A circuit digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/circuit_3.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Circuit,
//! };
//!
//! assert!(
//!     AdjacencyList::circuit(3)
//!         .arcs()
//!         .eq([(0, 1), (1, 2), (2, 0)])
//! );
//! ```
//!
//! ## Order 4
//!
//! Generate a circuit digraph of order `4`.
//!
//! ![A circuit digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/circuit_4.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Circuit,
//! };
//!
//! assert!(AdjacencyList::circuit(4).arcs().eq([
//!     (0, 1),
//!     (1, 2),
//!     (2, 3),
//!     (3, 0)
//! ]));
//! ```

/// Circuit digraphs
pub trait Circuit {
    /// Generate a circuit digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ## Order 2
    ///
    /// Generate a circuit digraph of order `2`.
    ///
    /// ![A circuit digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/circuit_2.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Circuit,
    /// };
    ///
    /// assert!(AdjacencyList::circuit(2).arcs().eq([(0, 1), (1, 0)]));
    /// ```
    ///
    /// ## Order 3
    ///
    /// Generate a circuit digraph of order `3`.
    ///
    /// ![A circuit digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/circuit_3.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Circuit,
    /// };
    ///
    /// assert!(
    ///     AdjacencyList::circuit(3)
    ///         .arcs()
    ///         .eq([(0, 1), (1, 2), (2, 0)])
    /// );
    /// ```
    ///
    /// ## Order 4
    ///
    /// Generate a circuit digraph of order `4`.
    ///
    /// ![A circuit digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/circuit_4.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Circuit,
    /// };
    ///
    /// assert!(AdjacencyList::circuit(4).arcs().eq([
    ///     (0, 1),
    ///     (1, 2),
    ///     (2, 3),
    ///     (3, 0)
    /// ]));
    /// ```
    #[must_use]
    fn circuit(order: usize) -> Self;
}

/// `Circuit` tests
#[macro_export]
macro_rules! test_circuit {
    ($type:ty) => {
        #[test]
        #[should_panic(expected = "a digraph has at least one vertex")]
        fn circuit_0() {
            drop(<$type>::circuit(0));
        }

        #[test]
        fn circuit_1() {
            let digraph = <$type>::circuit(1);

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn circuit_1_complement() {
            let digraph = <$type>::circuit(1).complement();

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn circuit_2() {
            let digraph = <$type>::circuit(2);

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([(0, 1), (1, 0)]));
        }

        #[test]
        fn circuit_2_complement() {
            let digraph = <$type>::circuit(2).complement();

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn circuit_3() {
            let digraph = <$type>::circuit(3);

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([(0, 1), (1, 2), (2, 0)]));
        }

        #[test]
        fn circuit_3_complement() {
            let digraph = <$type>::circuit(3).complement();

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([(0, 2), (1, 0), (2, 1)]));
        }
    };
}

/// `Circuit` proptests
#[macro_export]
macro_rules! proptest_circuit {
    ($type:ty) => {
        use {
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
                Sources
            },
            proptest::proptest,
        };

        proptest! {
            #[test]
            fn circuit_complement_size(order in 1..5_usize) {
                assert_eq!(
                    <$type>::circuit(order).complement().size(),
                    order * order.saturating_sub(2)
                );
            }

            #[test]
            fn circuit_degree(order in 1..5_usize) {
                let digraph = <$type>::circuit(order);

                assert!(digraph
                    .vertices()
                    .all(|u| digraph.degree(u) == if order == 1 { 0 } else { 2 }));
            }

            #[test]
            fn circuit_degree_sequence(order in 1..5_usize) {
                assert!(<$type>::circuit(order)
                    .degree_sequence()
                    .all(|d| d == if order == 1 { 0 } else { 2 }));
            }

            #[test]
            fn circuit_degree_sum_equals_2size(order in 1..5_usize) {
                let digraph = <$type>::circuit(order);

                assert_eq!(
                    digraph
                        .vertices()
                        .fold(0, |acc, u| acc + digraph.degree(u)),
                    2 * digraph.size()
                );
            }

            #[test]
            fn circuit_even_number_odd_degrees(order in 1..5_usize) {
                let digraph = <$type>::circuit(order);

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
            fn circuit_has_edge(order in 1..5_usize) {
                let digraph = <$type>::circuit(order);

                assert!(digraph.vertices().all(|u| (u + 1..order)
                    .all(|v| (order == 2) == digraph.has_edge(u, v))));
            }

            #[test]
            fn circuit_indegree(order in 1..5_usize) {
                let digraph = <$type>::circuit(order);

                assert!(digraph
                    .vertices()
                    .all(|u| digraph.indegree(u) == usize::from(order != 1)));
            }

            #[test]
            fn circuit_indegree_sequence(order in 1..5_usize) {
                assert!(<$type>::circuit(order)
                    .indegree_sequence()
                    .all(|d| d == usize::from(order != 1)));
            }

            #[test]
            fn circuit_is_balanced(order in 1..5_usize) {
                assert!(<$type>::circuit(order).is_balanced());
            }

            #[test]
            fn circuit_is_complete(order in 1..5_usize) {
                assert!((order < 3) == <$type>::circuit(order).is_complete());
            }

            #[test]
            fn circuit_is_isolated(order in 1..5_usize) {
                let digraph = <$type>::circuit(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_isolated(u)));
            }

            #[test]
            fn circuit_is_oriented(order in 1..5_usize) {
                assert!((order == 2) != <$type>::circuit(order).is_oriented());
            }

            #[test]
            fn circuit_is_pendant(order in 1..5_usize) {
                let digraph = <$type>::circuit(order);

                assert!(digraph.vertices().all(|u| !digraph.is_pendant(u)));
            }

            #[test]
            fn circuit_is_regular(order in 1..5_usize) {
                assert!(<$type>::circuit(order).is_regular());
            }

            #[test]
            fn circuit_is_semicomplete(order in 1..5_usize) {
                assert!(
                    (order < 4) == <$type>::circuit(order).is_semicomplete()
                );
            }

            #[test]
            fn circuit_is_simple(order in 1..5_usize) {
                assert!(<$type>::circuit(order).is_simple());
            }

            #[test]
            fn circuit_is_sink(order in 1..5_usize) {
                let digraph = <$type>::circuit(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_sink(u)));
            }

            #[test]
            fn circuit_is_source(order in 1..5_usize) {
                let digraph = <$type>::circuit(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_source(u)));
            }

            #[test]
            fn circuit_is_spanning_subdigraph(order in 1..5_usize) {
                let digraph = <$type>::circuit(order);

                assert!(digraph.is_spanning_subdigraph(&digraph));
            }

            #[test]
            fn circuit_is_subdigraph(order in 1..5_usize) {
                let digraph = <$type>::circuit(order);

                assert!(digraph.is_subdigraph(&digraph));
            }

            #[test]
            fn circuit_is_superdigraph(order in 1..5_usize) {
                let digraph = <$type>::circuit(order);

                assert!(digraph.is_superdigraph(&digraph));
            }

            #[test]
            fn circuit_is_symmetric(order in 1..5_usize) {
                assert!((order < 3) == <$type>::circuit(order).is_symmetric());
            }

            #[test]
            fn circuit_is_tournament(order in 1..5_usize) {
                assert!(
                    (order == 1 || order == 3)
                        == <$type>::circuit(order).is_tournament()
                );
            }

            #[test]
            fn circuit_max_degree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::circuit(order).max_degree(),
                    if order == 1 { 0 } else { 2 }
                );
            }

            #[test]
            fn circuit_max_indegree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::circuit(order).max_indegree(),
                    if order == 1 { 0 } else { 1 }
                );
            }

            #[test]
            fn circuit_max_outdegree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::circuit(order).max_outdegree(),
                    if order == 1 { 0 } else { 1 }
                );
            }

            #[test]
            fn circuit_min_degree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::circuit(order).min_degree(),
                    if order == 1 { 0 } else { 2 }
                );
            }

            #[test]
            fn circuit_min_indegree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::circuit(order).min_indegree(),
                    if order == 1 { 0 } else { 1 }
                );
            }

            #[test]
            fn circuit_min_outdegree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::circuit(order).min_outdegree(),
                    if order == 1 { 0 } else { 1 }
                );
            }

            #[test]
            fn circuit_outdegree(order in 1..5_usize) {
                let digraph = <$type>::circuit(order);

                assert!(digraph
                    .vertices()
                    .all(|u| digraph.outdegree(u) == usize::from(order != 1)));
            }

            #[test]
            fn circuit_outdegree_sequence(order in 1..5_usize) {
                assert!(<$type>::circuit(order)
                    .outdegree_sequence()
                    .all(|d| d == usize::from(order != 1)));
            }

            #[test]
            fn circuit_semidegree_sequence(order in 1..5_usize) {
                assert!(<$type>::circuit(order)
                    .semidegree_sequence()
                    .all(|d| d == if order == 1 { (0, 0) } else { (1, 1) }));
            }

            #[test]
            fn circuit_sinks(order in 1..5_usize) {
                let digraph = <$type>::circuit(order);
                let sinks = digraph.sinks();

                assert!(if order == 1 { sinks.eq([0]) } else { sinks.eq([]) });
            }

            #[test]
            fn circuit_size(order in 1..5_usize) {
                assert_eq!(
                    <$type>::circuit(order).size(),
                    if order == 1 { 0 } else { order }
                );
            }

            #[test]
            fn circuit_sources(order in 1..5_usize) {
                let digraph = <$type>::circuit(order);
                let sources = digraph.sources();

                assert!(
                    if order == 1 { sources.eq([0]) } else { sources.eq([]) }
                );
            }
        }
    }
}

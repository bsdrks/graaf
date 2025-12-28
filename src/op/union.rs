//! Return the union of two digraphs.
//!
//! # Examples
//!
//! The union of a cycle digraph and a star digraph form a wheel digraph.
//!
//! The cycle digraph:
//!
//! ![A cycle digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/union_cycle_1-0.88.4.svg?)
//!
//! The star digraph:
//!
//! ![A star digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/union_star_1-0.88.4.svg?)
//!
//! The union, a wheel digraph:
//!
//! ![The union forms a wheel digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/union_union_1-0.88.4.svg?)
//!
//! ```
//! use {
//!     graaf::{
//!         AdjacencyList,
//!         Arcs,
//!         Cycle,
//!         Star,
//!         Union,
//!         Wheel,
//!     },
//!     std::collections::BTreeSet,
//! };
//!
//! let cycle = AdjacencyList::from(vec![
//!     BTreeSet::new(),
//!     BTreeSet::from([2, 4]),
//!     BTreeSet::from([1, 3]),
//!     BTreeSet::from([2, 4]),
//!     BTreeSet::from([1, 3]),
//! ]);
//!
//! let star = AdjacencyList::from(vec![
//!     BTreeSet::from([1, 2, 3, 4]),
//!     BTreeSet::from([0]),
//!     BTreeSet::from([0]),
//!     BTreeSet::from([0]),
//!     BTreeSet::from([0]),
//! ]);
//!
//! let wheel = AdjacencyList::wheel(5);
//!
//! assert!(cycle.union(&star).arcs().eq(wheel.arcs()));
//! ```

/// Digraph union
pub trait Union {
    /// Return the union of two digraphs.
    ///
    /// # Arguments
    ///
    /// * `other`: The other digraph.
    ///
    /// # Examples
    ///
    /// The union of a cycle digraph and a star digraph form a wheel digraph.
    ///
    /// The cycle digraph:
    ///
    /// ![A cycle digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/union_cycle_1-0.88.4.svg?)
    ///
    /// The star digraph:
    ///
    /// ![A star digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/union_star_1-0.88.4.svg?)
    ///
    /// The union, a wheel digraph:
    ///
    /// ![The union forms a wheel digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/union_union_1-0.88.4.svg?)
    ///
    /// ```
    /// use {
    ///     graaf::{
    ///         AdjacencyList,
    ///         Arcs,
    ///         Cycle,
    ///         Star,
    ///         Union,
    ///         Wheel,
    ///     },
    ///     std::collections::BTreeSet,
    /// };
    ///
    /// let cycle = AdjacencyList::from(vec![
    ///     BTreeSet::new(),
    ///     BTreeSet::from([2, 4]),
    ///     BTreeSet::from([1, 3]),
    ///     BTreeSet::from([2, 4]),
    ///     BTreeSet::from([1, 3]),
    /// ]);
    ///
    /// let star = AdjacencyList::from(vec![
    ///     BTreeSet::from([1, 2, 3, 4]),
    ///     BTreeSet::from([0]),
    ///     BTreeSet::from([0]),
    ///     BTreeSet::from([0]),
    ///     BTreeSet::from([0]),
    /// ]);
    ///
    /// let wheel = AdjacencyList::wheel(5);
    ///
    /// assert!(cycle.union(&star).arcs().eq(wheel.arcs()));
    /// ```
    #[must_use]
    fn union(&self, other: &Self) -> Self;
}

/// `Union` proptests
#[macro_export]
macro_rules! proptest_union {
    ($type:ty) => {
        use proptest::proptest;

        proptest! {
            #[test]
            fn union_biclique_commutative(
                m in 1..5_usize,
                n in 1..5_usize
            ) {
                let biclique = <$type>::biclique(m, n);
                let other = <$type>::biclique(n, m);

                assert_eq!(biclique.union(&other), other.union(&biclique));
            }

            #[test]
            fn union_biclique_complement_is_complete(
                m in 1..5_usize,
                n in 1..5_usize
            ) {
                let biclique = <$type>::biclique(m, n);
                let complement = biclique.complement();

                assert!(biclique.union(&complement).is_complete());
                assert!(complement.union(&biclique).is_complete());
            }

            #[test]
            fn union_circuit_complement_is_complete(order in 3..5_usize) {
                let circuit = <$type>::circuit(order);
                let complement = circuit.complement();

                assert!(circuit.union(&complement).is_complete());
                assert!(complement.union(&circuit).is_complete());
            }

            #[test]
            fn union_complete_commutative(order in 1..5_usize) {
                let complete = <$type>::complete(order);
                let other = <$type>::complete(order);

                assert_eq!(complete.union(&other), other.union(&complete));
            }

            #[test]
            fn union_complete_complement_is_complete(order in 1..5_usize) {
                let complete = <$type>::complete(order);
                let complement = complete.complement();

                assert!(complete.union(&complement).is_complete());
                assert!(complement.union(&complete).is_complete());
            }

            #[test]
            fn union_circuit_cycle(order in 1..5_usize) {
                let circuit = <$type>::circuit(order);
                let converse = circuit.converse();
                let cycle = <$type>::cycle(order);

                assert!(circuit.union(&converse).arcs().eq(cycle.arcs()));
                assert!(converse.union(&circuit).arcs().eq(cycle.arcs()));
            }

            #[test]
            fn union_circuit_path(order in 1..5_usize) {
                let circuit = <$type>::circuit(order);
                let path = <$type>::path(order);
                let mut bridge = <$type>::empty(order);

                if order > 1 {
                    bridge.add_arc(order - 1, 0);
                }

                assert!(bridge.union(&path).arcs().eq(circuit.arcs()));
                assert!(path.union(&bridge).arcs().eq(circuit.arcs()));
            }

            #[test]
            fn union_cycle_commutative(order in 1..5_usize) {
                let cycle = <$type>::cycle(order);
                let other = <$type>::cycle(order);

                assert_eq!(cycle.union(&other), other.union(&cycle));
            }

            #[test]
            fn union_cycle_complement_is_complete(order in 1..5_usize) {
                let cycle = <$type>::cycle(order);
                let complement = cycle.complement();

                assert!(cycle.union(&complement).is_complete());
                assert!(complement.union(&cycle).is_complete());
            }

            #[test]
            fn union_empty_commutative(order in 1..5_usize) {
                let empty = <$type>::empty(order);
                let other = <$type>::empty(order);

                assert_eq!(empty.union(&other), other.union(&empty));
            }

            #[test]
            fn union_empty_complement_is_complete(order in 1..5_usize) {
                let empty = <$type>::empty(order);
                let complement = empty.complement();

                assert!(empty.union(&complement).is_complete());
                assert!(complement.union(&empty).is_complete());
            }

            #[test]
            fn union_empty_id_biclique(m in 1..5_usize, n in 1..5_usize) {
                let digraph = <$type>::biclique(m, n);
                let empty = <$type>::empty(m + n);

                assert!(empty.union(&digraph).arcs().eq(digraph.arcs()));
                assert!(digraph.union(&empty).arcs().eq(digraph.arcs()));
            }

            #[test]
            fn union_empty_id_circuit(order in 3..5_usize) {
                let digraph = <$type>::circuit(order);
                let empty = <$type>::empty(order);

                assert!(empty.union(&digraph).arcs().eq(digraph.arcs()));
                assert!(digraph.union(&empty).arcs().eq(digraph.arcs()));
            }

            #[test]
            fn union_empty_id_cycle(order in 1..5_usize) {
                let digraph = <$type>::cycle(order);
                let empty = <$type>::empty(order);

                assert!(empty.union(&digraph).arcs().eq(digraph.arcs()));
                assert!(digraph.union(&empty).arcs().eq(digraph.arcs()));
            }

            #[test]
            fn union_empty_id_empty(order in 1..5_usize) {
                let digraph = <$type>::empty(order);
                let empty = <$type>::empty(order);

                assert!(empty.union(&digraph).arcs().eq(digraph.arcs()));
                assert!(digraph.union(&empty).arcs().eq(digraph.arcs()));
            }

            #[test]
            fn union_empty_id_erdos_renyi(
                order in 1..5_usize,
                p in 0.0..1.0,
                seed in 0..10_u64
            ) {
                let digraph = <$type>::erdos_renyi(order, p, seed);
                let empty = <$type>::empty(order);

                assert!(empty.union(&digraph).arcs().eq(digraph.arcs()));
                assert!(digraph.union(&empty).arcs().eq(digraph.arcs()));
            }

            #[test]
            fn union_empty_id_path(order in 1..5_usize) {
                let digraph = <$type>::path(order);
                let empty = <$type>::empty(order);

                assert!(empty.union(&digraph).arcs().eq(digraph.arcs()));
                assert!(digraph.union(&empty).arcs().eq(digraph.arcs()));
            }

            #[test]
            fn union_empty_id_random_tournament(
                order in 1..5_usize,
                seed in 0..10_u64
            ) {
                let digraph = <$type>::random_tournament(order, seed);
                let empty = <$type>::empty(order);

                assert!(empty.union(&digraph).arcs().eq(digraph.arcs()));
                assert!(digraph.union(&empty).arcs().eq(digraph.arcs()));
            }

            #[test]
            fn union_empty_id_star(order in 1..5_usize) {
                let digraph = <$type>::star(order);
                let empty = <$type>::empty(order);

                assert!(empty.union(&digraph).arcs().eq(digraph.arcs()));
                assert!(digraph.union(&empty).arcs().eq(digraph.arcs()));
            }

            #[test]
            fn union_empty_id_wheel(order in 4..7_usize) {
                let digraph = <$type>::wheel(order);
                let empty = <$type>::empty(order);

                assert!(empty.union(&digraph).arcs().eq(digraph.arcs()));
                assert!(digraph.union(&empty).arcs().eq(digraph.arcs()));
            }

            #[test]
            fn union_erdos_renyi_commutative(
                order_1 in 1..5_usize,
                order_2 in 1..5_usize,
                p in 0.0..1.0,
                seed_1 in 0..10_u64,
                seed_2 in 0..10_u64
            ) {
                let digraph = <$type>::erdos_renyi(order_1, p, seed_1);
                let other = <$type>::erdos_renyi(order_2, p, seed_2);

                assert_eq!(digraph.union(&other), other.union(&digraph));
            }

            #[test]
            fn union_erdos_renyi_complement_is_complete(
                order in 1..5_usize,
                p in 0.0..1.0,
                seed in 0..10_u64
            ) {
                let digraph = <$type>::erdos_renyi(order, p, seed);
                let complement = digraph.complement();

                assert!(digraph.union(&complement).is_complete());
                assert!(complement.union(&digraph).is_complete());
            }

            #[test]
            fn union_erdos_renyi_size_gte(
                order_1 in 1..5_usize,
                order_2 in 1..5_usize,
                p in 0.0..1.0,
                seed_1 in 0..10_u64,
                seed_2 in 0..10_u64
            ) {
                let digraph = <$type>::erdos_renyi(order_1, p, seed_1);
                let other = <$type>::erdos_renyi(order_2, p, seed_2);

                assert!(digraph.size() + other.size() >= digraph.union(&other)
                    .size());
            }

            #[test]
            fn union_random_recursive_tree_commutative(
                order_1 in 1..5_usize,
                order_2 in 1..5_usize,
                seed_1 in 0..10_u64,
                seed_2 in 0..10_u64
            ) {
                let digraph = <$type>::random_recursive_tree(order_1, seed_1);
                let other = <$type>::random_recursive_tree(order_2, seed_2);

                assert_eq!(digraph.union(&other), other.union(&digraph));
            }

            #[test]
            fn union_random_recursive_tree_complement_is_complete(
                order in 1..5_usize,
                seed in 0..10_u64
            ) {
                let digraph = <$type>::random_recursive_tree(order, seed);
                let complement = digraph.complement();

                assert!(digraph.union(&complement).is_complete());
                assert!(complement.union(&digraph).is_complete());
            }

            #[test]
            fn union_path_commutative(
                order_1 in 1..5_usize,
                order_2 in 1..5_usize
            ) {
                let digraph = <$type>::path(order_1);
                let other = <$type>::path(order_2);

                assert_eq!(digraph.union(&other), other.union(&digraph));
            }

            #[test]
            fn union_path_complement_is_complete(order in 1..5_usize) {
                let digraph = <$type>::path(order);
                let complement = digraph.complement();

                assert!(digraph.union(&complement).is_complete());
                assert!(complement.union(&digraph).is_complete());

            }

            #[test]
            fn union_random_tournament_commutative(
                order_1 in 1..5_usize,
                order_2 in 1..5_usize,
                seed_1 in 0..10_u64,
                seed_2 in 0..10_u64
            ) {
                let digraph = <$type>::random_tournament(order_1, seed_1);
                let other = <$type>::random_tournament(order_2, seed_2);

                assert_eq!(digraph.union(&other), other.union(&digraph));
            }

            #[test]
            fn union_random_tournament_size_gte(
                order_1 in 1..5_usize,
                order_2 in 1..5_usize,
                seed_1 in 0..10_u64,
                seed_2 in 0..10_u64
            ) {
                let digraph = <$type>::random_tournament(order_1, seed_1);
                let other = <$type>::random_tournament(order_2, seed_2);

                assert!(digraph.size() + other.size() >= digraph.union(&other)
                    .size());
            }

            #[test]
            fn union_random_tournament_complement_is_complete(
                order in 1..5_usize,
                seed in 0..10_u64
            ) {
                let digraph = <$type>::random_tournament(order, seed);
                let complement = digraph.complement();

                assert!(digraph.union(&complement).is_complete());
                assert!(complement.union(&digraph).is_complete());
            }

            #[test]
            fn union_star_commutative(
                order_1 in 1..5_usize,
                order_2 in 1..5_usize
            ) {
                let digraph = <$type>::star(order_1);
                let other = <$type>::star(order_2);

                assert_eq!(digraph.union(&other), other.union(&digraph));
            }

            #[test]
            fn union_star_complement_is_complete(order in 1..5_usize) {
                let digraph = <$type>::star(order);
                let complement = digraph.complement();

                assert!(digraph.union(&complement).is_complete());
                assert!(complement.union(&digraph).is_complete());
            }

            #[test]
            fn union_wheel_commutative(
                order_1 in 4..7_usize,
                order_2 in 4..7_usize
            ) {
                let digraph = <$type>::wheel(order_1);
                let other = <$type>::wheel(order_2);

                assert_eq!(digraph.union(&other), other.union(&digraph));
            }

            #[test]
            fn union_wheel_complement_is_complete(order in 4..7_usize) {
                let digraph = <$type>::wheel(order);
                let complement = digraph.complement();

                assert!(digraph.union(&complement).is_complete());
                assert!(complement.union(&digraph).is_complete());
            }

        }
    };
}

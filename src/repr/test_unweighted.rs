//! Common tests for unweighted digraphs.

/// Common tests for unweighted digraphs.
#[macro_export]
macro_rules! test_unweighted {
    ($type:ident, $fixture:path) => {
        use {
            $crate::{
                proptest_strategy::arc,
                $fixture::{
                    bang_jensen_196,
                    bang_jensen_34,
                    bang_jensen_94,
                    kattis_builddeps,
                    kattis_cantinaofbabel_1,
                    kattis_cantinaofbabel_2,
                    kattis_escapewallmaria_1,
                    kattis_escapewallmaria_2,
                    kattis_escapewallmaria_3,
                },
                Biclique,
                Circuit,
                Complement,
                Complete,
                Converse,
                Cycle,
                Degree,
                DegreeSequence,
                ErdosRenyi,
                GrowingNetwork,
                HasEdge,
                HasWalk,
                InNeighbors,
                IndegreeSequence,
                IsBalanced,
                IsComplete,
                IsIsolated,
                IsOriented,
                IsPendant,
                IsRegular,
                IsSemicomplete,
                IsSimple,
                IsSpanningSubdigraph,
                IsSubdigraph,
                IsSuperdigraph,
                IsSymmetric,
                IsTournament,
                OutdegreeSequence,
                Path,
                RandomTournament,
                SemidegreeSequence,
                Sinks,
                Sources,
                Star,
                Union,
                Wheel,
            },
            proptest::proptest,
        };

        proptest! {
            #[test]
            fn add_arc_arc_weight((u, v) in arc(25_usize)) {
                let mut digraph = $type::empty(100);

                digraph.add_arc(u, v);

                assert!(digraph.vertices().all(|x| digraph.vertices().all(|y| {
                    digraph.arc_weight(x, y) == (x == u && y == v).then_some(&1)
                })));
            }

            #[test]
            fn add_arc_degree((u, v) in arc(25_usize)) {
                let mut digraph = $type::empty(100);

                digraph.add_arc(u, v);

                assert!(digraph.vertices().all(|x| {
                    digraph.degree(x) == usize::from(x == u) + usize::from(x == v)
                }));
            }

            #[test]
            fn add_arc_has_arc((u, v) in arc(25_usize)) {
                let mut digraph = $type::empty(100);

                digraph.add_arc(u, v);

                assert!(digraph.has_arc(u, v));
            }

            #[test]
            fn add_arc_indegree((u, v) in arc(25_usize)) {
                let mut digraph = $type::empty(100);

                digraph.add_arc(u, v);

                assert!(digraph
                    .vertices()
                    .all(|x| digraph.indegree(x) == usize::from(x == v)));
            }

            #[test]
            fn add_arc_outdegree((u, v) in arc(25_usize)) {
                let mut digraph = $type::empty(100);

                digraph.add_arc(u, v);

                assert!(digraph
                    .vertices()
                    .all(|x| digraph.outdegree(x) == usize::from(x == u)));
            }

            #[test]
            fn add_arc_remove_arc((u, v) in arc(25_usize)) {
                let digraph = $type::empty(100);
                let mut h = digraph.clone();

                h.add_arc(u, v);

                assert!(digraph.vertices().all(|x| digraph
                    .vertices()
                    .all(|y| h.remove_arc(x, y) == (x == u && y == v))));

                assert_eq!(digraph, h);
            }

            #[test]
            fn biclique_1_n_equals_star_n_plus_1(n in 1..25_usize) {
                assert_eq!($type::biclique(1, n), $type::star(n + 1));
            }

            #[test]
            fn biclique_complement_size(m in 1..25_usize, n in 1..25_usize) {
                assert_eq!(
                    $type::biclique(m, n).complement().size(),
                    m * (m - 1) + n * (n - 1)
                );
            }

            #[test]
            fn biclique_degree(m in 1..25_usize, n in 1..25_usize) {
                let digraph = $type::biclique(m, n);
                let clique_1_degree = n * 2;
                let clique_2_degree = m * 2;

                assert!((0..m).all(|u| digraph.degree(u) == clique_1_degree));
                assert!((m..m + n).all(|u| digraph.degree(u) == clique_2_degree));
            }

            #[test]
            fn biclique_degree_sequence(m in 1..25_usize, n in 1..25_usize) {
                let digraph = $type::biclique(m, n);
                let degree_sequence = &mut digraph.degree_sequence();
                let clique_1_degree = n * 2;
                let clique_2_degree = m * 2;

                assert!(degree_sequence.take(m).all(|d| d == clique_1_degree));
                assert!(degree_sequence.all(|d| d == clique_2_degree));
            }

            #[test]
            fn biclique_degree_sum_equals_2size(
                m in 1..25_usize,
                n in 1..25_usize
            ) {
                let digraph = $type::biclique(m, n);

                assert_eq!(
                    digraph.vertices().fold(0, |acc, u| acc + digraph.degree(u)),
                    2 * digraph.size()
                );
            }

            #[test]
            fn biclique_even_number_odd_degrees(
                m in 1..25_usize,
                n in 1..25_usize
            ) {
                let digraph = $type::biclique(m, n);

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
            fn biclique_has_arc(m in 1..25_usize, n in 1..25_usize) {
                let digraph = $type::biclique(m, n);
                let order = m + n;

                assert!(
                    (0..m).all(|u| (0..m).all(|v| !digraph.has_arc(u, v)))
                        && (m..order)
                            .all(|u| (m..order).all(|v| !digraph.has_arc(u, v)))
                        && (0..m)
                            .all(|u| (m..order).all(|v| digraph.has_arc(u, v)))
                );
            }

            #[test]
            fn biclique_has_edge(m in 1..25_usize, n in 1..25_usize) {
                let digraph = $type::biclique(m, n);
                let order = m + n;

                assert!(
                    (0..m).all(|u| (0..m).all(|v| !digraph.has_edge(u, v)))
                        && (m..order)
                            .all(|u| (m..order).all(|v| !digraph.has_edge(u, v)))
                        && (0..m)
                            .all(|u| (m..order).all(|v| digraph.has_edge(u, v)))
                );
            }

            #[test]
            fn biclique_in_neighbors(m in 1..25_usize, n in 1..25_usize) {
                let digraph = $type::biclique(m, n);
                let order = m + n;

                assert!(
                    (0..m).all(|u| digraph.in_neighbors(u).eq(m..order))
                        && (m..order).all(|u| digraph.in_neighbors(u).eq(0..m))
                );
            }

            #[test]
            fn biclique_indegree(m in 1..25_usize, n in 1..25_usize) {
                let digraph = $type::biclique(m, n);

                assert!(
                    (0..m).all(|u| digraph.indegree(u) == n)
                        && (m..m + n).all(|u| digraph.indegree(u) == m)
                );
            }

            #[test]
            fn biclique_indegree_sequence(m in 1..25_usize, n in 1..25_usize) {
                let digraph = $type::biclique(m, n);
                let indegree_sequence = &mut digraph.indegree_sequence();

                assert!(indegree_sequence.take(m).all(|d| d == n));
                assert!(indegree_sequence.all(|d| d == m));
            }

            #[test]
            fn biclique_is_balanced(m in 1..25_usize, n in 1..25_usize) {
                assert!($type::biclique(m, n).is_balanced());
            }

            #[test]
            fn biclique_is_complete(m in 1..25_usize, n in 1..25_usize) {
                assert!(
                    ((m, n) == (1, 1)) == $type::biclique(m, n).is_complete()
                );
            }

            #[test]
            fn biclique_is_isolated(m in 1..25_usize, n in 1..25_usize) {
                let digraph = $type::biclique(m, n);

                assert!(digraph.vertices().all(|u| !digraph.is_isolated(u)));
            }

            #[test]
            fn biclique_is_oriented(m in 1..25_usize, n in 1..25_usize) {
                assert!(!$type::biclique(m, n).is_oriented());
            }

            #[test]
            fn biclique_is_pendant(m in 1..25_usize, n in 1..25_usize) {
                let digraph = $type::biclique(m, n);

                assert!(digraph.vertices().all(|u| !digraph.is_pendant(u)));
            }

            #[test]
            fn biclique_is_regular(m in 1..25_usize, n in 1..25_usize) {
                assert!($type::biclique(m, n).is_regular() == (m == n));
            }

            #[test]
            fn biclique_is_semicomplete(m in 1..25_usize, n in 1..25_usize) {
                assert!(
                    ((m, n) == (1, 1))
                        == $type::biclique(m, n).is_semicomplete()
                );
            }

            #[test]
            fn biclique_is_simple(m in 1..25_usize, n in 1..25_usize) {
                assert!($type::biclique(m, n).is_simple());
            }

            #[test]
            fn biclique_is_spanning_subdigraph(
                m in 1..25_usize,
                n in 1..25_usize
            ) {
                let digraph = $type::biclique(m, n);

                assert!(digraph.is_spanning_subdigraph(&digraph));
            }

            #[test]
            fn biclique_is_subdigraph(m in 1..25_usize, n in 1..25_usize) {
                let digraph = $type::biclique(m, n);

                assert!(digraph.is_subdigraph(&digraph));
            }

            #[test]
            fn biclique_is_superdigraph(m in 1..25_usize, n in 1..25_usize) {
                let digraph = $type::biclique(m, n);

                assert!(digraph.is_superdigraph(&digraph));
            }

            #[test]
            fn biclique_is_symmetric(m in 1..25_usize, n in 1..25_usize) {
                assert!($type::biclique(m, n).is_symmetric());
            }

            #[test]
            fn biclique_is_tournament(m in 1..25_usize, n in 1..25_usize) {
                assert!(!$type::biclique(m, n).is_tournament());
            }

            #[test]
            fn biclique_max_degree(m in 1..25_usize, n in 1..25_usize) {
                let digraph = $type::biclique(m, n);

                assert_eq!(digraph.max_degree(), m.max(n) * 2);
            }

            #[test]
            fn biclique_max_indegree(m in 1..25_usize, n in 1..25_usize) {
                let digraph = $type::biclique(m, n);

                assert_eq!(digraph.max_indegree(), m.max(n));
            }

            #[test]
            fn biclique_max_outdegree(m in 1..25_usize, n in 1..25_usize) {
                let digraph = $type::biclique(m, n);

                assert_eq!(digraph.max_outdegree(), m.max(n));
            }

            #[test]
            fn biclique_min_degree(m in 1..25_usize, n in 1..25_usize) {
                let digraph = $type::biclique(m, n);

                assert_eq!(digraph.min_degree(), m.min(n) * 2);
            }

            #[test]
            fn biclique_min_indegree(m in 1..25_usize, n in 1..25_usize) {
                let digraph = $type::biclique(m, n);

                assert_eq!(digraph.min_indegree(), m.min(n));
            }

            #[test]
            fn biclique_min_outdegree(m in 1..25_usize, n in 1..25_usize) {
                let digraph = $type::biclique(m, n);

                assert_eq!(digraph.min_outdegree(), m.min(n));
            }

            #[test]
            fn biclique_order(m in 1..25_usize, n in 1..25_usize) {
                assert_eq!($type::biclique(m, n).order(), m + n);
            }

            #[test]
            fn biclique_out_neighbors(m in 1..25_usize, n in 1..25_usize) {
                let digraph = $type::biclique(m, n);
                let order = m + n;

                assert!(
                    (0..m).all(|u| digraph.out_neighbors(u).eq(m..order))
                        && (m..order).all(|u| digraph.out_neighbors(u).eq(0..m))
                );
            }

            #[test]
            fn biclique_out_neighbors_weighted(
                m in 1..25_usize,
                n in 1..25_usize
            ) {
                let digraph = $type::biclique(m, n);
                let order = m + n;

                assert!(
                    (0..m).all(|u| {
                        digraph
                            .out_neighbors_weighted(u)
                            .eq((m..order).map(|v| (v, &1)))
                    }) && (m..order).all(|u| {
                        digraph
                            .out_neighbors_weighted(u)
                            .eq((0..m).map(|v| (v, &1)))
                    })
                );
            }

            #[test]
            fn biclique_outdegree(m in 1..25_usize, n in 1..25_usize) {
                let digraph = $type::biclique(m, n);

                assert!(
                    (0..m).all(|u| digraph.outdegree(u) == n)
                        && (m..m + n).all(|u| digraph.outdegree(u) == m)
                );
            }

            #[test]
            fn biclique_outdegree_sequence(m in 1..25_usize, n in 1..25_usize) {
                let digraph = $type::biclique(m, n);
                let outdegree_sequence = &mut digraph.outdegree_sequence();

                assert!(outdegree_sequence.take(m).all(|d| d == n));
                assert!(outdegree_sequence.all(|d| d == m));
            }

            #[test]
            fn biclique_semidegree_sequence(m in 1..25_usize, n in 1..25_usize) {
                let digraph = $type::biclique(m, n);
                let semidegree_sequence = &mut digraph.semidegree_sequence();

                assert!(semidegree_sequence.take(m).all(|d| d == (n, n)));
                assert!(semidegree_sequence.all(|d| d == (m, m)));
            }

            #[test]
            fn biclique_sinks(m in 1..25_usize, n in 1..25_usize) {
                assert!($type::biclique(m, n).sinks().eq([]));
            }

            #[test]
            fn biclique_size(m in 1..25_usize, n in 1..25_usize) {
                assert_eq!($type::biclique(m, n).size(), m * n * 2);
            }

            #[test]
            fn biclique_sources(m in 1..25_usize, n in 1..25_usize) {
                assert!($type::biclique(m, n).sources().eq([]));
            }

            #[test]
            fn circuit_complement_size(order in 1..25_usize) {
                assert_eq!(
                    $type::circuit(order).complement().size(),
                    order * order.saturating_sub(2)
                );
            }

            #[test]
            fn circuit_degree(order in 1..25_usize) {
                let digraph = $type::circuit(order);

                assert!(digraph
                    .vertices()
                    .all(|u| digraph.degree(u) == if order == 1 { 0 } else { 2 }));
            }

            #[test]
            fn circuit_degree_sequence(order in 1..25_usize) {
                assert!($type::circuit(order)
                    .degree_sequence()
                    .all(|d| d == if order == 1 { 0 } else { 2 }));
            }

            #[test]
            fn circuit_degree_sum_equals_2size(order in 1..25_usize) {
                let digraph = $type::circuit(order);

                assert_eq!(
                    digraph
                        .vertices()
                        .fold(0, |acc, u| acc + digraph.degree(u)),
                    2 * digraph.size()
                );
            }

            #[test]
            fn circuit_even_number_odd_degrees(order in 1..25_usize) {
                let digraph = $type::circuit(order);

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
            fn circuit_has_edge(order in 1..25_usize) {
                let digraph = $type::circuit(order);

                assert!(digraph.vertices().all(|u| (u + 1..order)
                    .all(|v| (order == 2) == digraph.has_edge(u, v))));
            }

            #[test]
            fn circuit_indegree(order in 1..25_usize) {
                let digraph = $type::circuit(order);

                assert!(digraph
                    .vertices()
                    .all(|u| digraph.indegree(u) == usize::from(order != 1)));
            }

            #[test]
            fn circuit_indegree_sequence(order in 1..25_usize) {
                assert!($type::circuit(order)
                    .indegree_sequence()
                    .all(|d| d == usize::from(order != 1)));
            }

            #[test]
            fn circuit_is_balanced(order in 1..25_usize) {
                assert!($type::circuit(order).is_balanced());
            }

            #[test]
            fn circuit_is_complete(order in 1..25_usize) {
                assert!((order < 3) == $type::circuit(order).is_complete());
            }

            #[test]
            fn circuit_is_isolated(order in 1..25_usize) {
                let digraph = $type::circuit(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_isolated(u)));
            }

            #[test]
            fn circuit_is_oriented(order in 1..25_usize) {
                assert!((order == 2) != $type::circuit(order).is_oriented());
            }

            #[test]
            fn circuit_is_pendant(order in 1..25_usize) {
                let digraph = $type::circuit(order);

                assert!(digraph.vertices().all(|u| !digraph.is_pendant(u)));
            }

            #[test]
            fn circuit_is_regular(order in 1..25_usize) {
                assert!($type::circuit(order).is_regular());
            }

            #[test]
            fn circuit_is_semicomplete(order in 1..25_usize) {
                assert!(
                    (order < 4) == $type::circuit(order).is_semicomplete()
                );
            }

            #[test]
            fn circuit_is_simple(order in 1..25_usize) {
                assert!($type::circuit(order).is_simple());
            }

            #[test]
            fn circuit_is_sink(order in 1..25_usize) {
                let digraph = $type::circuit(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_sink(u)));
            }

            #[test]
            fn circuit_is_source(order in 1..25_usize) {
                let digraph = $type::circuit(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_source(u)));
            }

            #[test]
            fn circuit_is_spanning_subdigraph(order in 1..25_usize) {
                let digraph = $type::circuit(order);

                assert!(digraph.is_spanning_subdigraph(&digraph));
            }

            #[test]
            fn circuit_is_subdigraph(order in 1..25_usize) {
                let digraph = $type::circuit(order);

                assert!(digraph.is_subdigraph(&digraph));
            }

            #[test]
            fn circuit_is_superdigraph(order in 1..25_usize) {
                let digraph = $type::circuit(order);

                assert!(digraph.is_superdigraph(&digraph));
            }

            #[test]
            fn circuit_is_symmetric(order in 1..25_usize) {
                assert!((order < 3) == $type::circuit(order).is_symmetric());
            }

            #[test]
            fn circuit_is_tournament(order in 1..25_usize) {
                assert!(
                    (order == 1 || order == 3)
                        == $type::circuit(order).is_tournament()
                );
            }

            #[test]
            fn circuit_max_degree(order in 1..25_usize) {
                assert_eq!(
                    $type::circuit(order).max_degree(),
                    if order == 1 { 0 } else { 2 }
                );
            }

            #[test]
            fn circuit_max_indegree(order in 1..25_usize) {
                assert_eq!(
                    $type::circuit(order).max_indegree(),
                    if order == 1 { 0 } else { 1 }
                );
            }

            #[test]
            fn circuit_max_outdegree(order in 1..25_usize) {
                assert_eq!(
                    $type::circuit(order).max_outdegree(),
                    if order == 1 { 0 } else { 1 }
                );
            }

            #[test]
            fn circuit_min_degree(order in 1..25_usize) {
                assert_eq!(
                    $type::circuit(order).min_degree(),
                    if order == 1 { 0 } else { 2 }
                );
            }

            #[test]
            fn circuit_min_indegree(order in 1..25_usize) {
                assert_eq!(
                    $type::circuit(order).min_indegree(),
                    if order == 1 { 0 } else { 1 }
                );
            }

            #[test]
            fn circuit_min_outdegree(order in 1..25_usize) {
                assert_eq!(
                    $type::circuit(order).min_outdegree(),
                    if order == 1 { 0 } else { 1 }
                );
            }

            #[test]
            fn circuit_outdegree(order in 1..25_usize) {
                let digraph = $type::circuit(order);

                assert!(digraph
                    .vertices()
                    .all(|u| digraph.outdegree(u) == usize::from(order != 1)));
            }

            #[test]
            fn circuit_outdegree_sequence(order in 1..25_usize) {
                assert!($type::circuit(order)
                    .outdegree_sequence()
                    .all(|d| d == usize::from(order != 1)));
            }

            #[test]
            fn circuit_semidegree_sequence(order in 1..25_usize) {
                assert!($type::circuit(order)
                    .semidegree_sequence()
                    .all(|d| d == if order == 1 { (0, 0) } else { (1, 1) }));
            }

            #[test]
            fn circuit_sinks(order in 1..25_usize) {
                let digraph = $type::circuit(order);
                let sinks = digraph.sinks();

                assert!(if order == 1 { sinks.eq([0]) } else { sinks.eq([]) });
            }

            #[test]
            fn circuit_size(order in 1..25_usize) {
                assert_eq!(
                    $type::circuit(order).size(),
                    if order == 1 { 0 } else { order }
                );
            }

            #[test]
            fn circuit_sources(order in 1..25_usize) {
                let digraph = $type::circuit(order);
                let sources = digraph.sources();

                assert!(
                    if order == 1 { sources.eq([0]) } else { sources.eq([]) }
                );
            }

            #[test]
            fn complete_complement_equals_empty(order in 1..25_usize) {
                assert_eq!(
                    $type::complete(order).complement(),
                    $type::empty(order)
                );
            }

            #[test]
            fn complete_complement_size(order in 1..25_usize) {
                assert_eq!($type::complete(order).complement().size(), 0);
            }

            #[test]
            fn complete_degree(order in 1..25_usize) {
                let digraph = $type::complete(order);
                let degree = order * 2 - 2;

                assert!(digraph
                    .vertices()
                    .all(|u| digraph.degree(u) == degree));
            }

            #[test]
            fn complete_degree_sequence(order in 1..25_usize) {
                let degree = order * 2 - 2;

                assert!($type::complete(order)
                    .degree_sequence()
                    .all(|d| d == degree));
            }

            #[test]
            fn complete_degree_sum_equals_2size(order in 1..25_usize) {
                let digraph = $type::complete(order);

                assert_eq!(
                    digraph
                    .vertices()
                    .fold(0, |acc, u| acc + digraph.degree(u)),
                    2 * digraph.size()
                );
            }

            #[test]
            fn complete_even_number_odd_degrees(order in 1..25_usize) {
                let digraph = $type::complete(order);

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
            fn complete_has_edge(order in 1..25_usize) {
                let digraph = $type::complete(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (u + 1..order).all(|v| digraph.has_edge(u, v))));
            }

            #[test]
            fn complete_indegree(order in 1..25_usize) {
                let digraph = $type::complete(order);
                let indegree = order - 1;

                assert!(digraph
                    .vertices()
                    .all(|v| digraph.indegree(v) == indegree));
            }

            #[test]
            fn complete_indegree_sequence(order in 1..25_usize) {
                let indegree = order - 1;

                assert!($type::complete(order)
                    .indegree_sequence()
                    .all(|d| d == indegree));
            }

            #[test]
            fn complete_is_balanced(order in 1..25_usize) {
                assert!($type::complete(order).is_balanced());
            }

            #[test]
            fn complete_is_complete(order in 1..25_usize) {
                assert!($type::complete(order).is_complete());
            }

            #[test]
            fn complete_is_isolated(order in 1..25_usize) {
                let digraph = $type::complete(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_isolated(u)));
            }

            #[test]
            fn complete_is_oriented(order in 1..25_usize) {
                assert!((order == 1) == $type::complete(order).is_oriented());
            }

            #[test]
            fn complete_is_pendant(order in 1..25_usize) {
                let digraph = $type::complete(order);

                assert!(digraph.vertices().all(|u| !digraph.is_pendant(u)));
            }

            #[test]
            fn complete_is_regular(order in 1..25_usize) {
                assert!($type::complete(order).is_regular());
            }

            #[test]
            fn complete_is_semicomplete(order in 1..25_usize) {
                assert!($type::complete(order).is_semicomplete());
            }

            #[test]
            fn complete_is_simple(order in 1..25_usize) {
                assert!($type::complete(order).is_simple());
            }

            #[test]
            fn complete_is_spanning_subdigraph(order in 1..25_usize) {
                let digraph = $type::complete(order);

                assert!(digraph.is_spanning_subdigraph(&digraph));
            }

            #[test]
            fn complete_is_subdigraph(order in 1..25_usize) {
                let digraph = $type::complete(order);

                assert!(digraph.is_subdigraph(&digraph));
            }

            #[test]
            fn complete_is_superdigraph(order in 1..25_usize) {
                let digraph = $type::complete(order);

                assert!(digraph.is_superdigraph(&digraph));
            }

            #[test]
            fn complete_is_symmetric(order in 1..25_usize) {
                assert!($type::complete(order).is_symmetric());
            }

            #[test]
            fn complete_is_tournament(order in 1..25_usize) {
                assert!(
                    (order == 1) == $type::complete(order).is_tournament()
                );
            }

            #[test]
            fn complete_max_degree(order in 1..25_usize) {
                assert_eq!(
                    $type::complete(order).max_degree(),
                    if order == 1 { 0 } else { (order - 1) * 2 }
                );
            }

            #[test]
            fn complete_max_indegree(order in 1..25_usize) {
                assert_eq!(
                    $type::complete(order).max_indegree(),
                    if order == 1 { 0 } else { order - 1 }
                );
            }

            #[test]
            fn complete_max_outdegree(order in 1..25_usize) {
                assert_eq!(
                    $type::complete(order).max_outdegree(),
                    if order == 1 { 0 } else { order - 1 }
                );
            }

            #[test]
            fn complete_min_degree(order in 1..25_usize) {
                assert_eq!(
                    $type::complete(order).min_degree(),
                    if order == 1 { 0 } else { (order - 1) * 2 }
                );
            }

            #[test]
            fn complete_min_indegree(order in 1..25_usize) {
                assert_eq!(
                    $type::complete(order).min_indegree(),
                    if order == 1 { 0 } else { order - 1 }
                );
            }

            #[test]
            fn complete_min_outdegree(order in 1..25_usize) {
                assert_eq!(
                    $type::complete(order).min_outdegree(),
                    if order == 1 { 0 } else { order - 1 }
                );
            }

            #[test]
            fn complete_order(order in 1..25_usize) {
                assert_eq!($type::complete(order).order(), order);
            }

            #[test]
            fn complete_outdegree(order in 1..25_usize) {
                let digraph = $type::complete(order);
                let outdegree = order - 1;

                assert!(digraph
                    .vertices()
                    .all(|s| digraph.outdegree(s) == outdegree));
            }

            #[test]
            fn complete_outdegree_sequence(order in 1..25_usize) {
                let outdegree = order - 1;

                assert!($type::complete(order)
                    .outdegree_sequence()
                    .all(|d| d == outdegree));
            }

            #[test]
            fn complete_semidegree_sequence(order in 1..25_usize) {
                let degree = order - 1;
                let pair = (degree, degree);

                assert!($type::complete(order)
                    .semidegree_sequence()
                    .all(|d| d == pair));
            }

            #[test]
            fn complete_sinks(order in 1..25_usize) {
                assert!(if order == 1 {
                    $type::complete(order).sinks().eq([0])
                } else {
                    $type::complete(order).sinks().eq([])
                });
            }

            #[test]
            fn complete_size(order in 1..25_usize) {
                assert_eq!($type::complete(order).size(), order * (order - 1));
            }

            #[test]
            fn complete_sources(order in 1..25_usize) {
                assert!(if order == 1 {
                    $type::complete(order).sources().eq([0])
                } else {
                    $type::complete(order).sources().eq([])
                });
            }

            #[test]
            fn cycle_complement_size(order in 1..25_usize) {
                assert_eq!(
                    $type::cycle(order).complement().size(),
                    order * order.saturating_sub(3)
                );
            }

            #[test]
            fn cycle_degree(order in 1..25_usize) {
                let digraph = $type::cycle(order);

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
            fn cycle_degree_sequence(order in 1..25_usize) {
                assert!($type::cycle(order)
                    .degree_sequence()
                    .all(|d| match order {
                        1 => d == 0,
                        2 => d == 2,
                        _ => d == 4,
                    }));
            }

            #[test]
            fn cycle_degree_sum_equals_2size(order in 1..25_usize) {
                let digraph = $type::cycle(order);

                assert_eq!(
                    digraph
                        .vertices()
                        .fold(0, |acc, u| acc + digraph.degree(u)),
                    2 * digraph.size()
                );
            }

            #[test]
            fn cycle_even_number_odd_degrees(order in 1..25_usize) {
                let digraph = $type::cycle(order);

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
            fn cycle_indegree(order in 1..25_usize) {
                let digraph = $type::cycle(order);

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
            fn cycle_indegree_sequence(order in 1..25_usize) {
                assert!($type::cycle(order).indegree_sequence().all(|d| d
                    == match order {
                        1 => 0,
                        2 => 1,
                        _ => 2,
                    }));
            }

            #[test]
            fn cycle_is_balanced(order in 1..25_usize) {
                assert!($type::cycle(order).is_balanced());
            }

            #[test]
            fn cycle_is_complete(order in 1..25_usize) {
                assert!((order < 4) == $type::cycle(order).is_complete());
            }

            #[test]
            fn cycle_is_isolated(order in 1..25_usize) {
                let digraph = $type::cycle(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_isolated(u)));
            }

            #[test]
            fn cycle_is_oriented(order in 1..25_usize) {
                assert!((order == 1) == $type::cycle(order).is_oriented());
            }

            #[test]
            fn cycle_is_pendant(order in 1..25_usize) {
                let digraph = $type::cycle(order);

                assert!(digraph.vertices().all(|u| !digraph.is_pendant(u)));
            }

            #[test]
            fn cycle_is_regular(order in 1..25_usize) {
                assert!($type::cycle(order).is_regular());
            }

            #[test]
            fn cycle_is_semicomplete(order in 1..25_usize) {
                assert!((order < 4) == $type::cycle(order).is_semicomplete());
            }

            #[test]
            fn cycle_is_simple(order in 1..25_usize) {
                assert!($type::cycle(order).is_simple());
            }

            #[test]
            fn cycle_is_sink(order in 1..25_usize) {
                let digraph = $type::cycle(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_sink(u)));
            }

            #[test]
            fn cycle_is_source(order in 1..25_usize) {
                let digraph = $type::cycle(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_source(u)));
            }

            #[test]
            fn cycle_is_spanning_subdigraph(order in 1..25_usize) {
                let digraph = $type::cycle(order);

                assert!(digraph.is_spanning_subdigraph(&digraph));
            }

            #[test]
            fn cycle_is_subdigraph(order in 1..25_usize) {
                let digraph = $type::cycle(order);

                assert!(digraph.is_subdigraph(&digraph));
            }

            #[test]
            fn cycle_is_superdigraph(order in 1..25_usize) {
                let digraph = $type::cycle(order);

                assert!(digraph.is_superdigraph(&digraph));
            }

            #[test]
            fn cycle_is_symmetric(order in 1..25_usize) {
                assert!($type::cycle(order).is_symmetric());
            }

            #[test]
            fn cycle_is_tournament(order in 1..25_usize) {
                assert!((order == 1) == $type::cycle(order).is_tournament());
            }

            #[test]
            fn cycle_max_degree(order in 1..25_usize) {
                assert_eq!(
                    $type::cycle(order).max_degree(),
                    match order {
                        1 => 0,
                        2 => 2,
                        _ => 4,
                    }
                );
            }

            #[test]
            fn cycle_max_indegree(order in 1..25_usize) {
                assert_eq!(
                    $type::cycle(order).max_indegree(),
                    match order {
                        1 => 0,
                        2 => 1,
                        _ => 2,
                    }
                );
            }

            #[test]
            fn cycle_max_outdegree(order in 1..25_usize) {
                assert_eq!(
                    $type::cycle(order).max_outdegree(),
                    match order {
                        1 => 0,
                        2 => 1,
                        _ => 2,
                    }
                );
            }

            #[test]
            fn cycle_min_degree(order in 1..25_usize) {
                assert_eq!(
                    $type::cycle(order).min_degree(),
                    match order {
                        1 => 0,
                        2 => 2,
                        _ => 4,
                    }
                );
            }

            #[test]
            fn cycle_min_indegree(order in 1..25_usize) {
                assert_eq!(
                    $type::cycle(order).min_indegree(),
                    match order {
                        1 => 0,
                        2 => 1,
                        _ => 2,
                    }
                );
            }

            #[test]
            fn cycle_min_outdegree(order in 1..25_usize) {
                assert_eq!(
                    $type::cycle(order).min_outdegree(),
                    match order {
                        1 => 0,
                        2 => 1,
                        _ => 2,
                    }
                );
            }

            #[test]
            fn cycle_outdegree(order in 1..25_usize) {
                let digraph = $type::cycle(order);

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
            fn cycle_outdegree_sequence(order in 1..25_usize) {
                assert!($type::cycle(order).outdegree_sequence().all(|d| d
                    == match order {
                        1 => 0,
                        2 => 1,
                        _ => 2,
                    }));
            }

            #[test]
            fn cycle_semidegree_sequence(order in 1..25_usize) {
                assert!($type::cycle(order).semidegree_sequence().all(|d| d
                    == match order {
                        1 => (0, 0),
                        2 => (1, 1),
                        _ => (2, 2),
                    }));
            }

            #[test]
            fn cycle_sinks(order in 1..25_usize) {
                assert!(if order == 1 {
                    $type::cycle(order).sinks().eq([0])
                } else {
                    $type::cycle(order).sinks().eq([])
                });
            }

            #[test]
            fn cycle_size(order in 1..25_usize) {
                assert_eq!(
                    $type::cycle(order).size(),
                    match order {
                        1 => 0,
                        2 => 2,
                        _ => order * 2
                    }
                );
            }

            #[test]
            fn cycle_sources(order in 1..25_usize) {
                assert!(if order == 1 {
                    $type::cycle(order).sources().eq([0])
                } else {
                    $type::cycle(order).sources().eq([])
                });
            }

            #[test]
            fn empty_arcs(order in 1..25_usize) {
                assert!($type::empty(order).arcs().eq([]));
            }

            #[test]
            fn empty_complement_size(order in 1..25_usize) {
                assert_eq!(
                    $type::empty(order).complement().size(),
                    order * (order - 1)
                );
            }

            #[test]
            fn empty_complement_equals_complete(order in 1..25_usize) {
                assert_eq!(
                    $type::empty(order).complement(),
                    $type::complete(order)
                );
            }

            #[test]
            fn empty_degree(order in 1..25_usize) {
                let digraph = $type::empty(order);

                assert!(digraph.vertices().all(|u| digraph.degree(u) == 0));
            }

            #[test]
            fn empty_degree_sequence(order in 1..25_usize) {
                assert!($type::empty(order).degree_sequence().all(|d| d == 0));
            }

            #[test]
            fn empty_degree_sum_equals_2size(order in 1..25_usize) {
                let digraph = $type::empty(order);

                assert_eq!(
                    digraph
                        .vertices()
                        .fold(0, |acc, u| acc + digraph.degree(u)),
                    2 * digraph.size()
                );
            }

            #[test]
            fn empty_even_number_odd_degrees(order in 1..25_usize) {
                let digraph = $type::empty(order);

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
            fn empty_has_arc(order in 1..25_usize) {
                let digraph = $type::empty(order);

                assert!(digraph.vertices().all(|u| {
                    digraph.vertices().all(|v| !digraph.has_arc(u, v))
                }));
            }

            #[test]
            fn empty_has_edge(order in 1..25_usize) {
                let digraph = $type::empty(order);

                assert!(digraph.vertices().all(|u| {
                    digraph.vertices().all(|v| !digraph.has_edge(u, v))
                }));
            }

            #[test]
            fn empty_indegree(order in 1..25_usize) {
                let digraph = $type::empty(order);

                assert!(digraph.vertices().all(|u| digraph.indegree(u) == 0));
            }

            #[test]
            fn empty_indegree_sequence(order in 1..25_usize) {
                assert!($type::empty(order)
                    .indegree_sequence()
                    .all(|d| d == 0));
            }

            #[test]
            fn empty_is_balanced(order in 1..25_usize) {
                assert!($type::empty(order).is_balanced());
            }

            #[test]
            fn empty_is_complete(order in 1..25_usize) {
                assert!((order == 1) == $type::empty(order).is_complete());
            }

            #[test]
            fn empty_is_isolated(order in 1..25_usize) {
                let digraph = $type::empty(order);

                assert!(digraph.vertices().all(|u| digraph.is_isolated(u)));
            }

            #[test]
            fn empty_is_oriented(order in 1..25_usize) {
                assert!($type::empty(order).is_oriented());
            }

            #[test]
            fn empty_is_pendant(order in 1..25_usize) {
                let digraph = $type::empty(order);

                assert!(digraph.vertices().all(|u| !digraph.is_pendant(u)));
            }

            #[test]
            fn empty_is_regular(order in 1..25_usize) {
                assert!($type::empty(order).is_regular());
            }

            #[test]
            fn empty_is_semicomplete(order in 1..25_usize) {
                assert!((order == 1) == $type::empty(order).is_semicomplete());
            }

            #[test]
            fn empty_is_simple(order in 1..25_usize) {
                assert!($type::empty(order).is_simple());
            }

            #[test]
            fn empty_is_sink(order in 1..25_usize) {
                let digraph = $type::empty(order);

                assert!(digraph.vertices().all(|u| digraph.is_sink(u)));
            }

            #[test]
            fn empty_is_source(order in 1..25_usize) {
                let digraph = $type::empty(order);

                assert!(digraph.vertices().all(|u| digraph.is_source(u)));
            }

            #[test]
            fn empty_is_spanning_subdigraph(order in 1..25_usize) {
                let digraph = $type::empty(order);

                assert!(digraph.is_spanning_subdigraph(&digraph));
            }

            #[test]
            fn empty_is_subdigraph(order in 1..25_usize) {
                let digraph = $type::empty(order);

                assert!(digraph.is_subdigraph(&digraph));
            }

            #[test]
            fn empty_is_superdigraph(order in 1..25_usize) {
                let digraph = $type::empty(order);

                assert!(digraph.is_superdigraph(&digraph));
            }

            #[test]
            fn empty_is_symmetric(order in 1..25_usize) {
                assert!($type::empty(order).is_symmetric());
            }

            #[test]
            fn empty_is_tournament(order in 1..25_usize) {
                assert!((order == 1) == $type::empty(order).is_tournament());
            }

            #[test]
            fn empty_max_degree(order in 1..25_usize) {
                assert_eq!($type::empty(order).max_degree(), 0);
            }

            #[test]
            fn empty_max_indegree(order in 1..25_usize) {
                assert_eq!($type::empty(order).max_indegree(), 0);
            }

            #[test]
            fn empty_max_outdegree(order in 1..25_usize) {
                assert_eq!($type::empty(order).max_outdegree(), 0);
            }

            #[test]
            fn empty_min_degree(order in 1..25_usize) {
                assert_eq!($type::empty(order).min_degree(), 0);
            }

            #[test]
            fn empty_min_indegree(order in 1..25_usize) {
                assert_eq!($type::empty(order).min_indegree(), 0);
            }

            #[test]
            fn empty_min_outdegree(order in 1..25_usize) {
                assert_eq!($type::empty(order).min_outdegree(), 0);
            }

            #[test]
            fn empty_outdegree(order in 1..25_usize) {
                let digraph = $type::empty(order);

                assert!(digraph.vertices().all(|u| digraph.outdegree(u) == 0));
            }

            #[test]
            fn empty_outdegree_sequence(order in 1..25_usize) {
                assert!($type::empty(order)
                    .outdegree_sequence()
                    .all(|d| d == 0));
            }

            #[test]
            fn empty_semidegree_sequence(order in 1..25_usize) {
                assert!($type::empty(order)
                    .semidegree_sequence()
                    .all(|d| d == (0, 0)));
            }

            #[test]
            fn empty_sinks(order in 1..25_usize) {
                assert!($type::empty(order).sinks().eq(0..order));
            }

            #[test]
            fn empty_size(order in 1..25_usize) {
                assert_eq!($type::empty(order).size(), 0);
            }

            #[test]
            fn empty_sources(order in 1..25_usize) {
                assert!($type::empty(order).sources().eq(0..order));
            }

            #[test]
            fn erdos_renyi_degree(
                order in 1..25_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                let digraph = $type::erdos_renyi(order, p, seed);
                let max_degree = (order - 1) * 2;

                assert!(digraph.vertices().all(|u| {
                    (0..=max_degree).contains(&digraph.degree(u))
                }));
            }

            #[test]
            fn erdos_renyi_degree_sum_equals_2size(
                order in 1..25_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                let digraph = $type::erdos_renyi(order, p, seed);

                assert_eq!(
                    digraph
                        .vertices()
                        .fold(0, |acc, u| acc + digraph.degree(u)),
                    2 * digraph.size()
                );
            }

            #[test]
            fn erdos_renyi_even_number_odd_degrees(
                order in 1..25_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                let digraph = $type::erdos_renyi(order, p, seed);

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
            fn erdos_renyi_has_arc(
                order in 1..25_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                let digraph = $type::erdos_renyi(order, p, seed);

                assert!(digraph.vertices().all(|u| !digraph.has_arc(u, u) ));
            }

            #[test]
            fn erdos_renyi_indegree(
                order in 1..25_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                let digraph = $type::erdos_renyi(order, p, seed);

                assert!(digraph.vertices().all(|v| {
                    (0..order).contains(&digraph.indegree(v))
                }));
            }

            #[allow(clippy::float_cmp)]
            #[test]
            fn erdos_renyi_is_complete(
                order in 1..25_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                if p == 0.0 {
                    assert!(!$type::erdos_renyi(order, p, seed).is_complete());
                } else if order == 1 {
                    assert!($type::erdos_renyi(order, p, seed).is_complete());
                }
            }

            #[test]
            fn erdos_renyi_is_simple(
                order in 1..25_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                assert!($type::erdos_renyi(order, p, seed).is_simple());
            }

            #[test]
            fn erdos_renyi_is_subdigraph(
                order in 1..25_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                let digraph = $type::erdos_renyi(order, p, seed);

                assert!(digraph.is_subdigraph(&digraph));
            }

            #[test]
            fn erdos_renyi_is_superdigraph(
                order in 1..25_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                let digraph = $type::erdos_renyi(order, p, seed);

                assert!(digraph.is_superdigraph(&digraph));
            }

            #[test]
            fn erdos_renyi_order(
                order in 1..25_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                assert_eq!($type::erdos_renyi(order, p, seed).order(), order);
            }

            #[test]
            fn erdos_renyi_outdegree(
                order in 1..25_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                let digraph = $type::erdos_renyi(order, p, seed);

                assert!(digraph.vertices().all(|u| {
                    (0..order).contains(&digraph.outdegree(u))
                }));
            }

            #[test]
            fn erdos_renyi_size_p_0(order in 1..25_usize, seed: u64) {
                assert_eq!($type::erdos_renyi(order, 0.0, seed).size(), 0);
            }

            #[test]
            fn erdos_renyi_size_p_1(order in 1..25_usize, seed: u64) {
                assert_eq!(
                    $type::erdos_renyi(order, 1.0, seed).size(),
                    order * (order - 1)
                );
            }

            #[test]
            fn growing_network_degree(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::growing_network(order, seed);

                assert!(digraph.vertices().all(|u| {
                    digraph.degree(u) <= order - u
                }));
            }

            #[test]
            fn growing_network_degree_sequence(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::growing_network(order, seed);
                let degree_sequence = &mut digraph.degree_sequence();

                assert!(degree_sequence.all(|d| d < order));
            }

            #[test]
            fn growing_network_degree_sum_equals_2size(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::growing_network(order, seed);

                assert_eq!(
                    digraph
                        .vertices()
                        .fold(0, |acc, u| acc + digraph.degree(u)),
                    2 * digraph.size()
                );
            }

            #[test]
            fn growing_network_even_number_odd_degrees(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::growing_network(order, seed);

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
            fn growing_network_has_edge(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::growing_network(order, seed);

                assert!(digraph.vertices().all(|u| {
                    digraph.vertices().all(|v| !digraph.has_edge(u, v))
                }));
            }

            #[test]
            fn growing_network_indegree(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::growing_network(order, seed);

                assert!(digraph.vertices().all(|v| {
                    digraph.indegree(v) <= order - v
                }));
            }

            #[test]
            fn growing_network_indegree_sequence(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::growing_network(order, seed);
                let indegree_sequence = &mut digraph.indegree_sequence();

                assert!(indegree_sequence.all(|d| d < order));
            }

            #[test]
            fn growing_network_is_complete(order in 1..25_usize, seed: u64) {
                assert!((order == 1) == $type::growing_network(order, seed).is_complete());
            }

            #[test]
            fn growing_network_is_simple(order in 1..25_usize, seed: u64) {
                assert!($type::growing_network(order, seed).is_simple());
            }

            #[test]
            fn growing_network_is_subdigraph(order in 1..25_usize, seed: u64) {
                let digraph = $type::growing_network(order, seed);

                assert!(digraph.is_subdigraph(&digraph));
            }

            #[test]
            fn growing_network_is_superdigraph(order in 1..25_usize, seed: u64) {
                let digraph = $type::growing_network(order, seed);

                assert!(digraph.is_superdigraph(&digraph));
            }

            #[test]
            fn growing_network_order(order in 1..25_usize, seed: u64) {
                assert_eq!($type::growing_network(order, seed).order(), order);
            }

            #[test]
            fn growing_network_outdegree(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::growing_network(order, seed);

                assert!(digraph.vertices().all(|u| {
                    digraph.outdegree(u) <= order - u
                }));
            }

            #[test]
            fn growing_network_outdegree_sequence(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::growing_network(order, seed);
                let outdegree_sequence = &mut digraph.outdegree_sequence();

                assert!(outdegree_sequence.all(|d| d < order));
            }

            #[test]
            fn growing_network_size(order in 1..25_usize, seed: u64) {
                assert_eq!($type::growing_network(order, seed).size(), order - 1);
            }

            #[test]
            fn has_arc_out_of_bounds(order in 1..25_usize) {
                let digraph = $type::empty(order);

                assert!((0..order)
                    .all(|u| !digraph.has_arc(u, order)
                        && !digraph.has_arc(order, u)));
            }

            #[test]
            fn path_complement_size(order in 1..25_usize) {
                assert_eq!(
                    $type::path(order).complement().size(),
                    (order - 1).pow(2)
                );
            }

            #[test]
            fn path_degree(order in 1..25_usize) {
                let digraph = $type::path(order);
                let last = order - 1;

                assert!(
                    (order == 1 && digraph.degree(0) == 0)
                        || digraph.degree(0) == 1
                            && (1..last).all(|u| digraph.degree(u) == 2
                                && digraph.degree(last) == 1)
                );
            }

            #[test]
            fn path_degree_sequence(order in 1..25_usize) {
                let digraph = $type::path(order);
                let degree_sequence = &mut digraph.degree_sequence();

                if order == 1 {
                    assert_eq!(degree_sequence.next(), Some(0));
                } else {
                    assert_eq!(degree_sequence.next(), Some(1));
                    assert!(degree_sequence.take(order - 2).all(|d| d == 2));
                    assert_eq!(degree_sequence.next(), Some(1));
                }
            }

            #[test]
            fn path_degree_sum_equals_2size(order in 1..25_usize) {
                let digraph = $type::path(order);

                assert_eq!(
                    digraph
                        .vertices()
                        .fold(0, |acc, u| acc + digraph.degree(u)),
                    2 * digraph.size()
                );
            }

            #[test]
            fn path_even_number_odd_degrees(order in 1..25_usize) {
                let digraph = $type::path(order);

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
            fn path_has_edge(order in 1..25_usize) {
                let digraph = $type::path(order);

                assert!((0..order)
                    .all(|u| (0..order).all(|v| !digraph.has_edge(u, v))));
            }

            #[test]
            fn path_indegree(order in 1..25_usize) {
                let digraph = $type::path(order);

                assert_eq!(digraph.indegree(0), 0);
                assert!((1..order).all(|u| digraph.indegree(u) == 1));
            }

            #[test]
            fn path_indegree_sequence(order in 1..25_usize) {
                let digraph = $type::path(order);
                let indegree_sequence = &mut digraph.indegree_sequence();

                assert_eq!(indegree_sequence.next(), Some(0));
                assert!(indegree_sequence.all(|d| d == 1));
            }

            #[test]
            fn path_is_balanced(order in 1..25_usize) {
                assert!((order == 1) == $type::path(order).is_balanced());
            }

            #[test]
            fn path_is_complete(order in 1..25_usize) {
                assert!((order == 1) == $type::path(order).is_complete());
            }

            #[test]
            fn path_is_isolated(order in 1..25_usize) {
                let digraph = $type::path(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_isolated(u)));
            }

            #[test]
            fn path_is_oriented(order in 1..25_usize) {
                assert!($type::path(order).is_oriented());
            }

            #[test]
            fn path_is_pendant(order in 1..25_usize) {
                let digraph = $type::path(order);
                let last = order - 1;

                assert!(
                    (order == 1 && !digraph.is_pendant(0))
                        || (digraph.is_pendant(0)
                            && (1..last).all(|u| !digraph.is_pendant(u))
                            && digraph.is_pendant(last))
                );
            }

            #[test]
            fn path_is_regular(order in 1..25_usize) {
                assert!((order == 1) == $type::path(order).is_regular());
            }

            #[test]
            fn path_is_semicomplete(order in 1..25_usize) {
                assert!((order < 3) == $type::path(order).is_semicomplete());
            }

            #[test]
            fn path_is_simple(order in 1..25_usize) {
                assert!($type::path(order).is_simple());
            }

            #[test]
            fn path_is_sink(order in 1..25_usize) {
                let digraph = $type::path(order);
                let last = order - 1;

                assert!(
                    (order == 1 && digraph.is_sink(0))
                        || ((0..last).all(|u| !digraph.is_sink(u))
                            && digraph.is_sink(last))
                );
            }

            #[test]
            fn path_is_source(order in 1..25_usize) {
                let digraph = $type::path(order);

                assert!(digraph.is_source(0));
                assert!((1..order).all(|u| !digraph.is_source(u)));
            }

            #[test]
            fn path_is_spanning_subdigraph(order in 1..25_usize) {
                let digraph = $type::path(order);

                assert!(digraph.is_spanning_subdigraph(&digraph));
            }

            #[test]
            fn path_is_subdigraph(order in 1..25_usize) {
                let digraph = $type::path(order);

                assert!(digraph.is_subdigraph(&digraph));
            }

            #[test]
            fn path_is_superdigraph(order in 1..25_usize) {
                let digraph = $type::path(order);

                assert!(digraph.is_superdigraph(&digraph));
            }

            #[test]
            fn path_is_symmetric(order in 1..25_usize) {
                assert!((order == 1) == $type::path(order).is_symmetric());
            }

            #[test]
            fn path_is_tournament(order in 1..25_usize) {
                assert!((order < 3) == $type::path(order).is_tournament());
            }

            #[test]
            fn path_max_degree(order in 1..25_usize) {
                assert_eq!(
                    $type::path(order).max_degree(),
                    match order {
                        1 => 0,
                        2 => 1,
                        _ => 2,
                    }
                );
            }

            #[test]
            fn path_max_indegree(order in 1..25_usize) {
                assert_eq!(
                    $type::path(order).max_indegree(),
                    if order == 1 { 0 } else { 1 }
                );
            }

            #[test]
            fn path_max_outdegree(order in 1..25_usize) {
                assert_eq!(
                    $type::path(order).max_outdegree(),
                    if order == 1 { 0 } else { 1 }
                );
            }

            #[test]
            fn path_min_degree(order in 1..25_usize) {
                assert_eq!(
                    $type::path(order).min_degree(),
                    if order == 1 { 0 } else { 1 }
                );
            }

            #[test]
            fn path_min_indegree(order in 1..25_usize) {
                assert_eq!($type::path(order).min_indegree(), 0);
            }

            #[test]
            fn path_min_outdegree(order in 1..25_usize) {
                assert_eq!($type::path(order).min_outdegree(), 0);
            }

            #[test]
            fn path_outdegree(order in 1..25_usize) {
                let digraph = $type::path(order);
                let last = order - 1;

                assert!((0..last).all(|u| digraph.outdegree(u) == 1));
                assert_eq!(digraph.outdegree(last), 0);
            }

            #[test]
            fn path_outdegree_sequence(order in 1..25_usize) {
                let digraph = $type::path(order);
                let outdegree_sequence = &mut digraph.outdegree_sequence();

                assert!(outdegree_sequence.take(order - 1).all(|d| d == 1));
                assert_eq!(outdegree_sequence.next(), Some(0));
            }

            #[test]
            fn path_semidegree_sequence(order in 1..25_usize) {
                let digraph = $type::path(order);
                let semidegree_sequence = &mut digraph.semidegree_sequence();

                assert!(if order == 1 {
                    semidegree_sequence.next() == Some((0, 0))
                } else {
                    semidegree_sequence.next() == Some((0, 1))
                        && semidegree_sequence
                            .take(order - 2)
                            .all(|d| d == (1, 1))
                        && semidegree_sequence.next() == Some((1, 0))
                });
            }

            #[test]
            fn path_sinks(order in 1..25_usize) {
                assert!($type::path(order).sinks().eq([order - 1]));
            }

            #[test]
            fn path_size(order in 1..25_usize) {
                assert_eq!($type::path(order).size(), order - 1);
            }

            #[test]
            fn path_sources(order in 1..25_usize) {
                assert!($type::path(order).sources().eq([0]));
            }

            #[test]
            fn random_tournament_complement_size(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                assert_eq!(
                    $type::random_tournament(order, seed).complement().size(),
                    order * (order - 1) / 2
                );
            }

            #[test]
            fn random_tournament_degree(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::random_tournament(order, seed);
                let degree = order - 1;

                assert!(digraph
                    .vertices()
                    .all(|u| digraph.degree(u) == degree));
            }

            #[test]
            fn random_tournament_degree_sequence(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let degree = order - 1;

                assert!($type::random_tournament(order, seed)
                    .degree_sequence()
                    .all(|d| d == degree));
            }

            #[test]
            fn random_tournament_degree_sum_equals_2size(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::random_tournament(order, seed);

                assert_eq!(
                    digraph
                        .vertices()
                        .fold(0, |acc, u| acc + digraph.degree(u)),
                    2 * digraph.size()
                );
            }

            #[test]
            fn random_tournament_even_number_odd_degrees(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::random_tournament(order, seed);

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
            fn random_tournament_has_arc(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::random_tournament(order, seed);

                assert!(digraph.vertices().all(|u| {
                    digraph.vertices().all(|v| {
                        u == v || digraph.has_arc(u, v) ^ digraph.has_arc(v, u)
                    })
                }));
            }

            #[test]
            fn random_tournament_has_edge(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::random_tournament(order, seed);

                assert!(digraph.vertices().all(|u| {
                    digraph.vertices().all(|v| !digraph.has_edge(u, v))
                }));
            }

            #[test]
            fn random_tournament_indegree(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::random_tournament(order, seed);

                assert!(digraph
                    .vertices()
                    .all(|u| (0..order).contains(&digraph.indegree(u))));
            }

            #[test]
            fn random_tournament_indegree_sequence(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::random_tournament(order, seed);
                let indegree_sequence = &mut digraph.indegree_sequence();

                assert!(indegree_sequence.all(|d| (0..order).contains(&d)));
            }

            #[test]
            fn random_tournament_is_complete(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                assert!(
                    (order == 1)
                        == $type::random_tournament(order, seed).is_complete()
                );
            }

            #[test]
            fn random_tournament_is_isolated(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::random_tournament(order, seed);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_isolated(u)));
            }

            #[test]
            fn random_tournament_is_oriented(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                assert!($type::random_tournament(order, seed).is_oriented());
            }

            #[test]
            fn random_tournament_is_pendant(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::random_tournament(order, seed);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 2) == digraph.is_pendant(u)));
            }

            #[test]
            fn random_tournament_is_semicomplete(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                assert!(
                    $type::random_tournament(order, seed).is_semicomplete()
                );
            }

            #[test]
            fn random_tournament_is_simple(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                assert!($type::random_tournament(order, seed).is_simple());
            }

            #[test]
            fn random_tournament_is_spanning_subdigraph(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::random_tournament(order, seed);

                assert!(digraph.is_spanning_subdigraph(&digraph));
            }

            #[test]
            fn random_tournament_is_subdigraph(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::random_tournament(order, seed);

                assert!(digraph.is_subdigraph(&digraph));
            }

            #[test]
            fn random_tournament_is_superdigraph(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::random_tournament(order, seed);

                assert!(digraph.is_superdigraph(&digraph));
            }

            #[test]
            fn random_tournament_is_symmetric(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                assert!(
                    (order == 1)
                        == $type::random_tournament(order, seed).is_symmetric()
                );
            }

            #[test]
            fn random_tournament_is_tournament(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                assert!($type::random_tournament(order, seed).is_tournament());
            }

            #[test]
            fn random_tournament_max_degree(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                assert_eq!(
                    $type::random_tournament(order, seed).max_degree(),
                    if order == 1 { 0 } else { order - 1 }
                );
            }

            #[test]
            fn random_tournament_min_degree(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                assert_eq!(
                    $type::random_tournament(order, seed).min_degree(),
                    if order == 1 { 0 } else { order - 1 }
                );
            }

            #[test]
            fn random_tournament_order(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                assert_eq!(
                    $type::random_tournament(order, seed).order(), order
                );
            }

            #[test]
            fn random_tournament_outdegree(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::random_tournament(order, seed);

                assert!(digraph
                    .vertices()
                    .all(|u| (0..order).contains(&digraph.outdegree(u))));
            }

            #[test]
            fn random_tournament_outdegree_sequence(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::random_tournament(order, seed);
                let outdegree_sequence = &mut digraph.outdegree_sequence();

                assert!(outdegree_sequence.all(|d| (0..order).contains(&d)));
            }

            #[test]
            fn random_tournament_semidegree_sequence(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                assert_eq!(
                    $type::random_tournament(order, seed)
                        .semidegree_sequence()
                        .fold(0, |acc, (indegree, outdegree)| acc
                            + indegree
                            + outdegree),
                    order * (order - 1)
                );
            }

            #[test]
            fn random_tournament_size(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                assert_eq!(
                    $type::random_tournament(order, seed).size(),
                    order * (order - 1) / 2
                );
            }

            #[test]
            fn star_complement_size(order in 1..25_usize) {
                assert_eq!(
                    $type::star(order).complement().size(),
                    (order - 1) * order.saturating_sub(2)
                );
            }

            #[test]
            fn star_degree(order in 1..25_usize) {
                let digraph = $type::star(order);

                assert_eq!(digraph.degree(0), (order - 1) * 2);
                assert!((1..order).all(|u| digraph.degree(u) == 2));
            }

            #[test]
            fn star_degree_sequence(order in 1..25_usize) {
                let digraph = $type::star(order);
                let degree_sequence = &mut digraph.degree_sequence();

                assert_eq!(degree_sequence.next(), Some((order - 1) * 2));
                assert!(degree_sequence.all(|d| d == 2));
            }

            #[test]
            fn star_degree_sum_equals_2size(order in 1..25_usize) {
                let digraph = $type::star(order);

                assert_eq!(
                    digraph
                        .vertices()
                        .fold(0, |acc, u| acc + digraph.degree(u)),
                    2 * digraph.size()
                );
            }

            #[test]
            fn star_even_number_odd_degrees(order in 1..25_usize) {
                let digraph = $type::star(order);

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
            fn star_has_edge(order in 1..25_usize) {
                let digraph = $type::star(order);

                assert!((1..order).all(|u| digraph.has_edge(0, u)
                    && (u..order).all(|v| !digraph.has_edge(u, v))));
            }

            #[test]
            fn star_indegree(order in 1..25_usize) {
                let digraph = $type::star(order);

                assert_eq!(digraph.indegree(0), order - 1);
                assert!((1..order).all(|u| digraph.indegree(u) == 1));
            }

            #[test]
            fn star_indegree_sequence(order in 1..25_usize) {
                let digraph = $type::star(order);
                let indegree_sequence = &mut digraph.indegree_sequence();

                assert_eq!(indegree_sequence.next(), Some(order - 1));
                assert!(indegree_sequence.all(|d| d == 1));
            }

            #[test]
            fn star_is_balanced(order in 1..25_usize) {
                assert!($type::star(order).is_balanced());
            }

            #[test]
            fn star_is_complete(order in 1..25_usize) {
                assert!((order < 3) == $type::star(order).is_complete());
            }

            #[test]
            fn star_is_isolated(order in 1..25_usize) {
                let digraph = $type::star(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_isolated(u)));
            }

            #[test]
            fn star_is_oriented(order in 1..25_usize) {
                assert!((order == 1) == $type::star(order).is_oriented());
            }

            #[test]
            fn star_is_pendant(order in 1..25_usize) {
                let digraph = $type::star(order);

                assert!(digraph.vertices().all(|u| !digraph.is_pendant(u)));
            }

            #[test]
            fn star_is_regular(order in 1..25_usize) {
                assert!((order < 3) == $type::star(order).is_regular());
            }

            #[test]
            fn star_is_semicomplete(order in 1..25_usize) {
                assert!((order < 3) == $type::star(order).is_semicomplete());
            }

            #[test]
            fn star_is_simple(order in 1..25_usize) {
                assert!($type::star(order).is_simple());
            }

            #[test]
            fn star_is_sink(order in 1..25_usize) {
                let digraph = $type::star(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_sink(u)));
            }

            #[test]
            fn star_is_source(order in 1..25_usize) {
                let digraph = $type::star(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_source(u)));
            }

            #[test]
            fn star_is_spanning_subdigraph(order in 1..25_usize) {
                let digraph = $type::star(order);

                assert!(digraph.is_spanning_subdigraph(&digraph));
            }

            #[test]
            fn star_is_subdigraph(order in 1..25_usize) {
                let digraph = $type::star(order);

                assert!(digraph.is_subdigraph(&digraph));
            }

            #[test]
            fn star_is_superdigraph(order in 1..25_usize) {
                let digraph = $type::star(order);

                assert!(digraph.is_superdigraph(&digraph));
            }

            #[test]
            fn star_is_symmetric(order in 1..25_usize) {
                assert!($type::star(order).is_symmetric());
            }

            #[test]
            fn star_is_tournament(order in 1..25_usize) {
                assert!((order == 1) == $type::star(order).is_tournament());
            }

            #[test]
            fn star_max_degree(order in 1..25_usize) {
                assert_eq!(
                    $type::star(order).max_degree(),
                    (order - 1) * 2
                );
            }

            #[test]
            fn star_max_indegree(order in 1..25_usize) {
                assert_eq!($type::star(order).max_indegree(), order - 1);
            }

            #[test]
            fn star_max_outdegree(order in 1..25_usize) {
                assert_eq!($type::star(order).max_outdegree(), order - 1);
            }

            #[test]
            fn star_min_degree(order in 1..25_usize) {
                assert_eq!(
                    $type::star(order).min_degree(),
                    if order == 1 { 0 } else { 2 }
                );
            }

            #[test]
            fn star_min_indegree(order in 1..25_usize) {
                assert_eq!(
                    $type::star(order).min_indegree(),
                    if order == 1 { 0 } else { 1 }
                );
            }

            #[test]
            fn star_min_outdegree(order in 1..25_usize) {
                assert_eq!(
                    $type::star(order).min_outdegree(),
                    if order == 1 { 0 } else { 1 }
                );
            }

            #[test]
            fn star_outdegree(order in 1..25_usize) {
                let digraph = $type::star(order);

                assert_eq!(digraph.outdegree(0), order - 1);
                assert!((1..order).all(|u| digraph.outdegree(u) == 1));
            }

            #[test]
            fn star_outdegree_sequence(order in 1..25_usize) {
                let digraph = $type::star(order);
                let outdegree_sequence = &mut digraph.outdegree_sequence();

                assert_eq!(outdegree_sequence.next(), Some(order - 1));
                assert!(outdegree_sequence.all(|d| d == 1));
            }

            #[test]
            fn star_semidegree_sequence(order in 1..25_usize) {
                let digraph = $type::star(order);
                let mut semidegree_sequence = digraph.semidegree_sequence();

                assert_eq!(
                    semidegree_sequence.next(),
                    Some((order - 1, order - 1))
                );

                assert!(semidegree_sequence.all(|d| d == (1, 1)));
            }

            #[test]
            fn star_sinks(order in 1..25_usize) {
                assert!(if order == 1 {
                    $type::star(order).sinks().eq([0])
                } else {
                    $type::star(order).sinks().eq([])
                });
            }

            #[test]
            fn star_size(order in 1..25_usize) {
                assert_eq!($type::star(order).size(), (order - 1) * 2);
            }

            #[test]
            fn star_sources(order in 1..25_usize) {
                assert!(if order == 1 {
                    $type::star(order).sources().eq([0])
                } else {
                    $type::star(order).sources().eq([])
                });
            }

            #[test]
            fn union_biclique_commutative(
                m in 1..25_usize,
                n in 1..25_usize
            ) {
                let biclique = $type::biclique(m, n);
                let other = $type::biclique(n, m);

                assert_eq!(biclique.union(&other), other.union(&biclique));
            }

            #[test]
            fn union_biclique_complement_is_complete(
                m in 1..25_usize,
                n in 1..25_usize
            ) {
                let biclique = $type::biclique(m, n);
                let complement = biclique.complement();

                assert!(biclique.union(&complement).is_complete());
                assert!(complement.union(&biclique).is_complete());
            }

            #[test]
            fn union_circuit_complement_is_complete(order in 3..25_usize) {
                let circuit = $type::circuit(order);
                let complement = circuit.complement();

                assert!(circuit.union(&complement).is_complete());
                assert!(complement.union(&circuit).is_complete());
            }

            #[test]
            fn union_complete_commutative(order in 1..25_usize) {
                let complete = $type::complete(order);
                let other = $type::complete(order);

                assert_eq!(complete.union(&other), other.union(&complete));
            }

            #[test]
            fn union_complete_complement_is_complete(order in 1..25_usize) {
                let complete = $type::complete(order);
                let complement = complete.complement();

                assert!(complete.union(&complement).is_complete());
                assert!(complement.union(&complete).is_complete());
            }

            #[test]
            fn union_circuit_cycle(order in 1..25_usize) {
                let circuit = $type::circuit(order);
                let converse = circuit.converse();
                let cycle = $type::cycle(order);

                assert!(circuit.union(&converse).arcs().eq(cycle.arcs()));
                assert!(converse.union(&circuit).arcs().eq(cycle.arcs()));
            }

            #[test]
            fn union_circuit_path(order in 1..25_usize) {
                let circuit = $type::circuit(order);
                let path = $type::path(order);
                let mut bridge = $type::empty(order);

                if order > 1 {
                    bridge.add_arc(order - 1, 0);
                }

                assert!(bridge.union(&path).arcs().eq(circuit.arcs()));
                assert!(path.union(&bridge).arcs().eq(circuit.arcs()));
            }

            #[test]
            fn union_cycle_commutative(order in 1..25_usize) {
                let cycle = $type::cycle(order);
                let other = $type::cycle(order);

                assert_eq!(cycle.union(&other), other.union(&cycle));
            }

            #[test]
            fn union_cycle_complement_is_complete(order in 1..25_usize) {
                let cycle = $type::cycle(order);
                let complement = cycle.complement();

                assert!(cycle.union(&complement).is_complete());
                assert!(complement.union(&cycle).is_complete());
            }

            #[test]
            fn union_empty_commutative(order in 1..25_usize) {
                let empty = $type::empty(order);
                let other = $type::empty(order);

                assert_eq!(empty.union(&other), other.union(&empty));
            }

            #[test]
            fn union_empty_complement_is_complete(order in 1..25_usize) {
                let empty = $type::empty(order);
                let complement = empty.complement();

                assert!(empty.union(&complement).is_complete());
                assert!(complement.union(&empty).is_complete());
            }

            #[test]
            fn union_empty_id_biclique(m in 1..25_usize, n in 1..25_usize) {
                let digraph = $type::biclique(m, n);
                let empty = $type::empty(m + n);

                assert!(empty.union(&digraph).arcs().eq(digraph.arcs()));
                assert!(digraph.union(&empty).arcs().eq(digraph.arcs()));
            }

            #[test]
            fn union_empty_id_circuit(order in 3..25_usize) {
                let digraph = $type::circuit(order);
                let empty = $type::empty(order);

                assert!(empty.union(&digraph).arcs().eq(digraph.arcs()));
                assert!(digraph.union(&empty).arcs().eq(digraph.arcs()));
            }

            #[test]
            fn union_empty_id_cycle(order in 1..25_usize) {
                let digraph = $type::cycle(order);
                let empty = $type::empty(order);

                assert!(empty.union(&digraph).arcs().eq(digraph.arcs()));
                assert!(digraph.union(&empty).arcs().eq(digraph.arcs()));
            }

            #[test]
            fn union_empty_id_empty(order in 1..25_usize) {
                let digraph = $type::empty(order);
                let empty = $type::empty(order);

                assert!(empty.union(&digraph).arcs().eq(digraph.arcs()));
                assert!(digraph.union(&empty).arcs().eq(digraph.arcs()));
            }

            #[test]
            fn union_empty_id_erdos_renyi(
                order in 1..25_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                let digraph = $type::erdos_renyi(order, p, seed);
                let empty = $type::empty(order);

                assert!(empty.union(&digraph).arcs().eq(digraph.arcs()));
                assert!(digraph.union(&empty).arcs().eq(digraph.arcs()));
            }

            #[test]
            fn union_empty_id_path(order in 1..25_usize) {
                let digraph = $type::path(order);
                let empty = $type::empty(order);

                assert!(empty.union(&digraph).arcs().eq(digraph.arcs()));
                assert!(digraph.union(&empty).arcs().eq(digraph.arcs()));
            }

            #[test]
            fn union_empty_id_random_tournament(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::random_tournament(order, seed);
                let empty = $type::empty(order);

                assert!(empty.union(&digraph).arcs().eq(digraph.arcs()));
                assert!(digraph.union(&empty).arcs().eq(digraph.arcs()));
            }

            #[test]
            fn union_empty_id_star(order in 1..25_usize) {
                let digraph = $type::star(order);
                let empty = $type::empty(order);

                assert!(empty.union(&digraph).arcs().eq(digraph.arcs()));
                assert!(digraph.union(&empty).arcs().eq(digraph.arcs()));
            }

            #[test]
            fn union_empty_id_wheel(order in 4..25_usize) {
                let digraph = $type::wheel(order);
                let empty = $type::empty(order);

                assert!(empty.union(&digraph).arcs().eq(digraph.arcs()));
                assert!(digraph.union(&empty).arcs().eq(digraph.arcs()));
            }

            #[test]
            fn union_erdos_renyi_commutative(
                order_1 in 1..25_usize,
                order_2 in 1..25_usize,
                p in 0.0..1.0,
                seed_1 in 0..1000_u64,
                seed_2 in 0..1000_u64
            ) {
                let digraph = $type::erdos_renyi(order_1, p, seed_1);
                let other = $type::erdos_renyi(order_2, p, seed_2);

                assert_eq!(digraph.union(&other), other.union(&digraph));
            }

            #[test]
            fn union_erdos_renyi_complement_is_complete(
                order in 1..25_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                let digraph = $type::erdos_renyi(order, p, seed);
                let complement = digraph.complement();

                assert!(digraph.union(&complement).is_complete());
                assert!(complement.union(&digraph).is_complete());
            }

            #[test]
            fn union_erdos_renyi_size_gte(
                order_1 in 1..25_usize,
                order_2 in 1..25_usize,
                p in 0.0..1.0,
                seed_1 in 0..1000_u64,
                seed_2 in 0..1000_u64
            ) {
                let digraph = $type::erdos_renyi(order_1, p, seed_1);
                let other = $type::erdos_renyi(order_2, p, seed_2);

                assert!(digraph.size() + other.size() >= digraph.union(&other)
                    .size());
            }

            #[test]
            fn union_growing_network_commutative(
                order_1 in 1..25_usize,
                order_2 in 1..25_usize,
                seed_1 in 0..1000_u64,
                seed_2 in 0..1000_u64
            ) {
                let digraph = $type::growing_network(order_1, seed_1);
                let other = $type::growing_network(order_2, seed_2);

                assert_eq!(digraph.union(&other), other.union(&digraph));
            }

            #[test]
            fn union_growing_network_complement_is_complete(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::growing_network(order, seed);
                let complement = digraph.complement();

                assert!(digraph.union(&complement).is_complete());
                assert!(complement.union(&digraph).is_complete());
            }

            #[test]
            fn union_path_commutative(
                order_1 in 1..25_usize,
                order_2 in 1..25_usize
            ) {
                let digraph = $type::path(order_1);
                let other = $type::path(order_2);

                assert_eq!(digraph.union(&other), other.union(&digraph));
            }

            #[test]
            fn union_path_complement_is_complete(order in 1..25_usize) {
                let digraph = $type::path(order);
                let complement = digraph.complement();

                assert!(digraph.union(&complement).is_complete());
                assert!(complement.union(&digraph).is_complete());

            }

            #[test]
            fn union_random_tournament_commutative(
                order_1 in 1..25_usize,
                order_2 in 1..25_usize,
                seed_1 in 0..1000_u64,
                seed_2 in 0..1000_u64
            ) {
                let digraph = $type::random_tournament(order_1, seed_1);
                let other = $type::random_tournament(order_2, seed_2);

                assert_eq!(digraph.union(&other), other.union(&digraph));
            }

            #[test]
            fn union_random_tournament_size_gte(
                order_1 in 1..25_usize,
                order_2 in 1..25_usize,
                seed_1 in 0..1000_u64,
                seed_2 in 0..1000_u64
            ) {
                let digraph = $type::random_tournament(order_1, seed_1);
                let other = $type::random_tournament(order_2, seed_2);

                assert!(digraph.size() + other.size() >= digraph.union(&other)
                    .size());
            }

            #[test]
            fn union_random_tournament_complement_is_complete(
                order in 1..25_usize,
                seed in 0..1000_u64
            ) {
                let digraph = $type::random_tournament(order, seed);
                let complement = digraph.complement();

                assert!(digraph.union(&complement).is_complete());
                assert!(complement.union(&digraph).is_complete());
            }

            #[test]
            fn union_star_commutative(
                order_1 in 1..25_usize,
                order_2 in 1..25_usize
            ) {
                let digraph = $type::star(order_1);
                let other = $type::star(order_2);

                assert_eq!(digraph.union(&other), other.union(&digraph));
            }

            #[test]
            fn union_star_complement_is_complete(order in 1..25_usize) {
                let digraph = $type::star(order);
                let complement = digraph.complement();

                assert!(digraph.union(&complement).is_complete());
                assert!(complement.union(&digraph).is_complete());
            }

            #[test]
            fn union_wheel_commutative(
                order_1 in 4..25_usize,
                order_2 in 4..25_usize
            ) {
                let digraph = $type::wheel(order_1);
                let other = $type::wheel(order_2);

                assert_eq!(digraph.union(&other), other.union(&digraph));
            }

            #[test]
            fn union_wheel_complement_is_complete(order in 4..25_usize) {
                let digraph = $type::wheel(order);
                let complement = digraph.complement();

                assert!(digraph.union(&complement).is_complete());
                assert!(complement.union(&digraph).is_complete());
            }

            #[test]
            fn wheel_complement_size(order in 4..25_usize) {
                assert_eq!(
                    $type::wheel(order).complement().size(),
                    (order - 1) * (order - 4)
                );
            }

            #[test]
            fn wheel_degree(order in 4..25_usize) {
                let digraph = $type::wheel(order);

                assert_eq!(digraph.degree(0), (order - 1) * 2);
                assert!((1..order).all(|u| digraph.degree(u) == 6));
            }

            #[test]
            fn wheel_degree_sequence(order in 4..25_usize) {
                let digraph = $type::wheel(order);
                let degree_sequence = &mut digraph.degree_sequence();

                assert_eq!(degree_sequence.next(), Some((order - 1) * 2));
                assert!(degree_sequence.all(|d| d == 6));
            }

            #[test]
            fn wheel_degree_sum_equals_2size(order in 4..25_usize) {
                let digraph = $type::wheel(order);

                assert_eq!(
                    digraph
                        .vertices()
                        .fold(0, |acc, u| acc + digraph.degree(u)),
                    2 * digraph.size()
                );
            }

            #[test]
            fn wheel_even_number_odd_degrees(order in 4..25_usize) {
                let digraph = $type::wheel(order);

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
            fn wheel_has_edge(order in 4..25_usize) {
                let digraph = $type::wheel(order);

                assert!((1..order).all(|u| digraph.has_edge(0, u)
                    && digraph.has_edge(u, ((u + order + 1) % order))
                    && digraph.has_edge(u, ((u + order - 1) % order))));
            }

            #[test]
            fn wheel_indegree(order in 4..25_usize) {
                let digraph = $type::wheel(order);

                assert_eq!(digraph.indegree(0), order - 1);
                assert!((1..order).all(|u| digraph.indegree(u) == 3));
            }

            #[test]
            fn wheel_indegree_sequence(order in 4..25_usize) {
                let digraph = $type::wheel(order);
                let indegree_sequence = &mut digraph.indegree_sequence();

                assert_eq!(indegree_sequence.next(), Some(order - 1));
                assert!(indegree_sequence.all(|d| d == 3));
            }

            #[test]
            fn wheel_is_balanced(order in 4..25_usize) {
                assert!($type::wheel(order).is_balanced());
            }

            #[test]
            fn wheel_is_complete(order in 4..25_usize) {
                assert!((order == 4) == $type::wheel(order).is_complete());
            }

            #[test]
            fn wheel_is_isolated(order in 4..25_usize) {
                let digraph = $type::wheel(order);

                assert!(digraph.vertices().all(|u| !digraph.is_isolated(u)));
            }

            #[test]
            fn wheel_is_oriented(order in 4..25_usize) {
                assert!(!$type::wheel(order).is_oriented());
            }

            #[test]
            fn wheel_is_pendant(order in 4..25_usize) {
                let digraph = $type::wheel(order);

                assert!(digraph.vertices().all(|u| !digraph.is_pendant(u)));
            }

            #[test]
            fn wheel_is_regular(order in 4..25_usize) {
                assert!((order == 4) == $type::wheel(order).is_regular());
            }

            #[test]
            fn wheel_is_semicomplete(order in 4..25_usize) {
                assert!((order == 4) == $type::wheel(order).is_semicomplete());
            }

            #[test]
            fn wheel_is_simple(order in 4..25_usize) {
                assert!($type::wheel(order).is_simple());
            }

            #[test]
            fn wheel_is_sink(order in 4..25_usize) {
                let digraph = $type::wheel(order);

                assert!(digraph.vertices().all(|u| !digraph.is_sink(u)));
            }

            #[test]
            fn wheel_is_source(order in 4..25_usize) {
                let digraph = $type::wheel(order);

                assert!(digraph.vertices().all(|u| !digraph.is_source(u)));
            }

            #[test]
            fn wheel_is_spanning_subdigraph(order in 4..25_usize) {
                let digraph = $type::wheel(order);

                assert!(digraph.is_spanning_subdigraph(&digraph));
            }

            #[test]
            fn wheel_is_subdigraph(order in 4..25_usize) {
                let digraph = $type::wheel(order);

                assert!(digraph.is_subdigraph(&digraph));
            }

            #[test]
            fn wheel_is_superdigraph(order in 4..25_usize) {
                let digraph = $type::wheel(order);

                assert!(digraph.is_superdigraph(&digraph));
            }

            #[test]
            fn wheel_is_symmetric(order in 4..25_usize) {
                assert!($type::wheel(order).is_symmetric());
            }

            #[test]
            fn wheel_is_tournament(order in 4..25_usize) {
                assert!(!$type::wheel(order).is_tournament());
            }

            #[test]
            fn wheel_max_degree(order in 4..25_usize) {
                assert_eq!($type::wheel(order).max_degree(), (order - 1) * 2);
            }

            #[test]
            fn wheel_max_indegree(order in 4..25_usize) {
                assert_eq!($type::wheel(order).max_indegree(), order - 1);
            }

            #[test]
            fn wheel_max_outdegree(order in 4..25_usize) {
                assert_eq!($type::wheel(order).max_outdegree(), order - 1);
            }

            #[test]
            fn wheel_min_degree(order in 4..25_usize) {
                assert_eq!($type::wheel(order).min_degree(), 6);
            }

            #[test]
            fn wheel_min_indegree(order in 4..25_usize) {
                assert_eq!($type::wheel(order).min_indegree(), 3);
            }

            #[test]
            fn wheel_min_outdegree(order in 4..25_usize) {
                assert_eq!($type::wheel(order).min_outdegree(), 3);
            }

            #[test]
            fn wheel_outdegree(order in 4..25_usize) {
                let digraph = $type::wheel(order);

                assert_eq!(digraph.outdegree(0), order - 1);
                assert!((1..order).all(|u| digraph.outdegree(u) == 3));
            }

            #[test]
            fn wheel_outdegree_sequence(order in 4..25_usize) {
                let digraph = $type::wheel(order);
                let outdegree_sequence = &mut digraph.outdegree_sequence();

                assert_eq!(outdegree_sequence.next(), Some(order - 1));
                assert!(outdegree_sequence.all(|d| d == 3));
            }

            #[test]
            fn wheel_semidegree_sequence(order in 4..25_usize) {
                let digraph = $type::wheel(order);
                let mut semidegree_sequence = digraph.semidegree_sequence();

                assert_eq!(
                    semidegree_sequence.next(),
                    Some((order - 1, order - 1))
                );

                assert!(semidegree_sequence.all(|d| d == (3, 3)));
            }

            #[test]
            fn wheel_sinks(order in 4..25_usize) {
                assert!($type::wheel(order).sinks().eq([]));
            }

            #[test]
            fn wheel_size(order in 4..25_usize) {
                assert_eq!($type::wheel(order).size(), (order - 1) * 4);
            }

            #[test]
            fn wheel_sources(order in 4..25_usize) {
                assert!($type::wheel(order).sources().eq([]));
            }
        }

        #[test]
        #[should_panic(expected = "u = 0 equals v = 0")]
        fn add_arc_u_equals_v() {
            $type::trivial().add_arc(0, 0);
        }

        #[test]
        fn arcs_bang_jensen_196() {
            assert!(bang_jensen_196().arcs().eq(vec![
                (0, 1),
                (0, 4),
                (0, 7),
                (1, 0),
                (1, 2),
                (1, 7),
                (2, 3),
                (3, 2),
                (3, 4),
                (4, 2),
                (5, 6),
                (6, 7),
                (7, 5)
            ]));
        }

        #[test]
        fn arcs_bang_jensen_34() {
            assert!(bang_jensen_34().arcs().eq(vec![
                (0, 4),
                (1, 0),
                (2, 1),
                (2, 3),
                (2, 5),
                (5, 4)
            ]));
        }

        #[test]
        fn arcs_bang_jensen_94() {
            assert!(bang_jensen_94().arcs().eq(vec![
                (0, 1),
                (0, 2),
                (1, 3),
                (2, 1),
                (2, 3),
                (2, 4),
                (2, 5),
                (3, 5),
                (4, 6)
            ]));
        }

        #[test]
        fn arcs_kattis_builddeps() {
            assert!(kattis_builddeps().arcs().eq(vec![
                (0, 3),
                (0, 4),
                (2, 3),
                (2, 4),
                (2, 5),
                (3, 1),
                (4, 1),
                (5, 1)
            ]));
        }

        #[test]
        fn arcs_kattis_cantinaofbabel_1() {
            assert!(kattis_cantinaofbabel_1().arcs().eq(vec![
                (0, 1),
                (1, 0),
                (1, 2),
                (1, 4),
                (2, 1),
                (3, 2),
                (3, 4),
                (3, 5),
                (3, 7),
                (3, 10),
                (3, 11),
                (4, 3),
                (5, 6),
                (6, 5),
                (6, 10),
                (7, 3),
                (8, 7),
                (8, 10),
                (9, 7),
                (9, 11),
                (10, 6),
                (11, 9)
            ]));
        }

        #[test]
        fn arcs_kattis_cantinaofbabel_2() {
            assert!(kattis_cantinaofbabel_2().arcs().eq(vec![
                (0, 1),
                (1, 0),
                (1, 7),
                (2, 0),
                (2, 5),
                (2, 7),
                (3, 4),
                (4, 3),
                (5, 3),
                (5, 6),
                (6, 5),
                (7, 2),
                (8, 7),
                (8, 9),
                (8, 11),
                (9, 8),
                (10, 9),
                (10, 11),
                (11, 10)
            ]));
        }

        #[test]
        fn arcs_kattis_escapewallmaria_1() {
            assert!(kattis_escapewallmaria_1().arcs().eq(vec![
                (5, 6),
                (5, 9),
                (6, 5),
                (9, 5),
                (9, 13),
                (13, 9),
                (13, 12)
            ]));
        }

        #[test]
        fn arcs_kattis_escapewallmaria_2() {
            assert!(kattis_escapewallmaria_2().arcs().eq(vec![
                (5, 6),
                (5, 9),
                (6, 5),
                (9, 5),
                (12, 13),
                (13, 9),
                (13, 12)
            ]));
        }

        #[test]
        fn arcs_kattis_escapewallmaria_3() {
            assert!(kattis_escapewallmaria_3().arcs().eq(vec![
                (1, 2),
                (1, 5),
                (2, 1),
                (2, 6),
                (5, 1),
                (5, 6),
                (5, 9),
                (6, 2),
                (6, 5),
                (9, 5),
                (9, 13),
                (12, 13),
                (13, 9),
                (13, 12)
            ]));
        }

        #[test]
        fn arcs_weighted_bang_jensen_196() {
            assert!(bang_jensen_196().arcs_weighted().eq(vec![
                (0, 1, &1),
                (0, 4, &1),
                (0, 7, &1),
                (1, 0, &1),
                (1, 2, &1),
                (1, 7, &1),
                (2, 3, &1),
                (3, 2, &1),
                (3, 4, &1),
                (4, 2, &1),
                (5, 6, &1),
                (6, 7, &1),
                (7, 5, &1)
            ]));
        }

        #[test]
        fn arcs_weighted_bang_jensen_34() {
            assert!(bang_jensen_34().arcs_weighted().eq(vec![
                (0, 4, &1),
                (1, 0, &1),
                (2, 1, &1),
                (2, 3, &1),
                (2, 5, &1),
                (5, 4, &1)
            ]));
        }

        #[test]
        fn arcs_weighted_bang_jensen_94() {
            assert!(bang_jensen_94().arcs_weighted().eq(vec![
                (0, 1, &1),
                (0, 2, &1),
                (1, 3, &1),
                (2, 1, &1),
                (2, 3, &1),
                (2, 4, &1),
                (2, 5, &1),
                (3, 5, &1),
                (4, 6, &1)
            ]));
        }

        #[test]
        fn arcs_weighted_kattis_builddeps() {
            assert!(kattis_builddeps().arcs_weighted().eq(vec![
                (0, 3, &1),
                (0, 4, &1),
                (2, 3, &1),
                (2, 4, &1),
                (2, 5, &1),
                (3, 1, &1),
                (4, 1, &1),
                (5, 1, &1)
            ]));
        }

        #[test]
        fn arcs_weighted_kattis_cantinaofbabel_1() {
            assert!(kattis_cantinaofbabel_1().arcs_weighted().eq(vec![
                (0, 1, &1),
                (1, 0, &1),
                (1, 2, &1),
                (1, 4, &1),
                (2, 1, &1),
                (3, 2, &1),
                (3, 4, &1),
                (3, 5, &1),
                (3, 7, &1),
                (3, 10, &1),
                (3, 11, &1),
                (4, 3, &1),
                (5, 6, &1),
                (6, 5, &1),
                (6, 10, &1),
                (7, 3, &1),
                (8, 7, &1),
                (8, 10, &1),
                (9, 7, &1),
                (9, 11, &1),
                (10, 6, &1),
                (11, 9, &1)
            ]));
        }

        #[test]
        fn arcs_weighted_kattis_cantinaofbabel_2() {
            assert!(kattis_cantinaofbabel_2().arcs_weighted().eq(vec![
                (0, 1, &1),
                (1, 0, &1),
                (1, 7, &1),
                (2, 0, &1),
                (2, 5, &1),
                (2, 7, &1),
                (3, 4, &1),
                (4, 3, &1),
                (5, 3, &1),
                (5, 6, &1),
                (6, 5, &1),
                (7, 2, &1),
                (8, 7, &1),
                (8, 9, &1),
                (8, 11, &1),
                (9, 8, &1),
                (10, 9, &1),
                (10, 11, &1),
                (11, 10, &1)
            ]));
        }

        #[test]
        fn arcs_weighted_kattis_escapewallmaria_1() {
            assert!(kattis_escapewallmaria_1().arcs_weighted().eq(vec![
                (5, 6, &1),
                (5, 9, &1),
                (6, 5, &1),
                (9, 5, &1),
                (9, 13, &1),
                (13, 9, &1),
                (13, 12, &1)
            ]));
        }

        #[test]
        fn arcs_weighted_kattis_escapewallmaria_2() {
            assert!(kattis_escapewallmaria_2().arcs_weighted().eq(vec![
                (5, 6, &1),
                (5, 9, &1),
                (6, 5, &1),
                (9, 5, &1),
                (12, 13, &1),
                (13, 9, &1),
                (13, 12, &1)
            ]));
        }

        #[test]
        fn arcs_weighted_kattis_escapewallmaria_3() {
            assert!(kattis_escapewallmaria_3().arcs_weighted().eq(vec![
                (1, 2, &1),
                (1, 5, &1),
                (2, 1, &1),
                (2, 6, &1),
                (5, 1, &1),
                (5, 6, &1),
                (5, 9, &1),
                (6, 2, &1),
                (6, 5, &1),
                (9, 5, &1),
                (9, 13, &1),
                (12, 13, &1),
                (13, 9, &1),
                (13, 12, &1)
            ]));
        }

        #[test]
        #[should_panic(expected = "m = 0 must be greater than zero")]
        fn biclique_0_1() {
            let _ = $type::biclique(0, 1);
        }

        #[test]
        #[should_panic(expected = "n = 0 must be greater than zero")]
        fn biclique_1_0() {
            let _ = $type::biclique(1, 0);
        }

        #[test]
        fn biclique_1_1() {
            let digraph = $type::biclique(1, 1);

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([(0, 1), (1, 0)]));
        }

        #[test]
        fn biclique_1_1_complement() {
            let digraph = $type::biclique(1, 1).complement();

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn biclique_1_2() {
            let digraph = $type::biclique(1, 2);

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([(0, 1), (0, 2), (1, 0), (2, 0)]));
        }

        #[test]
        fn biclique_1_2_complement() {
            let digraph = $type::biclique(1, 2).complement();

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([(1, 2), (2, 1)]));
        }

        #[test]
        fn biclique_2_1() {
            let digraph = $type::biclique(2, 1);

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([(0, 2), (1, 2), (2, 0), (2, 1)]));
        }

        #[test]
        fn biclique_2_1_complement() {
            let digraph = $type::biclique(2, 1).complement();

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([(0, 1), (1, 0)]));
        }

        #[test]
        fn biclique_2_2() {
            let digraph = $type::biclique(2, 2);

            assert_eq!(digraph.order(), 4);

            assert!(digraph.arcs().eq([
                (0, 2),
                (0, 3),
                (1, 2),
                (1, 3),
                (2, 0),
                (2, 1),
                (3, 0),
                (3, 1)
            ]));
        }

        #[test]
        fn biclique_2_2_complement() {
            let digraph = $type::biclique(2, 2).complement();

            assert_eq!(digraph.order(), 4);
            assert!(digraph.arcs().eq([(0, 1), (1, 0), (2, 3), (3, 2)]));
        }

        #[test]
        fn biclique_claw() {
            let digraph = $type::claw();

            assert_eq!(digraph.order(), 4);

            assert!(digraph.arcs().eq([
                (0, 1),
                (0, 2),
                (0, 3),
                (1, 0),
                (2, 0),
                (3, 0)
            ]));
        }

        #[test]
        fn biclique_utility() {
            let digraph = $type::utility();

            assert_eq!(digraph.order(), 6);

            assert!($type::utility().arcs().eq([
                (0, 3),
                (0, 4),
                (0, 5),
                (1, 3),
                (1, 4),
                (1, 5),
                (2, 3),
                (2, 4),
                (2, 5),
                (3, 0),
                (3, 1),
                (3, 2),
                (4, 0),
                (4, 1),
                (4, 2),
                (5, 0),
                (5, 1),
                (5, 2)
            ]));
        }

        #[test]
        #[should_panic(expected = "a digraph has at least one vertex")]
        fn circuit_0() {
            let _ = $type::circuit(0);
        }

        #[test]
        fn circuit_1() {
            let digraph = $type::circuit(1);

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn circuit_1_complement() {
            let digraph = $type::circuit(1).complement();

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn circuit_2() {
            let digraph = $type::circuit(2);

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([(0, 1), (1, 0)]));
        }

        #[test]
        fn circuit_2_complement() {
            let digraph = $type::circuit(2).complement();

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn circuit_3() {
            let digraph = $type::circuit(3);

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([(0, 1), (1, 2), (2, 0)]));
        }

        #[test]
        fn circuit_3_complement() {
            let digraph = $type::circuit(3).complement();

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([(0, 2), (1, 0), (2, 1)]));
        }

        #[test]
        #[should_panic(expected = "a digraph has at least one vertex")]
        fn complete_0() {
            let _ = $type::complete(0);
        }

        #[test]
        fn complete_1() {
            let digraph = $type::complete(1);

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn complete_1_complement() {
            let digraph = $type::complete(1).complement();

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn complete_2() {
            let digraph = $type::complete(2);

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([(0, 1), (1, 0)]));
        }

        #[test]
        fn complete_2_complement() {
            let digraph = $type::complete(2).complement();

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn complete_3() {
            let digraph = $type::complete(3);

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
            let digraph = $type::complete(3).complement();

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn converse_bang_jensen_196() {
            let digraph = bang_jensen_196();
            let converse = digraph.converse();

            assert_eq!(digraph.order(), converse.order());

            assert!(converse.arcs().eq([
                (0, 1),
                (1, 0),
                (2, 1),
                (2, 3),
                (2, 4),
                (3, 2),
                (4, 0),
                (4, 3),
                (5, 7),
                (6, 5),
                (7, 0),
                (7, 1),
                (7, 6),
            ]));
        }

        #[test]
        fn converse_bang_jensen_34() {
            let digraph = bang_jensen_34();
            let converse = digraph.converse();

            assert_eq!(digraph.order(), converse.order());

            assert!(converse.arcs().eq([
                (0, 1),
                (1, 2),
                (3, 2),
                (4, 0),
                (4, 5),
                (5, 2)
            ]));
        }

        #[test]
        fn converse_bang_jensen_94() {
            let digraph = bang_jensen_94();
            let converse = digraph.converse();

            assert_eq!(digraph.order(), converse.order());

            assert!(converse.arcs().eq([
                (1, 0),
                (1, 2),
                (2, 0),
                (3, 1),
                (3, 2),
                (4, 2),
                (5, 2),
                (5, 3),
                (6, 4)
            ]));
        }

        #[test]
        fn converse_kattis_builddeps() {
            let digraph = kattis_builddeps();
            let converse = digraph.converse();

            assert!(converse.arcs().eq([
                (1, 3),
                (1, 4),
                (1, 5),
                (3, 0),
                (3, 2),
                (4, 0),
                (4, 2),
                (5, 2)
            ]));
        }

        #[test]
        fn converse_kattis_cantinaofbabel_1() {
            let digraph = kattis_cantinaofbabel_1();
            let converse = digraph.converse();

            assert!(converse.arcs().eq([
                (0, 1),
                (1, 0),
                (1, 2),
                (2, 1),
                (2, 3),
                (3, 4),
                (3, 7),
                (4, 1),
                (4, 3),
                (5, 3),
                (5, 6),
                (6, 5),
                (6, 10),
                (7, 3),
                (7, 8),
                (7, 9),
                (9, 11),
                (10, 3),
                (10, 6),
                (10, 8),
                (11, 3),
                (11, 9)
            ]));
        }

        #[test]
        fn converse_kattis_cantinaofbabel_2() {
            let digraph = kattis_cantinaofbabel_2();
            let converse = digraph.converse();

            assert_eq!(digraph.order(), converse.order());

            assert!(converse.arcs().eq([
                (0, 1),
                (0, 2),
                (1, 0),
                (2, 7),
                (3, 4),
                (3, 5),
                (4, 3),
                (5, 2),
                (5, 6),
                (6, 5),
                (7, 1),
                (7, 2),
                (7, 8),
                (8, 9),
                (9, 8),
                (9, 10),
                (10, 11),
                (11, 8),
                (11, 10)
            ]));
        }

        #[test]
        fn converse_kattis_escapewallmaria_1() {
            let digraph = kattis_escapewallmaria_1();
            let converse = digraph.converse();

            assert_eq!(digraph.order(), converse.order());

            assert!(converse.arcs().eq([
                (5, 6),
                (5, 9),
                (6, 5),
                (9, 5),
                (9, 13),
                (12, 13),
                (13, 9)
            ]));
        }

        #[test]
        fn converse_kattis_escapewallmaria_2() {
            let digraph = kattis_escapewallmaria_2();
            let converse = digraph.converse();

            assert_eq!(digraph.order(), converse.order());

            assert!(converse.arcs().eq([
                (5, 6),
                (5, 9),
                (6, 5),
                (9, 5),
                (9, 13),
                (12, 13),
                (13, 12)
            ]));
        }

        #[test]
        fn converse_kattis_escapewallmaria_3() {
            let digraph = kattis_escapewallmaria_3();
            let converse = digraph.converse();

            assert_eq!(digraph.order(), converse.order());

            assert!(converse.arcs().eq([
                (1, 2),
                (1, 5),
                (2, 1),
                (2, 6),
                (5, 1),
                (5, 6),
                (5, 9),
                (6, 2),
                (6, 5),
                (9, 5),
                (9, 13),
                (12, 13),
                (13, 9),
                (13, 12)
            ]));
        }

        #[test]
        #[should_panic(expected = "a digraph has at least one vertex")]
        fn cycle_0() {
            let _ = $type::cycle(0);
        }

        #[test]
        fn cycle_1() {
            let digraph = $type::cycle(1);

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn cycle_1_complement() {
            let digraph = $type::cycle(1).complement();

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn cycle_2() {
            let digraph = $type::cycle(2);

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([(0, 1), (1, 0)]));
        }

        #[test]
        fn cycle_2_complement() {
            let digraph = $type::cycle(2).complement();

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn cycle_3() {
            let digraph = $type::cycle(3);

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
            let digraph = $type::cycle(3).complement();

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn degree_bang_jensen_196() {
            let digraph = bang_jensen_196();

            assert!(digraph.degree(0) == 4);
            assert!(digraph.degree(1) == 4);
            assert!(digraph.degree(2) == 4);
            assert!(digraph.degree(3) == 3);
            assert!(digraph.degree(4) == 3);
            assert!(digraph.degree(5) == 2);
            assert!(digraph.degree(6) == 2);
            assert!(digraph.degree(7) == 4);
        }

        #[test]
        fn degree_bang_jensen_34() {
            let digraph = bang_jensen_34();

            assert!(digraph.degree(0) == 2);
            assert!(digraph.degree(1) == 2);
            assert!(digraph.degree(2) == 3);
            assert!(digraph.degree(3) == 1);
            assert!(digraph.degree(4) == 2);
            assert!(digraph.degree(5) == 2);
        }

        #[test]
        fn degree_bang_jensen_94() {
            let digraph = bang_jensen_94();

            assert!(digraph.degree(0) == 2);
            assert!(digraph.degree(1) == 3);
            assert!(digraph.degree(2) == 5);
            assert!(digraph.degree(3) == 3);
            assert!(digraph.degree(4) == 2);
            assert!(digraph.degree(5) == 2);
            assert!(digraph.degree(6) == 1);
        }

        #[test]
        fn degree_kattis_builddeps() {
            let digraph = kattis_builddeps();

            assert!(digraph.degree(0) == 2);
            assert!(digraph.degree(1) == 3);
            assert!(digraph.degree(2) == 3);
            assert!(digraph.degree(3) == 3);
            assert!(digraph.degree(4) == 3);
            assert!(digraph.degree(5) == 2);
        }

        #[test]
        fn degree_kattis_cantinaofbabel_1() {
            let digraph = kattis_cantinaofbabel_1();

            assert!(digraph.degree(0) == 2);
            assert!(digraph.degree(1) == 5);
            assert!(digraph.degree(2) == 3);
            assert!(digraph.degree(3) == 8);
            assert!(digraph.degree(4) == 3);
            assert!(digraph.degree(5) == 3);
            assert!(digraph.degree(6) == 4);
            assert!(digraph.degree(7) == 4);
            assert!(digraph.degree(8) == 2);
            assert!(digraph.degree(9) == 3);
            assert!(digraph.degree(10) == 4);
            assert!(digraph.degree(11) == 3);
        }

        #[test]
        fn degree_kattis_cantinaofbabel_2() {
            let digraph = kattis_cantinaofbabel_2();

            assert!(digraph.degree(0) == 3);
            assert!(digraph.degree(1) == 3);
            assert!(digraph.degree(2) == 4);
            assert!(digraph.degree(3) == 3);
            assert!(digraph.degree(4) == 2);
            assert!(digraph.degree(5) == 4);
            assert!(digraph.degree(6) == 2);
            assert!(digraph.degree(7) == 4);
            assert!(digraph.degree(8) == 4);
            assert!(digraph.degree(9) == 3);
            assert!(digraph.degree(10) == 3);
            assert!(digraph.degree(11) == 3);
        }

        #[test]
        fn degree_kattis_escapewallmaria_1() {
            let digraph = kattis_escapewallmaria_1();

            assert!(digraph.degree(0) == 0);
            assert!(digraph.degree(1) == 0);
            assert!(digraph.degree(2) == 0);
            assert!(digraph.degree(3) == 0);
            assert!(digraph.degree(4) == 0);
            assert!(digraph.degree(5) == 4);
            assert!(digraph.degree(6) == 2);
            assert!(digraph.degree(7) == 0);
            assert!(digraph.degree(8) == 0);
            assert!(digraph.degree(9) == 4);
            assert!(digraph.degree(10) == 0);
            assert!(digraph.degree(11) == 0);
            assert!(digraph.degree(12) == 1);
            assert!(digraph.degree(13) == 3);
        }

        #[test]
        fn degree_kattis_escapewallmaria_2() {
            let digraph = kattis_escapewallmaria_2();

            assert!(digraph.degree(0) == 0);
            assert!(digraph.degree(1) == 0);
            assert!(digraph.degree(2) == 0);
            assert!(digraph.degree(3) == 0);
            assert!(digraph.degree(4) == 0);
            assert!(digraph.degree(5) == 4);
            assert!(digraph.degree(6) == 2);
            assert!(digraph.degree(7) == 0);
            assert!(digraph.degree(8) == 0);
            assert!(digraph.degree(9) == 3);
            assert!(digraph.degree(10) == 0);
            assert!(digraph.degree(11) == 0);
            assert!(digraph.degree(12) == 2);
            assert!(digraph.degree(13) == 3);
        }

        #[test]
        fn degree_kattis_escapewallmaria_3() {
            let digraph = kattis_escapewallmaria_3();

            assert!(digraph.degree(0) == 0);
            assert!(digraph.degree(1) == 4);
            assert!(digraph.degree(2) == 4);
            assert!(digraph.degree(3) == 0);
            assert!(digraph.degree(4) == 0);
            assert!(digraph.degree(5) == 6);
            assert!(digraph.degree(6) == 4);
            assert!(digraph.degree(7) == 0);
            assert!(digraph.degree(8) == 0);
            assert!(digraph.degree(9) == 4);
            assert!(digraph.degree(10) == 0);
            assert!(digraph.degree(11) == 0);
            assert!(digraph.degree(12) == 2);
            assert!(digraph.degree(13) == 4);
        }

        #[test]
        fn degree_sequence_bang_jensen_196() {
            assert!(bang_jensen_196()
                .degree_sequence()
                .eq([4, 4, 4, 3, 3, 2, 2, 4]));
        }

        #[test]
        fn degree_sequence_bang_jensen_34() {
            assert!(bang_jensen_34().degree_sequence().eq([2, 2, 3, 1, 2, 2]));
        }

        #[test]
        fn degree_sequence_bang_jensen_94() {
            assert!(bang_jensen_94()
                .degree_sequence()
                .eq([2, 3, 5, 3, 2, 2, 1]));
        }

        #[test]
        fn degree_sequence_kattis_builddeps() {
            assert!(kattis_builddeps()
                .degree_sequence()
                .eq([2, 3, 3, 3, 3, 2]));
        }

        #[test]
        fn degree_sequence_kattis_cantinaofbabel_1() {
            assert!(kattis_cantinaofbabel_1()
                .degree_sequence()
                .eq([2, 5, 3, 8, 3, 3, 4, 4, 2, 3, 4, 3]));
        }

        #[test]
        fn degree_sequence_kattis_cantinaofbabel_2() {
            assert!(kattis_cantinaofbabel_2()
                .degree_sequence()
                .eq([3, 3, 4, 3, 2, 4, 2, 4, 4, 3, 3, 3]));
        }

        #[test]
        fn degree_sequence_kattis_escapewallmaria_1() {
            assert!(kattis_escapewallmaria_1()
                .degree_sequence()
                .eq([0, 0, 0, 0, 0, 4, 2, 0, 0, 4, 0, 0, 1, 3, 0, 0]));
        }

        #[test]
        fn degree_sequence_kattis_escapewallmaria_2() {
            assert!(kattis_escapewallmaria_2()
                .degree_sequence()
                .eq([0, 0, 0, 0, 0, 4, 2, 0, 0, 3, 0, 0, 2, 3, 0, 0]));
        }

        #[test]
        fn degree_sequence_kattis_escapewallmaria_3() {
            assert!(kattis_escapewallmaria_3()
                .degree_sequence()
                .eq([0, 4, 4, 0, 0, 6, 4, 0, 0, 4, 0, 0, 2, 4, 0, 0]));
        }

        #[test]
        #[should_panic(expected = "a digraph has at least one vertex")]
        fn empty_0() {
            let _ = $type::empty(0);
        }

        #[test]
        fn empty_1() {
            let digraph = $type::empty(1);

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn empty_1_complement() {
            let digraph = $type::empty(1).complement();

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn empty_2() {
            let digraph = $type::empty(2);

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn empty_2_complement() {
            let digraph = $type::empty(2).complement();

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([(0, 1), (1, 0)]));
        }

        #[test]
        fn empty_3() {
            let digraph = $type::empty(3);

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn empty_3_complement() {
            let digraph = $type::empty(3).complement();

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
            let digraph = $type::trivial();

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        #[should_panic(expected = "a digraph has at least one vertex")]
        fn erdos_renyi_0() {
            let _ = $type::erdos_renyi(0, 0.5, 0);
        }

        #[test]
        #[should_panic(expected = "p = -0.1 must be in [0, 1]")]
        fn erdos_renyi_p_negative() {
            let _ = $type::erdos_renyi(2, -0.1, 0);
        }

        #[test]
        #[should_panic(expected = "p = 1.1 must be in [0, 1]")]
        fn erdos_renyi_p_gt_1() {
            let _ = $type::erdos_renyi(2, 1.1, 0);
        }

        #[test]
        #[should_panic(expected = "a digraph has at least one vertex")]
        fn growing_network_0() {
            let _ = $type::growing_network(0, 0);
        }

        #[test]
        fn in_neighbors_bang_jensen_196() {
            let digraph = bang_jensen_196();

            assert!(digraph.in_neighbors(0).eq([1]));
            assert!(digraph.in_neighbors(1).eq([0]));
            assert!(digraph.in_neighbors(2).eq([1, 3, 4]));
            assert!(digraph.in_neighbors(3).eq([2]));
            assert!(digraph.in_neighbors(4).eq([0, 3]));
            assert!(digraph.in_neighbors(5).eq([7]));
            assert!(digraph.in_neighbors(6).eq([5]));
            assert!(digraph.in_neighbors(7).eq([0, 1, 6]));
        }

        #[test]
        fn in_neighbors_bang_jensen_34() {
            let digraph = bang_jensen_34();

            assert!(digraph.in_neighbors(0).eq([1]));
            assert!(digraph.in_neighbors(1).eq([2]));
            assert!(digraph.in_neighbors(2).eq([]));
            assert!(digraph.in_neighbors(3).eq([2]));
            assert!(digraph.in_neighbors(4).eq([0, 5]));
            assert!(digraph.in_neighbors(5).eq([2]));
        }

        #[test]
        fn in_neighbors_bang_jensen_94() {
            let digraph = bang_jensen_94();

            assert!(digraph.in_neighbors(0).eq([]));
            assert!(digraph.in_neighbors(1).eq([0, 2]));
            assert!(digraph.in_neighbors(2).eq([0]));
            assert!(digraph.in_neighbors(3).eq([1, 2]));
            assert!(digraph.in_neighbors(4).eq([2]));
            assert!(digraph.in_neighbors(5).eq([2, 3]));
            assert!(digraph.in_neighbors(6).eq([4]));
        }

        #[test]
        fn in_neighbors_kattis_builddeps() {
            let digraph = kattis_builddeps();

            assert!(digraph.in_neighbors(0).eq([]));
            assert!(digraph.in_neighbors(1).eq([3, 4, 5]));
            assert!(digraph.in_neighbors(2).eq([]));
            assert!(digraph.in_neighbors(3).eq([0, 2]));
            assert!(digraph.in_neighbors(4).eq([0, 2]));
            assert!(digraph.in_neighbors(5).eq([2]));
        }

        #[test]
        fn in_neighbors_kattis_cantinaofbabel_1() {
            let digraph = kattis_cantinaofbabel_1();

            assert!(digraph.in_neighbors(0).eq([1]));
            assert!(digraph.in_neighbors(1).eq([0, 2]));
            assert!(digraph.in_neighbors(2).eq([1, 3]));
            assert!(digraph.in_neighbors(3).eq([4, 7]));
            assert!(digraph.in_neighbors(4).eq([1, 3]));
            assert!(digraph.in_neighbors(5).eq([3, 6]));
            assert!(digraph.in_neighbors(6).eq([5, 10]));
            assert!(digraph.in_neighbors(7).eq([3, 8, 9]));
            assert!(digraph.in_neighbors(8).eq([]));
            assert!(digraph.in_neighbors(9).eq([11]));
            assert!(digraph.in_neighbors(10).eq([3, 6, 8]));
            assert!(digraph.in_neighbors(11).eq([3, 9]));
        }

        #[test]
        fn in_neighbors_kattis_cantinaofbabel_2() {
            let digraph = kattis_cantinaofbabel_2();

            assert!(digraph.in_neighbors(0).eq([1, 2]));
            assert!(digraph.in_neighbors(1).eq([0]));
            assert!(digraph.in_neighbors(2).eq([7]));
            assert!(digraph.in_neighbors(3).eq([4, 5]));
            assert!(digraph.in_neighbors(4).eq([3]));
            assert!(digraph.in_neighbors(5).eq([2, 6]));
            assert!(digraph.in_neighbors(6).eq([5]));
            assert!(digraph.in_neighbors(7).eq([1, 2, 8]));
            assert!(digraph.in_neighbors(8).eq([9]));
            assert!(digraph.in_neighbors(9).eq([8, 10]));
            assert!(digraph.in_neighbors(10).eq([11]));
            assert!(digraph.in_neighbors(11).eq([8, 10]));
        }

        #[test]
        fn in_neighbors_kattis_escapewallmaria_1() {
            let digraph = kattis_escapewallmaria_1();

            assert!(digraph.in_neighbors(0).eq([]));
            assert!(digraph.in_neighbors(1).eq([]));
            assert!(digraph.in_neighbors(2).eq([]));
            assert!(digraph.in_neighbors(3).eq([]));
            assert!(digraph.in_neighbors(4).eq([]));
            assert!(digraph.in_neighbors(5).eq([6, 9]));
            assert!(digraph.in_neighbors(6).eq([5]));
            assert!(digraph.in_neighbors(7).eq([]));
            assert!(digraph.in_neighbors(8).eq([]));
            assert!(digraph.in_neighbors(9).eq([5, 13]));
            assert!(digraph.in_neighbors(10).eq([]));
            assert!(digraph.in_neighbors(11).eq([]));
            assert!(digraph.in_neighbors(12).eq([13]));
            assert!(digraph.in_neighbors(13).eq([9]));
            assert!(digraph.in_neighbors(14).eq([]));
            assert!(digraph.in_neighbors(15).eq([]));
        }

        #[test]
        fn in_neighbors_kattis_escapewallmaria_2() {
            let digraph = kattis_escapewallmaria_2();

            assert!(digraph.in_neighbors(0).eq([]));
            assert!(digraph.in_neighbors(1).eq([]));
            assert!(digraph.in_neighbors(2).eq([]));
            assert!(digraph.in_neighbors(3).eq([]));
            assert!(digraph.in_neighbors(4).eq([]));
            assert!(digraph.in_neighbors(5).eq([6, 9]));
            assert!(digraph.in_neighbors(6).eq([5]));
            assert!(digraph.in_neighbors(7).eq([]));
            assert!(digraph.in_neighbors(8).eq([]));
            assert!(digraph.in_neighbors(9).eq([5, 13]));
            assert!(digraph.in_neighbors(10).eq([]));
            assert!(digraph.in_neighbors(11).eq([]));
            assert!(digraph.in_neighbors(12).eq([13]));
            assert!(digraph.in_neighbors(13).eq([12]));
            assert!(digraph.in_neighbors(14).eq([]));
            assert!(digraph.in_neighbors(15).eq([]));
        }

        #[test]
        fn in_neighbors_kattis_escapewallmaria_3() {
            let digraph = kattis_escapewallmaria_3();

            assert!(digraph.in_neighbors(0).eq([]));
            assert!(digraph.in_neighbors(1).eq([2, 5]));
            assert!(digraph.in_neighbors(2).eq([1, 6]));
            assert!(digraph.in_neighbors(3).eq([]));
            assert!(digraph.in_neighbors(4).eq([]));
            assert!(digraph.in_neighbors(5).eq([1, 6, 9]));
            assert!(digraph.in_neighbors(6).eq([2, 5]));
            assert!(digraph.in_neighbors(7).eq([]));
            assert!(digraph.in_neighbors(8).eq([]));
            assert!(digraph.in_neighbors(9).eq([5, 13]));
            assert!(digraph.in_neighbors(10).eq([]));
            assert!(digraph.in_neighbors(11).eq([]));
            assert!(digraph.in_neighbors(12).eq([13]));
            assert!(digraph.in_neighbors(13).eq([9, 12]));
            assert!(digraph.in_neighbors(14).eq([]));
            assert!(digraph.in_neighbors(15).eq([]));
        }

        #[test]
        fn indegree_bang_jensen_196() {
            let digraph = bang_jensen_196();

            assert!(digraph.indegree(0) == 1);
            assert!(digraph.indegree(1) == 1);
            assert!(digraph.indegree(2) == 3);
            assert!(digraph.indegree(3) == 1);
            assert!(digraph.indegree(4) == 2);
            assert!(digraph.indegree(5) == 1);
            assert!(digraph.indegree(6) == 1);
            assert!(digraph.indegree(7) == 3);
        }

        #[test]
        fn indegree_bang_jensen_34() {
            let digraph = bang_jensen_34();

            assert!(digraph.indegree(0) == 1);
            assert!(digraph.indegree(1) == 1);
            assert!(digraph.indegree(2) == 0);
            assert!(digraph.indegree(3) == 1);
            assert!(digraph.indegree(4) == 2);
            assert!(digraph.indegree(5) == 1);
        }

        #[test]
        fn indegree_bang_jensen_94() {
            let digraph = bang_jensen_94();

            assert!(digraph.indegree(0) == 0);
            assert!(digraph.indegree(1) == 2);
            assert!(digraph.indegree(2) == 1);
            assert!(digraph.indegree(3) == 2);
            assert!(digraph.indegree(4) == 1);
            assert!(digraph.indegree(5) == 2);
            assert!(digraph.indegree(6) == 1);
        }

        #[test]
        fn indegree_kattis_builddeps() {
            let digraph = kattis_builddeps();

            assert!(digraph.indegree(0) == 0);
            assert!(digraph.indegree(1) == 3);
            assert!(digraph.indegree(2) == 0);
            assert!(digraph.indegree(3) == 2);
            assert!(digraph.indegree(4) == 2);
            assert!(digraph.indegree(5) == 1);
        }

        #[test]
        fn indegree_kattis_cantinaofbabel_1() {
            let digraph = kattis_cantinaofbabel_1();

            assert!(digraph.indegree(0) == 1);
            assert!(digraph.indegree(1) == 2);
            assert!(digraph.indegree(2) == 2);
            assert!(digraph.indegree(3) == 2);
            assert!(digraph.indegree(4) == 2);
            assert!(digraph.indegree(5) == 2);
            assert!(digraph.indegree(6) == 2);
            assert!(digraph.indegree(7) == 3);
            assert!(digraph.indegree(8) == 0);
            assert!(digraph.indegree(9) == 1);
            assert!(digraph.indegree(10) == 3);
            assert!(digraph.indegree(11) == 2);
        }

        #[test]
        fn indegree_kattis_cantinaofbabel_2() {
            let digraph = kattis_cantinaofbabel_2();

            assert!(digraph.indegree(0) == 2);
            assert!(digraph.indegree(1) == 1);
            assert!(digraph.indegree(2) == 1);
            assert!(digraph.indegree(3) == 2);
            assert!(digraph.indegree(4) == 1);
            assert!(digraph.indegree(5) == 2);
            assert!(digraph.indegree(6) == 1);
            assert!(digraph.indegree(7) == 3);
            assert!(digraph.indegree(8) == 1);
            assert!(digraph.indegree(9) == 2);
            assert!(digraph.indegree(10) == 1);
            assert!(digraph.indegree(11) == 2);
        }

        #[test]
        fn indegree_kattis_escapewallmaria_1() {
            let digraph = kattis_escapewallmaria_1();

            assert!(digraph.indegree(0) == 0);
            assert!(digraph.indegree(1) == 0);
            assert!(digraph.indegree(2) == 0);
            assert!(digraph.indegree(3) == 0);
            assert!(digraph.indegree(4) == 0);
            assert!(digraph.indegree(5) == 2);
            assert!(digraph.indegree(6) == 1);
            assert!(digraph.indegree(7) == 0);
            assert!(digraph.indegree(8) == 0);
            assert!(digraph.indegree(9) == 2);
            assert!(digraph.indegree(10) == 0);
            assert!(digraph.indegree(11) == 0);
            assert!(digraph.indegree(12) == 1);
            assert!(digraph.indegree(13) == 1);
            assert!(digraph.indegree(14) == 0);
            assert!(digraph.indegree(15) == 0);
        }

        #[test]
        fn indegree_kattis_escapewallmaria_2() {
            let digraph = kattis_escapewallmaria_2();

            assert!(digraph.indegree(0) == 0);
            assert!(digraph.indegree(1) == 0);
            assert!(digraph.indegree(2) == 0);
            assert!(digraph.indegree(3) == 0);
            assert!(digraph.indegree(4) == 0);
            assert!(digraph.indegree(5) == 2);
            assert!(digraph.indegree(6) == 1);
            assert!(digraph.indegree(7) == 0);
            assert!(digraph.indegree(8) == 0);
            assert!(digraph.indegree(9) == 2);
            assert!(digraph.indegree(10) == 0);
            assert!(digraph.indegree(11) == 0);
            assert!(digraph.indegree(12) == 1);
            assert!(digraph.indegree(13) == 1);
            assert!(digraph.indegree(14) == 0);
            assert!(digraph.indegree(15) == 0);
        }

        #[test]
        fn indegree_kattis_escapewallmaria_3() {
            let digraph = kattis_escapewallmaria_3();

            assert!(digraph.indegree(0) == 0);
            assert!(digraph.indegree(1) == 2);
            assert!(digraph.indegree(2) == 2);
            assert!(digraph.indegree(3) == 0);
            assert!(digraph.indegree(4) == 0);
            assert!(digraph.indegree(5) == 3);
            assert!(digraph.indegree(6) == 2);
            assert!(digraph.indegree(7) == 0);
            assert!(digraph.indegree(8) == 0);
            assert!(digraph.indegree(9) == 2);
            assert!(digraph.indegree(10) == 0);
            assert!(digraph.indegree(11) == 0);
            assert!(digraph.indegree(12) == 1);
            assert!(digraph.indegree(13) == 2);
            assert!(digraph.indegree(14) == 0);
            assert!(digraph.indegree(15) == 0);
        }

        #[test]
        #[should_panic(expected = "v = 1 isn't in the digraph")]
        fn indegree_out_of_bounds() {
            let _ = $type::trivial().indegree(1);
        }

        #[test]
        fn indegree_sequence_bang_jensen_196() {
            assert!(bang_jensen_196()
                .indegree_sequence()
                .eq([1, 1, 3, 1, 2, 1, 1, 3]));
        }

        #[test]
        fn indegree_sequence_bang_jensen_34() {
            assert!(bang_jensen_34()
                .indegree_sequence()
                .eq([1, 1, 0, 1, 2, 1]));
        }

        #[test]
        fn indegree_sequence_bang_jensen_94() {
            assert!(bang_jensen_94()
                .indegree_sequence()
                .eq([0, 2, 1, 2, 1, 2, 1]));
        }

        #[test]
        fn indegree_sequence_kattis_builddeps() {
            assert!(kattis_builddeps()
                .indegree_sequence()
                .eq([0, 3, 0, 2, 2, 1]));
        }

        #[test]
        fn indegree_sequence_kattis_cantinaofbabel_1() {
            assert!(kattis_cantinaofbabel_1()
                .indegree_sequence()
                .eq([1, 2, 2, 2, 2, 2, 2, 3, 0, 1, 3, 2]));
        }

        #[test]
        fn indegree_sequence_kattis_cantinaofbabel_2() {
            assert!(kattis_cantinaofbabel_2()
                .indegree_sequence()
                .eq([2, 1, 1, 2, 1, 2, 1, 3, 1, 2, 1, 2]));
        }

        #[test]
        fn indegree_sequence_kattis_escapewallmaria_1() {
            assert!(kattis_escapewallmaria_1()
                .indegree_sequence()
                .eq([0, 0, 0, 0, 0, 2, 1, 0, 0, 2, 0, 0, 1, 1, 0, 0]));
        }

        #[test]
        fn indegree_sequence_kattis_escapewallmaria_2() {
            assert!(kattis_escapewallmaria_2()
                .indegree_sequence()
                .eq([0, 0, 0, 0, 0, 2, 1, 0, 0, 2, 0, 0, 1, 1, 0, 0]));
        }

        #[test]
        fn indegree_sequence_kattis_escapewallmaria_3() {
            assert!(kattis_escapewallmaria_3()
                .indegree_sequence()
                .eq([0, 2, 2, 0, 0, 3, 2, 0, 0, 2, 0, 0, 1, 2, 0, 0]));
        }

        #[test]
        fn is_balanced_bang_jensen_196() {
            assert!(!bang_jensen_196().is_balanced());
        }

        #[test]
        fn is_balanced_bang_jensen_34() {
            assert!(!bang_jensen_34().is_balanced());
        }

        #[test]
        fn is_balanced_bang_jensen_94() {
            assert!(!bang_jensen_94().is_balanced());
        }

        #[test]
        fn is_balanced_kattis_builddeps() {
            assert!(!kattis_builddeps().is_balanced());
        }

        #[test]
        fn is_balanced_kattis_cantinaofbabel_1() {
            assert!(!kattis_cantinaofbabel_1().is_balanced());
        }

        #[test]
        fn is_balanced_kattis_cantinaofbabel_2() {
            assert!(!kattis_cantinaofbabel_2().is_balanced());
        }

        #[test]
        fn is_balanced_kattis_escapewallmaria_1() {
            assert!(!kattis_escapewallmaria_1().is_balanced());
        }

        #[test]
        fn is_balanced_kattis_escapewallmaria_2() {
            assert!(!kattis_escapewallmaria_2().is_balanced());
        }

        #[test]
        fn is_balanced_kattis_escapewallmaria_3() {
            assert!(kattis_escapewallmaria_3().is_balanced());
        }

        #[test]
        fn is_complete_bang_jensen_196() {
            assert!(!bang_jensen_196().is_complete());
        }

        #[test]
        fn is_complete_bang_jensen_34() {
            assert!(!bang_jensen_34().is_complete());
        }

        #[test]
        fn is_complete_bang_jensen_94() {
            assert!(!bang_jensen_94().is_complete());
        }

        #[test]
        fn is_complete_kattis_builddeps() {
            assert!(!kattis_builddeps().is_complete());
        }

        #[test]
        fn is_complete_kattis_cantinaofbabel_1() {
            assert!(!kattis_cantinaofbabel_1().is_complete());
        }

        #[test]
        fn is_complete_kattis_cantinaofbabel_2() {
            assert!(!kattis_cantinaofbabel_2().is_complete());
        }

        #[test]
        fn is_complete_kattis_escapewallmaria_1() {
            assert!(!kattis_escapewallmaria_1().is_complete());
        }

        #[test]
        fn is_complete_kattis_escapewallmaria_2() {
            assert!(!kattis_escapewallmaria_2().is_complete());
        }

        #[test]
        fn is_complete_kattis_escapewallmaria_3() {
            assert!(!kattis_escapewallmaria_3().is_complete());
        }

        #[test]
        fn is_isolated_bang_jensen_196() {
            let digraph = bang_jensen_196();

            assert!(!digraph.is_isolated(0));
            assert!(!digraph.is_isolated(1));
            assert!(!digraph.is_isolated(2));
            assert!(!digraph.is_isolated(3));
            assert!(!digraph.is_isolated(4));
            assert!(!digraph.is_isolated(5));
            assert!(!digraph.is_isolated(6));
            assert!(!digraph.is_isolated(7));
        }

        #[test]
        fn is_isolated_bang_jensen_34() {
            let digraph = bang_jensen_34();

            assert!(!digraph.is_isolated(0));
            assert!(!digraph.is_isolated(1));
            assert!(!digraph.is_isolated(2));
            assert!(!digraph.is_isolated(3));
            assert!(!digraph.is_isolated(4));
            assert!(!digraph.is_isolated(5));
        }

        #[test]
        fn is_isolated_bang_jensen_94() {
            let digraph = bang_jensen_94();

            assert!(!digraph.is_isolated(0));
            assert!(!digraph.is_isolated(1));
            assert!(!digraph.is_isolated(2));
            assert!(!digraph.is_isolated(3));
            assert!(!digraph.is_isolated(4));
            assert!(!digraph.is_isolated(5));
            assert!(!digraph.is_isolated(6));
        }

        #[test]
        fn is_isolated_kattis_builddeps() {
            let digraph = kattis_builddeps();

            assert!(!digraph.is_isolated(0));
            assert!(!digraph.is_isolated(1));
            assert!(!digraph.is_isolated(2));
            assert!(!digraph.is_isolated(3));
            assert!(!digraph.is_isolated(4));
            assert!(!digraph.is_isolated(5));
        }

        #[test]
        fn is_isolated_kattis_cantinaofbabel_1() {
            let digraph = kattis_cantinaofbabel_1();

            assert!(!digraph.is_isolated(0));
            assert!(!digraph.is_isolated(1));
            assert!(!digraph.is_isolated(2));
            assert!(!digraph.is_isolated(3));
            assert!(!digraph.is_isolated(4));
            assert!(!digraph.is_isolated(5));
            assert!(!digraph.is_isolated(6));
            assert!(!digraph.is_isolated(7));
            assert!(!digraph.is_isolated(8));
            assert!(!digraph.is_isolated(9));
            assert!(!digraph.is_isolated(10));
            assert!(!digraph.is_isolated(11));
        }

        #[test]
        fn is_isolated_kattis_cantinaofbabel_2() {
            let digraph = kattis_cantinaofbabel_2();

            assert!(!digraph.is_isolated(0));
            assert!(!digraph.is_isolated(1));
            assert!(!digraph.is_isolated(2));
            assert!(!digraph.is_isolated(3));
            assert!(!digraph.is_isolated(4));
            assert!(!digraph.is_isolated(5));
            assert!(!digraph.is_isolated(6));
            assert!(!digraph.is_isolated(7));
            assert!(!digraph.is_isolated(8));
            assert!(!digraph.is_isolated(9));
            assert!(!digraph.is_isolated(10));
            assert!(!digraph.is_isolated(11));
        }

        #[test]
        fn is_isolated_kattis_escapewallmaria_1() {
            let digraph = kattis_escapewallmaria_1();

            assert!(digraph.is_isolated(0));
            assert!(digraph.is_isolated(1));
            assert!(digraph.is_isolated(2));
            assert!(digraph.is_isolated(3));
            assert!(digraph.is_isolated(4));
            assert!(!digraph.is_isolated(5));
            assert!(!digraph.is_isolated(6));
            assert!(digraph.is_isolated(7));
            assert!(digraph.is_isolated(8));
            assert!(!digraph.is_isolated(9));
            assert!(digraph.is_isolated(10));
            assert!(digraph.is_isolated(11));
            assert!(!digraph.is_isolated(12));
            assert!(!digraph.is_isolated(13));
            assert!(digraph.is_isolated(14));
            assert!(digraph.is_isolated(15));
        }

        #[test]
        fn is_isolated_kattis_escapewallmaria_2() {
            let digraph = kattis_escapewallmaria_2();

            assert!(digraph.is_isolated(0));
            assert!(digraph.is_isolated(1));
            assert!(digraph.is_isolated(2));
            assert!(digraph.is_isolated(3));
            assert!(digraph.is_isolated(4));
            assert!(!digraph.is_isolated(5));
            assert!(!digraph.is_isolated(6));
            assert!(digraph.is_isolated(7));
            assert!(digraph.is_isolated(8));
            assert!(!digraph.is_isolated(9));
            assert!(digraph.is_isolated(10));
            assert!(digraph.is_isolated(11));
            assert!(!digraph.is_isolated(12));
            assert!(!digraph.is_isolated(13));
            assert!(digraph.is_isolated(14));
            assert!(digraph.is_isolated(15));
        }

        #[test]
        fn is_isolated_kattis_escapewallmaria_3() {
            let digraph = kattis_escapewallmaria_3();

            assert!(digraph.is_isolated(0));
            assert!(!digraph.is_isolated(1));
            assert!(!digraph.is_isolated(2));
            assert!(digraph.is_isolated(3));
            assert!(digraph.is_isolated(4));
            assert!(!digraph.is_isolated(5));
            assert!(!digraph.is_isolated(6));
            assert!(digraph.is_isolated(7));
            assert!(digraph.is_isolated(8));
            assert!(!digraph.is_isolated(9));
            assert!(digraph.is_isolated(10));
            assert!(digraph.is_isolated(11));
            assert!(!digraph.is_isolated(12));
            assert!(!digraph.is_isolated(13));
            assert!(digraph.is_isolated(14));
            assert!(digraph.is_isolated(15));
        }

        #[test]
        fn is_oriented_bang_jensen_196() {
            assert!(!bang_jensen_196().is_oriented());
        }

        #[test]
        fn is_oriented_bang_jensen_34() {
            assert!(bang_jensen_34().is_oriented());
        }

        #[test]
        fn is_oriented_bang_jensen_94() {
            assert!(bang_jensen_94().is_oriented());
        }

        #[test]
        fn is_oriented_kattis_builddeps() {
            assert!(kattis_builddeps().is_oriented());
        }

        #[test]
        fn is_oriented_kattis_cantinaofbabel_1() {
            assert!(!kattis_cantinaofbabel_1().is_oriented());
        }

        #[test]
        fn is_oriented_kattis_cantinaofbabel_2() {
            assert!(!kattis_cantinaofbabel_2().is_oriented());
        }

        #[test]
        fn is_oriented_kattis_escapewallmaria_1() {
            assert!(!kattis_escapewallmaria_1().is_oriented());
        }

        #[test]
        fn is_oriented_kattis_escapewallmaria_2() {
            assert!(!kattis_escapewallmaria_2().is_oriented());
        }

        #[test]
        fn is_oriented_kattis_escapewallmaria_3() {
            assert!(!kattis_escapewallmaria_3().is_oriented());
        }

        #[test]
        fn is_pendant_bang_jensen_196() {
            let digraph = bang_jensen_196();

            assert!(!digraph.is_pendant(0));
            assert!(!digraph.is_pendant(1));
            assert!(!digraph.is_pendant(2));
            assert!(!digraph.is_pendant(3));
            assert!(!digraph.is_pendant(4));
            assert!(!digraph.is_pendant(5));
            assert!(!digraph.is_pendant(6));
            assert!(!digraph.is_pendant(7));
        }

        #[test]
        fn is_pendant_bang_jensen_34() {
            let digraph = bang_jensen_34();

            assert!(!digraph.is_pendant(0));
            assert!(!digraph.is_pendant(1));
            assert!(!digraph.is_pendant(2));
            assert!(digraph.is_pendant(3));
            assert!(!digraph.is_pendant(4));
            assert!(!digraph.is_pendant(5));
        }

        #[test]
        fn is_pendant_bang_jensen_94() {
            let digraph = bang_jensen_94();

            assert!(!digraph.is_pendant(0));
            assert!(!digraph.is_pendant(1));
            assert!(!digraph.is_pendant(2));
            assert!(!digraph.is_pendant(3));
            assert!(!digraph.is_pendant(4));
            assert!(!digraph.is_pendant(5));
            assert!(digraph.is_pendant(6));
        }

        #[test]
        fn is_pendant_kattis_builddeps() {
            let digraph = kattis_builddeps();

            assert!(!digraph.is_pendant(0));
            assert!(!digraph.is_pendant(1));
            assert!(!digraph.is_pendant(2));
            assert!(!digraph.is_pendant(3));
            assert!(!digraph.is_pendant(4));
            assert!(!digraph.is_pendant(5));
        }

        #[test]
        fn is_pendant_kattis_cantinaofbabel_1() {
            let digraph = kattis_cantinaofbabel_1();

            assert!(!digraph.is_pendant(0));
            assert!(!digraph.is_pendant(1));
            assert!(!digraph.is_pendant(2));
            assert!(!digraph.is_pendant(3));
            assert!(!digraph.is_pendant(4));
            assert!(!digraph.is_pendant(5));
            assert!(!digraph.is_pendant(6));
            assert!(!digraph.is_pendant(7));
            assert!(!digraph.is_pendant(8));
            assert!(!digraph.is_pendant(9));
            assert!(!digraph.is_pendant(10));
            assert!(!digraph.is_pendant(11));
        }

        #[test]
        fn is_pendant_kattis_cantinaofbabel_2() {
            let digraph = kattis_cantinaofbabel_2();

            assert!(!digraph.is_pendant(0));
            assert!(!digraph.is_pendant(1));
            assert!(!digraph.is_pendant(2));
            assert!(!digraph.is_pendant(3));
            assert!(!digraph.is_pendant(4));
            assert!(!digraph.is_pendant(5));
            assert!(!digraph.is_pendant(6));
            assert!(!digraph.is_pendant(7));
            assert!(!digraph.is_pendant(8));
            assert!(!digraph.is_pendant(9));
            assert!(!digraph.is_pendant(10));
            assert!(!digraph.is_pendant(11));
        }

        #[test]
        fn is_pendant_kattis_escapewallmaria_1() {
            let digraph = kattis_escapewallmaria_1();

            assert!(!digraph.is_pendant(0));
            assert!(!digraph.is_pendant(1));
            assert!(!digraph.is_pendant(2));
            assert!(!digraph.is_pendant(3));
            assert!(!digraph.is_pendant(4));
            assert!(!digraph.is_pendant(5));
            assert!(!digraph.is_pendant(6));
            assert!(!digraph.is_pendant(7));
            assert!(!digraph.is_pendant(8));
            assert!(!digraph.is_pendant(9));
            assert!(!digraph.is_pendant(10));
            assert!(!digraph.is_pendant(11));
            assert!(digraph.is_pendant(12));
            assert!(!digraph.is_pendant(13));
            assert!(!digraph.is_pendant(14));
            assert!(!digraph.is_pendant(15));
        }

        #[test]
        fn is_pendant_kattis_escapewallmaria_2() {
            let digraph = kattis_escapewallmaria_2();

            assert!(!digraph.is_pendant(0));
            assert!(!digraph.is_pendant(1));
            assert!(!digraph.is_pendant(2));
            assert!(!digraph.is_pendant(3));
            assert!(!digraph.is_pendant(4));
            assert!(!digraph.is_pendant(5));
            assert!(!digraph.is_pendant(6));
            assert!(!digraph.is_pendant(7));
            assert!(!digraph.is_pendant(8));
            assert!(!digraph.is_pendant(9));
            assert!(!digraph.is_pendant(10));
            assert!(!digraph.is_pendant(11));
            assert!(!digraph.is_pendant(12));
            assert!(!digraph.is_pendant(13));
            assert!(!digraph.is_pendant(14));
            assert!(!digraph.is_pendant(15));
        }

        #[test]
        fn is_pendant_kattis_escapewallmaria_3() {
            let digraph = kattis_escapewallmaria_3();

            assert!(!digraph.is_pendant(0));
            assert!(!digraph.is_pendant(1));
            assert!(!digraph.is_pendant(2));
            assert!(!digraph.is_pendant(3));
            assert!(!digraph.is_pendant(4));
            assert!(!digraph.is_pendant(5));
            assert!(!digraph.is_pendant(6));
            assert!(!digraph.is_pendant(7));
            assert!(!digraph.is_pendant(8));
            assert!(!digraph.is_pendant(9));
            assert!(!digraph.is_pendant(10));
            assert!(!digraph.is_pendant(11));
            assert!(!digraph.is_pendant(12));
            assert!(!digraph.is_pendant(13));
            assert!(!digraph.is_pendant(14));
            assert!(!digraph.is_pendant(15));
        }

        #[test]
        fn is_regular_bang_jensen_196() {
            assert!(!bang_jensen_196().is_regular());
        }

        #[test]
        fn is_regular_bang_jensen_34() {
            assert!(!bang_jensen_34().is_regular());
        }

        #[test]
        fn is_regular_bang_jensen_94() {
            assert!(!bang_jensen_94().is_regular());
        }

        #[test]
        fn is_regular_kattis_builddeps() {
            assert!(!kattis_builddeps().is_regular());
        }

        #[test]
        fn is_regular_kattis_cantinaofbabel_1() {
            assert!(!kattis_cantinaofbabel_1().is_regular());
        }

        #[test]
        fn is_regular_kattis_cantinaofbabel_2() {
            assert!(!kattis_cantinaofbabel_2().is_regular());
        }

        #[test]
        fn is_regular_kattis_escapewallmaria_1() {
            assert!(!kattis_escapewallmaria_1().is_regular());
        }

        #[test]
        fn is_regular_kattis_escapewallmaria_2() {
            assert!(!kattis_escapewallmaria_2().is_regular());
        }

        #[test]
        fn is_regular_kattis_escapewallmaria_3() {
            assert!(!kattis_escapewallmaria_3().is_regular());
        }

        #[test]
        fn is_semicomplete_bang_jensen_196() {
            assert!(!bang_jensen_196().is_semicomplete());
        }

        #[test]
        fn is_semicomplete_bang_jensen_34() {
            assert!(!bang_jensen_34().is_semicomplete());
        }

        #[test]
        fn is_semicomplete_bang_jensen_94() {
            assert!(!bang_jensen_94().is_semicomplete());
        }

        #[test]
        fn is_semicomplete_kattis_builddeps() {
            assert!(!kattis_builddeps().is_semicomplete());
        }

        #[test]
        fn is_semicomplete_kattis_cantinaofbabel_1() {
            assert!(!kattis_cantinaofbabel_1().is_semicomplete());
        }

        #[test]
        fn is_semicomplete_kattis_cantinaofbabel_2() {
            assert!(!kattis_cantinaofbabel_2().is_semicomplete());
        }

        #[test]
        fn is_semicomplete_kattis_escapewallmaria_1() {
            assert!(!kattis_escapewallmaria_1().is_semicomplete());
        }

        #[test]
        fn is_semicomplete_kattis_escapewallmaria_2() {
            assert!(!kattis_escapewallmaria_2().is_semicomplete());
        }

        #[test]
        fn is_semicomplete_kattis_escapewallmaria_3() {
            assert!(!kattis_escapewallmaria_3().is_semicomplete());
        }

        #[test]
        fn is_simple_bang_jensen_196() {
            assert!(bang_jensen_196().is_simple());
        }

        #[test]
        fn is_simple_bang_jensen_34() {
            assert!(bang_jensen_34().is_simple());
        }

        #[test]
        fn is_simple_bang_jensen_94() {
            assert!(bang_jensen_94().is_simple());
        }

        #[test]
        fn is_simple_kattis_builddeps() {
            assert!(kattis_builddeps().is_simple());
        }

        #[test]
        fn is_simple_kattis_cantinaofbabel_1() {
            assert!(kattis_cantinaofbabel_1().is_simple());
        }

        #[test]
        fn is_simple_kattis_cantinaofbabel_2() {
            assert!(kattis_cantinaofbabel_2().is_simple());
        }

        #[test]
        fn is_simple_kattis_escapewallmaria_1() {
            assert!(kattis_escapewallmaria_1().is_simple());
        }

        #[test]
        fn is_simple_kattis_escapewallmaria_2() {
            assert!(kattis_escapewallmaria_2().is_simple());
        }

        #[test]
        fn is_simple_kattis_escapewallmaria_3() {
            assert!(kattis_escapewallmaria_3().is_simple());
        }

        #[test]
        fn is_symmetric_bang_jensen_196() {
            assert!(!bang_jensen_196().is_symmetric());
        }

        #[test]
        fn is_symmetric_bang_jensen_34() {
            assert!(!bang_jensen_34().is_symmetric());
        }

        #[test]
        fn is_symmetric_bang_jensen_94() {
            assert!(!bang_jensen_94().is_symmetric());
        }

        #[test]
        fn is_symmetric_kattis_builddeps() {
            assert!(!kattis_builddeps().is_symmetric());
        }

        #[test]
        fn is_symmetric_kattis_cantinaofbabel_1() {
            assert!(!kattis_cantinaofbabel_1().is_symmetric());
        }

        #[test]
        fn is_symmetric_kattis_cantinaofbabel_2() {
            assert!(!kattis_cantinaofbabel_2().is_symmetric());
        }

        #[test]
        fn is_symmetric_kattis_escapewallmaria_1() {
            assert!(!kattis_escapewallmaria_1().is_symmetric());
        }

        #[test]
        fn is_symmetric_kattis_escapewallmaria_2() {
            assert!(!kattis_escapewallmaria_2().is_symmetric());
        }

        #[test]
        fn is_symmetric_kattis_escapewallmaria_3() {
            assert!(kattis_escapewallmaria_3().is_symmetric());
        }

        #[test]
        fn is_tournament_bang_jensen_196() {
            assert!(!bang_jensen_196().is_tournament());
        }

        #[test]
        fn is_tournament_bang_jensen_34() {
            assert!(!bang_jensen_34().is_tournament());
        }

        #[test]
        fn is_tournament_bang_jensen_94() {
            assert!(!bang_jensen_94().is_tournament());
        }

        #[test]
        fn is_tournament_kattis_builddeps() {
            assert!(!kattis_builddeps().is_tournament());
        }

        #[test]
        fn is_tournament_kattis_cantinaofbabel_1() {
            assert!(!kattis_cantinaofbabel_1().is_tournament());
        }

        #[test]
        fn is_tournament_kattis_cantinaofbabel_2() {
            assert!(!kattis_cantinaofbabel_2().is_tournament());
        }

        #[test]
        fn is_tournament_kattis_escapewallmaria_1() {
            assert!(!kattis_escapewallmaria_1().is_tournament());
        }

        #[test]
        fn is_tournament_kattis_escapewallmaria_2() {
            assert!(!kattis_escapewallmaria_2().is_tournament());
        }

        #[test]
        fn is_tournament_kattis_escapewallmaria_3() {
            assert!(!kattis_escapewallmaria_3().is_tournament());
        }

        #[test]
        fn has_walk_bang_jensen_34() {
            let digraph = bang_jensen_34();

            assert!(digraph.has_walk(&[0, 4]));
            assert!(digraph.has_walk(&[1, 0, 4]));
            assert!(digraph.has_walk(&[2, 1, 0, 4]));
            assert!(digraph.has_walk(&[2, 3]));
            assert!(digraph.has_walk(&[2, 5, 4]));
            assert!(digraph.has_walk(&[5, 4]));

            assert!(!digraph.has_walk(&[0, 1]));
            assert!(!digraph.has_walk(&[1, 2]));
            assert!(!digraph.has_walk(&[2, 0]));
            assert!(!digraph.has_walk(&[3]));
            assert!(!digraph.has_walk(&[4]));
            assert!(!digraph.has_walk(&[5, 0]));
        }

        #[test]
        fn has_walk_bang_jensen_94() {
            let digraph = bang_jensen_94();

            assert!(digraph.has_walk(&[0, 1, 3, 5]));
            assert!(digraph.has_walk(&[0, 2, 1, 3, 5]));
            assert!(digraph.has_walk(&[0, 2, 3, 5]));
            assert!(digraph.has_walk(&[0, 2, 4, 6]));
            assert!(digraph.has_walk(&[0, 2, 5]));
            assert!(digraph.has_walk(&[1, 3, 5]));
            assert!(digraph.has_walk(&[2, 1, 3, 5]));
            assert!(digraph.has_walk(&[2, 3, 5]));
            assert!(digraph.has_walk(&[2, 4, 6]));
            assert!(digraph.has_walk(&[2, 5]));
            assert!(digraph.has_walk(&[3, 5]));
            assert!(digraph.has_walk(&[4, 6]));

            assert!(!digraph.has_walk(&[0, 3]));
            assert!(!digraph.has_walk(&[1, 0]));
            assert!(!digraph.has_walk(&[2, 0]));
            assert!(!digraph.has_walk(&[3, 0]));
            assert!(!digraph.has_walk(&[4, 0]));
            assert!(!digraph.has_walk(&[5]));
            assert!(!digraph.has_walk(&[6]));
        }

        #[test]
        fn has_walk_kattis_builddeps() {
            let digraph = kattis_builddeps();

            assert!(digraph.has_walk(&[0, 3, 1]));
            assert!(digraph.has_walk(&[0, 4, 1]));
            assert!(digraph.has_walk(&[2, 3, 1]));
            assert!(digraph.has_walk(&[2, 4, 1]));
            assert!(digraph.has_walk(&[2, 5, 1]));
            assert!(digraph.has_walk(&[3, 1]));
            assert!(digraph.has_walk(&[4, 1]));
            assert!(digraph.has_walk(&[5, 1]));

            assert!(!digraph.has_walk(&[0, 1]));
            assert!(!digraph.has_walk(&[1]));
            assert!(!digraph.has_walk(&[2, 0]));
            assert!(!digraph.has_walk(&[3, 0]));
            assert!(!digraph.has_walk(&[4, 0]));
            assert!(!digraph.has_walk(&[5, 0]));
        }

        #[test]
        #[allow(clippy::cognitive_complexity)]
        fn has_walk_kattis_escapewallmaria_1() {
            let digraph = kattis_escapewallmaria_1();

            assert!(digraph.has_walk(&[5, 6, 5]));
            assert!(digraph.has_walk(&[5, 9, 5]));
            assert!(digraph.has_walk(&[5, 9, 13, 9]));
            assert!(digraph.has_walk(&[5, 9, 13, 12]));
            assert!(digraph.has_walk(&[6, 5, 6]));
            assert!(digraph.has_walk(&[6, 5, 9, 13, 9]));
            assert!(digraph.has_walk(&[6, 5, 9, 13, 12]));
            assert!(digraph.has_walk(&[9, 5, 6, 5]));
            assert!(digraph.has_walk(&[9, 5, 9]));
            assert!(digraph.has_walk(&[9, 13, 9]));
            assert!(digraph.has_walk(&[9, 13, 12]));
            assert!(digraph.has_walk(&[13, 9, 5, 6, 5]));
            assert!(digraph.has_walk(&[13, 9, 5, 9]));
            assert!(digraph.has_walk(&[13, 9, 13]));
            assert!(digraph.has_walk(&[13, 12]));

            assert!(!digraph.has_walk(&[0]));
            assert!(!digraph.has_walk(&[1]));
            assert!(!digraph.has_walk(&[2]));
            assert!(!digraph.has_walk(&[3]));
            assert!(!digraph.has_walk(&[4]));
            assert!(!digraph.has_walk(&[5, 0]));
            assert!(!digraph.has_walk(&[6, 0]));
            assert!(!digraph.has_walk(&[7]));
            assert!(!digraph.has_walk(&[8]));
            assert!(!digraph.has_walk(&[9, 0]));
            assert!(!digraph.has_walk(&[10]));
            assert!(!digraph.has_walk(&[11]));
            assert!(!digraph.has_walk(&[12]));
            assert!(!digraph.has_walk(&[13, 0]));
        }

        #[test]
        #[allow(clippy::cognitive_complexity)]
        fn has_walk_kattis_escapewallmaria_2() {
            let digraph = kattis_escapewallmaria_2();

            assert!(digraph.has_walk(&[5, 6, 5]));
            assert!(digraph.has_walk(&[5, 9, 5]));
            assert!(digraph.has_walk(&[6, 5, 6]));
            assert!(digraph.has_walk(&[6, 5, 9, 5]));
            assert!(digraph.has_walk(&[9, 5, 6, 5]));
            assert!(digraph.has_walk(&[9, 5, 9]));
            assert!(digraph.has_walk(&[12, 13, 9, 5, 6, 5]));
            assert!(digraph.has_walk(&[12, 13, 9, 5, 9]));
            assert!(digraph.has_walk(&[12, 13, 12]));
            assert!(digraph.has_walk(&[13, 9, 5, 6, 5]));
            assert!(digraph.has_walk(&[13, 9, 5, 9]));
            assert!(digraph.has_walk(&[13, 12, 13]));

            assert!(!digraph.has_walk(&[0]));
            assert!(!digraph.has_walk(&[1]));
            assert!(!digraph.has_walk(&[2]));
            assert!(!digraph.has_walk(&[3]));
            assert!(!digraph.has_walk(&[4]));
            assert!(!digraph.has_walk(&[5, 0]));
            assert!(!digraph.has_walk(&[6, 0]));
            assert!(!digraph.has_walk(&[7]));
            assert!(!digraph.has_walk(&[8]));
            assert!(!digraph.has_walk(&[9, 0]));
            assert!(!digraph.has_walk(&[10]));
            assert!(!digraph.has_walk(&[11]));
            assert!(!digraph.has_walk(&[12, 0]));
            assert!(!digraph.has_walk(&[13, 0]));
        }

        #[test]
        #[allow(clippy::cognitive_complexity)]
        fn has_walk_kattis_escapewallmaria_3() {
            let digraph = kattis_escapewallmaria_3();

            assert!(digraph.has_walk(&[1, 2, 1]));
            assert!(digraph.has_walk(&[1, 2, 6, 2]));
            assert!(digraph.has_walk(&[1, 2, 6, 5, 1]));
            assert!(digraph.has_walk(&[1, 2, 6, 5, 6]));
            assert!(digraph.has_walk(&[1, 2, 6, 5, 9, 5]));
            assert!(digraph.has_walk(&[1, 2, 6, 5, 9, 13]));
            assert!(digraph.has_walk(&[1, 2, 6, 5, 9, 13, 9]));
            assert!(digraph.has_walk(&[1, 2, 6, 5, 9, 13, 12, 13]));
            assert!(digraph.has_walk(&[1, 5, 1]));
            assert!(digraph.has_walk(&[1, 5, 6, 2, 1]));
            assert!(digraph.has_walk(&[1, 5, 6, 2, 6]));
            assert!(digraph.has_walk(&[1, 5, 9, 5]));
            assert!(digraph.has_walk(&[1, 5, 9, 13, 9]));
            assert!(digraph.has_walk(&[1, 5, 9, 13, 12, 13]));
            assert!(digraph.has_walk(&[2, 1, 2]));
            assert!(digraph.has_walk(&[2, 1, 5, 1]));
            assert!(digraph.has_walk(&[2, 1, 5, 6, 2]));
            assert!(digraph.has_walk(&[2, 1, 5, 6, 5]));
            assert!(digraph.has_walk(&[2, 1, 5, 9, 5]));
            assert!(digraph.has_walk(&[2, 1, 5, 9, 13, 9]));
            assert!(digraph.has_walk(&[2, 1, 5, 9, 13, 12, 13]));
            assert!(digraph.has_walk(&[2, 6, 2]));
            assert!(digraph.has_walk(&[2, 6, 5, 1, 2]));
            assert!(digraph.has_walk(&[2, 6, 5, 1, 5]));
            assert!(digraph.has_walk(&[2, 6, 5, 6]));
            assert!(digraph.has_walk(&[2, 6, 5, 9, 5]));
            assert!(digraph.has_walk(&[2, 6, 5, 9, 13, 9]));
            assert!(digraph.has_walk(&[2, 6, 5, 9, 13, 12, 13]));
            assert!(digraph.has_walk(&[5, 1, 2, 1]));
            assert!(digraph.has_walk(&[5, 1, 2, 6, 2]));
            assert!(digraph.has_walk(&[5, 1, 2, 6, 5]));
            assert!(digraph.has_walk(&[5, 1, 5]));
            assert!(digraph.has_walk(&[5, 6, 2, 1, 2]));
            assert!(digraph.has_walk(&[5, 6, 2, 1, 5]));
            assert!(digraph.has_walk(&[5, 6, 2, 6]));
            assert!(digraph.has_walk(&[5, 6, 5]));
            assert!(digraph.has_walk(&[5, 9, 5]));
            assert!(digraph.has_walk(&[5, 9, 13, 9]));
            assert!(digraph.has_walk(&[5, 9, 13, 12, 13]));
            assert!(digraph.has_walk(&[6, 2, 1, 2]));
            assert!(digraph.has_walk(&[6, 2, 1, 5, 1]));
            assert!(digraph.has_walk(&[6, 2, 1, 5, 6]));
            assert!(digraph.has_walk(&[6, 2, 1, 5, 9, 5]));
            assert!(digraph.has_walk(&[6, 2, 1, 5, 9, 13, 9]));
            assert!(digraph.has_walk(&[6, 2, 1, 5, 9, 13, 12, 13]));
            assert!(digraph.has_walk(&[6, 2, 6]));
            assert!(digraph.has_walk(&[6, 5, 1, 2, 1]));
            assert!(digraph.has_walk(&[6, 5, 1, 5]));
            assert!(digraph.has_walk(&[6, 5, 6]));
            assert!(digraph.has_walk(&[6, 5, 9, 5]));
            assert!(digraph.has_walk(&[6, 5, 9, 13, 9]));
            assert!(digraph.has_walk(&[6, 5, 9, 13, 12, 13]));
            assert!(digraph.has_walk(&[9, 5, 1, 2, 1]));
            assert!(digraph.has_walk(&[9, 5, 1, 2, 6, 2]));
            assert!(digraph.has_walk(&[9, 5, 1, 2, 6, 5]));
            assert!(digraph.has_walk(&[9, 5, 1, 5]));
            assert!(digraph.has_walk(&[9, 5, 6, 2, 1, 2]));
            assert!(digraph.has_walk(&[9, 5, 6, 2, 1, 5]));
            assert!(digraph.has_walk(&[9, 5, 6, 2, 6]));
            assert!(digraph.has_walk(&[9, 5, 6, 5]));
            assert!(digraph.has_walk(&[9, 5, 9]));
            assert!(digraph.has_walk(&[9, 13, 9]));
            assert!(digraph.has_walk(&[9, 13, 12, 13]));
            assert!(digraph.has_walk(&[12, 13, 9, 5, 1, 2, 1]));
            assert!(digraph.has_walk(&[12, 13, 9, 5, 1, 5]));
            assert!(digraph.has_walk(&[12, 13, 9, 5, 6, 2, 1, 2]));
            assert!(digraph.has_walk(&[12, 13, 9, 5, 6, 2, 1, 5]));
            assert!(digraph.has_walk(&[12, 13, 9, 5, 6, 2, 6]));
            assert!(digraph.has_walk(&[12, 13, 9, 5, 6, 5]));
            assert!(digraph.has_walk(&[12, 13, 9, 5, 9]));
            assert!(digraph.has_walk(&[12, 13, 9, 13]));
            assert!(digraph.has_walk(&[12, 13, 12]));

            assert!(!digraph.has_walk(&[0]));
            assert!(!digraph.has_walk(&[1, 0]));
            assert!(!digraph.has_walk(&[2, 0]));
            assert!(!digraph.has_walk(&[3]));
            assert!(!digraph.has_walk(&[4]));
            assert!(!digraph.has_walk(&[5, 0]));
            assert!(!digraph.has_walk(&[6, 0]));
            assert!(!digraph.has_walk(&[7]));
            assert!(!digraph.has_walk(&[8]));
            assert!(!digraph.has_walk(&[9, 0]));
            assert!(!digraph.has_walk(&[10]));
            assert!(!digraph.has_walk(&[11]));
            assert!(!digraph.has_walk(&[12, 0]));
            assert!(!digraph.has_walk(&[13, 0]));
        }

        #[test]
        fn max_degree_bang_jensen_196() {
            assert_eq!(bang_jensen_196().max_degree(), 4);
        }

        #[test]
        fn max_degree_bang_jensen_34() {
            assert_eq!(bang_jensen_34().max_degree(), 3);
        }

        #[test]
        fn max_degree_bang_jensen_94() {
            assert_eq!(bang_jensen_94().max_degree(), 5);
        }

        #[test]
        fn max_degree_kattis_builddeps() {
            assert_eq!(kattis_builddeps().max_degree(), 3);
        }

        #[test]
        fn max_degree_kattis_cantinaofbabel_1() {
            assert_eq!(kattis_cantinaofbabel_1().max_degree(), 8);
        }

        #[test]
        fn max_degree_kattis_cantinaofbabel_2() {
            assert_eq!(kattis_cantinaofbabel_2().max_degree(), 4);
        }

        #[test]
        fn max_degree_kattis_escapewallmaria_1() {
            assert_eq!(kattis_escapewallmaria_1().max_degree(), 4);
        }

        #[test]
        fn max_degree_kattis_escapewallmaria_2() {
            assert_eq!(kattis_escapewallmaria_2().max_degree(), 4);
        }

        #[test]
        fn max_degree_kattis_escapewallmaria_3() {
            assert_eq!(kattis_escapewallmaria_3().max_degree(), 6);
        }

        #[test]
        fn max_indegree_bang_jensen_196() {
            assert_eq!(bang_jensen_196().max_indegree(), 3);
        }

        #[test]
        fn max_indegree_bang_jensen_34() {
            assert_eq!(bang_jensen_34().max_indegree(), 2);
        }

        #[test]
        fn max_indegree_bang_jensen_94() {
            assert_eq!(bang_jensen_94().max_indegree(), 2);
        }

        #[test]
        fn max_indegree_kattis_builddeps() {
            assert_eq!(kattis_builddeps().max_indegree(), 3);
        }

        #[test]
        fn max_indegree_kattis_cantinaofbabel_1() {
            assert_eq!(kattis_cantinaofbabel_1().max_indegree(), 3);
        }

        #[test]
        fn max_indegree_kattis_cantinaofbabel_2() {
            assert_eq!(kattis_cantinaofbabel_2().max_indegree(), 3);
        }

        #[test]
        fn max_indegree_kattis_escapewallmaria_1() {
            assert_eq!(kattis_escapewallmaria_1().max_indegree(), 2);
        }

        #[test]
        fn max_indegree_kattis_escapewallmaria_2() {
            assert_eq!(kattis_escapewallmaria_2().max_indegree(), 2);
        }

        #[test]
        fn max_indegree_kattis_escapewallmaria_3() {
            assert_eq!(kattis_escapewallmaria_3().max_indegree(), 3);
        }

        #[test]
        fn max_outdegree_bang_jensen_196() {
            assert_eq!(bang_jensen_196().max_outdegree(), 3);
        }

        #[test]
        fn max_outdegree_bang_jensen_34() {
            assert_eq!(bang_jensen_34().max_outdegree(), 3);
        }

        #[test]
        fn max_outdegree_bang_jensen_94() {
            assert_eq!(bang_jensen_94().max_outdegree(), 4);
        }

        #[test]
        fn max_outdegree_kattis_builddeps() {
            assert_eq!(kattis_builddeps().max_outdegree(), 3);
        }

        #[test]
        fn max_outdegree_kattis_cantinaofbabel_1() {
            assert_eq!(kattis_cantinaofbabel_1().max_outdegree(), 6);
        }

        #[test]
        fn max_outdegree_kattis_cantinaofbabel_2() {
            assert_eq!(kattis_cantinaofbabel_2().max_outdegree(), 3);
        }

        #[test]
        fn max_outdegree_kattis_escapewallmaria_1() {
            assert_eq!(kattis_escapewallmaria_1().max_outdegree(), 2);
        }

        #[test]
        fn max_outdegree_kattis_escapewallmaria_2() {
            assert_eq!(kattis_escapewallmaria_2().max_outdegree(), 2);
        }

        #[test]
        fn max_outdegree_kattis_escapewallmaria_3() {
            assert_eq!(kattis_escapewallmaria_3().max_outdegree(), 3);
        }

        #[test]
        fn min_degree_bang_jensen_196() {
            assert_eq!(bang_jensen_196().min_degree(), 2);
        }

        #[test]
        fn min_degree_bang_jensen_34() {
            assert_eq!(bang_jensen_34().min_degree(), 1);
        }

        #[test]
        fn min_degree_bang_jensen_94() {
            assert_eq!(bang_jensen_94().min_degree(), 1);
        }

        #[test]
        fn min_degree_kattis_builddeps() {
            assert_eq!(kattis_builddeps().min_degree(), 2);
        }

        #[test]
        fn min_degree_kattis_cantinaofbabel_1() {
            assert_eq!(kattis_cantinaofbabel_1().min_degree(), 2);
        }

        #[test]
        fn min_degree_kattis_cantinaofbabel_2() {
            assert_eq!(kattis_cantinaofbabel_2().min_degree(), 2);
        }

        #[test]
        fn min_degree_kattis_escapewallmaria_1() {
            assert_eq!(kattis_escapewallmaria_1().min_degree(), 0);
        }

        #[test]
        fn min_degree_kattis_escapewallmaria_2() {
            assert_eq!(kattis_escapewallmaria_2().min_degree(), 0);
        }

        #[test]
        fn min_degree_kattis_escapewallmaria_3() {
            assert_eq!(kattis_escapewallmaria_3().min_degree(), 0);
        }

        #[test]
        fn min_indegree_bang_jensen_196() {
            assert_eq!(bang_jensen_196().min_indegree(), 1);
        }

        #[test]
        fn min_indegree_bang_jensen_34() {
            assert_eq!(bang_jensen_34().min_indegree(), 0);
        }

        #[test]
        fn min_indegree_bang_jensen_94() {
            assert_eq!(bang_jensen_94().min_indegree(), 0);
        }

        #[test]
        fn min_indegree_kattis_builddeps() {
            assert_eq!(kattis_builddeps().min_indegree(), 0);
        }

        #[test]
        fn min_indegree_kattis_cantinaofbabel_1() {
            assert_eq!(kattis_cantinaofbabel_1().min_indegree(), 0);
        }

        #[test]
        fn min_indegree_kattis_cantinaofbabel_2() {
            assert_eq!(kattis_cantinaofbabel_2().min_indegree(), 1);
        }

        #[test]
        fn min_indegree_kattis_escapewallmaria_1() {
            assert_eq!(kattis_escapewallmaria_1().min_indegree(), 0);
        }

        #[test]
        fn min_indegree_kattis_escapewallmaria_2() {
            assert_eq!(kattis_escapewallmaria_2().min_indegree(), 0);
        }

        #[test]
        fn min_indegree_kattis_escapewallmaria_3() {
            assert_eq!(kattis_escapewallmaria_3().min_indegree(), 0);
        }

        #[test]
        fn min_outdegree_bang_jensen_196() {
            assert_eq!(bang_jensen_196().min_outdegree(), 1);
        }

        #[test]
        fn min_outdegree_bang_jensen_34() {
            assert_eq!(bang_jensen_34().min_outdegree(), 0);
        }

        #[test]
        fn min_outdegree_bang_jensen_94() {
            assert_eq!(bang_jensen_94().min_outdegree(), 0);
        }

        #[test]
        fn min_outdegree_kattis_builddeps() {
            assert_eq!(kattis_builddeps().min_outdegree(), 0);
        }

        #[test]
        fn min_outdegree_kattis_cantinaofbabel_1() {
            assert_eq!(kattis_cantinaofbabel_1().min_outdegree(), 1);
        }

        #[test]
        fn min_outdegree_kattis_cantinaofbabel_2() {
            assert_eq!(kattis_cantinaofbabel_2().min_outdegree(), 1);
        }

        #[test]
        fn min_outdegree_kattis_escapewallmaria_1() {
            assert_eq!(kattis_escapewallmaria_1().min_outdegree(), 0);
        }

        #[test]
        fn min_outdegree_kattis_escapewallmaria_2() {
            assert_eq!(kattis_escapewallmaria_2().min_outdegree(), 0);
        }

        #[test]
        fn min_outdegree_kattis_escapewallmaria_3() {
            assert_eq!(kattis_escapewallmaria_3().min_outdegree(), 0);
        }

        #[test]
        fn order_bang_jensen_196() {
            assert_eq!(bang_jensen_196().order(), 8);
        }

        #[test]
        fn order_bang_jensen_34() {
            assert_eq!(bang_jensen_34().order(), 6);
        }

        #[test]
        fn order_bang_jensen_94() {
            assert_eq!(bang_jensen_94().order(), 7);
        }

        #[test]
        fn order_kattis_builddeps() {
            assert_eq!(kattis_builddeps().order(), 6);
        }

        #[test]
        fn order_kattis_cantinaofbabel_1() {
            assert_eq!(kattis_cantinaofbabel_1().order(), 12);
        }

        #[test]
        fn order_kattis_cantinaofbabel_2() {
            assert_eq!(kattis_cantinaofbabel_2().order(), 12);
        }

        #[test]
        fn order_kattis_escapewallmaria_1() {
            assert_eq!(kattis_escapewallmaria_1().order(), 16);
        }

        #[test]
        fn order_kattis_escapewallmaria_2() {
            assert_eq!(kattis_escapewallmaria_2().order(), 16);
        }

        #[test]
        fn order_kattis_escapewallmaria_3() {
            assert_eq!(kattis_escapewallmaria_3().order(), 16);
        }

        #[test]
        fn out_neighbors_bang_jensen_196() {
            let digraph = bang_jensen_196();

            assert!(digraph.out_neighbors(0).eq([1, 4, 7]));
            assert!(digraph.out_neighbors(1).eq([0, 2, 7]));
            assert!(digraph.out_neighbors(2).eq([3]));
            assert!(digraph.out_neighbors(3).eq([2, 4]));
            assert!(digraph.out_neighbors(4).eq([2]));
            assert!(digraph.out_neighbors(5).eq([6]));
            assert!(digraph.out_neighbors(6).eq([7]));
            assert!(digraph.out_neighbors(7).eq([5]));
        }

        #[test]
        fn out_neighbors_bang_jensen_34() {
            let digraph = bang_jensen_34();

            assert!(digraph.out_neighbors(0).eq([4]));
            assert!(digraph.out_neighbors(1).eq([0]));
            assert!(digraph.out_neighbors(2).eq([1, 3, 5]));
            assert!(digraph.out_neighbors(3).eq([]));
            assert!(digraph.out_neighbors(4).eq([]));
            assert!(digraph.out_neighbors(5).eq([4]));
        }

        #[test]
        fn out_neighbors_bang_jensen_94() {
            let digraph = bang_jensen_94();

            assert!(digraph.out_neighbors(0).eq([1, 2]));
            assert!(digraph.out_neighbors(1).eq([3]));
            assert!(digraph.out_neighbors(2).eq([1, 3, 4, 5]));
            assert!(digraph.out_neighbors(3).eq([5]));
            assert!(digraph.out_neighbors(4).eq([6]));
            assert!(digraph.out_neighbors(5).eq([]));
            assert!(digraph.out_neighbors(6).eq([]));
        }

        #[test]
        fn out_neighbors_kattis_builddeps() {
            let digraph = kattis_builddeps();

            assert!(digraph.out_neighbors(0).eq([3, 4]));
            assert!(digraph.out_neighbors(1).eq([]));
            assert!(digraph.out_neighbors(2).eq([3, 4, 5]));
            assert!(digraph.out_neighbors(3).eq([1]));
            assert!(digraph.out_neighbors(4).eq([1]));
            assert!(digraph.out_neighbors(5).eq([1]));
        }

        #[test]
        fn out_neighbors_kattis_cantinaofbabel_1() {
            let digraph = kattis_cantinaofbabel_1();

            assert!(digraph.out_neighbors(0).eq([1]));
            assert!(digraph.out_neighbors(1).eq([0, 2, 4]));
            assert!(digraph.out_neighbors(2).eq([1]));
            assert!(digraph.out_neighbors(3).eq([2, 4, 5, 7, 10, 11]));
            assert!(digraph.out_neighbors(4).eq([3]));
            assert!(digraph.out_neighbors(5).eq([6]));
            assert!(digraph.out_neighbors(6).eq([5, 10]));
            assert!(digraph.out_neighbors(7).eq([3]));
            assert!(digraph.out_neighbors(8).eq([7, 10]));
            assert!(digraph.out_neighbors(9).eq([7, 11]));
            assert!(digraph.out_neighbors(10).eq([6]));
            assert!(digraph.out_neighbors(11).eq([9]));
        }

        #[test]
        fn out_neighbors_kattis_cantinaofbabel_2() {
            let digraph = kattis_cantinaofbabel_2();

            assert!(digraph.out_neighbors(0).eq([1]));
            assert!(digraph.out_neighbors(1).eq([0, 7]));
            assert!(digraph.out_neighbors(2).eq([0, 5, 7]));
            assert!(digraph.out_neighbors(3).eq([4]));
            assert!(digraph.out_neighbors(4).eq([3]));
            assert!(digraph.out_neighbors(5).eq([3, 6]));
            assert!(digraph.out_neighbors(6).eq([5]));
            assert!(digraph.out_neighbors(7).eq([2]));
            assert!(digraph.out_neighbors(8).eq([7, 9, 11]));
            assert!(digraph.out_neighbors(9).eq([8]));
            assert!(digraph.out_neighbors(10).eq([9, 11]));
            assert!(digraph.out_neighbors(11).eq([10]));
        }

        #[test]
        fn out_neighbors_kattis_escapewallmaria_1() {
            let digraph = kattis_escapewallmaria_1();

            assert!(digraph.out_neighbors(0).eq([]));
            assert!(digraph.out_neighbors(1).eq([]));
            assert!(digraph.out_neighbors(2).eq([]));
            assert!(digraph.out_neighbors(3).eq([]));
            assert!(digraph.out_neighbors(4).eq([]));
            assert!(digraph.out_neighbors(5).eq([6, 9]));
            assert!(digraph.out_neighbors(6).eq([5]));
            assert!(digraph.out_neighbors(7).eq([]));
            assert!(digraph.out_neighbors(8).eq([]));
            assert!(digraph.out_neighbors(9).eq([5, 13]));
            assert!(digraph.out_neighbors(10).eq([]));
            assert!(digraph.out_neighbors(11).eq([]));
            assert!(digraph.out_neighbors(12).eq([]));
            assert!(digraph.out_neighbors(13).eq([9, 12]));
        }

        #[test]
        fn out_neighbors_kattis_escapewallmaria_2() {
            let digraph = kattis_escapewallmaria_2();

            assert!(digraph.out_neighbors(0).eq([]));
            assert!(digraph.out_neighbors(1).eq([]));
            assert!(digraph.out_neighbors(2).eq([]));
            assert!(digraph.out_neighbors(3).eq([]));
            assert!(digraph.out_neighbors(4).eq([]));
            assert!(digraph.out_neighbors(5).eq([6, 9]));
            assert!(digraph.out_neighbors(6).eq([5]));
            assert!(digraph.out_neighbors(7).eq([]));
            assert!(digraph.out_neighbors(8).eq([]));
            assert!(digraph.out_neighbors(9).eq([5]));
            assert!(digraph.out_neighbors(10).eq([]));
            assert!(digraph.out_neighbors(11).eq([]));
            assert!(digraph.out_neighbors(12).eq([13]));
            assert!(digraph.out_neighbors(13).eq([9, 12]));
        }

        #[test]
        fn out_neighbors_kattis_escapewallmaria_3() {
            let digraph = kattis_escapewallmaria_3();

            assert!(digraph.out_neighbors(0).eq([]));
            assert!(digraph.out_neighbors(1).eq([2, 5]));
            assert!(digraph.out_neighbors(2).eq([1, 6]));
            assert!(digraph.out_neighbors(3).eq([]));
            assert!(digraph.out_neighbors(4).eq([]));
            assert!(digraph.out_neighbors(5).eq([1, 6, 9]));
            assert!(digraph.out_neighbors(6).eq([2, 5]));
            assert!(digraph.out_neighbors(7).eq([]));
            assert!(digraph.out_neighbors(8).eq([]));
            assert!(digraph.out_neighbors(9).eq([5, 13]));
            assert!(digraph.out_neighbors(10).eq([]));
            assert!(digraph.out_neighbors(11).eq([]));
            assert!(digraph.out_neighbors(12).eq([13]));
            assert!(digraph.out_neighbors(13).eq([9, 12]));
        }

        #[test]
        #[should_panic(expected = "u = 1 isn't in the digraph")]
        fn out_neighbors_out_of_bounds() {
            let _ = $type::trivial().out_neighbors(1);
        }

        #[test]
        fn out_neighbors_weighted_bang_jensen_196() {
            let digraph = bang_jensen_196();

            assert!(digraph.out_neighbors_weighted(0).eq([
                (1, &1),
                (4, &1),
                (7, &1)
            ]));

            assert!(digraph.out_neighbors_weighted(1).eq([
                (0, &1),
                (2, &1),
                (7, &1)
            ]));

            assert!(digraph.out_neighbors_weighted(2).eq([(3, &1)]));
            assert!(digraph.out_neighbors_weighted(3).eq([(2, &1), (4, &1)]));
            assert!(digraph.out_neighbors_weighted(4).eq([(2, &1)]));
            assert!(digraph.out_neighbors_weighted(5).eq([(6, &1)]));
            assert!(digraph.out_neighbors_weighted(6).eq([(7, &1)]));
            assert!(digraph.out_neighbors_weighted(7).eq([(5, &1)]));
        }

        #[test]
        fn out_neighbors_weighted_bang_jensen_34() {
            let digraph = bang_jensen_34();

            assert!(digraph.out_neighbors_weighted(0).eq([(4, &1)]));
            assert!(digraph.out_neighbors_weighted(1).eq([(0, &1)]));

            assert!(digraph.out_neighbors_weighted(2).eq([
                (1, &1),
                (3, &1),
                (5, &1)
            ]));

            assert!(digraph.out_neighbors_weighted(3).eq([]));
            assert!(digraph.out_neighbors_weighted(4).eq([]));
            assert!(digraph.out_neighbors_weighted(5).eq([(4, &1)]));
        }

        #[test]
        fn out_neighbors_weighted_bang_jensen_94() {
            let digraph = bang_jensen_94();

            assert!(digraph.out_neighbors_weighted(0).eq([(1, &1), (2, &1)]));
            assert!(digraph.out_neighbors_weighted(1).eq([(3, &1)]));

            assert!(digraph.out_neighbors_weighted(2).eq([
                (1, &1),
                (3, &1),
                (4, &1),
                (5, &1)
            ]));

            assert!(digraph.out_neighbors_weighted(3).eq([(5, &1)]));
            assert!(digraph.out_neighbors_weighted(4).eq([(6, &1)]));
            assert!(digraph.out_neighbors_weighted(5).eq([]));
            assert!(digraph.out_neighbors_weighted(6).eq([]));
        }

        #[test]
        fn out_neighbors_weighted_kattis_builddeps() {
            let digraph = kattis_builddeps();

            assert!(digraph.out_neighbors_weighted(0).eq([(3, &1), (4, &1)]));
            assert!(digraph.out_neighbors_weighted(1).eq([]));

            assert!(digraph.out_neighbors_weighted(2).eq([
                (3, &1),
                (4, &1),
                (5, &1)
            ]));

            assert!(digraph.out_neighbors_weighted(3).eq([(1, &1)]));
            assert!(digraph.out_neighbors_weighted(4).eq([(1, &1)]));
            assert!(digraph.out_neighbors_weighted(5).eq([(1, &1)]));
        }

        #[test]
        fn out_neighbors_weighted_kattis_cantinaofbabel_1() {
            let digraph = kattis_cantinaofbabel_1();

            assert!(digraph.out_neighbors_weighted(0).eq([(1, &1)]));

            assert!(digraph.out_neighbors_weighted(1).eq([
                (0, &1),
                (2, &1),
                (4, &1)
            ]));

            assert!(digraph.out_neighbors_weighted(2).eq([(1, &1)]));

            assert!(digraph.out_neighbors_weighted(3).eq([
                (2, &1),
                (4, &1),
                (5, &1),
                (7, &1),
                (10, &1),
                (11, &1)
            ]));

            assert!(digraph.out_neighbors_weighted(4).eq([(3, &1)]));
            assert!(digraph.out_neighbors_weighted(5).eq([(6, &1)]));
            assert!(digraph.out_neighbors_weighted(6).eq([(5, &1), (10, &1)]));
            assert!(digraph.out_neighbors_weighted(7).eq([(3, &1)]));
            assert!(digraph.out_neighbors_weighted(8).eq([(7, &1), (10, &1)]));
            assert!(digraph.out_neighbors_weighted(9).eq([(7, &1), (11, &1)]));
            assert!(digraph.out_neighbors_weighted(10).eq([(6, &1)]));
            assert!(digraph.out_neighbors_weighted(11).eq([(9, &1)]));
        }

        #[test]
        fn out_neighbors_weighted_kattis_cantinaofbabel_2() {
            let digraph = kattis_cantinaofbabel_2();

            assert!(digraph.out_neighbors_weighted(0).eq([(1, &1)]));
            assert!(digraph.out_neighbors_weighted(1).eq([(0, &1), (7, &1)]));

            assert!(digraph.out_neighbors_weighted(2).eq([
                (0, &1),
                (5, &1),
                (7, &1)
            ]));

            assert!(digraph.out_neighbors_weighted(3).eq([(4, &1)]));
            assert!(digraph.out_neighbors_weighted(4).eq([(3, &1)]));
            assert!(digraph.out_neighbors_weighted(5).eq([(3, &1), (6, &1)]));
            assert!(digraph.out_neighbors_weighted(6).eq([(5, &1)]));
            assert!(digraph.out_neighbors_weighted(7).eq([(2, &1)]));

            assert!(digraph.out_neighbors_weighted(8).eq([
                (7, &1),
                (9, &1),
                (11, &1)
            ]));

            assert!(digraph.out_neighbors_weighted(9).eq([(8, &1)]));

            assert!(digraph
                .out_neighbors_weighted(10)
                .eq([(9, &1), (11, &1)]));

            assert!(digraph.out_neighbors_weighted(11).eq([(10, &1)]));
        }

        #[test]
        fn out_neighbors_weighted_kattis_escapewallmaria_1() {
            let digraph = kattis_escapewallmaria_1();

            assert!(digraph.out_neighbors_weighted(0).eq([]));
            assert!(digraph.out_neighbors_weighted(1).eq([]));
            assert!(digraph.out_neighbors_weighted(2).eq([]));
            assert!(digraph.out_neighbors_weighted(3).eq([]));
            assert!(digraph.out_neighbors_weighted(4).eq([]));

            assert!(digraph.out_neighbors_weighted(5).eq([(6, &1), (9, &1)]));
            assert!(digraph.out_neighbors_weighted(6).eq([(5, &1)]));
            assert!(digraph.out_neighbors_weighted(7).eq([]));
            assert!(digraph.out_neighbors_weighted(8).eq([]));
            assert!(digraph.out_neighbors_weighted(9).eq([(5, &1), (13, &1)]));
            assert!(digraph.out_neighbors_weighted(10).eq([]));
            assert!(digraph.out_neighbors_weighted(11).eq([]));
            assert!(digraph.out_neighbors_weighted(12).eq([]));

            assert!(digraph
                .out_neighbors_weighted(13)
                .eq([(9, &1), (12, &1)]));
        }

        #[test]
        fn out_neighbors_weighted_kattis_escapewallmaria_2() {
            let digraph = kattis_escapewallmaria_2();

            assert!(digraph.out_neighbors_weighted(0).eq([]));
            assert!(digraph.out_neighbors_weighted(1).eq([]));
            assert!(digraph.out_neighbors_weighted(2).eq([]));
            assert!(digraph.out_neighbors_weighted(3).eq([]));
            assert!(digraph.out_neighbors_weighted(4).eq([]));
            assert!(digraph.out_neighbors_weighted(5).eq([(6, &1), (9, &1)]));
            assert!(digraph.out_neighbors_weighted(6).eq([(5, &1)]));
            assert!(digraph.out_neighbors_weighted(7).eq([]));
            assert!(digraph.out_neighbors_weighted(8).eq([]));
            assert!(digraph.out_neighbors_weighted(9).eq([(5, &1)]));
            assert!(digraph.out_neighbors_weighted(10).eq([]));
            assert!(digraph.out_neighbors_weighted(11).eq([]));
            assert!(digraph.out_neighbors_weighted(12).eq([(13, &1)]));

            assert!(digraph
                .out_neighbors_weighted(13)
                .eq([(9, &1), (12, &1)]));
        }

        #[test]
        fn out_neighbors_weighted_kattis_escapewallmaria_3() {
            let digraph = kattis_escapewallmaria_3();

            assert!(digraph.out_neighbors_weighted(0).eq([]));
            assert!(digraph.out_neighbors_weighted(1).eq([(2, &1), (5, &1)]));
            assert!(digraph.out_neighbors_weighted(2).eq([(1, &1), (6, &1)]));
            assert!(digraph.out_neighbors_weighted(3).eq([]));
            assert!(digraph.out_neighbors_weighted(4).eq([]));

            assert!(digraph.out_neighbors_weighted(5).eq([
                (1, &1),
                (6, &1),
                (9, &1)
            ]));

            assert!(digraph.out_neighbors_weighted(6).eq([(2, &1), (5, &1)]));
            assert!(digraph.out_neighbors_weighted(7).eq([]));
            assert!(digraph.out_neighbors_weighted(8).eq([]));
            assert!(digraph.out_neighbors_weighted(9).eq([(5, &1), (13, &1)]));
            assert!(digraph.out_neighbors_weighted(10).eq([]));
            assert!(digraph.out_neighbors_weighted(11).eq([]));
            assert!(digraph.out_neighbors_weighted(12).eq([(13, &1)]));

            assert!(digraph
                .out_neighbors_weighted(13)
                .eq([(9, &1), (12, &1)]));
        }

        #[test]
        #[should_panic(expected = "u = 1 isn't in the digraph")]
        fn out_neighbors_weighted_out_of_bounds() {
            let _ = $type::trivial().out_neighbors_weighted(1);
        }

        #[test]
        fn outdegree_bang_jensen_196() {
            let digraph = bang_jensen_196();

            assert_eq!(digraph.outdegree(0), 3);
            assert_eq!(digraph.outdegree(1), 3);
            assert_eq!(digraph.outdegree(2), 1);
            assert_eq!(digraph.outdegree(3), 2);
            assert_eq!(digraph.outdegree(4), 1);
            assert_eq!(digraph.outdegree(5), 1);
            assert_eq!(digraph.outdegree(6), 1);
            assert_eq!(digraph.outdegree(7), 1);
        }

        #[test]
        fn outdegree_bang_jensen_34() {
            let digraph = bang_jensen_34();

            assert_eq!(digraph.outdegree(0), 1);
            assert_eq!(digraph.outdegree(1), 1);
            assert_eq!(digraph.outdegree(2), 3);
            assert_eq!(digraph.outdegree(3), 0);
            assert_eq!(digraph.outdegree(4), 0);
            assert_eq!(digraph.outdegree(5), 1);
        }

        #[test]
        fn outdegree_bang_jensen_94() {
            let digraph = bang_jensen_94();

            assert_eq!(digraph.outdegree(0), 2);
            assert_eq!(digraph.outdegree(1), 1);
            assert_eq!(digraph.outdegree(2), 4);
            assert_eq!(digraph.outdegree(3), 1);
            assert_eq!(digraph.outdegree(4), 1);
            assert_eq!(digraph.outdegree(5), 0);
            assert_eq!(digraph.outdegree(6), 0);
        }

        #[test]
        fn outdegree_kattis_builddeps() {
            let digraph = kattis_builddeps();

            assert_eq!(digraph.outdegree(0), 2);
            assert_eq!(digraph.outdegree(1), 0);
            assert_eq!(digraph.outdegree(2), 3);
            assert_eq!(digraph.outdegree(3), 1);
            assert_eq!(digraph.outdegree(4), 1);
            assert_eq!(digraph.outdegree(5), 1);
        }

        #[test]
        fn outdegree_kattis_cantinaofbabel_1() {
            let digraph = kattis_cantinaofbabel_1();

            assert_eq!(digraph.outdegree(0), 1);
            assert_eq!(digraph.outdegree(1), 3);
            assert_eq!(digraph.outdegree(2), 1);
            assert_eq!(digraph.outdegree(3), 6);
            assert_eq!(digraph.outdegree(4), 1);
            assert_eq!(digraph.outdegree(5), 1);
            assert_eq!(digraph.outdegree(6), 2);
            assert_eq!(digraph.outdegree(7), 1);
            assert_eq!(digraph.outdegree(8), 2);
            assert_eq!(digraph.outdegree(9), 2);
            assert_eq!(digraph.outdegree(10), 1);
            assert_eq!(digraph.outdegree(11), 1);
        }

        #[test]
        fn outdegree_kattis_cantinaofbabel_2() {
            let digraph = kattis_cantinaofbabel_2();

            assert_eq!(digraph.outdegree(0), 1);
            assert_eq!(digraph.outdegree(1), 2);
            assert_eq!(digraph.outdegree(2), 3);
            assert_eq!(digraph.outdegree(3), 1);
            assert_eq!(digraph.outdegree(4), 1);
            assert_eq!(digraph.outdegree(5), 2);
            assert_eq!(digraph.outdegree(6), 1);
            assert_eq!(digraph.outdegree(7), 1);
            assert_eq!(digraph.outdegree(8), 3);
            assert_eq!(digraph.outdegree(9), 1);
            assert_eq!(digraph.outdegree(10), 2);
            assert_eq!(digraph.outdegree(11), 1);
        }

        #[test]
        fn outdegree_kattis_escapewallmaria_1() {
            let digraph = kattis_escapewallmaria_1();

            assert_eq!(digraph.outdegree(0), 0);
            assert_eq!(digraph.outdegree(1), 0);
            assert_eq!(digraph.outdegree(2), 0);
            assert_eq!(digraph.outdegree(3), 0);
            assert_eq!(digraph.outdegree(4), 0);
            assert_eq!(digraph.outdegree(5), 2);
            assert_eq!(digraph.outdegree(6), 1);
            assert_eq!(digraph.outdegree(7), 0);
            assert_eq!(digraph.outdegree(8), 0);
            assert_eq!(digraph.outdegree(9), 2);
            assert_eq!(digraph.outdegree(10), 0);
            assert_eq!(digraph.outdegree(11), 0);
            assert_eq!(digraph.outdegree(12), 0);
            assert_eq!(digraph.outdegree(13), 2);
            assert_eq!(digraph.outdegree(14), 0);
            assert_eq!(digraph.outdegree(15), 0);
        }

        #[test]
        fn outdegree_kattis_escapewallmaria_2() {
            let digraph = kattis_escapewallmaria_2();

            assert_eq!(digraph.outdegree(0), 0);
            assert_eq!(digraph.outdegree(1), 0);
            assert_eq!(digraph.outdegree(2), 0);
            assert_eq!(digraph.outdegree(3), 0);
            assert_eq!(digraph.outdegree(4), 0);
            assert_eq!(digraph.outdegree(5), 2);
            assert_eq!(digraph.outdegree(6), 1);
            assert_eq!(digraph.outdegree(7), 0);
            assert_eq!(digraph.outdegree(8), 0);
            assert_eq!(digraph.outdegree(9), 1);
            assert_eq!(digraph.outdegree(10), 0);
            assert_eq!(digraph.outdegree(11), 0);
            assert_eq!(digraph.outdegree(12), 1);
            assert_eq!(digraph.outdegree(13), 2);
            assert_eq!(digraph.outdegree(14), 0);
            assert_eq!(digraph.outdegree(15), 0);
        }

        #[test]
        fn outdegree_kattis_escapewallmaria_3() {
            let digraph = kattis_escapewallmaria_3();

            assert_eq!(digraph.outdegree(0), 0);
            assert_eq!(digraph.outdegree(1), 2);
            assert_eq!(digraph.outdegree(2), 2);
            assert_eq!(digraph.outdegree(3), 0);
            assert_eq!(digraph.outdegree(4), 0);
            assert_eq!(digraph.outdegree(5), 3);
            assert_eq!(digraph.outdegree(6), 2);
            assert_eq!(digraph.outdegree(7), 0);
            assert_eq!(digraph.outdegree(8), 0);
            assert_eq!(digraph.outdegree(9), 2);
            assert_eq!(digraph.outdegree(10), 0);
            assert_eq!(digraph.outdegree(11), 0);
            assert_eq!(digraph.outdegree(12), 1);
            assert_eq!(digraph.outdegree(13), 2);
            assert_eq!(digraph.outdegree(14), 0);
            assert_eq!(digraph.outdegree(15), 0);
        }

        #[test]
        fn outdegree_sequence_bang_jensen_196() {
            assert!(bang_jensen_196()
                .outdegree_sequence()
                .eq([3, 3, 1, 2, 1, 1, 1, 1]));
        }

        #[test]
        fn outdegree_sequence_bang_jensen_34() {
            assert!(bang_jensen_34()
                .outdegree_sequence()
                .eq([1, 1, 3, 0, 0, 1]));
        }

        #[test]
        fn outdegree_sequence_bang_jensen_94() {
            assert!(bang_jensen_94()
                .outdegree_sequence()
                .eq([2, 1, 4, 1, 1, 0, 0]));
        }

        #[test]
        fn outdegree_sequence_kattis_builddeps() {
            assert!(kattis_builddeps()
                .outdegree_sequence()
                .eq([2, 0, 3, 1, 1, 1]));
        }

        #[test]
        fn outdegree_sequence_kattis_cantinaofbabel_1() {
            assert!(kattis_cantinaofbabel_1()
                .outdegree_sequence()
                .eq([1, 3, 1, 6, 1, 1, 2, 1, 2, 2, 1, 1]));
        }

        #[test]
        fn outdegree_sequence_kattis_cantinaofbabel_2() {
            assert!(kattis_cantinaofbabel_2()
                .outdegree_sequence()
                .eq([1, 2, 3, 1, 1, 2, 1, 1, 3, 1, 2, 1]));
        }

        #[test]
        fn outdegree_sequence_kattis_escapewallmaria_1() {
            assert!(kattis_escapewallmaria_1()
                .outdegree_sequence()
                .eq([0, 0, 0, 0, 0, 2, 1, 0, 0, 2, 0, 0, 0, 2, 0, 0]));
        }

        #[test]
        fn outdegree_sequence_kattis_escapewallmaria_2() {
            assert!(kattis_escapewallmaria_2()
                .outdegree_sequence()
                .eq([0, 0, 0, 0, 0, 2, 1, 0, 0, 1, 0, 0, 1, 2, 0, 0]));
        }

        #[test]
        fn outdegree_sequence_kattis_escapewallmaria_3() {
            assert!(kattis_escapewallmaria_3()
                .outdegree_sequence()
                .eq([0, 2, 2, 0, 0, 3, 2, 0, 0, 2, 0, 0, 1, 2, 0, 0]));
        }

        #[test]
        #[should_panic(expected = "u = 1 isn't in the digraph")]
        fn outdegree_out_of_bounds() {
            let _ = $type::trivial().outdegree(1);
        }

        #[test]
        #[should_panic(expected = "a digraph has at least one vertex")]
        fn path_0() {
            let _ = $type::path(0);
        }

        #[test]
        fn path_1() {
            let digraph = $type::path(1);

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn path_1_complement() {
            let digraph = $type::path(1).complement();

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn path_2() {
            let digraph = $type::path(2);

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([(0, 1)]));
        }

        #[test]
        fn path_2_complement() {
            let digraph = $type::path(2).complement();

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([(1, 0)]));
        }

        #[test]
        fn path_3() {
            let digraph = $type::path(3);

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([(0, 1), (1, 2)]));
        }

        #[test]
        fn path_3_complement() {
            let digraph = $type::path(3).complement();

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([(0, 2), (1, 0), (2, 0), (2, 1)]));
        }

        #[test]
        #[should_panic(expected = "a digraph has at least one vertex")]
        fn random_tournament_0() {
            let _ = $type::random_tournament(0, 0);
        }

        #[test]
        #[allow(clippy::cognitive_complexity)]
        fn remove_arc_bang_jensen_196() {
            let mut digraph = bang_jensen_196();

            assert!(digraph.arcs().eq([
                (0, 1),
                (0, 4),
                (0, 7),
                (1, 0),
                (1, 2),
                (1, 7),
                (2, 3),
                (3, 2),
                (3, 4),
                (4, 2),
                (5, 6),
                (6, 7),
                (7, 5)
            ]));

            assert!(!digraph.remove_arc(8, 0));
            assert!(!digraph.remove_arc(0, 3));

            assert!(digraph.remove_arc(0, 1));
            assert!(digraph.remove_arc(0, 4));
            assert!(digraph.remove_arc(0, 7));
            assert!(digraph.remove_arc(1, 0));
            assert!(digraph.remove_arc(1, 2));
            assert!(digraph.remove_arc(1, 7));
            assert!(digraph.remove_arc(2, 3));
            assert!(digraph.remove_arc(3, 2));
            assert!(digraph.remove_arc(3, 4));
            assert!(digraph.remove_arc(4, 2));
            assert!(digraph.remove_arc(5, 6));
            assert!(digraph.remove_arc(6, 7));
            assert!(digraph.remove_arc(7, 5));

            assert!(digraph.arcs().eq([]));

            assert!(!digraph.remove_arc(0, 1));
            assert!(!digraph.remove_arc(0, 4));
            assert!(!digraph.remove_arc(0, 7));
            assert!(!digraph.remove_arc(1, 0));
            assert!(!digraph.remove_arc(1, 2));
            assert!(!digraph.remove_arc(1, 7));
            assert!(!digraph.remove_arc(2, 3));
            assert!(!digraph.remove_arc(3, 2));
            assert!(!digraph.remove_arc(3, 4));
            assert!(!digraph.remove_arc(4, 2));
            assert!(!digraph.remove_arc(5, 6));
            assert!(!digraph.remove_arc(6, 7));
            assert!(!digraph.remove_arc(7, 5));
        }

        #[test]
        fn remove_arc_bang_jensen_34() {
            let mut digraph = bang_jensen_34();

            assert!(digraph.arcs().eq([
                (0, 4),
                (1, 0),
                (2, 1),
                (2, 3),
                (2, 5),
                (5, 4)
            ]));

            assert!(!digraph.remove_arc(0, 1));

            assert!(digraph.remove_arc(0, 4));
            assert!(digraph.remove_arc(1, 0));
            assert!(digraph.remove_arc(2, 1));
            assert!(digraph.remove_arc(2, 3));
            assert!(digraph.remove_arc(2, 5));
            assert!(digraph.remove_arc(5, 4));

            assert!(digraph.arcs().eq([]));

            assert!(!digraph.remove_arc(0, 4));
            assert!(!digraph.remove_arc(1, 0));
            assert!(!digraph.remove_arc(2, 1));
            assert!(!digraph.remove_arc(2, 3));
            assert!(!digraph.remove_arc(2, 5));
            assert!(!digraph.remove_arc(5, 4));
        }

        #[test]
        fn remove_arc_bang_jensen_94() {
            let mut digraph = bang_jensen_94();

            assert!(digraph.arcs().eq([
                (0, 1),
                (0, 2),
                (1, 3),
                (2, 1),
                (2, 3),
                (2, 4),
                (2, 5),
                (3, 5),
                (4, 6)
            ]));

            assert!(!digraph.remove_arc(0, 3));

            assert!(digraph.remove_arc(0, 1));
            assert!(digraph.remove_arc(0, 2));
            assert!(digraph.remove_arc(1, 3));
            assert!(digraph.remove_arc(2, 1));
            assert!(digraph.remove_arc(2, 3));
            assert!(digraph.remove_arc(2, 4));
            assert!(digraph.remove_arc(2, 5));
            assert!(digraph.remove_arc(3, 5));
            assert!(digraph.remove_arc(4, 6));

            assert!(digraph.arcs().eq([]));

            assert!(!digraph.remove_arc(0, 1));
            assert!(!digraph.remove_arc(0, 2));
            assert!(!digraph.remove_arc(1, 3));
            assert!(!digraph.remove_arc(2, 1));
            assert!(!digraph.remove_arc(2, 3));
            assert!(!digraph.remove_arc(2, 4));
            assert!(!digraph.remove_arc(2, 5));
            assert!(!digraph.remove_arc(3, 5));
            assert!(!digraph.remove_arc(4, 6));
        }

        #[test]
        fn remove_arc_kattis_builddeps() {
            let mut digraph = kattis_builddeps();

            assert!(digraph.arcs().eq([
                (0, 3),
                (0, 4),
                (2, 3),
                (2, 4),
                (2, 5),
                (3, 1),
                (4, 1),
                (5, 1)
            ]));

            assert!(!digraph.remove_arc(0, 1));

            assert!(digraph.remove_arc(0, 3));
            assert!(digraph.remove_arc(0, 4));
            assert!(digraph.remove_arc(2, 3));
            assert!(digraph.remove_arc(2, 4));
            assert!(digraph.remove_arc(2, 5));
            assert!(digraph.remove_arc(3, 1));
            assert!(digraph.remove_arc(4, 1));
            assert!(digraph.remove_arc(5, 1));

            assert!(digraph.arcs().eq([]));

            assert!(!digraph.remove_arc(0, 3));
            assert!(!digraph.remove_arc(0, 4));
            assert!(!digraph.remove_arc(2, 3));
            assert!(!digraph.remove_arc(2, 4));
            assert!(!digraph.remove_arc(2, 5));
            assert!(!digraph.remove_arc(3, 1));
            assert!(!digraph.remove_arc(4, 1));
            assert!(!digraph.remove_arc(5, 1));
        }

        #[test]
        #[allow(clippy::cognitive_complexity)]
        fn remove_arc_kattis_cantinaofbabel_1() {
            let mut digraph = kattis_cantinaofbabel_1();

            assert!(digraph.arcs().eq([
                (0, 1),
                (1, 0),
                (1, 2),
                (1, 4),
                (2, 1),
                (3, 2),
                (3, 4),
                (3, 5),
                (3, 7),
                (3, 10),
                (3, 11),
                (4, 3),
                (5, 6),
                (6, 5),
                (6, 10),
                (7, 3),
                (8, 7),
                (8, 10),
                (9, 7),
                (9, 11),
                (10, 6),
                (11, 9)
            ]));

            assert!(!digraph.remove_arc(0, 3));

            assert!(digraph.remove_arc(0, 1));
            assert!(digraph.remove_arc(1, 0));
            assert!(digraph.remove_arc(1, 2));
            assert!(digraph.remove_arc(1, 4));
            assert!(digraph.remove_arc(2, 1));
            assert!(digraph.remove_arc(3, 2));
            assert!(digraph.remove_arc(3, 4));
            assert!(digraph.remove_arc(3, 5));
            assert!(digraph.remove_arc(3, 7));
            assert!(digraph.remove_arc(3, 10));
            assert!(digraph.remove_arc(3, 11));
            assert!(digraph.remove_arc(4, 3));
            assert!(digraph.remove_arc(5, 6));
            assert!(digraph.remove_arc(6, 5));
            assert!(digraph.remove_arc(6, 10));
            assert!(digraph.remove_arc(7, 3));
            assert!(digraph.remove_arc(8, 7));
            assert!(digraph.remove_arc(8, 10));
            assert!(digraph.remove_arc(9, 7));
            assert!(digraph.remove_arc(9, 11));
            assert!(digraph.remove_arc(10, 6));
            assert!(digraph.remove_arc(11, 9));

            assert!(digraph.arcs().eq([]));

            assert!(!digraph.remove_arc(0, 1));
            assert!(!digraph.remove_arc(1, 0));
            assert!(!digraph.remove_arc(1, 2));
            assert!(!digraph.remove_arc(1, 4));
            assert!(!digraph.remove_arc(2, 1));
            assert!(!digraph.remove_arc(3, 2));
            assert!(!digraph.remove_arc(3, 4));
            assert!(!digraph.remove_arc(3, 5));
            assert!(!digraph.remove_arc(3, 7));
            assert!(!digraph.remove_arc(3, 10));
            assert!(!digraph.remove_arc(3, 11));
            assert!(!digraph.remove_arc(4, 3));
            assert!(!digraph.remove_arc(5, 6));
            assert!(!digraph.remove_arc(6, 5));
            assert!(!digraph.remove_arc(6, 10));
            assert!(!digraph.remove_arc(7, 3));
            assert!(!digraph.remove_arc(8, 7));
            assert!(!digraph.remove_arc(8, 10));
            assert!(!digraph.remove_arc(9, 7));
            assert!(!digraph.remove_arc(9, 11));
            assert!(!digraph.remove_arc(10, 6));
            assert!(!digraph.remove_arc(11, 9));
        }

        #[test]
        #[allow(clippy::cognitive_complexity)]
        fn remove_arc_kattis_cantinaofbabel_2() {
            let mut digraph = kattis_cantinaofbabel_2();

            assert!(digraph.arcs().eq([
                (0, 1),
                (1, 0),
                (1, 7),
                (2, 0),
                (2, 5),
                (2, 7),
                (3, 4),
                (4, 3),
                (5, 3),
                (5, 6),
                (6, 5),
                (7, 2),
                (8, 7),
                (8, 9),
                (8, 11),
                (9, 8),
                (10, 9),
                (10, 11),
                (11, 10),
            ]));

            assert!(!digraph.remove_arc(0, 3));

            assert!(digraph.remove_arc(0, 1));
            assert!(digraph.remove_arc(1, 0));
            assert!(digraph.remove_arc(1, 7));
            assert!(digraph.remove_arc(2, 0));
            assert!(digraph.remove_arc(2, 5));
            assert!(digraph.remove_arc(2, 7));
            assert!(digraph.remove_arc(3, 4));
            assert!(digraph.remove_arc(4, 3));
            assert!(digraph.remove_arc(5, 3));
            assert!(digraph.remove_arc(5, 6));
            assert!(digraph.remove_arc(6, 5));
            assert!(digraph.remove_arc(7, 2));
            assert!(digraph.remove_arc(8, 7));
            assert!(digraph.remove_arc(8, 9));
            assert!(digraph.remove_arc(8, 11));
            assert!(digraph.remove_arc(9, 8));
            assert!(digraph.remove_arc(10, 9));
            assert!(digraph.remove_arc(10, 11));
            assert!(digraph.remove_arc(11, 10));

            assert!(digraph.arcs().eq([]));

            assert!(!digraph.remove_arc(0, 1));
            assert!(!digraph.remove_arc(1, 0));
            assert!(!digraph.remove_arc(1, 7));
            assert!(!digraph.remove_arc(2, 0));
            assert!(!digraph.remove_arc(2, 5));
            assert!(!digraph.remove_arc(2, 7));
            assert!(!digraph.remove_arc(3, 4));
            assert!(!digraph.remove_arc(4, 3));
            assert!(!digraph.remove_arc(5, 3));
            assert!(!digraph.remove_arc(5, 6));
            assert!(!digraph.remove_arc(6, 5));
            assert!(!digraph.remove_arc(7, 2));
            assert!(!digraph.remove_arc(8, 7));
            assert!(!digraph.remove_arc(8, 9));
            assert!(!digraph.remove_arc(8, 11));
            assert!(!digraph.remove_arc(9, 8));
            assert!(!digraph.remove_arc(10, 9));
            assert!(!digraph.remove_arc(10, 11));
            assert!(!digraph.remove_arc(11, 10));
        }

        #[test]
        fn remove_arc_kattis_escapewallmaria_1() {
            let mut digraph = kattis_escapewallmaria_1();

            assert!(digraph.arcs().eq([
                (5, 6),
                (5, 9),
                (6, 5),
                (9, 5),
                (9, 13),
                (13, 9),
                (13, 12)
            ]));

            assert!(!digraph.remove_arc(0, 1));

            assert!(digraph.remove_arc(5, 6));
            assert!(digraph.remove_arc(5, 9));
            assert!(digraph.remove_arc(6, 5));
            assert!(digraph.remove_arc(9, 5));
            assert!(digraph.remove_arc(9, 13));
            assert!(digraph.remove_arc(13, 9));
            assert!(digraph.remove_arc(13, 12));

            assert!(digraph.arcs().eq([]));

            assert!(!digraph.remove_arc(5, 6));
            assert!(!digraph.remove_arc(5, 9));
            assert!(!digraph.remove_arc(6, 5));
            assert!(!digraph.remove_arc(9, 5));
            assert!(!digraph.remove_arc(9, 13));
            assert!(!digraph.remove_arc(13, 9));
            assert!(!digraph.remove_arc(13, 12));
        }

        #[test]
        fn remove_arc_kattis_escapewallmaria_2() {
            let mut digraph = kattis_escapewallmaria_2();

            assert!(digraph.arcs().eq([
                (5, 6),
                (5, 9),
                (6, 5),
                (9, 5),
                (12, 13),
                (13, 9),
                (13, 12)
            ]));

            assert!(!digraph.remove_arc(0, 1));

            assert!(digraph.remove_arc(5, 6));
            assert!(digraph.remove_arc(5, 9));
            assert!(digraph.remove_arc(6, 5));
            assert!(digraph.remove_arc(9, 5));
            assert!(digraph.remove_arc(12, 13));
            assert!(digraph.remove_arc(13, 9));
            assert!(digraph.remove_arc(13, 12));

            assert!(digraph.arcs().eq([]));

            assert!(!digraph.remove_arc(5, 6));
            assert!(!digraph.remove_arc(5, 9));
            assert!(!digraph.remove_arc(6, 5));
            assert!(!digraph.remove_arc(9, 5));
            assert!(!digraph.remove_arc(12, 13));
            assert!(!digraph.remove_arc(13, 9));
            assert!(!digraph.remove_arc(13, 12));
        }

        #[test]
        #[allow(clippy::cognitive_complexity)]
        fn remove_arc_kattis_escapewallmaria_3() {
            let mut digraph = kattis_escapewallmaria_3();

            assert!(digraph.arcs().eq([
                (1, 2),
                (1, 5),
                (2, 1),
                (2, 6),
                (5, 1),
                (5, 6),
                (5, 9),
                (6, 2),
                (6, 5),
                (9, 5),
                (9, 13),
                (12, 13),
                (13, 9),
                (13, 12)
            ]));

            assert!(!digraph.remove_arc(0, 1));

            assert!(digraph.remove_arc(1, 2));
            assert!(digraph.remove_arc(1, 5));
            assert!(digraph.remove_arc(2, 1));
            assert!(digraph.remove_arc(2, 6));
            assert!(digraph.remove_arc(5, 1));
            assert!(digraph.remove_arc(5, 6));
            assert!(digraph.remove_arc(5, 9));
            assert!(digraph.remove_arc(6, 2));
            assert!(digraph.remove_arc(6, 5));
            assert!(digraph.remove_arc(9, 5));
            assert!(digraph.remove_arc(9, 13));
            assert!(digraph.remove_arc(12, 13));
            assert!(digraph.remove_arc(13, 9));
            assert!(digraph.remove_arc(13, 12));

            assert!(digraph.arcs().eq([]));

            assert!(!digraph.remove_arc(1, 2));
            assert!(!digraph.remove_arc(1, 5));
            assert!(!digraph.remove_arc(2, 1));
            assert!(!digraph.remove_arc(2, 6));
            assert!(!digraph.remove_arc(5, 1));
            assert!(!digraph.remove_arc(5, 6));
            assert!(!digraph.remove_arc(5, 9));
            assert!(!digraph.remove_arc(6, 2));
            assert!(!digraph.remove_arc(6, 5));
            assert!(!digraph.remove_arc(9, 5));
            assert!(!digraph.remove_arc(9, 13));
            assert!(!digraph.remove_arc(12, 13));
            assert!(!digraph.remove_arc(13, 9));
            assert!(!digraph.remove_arc(13, 12));
        }

        #[test]
        fn remove_arc_out_of_bounds() {
            assert!(!$type::trivial().remove_arc(0, 1));
            assert!(!$type::trivial().remove_arc(1, 0));
        }

        #[test]
        fn semidegree_sequence_bang_jensen_196() {
            assert!(bang_jensen_196().semidegree_sequence().eq([
                (1, 3),
                (1, 3),
                (3, 1),
                (1, 2),
                (2, 1),
                (1, 1),
                (1, 1),
                (3, 1)
            ]));
        }

        #[test]
        fn semidegree_sequence_bang_jensen_34() {
            assert!(bang_jensen_34().semidegree_sequence().eq([
                (1, 1),
                (1, 1),
                (0, 3),
                (1, 0),
                (2, 0),
                (1, 1)
            ]));
        }

        #[test]
        fn semidegree_sequence_bang_jensen_94() {
            assert!(bang_jensen_94().semidegree_sequence().eq([
                (0, 2),
                (2, 1),
                (1, 4),
                (2, 1),
                (1, 1),
                (2, 0),
                (1, 0)
            ]));
        }

        #[test]
        fn semidegree_sequence_kattis_builddeps() {
            assert!(kattis_builddeps().semidegree_sequence().eq([
                (0, 2),
                (3, 0),
                (0, 3),
                (2, 1),
                (2, 1),
                (1, 1)
            ]));
        }

        #[test]
        fn semidegree_sequence_kattis_cantinaofbabel_1() {
            assert!(kattis_cantinaofbabel_1().semidegree_sequence().eq([
                (1, 1),
                (2, 3),
                (2, 1),
                (2, 6),
                (2, 1),
                (2, 1),
                (2, 2),
                (3, 1),
                (0, 2),
                (1, 2),
                (3, 1),
                (2, 1)
            ]));
        }

        #[test]
        fn semidegree_sequence_kattis_cantinaofbabel_2() {
            assert!(kattis_cantinaofbabel_2().semidegree_sequence().eq([
                (2, 1),
                (1, 2),
                (1, 3),
                (2, 1),
                (1, 1),
                (2, 2),
                (1, 1),
                (3, 1),
                (1, 3),
                (2, 1),
                (1, 2),
                (2, 1)
            ]));
        }

        #[test]
        fn semidegree_sequence_kattis_escapewallmaria_1() {
            assert!(kattis_escapewallmaria_1().semidegree_sequence().eq([
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (2, 2),
                (1, 1),
                (0, 0),
                (0, 0),
                (2, 2),
                (0, 0),
                (0, 0),
                (1, 0),
                (1, 2),
                (0, 0),
                (0, 0)
            ]));
        }

        #[test]
        fn semidegree_sequence_kattis_escapewallmaria_2() {
            assert!(kattis_escapewallmaria_2().semidegree_sequence().eq([
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (2, 2),
                (1, 1),
                (0, 0),
                (0, 0),
                (2, 1),
                (0, 0),
                (0, 0),
                (1, 1),
                (1, 2),
                (0, 0),
                (0, 0)
            ]));
        }

        #[test]
        fn semidegree_sequence_kattis_escapewallmaria_3() {
            assert!(kattis_escapewallmaria_3().semidegree_sequence().eq([
                (0, 0),
                (2, 2),
                (2, 2),
                (0, 0),
                (0, 0),
                (3, 3),
                (2, 2),
                (0, 0),
                (0, 0),
                (2, 2),
                (0, 0),
                (0, 0),
                (1, 1),
                (2, 2),
                (0, 0),
                (0, 0)
            ]));
        }

        #[test]
        fn sinks_bang_jensen_196() {
            assert!(bang_jensen_196().sinks().eq([]));
        }

        #[test]
        fn sinks_bang_jensen_34() {
            assert!(bang_jensen_34().sinks().eq([3, 4]));
        }

        #[test]
        fn sinks_bang_jensen_94() {
            assert!(bang_jensen_94().sinks().eq([5, 6]));
        }

        #[test]
        fn sinks_kattis_builddeps() {
            assert!(kattis_builddeps().sinks().eq([1]));
        }

        #[test]
        fn sinks_kattis_cantinaofbabel_1() {
            assert!(kattis_cantinaofbabel_1().sinks().eq([]));
        }

        #[test]
        fn sinks_kattis_cantinaofbabel_2() {
            assert!(kattis_cantinaofbabel_2().sinks().eq([]));
        }

        #[test]
        fn sinks_kattis_escapewallmaria_1() {
            assert!(kattis_escapewallmaria_1()
                .sinks()
                .eq([0, 1, 2, 3, 4, 7, 8, 10, 11, 12, 14, 15]));
        }

        #[test]
        fn sinks_kattis_escapewallmaria_2() {
            assert!(kattis_escapewallmaria_2()
                .sinks()
                .eq([0, 1, 2, 3, 4, 7, 8, 10, 11, 14, 15]));
        }

        #[test]
        fn sinks_kattis_escapewallmaria_3() {
            assert!(kattis_escapewallmaria_3()
                .sinks()
                .eq([0, 3, 4, 7, 8, 10, 11, 14, 15]));
        }

        #[test]
        fn size_bang_jensen_196() {
            assert_eq!(bang_jensen_196().size(), 13);
        }

        #[test]
        fn size_bang_jensen_34() {
            assert_eq!(bang_jensen_34().size(), 6);
        }

        #[test]
        fn size_bang_jensen_94() {
            assert_eq!(bang_jensen_94().size(), 9);
        }

        #[test]
        fn size_kattis_builddeps() {
            assert_eq!(kattis_builddeps().size(), 8);
        }

        #[test]
        fn size_kattis_cantinaofbabel_1() {
            assert_eq!(kattis_cantinaofbabel_1().size(), 22);
        }

        #[test]
        fn size_kattis_cantinaofbabel_2() {
            assert_eq!(kattis_cantinaofbabel_2().size(), 19);
        }

        #[test]
        fn size_kattis_escapewallmaria_1() {
            assert_eq!(kattis_escapewallmaria_1().size(), 7);
        }

        #[test]
        fn size_kattis_escapewallmaria_2() {
            assert_eq!(kattis_escapewallmaria_2().size(), 7);
        }

        #[test]
        fn size_kattis_escapewallmaria_3() {
            assert_eq!(kattis_escapewallmaria_3().size(), 14);
        }

        #[test]
        fn sources_bang_jensen_196() {
            assert!(bang_jensen_196().sources().eq([]));
        }

        #[test]
        fn sources_bang_jensen_34() {
            assert!(bang_jensen_34().sources().eq([2]));
        }

        #[test]
        fn sources_bang_jensen_94() {
            assert!(bang_jensen_94().sources().eq([0]));
        }

        #[test]
        fn sources_kattis_builddeps() {
            assert!(kattis_builddeps().sources().eq([0, 2]));
        }

        #[test]
        fn sources_kattis_cantinaofbabel_1() {
            assert!(kattis_cantinaofbabel_1().sources().eq([8]));
        }

        #[test]
        fn sources_kattis_cantinaofbabel_2() {
            assert!(kattis_cantinaofbabel_2().sources().eq([]));
        }

        #[test]
        fn sources_kattis_escapewallmaria_1() {
            assert!(kattis_escapewallmaria_1()
                .sources()
                .eq([0, 1, 2, 3, 4, 7, 8, 10, 11, 14, 15]));
        }

        #[test]
        fn sources_kattis_escapewallmaria_2() {
            assert!(kattis_escapewallmaria_2()
                .sources()
                .eq([0, 1, 2, 3, 4, 7, 8, 10, 11, 14, 15]));
        }

        #[test]
        fn sources_kattis_escapewallmaria_3() {
            assert!(kattis_escapewallmaria_3()
                .sources()
                .eq([0, 3, 4, 7, 8, 10, 11, 14, 15]));
        }

        #[test]
        #[should_panic(expected = "a digraph has at least one vertex")]
        fn star_0() {
            let _ = $type::star(0);
        }

        #[test]
        fn star_1() {
            let digraph = $type::star(1);

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn star_1_complement() {
            let digraph = $type::star(1).complement();

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn star_2() {
            let digraph = $type::star(2);

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([(0, 1), (1, 0)]));
        }

        #[test]
        fn star_2_complement() {
            let digraph = $type::star(2).complement();

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn star_3() {
            let digraph = $type::star(3);

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([(0, 1), (0, 2), (1, 0), (2, 0)]));
        }

        #[test]
        fn star_3_complement() {
            let digraph = $type::star(3).complement();

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([(1, 2), (2, 1)]));
        }

        #[test]
        #[should_panic(expected = "a wheel digraph has at least four vertices")]
        fn wheel_0() {
            let _ = $type::wheel(0);
        }

        #[test]
        #[should_panic(expected = "a wheel digraph has at least four vertices")]
        fn wheel_1() {
            let _ = $type::wheel(1);
        }

        #[test]
        #[should_panic(expected = "a wheel digraph has at least four vertices")]
        fn wheel_2() {
            let _ = $type::wheel(2);
        }

        #[test]
        #[should_panic(expected = "a wheel digraph has at least four vertices")]
        fn wheel_3() {
            let _ = $type::wheel(3);
        }

        #[test]
        fn wheel_4() {
            let digraph = $type::wheel(4);

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
            let digraph = $type::wheel(4).complement();

            assert_eq!(digraph.order(), 4);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn wheel_5() {
            let digraph = $type::wheel(5);

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
            let digraph = $type::wheel(5).complement();

            assert_eq!(digraph.order(), 5);
            assert!(digraph.arcs().eq([(1, 3), (2, 4), (3, 1), (4, 2)]));
        }

        #[test]
        fn wheel_6() {
            let digraph = $type::wheel(6);

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
            let digraph = $type::wheel(6).complement();

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
    }
}

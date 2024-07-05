//! An adjacency list representation of an unweighted digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         IsSimple,
//!     },
//! };
//!
//! let mut digraph = Digraph::empty(3);
//!
//! digraph.add_arc(0, 1);
//!
//! assert!(digraph.is_simple());
//!
//! digraph.add_arc(1, 1);
//!
//! assert!(!digraph.is_simple());
//! ```

use {
    crate::{
        gen::Empty,
        op::{
            AddArc,
            ArcWeight,
            Arcs,
            ArcsWeighted,
            Converse,
            HasArc,
            Indegree,
            IsSimple,
            Order,
            OutNeighbors,
            OutNeighborsWeighted,
            Outdegree,
            RemoveArc,
            Size,
            Vertices,
        },
    },
    std::collections::BTreeSet,
};

/// An adjacency list representation of an unweighted digraph
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Digraph {
    /// The arcs of the digraph
    arcs: Vec<BTreeSet<usize>>,
}

impl AddArc for Digraph {
    fn add_arc(&mut self, u: usize, v: usize) {
        let _ = self.arcs[u].insert(v);
    }
}

impl ArcWeight<usize> for Digraph {
    fn arc_weight(&self, u: usize, v: usize) -> Option<&usize> {
        self.has_arc(u, v).then_some(&1)
    }
}

impl Arcs for Digraph {
    fn arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.arcs
            .iter()
            .enumerate()
            .flat_map(|(u, set)| set.iter().map(move |v| (u, *v)))
    }
}

impl ArcsWeighted<usize> for Digraph {
    fn arcs_weighted<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a usize)>
    where
        usize: 'a,
    {
        self.arcs
            .iter()
            .enumerate()
            .flat_map(|(u, set)| set.iter().map(move |v| (u, *v, &1)))
    }
}

impl Converse for Digraph {
    fn converse(&self) -> Self {
        let order = self.order();
        let mut converse = Self::empty(order);

        for (u, v) in self.arcs() {
            converse.add_arc(v, u);
        }

        converse
    }
}

impl Empty for Digraph {
    fn empty(order: usize) -> Self {
        assert!(order > 0, "a digraph must have at least one vertex");

        Self::from(vec![BTreeSet::new(); order])
    }
}

impl From<Vec<BTreeSet<usize>>> for Digraph {
    fn from(arcs: Vec<BTreeSet<usize>>) -> Self {
        Self { arcs }
    }
}

impl HasArc for Digraph {
    fn has_arc(&self, u: usize, v: usize) -> bool {
        self.arcs.get(u).map_or(false, |set| set.contains(&v))
    }
}

impl Indegree for Digraph {
    fn indegree(&self, v: usize) -> usize {
        self.arcs.iter().filter(|set| set.contains(&v)).count()
    }
}

impl IsSimple for Digraph {
    fn is_simple(&self) -> bool {
        self.arcs
            .iter()
            .enumerate()
            .all(|(u, set)| !set.contains(&u))
    }
}

impl Order for Digraph {
    fn order(&self) -> usize {
        self.arcs.len()
    }
}

impl OutNeighbors for Digraph {
    /// # Panics
    ///
    /// Panics if `u` is out of bounds.
    fn out_neighbors(&self, u: usize) -> impl Iterator<Item = usize> {
        self.arcs[u].iter().copied()
    }
}

impl OutNeighborsWeighted<usize> for Digraph {
    /// # Panics
    ///
    /// Panics if `u` is out of bounds.
    fn out_neighbors_weighted<'a>(&'a self, u: usize) -> impl Iterator<Item = (usize, &'a usize)>
    where
        usize: 'a,
    {
        self.out_neighbors(u).map(move |v| (v, &1))
    }
}

impl Outdegree for Digraph {
    /// # Panics
    ///
    /// Panics if `u` is out of bounds.
    fn outdegree(&self, u: usize) -> usize {
        self.arcs[u].len()
    }
}

impl RemoveArc for Digraph {
    fn remove_arc(&mut self, u: usize, v: usize) -> bool {
        self.arcs.get_mut(u).map_or(false, |set| set.remove(&v))
    }
}

impl Size for Digraph {
    fn size(&self) -> usize {
        self.arcs.iter().map(BTreeSet::len).sum()
    }
}

impl Vertices for Digraph {
    fn vertices(&self) -> impl Iterator<Item = usize> {
        0..self.order()
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            adjacency_list::fixture::{
                bang_jensen_34,
                bang_jensen_94,
                kattis_builddeps,
                kattis_escapewallmaria_1,
                kattis_escapewallmaria_2,
                kattis_escapewallmaria_3,
            },
            gen::{
                Complete,
                Cycle,
                RandomTournament,
            },
            op::{
                Converse,
                Degree,
                DegreeSequence,
                HasEdge,
                InNeighbors,
                IsBalanced,
                IsComplete,
                IsIsolated,
                IsOriented,
                IsPendant,
                IsRegular,
                IsSemicomplete,
                IsSimple,
                IsSubdigraph,
                IsSuperdigraph,
                IsSymmetric,
                IsWalk,
            },
        },
        proptest::proptest,
    };

    proptest! {
        #[test]
        fn add_arc_arc_weight(u in 1..100_usize, v in 1..100_usize) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc(u, v);

            for x in digraph.vertices() {
                for y in digraph.vertices() {
                    assert_eq!(
                        digraph.arc_weight(x, y),
                        (x == u && y == v).then_some(&1)
                    );
                }
            }
        }

        #[test]
        fn add_arc_degree(u in 1..100_usize, v in 1..100_usize) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc(u, v);

            for x in digraph.vertices() {
                assert_eq!(
                    digraph.degree(x),
                    usize::from(x == u) + usize::from(x == v)
                );
            }
        }

        #[test]
        fn add_arc_has_arc(u in 1..100_usize, v in 1..100_usize) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc(u, v);

            assert!(digraph.has_arc(u, v));
        }

        #[test]
        fn add_arc_indegree(u in 1..100_usize, v in 1..100_usize) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc(u, v);

            for x in digraph.vertices() {
                assert_eq!(digraph.indegree(x), usize::from(x == v));
            }
        }

        #[test]
        fn add_arc_outdegree(u in 1..100_usize, v in 1..100_usize) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc(u, v);

            for x in digraph.vertices() {
                assert_eq!(digraph.outdegree(x), usize::from(x == u));
            }
        }

        #[test]
        fn add_arc_remove_arc(u in 1..100_usize, v in 1..100_usize) {
            let d = Digraph::empty(100);
            let mut h = d.clone();

            h.add_arc(u, v);

            for x in d.vertices() {
                for y in d.vertices() {
                    if x == u && y == v {
                        assert!(h.remove_arc(x, y));
                    } else {
                        assert!(!h.remove_arc(x, y));
                    }
                }
            }

            assert_eq!(d, h);
        }

        #[test]
        fn complete_degree(order in 1..100_usize) {
            let digraph = Digraph::complete(order);

            for u in digraph.vertices() {
                assert_eq!(digraph.degree(u), order * 2 - 2);
            }
        }

        #[test]
        fn complete_has_edge(order in 2..100_usize) {
            let digraph = Digraph::complete(order);

            for u in 0..order {
                for v in u + 1..order {
                    assert!(digraph.has_edge(u, v));
                }
            }
        }

        #[test]
        fn complete_indegree(order in 1..100_usize) {
            let digraph = Digraph::complete(order);

            for u in digraph.vertices() {
                assert_eq!(digraph.indegree(u), order - 1);
            }
        }

        #[test]
        fn complete_is_balanced(order in 1..100_usize) {
            assert!(Digraph::complete(order).is_balanced());
        }

        #[test]
        fn complete_is_complete(order in 1..100_usize) {
            assert!(Digraph::complete(order).is_complete());
        }

        #[test]
        fn complete_is_isolated(order in 2..100_usize) {
            let digraph = Digraph::complete(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_isolated(u));
            }
        }

        #[test]
        fn complete_is_oriented(order in 2..100_usize) {
            assert!(!Digraph::complete(order).is_oriented());
        }

        #[test]
        fn complete_is_pendant(order in 1..100_usize) {
            let digraph = Digraph::complete(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_pendant(u));
            }
        }

        #[test]
        fn complete_is_regular(order in 1..100_usize) {
            assert!(Digraph::complete(order).is_regular());
        }

        #[test]
        fn complete_is_semicomplete(order in 1..100_usize) {
            assert!(Digraph::complete(order).is_semicomplete());
        }

        #[test]
        fn complete_is_simple(order in 2..100_usize) {
            assert!(Digraph::complete(order).is_simple());
        }

        #[test]
        fn complete_is_subdigraph(order in 1..100_usize) {
            let digraph = Digraph::complete(order);

            assert!(digraph.is_subdigraph(&digraph));
        }

        #[test]
        fn complete_is_superdigraph(order in 1..100_usize) {
            let digraph = Digraph::complete(order);

            assert!(digraph.is_superdigraph(&digraph));
        }

        #[test]
        fn complete_is_symmetric(order in 1..100_usize) {
            assert!(Digraph::complete(order).is_symmetric());
        }

        #[test]
        fn complete_order(order in 1..100_usize) {
            assert_eq!(Digraph::complete(order).order(), order);
        }

        #[test]
        fn complete_outdegree(order in 1..100_usize) {
            let digraph = Digraph::complete(order);

            for s in digraph.vertices() {
                assert_eq!(digraph.outdegree(s), order - 1);
            }
        }

        #[test]
        fn complete_size(order in 1..100_usize) {
            assert_eq!(Digraph::complete(order).size(), order * (order - 1));
        }

        #[test]
        fn cycle_degree(order in 1..100_usize) {
            let digraph = Digraph::cycle(order);

            for u in digraph.vertices() {
                assert_eq!(digraph.degree(u), 2);
            }
        }

        #[test]
        fn cycle_has_edge(order in 3..100_usize) {
            let digraph = Digraph::cycle(order);

            for u in 0..order {
                for v in u + 1..order {
                    assert!(!digraph.has_edge(u, v));
                }
            }
        }

        #[test]
        fn cycle_indegree(order in 1..100_usize) {
            let digraph = Digraph::cycle(order);

            for u in digraph.vertices() {
                assert_eq!(digraph.indegree(u), 1);
            }
        }

        #[test]
        fn cycle_is_balanced(order in 1..100_usize) {
            assert!(Digraph::cycle(order).is_balanced());
        }

        #[test]
        fn cycle_is_complete(order in 3..100_usize) {
            assert!(!Digraph::cycle(order).is_complete());
        }

        #[test]
        fn cycle_is_isolated(order in 1..100_usize) {
            let digraph = Digraph::cycle(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_isolated(u));
            }
        }

        #[test]
        fn cycle_is_oriented(order in 3..100_usize) {
            assert!(Digraph::cycle(order).is_oriented());
        }

        #[test]
        fn cycle_is_pendant(order in 1..100_usize) {
            let digraph = Digraph::cycle(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_pendant(u));
            }
        }

        #[test]
        fn cycle_is_regular(order in 1..100_usize) {
            assert!(Digraph::cycle(order).is_regular());
        }

        #[test]
        fn cycle_is_semicomplete(order in 4..100_usize) {
            assert!(!Digraph::cycle(order).is_semicomplete());
        }

        #[test]
        fn cycle_is_simple(order in 2..100_usize) {
            assert!(Digraph::cycle(order).is_simple());
        }

        #[test]
        fn cycle_is_sink(order in 1..100_usize) {
            let digraph = Digraph::cycle(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_sink(u));
            }
        }

        #[test]
        fn cycle_is_source(order in 1..100_usize) {
            let digraph = Digraph::cycle(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_source(u));
            }
        }

        #[test]
        fn cycle_is_subdigraph(order in 1..100_usize) {
            let digraph = Digraph::cycle(order);

            assert!(digraph.is_subdigraph(&digraph));
        }

        #[test]
        fn cycle_is_superdigraph(order in 1..100_usize) {
            let digraph = Digraph::cycle(order);

            assert!(digraph.is_superdigraph(&digraph));
        }

        #[test]
        fn cycle_is_symmetric(order in 3..100_usize) {
            assert!(!Digraph::cycle(order).is_symmetric());
        }

        #[test]
        fn empty_arcs(order in 1..100_usize) {
            assert!(Digraph::empty(order).arcs().eq([]));
        }

        #[test]
        fn empty_degree(order in 1..100_usize) {
            let digraph = Digraph::empty(order);

            for u in digraph.vertices() {
                assert_eq!(digraph.degree(u), 0);
            }
        }

        #[test]
        fn empty_has_arc(order in 1..100_usize) {
            let digraph = Digraph::empty(order);

            for u in digraph.vertices() {
                for v in digraph.vertices() {
                    assert!(!digraph.has_arc(u, v));
                }
            }
        }

        #[test]
        fn empty_has_edge(order in 1..100_usize) {
            let digraph = Digraph::empty(order);

            for u in digraph.vertices() {
                for v in digraph.vertices() {
                    assert!(!digraph.has_edge(u, v));
                }
            }
        }

        #[test]
        fn empty_indegree(order in 1..100_usize) {
            let digraph = Digraph::empty(order);

            for u in digraph.vertices() {
                assert_eq!(digraph.indegree(u), 0);
            }
        }

        #[test]
        fn empty_is_balanced(order in 1..100_usize) {
            assert!(Digraph::empty(order).is_balanced());
        }

        #[test]
        fn empty_is_complete(order in 2..100_usize) {
            assert!(!Digraph::empty(order).is_complete());
        }

        #[test]
        fn empty_is_isolated(order in 1..100_usize) {
            let digraph = Digraph::empty(order);

            for u in digraph.vertices() {
                assert!(digraph.is_isolated(u));
            }
        }

        #[test]
        fn empty_is_oriented(order in 1..100_usize) {
            assert!(Digraph::empty(order).is_oriented());
        }

        #[test]
        fn empty_is_pendant(order in 1..100_usize) {
            let digraph = Digraph::empty(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_pendant(u));
            }
        }

        #[test]
        fn empty_is_regular(order in 1..100_usize) {
            assert!(Digraph::empty(order).is_regular());
        }

        #[test]
        fn empty_is_semicomplete(order in 2..100_usize) {
            assert!(!Digraph::empty(order).is_semicomplete());
        }

        #[test]
        fn empty_is_simple(order in 1..100_usize) {
            assert!(Digraph::empty(order).is_simple());
        }

        #[test]
        fn empty_is_sink(order in 1..100_usize) {
            let digraph = Digraph::empty(order);

            for u in digraph.vertices() {
                assert!(digraph.is_sink(u));
            }
        }

        #[test]
        fn empty_is_source(order in 1..100_usize) {
            let digraph = Digraph::empty(order);

            for u in digraph.vertices() {
                assert!(digraph.is_source(u));
            }
        }

        #[test]
        fn empty_is_subdigraph(order in 1..100_usize) {
            let digraph = Digraph::empty(order);

            assert!(digraph.is_subdigraph(&digraph));
        }

        #[test]
        fn empty_is_superdigraph(order in 1..100_usize) {
            let digraph = Digraph::empty(order);

            assert!(digraph.is_superdigraph(&digraph));
        }

        #[test]
        fn empty_is_symmetric(order in 1..100_usize) {
            assert!(Digraph::empty(order).is_symmetric());
        }

        #[test]
        fn empty_outdegree(order in 1..100_usize) {
            let digraph = Digraph::empty(order);

            for u in digraph.vertices() {
                assert_eq!(digraph.outdegree(u), 0);
            }
        }

        #[test]
        fn has_arc_out_of_bounds(order in 1..100_usize) {
            let digraph = Digraph::empty(order);

            for u in 0..order {
                assert!(!digraph.has_arc(u, order));
                assert!(!digraph.has_arc(order, u));
            }
        }

        #[test]
        fn random_tournament_degree(order in 1..100_usize) {
            let digraph = Digraph::random_tournament(order);

            for u in digraph.vertices() {
                assert_eq!(digraph.degree(u), order - 1);
            }
        }

        #[test]
        fn random_tournament_has_edge(order in 1..100_usize) {
            let digraph = Digraph::random_tournament(order);

            for u in digraph.vertices() {
                for v in digraph.vertices() {
                    assert!(!digraph.has_edge(u, v));
                }
            }
        }

        #[test]
        fn random_tournament_indegree(order in 1..100_usize) {
            let digraph = Digraph::random_tournament(order);

            for u in digraph.vertices() {
                assert!((0..order).contains(&digraph.indegree(u)));
            }
        }

        #[test]
        fn random_tournament_is_complete(order in 2..100_usize) {
            assert!(!Digraph::random_tournament(order).is_complete());
        }

        #[test]
        fn random_tournament_is_isolated(order in 2..100_usize) {
            let digraph = Digraph::random_tournament(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_isolated(u));
            }
        }

        #[test]
        fn random_tournament_is_oriented(order in 1..100_usize) {
            assert!(Digraph::random_tournament(order).is_oriented());
        }

        #[test]
        fn random_tournament_is_pendant(order in 3..100_usize) {
            let digraph = Digraph::random_tournament(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_pendant(u));
            }
        }

        #[test]
        fn random_tournament_is_semicomplete(order in 1..100_usize) {
            assert!(Digraph::random_tournament(order).is_semicomplete());
        }

        #[test]
        fn random_tournament_is_simple(order in 1..100_usize) {
            assert!(Digraph::random_tournament(order).is_simple());
        }

        #[test]
        fn random_tournament_is_subdigraph(order in 1..100_usize) {
            let digraph = Digraph::random_tournament(order);

            assert!(digraph.is_subdigraph(&digraph));
        }

        #[test]
        fn random_tournament_is_superdigraph(order in 1..100_usize) {
            let digraph = Digraph::random_tournament(order);

            assert!(digraph.is_superdigraph(&digraph));
        }

        #[test]
        fn random_tournament_is_symmetric(order in 2..100_usize) {
            assert!(!Digraph::random_tournament(order).is_symmetric());
        }

        #[test]
        fn random_tournament_order(order in 1..100_usize) {
            assert_eq!(Digraph::random_tournament(order).order(), order);
        }

        #[test]
        fn random_tournament_outdegree(order in 1..100_usize) {
            let digraph = Digraph::random_tournament(order);

            for u in digraph.vertices() {
                assert!((0..order).contains(&digraph.outdegree(u)));
            }
        }

        #[test]
        fn random_tournament_size(order in 1..100_usize) {
            let digraph = Digraph::random_tournament(order);

            assert_eq!(digraph.size(), order * (order - 1) / 2);
        }
    }

    #[test]
    fn arcs_bang_jensen_34() {
        assert!(bang_jensen_34()
            .arcs()
            .eq(vec![(0, 4), (1, 0), (2, 1), (2, 3), (2, 5), (5, 4)]));
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
    fn complete() {
        assert!(Digraph::complete(1).arcs().eq([]));
        assert!(Digraph::complete(2).arcs().eq([(0, 1), (1, 0)]));

        assert!(Digraph::complete(3)
            .arcs()
            .eq([(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)]));
    }

    #[test]
    fn complete_has_edge_trivial() {
        assert!(!Digraph::complete(1).has_edge(0, 0));
    }

    #[test]
    fn complete_is_isolated_trivial() {
        assert!(Digraph::complete(1).is_isolated(0));
    }

    #[test]
    fn complete_is_oriented_trivial() {
        assert!(Digraph::complete(1).is_oriented());
    }

    #[test]
    fn complete_is_simple_trivial() {
        assert!(Digraph::complete(1).is_simple());
    }

    #[test]
    fn converse_bang_jensen_34() {
        assert!(bang_jensen_34().converse().arcs().eq([
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
        assert!(bang_jensen_94().converse().arcs().eq([
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
        assert!(kattis_builddeps().converse().arcs().eq([
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
    fn converse_kattis_escapewallmaria_1() {
        assert!(kattis_escapewallmaria_1().converse().arcs().eq([
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
        assert!(kattis_escapewallmaria_2().converse().arcs().eq([
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
        assert!(kattis_escapewallmaria_3().converse().arcs().eq([
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
    fn cycle() {
        assert!(Digraph::cycle(1).arcs().eq([(0, 0)]));
        assert!(Digraph::cycle(2).arcs().eq([(0, 1), (1, 0)]));
        assert!(Digraph::cycle(3).arcs().eq([(0, 1), (1, 2), (2, 0)]));
    }

    #[test]
    fn cycle_has_edge_trivial() {
        let digraph = Digraph::cycle(1);

        assert!(digraph.has_edge(0, 0));
    }

    #[test]
    fn cycle_has_edge_pair() {
        let digraph = Digraph::cycle(2);

        for u in 0..2 {
            for v in u + 1..2 {
                assert!(digraph.has_edge(u, v));
            }
        }
    }

    #[test]
    fn cycle_is_complete_trivial() {
        assert!(Digraph::cycle(1).is_complete());
    }

    #[test]
    fn cycle_is_complete_pair() {
        assert!(Digraph::cycle(2).is_complete());
    }

    #[test]
    fn cycle_is_oriented_trivial() {
        assert!(!Digraph::cycle(1).is_oriented());
    }

    #[test]
    fn cycle_is_oriented_pair() {
        assert!(!Digraph::cycle(2).is_oriented());
    }

    #[test]
    fn cycle_is_semicomplete_trivial() {
        assert!(Digraph::cycle(1).is_semicomplete());
    }

    #[test]
    fn cycle_is_semicomplete_pair() {
        assert!(Digraph::cycle(2).is_semicomplete());
    }

    #[test]
    fn cycle_is_semicomplete_triple() {
        assert!(Digraph::cycle(3).is_semicomplete());
    }

    #[test]
    fn cycle_is_simple_trivial() {
        assert!(!Digraph::cycle(1).is_simple());
    }

    #[test]
    fn cycle_is_symmetric_trivial() {
        assert!(Digraph::cycle(1).is_symmetric());
    }

    #[test]
    fn cycle_is_symmetric_pair() {
        assert!(Digraph::cycle(2).is_symmetric());
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
    fn degree_escapewallmaria_1() {
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
    fn degree_escapewallmaria_2() {
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
    fn degree_escapewallmaria_3() {
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
    fn degree_sequence_bang_jensen_34() {
        assert!(bang_jensen_34().degree_sequence().iter().eq(&[
            (1, 1),
            (1, 1),
            (0, 3),
            (1, 0),
            (2, 0),
            (1, 1)
        ]));
    }

    #[test]
    fn degree_sequence_bang_jensen_94() {
        assert!(bang_jensen_94().degree_sequence().iter().eq(&[
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
    fn degree_sequence_kattis_builddeps() {
        assert!(kattis_builddeps().degree_sequence().iter().eq(&[
            (0, 2),
            (3, 0),
            (0, 3),
            (2, 1),
            (2, 1),
            (1, 1)
        ]));
    }

    #[test]
    fn degree_sequence_kattis_escapewallmaria_1() {
        assert!(kattis_escapewallmaria_1().degree_sequence().iter().eq(&[
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
            (1, 2)
        ]));
    }

    #[test]
    fn degree_sequence_kattis_escapewallmaria_2() {
        assert!(kattis_escapewallmaria_2().degree_sequence().iter().eq(&[
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
            (1, 2)
        ]));
    }

    #[test]
    fn degree_sequence_kattis_escapewallmaria_3() {
        assert!(kattis_escapewallmaria_3().degree_sequence().iter().eq(&[
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
            (2, 2)
        ]));
    }

    #[test]
    fn empty() {
        assert!(Digraph::empty(1).arcs().eq([]));
        assert!(Digraph::empty(2).arcs().eq([]));
        assert!(Digraph::empty(3).arcs().eq([]));
    }

    #[test]
    fn empty_is_complete_trivial() {
        assert!(Digraph::trivial().is_complete());
    }

    #[test]
    fn empty_is_semicomplete_trivial() {
        assert!(Digraph::trivial().is_semicomplete());
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
    fn indegree_escapewallmaria_1() {
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
    }

    #[test]
    fn indegree_escapewallmaria_2() {
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
    }

    #[test]
    fn indegree_escapewallmaria_3() {
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
    fn is_walk_bang_jensen_34() {
        let digraph = bang_jensen_34();

        assert!(digraph.is_walk(&[0, 4]));
        assert!(digraph.is_walk(&[1, 0, 4]));
        assert!(digraph.is_walk(&[2, 1, 0, 4]));
        assert!(digraph.is_walk(&[2, 3]));
        assert!(digraph.is_walk(&[2, 5, 4]));
        assert!(digraph.is_walk(&[5, 4]));

        assert!(!digraph.is_walk(&[0, 1]));
        assert!(!digraph.is_walk(&[1, 2]));
        assert!(!digraph.is_walk(&[2, 0]));
        assert!(!digraph.is_walk(&[3]));
        assert!(!digraph.is_walk(&[4]));
        assert!(!digraph.is_walk(&[5, 0]));
    }

    #[test]
    fn is_walk_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(digraph.is_walk(&[0, 1, 3, 5]));
        assert!(digraph.is_walk(&[0, 2, 1, 3, 5]));
        assert!(digraph.is_walk(&[0, 2, 3, 5]));
        assert!(digraph.is_walk(&[0, 2, 4, 6]));
        assert!(digraph.is_walk(&[0, 2, 5]));
        assert!(digraph.is_walk(&[1, 3, 5]));
        assert!(digraph.is_walk(&[2, 1, 3, 5]));
        assert!(digraph.is_walk(&[2, 3, 5]));
        assert!(digraph.is_walk(&[2, 4, 6]));
        assert!(digraph.is_walk(&[2, 5]));
        assert!(digraph.is_walk(&[3, 5]));
        assert!(digraph.is_walk(&[4, 6]));

        assert!(!digraph.is_walk(&[0, 3]));
        assert!(!digraph.is_walk(&[1, 0]));
        assert!(!digraph.is_walk(&[2, 0]));
        assert!(!digraph.is_walk(&[3, 0]));
        assert!(!digraph.is_walk(&[4, 0]));
        assert!(!digraph.is_walk(&[5]));
        assert!(!digraph.is_walk(&[6]));
    }

    #[test]
    fn is_walk_kattis_builddeps() {
        let digraph = kattis_builddeps();

        assert!(digraph.is_walk(&[0, 3, 1]));
        assert!(digraph.is_walk(&[0, 4, 1]));
        assert!(digraph.is_walk(&[2, 3, 1]));
        assert!(digraph.is_walk(&[2, 4, 1]));
        assert!(digraph.is_walk(&[2, 5, 1]));
        assert!(digraph.is_walk(&[3, 1]));
        assert!(digraph.is_walk(&[4, 1]));
        assert!(digraph.is_walk(&[5, 1]));

        assert!(!digraph.is_walk(&[0, 1]));
        assert!(!digraph.is_walk(&[1]));
        assert!(!digraph.is_walk(&[2, 0]));
        assert!(!digraph.is_walk(&[3, 0]));
        assert!(!digraph.is_walk(&[4, 0]));
        assert!(!digraph.is_walk(&[5, 0]));
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn is_walk_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(digraph.is_walk(&[5, 6, 5]));
        assert!(digraph.is_walk(&[5, 9, 5]));
        assert!(digraph.is_walk(&[5, 9, 13, 9]));
        assert!(digraph.is_walk(&[5, 9, 13, 12]));
        assert!(digraph.is_walk(&[6, 5, 6]));
        assert!(digraph.is_walk(&[6, 5, 9, 13, 9]));
        assert!(digraph.is_walk(&[6, 5, 9, 13, 12]));
        assert!(digraph.is_walk(&[9, 5, 6, 5]));
        assert!(digraph.is_walk(&[9, 5, 9]));
        assert!(digraph.is_walk(&[9, 13, 9]));
        assert!(digraph.is_walk(&[9, 13, 12]));
        assert!(digraph.is_walk(&[13, 9, 5, 6, 5]));
        assert!(digraph.is_walk(&[13, 9, 5, 9]));
        assert!(digraph.is_walk(&[13, 9, 13]));
        assert!(digraph.is_walk(&[13, 12]));

        assert!(!digraph.is_walk(&[0]));
        assert!(!digraph.is_walk(&[1]));
        assert!(!digraph.is_walk(&[2]));
        assert!(!digraph.is_walk(&[3]));
        assert!(!digraph.is_walk(&[4]));
        assert!(!digraph.is_walk(&[5, 0]));
        assert!(!digraph.is_walk(&[6, 0]));
        assert!(!digraph.is_walk(&[7]));
        assert!(!digraph.is_walk(&[8]));
        assert!(!digraph.is_walk(&[9, 0]));
        assert!(!digraph.is_walk(&[10]));
        assert!(!digraph.is_walk(&[11]));
        assert!(!digraph.is_walk(&[12]));
        assert!(!digraph.is_walk(&[13, 0]));
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn is_walk_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(digraph.is_walk(&[5, 6, 5]));
        assert!(digraph.is_walk(&[5, 9, 5]));
        assert!(digraph.is_walk(&[6, 5, 6]));
        assert!(digraph.is_walk(&[6, 5, 9, 5]));
        assert!(digraph.is_walk(&[9, 5, 6, 5]));
        assert!(digraph.is_walk(&[9, 5, 9]));
        assert!(digraph.is_walk(&[12, 13, 9, 5, 6, 5]));
        assert!(digraph.is_walk(&[12, 13, 9, 5, 9]));
        assert!(digraph.is_walk(&[12, 13, 12]));
        assert!(digraph.is_walk(&[13, 9, 5, 6, 5]));
        assert!(digraph.is_walk(&[13, 9, 5, 9]));
        assert!(digraph.is_walk(&[13, 12, 13]));

        assert!(!digraph.is_walk(&[0]));
        assert!(!digraph.is_walk(&[1]));
        assert!(!digraph.is_walk(&[2]));
        assert!(!digraph.is_walk(&[3]));
        assert!(!digraph.is_walk(&[4]));
        assert!(!digraph.is_walk(&[5, 0]));
        assert!(!digraph.is_walk(&[6, 0]));
        assert!(!digraph.is_walk(&[7]));
        assert!(!digraph.is_walk(&[8]));
        assert!(!digraph.is_walk(&[9, 0]));
        assert!(!digraph.is_walk(&[10]));
        assert!(!digraph.is_walk(&[11]));
        assert!(!digraph.is_walk(&[12, 0]));
        assert!(!digraph.is_walk(&[13, 0]));
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn is_walk_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(digraph.is_walk(&[1, 2, 1]));
        assert!(digraph.is_walk(&[1, 2, 6, 2]));
        assert!(digraph.is_walk(&[1, 2, 6, 5, 1]));
        assert!(digraph.is_walk(&[1, 2, 6, 5, 6]));
        assert!(digraph.is_walk(&[1, 2, 6, 5, 9, 5]));
        assert!(digraph.is_walk(&[1, 2, 6, 5, 9, 13]));
        assert!(digraph.is_walk(&[1, 2, 6, 5, 9, 13, 9]));
        assert!(digraph.is_walk(&[1, 2, 6, 5, 9, 13, 12, 13]));
        assert!(digraph.is_walk(&[1, 5, 1]));
        assert!(digraph.is_walk(&[1, 5, 6, 2, 1]));
        assert!(digraph.is_walk(&[1, 5, 6, 2, 6]));
        assert!(digraph.is_walk(&[1, 5, 9, 5]));
        assert!(digraph.is_walk(&[1, 5, 9, 13, 9]));
        assert!(digraph.is_walk(&[1, 5, 9, 13, 12, 13]));
        assert!(digraph.is_walk(&[2, 1, 2]));
        assert!(digraph.is_walk(&[2, 1, 5, 1]));
        assert!(digraph.is_walk(&[2, 1, 5, 6, 2]));
        assert!(digraph.is_walk(&[2, 1, 5, 6, 5]));
        assert!(digraph.is_walk(&[2, 1, 5, 9, 5]));
        assert!(digraph.is_walk(&[2, 1, 5, 9, 13, 9]));
        assert!(digraph.is_walk(&[2, 1, 5, 9, 13, 12, 13]));
        assert!(digraph.is_walk(&[2, 6, 2]));
        assert!(digraph.is_walk(&[2, 6, 5, 1, 2]));
        assert!(digraph.is_walk(&[2, 6, 5, 1, 5]));
        assert!(digraph.is_walk(&[2, 6, 5, 6]));
        assert!(digraph.is_walk(&[2, 6, 5, 9, 5]));
        assert!(digraph.is_walk(&[2, 6, 5, 9, 13, 9]));
        assert!(digraph.is_walk(&[2, 6, 5, 9, 13, 12, 13]));
        assert!(digraph.is_walk(&[5, 1, 2, 1]));
        assert!(digraph.is_walk(&[5, 1, 2, 6, 2]));
        assert!(digraph.is_walk(&[5, 1, 2, 6, 5]));
        assert!(digraph.is_walk(&[5, 1, 5]));
        assert!(digraph.is_walk(&[5, 6, 2, 1, 2]));
        assert!(digraph.is_walk(&[5, 6, 2, 1, 5]));
        assert!(digraph.is_walk(&[5, 6, 2, 6]));
        assert!(digraph.is_walk(&[5, 6, 5]));
        assert!(digraph.is_walk(&[5, 9, 5]));
        assert!(digraph.is_walk(&[5, 9, 13, 9]));
        assert!(digraph.is_walk(&[5, 9, 13, 12, 13]));
        assert!(digraph.is_walk(&[6, 2, 1, 2]));
        assert!(digraph.is_walk(&[6, 2, 1, 5, 1]));
        assert!(digraph.is_walk(&[6, 2, 1, 5, 6]));
        assert!(digraph.is_walk(&[6, 2, 1, 5, 9, 5]));
        assert!(digraph.is_walk(&[6, 2, 1, 5, 9, 13, 9]));
        assert!(digraph.is_walk(&[6, 2, 1, 5, 9, 13, 12, 13]));
        assert!(digraph.is_walk(&[6, 2, 6]));
        assert!(digraph.is_walk(&[6, 5, 1, 2, 1]));
        assert!(digraph.is_walk(&[6, 5, 1, 5]));
        assert!(digraph.is_walk(&[6, 5, 6]));
        assert!(digraph.is_walk(&[6, 5, 9, 5]));
        assert!(digraph.is_walk(&[6, 5, 9, 13, 9]));
        assert!(digraph.is_walk(&[6, 5, 9, 13, 12, 13]));
        assert!(digraph.is_walk(&[9, 5, 1, 2, 1]));
        assert!(digraph.is_walk(&[9, 5, 1, 2, 6, 2]));
        assert!(digraph.is_walk(&[9, 5, 1, 2, 6, 5]));
        assert!(digraph.is_walk(&[9, 5, 1, 5]));
        assert!(digraph.is_walk(&[9, 5, 6, 2, 1, 2]));
        assert!(digraph.is_walk(&[9, 5, 6, 2, 1, 5]));
        assert!(digraph.is_walk(&[9, 5, 6, 2, 6]));
        assert!(digraph.is_walk(&[9, 5, 6, 5]));
        assert!(digraph.is_walk(&[9, 5, 9]));
        assert!(digraph.is_walk(&[9, 13, 9]));
        assert!(digraph.is_walk(&[9, 13, 12, 13]));
        assert!(digraph.is_walk(&[12, 13, 9, 5, 1, 2, 1]));
        assert!(digraph.is_walk(&[12, 13, 9, 5, 1, 5]));
        assert!(digraph.is_walk(&[12, 13, 9, 5, 6, 2, 1, 2]));
        assert!(digraph.is_walk(&[12, 13, 9, 5, 6, 2, 1, 5]));
        assert!(digraph.is_walk(&[12, 13, 9, 5, 6, 2, 6]));
        assert!(digraph.is_walk(&[12, 13, 9, 5, 6, 5]));
        assert!(digraph.is_walk(&[12, 13, 9, 5, 9]));
        assert!(digraph.is_walk(&[12, 13, 9, 13]));
        assert!(digraph.is_walk(&[12, 13, 12]));

        assert!(!digraph.is_walk(&[0]));
        assert!(!digraph.is_walk(&[1, 0]));
        assert!(!digraph.is_walk(&[2, 0]));
        assert!(!digraph.is_walk(&[3]));
        assert!(!digraph.is_walk(&[4]));
        assert!(!digraph.is_walk(&[5, 0]));
        assert!(!digraph.is_walk(&[6, 0]));
        assert!(!digraph.is_walk(&[7]));
        assert!(!digraph.is_walk(&[8]));
        assert!(!digraph.is_walk(&[9, 0]));
        assert!(!digraph.is_walk(&[10]));
        assert!(!digraph.is_walk(&[11]));
        assert!(!digraph.is_walk(&[12, 0]));
        assert!(!digraph.is_walk(&[13, 0]));
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
    fn order_kattis_escapewallmaria_1() {
        assert_eq!(kattis_escapewallmaria_1().order(), 14);
    }

    #[test]
    fn order_kattis_escapewallmaria_2() {
        assert_eq!(kattis_escapewallmaria_2().order(), 14);
    }

    #[test]
    fn order_kattis_escapewallmaria_3() {
        assert_eq!(kattis_escapewallmaria_3().order(), 14);
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
    fn out_neighbors_weighted_bang_jensen_34() {
        let digraph = bang_jensen_34();

        assert!(digraph.out_neighbors_weighted(0).eq([(4, &1)]));
        assert!(digraph.out_neighbors_weighted(1).eq([(0, &1)]));

        assert!(digraph
            .out_neighbors_weighted(2)
            .eq([(1, &1), (3, &1), (5, &1)]));

        assert!(digraph.out_neighbors_weighted(3).eq([]));
        assert!(digraph.out_neighbors_weighted(4).eq([]));
        assert!(digraph.out_neighbors_weighted(5).eq([(4, &1)]));
    }

    #[test]
    fn out_neighbors_weighted_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(digraph.out_neighbors_weighted(0).eq([(1, &1), (2, &1)]));
        assert!(digraph.out_neighbors_weighted(1).eq([(3, &1)]));

        assert!(digraph
            .out_neighbors_weighted(2)
            .eq([(1, &1), (3, &1), (4, &1), (5, &1)]));

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

        assert!(digraph
            .out_neighbors_weighted(2)
            .eq([(3, &1), (4, &1), (5, &1)]));

        assert!(digraph.out_neighbors_weighted(3).eq([(1, &1)]));
        assert!(digraph.out_neighbors_weighted(4).eq([(1, &1)]));
        assert!(digraph.out_neighbors_weighted(5).eq([(1, &1)]));
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
        assert!(digraph.out_neighbors_weighted(13).eq([(9, &1), (12, &1)]));
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
        assert!(digraph.out_neighbors_weighted(13).eq([(9, &1), (12, &1)]));
    }

    #[test]
    fn out_neighbors_weighted_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(digraph.out_neighbors_weighted(0).eq([]));
        assert!(digraph.out_neighbors_weighted(1).eq([(2, &1), (5, &1)]));
        assert!(digraph.out_neighbors_weighted(2).eq([(1, &1), (6, &1)]));
        assert!(digraph.out_neighbors_weighted(3).eq([]));
        assert!(digraph.out_neighbors_weighted(4).eq([]));

        assert!(digraph
            .out_neighbors_weighted(5)
            .eq([(1, &1), (6, &1), (9, &1)]));

        assert!(digraph.out_neighbors_weighted(6).eq([(2, &1), (5, &1)]));
        assert!(digraph.out_neighbors_weighted(7).eq([]));
        assert!(digraph.out_neighbors_weighted(8).eq([]));
        assert!(digraph.out_neighbors_weighted(9).eq([(5, &1), (13, &1)]));
        assert!(digraph.out_neighbors_weighted(10).eq([]));
        assert!(digraph.out_neighbors_weighted(11).eq([]));
        assert!(digraph.out_neighbors_weighted(12).eq([(13, &1)]));
        assert!(digraph.out_neighbors_weighted(13).eq([(9, &1), (12, &1)]));
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
    }

    #[test]
    fn random_tournament_is_complete_trivial() {
        assert!(Digraph::random_tournament(1).is_complete());
    }

    #[test]
    fn random_tournament_is_isolated_trivial() {
        assert!(Digraph::random_tournament(1).is_isolated(0));
    }

    #[test]
    fn random_tournament_is_pendant_trivial() {
        assert!(!Digraph::random_tournament(1).is_pendant(0));
    }

    #[test]
    fn random_tournament_is_pendant_pair() {
        let digraph = Digraph::random_tournament(2);

        for u in digraph.vertices() {
            assert!(digraph.is_pendant(u));
        }
    }

    #[test]
    fn random_tournament_is_simple_trivial() {
        assert!(Digraph::random_tournament(1).is_simple());
    }

    #[test]
    fn remove_arc_bang_jensen_34() {
        let mut digraph = bang_jensen_34();

        assert!(digraph
            .arcs()
            .eq([(0, 4), (1, 0), (2, 1), (2, 3), (2, 5), (5, 4)]));

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
    fn remove_arc_kattis_escapewallmaria_1() {
        let mut digraph = kattis_escapewallmaria_1();

        assert!(digraph
            .arcs()
            .eq([(5, 6), (5, 9), (6, 5), (9, 5), (9, 13), (13, 9), (13, 12)]));

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

        assert!(digraph
            .arcs()
            .eq([(5, 6), (5, 9), (6, 5), (9, 5), (12, 13), (13, 9), (13, 12)]));

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
}
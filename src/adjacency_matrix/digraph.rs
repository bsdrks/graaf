//! An adjacency matrix representation for unweighted digraphs
//!
//! An adjacency matrix is a symmetric binary matrix where a value of `1` at
//! row `s` and column `t` indicates an arc from vertex `s` to vertex `t`. The
//! matrix is stored as a bit array, and is suited for dense digraphs with a
//! small number of vertices.

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

/// An adjacency matrix representation of an unweighted digraph
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Digraph {
    blocks: Vec<usize>,
    order: usize,
}

impl Digraph {
    const fn mask(i: usize) -> usize {
        1 << (i & 63)
    }

    const fn index(&self, s: usize, t: usize) -> usize {
        s * self.order + t
    }

    /// Toggles the arc from the source vertex to the target vertex.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    /// * `t`: The target vertex.
    ///
    /// # Panics
    ///
    /// Panics if `s` or `t` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_matrix::Digraph,
    ///     gen::Empty,
    ///     op::HasArc,
    /// };
    ///
    /// let mut digraph = Digraph::empty(3);
    ///
    /// assert!(!digraph.has_arc(0, 1));
    ///
    /// digraph.toggle(0, 1);
    ///
    /// assert!(digraph.has_arc(0, 1));
    /// ```
    pub fn toggle(&mut self, s: usize, t: usize) {
        let v = self.order;

        assert!(s < v, "s = {s} is out of bounds.");
        assert!(t < v, "t = {t} is out of bounds.");

        let i = self.index(s, t);

        self.blocks[i >> 6] ^= Self::mask(i);
    }
}

impl AddArc for Digraph {
    /// # Panics
    ///
    /// Panics if `s` or `t` is out of bounds.
    fn add_arc(&mut self, s: usize, t: usize) {
        let v = self.order;

        assert!(s < v, "s = {s} is out of bounds.");
        assert!(t < v, "t = {t} is out of bounds.");

        let i = self.index(s, t);

        self.blocks[i >> 6] |= Self::mask(i);
    }
}

impl ArcWeight<usize> for Digraph {
    fn arc_weight(&self, s: usize, t: usize) -> Option<&usize> {
        self.has_arc(s, t).then_some(&1)
    }
}

impl ArcsWeighted<usize> for Digraph {
    fn arcs_weighted<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a usize)>
    where
        usize: 'a,
    {
        self.arcs().map(|(s, t)| (s, t, &1))
    }
}

impl Converse for Digraph {
    fn converse(&self) -> Self {
        let v = self.order;
        let mut converse = Self::empty(v);

        for (s, t) in self.arcs() {
            converse.add_arc(t, s);
        }

        converse
    }
}

impl Empty for Digraph {
    /// # Panics
    ///
    /// Panics if `V` is zero.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a digraph must have at least one vertex");

        let n = (v * v + 63) / 64;

        Self {
            blocks: vec![0; n],
            order: v,
        }
    }
}

impl From<Vec<BTreeSet<usize>>> for Digraph {
    fn from(adjacency: Vec<BTreeSet<usize>>) -> Self {
        let v = adjacency.len();
        let mut digraph = Self::empty(v);

        for (s, neighbors) in adjacency.into_iter().enumerate() {
            for t in neighbors {
                digraph.add_arc(s, t);
            }
        }

        digraph
    }
}

impl HasArc for Digraph {
    /// # Panics
    ///
    /// Panics if `s` or `t` is out of bounds.
    fn has_arc(&self, s: usize, t: usize) -> bool {
        let v = self.order;

        assert!(s < v, "s = {s} is out of bounds.");
        assert!(t < v, "t = {t} is out of bounds.");

        let i = self.index(s, t);

        self.blocks[i >> 6] & Self::mask(i) != 0
    }
}

impl Indegree for Digraph {
    /// # Panics
    ///
    /// Panics if `t >= V`.
    fn indegree(&self, t: usize) -> usize {
        assert!(t < self.order, "t = {t} is out of bounds.");

        self.vertices().filter(|&s| self.has_arc(s, t)).count()
    }
}

impl IsSimple for Digraph {
    fn is_simple(&self) -> bool {
        self.vertices().all(|s| !self.has_arc(s, s))
    }
}

impl Arcs for Digraph {
    fn arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.vertices().flat_map(move |s| {
            self.vertices()
                .filter_map(move |t| self.has_arc(s, t).then_some((s, t)))
        })
    }
}

impl Order for Digraph {
    fn order(&self) -> usize {
        self.order
    }
}

impl OutNeighbors for Digraph {
    /// # Panics
    ///
    /// Panics if `s` is out of bounds.
    fn out_neighbors(&self, s: usize) -> impl Iterator<Item = usize> {
        assert!(s < self.order, "s = {s} is out of bounds.");

        self.vertices().filter(move |&t| self.has_arc(s, t))
    }
}

impl OutNeighborsWeighted<usize> for Digraph {
    /// # Panics
    ///
    /// Panics if `s` is out of bounds.
    fn out_neighbors_weighted<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a usize)>
    where
        usize: 'a,
    {
        self.out_neighbors(s).map(move |t| (t, &1))
    }
}

impl Outdegree for Digraph {
    /// # Panics
    ///
    /// Panics if `s >= V`.
    fn outdegree(&self, s: usize) -> usize {
        assert!(s < self.order, "s = {s} is out of bounds.");

        self.vertices().filter(|&t| self.has_arc(s, t)).count()
    }
}

impl RemoveArc for Digraph {
    /// # Panics
    ///
    /// Panics if `s >= V` or `t >= V`.
    fn remove_arc(&mut self, s: usize, t: usize) -> bool {
        let v = self.order;

        assert!(s < v, "s = {s} is out of bounds.");
        assert!(t < v, "t = {t} is out of bounds.");

        let has_arc = self.has_arc(s, t);
        let i = self.index(s, t);

        self.blocks[i >> 6] &= !Self::mask(i);

        has_arc
    }
}

impl Size for Digraph {
    /// # Panics
    ///
    /// Panics when the number of arcs is greater than `usize::MAX`.
    fn size(&self) -> usize {
        self.blocks
            .iter()
            .map(|&block| block.count_ones() as usize)
            .sum()
    }
}

impl Vertices for Digraph {
    fn vertices(&self) -> impl Iterator<Item = usize> {
        0..self.order
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            adjacency_matrix::fixture::{
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
                Empty,
                RandomTournament,
            },
            op::{
                Degree,
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
                Order,
            },
        },
        proptest::proptest,
    };

    proptest! {
        #[test]
        fn add_arc_arc_weight(s in 1..100_usize, t in 1..100_usize) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc(s, t);

            for u in digraph.vertices() {
                for v in digraph.vertices() {
                    assert_eq!(
                        digraph.arc_weight(u, v),
                        (u == s && v == t).then_some(&1)
                    );
                }
            }
        }

        #[test]
        fn add_arc_degree(s in 1..100_usize, t in 1..100_usize) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc(s, t);

            for u in digraph.vertices() {
                assert_eq!(
                    digraph.degree(u),
                    usize::from(u == s) + usize::from(u == t)
                );
            }
        }

        #[test]
        fn add_arc_has_arc(s in 1..100_usize, t in 1..100_usize) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc(s, t);

            assert!(digraph.has_arc(s, t));
        }

        #[test]
        fn add_arc_indegree(s in 1..100_usize, t in 1..100_usize) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc(s, t);

            for u in digraph.vertices() {
                assert_eq!(digraph.indegree(u), usize::from(u == t));
            }
        }

        #[test]
        fn add_arc_outdegree(s in 1..100_usize, t in 1..100_usize) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc(s, t);

            for u in digraph.vertices() {
                assert_eq!(digraph.outdegree(u), usize::from(u == s));
            }
        }

        #[test]
        fn add_arc_remove_arc(s in 1..100_usize, t in 1..100_usize) {
            let d = Digraph::empty(100);
            let mut h = d.clone();

            h.add_arc(s, t);

            for u in d.vertices() {
                for v in d.vertices() {
                    if u == s && v == t {
                        assert!(h.remove_arc(u, v));
                    } else {
                        assert!(!h.remove_arc(u, v));
                    }
                }
            }

            assert_eq!(d, h);
        }

        #[test]
        fn complete_degree(v in 1..100_usize) {
            let digraph = Digraph::complete(v);

            for s in digraph.vertices() {
                assert_eq!(digraph.degree(s), v * 2 - 2);
            }
        }

        #[test]
        fn complete_has_edge(v in 2..100_usize) {
            let digraph = Digraph::complete(v);

            for s in 0..v {
                for t in s + 1..v {
                    assert!(digraph.has_edge(s, t));
                }
            }
        }

        #[test]
        fn complete_indegree(v in 1..100_usize) {
            let digraph = Digraph::complete(v);

            for s in digraph.vertices() {
                assert_eq!(digraph.indegree(s), v - 1);
            }
        }

        #[test]
        fn complete_is_balanced(v in 1..100_usize) {
            assert!(Digraph::complete(v).is_balanced());
        }

        #[test]
        fn complete_is_complete(v in 1..100_usize) {
            assert!(Digraph::complete(v).is_complete());
        }

        #[test]
        fn complete_is_isolated(v in 2..100_usize) {
            let digraph = Digraph::complete(v);

            for s in digraph.vertices() {
                assert!(!digraph.is_isolated(s));
            }
        }

        #[test]
        fn complete_is_oriented(v in 2..100_usize) {
            assert!(!Digraph::complete(v).is_oriented());
        }

        #[test]
        fn complete_is_pendant(v in 1..100_usize) {
            let digraph = Digraph::complete(v);

            for s in digraph.vertices() {
                assert!(!digraph.is_pendant(s));
            }
        }

        #[test]
        fn complete_is_regular(v in 1..100_usize) {
            assert!(Digraph::complete(v).is_regular());
        }

        #[test]
        fn complete_is_semicomplete(v in 1..100_usize) {
            assert!(Digraph::complete(v).is_semicomplete());
        }

        #[test]
        fn complete_is_simple(v in 2..100_usize) {
            assert!(Digraph::complete(v).is_simple());
        }

        #[test]
        fn complete_is_subdigraph(v in 1..100_usize) {
            let digraph = Digraph::complete(v);

            assert!(digraph.is_subdigraph(&digraph));
        }

        #[test]
        fn complete_is_superdigraph(v in 1..100_usize) {
            let digraph = Digraph::complete(v);

            assert!(digraph.is_superdigraph(&digraph));
        }

        #[test]
        fn complete_is_symmetric(v in 1..100_usize) {
            assert!(Digraph::complete(v).is_symmetric());
        }

        #[test]
        fn complete_order(v in 1..100_usize) {
            assert_eq!(Digraph::complete(v).order(), v);
        }

        #[test]
        fn complete_outdegree(v in 1..100_usize) {
            let digraph = Digraph::complete(v);

            for s in digraph.vertices() {
                assert_eq!(digraph.outdegree(s), v - 1);
            }
        }

        #[test]
        fn complete_size(v in 1..100_usize) {
            assert_eq!(Digraph::complete(v).size(), v * (v - 1));
        }

        #[test]
        fn cycle_degree(v in 1..100_usize) {
            let digraph = Digraph::cycle(v);

            for s in digraph.vertices() {
                assert_eq!(digraph.degree(s), 2);
            }
        }

        #[test]
        fn cycle_has_edge(v in 3..100_usize) {
            let digraph = Digraph::cycle(v);

            for s in 0..v {
                for t in s + 1..v {
                    assert!(!digraph.has_edge(s, t));
                }
            }
        }

        #[test]
        fn cycle_indegree(v in 1..100_usize) {
            let digraph = Digraph::cycle(v);

            for s in digraph.vertices() {
                assert_eq!(digraph.indegree(s), 1);
            }
        }

        #[test]
        fn cycle_is_balanced(v in 1..100_usize) {
            assert!(Digraph::cycle(v).is_balanced());
        }

        #[test]
        fn cycle_is_complete(v in 3..100_usize) {
            assert!(!Digraph::cycle(v).is_complete());
        }

        #[test]
        fn cycle_is_isolated(v in 1..100_usize) {
            let digraph = Digraph::cycle(v);

            for s in digraph.vertices() {
                assert!(!digraph.is_isolated(s));
            }
        }

        #[test]
        fn cycle_is_oriented(v in 3..100_usize) {
            assert!(Digraph::cycle(v).is_oriented());
        }

        #[test]
        fn cycle_is_pendant(v in 1..100_usize) {
            let digraph = Digraph::cycle(v);

            for s in digraph.vertices() {
                assert!(!digraph.is_pendant(s));
            }
        }

        #[test]
        fn cycle_is_regular(v in 1..100_usize) {
            assert!(Digraph::cycle(v).is_regular());
        }

        #[test]
        fn cycle_is_semicomplete(v in 4..100_usize) {
            assert!(!Digraph::cycle(v).is_semicomplete());
        }

        #[test]
        fn cycle_is_simple(v in 2..100_usize) {
            assert!(Digraph::cycle(v).is_simple());
        }

        #[test]
        fn cycle_is_sink(v in 1..100_usize) {
            let digraph = Digraph::cycle(v);

            for s in digraph.vertices() {
                assert!(!digraph.is_sink(s));
            }
        }

        #[test]
        fn cycle_is_source(v in 1..100_usize) {
            let digraph = Digraph::cycle(v);

            for s in digraph.vertices() {
                assert!(!digraph.is_source(s));
            }
        }

        #[test]
        fn cycle_is_subdigraph(v in 1..100_usize) {
            let digraph = Digraph::cycle(v);

            assert!(digraph.is_subdigraph(&digraph));
        }

        #[test]
        fn cycle_is_superdigraph(v in 1..100_usize) {
            let digraph = Digraph::cycle(v);

            assert!(digraph.is_superdigraph(&digraph));
        }

        #[test]
        fn cycle_is_symmetric(v in 3..100_usize) {
            assert!(!Digraph::cycle(v).is_symmetric());
        }

        #[test]
        fn empty_arcs(v in 1..100_usize) {
            assert!(Digraph::empty(v).arcs().eq([]));
        }

        #[test]
        fn empty_degree(v in 1..100_usize) {
            let digraph = Digraph::empty(v);

            for s in digraph.vertices() {
                assert_eq!(digraph.degree(s), 0);
            }
        }

        #[test]
        fn empty_has_arc(v in 1..100_usize) {
            let digraph = Digraph::empty(v);

            for s in digraph.vertices() {
                for t in digraph.vertices() {
                    assert!(!digraph.has_arc(s, t));
                }
            }
        }

        #[test]
        fn empty_has_edge(v in 1..100_usize) {
            let digraph = Digraph::empty(v);

            for s in digraph.vertices() {
                for t in digraph.vertices() {
                    assert!(!digraph.has_edge(s, t));
                }
            }
        }

        #[test]
        fn empty_indegree(v in 1..100_usize) {
            let digraph = Digraph::empty(v);

            for s in digraph.vertices() {
                assert_eq!(digraph.indegree(s), 0);
            }
        }

        #[test]
        fn empty_is_balanced(v in 1..100_usize) {
            assert!(Digraph::empty(v).is_balanced());
        }

        #[test]
        fn empty_is_complete(v in 2..100_usize) {
            assert!(!Digraph::empty(v).is_complete());
        }

        #[test]
        fn empty_is_isolated(v in 1..100_usize) {
            let digraph = Digraph::empty(v);

            for s in digraph.vertices() {
                assert!(digraph.is_isolated(s));
            }
        }

        #[test]
        fn empty_is_oriented(v in 1..100_usize) {
            assert!(Digraph::empty(v).is_oriented());
        }

        #[test]
        fn empty_is_pendant(v in 1..100_usize) {
            let digraph = Digraph::empty(v);

            for s in digraph.vertices() {
                assert!(!digraph.is_pendant(s));
            }
        }

        #[test]
        fn empty_is_regular(v in 1..100_usize) {
            assert!(Digraph::empty(v).is_regular());
        }

        #[test]
        fn empty_is_semicomplete(v in 2..100_usize) {
            assert!(!Digraph::empty(v).is_semicomplete());
        }

        #[test]
        fn empty_is_simple(v in 1..100_usize) {
            assert!(Digraph::empty(v).is_simple());
        }

        #[test]
        fn empty_is_sink(v in 1..100_usize) {
            let digraph = Digraph::empty(v);

            for s in digraph.vertices() {
                assert!(digraph.is_sink(s));
            }
        }

        #[test]
        fn empty_is_source(v in 1..100_usize) {
            let digraph = Digraph::empty(v);

            for s in digraph.vertices() {
                assert!(digraph.is_source(s));
            }
        }

        #[test]
        fn empty_is_subdigraph(v in 1..100_usize) {
            let digraph = Digraph::empty(v);

            assert!(digraph.is_subdigraph(&digraph));
        }

        #[test]
        fn empty_is_superdigraph(v in 1..100_usize) {
            let digraph = Digraph::empty(v);

            assert!(digraph.is_superdigraph(&digraph));
        }

        #[test]
        fn empty_is_symmetric(v in 1..100_usize) {
            assert!(Digraph::empty(v).is_symmetric());
        }

        #[test]
        fn empty_outdegree(v in 1..100_usize) {
            let digraph = Digraph::empty(v);

            for s in digraph.vertices() {
                assert_eq!(digraph.outdegree(s), 0);
            }
        }

        #[test]
        fn random_tournament_degree(v in 1..100_usize) {
            let digraph = Digraph::random_tournament(v);

            for s in digraph.vertices() {
                assert_eq!(digraph.degree(s), v - 1);
            }
        }

        #[test]
        fn random_tournament_has_edge(v in 1..100_usize) {
            let digraph = Digraph::random_tournament(v);

            for s in digraph.vertices() {
                for t in digraph.vertices() {
                    assert!(!digraph.has_edge(s, t));
                }
            }
        }

        #[test]
        fn random_tournament_indegree(v in 1..100_usize) {
            let digraph = Digraph::random_tournament(v);

            for s in digraph.vertices() {
                assert!((0..v).contains(&digraph.indegree(s)));
            }
        }

        #[test]
        fn random_tournament_is_complete(v in 2..100_usize) {
            assert!(!Digraph::random_tournament(v).is_complete());
        }

        #[test]
        fn random_tournament_is_isolated(v in 2..100_usize) {
            let digraph = Digraph::random_tournament(v);

            for s in digraph.vertices() {
                assert!(!digraph.is_isolated(s));
            }
        }

        #[test]
        fn random_tournament_is_oriented(v in 1..100_usize) {
            assert!(Digraph::random_tournament(v).is_oriented());
        }

        #[test]
        fn random_tournament_is_pendant(v in 3..100_usize) {
            let digraph = Digraph::random_tournament(v);

            for s in digraph.vertices() {
                assert!(!digraph.is_pendant(s));
            }
        }

        #[test]
        fn random_tournament_is_semicomplete(v in 1..100_usize) {
            assert!(Digraph::random_tournament(v).is_semicomplete());
        }

        #[test]
        fn random_tournament_is_simple(v in 1..100_usize) {
            assert!(Digraph::random_tournament(v).is_simple());
        }

        #[test]
        fn random_tournament_is_subdigraph(v in 1..100_usize) {
            let digraph = Digraph::random_tournament(v);

            assert!(digraph.is_subdigraph(&digraph));
        }

        #[test]
        fn random_tournament_is_superdigraph(v in 1..100_usize) {
            let digraph = Digraph::random_tournament(v);

            assert!(digraph.is_superdigraph(&digraph));
        }

        #[test]
        fn random_tournament_is_symmetric(v in 2..100_usize) {
            assert!(!Digraph::random_tournament(v).is_symmetric());
        }

        #[test]
        fn random_tournament_order(v in 1..100_usize) {
            assert_eq!(Digraph::random_tournament(v).order(), v);
        }

        #[test]
        fn random_tournament_outdegree(v in 1..100_usize) {
            let digraph = Digraph::random_tournament(v);

            for s in digraph.vertices() {
                assert!((0..v).contains(&digraph.outdegree(s)));
            }
        }

        #[test]
        fn random_tournament_size(v in 1..100_usize) {
            let digraph = Digraph::random_tournament(v);

            assert_eq!(digraph.size(), v * (v - 1) / 2);
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

        for s in 0..2 {
            for t in s + 1..2 {
                assert!(digraph.has_edge(s, t));
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

        for s in digraph.vertices() {
            assert!(digraph.is_pendant(s));
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

    #[test]
    fn toggle() {
        let mut digraph = Digraph::empty(3);

        digraph.toggle(0, 1);
        digraph.toggle(0, 2);

        assert_eq!(digraph.blocks, [0b110]);
    }

    #[test]
    #[should_panic(expected = "s = 3 is out of bounds.")]
    fn toggle_panic_s_gte_v() {
        let mut digraph = Digraph::empty(3);

        digraph.toggle(3, 0);
    }

    #[test]
    #[should_panic(expected = "t = 3 is out of bounds.")]
    fn toggle_panic_t_gte_v() {
        let mut digraph = Digraph::empty(3);

        digraph.toggle(0, 3);
    }
}

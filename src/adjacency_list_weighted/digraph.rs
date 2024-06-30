//! An adjacency list representation of a weighted digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list_weighted::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArcWeighted,
//!         IsSimple,
//!     },
//! };
//!
//! let mut digraph = Digraph::<isize>::empty(3);
//!
//! digraph.add_arc_weighted(0, 1, 2);
//!
//! assert!(digraph.is_simple());
//!
//! digraph.add_arc_weighted(1, 1, 3);
//!
//! assert!(!digraph.is_simple());
//! ```

use {
    crate::{
        gen::Empty,
        op::{
            AddArcWeighted,
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
    std::collections::BTreeMap,
};

/// An adjacency list representation of an unweighted digraph
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Digraph<W> {
    /// The edges of the digraph
    arcs: Vec<BTreeMap<usize, W>>,
}

impl<W> AddArcWeighted<W> for Digraph<W> {
    fn add_arc_weighted(&mut self, s: usize, t: usize, w: W) {
        let _ = self.arcs[s].insert(t, w);
    }
}

impl<W> ArcWeight<W> for Digraph<W> {
    fn arc_weight(&self, s: usize, t: usize) -> Option<&W> {
        self.arcs.get(s).and_then(|arcs| arcs.get(&t))
    }
}

impl<W> Arcs for Digraph<W> {
    fn arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.arcs
            .iter()
            .enumerate()
            .flat_map(|(s, set)| set.iter().map(move |(t, _)| (s, *t)))
    }
}

impl<W> ArcsWeighted<W> for Digraph<W> {
    fn arcs_weighted<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.arcs
            .iter()
            .enumerate()
            .flat_map(|(s, arcs)| arcs.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<W> Converse for Digraph<W>
where
    W: Copy,
{
    fn converse(&self) -> Self {
        let v = self.order();
        let mut converse = Self::empty(v);

        for (s, t, w) in self.arcs_weighted() {
            converse.add_arc_weighted(t, s, *w);
        }

        converse
    }
}

impl<W> Empty for Digraph<W>
where
    W: Clone,
{
    /// # Panics
    ///
    /// Panics if `v` is zero.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a digraph must have at least one vertex");

        Self {
            arcs: vec![BTreeMap::new(); v],
        }
    }
}

impl<W> From<Vec<BTreeMap<usize, W>>> for Digraph<W> {
    fn from(arcs: Vec<BTreeMap<usize, W>>) -> Self {
        Self { arcs }
    }
}

impl<W> HasArc for Digraph<W> {
    fn has_arc(&self, s: usize, t: usize) -> bool {
        self.arcs[s].contains_key(&t)
    }
}

impl<W> Indegree for Digraph<W> {
    fn indegree(&self, t: usize) -> usize {
        self.arcs
            .iter()
            .filter(|arcs| arcs.contains_key(&t))
            .count()
    }
}

impl<W> IsSimple for Digraph<W> {
    fn is_simple(&self) -> bool {
        self.arcs
            .iter()
            .enumerate()
            .all(|(s, map)| !map.contains_key(&s))
    }
}

impl<W> OutNeighbors for Digraph<W> {
    /// # Panics
    ///
    /// Panics if `s` is out of bounds.
    fn out_neighbors(&self, s: usize) -> impl Iterator<Item = usize> {
        self.arcs[s].keys().copied()
    }
}

impl<W> OutNeighborsWeighted<W> for Digraph<W> {
    /// # Panics
    ///
    /// Panics if `s` is out of bounds.
    fn out_neighbors_weighted<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self.arcs[s].iter().map(|(t, w)| (*t, w))
    }
}

impl<W> Order for Digraph<W> {
    fn order(&self) -> usize {
        self.arcs.len()
    }
}

impl<W> Outdegree for Digraph<W> {
    /// # Panics
    ///
    /// Panics if `s` is out of bounds.
    fn outdegree(&self, s: usize) -> usize {
        self.arcs[s].len()
    }
}

impl<W> RemoveArc for Digraph<W> {
    fn remove_arc(&mut self, s: usize, t: usize) -> bool {
        self.arcs
            .get_mut(s)
            .map_or(false, |set| set.remove(&t).is_some())
    }
}

impl<W> Size for Digraph<W> {
    fn size(&self) -> usize {
        self.arcs.iter().map(BTreeMap::len).sum()
    }
}

impl<W> Vertices for Digraph<W> {
    fn vertices(&self) -> impl Iterator<Item = usize> {
        0..self.order()
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            adjacency_list_weighted::fixture::{
                bang_jensen_94_weighted,
                bang_jensen_96,
                bang_jensen_99,
                kattis_bryr_1,
                kattis_bryr_2,
                kattis_bryr_3,
                kattis_crosscountry,
                kattis_shortestpath1,
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
                IsSubdigraph,
                IsSuperdigraph,
                IsSymmetric,
            },
        },
        proptest::proptest,
    };

    proptest! {
        #[test]
        fn add_arc_weighted_arc_weight(s in 1..100_usize, t in 1..100_usize, w in 1..100_usize) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc_weighted(s, t, w);

            for u in digraph.vertices() {
                for v in digraph.vertices() {
                    assert_eq!(
                        digraph.arc_weight(u, v),
                        (u == s && v == t).then_some(&w)
                    );
                }
            }
        }

        #[test]
        fn add_arc_weighted_degree(s in 1..100_usize, t in 1..100_usize, w in 1..100_usize) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc_weighted(s, t, w);

            for u in digraph.vertices() {
                assert_eq!(digraph.degree(u), usize::from(u == s) + usize::from(u == t));
            }
        }

        #[test]
        fn add_arc_weighted_has_arc(s in 1..100_usize, t in 1..100_usize, w in 1..100_usize) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc_weighted(s, t, w);

            assert!(digraph.has_arc(s, t));
        }

        #[test]
        fn add_arc_weighted_indegree(s in 1..100_usize, t in 1..100_usize, w in 1..100_usize) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc_weighted(s, t, w);

            for u in digraph.vertices() {
                assert_eq!(digraph.indegree(u), usize::from(u == t));
            }
        }

        #[test]
        fn add_arc_weighted_outdegree(s in 1..100_usize, t in 1..100_usize, w in 1..100_usize) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc_weighted(s, t, w);

            for u in digraph.vertices() {
                assert_eq!(digraph.outdegree(u), usize::from(u == s));
            }
        }

        #[test]
        fn add_arc_weighted_remove_arc(s in 1..100_usize, t in 1..100_usize, w in 1..100_usize) {
            let d = Digraph::empty(100);
            let mut h = d.clone();

            h.add_arc_weighted(s, t, w);

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
        fn empty_arcs(v in 1..100_usize) {
            assert!(Digraph::<usize>::empty(v).arcs().eq([]));
        }

        #[test]
        fn empty_degree(v in 1..100_usize) {
            let digraph = Digraph::<usize>::empty(v);

            for u in digraph.vertices() {
                assert_eq!(digraph.degree(u), 0);
            }
        }

        #[test]
        fn empty_has_arc(v in 1..100_usize) {
            let digraph = Digraph::<usize>::empty(v);

            for u in digraph.vertices() {
                for v in digraph.vertices() {
                    assert!(!digraph.has_arc(u, v));
                }
            }
        }

        #[test]
        fn empty_has_edge(v in 1..100_usize) {
            let digraph = Digraph::<usize>::empty(v);

            for s in digraph.vertices() {
                for t in digraph.vertices() {
                    assert!(!digraph.has_edge(s, t));
                }
            }
        }

        #[test]
        fn empty_indegree(v in 1..100_usize) {
            let digraph = Digraph::<usize>::empty(v);

            for u in digraph.vertices() {
                assert_eq!(digraph.indegree(u), 0);
            }
        }

        #[test]
        fn empty_is_balanced(v in 1..100_usize) {
            assert!(Digraph::<usize>::empty(v).is_balanced());
        }

        #[test]
        fn empty_is_complete(v in 2..100_usize) {
            assert!(!Digraph::<usize>::empty(v).is_complete());
        }

        #[test]
        fn empty_is_isolated(v in 1..100_usize) {
            let digraph = Digraph::<usize>::empty(v);

            for u in digraph.vertices() {
                assert!(digraph.is_isolated(u));
            }
        }

        #[test]
        fn empty_is_oriented(v in 1..100_usize) {
            assert!(Digraph::<usize>::empty(v).is_oriented());
        }

        #[test]
        fn empty_is_pendant(v in 1..100_usize) {
            let digraph = Digraph::<usize>::empty(v);

            for u in digraph.vertices() {
                assert!(!digraph.is_pendant(u));
            }
        }

        #[test]
        fn empty_is_regular(v in 1..100_usize) {
            assert!(Digraph::<usize>::empty(v).is_regular());
        }

        #[test]
        fn empty_is_semicomplete(v in 2..100_usize) {
            assert!(!Digraph::<usize>::empty(v).is_semicomplete());
        }

        #[test]
        fn empty_is_simple(v in 1..100_usize) {
            assert!(Digraph::<usize>::empty(v).is_simple());
        }

        #[test]
        fn empty_is_sink(v in 1..100_usize) {
            let digraph = Digraph::<usize>::empty(v);

            for u in digraph.vertices() {
                assert!(digraph.is_sink(u));
            }
        }

        #[test]
        fn empty_is_source(v in 1..100_usize) {
            let digraph = Digraph::<usize>::empty(v);

            for u in digraph.vertices() {
                assert!(digraph.is_source(u));
            }
        }

        #[test]
        fn empty_is_subdigraph(v in 1..100_usize) {
            let digraph = Digraph::<usize>::empty(v);

            assert!(digraph.is_subdigraph(&digraph));
        }

        #[test]
        fn empty_is_superdigraph(v in 1..100_usize) {
            let digraph = Digraph::<usize>::empty(v);

            assert!(digraph.is_superdigraph(&digraph));
        }

        #[test]
        fn empty_is_symmetric(v in 1..100_usize) {
            assert!(Digraph::<usize>::empty(v).is_symmetric());
        }

        #[test]
        fn empty_outdegree(v in 1..100_usize) {
            let digraph = Digraph::<usize>::empty(v);

            for u in digraph.vertices() {
                assert_eq!(digraph.outdegree(u), 0);
            }
        }
    }

    #[test]
    fn arcs_bang_jensen_94_weighted() {
        assert!(bang_jensen_94_weighted!().arcs().eq([
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
    fn arcs_bang_jensen_96() {
        assert!(bang_jensen_96!().arcs().eq([
            (0, 1),
            (0, 2),
            (1, 2),
            (1, 3),
            (2, 1),
            (2, 4),
            (3, 5),
            (4, 2),
            (4, 3),
            (4, 5),
            (5, 3)
        ]));
    }

    #[test]
    fn arcs_bang_jensen_99() {
        assert!(bang_jensen_99!().arcs().eq([
            (0, 1),
            (0, 2),
            (1, 2),
            (2, 3),
            (2, 4),
            (3, 5),
            (4, 3),
            (4, 5),
            (5, 3),
            (5, 4)
        ]));
    }

    #[test]
    fn arcs_kattis_bryr_1() {
        assert!(kattis_bryr_1!()
            .arcs()
            .eq([(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)]));
    }

    #[test]
    fn arcs_kattis_bryr_2() {
        assert!(kattis_bryr_2!().arcs().eq([
            (0, 1),
            (0, 3),
            (1, 0),
            (1, 2),
            (2, 1),
            (2, 3),
            (3, 0),
            (3, 2),
            (3, 4),
            (4, 3),
            (4, 5),
            (5, 4)
        ]));
    }

    #[test]
    fn arcs_kattis_bryr_3() {
        assert!(kattis_bryr_3!().arcs().eq([
            (0, 3),
            (1, 7),
            (1, 9),
            (2, 6),
            (3, 0),
            (3, 4),
            (3, 5),
            (3, 7),
            (4, 3),
            (4, 6),
            (4, 8),
            (5, 3),
            (5, 6),
            (5, 8),
            (6, 2),
            (6, 4),
            (6, 5),
            (6, 9),
            (7, 1),
            (7, 3),
            (8, 4),
            (8, 5),
            (9, 1),
            (9, 2),
            (9, 6)
        ]));
    }

    #[test]
    fn arcs_kattis_crosscountry() {
        assert!(kattis_crosscountry!().arcs().eq([
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
    fn arcs_kattis_shortestpath1() {
        assert!(kattis_shortestpath1!().arcs().eq([(0, 1), (1, 2), (3, 0)]));
    }

    #[test]
    fn arcs_weighted_bang_jensen_94_weighted() {
        assert!(bang_jensen_94_weighted!().arcs_weighted().eq([
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
    fn arcs_weighted_bang_jensen_96() {
        assert!(bang_jensen_96!().arcs_weighted().eq([
            (0, 1, &9),
            (0, 2, &3),
            (1, 2, &6),
            (1, 3, &2),
            (2, 1, &2),
            (2, 4, &1),
            (3, 5, &1),
            (4, 2, &2),
            (4, 3, &2),
            (4, 5, &7),
            (5, 3, &2)
        ]));
    }

    #[test]
    fn arcs_weighted_bang_jensen_99() {
        assert!(bang_jensen_99!().arcs_weighted().eq([
            (0, 1, &8),
            (0, 2, &4),
            (1, 2, &-5),
            (2, 3, &-2),
            (2, 4, &4),
            (3, 5, &-2),
            (4, 3, &10),
            (4, 5, &9),
            (5, 3, &5),
            (5, 4, &-3)
        ]));
    }

    #[test]
    fn arcs_weighted_kattis_bryr_1() {
        assert!(kattis_bryr_1!().arcs_weighted().eq([
            (0, 1, &1),
            (0, 2, &1),
            (1, 0, &1),
            (1, 2, &1),
            (2, 0, &1),
            (2, 1, &1)
        ]));
    }

    #[test]
    fn arcs_weighted_kattis_bryr_2() {
        assert!(kattis_bryr_2!().arcs_weighted().eq([
            (0, 1, &1),
            (0, 3, &1),
            (1, 0, &1),
            (1, 2, &1),
            (2, 1, &1),
            (2, 3, &1),
            (3, 0, &1),
            (3, 2, &1),
            (3, 4, &1),
            (4, 3, &1),
            (4, 5, &1),
            (5, 4, &1)
        ]));
    }

    #[test]
    fn arcs_weighted_kattis_bryr_3() {
        assert!(kattis_bryr_3!().arcs_weighted().eq([
            (0, 3, &0),
            (1, 7, &0),
            (1, 9, &1),
            (2, 6, &0),
            (3, 0, &0),
            (3, 4, &0),
            (3, 5, &0),
            (3, 7, &0),
            (4, 3, &0),
            (4, 6, &1),
            (4, 8, &1),
            (5, 3, &0),
            (5, 6, &1),
            (5, 8, &0),
            (6, 2, &0),
            (6, 4, &1),
            (6, 5, &1),
            (6, 9, &1),
            (7, 1, &0),
            (7, 3, &0),
            (8, 4, &1),
            (8, 5, &0),
            (9, 1, &1),
            (9, 2, &0),
            (9, 6, &1)
        ]));
    }

    #[test]
    fn arcs_weighted_kattis_crosscountry() {
        assert!(kattis_crosscountry!().arcs_weighted().eq([
            (0, 1, &1),
            (0, 2, &3),
            (0, 3, &14),
            (1, 0, &2),
            (1, 2, &4),
            (1, 3, &22),
            (2, 0, &3),
            (2, 1, &10),
            (2, 3, &7),
            (3, 0, &13),
            (3, 1, &8),
            (3, 2, &2)
        ]));
    }

    #[test]
    fn arcs_weighted_kattis_shortestpath1() {
        assert!(kattis_shortestpath1!()
            .arcs_weighted()
            .eq([(0, 1, &2), (1, 2, &2), (3, 0, &2)]));
    }

    #[test]
    fn converse_bang_jensen_94_weighted() {
        assert!(bang_jensen_94_weighted!().converse().arcs_weighted().eq([
            (1, 0, &1),
            (1, 2, &1),
            (2, 0, &1),
            (3, 1, &1),
            (3, 2, &1),
            (4, 2, &1),
            (5, 2, &1),
            (5, 3, &1),
            (6, 4, &1)
        ]));
    }

    #[test]
    fn converse_bang_jensen_96() {
        assert!(bang_jensen_96!().converse().arcs_weighted().eq([
            (1, 0, &9),
            (1, 2, &2),
            (2, 0, &3),
            (2, 1, &6),
            (2, 4, &2),
            (3, 1, &2),
            (3, 4, &2),
            (3, 5, &2),
            (4, 2, &1),
            (5, 3, &1),
            (5, 4, &7)
        ]));
    }

    #[test]
    fn converse_bang_jensen_99() {
        assert!(bang_jensen_99!().converse().arcs_weighted().eq([
            (1, 0, &8),
            (2, 0, &4),
            (2, 1, &-5),
            (3, 2, &-2),
            (3, 4, &10),
            (3, 5, &5),
            (4, 2, &4),
            (4, 5, &-3),
            (5, 3, &-2),
            (5, 4, &9)
        ]));
    }

    #[test]
    fn converse_kattis_bryr_1() {
        assert!(kattis_bryr_1!().converse().arcs_weighted().eq([
            (0, 1, &1),
            (0, 2, &1),
            (1, 0, &1),
            (1, 2, &1),
            (2, 0, &1),
            (2, 1, &1)
        ]));
    }

    #[test]
    fn converse_kattis_bryr_2() {
        assert!(kattis_bryr_2!().converse().arcs_weighted().eq([
            (0, 1, &1),
            (0, 3, &1),
            (1, 0, &1),
            (1, 2, &1),
            (2, 1, &1),
            (2, 3, &1),
            (3, 0, &1),
            (3, 2, &1),
            (3, 4, &1),
            (4, 3, &1),
            (4, 5, &1),
            (5, 4, &1)
        ]));
    }

    #[test]
    fn converse_kattis_bryr_3() {
        assert!(kattis_bryr_3!().converse().arcs_weighted().eq([
            (0, 3, &0),
            (1, 7, &0),
            (1, 9, &1),
            (2, 6, &0),
            (2, 9, &0),
            (3, 0, &0),
            (3, 4, &0),
            (3, 5, &0),
            (3, 7, &0),
            (4, 3, &0),
            (4, 6, &1),
            (4, 8, &1),
            (5, 3, &0),
            (5, 6, &1),
            (5, 8, &0),
            (6, 2, &0),
            (6, 4, &1),
            (6, 5, &1),
            (6, 9, &1),
            (7, 1, &0),
            (7, 3, &0),
            (8, 4, &1),
            (8, 5, &0),
            (9, 1, &1),
            (9, 6, &1)
        ]));
    }

    #[test]
    fn converse_kattis_crosscountry() {
        assert!(kattis_crosscountry!().converse().arcs_weighted().eq([
            (0, 1, &2),
            (0, 2, &3),
            (0, 3, &13),
            (1, 0, &1),
            (1, 2, &10),
            (1, 3, &8),
            (2, 0, &3),
            (2, 1, &4),
            (2, 3, &2),
            (3, 0, &14),
            (3, 1, &22),
            (3, 2, &7)
        ]));
    }

    #[test]
    fn converse_kattis_shortestpath1() {
        assert!(kattis_shortestpath1!().converse().arcs_weighted().eq([
            (0, 3, &2),
            (1, 0, &2),
            (2, 1, &2)
        ]));
    }

    #[test]
    fn degree_bang_jensen_94_weighted() {
        let digraph = bang_jensen_94_weighted!();

        assert!(digraph.degree(0) == 2);
        assert!(digraph.degree(1) == 3);
        assert!(digraph.degree(2) == 5);
        assert!(digraph.degree(3) == 3);
        assert!(digraph.degree(4) == 2);
        assert!(digraph.degree(5) == 2);
        assert!(digraph.degree(6) == 1);
    }

    #[test]
    fn degree_bang_jensen_96() {
        let digraph = bang_jensen_96!();

        assert!(digraph.degree(0) == 2);
        assert!(digraph.degree(1) == 4);
        assert!(digraph.degree(2) == 5);
        assert!(digraph.degree(3) == 4);
        assert!(digraph.degree(4) == 4);
        assert!(digraph.degree(5) == 3);
    }

    #[test]
    fn degree_bang_jensen_99() {
        let digraph = bang_jensen_99!();

        assert!(digraph.degree(0) == 2);
        assert!(digraph.degree(1) == 2);
        assert!(digraph.degree(2) == 4);
        assert!(digraph.degree(3) == 4);
        assert!(digraph.degree(4) == 4);
        assert!(digraph.degree(5) == 4);
    }

    #[test]
    fn degree_kattis_bryr_1() {
        let digraph = kattis_bryr_1!();

        assert!(digraph.degree(0) == 4);
        assert!(digraph.degree(1) == 4);
        assert!(digraph.degree(2) == 4);
    }

    #[test]
    fn degree_kattis_bryr_2() {
        let digraph = kattis_bryr_2!();

        assert!(digraph.degree(0) == 4);
        assert!(digraph.degree(1) == 4);
        assert!(digraph.degree(2) == 4);
        assert!(digraph.degree(3) == 6);
        assert!(digraph.degree(4) == 4);
        assert!(digraph.degree(5) == 2);
    }

    #[test]
    fn degree_kattis_bryr_3() {
        let digraph = kattis_bryr_3!();

        assert!(digraph.degree(0) == 2);
        assert!(digraph.degree(1) == 4);
        assert!(digraph.degree(2) == 3);
        assert!(digraph.degree(3) == 8);
        assert!(digraph.degree(4) == 6);
        assert!(digraph.degree(5) == 6);
        assert!(digraph.degree(6) == 8);
        assert!(digraph.degree(7) == 4);
        assert!(digraph.degree(8) == 4);
        assert!(digraph.degree(9) == 5);
    }

    #[test]
    fn degree_kattis_crosscountry() {
        let digraph = kattis_crosscountry!();

        assert!(digraph.degree(0) == 6);
        assert!(digraph.degree(1) == 6);
        assert!(digraph.degree(2) == 6);
        assert!(digraph.degree(3) == 6);
    }

    #[test]
    fn degree_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1!();

        assert!(digraph.degree(0) == 2);
        assert!(digraph.degree(1) == 2);
        assert!(digraph.degree(2) == 1);
        assert!(digraph.degree(3) == 1);
    }

    #[test]
    fn empty_is_complete_singleton() {
        assert!(Digraph::<usize>::trivial().is_complete());
    }

    #[test]
    fn empty_is_semicomplete_singleton() {
        assert!(Digraph::<usize>::trivial().is_semicomplete());
    }

    #[test]
    fn in_neighbors_bang_jensen_94_weighted() {
        let digraph = bang_jensen_94_weighted!();

        assert!(digraph.in_neighbors(0).eq([]));
        assert!(digraph.in_neighbors(1).eq([0, 2]));
        assert!(digraph.in_neighbors(2).eq([0]));
        assert!(digraph.in_neighbors(3).eq([1, 2]));
        assert!(digraph.in_neighbors(4).eq([2]));
        assert!(digraph.in_neighbors(5).eq([2, 3]));
        assert!(digraph.in_neighbors(6).eq([4]));
    }

    #[test]
    fn in_neighbors_bang_jensen_96() {
        let digraph = bang_jensen_96!();

        assert!(digraph.in_neighbors(0).eq([]));
        assert!(digraph.in_neighbors(1).eq([0, 2]));
        assert!(digraph.in_neighbors(2).eq([0, 1, 4]));
        assert!(digraph.in_neighbors(3).eq([1, 4, 5]));
        assert!(digraph.in_neighbors(4).eq([2]));
        assert!(digraph.in_neighbors(5).eq([3, 4]));
    }

    #[test]
    fn in_neighbors_bang_jensen_99() {
        let digraph = bang_jensen_99!();

        assert!(digraph.in_neighbors(0).eq([]));
        assert!(digraph.in_neighbors(1).eq([0]));
        assert!(digraph.in_neighbors(2).eq([0, 1]));
        assert!(digraph.in_neighbors(3).eq([2, 4, 5]));
        assert!(digraph.in_neighbors(4).eq([2, 5]));
        assert!(digraph.in_neighbors(5).eq([3, 4]));
    }

    #[test]
    fn in_neighbors_kattis_bryr_1() {
        let digraph = kattis_bryr_1!();

        assert!(digraph.in_neighbors(0).eq([1, 2]));
        assert!(digraph.in_neighbors(1).eq([0, 2]));
        assert!(digraph.in_neighbors(2).eq([0, 1]));
    }

    #[test]
    fn in_neighbors_kattis_bryr_2() {
        let digraph = kattis_bryr_2!();

        assert!(digraph.in_neighbors(0).eq([1, 3]));
        assert!(digraph.in_neighbors(1).eq([0, 2]));
        assert!(digraph.in_neighbors(2).eq([1, 3]));
        assert!(digraph.in_neighbors(3).eq([0, 2, 4]));
        assert!(digraph.in_neighbors(4).eq([3, 5]));
        assert!(digraph.in_neighbors(5).eq([4]));
    }

    #[test]
    fn in_neighbors_kattis_bryr_3() {
        let digraph = kattis_bryr_3!();

        assert!(digraph.in_neighbors(0).eq([3]));
        assert!(digraph.in_neighbors(1).eq([7, 9]));
        assert!(digraph.in_neighbors(2).eq([6, 9]));
        assert!(digraph.in_neighbors(3).eq([0, 4, 5, 7]));
        assert!(digraph.in_neighbors(4).eq([3, 6, 8]));
        assert!(digraph.in_neighbors(5).eq([3, 6, 8]));
        assert!(digraph.in_neighbors(6).eq([2, 4, 5, 9]));
        assert!(digraph.in_neighbors(7).eq([1, 3]));
        assert!(digraph.in_neighbors(8).eq([4, 5]));
        assert!(digraph.in_neighbors(9).eq([1, 6]));
    }

    #[test]
    fn in_neighbors_kattis_crosscountry() {
        let digraph = kattis_crosscountry!();

        assert!(digraph.in_neighbors(0).eq([1, 2, 3]));
        assert!(digraph.in_neighbors(1).eq([0, 2, 3]));
        assert!(digraph.in_neighbors(2).eq([0, 1, 3]));
        assert!(digraph.in_neighbors(3).eq([0, 1, 2]));
    }

    #[test]
    fn in_neighbors_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1!();

        assert!(digraph.in_neighbors(0).eq([3]));
        assert!(digraph.in_neighbors(1).eq([0]));
        assert!(digraph.in_neighbors(2).eq([1]));
        assert!(digraph.in_neighbors(3).eq([]));
    }

    #[test]
    fn indegree_bang_jensen_94_weighted() {
        let digraph = bang_jensen_94_weighted!();

        assert!(digraph.indegree(0) == 0);
        assert!(digraph.indegree(1) == 2);
        assert!(digraph.indegree(2) == 1);
        assert!(digraph.indegree(3) == 2);
        assert!(digraph.indegree(4) == 1);
        assert!(digraph.indegree(5) == 2);
        assert!(digraph.indegree(6) == 1);
    }

    #[test]
    fn indegree_bang_jensen_96() {
        let digraph = bang_jensen_96!();

        assert!(digraph.indegree(0) == 0);
        assert!(digraph.indegree(1) == 2);
        assert!(digraph.indegree(2) == 3);
        assert!(digraph.indegree(3) == 3);
        assert!(digraph.indegree(4) == 1);
        assert!(digraph.indegree(5) == 2);
    }

    #[test]
    fn indegree_bang_jensen_99() {
        let digraph = bang_jensen_99!();

        assert!(digraph.indegree(0) == 0);
        assert!(digraph.indegree(1) == 1);
        assert!(digraph.indegree(2) == 2);
        assert!(digraph.indegree(3) == 3);
        assert!(digraph.indegree(4) == 2);
        assert!(digraph.indegree(5) == 2);
    }

    #[test]
    fn indegree_kattis_bryr_1() {
        let digraph = kattis_bryr_1!();

        assert!(digraph.indegree(0) == 2);
        assert!(digraph.indegree(1) == 2);
        assert!(digraph.indegree(2) == 2);
    }

    #[test]
    fn indegree_kattis_bryr_2() {
        let digraph = kattis_bryr_2!();

        assert!(digraph.indegree(0) == 2);
        assert!(digraph.indegree(1) == 2);
        assert!(digraph.indegree(2) == 2);
        assert!(digraph.indegree(3) == 3);
        assert!(digraph.indegree(4) == 2);
        assert!(digraph.indegree(5) == 1);
    }

    #[test]
    fn indegree_kattis_bryr_3() {
        let digraph = kattis_bryr_3!();

        assert!(digraph.indegree(0) == 1);
        assert!(digraph.indegree(1) == 2);
        assert!(digraph.indegree(2) == 2);
        assert!(digraph.indegree(3) == 4);
        assert!(digraph.indegree(4) == 3);
        assert!(digraph.indegree(5) == 3);
        assert!(digraph.indegree(6) == 4);
        assert!(digraph.indegree(7) == 2);
        assert!(digraph.indegree(8) == 2);
        assert!(digraph.indegree(9) == 2);
    }

    #[test]
    fn indegree_kattis_crosscountry() {
        let digraph = kattis_crosscountry!();

        assert!(digraph.indegree(0) == 3);
        assert!(digraph.indegree(1) == 3);
        assert!(digraph.indegree(2) == 3);
        assert!(digraph.indegree(3) == 3);
    }

    #[test]
    fn indegree_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1!();

        assert!(digraph.indegree(0) == 1);
        assert!(digraph.indegree(1) == 1);
        assert!(digraph.indegree(2) == 1);
        assert!(digraph.indegree(3) == 0);
    }

    #[test]
    fn is_balanced_bang_jensen_94_weighted() {
        assert!(!bang_jensen_94_weighted!().is_balanced());
    }

    #[test]
    fn is_balanced_bang_jensen_96() {
        assert!(!bang_jensen_96!().is_balanced());
    }

    #[test]
    fn is_balanced_bang_jensen_99() {
        assert!(!bang_jensen_99!().is_balanced());
    }

    #[test]
    fn is_balanced_kattis_bryr_1() {
        assert!(kattis_bryr_1!().is_balanced());
    }

    #[test]
    fn is_balanced_kattis_bryr_2() {
        assert!(kattis_bryr_2!().is_balanced());
    }

    #[test]
    fn is_balanced_kattis_bryr_3() {
        assert!(!kattis_bryr_3!().is_balanced());
    }

    #[test]
    fn is_balanced_kattis_crosscountry() {
        assert!(kattis_crosscountry!().is_balanced());
    }

    #[test]
    fn is_balanced_kattis_shortestpath1() {
        assert!(!kattis_shortestpath1!().is_balanced());
    }

    #[test]
    fn is_complete_bang_jensen_94_weighted() {
        assert!(!bang_jensen_94_weighted!().is_complete());
    }

    #[test]
    fn is_complete_bang_jensen_96() {
        assert!(!bang_jensen_96!().is_complete());
    }

    #[test]
    fn is_complete_bang_jensen_99() {
        assert!(!bang_jensen_99!().is_complete());
    }

    #[test]
    fn is_complete_kattis_bryr_1() {
        assert!(kattis_bryr_1!().is_complete());
    }

    #[test]
    fn is_complete_kattis_bryr_2() {
        assert!(!kattis_bryr_2!().is_complete());
    }

    #[test]
    fn is_complete_kattis_bryr_3() {
        assert!(!kattis_bryr_3!().is_complete());
    }

    #[test]
    fn is_complete_kattis_crosscountry() {
        assert!(kattis_crosscountry!().is_complete());
    }

    #[test]
    fn is_complete_kattis_shortestpath1() {
        assert!(!kattis_shortestpath1!().is_complete());
    }

    #[test]
    fn is_isolated_bang_jensen_94_weighted() {
        let digraph = bang_jensen_94_weighted!();

        assert!(!digraph.is_isolated(0));
        assert!(!digraph.is_isolated(1));
        assert!(!digraph.is_isolated(2));
        assert!(!digraph.is_isolated(3));
        assert!(!digraph.is_isolated(4));
        assert!(!digraph.is_isolated(5));
        assert!(!digraph.is_isolated(6));
    }

    #[test]
    fn is_isolated_bang_jensen_96() {
        let digraph = bang_jensen_96!();

        assert!(!digraph.is_isolated(0));
        assert!(!digraph.is_isolated(1));
        assert!(!digraph.is_isolated(2));
        assert!(!digraph.is_isolated(3));
        assert!(!digraph.is_isolated(4));
        assert!(!digraph.is_isolated(5));
    }

    #[test]
    fn is_isolated_bang_jensen_99() {
        let digraph = bang_jensen_99!();

        assert!(!digraph.is_isolated(0));
        assert!(!digraph.is_isolated(1));
        assert!(!digraph.is_isolated(2));
        assert!(!digraph.is_isolated(3));
        assert!(!digraph.is_isolated(4));
        assert!(!digraph.is_isolated(5));
    }

    #[test]
    fn is_isolated_kattis_bryr_1() {
        let digraph = kattis_bryr_1!();

        assert!(!digraph.is_isolated(0));
        assert!(!digraph.is_isolated(1));
        assert!(!digraph.is_isolated(2));
    }

    #[test]
    fn is_isolated_kattis_bryr_2() {
        let digraph = kattis_bryr_2!();

        assert!(!digraph.is_isolated(0));
        assert!(!digraph.is_isolated(1));
        assert!(!digraph.is_isolated(2));
        assert!(!digraph.is_isolated(3));
        assert!(!digraph.is_isolated(4));
        assert!(!digraph.is_isolated(5));
    }

    #[test]
    fn is_isolated_kattis_bryr_3() {
        let digraph = kattis_bryr_3!();

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
    }

    #[test]
    fn is_isolated_kattis_crosscountry() {
        let digraph = kattis_crosscountry!();

        assert!(!digraph.is_isolated(0));
        assert!(!digraph.is_isolated(1));
        assert!(!digraph.is_isolated(2));
        assert!(!digraph.is_isolated(3));
    }

    #[test]
    fn is_isolated_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1!();

        assert!(!digraph.is_isolated(0));
        assert!(!digraph.is_isolated(1));
        assert!(!digraph.is_isolated(2));
        assert!(!digraph.is_isolated(3));
    }

    #[test]
    fn is_oriented_bang_jensen_94_weighted() {
        assert!(bang_jensen_94_weighted!().is_oriented());
    }

    #[test]
    fn is_oriented_bang_jensen_96() {
        assert!(!bang_jensen_96!().is_oriented());
    }

    #[test]
    fn is_oriented_bang_jensen_99() {
        assert!(!bang_jensen_99!().is_oriented());
    }

    #[test]
    fn is_oriented_kattis_bryr_1() {
        assert!(!kattis_bryr_1!().is_oriented());
    }

    #[test]
    fn is_oriented_kattis_bryr_2() {
        assert!(!kattis_bryr_2!().is_oriented());
    }

    #[test]
    fn is_oriented_kattis_bryr_3() {
        assert!(!kattis_bryr_3!().is_oriented());
    }

    #[test]
    fn is_oriented_kattis_crosscountry() {
        assert!(!kattis_crosscountry!().is_oriented());
    }

    #[test]
    fn is_oriented_kattis_shortestpath1() {
        assert!(kattis_shortestpath1!().is_oriented());
    }

    #[test]
    fn is_pendant_bang_jensen_94_weighted() {
        let digraph = bang_jensen_94_weighted!();

        assert!(!digraph.is_pendant(0));
        assert!(!digraph.is_pendant(1));
        assert!(!digraph.is_pendant(2));
        assert!(!digraph.is_pendant(3));
        assert!(!digraph.is_pendant(4));
        assert!(!digraph.is_pendant(5));
        assert!(digraph.is_pendant(6));
    }

    #[test]
    fn is_pendant_bang_jensen_96() {
        let digraph = bang_jensen_96!();

        assert!(!digraph.is_pendant(0));
        assert!(!digraph.is_pendant(1));
        assert!(!digraph.is_pendant(2));
        assert!(!digraph.is_pendant(3));
        assert!(!digraph.is_pendant(4));
        assert!(!digraph.is_pendant(5));
    }

    #[test]
    fn is_pendant_bang_jensen_99() {
        let digraph = bang_jensen_99!();

        assert!(!digraph.is_pendant(0));
        assert!(!digraph.is_pendant(1));
        assert!(!digraph.is_pendant(2));
        assert!(!digraph.is_pendant(3));
        assert!(!digraph.is_pendant(4));
        assert!(!digraph.is_pendant(5));
    }

    #[test]
    fn is_pendant_kattis_bryr_1() {
        let digraph = kattis_bryr_1!();

        assert!(!digraph.is_pendant(0));
        assert!(!digraph.is_pendant(1));
        assert!(!digraph.is_pendant(2));
    }

    #[test]
    fn is_pendant_kattis_bryr_2() {
        let digraph = kattis_bryr_2!();

        assert!(!digraph.is_pendant(0));
        assert!(!digraph.is_pendant(1));
        assert!(!digraph.is_pendant(2));
        assert!(!digraph.is_pendant(3));
        assert!(!digraph.is_pendant(4));
        assert!(!digraph.is_pendant(5));
    }

    #[test]
    fn is_pendant_kattis_bryr_3() {
        let digraph = kattis_bryr_3!();

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
    }

    #[test]
    fn is_pendant_kattis_crosscountry() {
        let digraph = kattis_crosscountry!();

        assert!(!digraph.is_pendant(0));
        assert!(!digraph.is_pendant(1));
        assert!(!digraph.is_pendant(2));
        assert!(!digraph.is_pendant(3));
    }

    #[test]
    fn is_pendant_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1!();

        assert!(!digraph.is_pendant(0));
        assert!(!digraph.is_pendant(1));
        assert!(digraph.is_pendant(2));
        assert!(digraph.is_pendant(3));
    }

    #[test]
    fn is_regular_bang_jensen_94_weighted() {
        assert!(!bang_jensen_94_weighted!().is_regular());
    }

    #[test]
    fn is_regular_bang_jensen_96() {
        assert!(!bang_jensen_96!().is_regular());
    }

    #[test]
    fn is_regular_bang_jensen_99() {
        assert!(!bang_jensen_99!().is_regular());
    }

    #[test]
    fn is_regular_kattis_bryr_1() {
        assert!(kattis_bryr_1!().is_regular());
    }

    #[test]
    fn is_regular_kattis_bryr_2() {
        assert!(!kattis_bryr_2!().is_regular());
    }

    #[test]
    fn is_regular_kattis_bryr_3() {
        assert!(!kattis_bryr_3!().is_regular());
    }

    #[test]
    fn is_regular_kattis_crosscountry() {
        assert!(kattis_crosscountry!().is_regular());
    }

    #[test]
    fn is_regular_kattis_shortestpath1() {
        assert!(!kattis_shortestpath1!().is_regular());
    }

    #[test]
    fn is_semicomplete_bang_jensen_94_weighted() {
        assert!(!bang_jensen_94_weighted!().is_semicomplete());
    }

    #[test]
    fn is_semicomplete_bang_jensen_96() {
        assert!(!bang_jensen_96!().is_semicomplete());
    }

    #[test]
    fn is_semicomplete_bang_jensen_99() {
        assert!(!bang_jensen_99!().is_semicomplete());
    }

    #[test]
    fn is_semicomplete_kattis_bryr_1() {
        assert!(kattis_bryr_1!().is_semicomplete());
    }

    #[test]
    fn is_semicomplete_kattis_bryr_2() {
        assert!(!kattis_bryr_2!().is_semicomplete());
    }

    #[test]
    fn is_semicomplete_kattis_bryr_3() {
        assert!(!kattis_bryr_3!().is_semicomplete());
    }

    #[test]
    fn is_semicomplete_kattis_crosscountry() {
        assert!(kattis_crosscountry!().is_semicomplete());
    }

    #[test]
    fn is_semicomplete_kattis_shortestpath1() {
        assert!(!kattis_shortestpath1!().is_semicomplete());
    }

    #[test]
    fn is_simple_bang_jensen_94_weighted() {
        assert!(bang_jensen_94_weighted!().is_simple());
    }

    #[test]
    fn is_simple_bang_jensen_96() {
        assert!(bang_jensen_96!().is_simple());
    }

    #[test]
    fn is_simple_bang_jensen_99() {
        assert!(bang_jensen_99!().is_simple());
    }

    #[test]
    fn is_simple_kattis_bryr_1() {
        assert!(kattis_bryr_1!().is_simple());
    }

    #[test]
    fn is_simple_kattis_bryr_2() {
        assert!(kattis_bryr_2!().is_simple());
    }

    #[test]
    fn is_simple_kattis_bryr_3() {
        assert!(kattis_bryr_3!().is_simple());
    }

    #[test]
    fn is_simple_kattis_crosscountry() {
        assert!(kattis_crosscountry!().is_simple());
    }

    #[test]
    fn is_simple_kattis_shortestpath1() {
        assert!(kattis_shortestpath1!().is_simple());
    }

    #[test]
    fn is_symmetric_bang_jensen_94_weighted() {
        assert!(!bang_jensen_94_weighted!().is_symmetric());
    }

    #[test]
    fn is_symmetric_bang_jensen_96() {
        assert!(!bang_jensen_96!().is_symmetric());
    }

    #[test]
    fn is_symmetric_bang_jensen_99() {
        assert!(!bang_jensen_99!().is_symmetric());
    }

    #[test]
    fn is_symmetric_kattis_bryr_1() {
        assert!(kattis_bryr_1!().is_symmetric());
    }

    #[test]
    fn is_symmetric_kattis_bryr_2() {
        assert!(kattis_bryr_2!().is_symmetric());
    }

    #[test]
    fn is_symmetric_kattis_bryr_3() {
        assert!(!kattis_bryr_3!().is_symmetric());
    }

    #[test]
    fn is_symmetric_kattis_crosscountry() {
        assert!(kattis_crosscountry!().is_symmetric());
    }

    #[test]
    fn is_symmetric_kattis_shortestpath1() {
        assert!(!kattis_shortestpath1!().is_symmetric());
    }

    #[test]
    fn order_bang_jensen_94_weighted() {
        assert!(bang_jensen_94_weighted!().order() == 7);
    }

    #[test]
    fn order_bang_jensen_96() {
        assert!(bang_jensen_96!().order() == 6);
    }

    #[test]
    fn order_bang_jensen_99() {
        assert!(bang_jensen_99!().order() == 6);
    }

    #[test]
    fn order_kattis_bryr_1() {
        assert!(kattis_bryr_1!().order() == 3);
    }

    #[test]
    fn order_kattis_bryr_2() {
        assert!(kattis_bryr_2!().order() == 6);
    }

    #[test]
    fn order_kattis_bryr_3() {
        assert!(kattis_bryr_3!().order() == 10);
    }

    #[test]
    fn order_kattis_crosscountry() {
        assert!(kattis_crosscountry!().order() == 4);
    }

    #[test]
    fn order_kattis_shortestpath1() {
        assert!(kattis_shortestpath1!().order() == 4);
    }

    #[test]
    fn out_neighbors_bang_jensen_94_weighted() {
        let digraph = bang_jensen_94_weighted!();

        assert!(digraph.out_neighbors(0).eq([1, 2]));
        assert!(digraph.out_neighbors(1).eq([3]));
        assert!(digraph.out_neighbors(2).eq([1, 3, 4, 5]));
        assert!(digraph.out_neighbors(3).eq([5]));
        assert!(digraph.out_neighbors(4).eq([6]));
        assert!(digraph.out_neighbors(5).eq([]));
        assert!(digraph.out_neighbors(6).eq([]));
    }

    #[test]
    fn out_neighbors_bang_jensen_96() {
        let digraph = bang_jensen_96!();

        assert!(digraph.out_neighbors(0).eq([1, 2]));
        assert!(digraph.out_neighbors(1).eq([2, 3]));
        assert!(digraph.out_neighbors(2).eq([1, 4]));
        assert!(digraph.out_neighbors(3).eq([5]));
        assert!(digraph.out_neighbors(4).eq([2, 3, 5]));
        assert!(digraph.out_neighbors(5).eq([3]));
    }

    #[test]
    fn outdegree_bang_jensen_94_weighted() {
        let digraph = bang_jensen_94_weighted!();

        assert!(digraph.outdegree(0) == 2);
        assert!(digraph.outdegree(1) == 1);
        assert!(digraph.outdegree(2) == 4);
        assert!(digraph.outdegree(3) == 1);
        assert!(digraph.outdegree(4) == 1);
        assert!(digraph.outdegree(5) == 0);
        assert!(digraph.outdegree(6) == 0);
    }

    #[test]
    fn outdegree_bang_jensen_96() {
        let digraph = bang_jensen_96!();

        assert!(digraph.outdegree(0) == 2);
        assert!(digraph.outdegree(1) == 2);
        assert!(digraph.outdegree(2) == 2);
        assert!(digraph.outdegree(3) == 1);
        assert!(digraph.outdegree(4) == 3);
        assert!(digraph.outdegree(5) == 1);
    }

    #[test]
    fn outdegree_bang_jensen_99() {
        let digraph = bang_jensen_99!();

        assert!(digraph.outdegree(0) == 2);
        assert!(digraph.outdegree(1) == 1);
        assert!(digraph.outdegree(2) == 2);
        assert!(digraph.outdegree(3) == 1);
        assert!(digraph.outdegree(4) == 2);
        assert!(digraph.outdegree(5) == 2);
    }

    #[test]
    fn outdegree_kattis_bryr_1() {
        let digraph = kattis_bryr_1!();

        assert!(digraph.outdegree(0) == 2);
        assert!(digraph.outdegree(1) == 2);
        assert!(digraph.outdegree(2) == 2);
    }

    #[test]
    fn outdegree_kattis_bryr_2() {
        let digraph = kattis_bryr_2!();

        assert!(digraph.outdegree(0) == 2);
        assert!(digraph.outdegree(1) == 2);
        assert!(digraph.outdegree(2) == 2);
        assert!(digraph.outdegree(3) == 3);
        assert!(digraph.outdegree(4) == 2);
        assert!(digraph.outdegree(5) == 1);
    }

    #[test]
    fn outdegree_kattis_bryr_3() {
        let digraph = kattis_bryr_3!();

        assert!(digraph.outdegree(0) == 1);
        assert!(digraph.outdegree(1) == 2);
        assert!(digraph.outdegree(2) == 1);
        assert!(digraph.outdegree(3) == 4);
        assert!(digraph.outdegree(4) == 3);
        assert!(digraph.outdegree(5) == 3);
        assert!(digraph.outdegree(6) == 4);
        assert!(digraph.outdegree(7) == 2);
        assert!(digraph.outdegree(8) == 2);
        assert!(digraph.outdegree(9) == 3);
    }

    #[test]
    fn outdegree_kattis_crosscountry() {
        let digraph = kattis_crosscountry!();

        assert!(digraph.outdegree(0) == 3);
        assert!(digraph.outdegree(1) == 3);
        assert!(digraph.outdegree(2) == 3);
        assert!(digraph.outdegree(3) == 3);
    }

    #[test]
    fn outdegree_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1!();

        assert!(digraph.outdegree(0) == 1);
        assert!(digraph.outdegree(1) == 1);
        assert!(digraph.outdegree(2) == 0);
        assert!(digraph.outdegree(3) == 1);
    }

    #[test]
    fn remove_arc_bang_jensen_94_weighted() {
        let mut digraph = bang_jensen_94_weighted!();

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
    fn remove_arc_bang_jensen_96() {
        let mut digraph = bang_jensen_96!();

        assert!(digraph.remove_arc(0, 1));
        assert!(digraph.remove_arc(0, 2));
        assert!(digraph.remove_arc(1, 2));
        assert!(digraph.remove_arc(1, 3));
        assert!(digraph.remove_arc(2, 1));
        assert!(digraph.remove_arc(2, 4));
        assert!(digraph.remove_arc(3, 5));
        assert!(digraph.remove_arc(4, 2));
        assert!(digraph.remove_arc(4, 3));
        assert!(digraph.remove_arc(4, 5));
        assert!(digraph.remove_arc(5, 3));

        assert!(digraph.arcs().eq([]));

        assert!(!digraph.remove_arc(0, 1));
        assert!(!digraph.remove_arc(0, 2));
        assert!(!digraph.remove_arc(1, 2));
        assert!(!digraph.remove_arc(1, 3));
        assert!(!digraph.remove_arc(2, 1));
        assert!(!digraph.remove_arc(2, 4));
        assert!(!digraph.remove_arc(3, 5));
        assert!(!digraph.remove_arc(4, 2));
        assert!(!digraph.remove_arc(4, 3));
        assert!(!digraph.remove_arc(4, 5));
        assert!(!digraph.remove_arc(5, 3));
    }

    #[test]
    fn remove_arc_bang_jensen_99() {
        let mut digraph = bang_jensen_99!();

        assert!(digraph.remove_arc(0, 1));
        assert!(digraph.remove_arc(0, 2));
        assert!(digraph.remove_arc(1, 2));
        assert!(digraph.remove_arc(2, 3));
        assert!(digraph.remove_arc(2, 4));
        assert!(digraph.remove_arc(3, 5));
        assert!(digraph.remove_arc(4, 3));
        assert!(digraph.remove_arc(4, 5));
        assert!(digraph.remove_arc(5, 3));
        assert!(digraph.remove_arc(5, 4));

        assert!(digraph.arcs().eq([]));

        assert!(!digraph.remove_arc(0, 1));
        assert!(!digraph.remove_arc(0, 2));
        assert!(!digraph.remove_arc(1, 2));
        assert!(!digraph.remove_arc(2, 3));
        assert!(!digraph.remove_arc(2, 4));
        assert!(!digraph.remove_arc(3, 5));
        assert!(!digraph.remove_arc(4, 3));
        assert!(!digraph.remove_arc(4, 5));
        assert!(!digraph.remove_arc(5, 3));
        assert!(!digraph.remove_arc(5, 4));
    }

    #[test]
    fn remove_arc_kattis_bryr_1() {
        let mut digraph = kattis_bryr_1!();

        assert!(digraph.remove_arc(0, 1));
        assert!(digraph.remove_arc(0, 2));
        assert!(digraph.remove_arc(1, 0));
        assert!(digraph.remove_arc(1, 2));
        assert!(digraph.remove_arc(2, 0));
        assert!(digraph.remove_arc(2, 1));

        assert!(digraph.arcs().eq([]));

        assert!(!digraph.remove_arc(0, 1));
        assert!(!digraph.remove_arc(0, 2));
        assert!(!digraph.remove_arc(1, 0));
        assert!(!digraph.remove_arc(1, 2));
        assert!(!digraph.remove_arc(2, 0));
        assert!(!digraph.remove_arc(2, 1));
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn remove_arc_kattis_bryr_2() {
        let mut digraph = kattis_bryr_2!();

        assert!(digraph.remove_arc(0, 1));
        assert!(digraph.remove_arc(0, 3));
        assert!(digraph.remove_arc(1, 0));
        assert!(digraph.remove_arc(1, 2));
        assert!(digraph.remove_arc(2, 1));
        assert!(digraph.remove_arc(2, 3));
        assert!(digraph.remove_arc(3, 0));
        assert!(digraph.remove_arc(3, 2));
        assert!(digraph.remove_arc(3, 4));
        assert!(digraph.remove_arc(4, 3));
        assert!(digraph.remove_arc(4, 5));
        assert!(digraph.remove_arc(5, 4));

        assert!(digraph.arcs().eq([]));

        assert!(!digraph.remove_arc(0, 1));
        assert!(!digraph.remove_arc(0, 3));
        assert!(!digraph.remove_arc(1, 0));
        assert!(!digraph.remove_arc(1, 2));
        assert!(!digraph.remove_arc(2, 1));
        assert!(!digraph.remove_arc(2, 3));
        assert!(!digraph.remove_arc(3, 0));
        assert!(!digraph.remove_arc(3, 2));
        assert!(!digraph.remove_arc(3, 4));
        assert!(!digraph.remove_arc(4, 3));
        assert!(!digraph.remove_arc(4, 5));
        assert!(!digraph.remove_arc(5, 4));
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn remove_arc_kattis_bryr_3() {
        let mut digraph = kattis_bryr_3!();

        assert!(digraph.remove_arc(0, 3));
        assert!(digraph.remove_arc(1, 7));
        assert!(digraph.remove_arc(1, 9));
        assert!(digraph.remove_arc(2, 6));
        assert!(digraph.remove_arc(3, 0));
        assert!(digraph.remove_arc(3, 4));
        assert!(digraph.remove_arc(3, 5));
        assert!(digraph.remove_arc(3, 7));
        assert!(digraph.remove_arc(4, 3));
        assert!(digraph.remove_arc(4, 6));
        assert!(digraph.remove_arc(4, 8));
        assert!(digraph.remove_arc(5, 3));
        assert!(digraph.remove_arc(5, 6));
        assert!(digraph.remove_arc(5, 8));
        assert!(digraph.remove_arc(6, 2));
        assert!(digraph.remove_arc(6, 4));
        assert!(digraph.remove_arc(6, 5));
        assert!(digraph.remove_arc(6, 9));
        assert!(digraph.remove_arc(7, 1));
        assert!(digraph.remove_arc(7, 3));
        assert!(digraph.remove_arc(8, 4));
        assert!(digraph.remove_arc(8, 5));
        assert!(digraph.remove_arc(9, 1));
        assert!(digraph.remove_arc(9, 2));
        assert!(digraph.remove_arc(9, 6));

        assert!(digraph.arcs().eq([]));

        assert!(!digraph.remove_arc(0, 3));
        assert!(!digraph.remove_arc(1, 7));
        assert!(!digraph.remove_arc(1, 9));
        assert!(!digraph.remove_arc(2, 6));
        assert!(!digraph.remove_arc(3, 0));
        assert!(!digraph.remove_arc(3, 4));
        assert!(!digraph.remove_arc(3, 5));
        assert!(!digraph.remove_arc(3, 7));
        assert!(!digraph.remove_arc(4, 3));
        assert!(!digraph.remove_arc(4, 6));
        assert!(!digraph.remove_arc(4, 8));
        assert!(!digraph.remove_arc(5, 3));
        assert!(!digraph.remove_arc(5, 6));
        assert!(!digraph.remove_arc(5, 8));
        assert!(!digraph.remove_arc(6, 2));
        assert!(!digraph.remove_arc(6, 4));
        assert!(!digraph.remove_arc(6, 5));
        assert!(!digraph.remove_arc(6, 9));
        assert!(!digraph.remove_arc(7, 1));
        assert!(!digraph.remove_arc(7, 3));
        assert!(!digraph.remove_arc(8, 4));
        assert!(!digraph.remove_arc(8, 5));
        assert!(!digraph.remove_arc(9, 1));
        assert!(!digraph.remove_arc(9, 2));
        assert!(!digraph.remove_arc(9, 6));
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn remove_arc_kattis_crosscountry() {
        let mut digraph = kattis_crosscountry!();

        assert!(digraph.remove_arc(0, 1));
        assert!(digraph.remove_arc(0, 2));
        assert!(digraph.remove_arc(0, 3));
        assert!(digraph.remove_arc(1, 0));
        assert!(digraph.remove_arc(1, 2));
        assert!(digraph.remove_arc(1, 3));
        assert!(digraph.remove_arc(2, 0));
        assert!(digraph.remove_arc(2, 1));
        assert!(digraph.remove_arc(2, 3));
        assert!(digraph.remove_arc(3, 0));
        assert!(digraph.remove_arc(3, 1));
        assert!(digraph.remove_arc(3, 2));

        assert!(digraph.arcs().eq([]));

        assert!(!digraph.remove_arc(0, 1));
        assert!(!digraph.remove_arc(0, 2));
        assert!(!digraph.remove_arc(0, 3));
        assert!(!digraph.remove_arc(1, 0));
        assert!(!digraph.remove_arc(1, 2));
        assert!(!digraph.remove_arc(1, 3));
        assert!(!digraph.remove_arc(2, 0));
        assert!(!digraph.remove_arc(2, 1));
        assert!(!digraph.remove_arc(2, 3));
        assert!(!digraph.remove_arc(3, 0));
        assert!(!digraph.remove_arc(3, 1));
        assert!(!digraph.remove_arc(3, 2));
    }

    #[test]
    fn remove_arc_kattis_shortestpath1() {
        let mut digraph = kattis_shortestpath1!();

        assert!(digraph.remove_arc(0, 1));
        assert!(digraph.remove_arc(1, 2));
        assert!(digraph.remove_arc(3, 0));

        assert!(digraph.arcs().eq([]));

        assert!(!digraph.remove_arc(0, 1));
        assert!(!digraph.remove_arc(1, 2));
        assert!(!digraph.remove_arc(3, 0));
    }

    #[test]
    fn size_bang_jensen_94_weighted() {
        assert_eq!(bang_jensen_94_weighted!().size(), 9);
    }

    #[test]
    fn size_bang_jensen_96() {
        assert_eq!(bang_jensen_96!().size(), 11);
    }

    #[test]
    fn size_bang_jensen_99() {
        assert_eq!(bang_jensen_99!().size(), 10);
    }

    #[test]
    fn size_kattis_bryr_1() {
        assert_eq!(kattis_bryr_1!().size(), 6);
    }

    #[test]
    fn size_kattis_bryr_2() {
        assert_eq!(kattis_bryr_2!().size(), 12);
    }

    #[test]
    fn size_kattis_bryr_3() {
        assert_eq!(kattis_bryr_3!().size(), 25);
    }

    #[test]
    fn size_kattis_crosscountry() {
        assert_eq!(kattis_crosscountry!().size(), 12);
    }

    #[test]
    fn size_kattis_shortestpath1() {
        assert_eq!(kattis_shortestpath1!().size(), 3);
    }
}

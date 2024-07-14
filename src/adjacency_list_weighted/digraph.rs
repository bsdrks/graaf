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
    /// The arcs of the digraph
    arcs: Vec<BTreeMap<usize, W>>,
}

impl<W> AddArcWeighted<W> for Digraph<W> {
    /// # Panics
    ///
    /// Panics if `u` is out of bounds.
    fn add_arc_weighted(&mut self, u: usize, v: usize, w: W) {
        let _ = self.arcs[u].insert(v, w);
    }
}

impl<W> ArcWeight<W> for Digraph<W> {
    fn arc_weight(&self, u: usize, v: usize) -> Option<&W> {
        self.arcs.get(u).and_then(|arcs| arcs.get(&v))
    }
}

impl<W> Arcs for Digraph<W> {
    fn arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.arcs
            .iter()
            .enumerate()
            .flat_map(|(u, set)| set.iter().map(move |(&v, _)| (u, v)))
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
            .flat_map(|(u, map)| map.iter().map(move |(&v, w)| (u, v, w)))
    }
}

impl<W> Converse for Digraph<W>
where
    W: Copy,
{
    fn converse(&self) -> Self {
        let order = self.order();
        let mut converse = Self::empty(order);

        for (u, v, &w) in self.arcs_weighted() {
            converse.add_arc_weighted(v, u, w);
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
    /// Panics if `order` is zero.
    fn empty(order: usize) -> Self {
        assert!(order > 0, "a digraph must have at least one vertex");

        Self {
            arcs: vec![BTreeMap::new(); order],
        }
    }
}

impl<W> From<Vec<BTreeMap<usize, W>>> for Digraph<W> {
    fn from(arcs: Vec<BTreeMap<usize, W>>) -> Self {
        Self { arcs }
    }
}

impl<W> HasArc for Digraph<W> {
    fn has_arc(&self, u: usize, v: usize) -> bool {
        self.arcs.get(u).map_or(false, |set| set.contains_key(&v))
    }
}

impl<W> Indegree for Digraph<W> {
    /// # Panics
    ///
    /// Panics if `v` is out of bounds.
    fn indegree(&self, v: usize) -> usize {
        assert!(v < self.order(), "v = {v} is out of bounds");

        self.arcs
            .iter()
            .filter(|arcs| arcs.contains_key(&v))
            .count()
    }
}

impl<W> IsSimple for Digraph<W> {
    fn is_simple(&self) -> bool {
        self.arcs
            .iter()
            .enumerate()
            .all(|(u, map)| !map.contains_key(&u))
    }
}

impl<W> OutNeighbors for Digraph<W> {
    /// # Panics
    ///
    /// Panics if `u` is out of bounds.
    fn out_neighbors(&self, u: usize) -> impl Iterator<Item = usize> {
        self.arcs[u].keys().copied()
    }
}

impl<W> OutNeighborsWeighted<W> for Digraph<W> {
    /// # Panics
    ///
    /// Panics if `u` is out of bounds.
    fn out_neighbors_weighted<'a>(&'a self, u: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self.arcs[u].iter().map(|(v, w)| (*v, w))
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
    /// Panics if `u` is out of bounds.
    fn outdegree(&self, u: usize) -> usize {
        self.arcs[u].len()
    }
}

impl<W> RemoveArc for Digraph<W> {
    fn remove_arc(&mut self, u: usize, v: usize) -> bool {
        self.arcs
            .get_mut(u)
            .map_or(false, |set| set.remove(&v).is_some())
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
                bang_jensen_94_weighted_usize,
                bang_jensen_96_usize,
                bang_jensen_99,
                kattis_bryr_1_usize,
                kattis_bryr_2_usize,
                kattis_bryr_3_usize,
                kattis_crosscountry_usize,
                kattis_shortestpath1_usize,
                kattis_shortestpath3,
            },
            op::{
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
                IsSubdigraph,
                IsSuperdigraph,
                IsSymmetric,
                IsTournament,
            },
        },
        proptest::proptest,
    };

    proptest! {
        #[test]
        fn add_arc_weighted_arc_weight(u in 1..25_usize, v in 1..25_usize, w in 1..25_usize) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc_weighted(u, v, w);

            for x in digraph.vertices() {
                for y in digraph.vertices() {
                    assert_eq!(
                        digraph.arc_weight(x, y),
                        (x == u && y == v).then_some(&w)
                    );
                }
            }
        }

        #[test]
        fn add_arc_weighted_degree(u in 1..25_usize, v in 1..25_usize, w in 1..25_usize) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc_weighted(u, v, w);

            for x in digraph.vertices() {
                assert_eq!(digraph.degree(x), usize::from(x == u) + usize::from(x == v));
            }
        }

        #[test]
        fn add_arc_weighted_has_arc(u in 1..25_usize, v in 1..25_usize, w in 1..25_usize) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc_weighted(u, v, w);

            assert!(digraph.has_arc(u, v));
        }

        #[test]
        fn add_arc_weighted_indegree(u in 1..25_usize, v in 1..25_usize, w in 1..25_usize) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc_weighted(u, v, w);

            for u in digraph.vertices() {
                assert_eq!(digraph.indegree(u), usize::from(u == v));
            }
        }

        #[test]
        fn add_arc_weighted_outdegree(u in 1..25_usize, v in 1..25_usize, w in 1..25_usize) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc_weighted(u, v, w);

            for x in digraph.vertices() {
                assert_eq!(digraph.outdegree(x), usize::from(x == u));
            }
        }

        #[test]
        fn add_arc_weighted_remove_arc(u in 1..25_usize, v in 1..25_usize, w in 1..25_usize) {
            let d = Digraph::empty(100);
            let mut h = d.clone();

            h.add_arc_weighted(u, v, w);

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
        fn empty_arcs(order in 1..25_usize) {
            assert!(Digraph::<usize>::empty(order).arcs().eq([]));
        }

        #[test]
        fn empty_degree(order in 1..25_usize) {
            let digraph = Digraph::<usize>::empty(order);

            for u in digraph.vertices() {
                assert_eq!(digraph.degree(u), 0);
            }
        }

        #[test]
        fn empty_has_arc(order in 1..25_usize) {
            let digraph = Digraph::<usize>::empty(order);

            for u in digraph.vertices() {
                for v in digraph.vertices() {
                    assert!(!digraph.has_arc(u, v));
                }
            }
        }

        #[test]
        fn empty_has_edge(order in 1..25_usize) {
            let digraph = Digraph::<usize>::empty(order);

            for u in digraph.vertices() {
                for v in digraph.vertices() {
                    assert!(!digraph.has_edge(u, v));
                }
            }
        }

        #[test]
        fn empty_indegree(order in 1..25_usize) {
            let digraph = Digraph::<usize>::empty(order);

            for u in digraph.vertices() {
                assert_eq!(digraph.indegree(u), 0);
            }
        }

        #[test]
        fn empty_is_balanced(order in 1..25_usize) {
            assert!(Digraph::<usize>::empty(order).is_balanced());
        }

        #[test]
        fn empty_is_complete(order in 2..25_usize) {
            assert!(!Digraph::<usize>::empty(order).is_complete());
        }

        #[test]
        fn empty_is_isolated(order in 1..25_usize) {
            let digraph = Digraph::<usize>::empty(order);

            for u in digraph.vertices() {
                assert!(digraph.is_isolated(u));
            }
        }

        #[test]
        fn empty_is_oriented(order in 1..25_usize) {
            assert!(Digraph::<usize>::empty(order).is_oriented());
        }

        #[test]
        fn empty_is_pendant(order in 1..25_usize) {
            let digraph = Digraph::<usize>::empty(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_pendant(u));
            }
        }

        #[test]
        fn empty_is_regular(order in 1..25_usize) {
            assert!(Digraph::<usize>::empty(order).is_regular());
        }

        #[test]
        fn empty_is_semicomplete(order in 2..25_usize) {
            assert!(!Digraph::<usize>::empty(order).is_semicomplete());
        }

        #[test]
        fn empty_is_simple(order in 1..25_usize) {
            assert!(Digraph::<usize>::empty(order).is_simple());
        }

        #[test]
        fn empty_is_sink(order in 1..25_usize) {
            let digraph = Digraph::<usize>::empty(order);

            for u in digraph.vertices() {
                assert!(digraph.is_sink(u));
            }
        }

        #[test]
        fn empty_is_source(order in 1..25_usize) {
            let digraph = Digraph::<usize>::empty(order);

            for u in digraph.vertices() {
                assert!(digraph.is_source(u));
            }
        }

        #[test]
        fn empty_is_subdigraph(order in 1..25_usize) {
            let digraph = Digraph::<usize>::empty(order);

            assert!(digraph.is_subdigraph(&digraph));
        }

        #[test]
        fn empty_is_superdigraph(order in 1..25_usize) {
            let digraph = Digraph::<usize>::empty(order);

            assert!(digraph.is_superdigraph(&digraph));
        }

        #[test]
        fn empty_is_symmetric(order in 1..25_usize) {
            assert!(Digraph::<usize>::empty(order).is_symmetric());
        }

        #[test]
        fn empty_is_tournament(order in 2..25_usize) {
            assert!(!Digraph::<usize>::empty(order).is_tournament());
        }

        #[test]
        fn empty_outdegree(order in 1..25_usize) {
            let digraph = Digraph::<usize>::empty(order);

            for u in digraph.vertices() {
                assert_eq!(digraph.outdegree(u), 0);
            }
        }

        #[test]
        fn has_arc_out_of_bounds(order in 1..25_usize) {
            let digraph = Digraph::<isize>::empty(order);

            for u in 0..order {
                assert!(!digraph.has_arc(u, order));
                assert!(!digraph.has_arc(order, u));
            }
        }
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 1 but the index is 1")]
    fn add_weighted_arc_out_of_bounds() {
        Digraph::trivial().add_arc_weighted(1, 0, 1);
    }

    #[test]
    fn arcs_bang_jensen_94_weighted() {
        assert!(bang_jensen_94_weighted_usize().arcs().eq([
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
        assert!(bang_jensen_96_usize().arcs().eq([
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
        assert!(bang_jensen_99().arcs().eq([
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
        assert!(kattis_bryr_1_usize()
            .arcs()
            .eq([(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)]));
    }

    #[test]
    fn arcs_kattis_bryr_2() {
        assert!(kattis_bryr_2_usize().arcs().eq([
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
        assert!(kattis_bryr_3_usize().arcs().eq([
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
        assert!(kattis_crosscountry_usize().arcs().eq([
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
        assert!(kattis_shortestpath1_usize()
            .arcs()
            .eq([(0, 1), (1, 2), (3, 0)]));
    }

    #[test]
    fn arcs_weighted_bang_jensen_94_weighted() {
        assert!(bang_jensen_94_weighted_usize().arcs_weighted().eq([
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
        assert!(bang_jensen_96_usize().arcs_weighted().eq([
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
        assert!(bang_jensen_99().arcs_weighted().eq([
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
        assert!(kattis_bryr_1_usize().arcs_weighted().eq([
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
        assert!(kattis_bryr_2_usize().arcs_weighted().eq([
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
        assert!(kattis_bryr_3_usize().arcs_weighted().eq([
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
        assert!(kattis_crosscountry_usize().arcs_weighted().eq([
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
        assert!(kattis_shortestpath1_usize().arcs_weighted().eq([
            (0, 1, &2),
            (1, 2, &2),
            (3, 0, &2)
        ]));
    }

    #[test]
    fn converse_bang_jensen_94_weighted() {
        assert!(bang_jensen_94_weighted_usize()
            .converse()
            .arcs_weighted()
            .eq([
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
        assert!(bang_jensen_96_usize().converse().arcs_weighted().eq([
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
        assert!(bang_jensen_99().converse().arcs_weighted().eq([
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
        assert!(kattis_bryr_1_usize().converse().arcs_weighted().eq([
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
        assert!(kattis_bryr_2_usize().converse().arcs_weighted().eq([
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
        assert!(kattis_bryr_3_usize().converse().arcs_weighted().eq([
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
        assert!(kattis_crosscountry_usize().converse().arcs_weighted().eq([
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
        assert!(kattis_shortestpath1_usize().converse().arcs_weighted().eq([
            (0, 3, &2),
            (1, 0, &2),
            (2, 1, &2)
        ]));
    }

    #[test]
    #[should_panic(expected = "a digraph must have at least one vertex")]
    fn converse_0() {
        let digraph: Digraph<usize> = Digraph { arcs: vec![] };
        let _ = digraph.converse();
    }

    #[test]
    fn degree_bang_jensen_94_weighted() {
        let digraph = bang_jensen_94_weighted_usize();

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
        let digraph = bang_jensen_96_usize();

        assert!(digraph.degree(0) == 2);
        assert!(digraph.degree(1) == 4);
        assert!(digraph.degree(2) == 5);
        assert!(digraph.degree(3) == 4);
        assert!(digraph.degree(4) == 4);
        assert!(digraph.degree(5) == 3);
    }

    #[test]
    fn degree_bang_jensen_99() {
        let digraph = bang_jensen_99();

        assert!(digraph.degree(0) == 2);
        assert!(digraph.degree(1) == 2);
        assert!(digraph.degree(2) == 4);
        assert!(digraph.degree(3) == 4);
        assert!(digraph.degree(4) == 4);
        assert!(digraph.degree(5) == 4);
    }

    #[test]
    fn degree_kattis_bryr_1() {
        let digraph = kattis_bryr_1_usize();

        assert!(digraph.degree(0) == 4);
        assert!(digraph.degree(1) == 4);
        assert!(digraph.degree(2) == 4);
    }

    #[test]
    fn degree_kattis_bryr_2() {
        let digraph = kattis_bryr_2_usize();

        assert!(digraph.degree(0) == 4);
        assert!(digraph.degree(1) == 4);
        assert!(digraph.degree(2) == 4);
        assert!(digraph.degree(3) == 6);
        assert!(digraph.degree(4) == 4);
        assert!(digraph.degree(5) == 2);
    }

    #[test]
    fn degree_kattis_bryr_3() {
        let digraph = kattis_bryr_3_usize();

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
        let digraph = kattis_crosscountry_usize();

        assert!(digraph.degree(0) == 6);
        assert!(digraph.degree(1) == 6);
        assert!(digraph.degree(2) == 6);
        assert!(digraph.degree(3) == 6);
    }

    #[test]
    fn degree_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1_usize();

        assert!(digraph.degree(0) == 2);
        assert!(digraph.degree(1) == 2);
        assert!(digraph.degree(2) == 1);
        assert!(digraph.degree(3) == 1);
    }

    #[test]
    fn degree_sequence_bang_jensen_94_weighted() {
        assert!(bang_jensen_94_weighted_usize()
            .degree_sequence()
            .iter()
            .eq(&[(0, 2), (2, 1), (1, 4), (2, 1), (1, 1), (2, 0), (1, 0)]));
    }

    #[test]
    fn degree_sequence_bang_jensen_96() {
        assert!(bang_jensen_96_usize().degree_sequence().iter().eq(&[
            (0, 2),
            (2, 2),
            (3, 2),
            (3, 1),
            (1, 3),
            (2, 1)
        ]));
    }

    #[test]
    fn degree_sequence_bang_jensen_99() {
        assert!(bang_jensen_99().degree_sequence().iter().eq(&[
            (0, 2),
            (1, 1),
            (2, 2),
            (3, 1),
            (2, 2),
            (2, 2)
        ]));
    }

    #[test]
    fn degree_sequence_kattis_bryr_1() {
        assert!(kattis_bryr_1_usize()
            .degree_sequence()
            .iter()
            .eq(&[(2, 2), (2, 2), (2, 2)]));
    }

    #[test]
    fn degree_sequence_kattis_bryr_2() {
        assert!(kattis_bryr_2_usize().degree_sequence().iter().eq(&[
            (2, 2),
            (2, 2),
            (2, 2),
            (3, 3),
            (2, 2),
            (1, 1)
        ]));
    }

    #[test]
    fn degree_sequence_kattis_bryr_3() {
        assert!(kattis_bryr_3_usize().degree_sequence().iter().eq(&[
            (1, 1),
            (2, 2),
            (2, 1),
            (4, 4),
            (3, 3),
            (3, 3),
            (4, 4),
            (2, 2),
            (2, 2),
            (2, 3)
        ]));
    }

    #[test]
    fn degree_sequence_kattis_crosscountry() {
        assert!(kattis_crosscountry_usize().degree_sequence().iter().eq(&[
            (3, 3),
            (3, 3),
            (3, 3),
            (3, 3)
        ]));
    }

    #[test]
    fn degree_sequence_kattis_shortestpath1() {
        assert!(kattis_shortestpath1_usize().degree_sequence().iter().eq(&[
            (1, 1),
            (1, 1),
            (1, 0),
            (0, 1)
        ]));
    }

    #[test]
    fn degree_sequence_kattis_shortestpath3() {
        assert!(kattis_shortestpath3().degree_sequence().iter().eq(&[
            (0, 2),
            (2, 1),
            (1, 1),
            (1, 0),
            (0, 0)
        ]));
    }

    #[test]
    fn empty() {
        assert!(Digraph::<usize>::empty(1).arcs().eq([]));
        assert!(Digraph::<usize>::empty(2).arcs().eq([]));
        assert!(Digraph::<usize>::empty(3).arcs().eq([]));
    }

    #[test]
    #[should_panic(expected = "a digraph must have at least one vertex")]
    fn empty_0() {
        let _ = Digraph::<usize>::empty(0);
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
    fn empty_trivial() {
        assert!(Digraph::<usize>::trivial().arcs().eq([]));
    }

    #[test]
    fn empty_trivial_is_tournament() {
        assert!(Digraph::<usize>::trivial().is_tournament());
    }

    #[test]
    fn in_neighbors_bang_jensen_94_weighted() {
        let digraph = bang_jensen_94_weighted_usize();

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
        let digraph = bang_jensen_96_usize();

        assert!(digraph.in_neighbors(0).eq([]));
        assert!(digraph.in_neighbors(1).eq([0, 2]));
        assert!(digraph.in_neighbors(2).eq([0, 1, 4]));
        assert!(digraph.in_neighbors(3).eq([1, 4, 5]));
        assert!(digraph.in_neighbors(4).eq([2]));
        assert!(digraph.in_neighbors(5).eq([3, 4]));
    }

    #[test]
    fn in_neighbors_bang_jensen_99() {
        let digraph = bang_jensen_99();

        assert!(digraph.in_neighbors(0).eq([]));
        assert!(digraph.in_neighbors(1).eq([0]));
        assert!(digraph.in_neighbors(2).eq([0, 1]));
        assert!(digraph.in_neighbors(3).eq([2, 4, 5]));
        assert!(digraph.in_neighbors(4).eq([2, 5]));
        assert!(digraph.in_neighbors(5).eq([3, 4]));
    }

    #[test]
    fn in_neighbors_kattis_bryr_1() {
        let digraph = kattis_bryr_1_usize();

        assert!(digraph.in_neighbors(0).eq([1, 2]));
        assert!(digraph.in_neighbors(1).eq([0, 2]));
        assert!(digraph.in_neighbors(2).eq([0, 1]));
    }

    #[test]
    fn in_neighbors_kattis_bryr_2() {
        let digraph = kattis_bryr_2_usize();

        assert!(digraph.in_neighbors(0).eq([1, 3]));
        assert!(digraph.in_neighbors(1).eq([0, 2]));
        assert!(digraph.in_neighbors(2).eq([1, 3]));
        assert!(digraph.in_neighbors(3).eq([0, 2, 4]));
        assert!(digraph.in_neighbors(4).eq([3, 5]));
        assert!(digraph.in_neighbors(5).eq([4]));
    }

    #[test]
    fn in_neighbors_kattis_bryr_3() {
        let digraph = kattis_bryr_3_usize();

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
        let digraph = kattis_crosscountry_usize();

        assert!(digraph.in_neighbors(0).eq([1, 2, 3]));
        assert!(digraph.in_neighbors(1).eq([0, 2, 3]));
        assert!(digraph.in_neighbors(2).eq([0, 1, 3]));
        assert!(digraph.in_neighbors(3).eq([0, 1, 2]));
    }

    #[test]
    fn in_neighbors_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1_usize();

        assert!(digraph.in_neighbors(0).eq([3]));
        assert!(digraph.in_neighbors(1).eq([0]));
        assert!(digraph.in_neighbors(2).eq([1]));
        assert!(digraph.in_neighbors(3).eq([]));
    }

    #[test]
    fn indegree_bang_jensen_94_weighted() {
        let digraph = bang_jensen_94_weighted_usize();

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
        let digraph = bang_jensen_96_usize();

        assert!(digraph.indegree(0) == 0);
        assert!(digraph.indegree(1) == 2);
        assert!(digraph.indegree(2) == 3);
        assert!(digraph.indegree(3) == 3);
        assert!(digraph.indegree(4) == 1);
        assert!(digraph.indegree(5) == 2);
    }

    #[test]
    fn indegree_bang_jensen_99() {
        let digraph = bang_jensen_99();

        assert!(digraph.indegree(0) == 0);
        assert!(digraph.indegree(1) == 1);
        assert!(digraph.indegree(2) == 2);
        assert!(digraph.indegree(3) == 3);
        assert!(digraph.indegree(4) == 2);
        assert!(digraph.indegree(5) == 2);
    }

    #[test]
    fn indegree_kattis_bryr_1() {
        let digraph = kattis_bryr_1_usize();

        assert!(digraph.indegree(0) == 2);
        assert!(digraph.indegree(1) == 2);
        assert!(digraph.indegree(2) == 2);
    }

    #[test]
    fn indegree_kattis_bryr_2() {
        let digraph = kattis_bryr_2_usize();

        assert!(digraph.indegree(0) == 2);
        assert!(digraph.indegree(1) == 2);
        assert!(digraph.indegree(2) == 2);
        assert!(digraph.indegree(3) == 3);
        assert!(digraph.indegree(4) == 2);
        assert!(digraph.indegree(5) == 1);
    }

    #[test]
    fn indegree_kattis_bryr_3() {
        let digraph = kattis_bryr_3_usize();

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
        let digraph = kattis_crosscountry_usize();

        assert!(digraph.indegree(0) == 3);
        assert!(digraph.indegree(1) == 3);
        assert!(digraph.indegree(2) == 3);
        assert!(digraph.indegree(3) == 3);
    }

    #[test]
    fn indegree_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1_usize();

        assert!(digraph.indegree(0) == 1);
        assert!(digraph.indegree(1) == 1);
        assert!(digraph.indegree(2) == 1);
        assert!(digraph.indegree(3) == 0);
    }

    #[test]
    #[should_panic(expected = "v = 1 is out of bounds")]
    fn indegree_out_of_bounds() {
        let _ = Digraph::<usize>::trivial().indegree(1);
    }

    #[test]
    fn is_balanced_bang_jensen_94_weighted() {
        assert!(!bang_jensen_94_weighted_usize().is_balanced());
    }

    #[test]
    fn is_balanced_bang_jensen_96() {
        assert!(!bang_jensen_96_usize().is_balanced());
    }

    #[test]
    fn is_balanced_bang_jensen_99() {
        assert!(!bang_jensen_99().is_balanced());
    }

    #[test]
    fn is_balanced_kattis_bryr_1() {
        assert!(kattis_bryr_1_usize().is_balanced());
    }

    #[test]
    fn is_balanced_kattis_bryr_2() {
        assert!(kattis_bryr_2_usize().is_balanced());
    }

    #[test]
    fn is_balanced_kattis_bryr_3() {
        assert!(!kattis_bryr_3_usize().is_balanced());
    }

    #[test]
    fn is_balanced_kattis_crosscountry() {
        assert!(kattis_crosscountry_usize().is_balanced());
    }

    #[test]
    fn is_balanced_kattis_shortestpath1() {
        assert!(!kattis_shortestpath1_usize().is_balanced());
    }

    #[test]
    fn is_complete_bang_jensen_94_weighted() {
        assert!(!bang_jensen_94_weighted_usize().is_complete());
    }

    #[test]
    fn is_complete_bang_jensen_96() {
        assert!(!bang_jensen_96_usize().is_complete());
    }

    #[test]
    fn is_complete_bang_jensen_99() {
        assert!(!bang_jensen_99().is_complete());
    }

    #[test]
    fn is_complete_kattis_bryr_1() {
        assert!(kattis_bryr_1_usize().is_complete());
    }

    #[test]
    fn is_complete_kattis_bryr_2() {
        assert!(!kattis_bryr_2_usize().is_complete());
    }

    #[test]
    fn is_complete_kattis_bryr_3() {
        assert!(!kattis_bryr_3_usize().is_complete());
    }

    #[test]
    fn is_complete_kattis_crosscountry() {
        assert!(kattis_crosscountry_usize().is_complete());
    }

    #[test]
    fn is_complete_kattis_shortestpath1() {
        assert!(!kattis_shortestpath1_usize().is_complete());
    }

    #[test]
    fn is_isolated_bang_jensen_94_weighted() {
        let digraph = bang_jensen_94_weighted_usize();

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
        let digraph = bang_jensen_96_usize();

        assert!(!digraph.is_isolated(0));
        assert!(!digraph.is_isolated(1));
        assert!(!digraph.is_isolated(2));
        assert!(!digraph.is_isolated(3));
        assert!(!digraph.is_isolated(4));
        assert!(!digraph.is_isolated(5));
    }

    #[test]
    fn is_isolated_bang_jensen_99() {
        let digraph = bang_jensen_99();

        assert!(!digraph.is_isolated(0));
        assert!(!digraph.is_isolated(1));
        assert!(!digraph.is_isolated(2));
        assert!(!digraph.is_isolated(3));
        assert!(!digraph.is_isolated(4));
        assert!(!digraph.is_isolated(5));
    }

    #[test]
    fn is_isolated_kattis_bryr_1() {
        let digraph = kattis_bryr_1_usize();

        assert!(!digraph.is_isolated(0));
        assert!(!digraph.is_isolated(1));
        assert!(!digraph.is_isolated(2));
    }

    #[test]
    fn is_isolated_kattis_bryr_2() {
        let digraph = kattis_bryr_2_usize();

        assert!(!digraph.is_isolated(0));
        assert!(!digraph.is_isolated(1));
        assert!(!digraph.is_isolated(2));
        assert!(!digraph.is_isolated(3));
        assert!(!digraph.is_isolated(4));
        assert!(!digraph.is_isolated(5));
    }

    #[test]
    fn is_isolated_kattis_bryr_3() {
        let digraph = kattis_bryr_3_usize();

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
        let digraph = kattis_crosscountry_usize();

        assert!(!digraph.is_isolated(0));
        assert!(!digraph.is_isolated(1));
        assert!(!digraph.is_isolated(2));
        assert!(!digraph.is_isolated(3));
    }

    #[test]
    fn is_isolated_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1_usize();

        assert!(!digraph.is_isolated(0));
        assert!(!digraph.is_isolated(1));
        assert!(!digraph.is_isolated(2));
        assert!(!digraph.is_isolated(3));
    }

    #[test]
    fn is_oriented_bang_jensen_94_weighted() {
        assert!(bang_jensen_94_weighted_usize().is_oriented());
    }

    #[test]
    fn is_oriented_bang_jensen_96() {
        assert!(!bang_jensen_96_usize().is_oriented());
    }

    #[test]
    fn is_oriented_bang_jensen_99() {
        assert!(!bang_jensen_99().is_oriented());
    }

    #[test]
    fn is_oriented_kattis_bryr_1() {
        assert!(!kattis_bryr_1_usize().is_oriented());
    }

    #[test]
    fn is_oriented_kattis_bryr_2() {
        assert!(!kattis_bryr_2_usize().is_oriented());
    }

    #[test]
    fn is_oriented_kattis_bryr_3() {
        assert!(!kattis_bryr_3_usize().is_oriented());
    }

    #[test]
    fn is_oriented_kattis_crosscountry() {
        assert!(!kattis_crosscountry_usize().is_oriented());
    }

    #[test]
    fn is_oriented_kattis_shortestpath1() {
        assert!(kattis_shortestpath1_usize().is_oriented());
    }

    #[test]
    fn is_pendant_bang_jensen_94_weighted() {
        let digraph = bang_jensen_94_weighted_usize();

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
        let digraph = bang_jensen_96_usize();

        assert!(!digraph.is_pendant(0));
        assert!(!digraph.is_pendant(1));
        assert!(!digraph.is_pendant(2));
        assert!(!digraph.is_pendant(3));
        assert!(!digraph.is_pendant(4));
        assert!(!digraph.is_pendant(5));
    }

    #[test]
    fn is_pendant_bang_jensen_99() {
        let digraph = bang_jensen_99();

        assert!(!digraph.is_pendant(0));
        assert!(!digraph.is_pendant(1));
        assert!(!digraph.is_pendant(2));
        assert!(!digraph.is_pendant(3));
        assert!(!digraph.is_pendant(4));
        assert!(!digraph.is_pendant(5));
    }

    #[test]
    fn is_pendant_kattis_bryr_1() {
        let digraph = kattis_bryr_1_usize();

        assert!(!digraph.is_pendant(0));
        assert!(!digraph.is_pendant(1));
        assert!(!digraph.is_pendant(2));
    }

    #[test]
    fn is_pendant_kattis_bryr_2() {
        let digraph = kattis_bryr_2_usize();

        assert!(!digraph.is_pendant(0));
        assert!(!digraph.is_pendant(1));
        assert!(!digraph.is_pendant(2));
        assert!(!digraph.is_pendant(3));
        assert!(!digraph.is_pendant(4));
        assert!(!digraph.is_pendant(5));
    }

    #[test]
    fn is_pendant_kattis_bryr_3() {
        let digraph = kattis_bryr_3_usize();

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
        let digraph = kattis_crosscountry_usize();

        assert!(!digraph.is_pendant(0));
        assert!(!digraph.is_pendant(1));
        assert!(!digraph.is_pendant(2));
        assert!(!digraph.is_pendant(3));
    }

    #[test]
    fn is_pendant_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1_usize();

        assert!(!digraph.is_pendant(0));
        assert!(!digraph.is_pendant(1));
        assert!(digraph.is_pendant(2));
        assert!(digraph.is_pendant(3));
    }

    #[test]
    fn is_regular_bang_jensen_94_weighted() {
        assert!(!bang_jensen_94_weighted_usize().is_regular());
    }

    #[test]
    fn is_regular_bang_jensen_96() {
        assert!(!bang_jensen_96_usize().is_regular());
    }

    #[test]
    fn is_regular_bang_jensen_99() {
        assert!(!bang_jensen_99().is_regular());
    }

    #[test]
    fn is_regular_kattis_bryr_1() {
        assert!(kattis_bryr_1_usize().is_regular());
    }

    #[test]
    fn is_regular_kattis_bryr_2() {
        assert!(!kattis_bryr_2_usize().is_regular());
    }

    #[test]
    fn is_regular_kattis_bryr_3() {
        assert!(!kattis_bryr_3_usize().is_regular());
    }

    #[test]
    fn is_regular_kattis_crosscountry() {
        assert!(kattis_crosscountry_usize().is_regular());
    }

    #[test]
    fn is_regular_kattis_shortestpath1() {
        assert!(!kattis_shortestpath1_usize().is_regular());
    }

    #[test]
    fn is_semicomplete_bang_jensen_94_weighted() {
        assert!(!bang_jensen_94_weighted_usize().is_semicomplete());
    }

    #[test]
    fn is_semicomplete_bang_jensen_96() {
        assert!(!bang_jensen_96_usize().is_semicomplete());
    }

    #[test]
    fn is_semicomplete_bang_jensen_99() {
        assert!(!bang_jensen_99().is_semicomplete());
    }

    #[test]
    fn is_semicomplete_kattis_bryr_1() {
        assert!(kattis_bryr_1_usize().is_semicomplete());
    }

    #[test]
    fn is_semicomplete_kattis_bryr_2() {
        assert!(!kattis_bryr_2_usize().is_semicomplete());
    }

    #[test]
    fn is_semicomplete_kattis_bryr_3() {
        assert!(!kattis_bryr_3_usize().is_semicomplete());
    }

    #[test]
    fn is_semicomplete_kattis_crosscountry() {
        assert!(kattis_crosscountry_usize().is_semicomplete());
    }

    #[test]
    fn is_semicomplete_kattis_shortestpath1() {
        assert!(!kattis_shortestpath1_usize().is_semicomplete());
    }

    #[test]
    fn is_simple_bang_jensen_94_weighted() {
        assert!(bang_jensen_94_weighted_usize().is_simple());
    }

    #[test]
    fn is_simple_bang_jensen_96() {
        assert!(bang_jensen_96_usize().is_simple());
    }

    #[test]
    fn is_simple_bang_jensen_99() {
        assert!(bang_jensen_99().is_simple());
    }

    #[test]
    fn is_simple_kattis_bryr_1() {
        assert!(kattis_bryr_1_usize().is_simple());
    }

    #[test]
    fn is_simple_kattis_bryr_2() {
        assert!(kattis_bryr_2_usize().is_simple());
    }

    #[test]
    fn is_simple_kattis_bryr_3() {
        assert!(kattis_bryr_3_usize().is_simple());
    }

    #[test]
    fn is_simple_kattis_crosscountry() {
        assert!(kattis_crosscountry_usize().is_simple());
    }

    #[test]
    fn is_simple_kattis_shortestpath1() {
        assert!(kattis_shortestpath1_usize().is_simple());
    }

    #[test]
    fn is_symmetric_bang_jensen_94_weighted() {
        assert!(!bang_jensen_94_weighted_usize().is_symmetric());
    }

    #[test]
    fn is_symmetric_bang_jensen_96() {
        assert!(!bang_jensen_96_usize().is_symmetric());
    }

    #[test]
    fn is_symmetric_bang_jensen_99() {
        assert!(!bang_jensen_99().is_symmetric());
    }

    #[test]
    fn is_symmetric_kattis_bryr_1() {
        assert!(kattis_bryr_1_usize().is_symmetric());
    }

    #[test]
    fn is_symmetric_kattis_bryr_2() {
        assert!(kattis_bryr_2_usize().is_symmetric());
    }

    #[test]
    fn is_symmetric_kattis_bryr_3() {
        assert!(!kattis_bryr_3_usize().is_symmetric());
    }

    #[test]
    fn is_symmetric_kattis_crosscountry() {
        assert!(kattis_crosscountry_usize().is_symmetric());
    }

    #[test]
    fn is_symmetric_kattis_shortestpath1() {
        assert!(!kattis_shortestpath1_usize().is_symmetric());
    }

    #[test]
    fn is_tournament_bang_jensen_94_weighted() {
        assert!(!bang_jensen_94_weighted_usize().is_tournament());
    }

    #[test]
    fn is_tournament_bang_jensen_96() {
        assert!(!bang_jensen_96_usize().is_tournament());
    }

    #[test]
    fn is_tournament_bang_jensen_99() {
        assert!(!bang_jensen_99().is_tournament());
    }

    #[test]
    fn is_tournament_kattis_bryr_1() {
        assert!(!kattis_bryr_1_usize().is_tournament());
    }

    #[test]
    fn is_tournament_kattis_bryr_2() {
        assert!(!kattis_bryr_2_usize().is_tournament());
    }

    #[test]
    fn is_tournament_kattis_bryr_3() {
        assert!(!kattis_bryr_3_usize().is_tournament());
    }

    #[test]
    fn is_tournament_kattis_crosscountry() {
        assert!(!kattis_crosscountry_usize().is_tournament());
    }

    #[test]
    fn is_tournament_kattis_shortestpath1() {
        assert!(!kattis_shortestpath1_usize().is_tournament());
    }

    #[test]
    fn order_bang_jensen_94_weighted() {
        assert!(bang_jensen_94_weighted_usize().order() == 7);
    }

    #[test]
    fn order_bang_jensen_96() {
        assert!(bang_jensen_96_usize().order() == 6);
    }

    #[test]
    fn order_bang_jensen_99() {
        assert!(bang_jensen_99().order() == 6);
    }

    #[test]
    fn order_kattis_bryr_1() {
        assert!(kattis_bryr_1_usize().order() == 3);
    }

    #[test]
    fn order_kattis_bryr_2() {
        assert!(kattis_bryr_2_usize().order() == 6);
    }

    #[test]
    fn order_kattis_bryr_3() {
        assert!(kattis_bryr_3_usize().order() == 10);
    }

    #[test]
    fn order_kattis_crosscountry() {
        assert!(kattis_crosscountry_usize().order() == 4);
    }

    #[test]
    fn order_kattis_shortestpath1() {
        assert!(kattis_shortestpath1_usize().order() == 4);
    }

    #[test]
    fn out_neighbors_bang_jensen_94_weighted() {
        let digraph = bang_jensen_94_weighted_usize();

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
        let digraph = bang_jensen_96_usize();

        assert!(digraph.out_neighbors(0).eq([1, 2]));
        assert!(digraph.out_neighbors(1).eq([2, 3]));
        assert!(digraph.out_neighbors(2).eq([1, 4]));
        assert!(digraph.out_neighbors(3).eq([5]));
        assert!(digraph.out_neighbors(4).eq([2, 3, 5]));
        assert!(digraph.out_neighbors(5).eq([3]));
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 1 but the index is 1")]
    fn out_neighbors_out_of_bounds() {
        let _ = Digraph::<usize>::trivial().out_neighbors(1);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 1 but the index is 1")]
    fn out_neighbors_weighted_out_of_bounds() {
        let _ = Digraph::<usize>::trivial().out_neighbors_weighted(1);
    }

    #[test]
    fn outdegree_bang_jensen_94_weighted() {
        let digraph = bang_jensen_94_weighted_usize();

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
        let digraph = bang_jensen_96_usize();

        assert!(digraph.outdegree(0) == 2);
        assert!(digraph.outdegree(1) == 2);
        assert!(digraph.outdegree(2) == 2);
        assert!(digraph.outdegree(3) == 1);
        assert!(digraph.outdegree(4) == 3);
        assert!(digraph.outdegree(5) == 1);
    }

    #[test]
    fn outdegree_bang_jensen_99() {
        let digraph = bang_jensen_99();

        assert!(digraph.outdegree(0) == 2);
        assert!(digraph.outdegree(1) == 1);
        assert!(digraph.outdegree(2) == 2);
        assert!(digraph.outdegree(3) == 1);
        assert!(digraph.outdegree(4) == 2);
        assert!(digraph.outdegree(5) == 2);
    }

    #[test]
    fn outdegree_kattis_bryr_1() {
        let digraph = kattis_bryr_1_usize();

        assert!(digraph.outdegree(0) == 2);
        assert!(digraph.outdegree(1) == 2);
        assert!(digraph.outdegree(2) == 2);
    }

    #[test]
    fn outdegree_kattis_bryr_2() {
        let digraph = kattis_bryr_2_usize();

        assert!(digraph.outdegree(0) == 2);
        assert!(digraph.outdegree(1) == 2);
        assert!(digraph.outdegree(2) == 2);
        assert!(digraph.outdegree(3) == 3);
        assert!(digraph.outdegree(4) == 2);
        assert!(digraph.outdegree(5) == 1);
    }

    #[test]
    fn outdegree_kattis_bryr_3() {
        let digraph = kattis_bryr_3_usize();

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
        let digraph = kattis_crosscountry_usize();

        assert!(digraph.outdegree(0) == 3);
        assert!(digraph.outdegree(1) == 3);
        assert!(digraph.outdegree(2) == 3);
        assert!(digraph.outdegree(3) == 3);
    }

    #[test]
    fn outdegree_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1_usize();

        assert!(digraph.outdegree(0) == 1);
        assert!(digraph.outdegree(1) == 1);
        assert!(digraph.outdegree(2) == 0);
        assert!(digraph.outdegree(3) == 1);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 1 but the index is 1")]
    fn outdegree_out_of_bounds() {
        let _ = Digraph::<usize>::trivial().outdegree(1);
    }

    #[test]
    fn remove_arc_bang_jensen_94_weighted() {
        let mut digraph = bang_jensen_94_weighted_usize();

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
        let mut digraph = bang_jensen_96_usize();

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
        let mut digraph = bang_jensen_99();

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
        let mut digraph = kattis_bryr_1_usize();

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
        let mut digraph = kattis_bryr_2_usize();

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
        let mut digraph = kattis_bryr_3_usize();

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
        let mut digraph = kattis_crosscountry_usize();

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
        let mut digraph = kattis_shortestpath1_usize();

        assert!(digraph.remove_arc(0, 1));
        assert!(digraph.remove_arc(1, 2));
        assert!(digraph.remove_arc(3, 0));

        assert!(digraph.arcs().eq([]));

        assert!(!digraph.remove_arc(0, 1));
        assert!(!digraph.remove_arc(1, 2));
        assert!(!digraph.remove_arc(3, 0));
    }

    #[test]
    fn remove_arc_out_of_bounds() {
        assert!(!Digraph::<usize>::trivial().remove_arc(0, 1));
        assert!(!Digraph::<usize>::trivial().remove_arc(1, 0));
    }

    #[test]
    fn size_bang_jensen_94_weighted() {
        assert_eq!(bang_jensen_94_weighted_usize().size(), 9);
    }

    #[test]
    fn size_bang_jensen_96() {
        assert_eq!(bang_jensen_96_usize().size(), 11);
    }

    #[test]
    fn size_bang_jensen_99() {
        assert_eq!(bang_jensen_99().size(), 10);
    }

    #[test]
    fn size_kattis_bryr_1() {
        assert_eq!(kattis_bryr_1_usize().size(), 6);
    }

    #[test]
    fn size_kattis_bryr_2() {
        assert_eq!(kattis_bryr_2_usize().size(), 12);
    }

    #[test]
    fn size_kattis_bryr_3() {
        assert_eq!(kattis_bryr_3_usize().size(), 25);
    }

    #[test]
    fn size_kattis_crosscountry() {
        assert_eq!(kattis_crosscountry_usize().size(), 12);
    }

    #[test]
    fn size_kattis_shortestpath1() {
        assert_eq!(kattis_shortestpath1_usize().size(), 3);
    }
}

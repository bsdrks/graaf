//! An adjacency matrix representation for unweighted digraphs
//!
//! An adjacency matrix is a symmetric binary matrix where a value of `1` at
//! row `u` and column `v` indicates an arc from vertex `u` to vertex `v`. The
//! matrix is stored as a bit vector, and is suited for dense digraphs with a
//! small number of vertices.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_matrix::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         Arcs,
//!     },
//! };
//!
//! let mut digraph = Digraph::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//!
//! assert!(digraph.arcs().eq([(0, 1), (0, 2), (1, 0)]));
//! ```

use {
    crate::{
        adjacency_list,
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

/// An adjacency matrix representation of an unweighted digraph.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Digraph {
    blocks: Vec<usize>,
    order: usize,
}

impl Digraph {
    const fn mask(u: usize) -> usize {
        1 << (u & 63)
    }

    const fn index(&self, u: usize, v: usize) -> usize {
        u * self.order + v
    }

    /// Toggles the arc from the tail vertex to the head vertex.
    ///
    /// # Arguments
    ///
    /// * `u`: The tail vertex.
    /// * `v`: The head vertex.
    ///
    /// # Panics
    ///
    /// * Panics if `u` is out of bounds.
    /// * Panics if `v` is out of bounds.
    /// * Panics if `u` equals `v`.
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
    pub fn toggle(&mut self, u: usize, v: usize) {
        assert!(u < self.order, "u = {u} is out of bounds.");
        assert!(v < self.order, "v = {v} is out of bounds.");
        assert_ne!(u, v, "u = {u} equals v = {v}.");

        let i = self.index(u, v);

        self.blocks[i >> 6] ^= Self::mask(i);
    }
}

impl AddArc for Digraph {
    /// # Panics
    ///
    /// * Panics if `u` is out of bounds.
    /// * Panics if `v` is out of bounds.
    /// * Panics if `u` equals `v`.
    fn add_arc(&mut self, u: usize, v: usize) {
        assert!(u < self.order, "u = {u} is out of bounds.");
        assert!(v < self.order, "v = {v} is out of bounds.");
        assert_ne!(u, v, "u = {u} equals v = {v}.");

        let i = self.index(u, v);

        self.blocks[i >> 6] |= Self::mask(i);
    }
}

impl ArcWeight<usize> for Digraph {
    fn arc_weight(&self, u: usize, v: usize) -> Option<&usize> {
        self.has_arc(u, v).then_some(&1)
    }
}

impl Arcs for Digraph {
    fn arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.vertices().flat_map(move |u| {
            self.vertices()
                .filter_map(move |v| self.has_arc(u, v).then_some((u, v)))
        })
    }
}

impl ArcsWeighted<usize> for Digraph {
    fn arcs_weighted<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a usize)>
    where
        usize: 'a,
    {
        self.arcs().map(|(u, v)| (u, v, &1))
    }
}

impl Converse for Digraph {
    fn converse(&self) -> Self {
        let mut converse = Self::empty(self.order);

        for (u, v) in self.arcs() {
            converse.add_arc(v, u);
        }

        converse
    }
}

impl Empty for Digraph {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn empty(order: usize) -> Self {
        assert!(order > 0, "a digraph must have at least one vertex");

        let n = (order * order + 63) / 64;

        Self {
            blocks: vec![0; n],
            order,
        }
    }
}

impl From<adjacency_list::Digraph> for Digraph {
    fn from(d: adjacency_list::Digraph) -> Self {
        let order = d.order();
        let mut h = Self::empty(order);

        for (u, v) in d.arcs() {
            h.add_arc(u, v);
        }

        h
    }
}

impl From<Vec<BTreeSet<usize>>> for Digraph {
    /// # Panics
    ///
    /// * Panics if for any arc `u -> v` in `arcs`, `u` equals `v`.
    /// * Panics if for any arc `u -> v` in `arcs`, `v` is out of bounds.
    fn from(vec: Vec<BTreeSet<usize>>) -> Self {
        let order = vec.len();
        let mut digraph = Self::empty(order);

        for (u, set) in vec.into_iter().enumerate() {
            for v in set {
                digraph.add_arc(u, v);
            }
        }

        digraph
    }
}

impl HasArc for Digraph {
    fn has_arc(&self, u: usize, v: usize) -> bool {
        if u >= self.order || v >= self.order {
            return false;
        }

        let i = self.index(u, v);

        self.blocks[i >> 6] & Self::mask(i) != 0
    }
}

impl Indegree for Digraph {
    /// # Panics
    ///
    /// Panics if `v` is out of bounds.
    fn indegree(&self, v: usize) -> usize {
        assert!(v < self.order, "v = {v} is out of bounds.");

        self.vertices().filter(|&u| self.has_arc(u, v)).count()
    }
}

impl IsSimple for Digraph {
    // We only check for self-loops. Parallel arcs can not exist in this
    // representation.
    fn is_simple(&self) -> bool {
        self.vertices().all(|u| !self.has_arc(u, u))
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
    /// Panics if `u` is out of bounds.
    fn out_neighbors(&self, u: usize) -> impl Iterator<Item = usize> {
        assert!(u < self.order, "u = {u} is out of bounds.");

        self.vertices().filter(move |&v| self.has_arc(u, v))
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
        assert!(u < self.order, "u = {u} is out of bounds.");

        self.vertices().filter(|&v| self.has_arc(u, v)).count()
    }
}

impl RemoveArc for Digraph {
    fn remove_arc(&mut self, u: usize, v: usize) -> bool {
        if u >= self.order || v >= self.order {
            return false;
        }

        let has_arc = self.has_arc(u, v);
        let i = self.index(u, v);

        self.blocks[i >> 6] &= !Self::mask(i);

        has_arc
    }
}

impl Size for Digraph {
    /// # Panics
    ///
    /// Panics if the number of arcs is greater than `usize::MAX`.
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
                Biclique,
                Circuit,
                Complete,
                Cycle,
                Empty,
                RandomTournament,
                Star,
            },
            op::{
                Complement,
                Degree,
                HasEdge,
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
                IsWalk,
                Order,
                SemidegreeSequence,
            },
            proptest_strategy::arc,
            r#gen::Path,
        },
        proptest::proptest,
    };

    proptest! {
        #[test]
        fn add_arc_arc_weight((u, v) in arc()) {
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
        fn add_arc_degree((u, v) in arc()) {
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
        fn add_arc_has_arc((u, v) in arc()) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc(u, v);

            assert!(digraph.has_arc(u, v));
        }

        #[test]
        fn add_arc_indegree((u, v) in arc()) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc(u, v);

            for x in digraph.vertices() {
                assert_eq!(digraph.indegree(x), usize::from(x == v));
            }
        }

        #[test]
        fn add_arc_outdegree((u, v) in arc()) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc(u, v);

            for x in digraph.vertices() {
                assert_eq!(digraph.outdegree(x), usize::from(x == u));
            }
        }

        #[test]
        fn add_arc_remove_arc((u, v) in arc()) {
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
        fn biclique_1_n_equals_star_n_plus_1(n in 1..25_usize) {
            assert_eq!(Digraph::biclique(1, n), Digraph::star(n + 1));
        }

        #[test]
        fn biclique_complement_size(m in 1..25_usize, n in 1..25_usize) {
            assert_eq!(
                Digraph::biclique(m, n).complement().size(),
                m * (m - 1) + n * (n - 1)
            );
        }

        #[test]
        fn biclique_degree(m in 1..25_usize, n in 1..25_usize) {
            let digraph = Digraph::biclique(m, n);

            for u in 0..m {
                assert_eq!(digraph.degree(u), n * 2);
            }

            for u in m..m + n {
                assert_eq!(digraph.degree(u), m * 2);
            }
        }

        #[test]
        fn biclique_degree_sum_equals_2size(m in 1..25_usize, n in 1..25_usize) {
            let digraph = Digraph::biclique(m, n);

            assert_eq!(
                digraph.vertices().fold(0, |acc, u| acc + digraph.degree(u)),
                2 * digraph.size()
            );
        }

        #[test]
        fn biclique_even_number_odd_degrees(m in 1..25_usize, n in 1..25_usize) {
            let digraph = Digraph::biclique(m, n);

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
            let digraph = Digraph::biclique(m, n);
            let order = m + n;

            for u in 0..m {
                for v in 0..m {
                    assert!(!digraph.has_arc(u, v));
                }
            }

            for u in m..order {
                for v in m..order {
                    assert!(!digraph.has_arc(u, v));
                }
            }

            for u in 0..m {
                for v in m..order {
                    assert!(digraph.has_arc(u, v));
                    assert!(digraph.has_arc(v, u));
                }
            }
        }

        #[test]
        fn biclique_has_edge(m in 1..25_usize, n in 1..25_usize) {
            let digraph = Digraph::biclique(m, n);
            let order = m + n;

            for u in 0..m {
                for v in 0..m {
                    assert!(!digraph.has_edge(u, v));
                }
            }

            for u in m..order {
                for v in m..order {
                    assert!(!digraph.has_edge(u, v));
                }
            }

            for u in 0..m {
                for v in m..order {
                    assert!(digraph.has_edge(u, v));
                }
            }
        }

        #[test]
        fn biclique_in_neighbors(m in 1..25_usize, n in 1..25_usize) {
            let digraph = Digraph::biclique(m, n);
            let order = m + n;

            for u in 0..m {
                assert!(digraph.in_neighbors(u).eq(m..order));
            }

            for u in m..order {
                assert!(digraph.in_neighbors(u).eq(0..m));
            }
        }

        #[test]
        fn biclique_indegree(m in 1..25_usize, n in 1..25_usize) {
            let digraph = Digraph::biclique(m, n);

            for u in 0..m {
                assert_eq!(digraph.indegree(u), n);
            }

            for u in m..m + n {
                assert_eq!(digraph.indegree(u), m);
            }
        }

        #[test]
        fn biclique_indegree_sequence(m in 1..25_usize, n in 1..25_usize) {
            let digraph = Digraph::biclique(m, n);
            let indegree_sequence = &mut digraph.indegree_sequence();

            assert!(indegree_sequence.take(m).all(|d| d == n));
            assert!(indegree_sequence.all(|d| d == m));
        }

        #[test]
        fn biclique_is_balanced(m in 1..25_usize, n in 1..25_usize) {
            assert!(Digraph::biclique(m, n).is_balanced());
        }

        #[test]
        fn biclique_is_complete(m in 2..25_usize, n in 2..25_usize) {
            assert!(!Digraph::biclique(m, n).is_complete());
        }

        #[test]
        fn biclique_is_isolated(m in 1..25_usize, n in 1..25_usize) {
            let digraph = Digraph::biclique(m, n);

            for u in digraph.vertices() {
                assert!(!digraph.is_isolated(u));
            }
        }

        #[test]
        fn biclique_is_oriented(m in 1..25_usize, n in 1..25_usize) {
            assert!(!Digraph::biclique(m, n).is_oriented());
        }

        #[test]
        fn biclique_is_pendant(m in 1..25_usize, n in 1..25_usize) {
            let digraph = Digraph::biclique(m, n);

            for u in digraph.vertices() {
                assert!(!digraph.is_pendant(u));
            }
        }

        #[test]
        fn biclique_is_regular(m in 1..25_usize, n in 1..25_usize) {
            assert!(Digraph::biclique(m, n).is_regular() == (m == n));
        }

        #[test]
        fn biclique_is_semicomplete(m in 2..25_usize, n in 2..25_usize) {
            assert!(!Digraph::biclique(m, n).is_semicomplete());
        }

        #[test]
        fn biclique_is_simple(m in 1..25_usize, n in 1..25_usize) {
            assert!(Digraph::biclique(m, n).is_simple());
        }

        #[test]
        fn biclique_is_spanning_subdigraph(m in 1..25_usize, n in 1..25_usize) {
            let digraph = Digraph::biclique(m, n);

            assert!(digraph.is_spanning_subdigraph(&digraph));
        }

        #[test]
        fn biclique_is_subdigraph(m in 1..25_usize, n in 1..25_usize) {
            let digraph = Digraph::biclique(m, n);

            assert!(digraph.is_subdigraph(&digraph));
        }

        #[test]
        fn biclique_is_superdigraph(m in 1..25_usize, n in 1..25_usize) {
            let digraph = Digraph::biclique(m, n);

            assert!(digraph.is_superdigraph(&digraph));
        }

        #[test]
        fn biclique_is_symmetric(m in 1..25_usize, n in 1..25_usize) {
            assert!(Digraph::biclique(m, n).is_symmetric());
        }

        #[test]
        fn biclique_is_tournament(m in 1..25_usize, n in 1..25_usize) {
            assert!(!Digraph::biclique(m, n).is_tournament());
        }

        #[test]
        fn biclique_order(m in 1..25_usize, n in 1..25_usize) {
            assert_eq!(Digraph::biclique(m, n).order(), m + n);
        }

        #[test]
        fn biclique_out_neighbors(m in 1..25_usize, n in 1..25_usize) {
            let digraph = Digraph::biclique(m, n);
            let order = m + n;

            for u in 0..m {
                assert!(digraph.out_neighbors(u).eq(m..order));
            }

            for u in m..order {
                assert!(digraph.out_neighbors(u).eq(0..m));
            }
        }

        #[test]
        fn biclique_out_neighbors_weighted(m in 1..25_usize, n in 1..25_usize) {
            let digraph = Digraph::biclique(m, n);
            let order = m + n;

            for u in 0..m {
                assert!(
                    digraph
                        .out_neighbors_weighted(u)
                        .eq((m..order).map(|v| (v, &1))
                    )
                );
            }

            for u in m..order {
                assert!(
                    digraph
                        .out_neighbors_weighted(u)
                        .eq((0..m).map(|v| (v, &1))
                    )
                );
            }
        }

        #[test]
        fn biclique_outdegree(m in 1..25_usize, n in 1..25_usize) {
            let digraph = Digraph::biclique(m, n);

            for u in 0..m {
                assert_eq!(digraph.outdegree(u), n);
            }

            for u in m..m + n {
                assert_eq!(digraph.outdegree(u), m);
            }
        }

        #[test]
        fn biclique_semidegree_sequence(m in 1..25_usize, n in 1..25_usize) {
            let digraph = Digraph::biclique(m, n);
            let semidegree_sequence = &mut digraph.semidegree_sequence();

            assert!(semidegree_sequence.take(m).all(|d| d == (n, n)));
            assert!(semidegree_sequence.all(|d| d == (m, m)));
        }

        #[test]
        fn biclique_size(m in 1..25_usize, n in 1..25_usize) {
            assert_eq!(Digraph::biclique(m, n).size(), m * n * 2);
        }

        #[test]
        fn circuit_complement_size(order in 1..25_usize) {
            assert_eq!(
                Digraph::circuit(order).complement().size(),
                order * order.saturating_sub(2)
            );
        }

        #[test]
        fn circuit_degree(order in 2..25_usize) {
            let digraph = Digraph::circuit(order);

            for u in digraph.vertices() {
                assert_eq!(digraph.degree(u), 2);
            }
        }

        #[test]
        fn circuit_degree_sum_equals_2size(order in 1..25_usize) {
            let digraph = Digraph::circuit(order);

            assert_eq!(
                digraph.vertices().fold(0, |acc, u| acc + digraph.degree(u)),
                2 * digraph.size()
            );
        }

        #[test]
        fn circuit_even_number_odd_degrees(order in 3..25_usize) {
            let digraph = Digraph::circuit(order);

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
        fn circuit_has_edge(order in 3..25_usize) {
            let digraph = Digraph::circuit(order);

            for u in 0..order {
                for v in u + 1..order {
                    assert!(!digraph.has_edge(u, v));
                }
            }
        }

        #[test]
        fn circuit_indegree(order in 2..25_usize) {
            let digraph = Digraph::circuit(order);

            for u in digraph.vertices() {
                assert_eq!(digraph.indegree(u), 1);
            }
        }

        #[test]
        fn circuit_indegree_sequence(order in 2..25_usize) {
            assert!(Digraph::circuit(order).indegree_sequence().all(|d| d == 1));
        }

        #[test]
        fn circuit_is_balanced(order in 1..25_usize) {
            assert!(Digraph::circuit(order).is_balanced());
        }

        #[test]
        fn circuit_is_complete(order in 3..25_usize) {
            assert!(!Digraph::circuit(order).is_complete());
        }

        #[test]
        fn circuit_is_isolated(order in 2..25_usize) {
            let digraph = Digraph::circuit(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_isolated(u));
            }
        }

        #[test]
        fn circuit_is_oriented(order in 3..25_usize) {
            assert!(Digraph::circuit(order).is_oriented());
        }

        #[test]
        fn circuit_is_pendant(order in 1..25_usize) {
            let digraph = Digraph::circuit(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_pendant(u));
            }
        }

        #[test]
        fn circuit_is_regular(order in 1..25_usize) {
            assert!(Digraph::circuit(order).is_regular());
        }

        #[test]
        fn circuit_is_semicomplete(order in 4..25_usize) {
            assert!(!Digraph::circuit(order).is_semicomplete());
        }

        #[test]
        fn circuit_is_simple(order in 2..25_usize) {
            assert!(Digraph::circuit(order).is_simple());
        }

        #[test]
        fn circuit_is_sink(order in 2..25_usize) {
            let digraph = Digraph::circuit(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_sink(u));
            }
        }

        #[test]
        fn circuit_is_source(order in 2..25_usize) {
            let digraph = Digraph::circuit(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_source(u));
            }
        }

        #[test]
        fn circuit_is_spanning_subdigraph(order in 1..25_usize) {
            let digraph = Digraph::circuit(order);

            assert!(digraph.is_spanning_subdigraph(&digraph));
        }

        #[test]
        fn circuit_is_subdigraph(order in 1..25_usize) {
            let digraph = Digraph::circuit(order);

            assert!(digraph.is_subdigraph(&digraph));
        }

        #[test]
        fn circuit_is_superdigraph(order in 1..25_usize) {
            let digraph = Digraph::circuit(order);

            assert!(digraph.is_superdigraph(&digraph));
        }

        #[test]
        fn circuit_is_symmetric(order in 3..25_usize) {
            assert!(!Digraph::circuit(order).is_symmetric());
        }

        #[test]
        fn circuit_is_tournament(order in 4..25_usize) {
            assert!(!Digraph::circuit(order).is_tournament());
        }

        #[test]
        fn circuit_semidegree_sequence(order in 2..25_usize) {
            assert!(Digraph::circuit(order).semidegree_sequence().all(|d| d == (1, 1)));
        }

        #[test]
        fn complete_complement_equals_empty(order in 1..25_usize) {
            assert_eq!(
                Digraph::complete(order).complement(),
                Digraph::empty(order)
            );
        }

        #[test]
        fn complete_degree(order in 1..25_usize) {
            let digraph = Digraph::complete(order);

            for u in digraph.vertices() {
                assert_eq!(digraph.degree(u), order * 2 - 2);
            }
        }

        #[test]
        fn complete_degree_sum_equals_2size(order in 1..25_usize) {
            let digraph = Digraph::complete(order);

            assert_eq!(
                digraph.vertices().fold(0, |acc, u| acc + digraph.degree(u)),
                2 * digraph.size()
            );
        }

        #[test]
        fn complete_even_number_odd_degrees(order in 1..25_usize) {
            let digraph = Digraph::complete(order);

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
        fn complete_has_edge(order in 2..25_usize) {
            let digraph = Digraph::complete(order);

            for u in 0..order {
                for v in u + 1..order {
                    assert!(digraph.has_edge(u, v));
                }
            }
        }

        #[test]
        fn complete_indegree(order in 1..25_usize) {
            let digraph = Digraph::complete(order);

            for u in digraph.vertices() {
                assert_eq!(digraph.indegree(u), order - 1);
            }
        }

        #[test]
        fn complete_indegree_sequence(order in 1..25_usize) {
            assert!(Digraph::complete(order).indegree_sequence().all(|d| d == order - 1));
        }

        #[test]
        fn complete_is_balanced(order in 1..25_usize) {
            assert!(Digraph::complete(order).is_balanced());
        }

        #[test]
        fn complete_is_complete(order in 1..25_usize) {
            assert!(Digraph::complete(order).is_complete());
        }

        #[test]
        fn complete_is_isolated(order in 2..25_usize) {
            let digraph = Digraph::complete(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_isolated(u));
            }
        }

        #[test]
        fn complete_is_oriented(order in 2..25_usize) {
            assert!(!Digraph::complete(order).is_oriented());
        }

        #[test]
        fn complete_is_pendant(order in 1..25_usize) {
            let digraph = Digraph::complete(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_pendant(u));
            }
        }

        #[test]
        fn complete_is_regular(order in 1..25_usize) {
            assert!(Digraph::complete(order).is_regular());
        }

        #[test]
        fn complete_is_semicomplete(order in 1..25_usize) {
            assert!(Digraph::complete(order).is_semicomplete());
        }

        #[test]
        fn complete_is_simple(order in 2..25_usize) {
            assert!(Digraph::complete(order).is_simple());
        }

        #[test]
        fn complete_is_spanning_subdigraph(order in 1..25_usize) {
            let digraph = Digraph::complete(order);

            assert!(digraph.is_spanning_subdigraph(&digraph));
        }

        #[test]
        fn complete_is_subdigraph(order in 1..25_usize) {
            let digraph = Digraph::complete(order);

            assert!(digraph.is_subdigraph(&digraph));
        }

        #[test]
        fn complete_is_superdigraph(order in 1..25_usize) {
            let digraph = Digraph::complete(order);

            assert!(digraph.is_superdigraph(&digraph));
        }

        #[test]
        fn complete_is_symmetric(order in 1..25_usize) {
            assert!(Digraph::complete(order).is_symmetric());
        }

        #[test]
        fn complete_is_tournament(order in 2..25_usize) {
            assert!(!Digraph::complete(order).is_tournament());
        }

        #[test]
        fn complete_order(order in 1..25_usize) {
            assert_eq!(Digraph::complete(order).order(), order);
        }

        #[test]
        fn complete_outdegree(order in 1..25_usize) {
            let digraph = Digraph::complete(order);

            for u in digraph.vertices() {
                assert_eq!(digraph.outdegree(u), order - 1);
            }
        }

        #[test]
        fn complete_size(order in 1..25_usize) {
            assert_eq!(Digraph::complete(order).size(), order * (order - 1));
        }

        #[test]
        fn cycle_complement_size(order in 1..25_usize) {
            assert_eq!(
                Digraph::cycle(order).complement().size(),
                order * order.saturating_sub(3)
            );
        }

        #[test]
        fn cycle_degree(order in 3..25_usize) {
            let digraph = Digraph::cycle(order);

            for u in digraph.vertices() {
                assert_eq!(digraph.degree(u), 4);
            }
        }

        #[test]
        fn cycle_degree_sum_equals_2size(order in 1..25_usize) {
            let digraph = Digraph::cycle(order);

            assert_eq!(
                digraph.vertices().fold(0, |acc, u| acc + digraph.degree(u)),
                2 * digraph.size()
            );
        }

        #[test]
        fn cycle_even_number_odd_degrees(order in 1..25_usize) {
            let digraph = Digraph::cycle(order);

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
        fn cycle_indegree(order in 3..25_usize) {
            let digraph = Digraph::cycle(order);

            for u in digraph.vertices() {
                assert_eq!(digraph.indegree(u), 2);
            }
        }

        #[test]
        fn cycle_indegree_sequence(order in 3..25_usize) {
            assert!(Digraph::cycle(order).indegree_sequence().all(|d| d == 2));
        }

        #[test]
        fn cycle_is_balanced(order in 1..25_usize) {
            assert!(Digraph::cycle(order).is_balanced());
        }

        #[test]
        fn cycle_is_complete(order in 4..25_usize) {
            assert!(!Digraph::cycle(order).is_complete());
        }

        #[test]
        fn cycle_is_isolated(order in 2..25_usize) {
            let digraph = Digraph::cycle(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_isolated(u));
            }
        }

        #[test]
        fn cycle_is_oriented(order in 2..25_usize) {
            assert!(!Digraph::cycle(order).is_oriented());
        }

        #[test]
        fn cycle_is_pendant(order in 1..25_usize) {
            let digraph = Digraph::cycle(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_pendant(u));
            }
        }

        #[test]
        fn cycle_is_regular(order in 1..25_usize) {
            assert!(Digraph::cycle(order).is_regular());
        }

        #[test]
        fn cycle_is_semicomplete(order in 4..25_usize) {
            assert!(!Digraph::cycle(order).is_semicomplete());
        }

        #[test]
        fn cycle_is_simple(order in 1..25_usize) {
            assert!(Digraph::cycle(order).is_simple());
        }

        #[test]
        fn cycle_is_sink(order in 2..25_usize) {
            let digraph = Digraph::cycle(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_sink(u));
            }
        }

        #[test]
        fn cycle_is_source(order in 2..25_usize) {
            let digraph = Digraph::cycle(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_source(u));
            }
        }

        #[test]
        fn cycle_is_spanning_subdigraph(order in 1..25_usize) {
            let digraph = Digraph::cycle(order);

            assert!(digraph.is_spanning_subdigraph(&digraph));
        }

        #[test]
        fn cycle_is_subdigraph(order in 1..25_usize) {
            let digraph = Digraph::cycle(order);

            assert!(digraph.is_subdigraph(&digraph));
        }

        #[test]
        fn cycle_is_superdigraph(order in 1..25_usize) {
            let digraph = Digraph::cycle(order);

            assert!(digraph.is_superdigraph(&digraph));
        }

        #[test]
        fn cycle_is_symmetric(order in 1..25_usize) {
            assert!(Digraph::cycle(order).is_symmetric());
        }

        #[test]
        fn cycle_is_tournament(order in 2..25_usize) {
            assert!(!Digraph::cycle(order).is_tournament());
        }

        #[test]
        fn cycle_semidegree_sequence(order in 3..25_usize) {
            assert!(
                Digraph::cycle(order).semidegree_sequence().all(|d| d == (2, 2))
            );
        }

        #[test]
        fn empty_complement_equals_complete(order in 1..25_usize) {
            assert_eq!(
                Digraph::empty(order).complement(),
                Digraph::complete(order)
            );
        }

        #[test]
        fn empty_arcs(order in 1..25_usize) {
            assert!(Digraph::empty(order).arcs().eq([]));
        }

        #[test]
        fn empty_degree(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            for u in digraph.vertices() {
                assert_eq!(digraph.degree(u), 0);
            }
        }

        #[test]
        fn empty_has_arc(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            for u in digraph.vertices() {
                for v in digraph.vertices() {
                    assert!(!digraph.has_arc(u, v));
                }
            }
        }

        #[test]
        fn empty_has_edge(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            for u in digraph.vertices() {
                for v in digraph.vertices() {
                    assert!(!digraph.has_edge(u, v));
                }
            }
        }

        #[test]
        fn empty_indegree(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            for u in digraph.vertices() {
                assert_eq!(digraph.indegree(u), 0);
            }
        }

        #[test]
        fn empty_indegree_sequence(order in 1..25_usize) {
            assert!(Digraph::empty(order).indegree_sequence().all(|d| d == 0));
        }

        #[test]
        fn empty_is_balanced(order in 1..25_usize) {
            assert!(Digraph::empty(order).is_balanced());
        }

        #[test]
        fn empty_is_complete(order in 2..25_usize) {
            assert!(!Digraph::empty(order).is_complete());
        }

        #[test]
        fn empty_is_isolated(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            for u in digraph.vertices() {
                assert!(digraph.is_isolated(u));
            }
        }

        #[test]
        fn empty_is_oriented(order in 1..25_usize) {
            assert!(Digraph::empty(order).is_oriented());
        }

        #[test]
        fn empty_is_pendant(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_pendant(u));
            }
        }

        #[test]
        fn empty_is_regular(order in 1..25_usize) {
            assert!(Digraph::empty(order).is_regular());
        }

        #[test]
        fn empty_is_semicomplete(order in 2..25_usize) {
            assert!(!Digraph::empty(order).is_semicomplete());
        }

        #[test]
        fn empty_is_simple(order in 1..25_usize) {
            assert!(Digraph::empty(order).is_simple());
        }

        #[test]
        fn empty_is_sink(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            for u in digraph.vertices() {
                assert!(digraph.is_sink(u));
            }
        }

        #[test]
        fn empty_is_source(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            for u in digraph.vertices() {
                assert!(digraph.is_source(u));
            }
        }

        #[test]
        fn empty_is_spanning_subdigraph(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            assert!(digraph.is_spanning_subdigraph(&digraph));
        }

        #[test]
        fn empty_is_subdigraph(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            assert!(digraph.is_subdigraph(&digraph));
        }

        #[test]
        fn empty_is_superdigraph(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            assert!(digraph.is_superdigraph(&digraph));
        }

        #[test]
        fn empty_is_symmetric(order in 1..25_usize) {
            assert!(Digraph::empty(order).is_symmetric());
        }

        #[test]
        fn empty_is_tournament(order in 2..25_usize) {
            assert!(!Digraph::empty(order).is_tournament());
        }

        #[test]
        fn empty_outdegree(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            for u in digraph.vertices() {
                assert_eq!(digraph.outdegree(u), 0);
            }
        }

        #[test]
        fn empty_semidegree_sequence(order in 1..25_usize) {
            assert!(
                Digraph::empty(order).semidegree_sequence().all(|d| d == (0, 0))
            );
        }

        #[test]
        fn has_arc_out_of_bounds(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            for u in 0..order {
                assert!(!digraph.has_arc(u, order));
                assert!(!digraph.has_arc(order, u));
            }
        }

        #[test]
        fn path_complement_size(order in 1..25_usize) {
            assert_eq!(
                Digraph::path(order).complement().size(),
                (order - 1).pow(2)
            );
        }

        #[test]
        fn path_degree(order in 2..25_usize) {
            let digraph = Digraph::path(order);
            let last = order - 1;

            assert_eq!(digraph.degree(0), 1);

            for u in 1..last {
                assert_eq!(digraph.degree(u), 2);
            }

            assert_eq!(digraph.degree(last), 1);
        }

        #[test]
        fn path_degree_sum_equals_2size(order in 1..25_usize) {
            let digraph = Digraph::path(order);

            assert_eq!(
                digraph.vertices().fold(0, |acc, u| acc + digraph.degree(u)),
                2 * digraph.size()
            );
        }

        #[test]
        fn path_even_number_odd_degrees(order in 1..25_usize) {
            let digraph = Digraph::path(order);

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
        fn path_has_edge(order in 2..25_usize) {
            let digraph = Digraph::path(order);

            for u in 0..order {
                for v in u..order {
                    assert!(!digraph.has_edge(u, v));
                }
            }
        }

        #[test]
        fn path_indegree(order in 1..25_usize) {
            let digraph = Digraph::path(order);

            assert_eq!(digraph.indegree(0), 0);

            for u in 1..order {
                assert_eq!(digraph.indegree(u), 1);
            }
        }

        #[test]
        fn path_indegree_sequence(order in 1..25_usize) {
            let digraph = Digraph::path(order);
            let indegree_sequence = &mut digraph.indegree_sequence();

            assert_eq!(indegree_sequence.next(), Some(0));
            assert!(indegree_sequence.all(|d| d == 1));
        }

        #[test]
        fn path_is_balanced(order in 2..25_usize) {
            assert!(!Digraph::path(order).is_balanced());
        }

        #[test]
        fn path_is_complete(order in 2..25_usize) {
            assert!(!Digraph::path(order).is_complete());
        }

        #[test]
        fn path_is_isolated(order in 2..25_usize) {
            let digraph = Digraph::path(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_isolated(u));
            }
        }

        #[test]
        fn path_is_oriented(order in 1..25_usize) {
            assert!(Digraph::path(order).is_oriented());
        }

        #[test]
        fn path_is_pendant(order in 2..25_usize) {
            let digraph = Digraph::path(order);
            let last = order - 1;

            assert!(digraph.is_pendant(0));

            for u in 1..last {
                assert!(!digraph.is_pendant(u));
            }

            assert!(digraph.is_pendant(last));
        }

        #[test]
        fn path_is_regular(order in 2..25_usize) {
            assert!(!Digraph::path(order).is_regular());
        }

        #[test]
        fn path_is_semicomplete(order in 3..25_usize) {
            assert!(!Digraph::path(order).is_semicomplete());
        }

        #[test]
        fn path_is_simple(order in 1..25_usize) {
            assert!(Digraph::path(order).is_simple());
        }

        #[test]
        fn path_is_sink(order in 2..25_usize) {
            let digraph = Digraph::path(order);
            let last = order - 1;

            for u in 0..last {
                assert!(!digraph.is_sink(u));
            }

            assert!(digraph.is_sink(last));
        }

        #[test]
        fn path_is_source(order in 1..25_usize) {
            let digraph = Digraph::path(order);

            assert!(digraph.is_source(0));

            for u in 1..order {
                assert!(!digraph.is_source(u));
            }
        }

        #[test]
        fn path_is_spanning_subdigraph(order in 1..25_usize) {
            let digraph = Digraph::path(order);

            assert!(digraph.is_spanning_subdigraph(&digraph));
        }

        #[test]
        fn path_is_subdigraph(order in 1..25_usize) {
            let digraph = Digraph::path(order);

            assert!(digraph.is_subdigraph(&digraph));
        }

        #[test]
        fn path_is_superdigraph(order in 1..25_usize) {
            let digraph = Digraph::path(order);

            assert!(digraph.is_superdigraph(&digraph));
        }

        #[test]
        fn path_is_symmetric(order in 3..25_usize) {
            assert!(!Digraph::path(order).is_symmetric());
        }

        #[test]
        fn path_is_tournament(order in 3..25_usize) {
            assert!(!Digraph::path(order).is_tournament());
        }

        #[test]
        fn path_semidegree_sequence(order in 2..25_usize) {
            let digraph = Digraph::path(order);
            let semidegree_sequence = &mut digraph.semidegree_sequence();

            assert_eq!(semidegree_sequence.next(), Some((0, 1)));
            assert!(semidegree_sequence.take(order - 2).all(|d| d == (1, 1)));
            assert_eq!(semidegree_sequence.next(), Some((1, 0)));
        }

        #[test]
        fn random_tournament_degree(order in 1..25_usize) {
            let digraph = Digraph::random_tournament(order);

            for u in digraph.vertices() {
                assert_eq!(digraph.degree(u), order - 1);
            }
        }

        #[test]
        fn random_tournament_degree_sum_equals_2size(order in 1..25_usize) {
            let digraph = Digraph::random_tournament(order);

            assert_eq!(
                digraph.vertices().fold(0, |acc, u| acc + digraph.degree(u)),
                2 * digraph.size()
            );
        }

        #[test]
        fn random_tournament_even_number_odd_degrees(order in 1..25_usize) {
            let digraph = Digraph::random_tournament(order);

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
        fn random_tournament_has_edge(order in 1..25_usize) {
            let digraph = Digraph::random_tournament(order);

            for u in digraph.vertices() {
                for v in digraph.vertices() {
                    assert!(!digraph.has_edge(u, v));
                }
            }
        }

        #[test]
        fn random_tournament_indegree(order in 1..25_usize) {
            let digraph = Digraph::random_tournament(order);

            for u in digraph.vertices() {
                assert!((0..order).contains(&digraph.indegree(u)));
            }
        }

        #[test]
        fn random_tournament_indegree_sequence(order in 1..25_usize) {
            let digraph = Digraph::random_tournament(order);
            let indegree_sequence = &mut digraph.indegree_sequence();

            assert!(indegree_sequence.all(|d| (0..order).contains(&d)));
        }

        #[test]
        fn random_tournament_is_complete(order in 2..25_usize) {
            assert!(!Digraph::random_tournament(order).is_complete());
        }

        #[test]
        fn random_tournament_is_isolated(order in 2..25_usize) {
            let digraph = Digraph::random_tournament(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_isolated(u));
            }
        }

        #[test]
        fn random_tournament_is_oriented(order in 1..25_usize) {
            assert!(Digraph::random_tournament(order).is_oriented());
        }

        #[test]
        fn random_tournament_is_pendant(order in 3..25_usize) {
            let digraph = Digraph::random_tournament(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_pendant(u));
            }
        }

        #[test]
        fn random_tournament_is_semicomplete(order in 1..25_usize) {
            assert!(Digraph::random_tournament(order).is_semicomplete());
        }

        #[test]
        fn random_tournament_is_simple(order in 1..25_usize) {
            assert!(Digraph::random_tournament(order).is_simple());
        }

        #[test]
        fn random_tournament_is_spanning_subdigraph(order in 1..25_usize) {
            let digraph = Digraph::random_tournament(order);

            assert!(digraph.is_spanning_subdigraph(&digraph));
        }

        #[test]
        fn random_tournament_is_subdigraph(order in 1..25_usize) {
            let digraph = Digraph::random_tournament(order);

            assert!(digraph.is_subdigraph(&digraph));
        }

        #[test]
        fn random_tournament_is_superdigraph(order in 1..25_usize) {
            let digraph = Digraph::random_tournament(order);

            assert!(digraph.is_superdigraph(&digraph));
        }

        #[test]
        fn random_tournament_is_symmetric(order in 2..25_usize) {
            assert!(!Digraph::random_tournament(order).is_symmetric());
        }

        #[test]
        fn random_tournament_is_tournament(order in 1..25_usize) {
            assert!(Digraph::random_tournament(order).is_tournament());
        }

        #[test]
        fn random_tournament_order(order in 1..25_usize) {
            assert_eq!(Digraph::random_tournament(order).order(), order);
        }

        #[test]
        fn random_tournament_outdegree(order in 1..25_usize) {
            let digraph = Digraph::random_tournament(order);

            for s in digraph.vertices() {
                assert!((0..order).contains(&digraph.outdegree(s)));
            }
        }

        #[test]
        fn random_tournament_semidegree_sequence(order in 1..25_usize) {
            assert_eq!(
                Digraph::random_tournament(order)
                    .semidegree_sequence()
                    .fold(
                        0,
                        |acc, (indegree, outdegree)| acc + indegree + outdegree
                    ),
                order * (order - 1)
            );
        }

        #[test]
        fn random_tournament_size(order in 1..25_usize) {
            let digraph = Digraph::random_tournament(order);

            assert_eq!(digraph.size(), order * (order - 1) / 2);
        }

        #[test]
        fn star_complement_size(order in 1..25_usize) {
            assert_eq!(
                Digraph::star(order).complement().size(),
                (order - 1) * order.saturating_sub(2)
            );
        }

        #[test]
        fn star_degree(order in 1..25_usize) {
            let digraph = Digraph::star(order);

            assert_eq!(digraph.degree(0), (order - 1) * 2);

            for u in 1..order {
                assert_eq!(digraph.degree(u), 2);
            }
        }

        #[test]
        fn star_degree_sum_equals_2size(order in 1..25_usize) {
            let digraph = Digraph::star(order);

            assert_eq!(
                digraph.vertices().fold(0, |acc, u| acc + digraph.degree(u)),
                2 * digraph.size()
            );
        }

        #[test]
        fn star_even_number_odd_degrees(order in 1..25_usize) {
            let digraph = Digraph::star(order);

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
            let digraph = Digraph::star(order);

            for u in 1..order {
                assert!(digraph.has_edge(0, u));
            }

            for u in 1..order {
                for v in u..order {
                    assert!(!digraph.has_edge(u, v));
                }
            }
        }

        #[test]
        fn star_indegree(order in 1..25_usize) {
            let digraph = Digraph::star(order);

            assert_eq!(digraph.indegree(0), order - 1);

            for u in 1..order {
                assert_eq!(digraph.indegree(u), 1);
            }
        }

        #[test]
        fn star_indegree_sequence(order in 1..25_usize) {
            let digraph = Digraph::star(order);
            let indegree_sequence = &mut digraph.indegree_sequence();

            assert_eq!(indegree_sequence.next(), Some(order - 1));
            assert!(indegree_sequence.all(|d| d == 1));
        }

        #[test]
        fn star_is_balanced(order in 3..25_usize) {
            assert!(Digraph::star(order).is_balanced());
        }

        #[test]
        fn star_is_complete(order in 3..25_usize) {
            assert!(!Digraph::star(order).is_complete());
        }

        #[test]
        fn star_is_isolated(order in 2..25_usize) {
            let digraph = Digraph::star(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_isolated(u));
            }
        }

        #[test]
        fn star_is_oriented(order in 2..25_usize) {
            assert!(!Digraph::star(order).is_oriented());
        }

        #[test]
        fn star_is_pendant(order in 1..25_usize) {
            let digraph = Digraph::star(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_pendant(u));
            }
        }

        #[test]
        fn star_is_regular(order in 3..25_usize) {
            assert!(!Digraph::star(order).is_regular());
        }

        #[test]
        fn star_is_semicomplete(order in 3..25_usize) {
            assert!(!Digraph::star(order).is_semicomplete());
        }

        #[test]
        fn star_is_simple(order in 1..25_usize) {
            assert!(Digraph::star(order).is_simple());
        }

        #[test]
        fn star_is_sink(order in 2..25_usize) {
            let digraph = Digraph::star(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_sink(u));
            }
        }

        #[test]
        fn star_is_source(order in 2..25_usize) {
            let digraph = Digraph::star(order);

            for u in digraph.vertices() {
                assert!(!digraph.is_source(u));
            }
        }

        #[test]
        fn star_is_spanning_subdigraph(order in 1..25_usize) {
            let digraph = Digraph::star(order);

            assert!(digraph.is_spanning_subdigraph(&digraph));
        }

        #[test]
        fn star_is_subdigraph(order in 1..25_usize) {
            let digraph = Digraph::star(order);

            assert!(digraph.is_subdigraph(&digraph));
        }

        #[test]
        fn star_is_superdigraph(order in 1..25_usize) {
            let digraph = Digraph::star(order);

            assert!(digraph.is_superdigraph(&digraph));
        }

        #[test]
        fn star_is_symmetric(order in 1..25_usize) {
            assert!(Digraph::star(order).is_symmetric());
        }

        #[test]
        fn star_is_tournament(order in 2..25_usize) {
            assert!(!Digraph::star(order).is_tournament());
        }

        #[test]
        fn star_semidegree_sequence(order in 1..25_usize) {
            let digraph = Digraph::star(order);
            let semidegree_sequence = &mut digraph.semidegree_sequence();

            assert_eq!(
                semidegree_sequence.next(), Some((order - 1, order - 1))
            );

            assert!(semidegree_sequence.all(|d| d == (1, 1)));
        }
    }

    #[test]
    #[should_panic(expected = "v = 1 is out of bounds")]
    fn add_arc_out_of_bounds_u() {
        Digraph::trivial().add_arc(0, 1);
    }

    #[test]
    #[should_panic(expected = "u = 1 is out of bounds")]
    fn add_arc_out_of_bounds_v() {
        Digraph::trivial().add_arc(1, 0);
    }

    #[test]
    #[should_panic(expected = "u = 0 equals v = 0")]
    fn add_arc_u_equals_v() {
        Digraph::trivial().add_arc(0, 0);
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
    #[should_panic(expected = "m must be greater than zero")]
    fn biclique_0_1() {
        let _ = Digraph::biclique(0, 1);
    }

    #[test]
    #[should_panic(expected = "n must be greater than zero")]
    fn biclique_1_0() {
        let _ = Digraph::biclique(1, 0);
    }

    #[test]
    fn biclique_1_1() {
        assert!(Digraph::biclique(1, 1).arcs().eq([(0, 1), (1, 0)]));
    }

    #[test]
    fn biclique_1_1_complement() {
        assert!(Digraph::biclique(1, 1).complement().arcs().eq([]));
    }

    #[test]
    fn biclique_1_1_is_complete() {
        assert!(Digraph::biclique(1, 1).is_balanced());
    }

    #[test]
    fn biclique_1_1_is_semicomplete() {
        assert!(Digraph::biclique(1, 1).is_semicomplete());
    }

    #[test]
    fn biclique_1_2() {
        assert!(Digraph::biclique(1, 2)
            .arcs()
            .eq([(0, 1), (0, 2), (1, 0), (2, 0)]));
    }

    #[test]
    fn biclique_1_2_complement() {
        assert!(Digraph::biclique(1, 2)
            .complement()
            .arcs()
            .eq([(1, 2), (2, 1)]));
    }

    #[test]
    fn biclique_1_2_is_complete() {
        assert!(!Digraph::biclique(1, 2).is_complete());
    }

    #[test]
    fn biclique_1_2_is_semicomplete() {
        assert!(!Digraph::biclique(1, 2).is_semicomplete());
    }

    #[test]
    fn biclique_2_1() {
        assert!(Digraph::biclique(2, 1)
            .arcs()
            .eq([(0, 2), (1, 2), (2, 0), (2, 1)]));
    }

    #[test]
    fn biclique_2_1_complement() {
        assert!(Digraph::biclique(2, 1)
            .complement()
            .arcs()
            .eq([(0, 1), (1, 0)]));
    }

    #[test]
    fn biclique_2_1_is_complete() {
        assert!(!Digraph::biclique(2, 1).is_complete());
    }

    #[test]
    fn biclique_2_1_is_semicomplete() {
        assert!(!Digraph::biclique(2, 1).is_semicomplete());
    }

    #[test]
    fn biclique_2_2() {
        assert!(Digraph::biclique(2, 2).arcs().eq([
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
        assert!(Digraph::biclique(2, 2)
            .complement()
            .arcs()
            .eq([(0, 1), (1, 0), (2, 3), (3, 2),]));
    }

    #[test]
    fn biclique_claw() {
        assert!(Digraph::claw()
            .arcs()
            .eq([(0, 1), (0, 2), (0, 3), (1, 0), (2, 0), (3, 0)]));
    }

    #[test]
    fn biclique_utility() {
        assert!(Digraph::utility().arcs().eq([
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
    fn circuit_1() {
        assert!(Digraph::circuit(1).arcs().eq([]));
    }

    #[test]
    fn circuit_1_complement() {
        assert!(Digraph::circuit(1).complement().arcs().eq([]));
    }

    #[test]
    fn circuit_1_degree() {
        assert_eq!(Digraph::circuit(1).degree(0), 0);
    }

    #[test]
    fn circuit_1_indegree() {
        assert_eq!(Digraph::circuit(1).indegree(0), 0);
    }

    #[test]
    fn circuit_1_indegree_sequence() {
        assert!(Digraph::circuit(1).indegree_sequence().all(|d| d == 0));
    }

    #[test]
    fn circuit_1_is_complete() {
        assert!(Digraph::circuit(1).is_complete());
    }

    #[test]
    fn circuit_1_is_isolated() {
        assert!(Digraph::circuit(1).is_isolated(0));
    }

    #[test]
    fn circuit_1_is_oriented() {
        assert!(Digraph::circuit(1).is_oriented());
    }

    #[test]
    fn circuit_1_is_semicomplete() {
        assert!(Digraph::circuit(1).is_semicomplete());
    }

    #[test]
    fn circuit_1_is_simple() {
        assert!(Digraph::circuit(1).is_simple());
    }

    #[test]
    fn circuit_1_is_sink() {
        assert!(Digraph::circuit(1).is_sink(0));
    }

    #[test]
    fn circuit_1_is_source() {
        assert!(Digraph::circuit(1).is_source(0));
    }

    #[test]
    fn circuit_1_is_symmetric() {
        assert!(Digraph::circuit(1).is_symmetric());
    }

    #[test]
    fn circuit_1_is_tournament() {
        assert!(Digraph::circuit(1).is_tournament());
    }

    #[test]
    fn circuit_1_semidegree_sequence() {
        assert!(Digraph::circuit(1)
            .semidegree_sequence()
            .all(|d| d == (0, 0)));
    }

    #[test]
    fn circuit_2() {
        assert!(Digraph::circuit(2).arcs().eq([(0, 1), (1, 0)]));
    }

    #[test]
    fn circuit_2_complement() {
        assert!(Digraph::circuit(2).complement().arcs().eq([]));
    }

    #[test]
    fn circuit_2_has_edge() {
        let digraph = Digraph::circuit(2);

        for u in 0..2 {
            for v in u + 1..2 {
                assert!(digraph.has_edge(u, v));
            }
        }
    }

    #[test]
    fn circuit_2_is_complete() {
        assert!(Digraph::circuit(2).is_complete());
    }

    #[test]
    fn circuit_2_is_oriented() {
        assert!(!Digraph::circuit(2).is_oriented());
    }

    #[test]
    fn circuit_2_is_semicomplete() {
        assert!(Digraph::circuit(2).is_semicomplete());
    }

    #[test]
    fn circuit_2_is_symmetric() {
        assert!(Digraph::circuit(2).is_symmetric());
    }

    #[test]
    fn circuit_2_is_tournament() {
        assert!(!Digraph::circuit(2).is_tournament());
    }

    #[test]
    fn circuit_3() {
        assert!(Digraph::circuit(3).arcs().eq([(0, 1), (1, 2), (2, 0)]));
    }

    #[test]
    fn circuit_3_complement() {
        assert!(Digraph::circuit(3)
            .complement()
            .arcs()
            .eq([(0, 2), (1, 0), (2, 1)]));
    }

    #[test]
    fn circuit_3_is_semicomplete() {
        assert!(Digraph::circuit(3).is_semicomplete());
    }

    #[test]
    fn circuit_3_is_tournament() {
        assert!(Digraph::circuit(3).is_tournament());
    }

    #[test]
    fn complete_1() {
        assert!(Digraph::complete(1).arcs().eq([]));
    }

    #[test]
    fn complete_1_complement() {
        assert!(Digraph::complete(1).complement().arcs().eq([]));
    }

    #[test]
    fn complete_1_has_edge() {
        assert!(!Digraph::complete(1).has_edge(0, 0));
    }

    #[test]
    fn complete_1_is_isolated() {
        assert!(Digraph::complete(1).is_isolated(0));
    }

    #[test]
    fn complete_1_is_oriented() {
        assert!(Digraph::complete(1).is_oriented());
    }

    #[test]
    fn complete_1_is_simple() {
        assert!(Digraph::complete(1).is_simple());
    }

    #[test]
    fn complete_1_is_tournament() {
        assert!(Digraph::complete(1).is_tournament());
    }

    #[test]
    fn complete_2() {
        assert!(Digraph::complete(2).arcs().eq([(0, 1), (1, 0)]));
    }

    #[test]
    fn complete_2_complement() {
        assert!(Digraph::complete(2).complement().arcs().eq([]));
    }

    #[test]
    fn complete_3() {
        assert!(Digraph::complete(3)
            .arcs()
            .eq([(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)]));
    }

    #[test]
    fn complete_3_complement() {
        assert!(Digraph::complete(3).complement().arcs().eq([]));
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
    #[should_panic(expected = "a digraph must have at least one vertex")]
    fn converse_0() {
        let _ = Digraph {
            order: 0,
            blocks: vec![],
        }
        .converse();
    }

    #[test]
    fn cycle_1() {
        assert!(Digraph::cycle(1).arcs().eq([]));
    }

    #[test]
    fn cycle_1_complement() {
        assert!(Digraph::cycle(1).complement().arcs().eq([]));
    }

    #[test]
    fn cycle_1_degree() {
        let digraph = Digraph::cycle(1);

        assert_eq!(digraph.degree(0), 0);
    }

    #[test]
    fn cycle_1_indegree() {
        let digraph = Digraph::cycle(1);

        assert_eq!(digraph.indegree(0), 0);
    }

    #[test]
    fn cycle_1_indegree_sequence() {
        assert!(Digraph::cycle(1).indegree_sequence().all(|d| d == 0));
    }

    #[test]
    fn cycle_1_is_complete() {
        assert!(Digraph::cycle(1).is_complete());
    }

    #[test]
    fn cycle_1_is_isolated() {
        assert!(Digraph::cycle(1).is_isolated(0));
    }

    #[test]
    fn cycle_1_is_oriented() {
        assert!(Digraph::cycle(1).is_oriented());
    }

    #[test]
    fn cycle_1_is_semicomplete() {
        assert!(Digraph::cycle(1).is_semicomplete());
    }

    #[test]
    fn cycle_1_is_sink() {
        assert!(Digraph::cycle(1).is_sink(0));
    }

    #[test]
    fn cycle_1_is_source() {
        assert!(Digraph::cycle(1).is_source(0));
    }

    #[test]
    fn cycle_1_is_tournament() {
        assert!(Digraph::cycle(1).is_tournament());
    }

    #[test]
    fn cycle_2() {
        assert!(Digraph::cycle(2).arcs().eq([(0, 1), (1, 0)]));
    }

    #[test]
    fn cycle_2_complement() {
        assert!(Digraph::cycle(2).complement().arcs().eq([]));
    }

    #[test]
    fn cycle_2_degree() {
        let digraph = Digraph::cycle(2);

        for u in digraph.vertices() {
            assert_eq!(digraph.degree(u), 2);
        }
    }

    #[test]
    fn cycle_2_indegree() {
        let digraph = Digraph::cycle(2);

        for u in digraph.vertices() {
            assert_eq!(digraph.indegree(u), 1);
        }
    }

    #[test]
    fn cycle_2_indegree_sequence() {
        assert!(Digraph::cycle(2).indegree_sequence().all(|d| d == 1));
    }

    #[test]
    fn cycle_2_is_complete() {
        assert!(Digraph::cycle(2).is_complete());
    }

    #[test]
    fn cycle_2_is_semicomplete() {
        assert!(Digraph::cycle(2).is_semicomplete());
    }

    #[test]
    fn cycle_3() {
        assert!(Digraph::cycle(3)
            .arcs()
            .eq([(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)]));
    }

    #[test]
    fn cycle_3_complement() {
        assert!(Digraph::cycle(3).complement().arcs().eq([]));
    }

    #[test]
    fn cycle_3_is_complete() {
        assert!(Digraph::cycle(3).is_complete());
    }

    #[test]
    fn cycle_3_is_semicomplete() {
        assert!(Digraph::cycle(3).is_semicomplete());
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
    #[should_panic(expected = "a digraph must have at least one vertex")]
    fn empty_0() {
        let _ = Digraph::empty(0);
    }

    #[test]
    fn empty_1() {
        assert!(Digraph::empty(1).arcs().eq([]));
    }

    #[test]
    fn empty_1_complement() {
        assert!(Digraph::empty(1).complement().arcs().eq([]));
    }

    #[test]
    fn empty_2() {
        assert!(Digraph::empty(2).arcs().eq([]));
    }

    #[test]
    fn empty_2_complement() {
        assert!(Digraph::empty(2).complement().arcs().eq([(0, 1), (1, 0)]));
    }

    #[test]
    fn empty_3() {
        assert!(Digraph::empty(3).arcs().eq([]));
    }

    #[test]
    fn empty_3_complement() {
        assert!(Digraph::empty(3).complement().arcs().eq([
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
        assert!(Digraph::trivial().arcs().eq([]));
    }

    #[test]
    fn empty_trivial_is_complete() {
        assert!(Digraph::trivial().is_complete());
    }

    #[test]
    fn empty_trivial_is_semicomplete() {
        assert!(Digraph::trivial().is_semicomplete());
    }

    #[test]
    fn empty_trivial_is_tournament() {
        assert!(Digraph::trivial().is_tournament());
    }

    #[test]
    fn from_adjacency_list() {
        let digraph = Digraph::from(adjacency_list::Digraph::star(5));

        assert!(digraph.arcs().eq([
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0)
        ]));
    }

    #[test]
    fn from_vec() {
        let arcs = vec![BTreeSet::from([1]), BTreeSet::from([2]), BTreeSet::new()];
        let digraph = Digraph::from(arcs);

        assert_eq!(digraph.order(), 3);
        assert!(digraph.arcs().eq([(0, 1), (1, 2)]));
    }

    #[test]
    #[should_panic(expected = "v = 1 is out of bounds")]
    fn from_vec_out_of_bounds_v() {
        let vec = vec![BTreeSet::from([1])];
        let _ = Digraph::from(vec);
    }

    #[test]
    #[should_panic(expected = "u = 0 equals v = 0")]
    fn from_vec_u_equals_v() {
        let vec = vec![BTreeSet::from([0])];
        let _ = Digraph::from(vec);
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
    #[should_panic(expected = "v = 1 is out of bounds")]
    fn indegree_out_of_bounds() {
        let _ = Digraph::trivial().indegree(1);
    }

    #[test]
    fn indegree_sequence_bang_jensen_34() {
        assert!(bang_jensen_34().indegree_sequence().eq([1, 1, 0, 1, 2, 1]));
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
    fn indegree_sequence_kattis_escapewallmaria_1() {
        assert!(kattis_escapewallmaria_1()
            .indegree_sequence()
            .eq([0, 0, 0, 0, 0, 2, 1, 0, 0, 2, 0, 0, 1, 1]));
    }

    #[test]
    fn indegree_sequence_kattis_escapewallmaria_2() {
        assert!(kattis_escapewallmaria_2()
            .indegree_sequence()
            .eq([0, 0, 0, 0, 0, 2, 1, 0, 0, 2, 0, 0, 1, 1]));
    }

    #[test]
    fn indegree_sequence_kattis_escapewallmaria_3() {
        assert!(kattis_escapewallmaria_3()
            .indegree_sequence()
            .eq([0, 2, 2, 0, 0, 3, 2, 0, 0, 2, 0, 0, 1, 2]));
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
    #[should_panic(expected = "u = 1 is out of bounds")]
    fn out_neighbors_out_of_bounds() {
        let _ = Digraph::trivial().out_neighbors(1);
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
    #[should_panic(expected = "u = 1 is out of bounds")]
    fn out_neighbors_weighted_out_of_bounds() {
        let _ = Digraph::trivial().out_neighbors_weighted(1);
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
    #[should_panic(expected = "u = 1 is out of bounds")]
    fn outdegree_out_of_bounds() {
        let _ = Digraph::trivial().outdegree(1);
    }

    #[test]
    fn path_1() {
        assert!(Digraph::path(1).arcs().eq([]));
    }

    #[test]
    fn path_1_complement() {
        assert!(Digraph::path(1).complement().arcs().eq([]));
    }

    #[test]
    fn path_1_is_balanced() {
        assert!(Digraph::path(1).is_balanced());
    }

    #[test]
    fn path_1_is_complete() {
        assert!(Digraph::path(1).is_complete());
    }

    #[test]
    fn path_1_is_isolated() {
        assert!(Digraph::path(1).is_isolated(0));
    }

    #[test]
    fn path_1_is_pendant() {
        assert!(!Digraph::path(1).is_pendant(0));
    }

    #[test]
    fn path_1_is_regular() {
        assert!(Digraph::path(1).is_regular());
    }

    #[test]
    fn path_1_is_semicomplete() {
        assert!(Digraph::path(1).is_semicomplete());
    }

    #[test]
    fn path_1_is_sink() {
        assert!(Digraph::path(1).is_sink(0));
    }

    #[test]
    fn path_1_is_symmetric() {
        assert!(Digraph::path(1).is_symmetric());
    }

    #[test]
    fn path_1_is_tournament() {
        assert!(Digraph::path(1).is_tournament());
    }

    #[test]
    fn path_2() {
        assert!(Digraph::path(2).arcs().eq([(0, 1)]));
    }

    #[test]
    fn path_2_complement() {
        assert!(Digraph::path(2).complement().arcs().eq([(1, 0)]));
    }

    #[test]
    fn path_2_is_semicomplete() {
        assert!(Digraph::path(2).is_semicomplete());
    }

    #[test]
    fn path_2_is_symmetric() {
        assert!(!Digraph::path(2).is_symmetric());
    }

    #[test]
    fn path_2_is_tournament() {
        assert!(Digraph::path(2).is_tournament());
    }

    #[test]
    fn path_3() {
        assert!(Digraph::path(3).arcs().eq([(0, 1), (1, 2)]));
    }

    #[test]
    fn path_3_complement() {
        assert!(Digraph::path(3)
            .complement()
            .arcs()
            .eq([(0, 2), (1, 0), (2, 0), (2, 1)]));
    }

    #[test]
    fn random_tournament_1_is_complete() {
        assert!(Digraph::random_tournament(1).is_complete());
    }

    #[test]
    fn random_tournament_1_is_isolated() {
        assert!(Digraph::random_tournament(1).is_isolated(0));
    }

    #[test]
    fn random_tournament_1_is_pendant() {
        assert!(!Digraph::random_tournament(1).is_pendant(0));
    }

    #[test]
    fn random_tournament_1_is_simple() {
        assert!(Digraph::random_tournament(1).is_simple());
    }

    #[test]
    fn random_tournament_2_is_pendant() {
        let digraph = Digraph::random_tournament(2);

        for s in digraph.vertices() {
            assert!(digraph.is_pendant(s));
        }
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
    fn remove_arc_out_of_bounds() {
        assert!(!Digraph::trivial().remove_arc(0, 1));
        assert!(!Digraph::trivial().remove_arc(1, 0));
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
            (1, 2)
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
            (1, 2)
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
            (2, 2)
        ]));
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
    fn star_1() {
        assert!(Digraph::star(1).arcs().eq([]));
    }

    #[test]
    fn star_1_complement() {
        assert!(Digraph::star(1).complement().arcs().eq([]));
    }

    #[test]
    fn star_1_is_balanced() {
        assert!(Digraph::star(1).is_balanced());
    }

    #[test]
    fn star_1_is_complete() {
        assert!(Digraph::star(1).is_complete());
    }

    #[test]
    fn star_1_is_isolated() {
        assert!(Digraph::star(1).is_isolated(0));
    }

    #[test]
    fn star_1_is_oriented() {
        assert!(Digraph::star(1).is_oriented());
    }

    #[test]
    fn star_1_is_regular() {
        assert!(Digraph::star(1).is_regular());
    }

    #[test]
    fn star_1_is_semicomplete() {
        assert!(Digraph::star(1).is_semicomplete());
    }

    #[test]
    fn star_1_is_sink() {
        assert!(Digraph::star(1).is_sink(0));
    }

    #[test]
    fn star_1_is_source() {
        assert!(Digraph::star(1).is_source(0));
    }

    #[test]
    fn star_1_is_tournament() {
        assert!(Digraph::star(1).is_tournament());
    }

    #[test]
    fn star_2() {
        assert!(Digraph::star(2).arcs().eq([(0, 1), (1, 0)]));
    }

    #[test]
    fn star_2_complement() {
        assert!(Digraph::star(2).complement().arcs().eq([]));
    }

    #[test]
    fn star_2_is_balanced() {
        assert!(Digraph::star(2).is_complete());
    }

    #[test]
    fn star_2_is_complete() {
        assert!(Digraph::star(2).is_complete());
    }

    #[test]
    fn star_2_is_regular() {
        assert!(Digraph::star(2).is_regular());
    }

    #[test]
    fn star_2_is_semicomplete() {
        assert!(Digraph::star(2).is_semicomplete());
    }

    #[test]
    fn star_3() {
        assert!(Digraph::star(3).arcs().eq([(0, 1), (0, 2), (1, 0), (2, 0)]));
    }

    #[test]
    fn star_3_complement() {
        assert!(Digraph::star(3).complement().arcs().eq([(1, 2), (2, 1)]));
    }

    #[test]
    fn toggle() {
        let mut digraph = Digraph::empty(3);

        digraph.toggle(0, 1);
        digraph.toggle(0, 2);

        assert_eq!(digraph.blocks, [0b110]);
    }

    #[test]
    #[should_panic(expected = "u = 1 is out of bounds.")]
    fn toggle_out_of_bounds_u() {
        let mut digraph = Digraph::trivial();

        digraph.toggle(1, 0);
    }

    #[test]
    #[should_panic(expected = "v = 1 is out of bounds.")]
    fn toggle_out_of_bounds_v() {
        let mut digraph = Digraph::trivial();

        digraph.toggle(0, 1);
    }
}

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
//! // 0 -> {1, 2}
//! // 1 -> {0}
//! // 2 -> {}
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
        r#gen::{
            Biclique,
            Circuit,
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
    /// // 0 -> {}
    /// // 1 -> {}
    /// // 2 -> {}
    ///
    /// let mut digraph = Digraph::empty(3);
    ///
    /// assert!(!digraph.has_arc(0, 1));
    ///
    /// // 0 -> {1}
    /// // 1 -> {}
    /// // 2 -> {}
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
    /// * Panics if `u` equals `v`.
    /// * Panics if `u` is out of bounds.
    /// * Panics if `v` is out of bounds.
    fn add_arc(&mut self, u: usize, v: usize) {
        assert_ne!(u, v, "u = {u} equals v = {v}.");
        assert!(u < self.order, "u = {u} is out of bounds.");
        assert!(v < self.order, "v = {v} is out of bounds.");

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
    fn arcs_weighted<'a>(
        &'a self,
    ) -> impl Iterator<Item = (usize, usize, &'a usize)>
    where
        usize: 'a,
    {
        self.arcs().map(|(u, v)| (u, v, &1))
    }
}

impl Biclique for Digraph {
    /// # Panics
    ///
    /// * Panics if `m` is zero.
    /// * Panics if `n` is zero.
    fn biclique(m: usize, n: usize) -> Self {
        assert!(m > 0, "m = {m} must be greater than zero");
        assert!(n > 0, "n = {n} must be greater than zero");

        let order = m + n;
        let mut digraph = Self::empty(order);

        for u in 0..m {
            for v in m..order {
                digraph.add_arc(u, v);
                digraph.add_arc(v, u);
            }
        }

        digraph
    }
}

impl Circuit for Digraph {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn circuit(order: usize) -> Self {
        let mut digraph = Self::empty(order);

        if order == 1 {
            return digraph;
        }

        for u in 0..order - 1 {
            digraph.add_arc(u, u + 1);
        }

        digraph.add_arc(order - 1, 0);

        digraph
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
    fn out_neighbors_weighted<'a>(
        &'a self,
        u: usize,
    ) -> impl Iterator<Item = (usize, &'a usize)>
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
                ErdosRenyi,
                RandomTournament,
                Star,
            },
            op::{
                Complement,
                Degree,
                DegreeSequence,
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
                Order,
                OutdegreeSequence,
                SemidegreeSequence,
                Sinks,
                Sources,
            },
            proptest_strategy::arc,
            r#gen::Path,
        },
        adjacency_list::fixture::{
            bang_jensen_196,
            kattis_cantinaofbabel_1,
            kattis_cantinaofbabel_2,
        },
        proptest::proptest,
    };

    proptest! {
        #[test]
        fn add_arc_arc_weight((u, v) in arc(25_usize)) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc(u, v);

            assert!(digraph.vertices().all(|x| digraph.vertices().all(|y| {
                digraph.arc_weight(x, y) == (x == u && y == v).then_some(&1)
            })));
        }

        #[test]
        fn add_arc_degree((u, v) in arc(25_usize)) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc(u, v);

            assert!(digraph.vertices().all(|x| {
                digraph.degree(x) == usize::from(x == u) + usize::from(x == v)
            }));
        }

        #[test]
        fn add_arc_has_arc((u, v) in arc(25_usize)) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc(u, v);

            assert!(digraph.has_arc(u, v));
        }

        #[test]
        fn add_arc_indegree((u, v) in arc(25_usize)) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc(u, v);

            assert!(digraph
                .vertices()
                .all(|x| {
                    digraph.indegree(x) == usize::from(x == v) }));
        }

        #[test]
        fn add_arc_outdegree((u, v) in arc(25_usize)) {
            let mut digraph = Digraph::empty(100);

            digraph.add_arc(u, v);

            assert!(digraph
                .vertices()
                .all(|x| { digraph.outdegree(x) == usize::from(x == u) }));
        }

        #[test]
        fn add_arc_remove_arc((u, v) in arc(25_usize)) {
            let digraph = Digraph::empty(100);
            let mut h = digraph.clone();

            h.add_arc(u, v);

            assert!(digraph.vertices().all(|x| digraph
                .vertices()
                .all(|y| { h.remove_arc(x, y) == (x == u && y == v) })));

            assert_eq!(digraph, h);
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
            let clique_1_degree = n * 2;
            let clique_2_degree = m * 2;

            assert!((0..m).all(|u| digraph.degree(u) == clique_1_degree));
            assert!((m..m + n).all(|u| digraph.degree(u) == clique_2_degree));
        }

        #[test]
        fn biclique_degree_sequence(m in 1..25_usize, n in 1..25_usize) {
            let digraph = Digraph::biclique(m, n);
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
            let digraph = Digraph::biclique(m, n);

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
            let digraph = Digraph::biclique(m, n);
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
            let digraph = Digraph::biclique(m, n);
            let order = m + n;

            assert!(
                (0..m).all(|u| digraph.in_neighbors(u).eq(m..order))
                    && (m..order).all(|u| digraph.in_neighbors(u).eq(0..m))
            );
        }

        #[test]
        fn biclique_indegree(m in 1..25_usize, n in 1..25_usize) {
            let digraph = Digraph::biclique(m, n);

            assert!(
                (0..m).all(|u| digraph.indegree(u) == n)
                    && (m..m + n).all(|u| digraph.indegree(u) == m)
            );
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
        fn biclique_is_complete(m in 1..25_usize, n in 1..25_usize) {
            assert!(
                ((m, n) == (1, 1)) == Digraph::biclique(m, n).is_complete()
            );
        }

        #[test]
        fn biclique_is_isolated(m in 1..25_usize, n in 1..25_usize) {
            let digraph = Digraph::biclique(m, n);

            assert!(digraph.vertices().all(|u| { !digraph.is_isolated(u) }));
        }

        #[test]
        fn biclique_is_oriented(m in 1..25_usize, n in 1..25_usize) {
            assert!(!Digraph::biclique(m, n).is_oriented());
        }

        #[test]
        fn biclique_is_pendant(m in 1..25_usize, n in 1..25_usize) {
            let digraph = Digraph::biclique(m, n);

            assert!(digraph.vertices().all(|u| { !digraph.is_pendant(u) }));
        }

        #[test]
        fn biclique_is_regular(m in 1..25_usize, n in 1..25_usize) {
            assert!(Digraph::biclique(m, n).is_regular() == (m == n));
        }

        #[test]
        fn biclique_is_semicomplete(m in 1..25_usize, n in 1..25_usize) {
            assert!(
                ((m, n) == (1, 1))
                    == Digraph::biclique(m, n).is_semicomplete()
            );
        }

        #[test]
        fn biclique_is_simple(m in 1..25_usize, n in 1..25_usize) {
            assert!(Digraph::biclique(m, n).is_simple());
        }

        #[test]
        fn biclique_is_spanning_subdigraph(
            m in 1..25_usize,
            n in 1..25_usize
        ) {
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
            let digraph = Digraph::biclique(m, n);
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
            let digraph = Digraph::biclique(m, n);

            assert!(
                (0..m).all(|u| digraph.outdegree(u) == n)
                    && (m..m + n).all(|u| digraph.outdegree(u) == m)
            );
        }

        #[test]
        fn biclique_outdegree_sequence(m in 1..25_usize, n in 1..25_usize) {
            let digraph = Digraph::biclique(m, n);
            let outdegree_sequence = &mut digraph.outdegree_sequence();

            assert!(outdegree_sequence.take(m).all(|d| d == n));
            assert!(outdegree_sequence.all(|d| d == m));
        }

        #[test]
        fn biclique_semidegree_sequence(m in 1..25_usize, n in 1..25_usize) {
            let digraph = Digraph::biclique(m, n);
            let semidegree_sequence = &mut digraph.semidegree_sequence();

            assert!(semidegree_sequence.take(m).all(|d| d == (n, n)));
            assert!(semidegree_sequence.all(|d| d == (m, m)));
        }

        #[test]
        fn biclique_sinks(m in 1..25_usize, n in 1..25_usize) {
            assert!(Digraph::biclique(m, n).sinks().eq([]));
        }

        #[test]
        fn biclique_size(m in 1..25_usize, n in 1..25_usize) {
            assert_eq!(Digraph::biclique(m, n).size(), m * n * 2);
        }

        #[test]
        fn biclique_sources(m in 1..25_usize, n in 1..25_usize) {
            assert!(Digraph::biclique(m, n).sources().eq([]));
        }

        #[test]
        fn circuit_complement_size(order in 1..25_usize) {
            assert_eq!(
                Digraph::circuit(order).complement().size(),
                order * order.saturating_sub(2)
            );
        }

        #[test]
        fn circuit_degree(order in 1..25_usize) {
            let digraph = Digraph::circuit(order);

            assert!(digraph
                .vertices()
                .all(|u| digraph.degree(u) == if order == 1 { 0 } else { 2 }));
        }

        #[test]
        fn circuit_degree_sequence(order in 1..25_usize) {
            assert!(Digraph::circuit(order)
                .degree_sequence()
                .all(|d| d == if order == 1 { 0 } else { 2 }));
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
        fn circuit_even_number_odd_degrees(order in 1..25_usize) {
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
        fn circuit_has_edge(order in 1..25_usize) {
            let digraph = Digraph::circuit(order);

            assert!(digraph.vertices().all(|u| {
                (u + 1..order).all(|v| (order == 2) == digraph.has_edge(u, v))
            }));
        }

        #[test]
        fn circuit_indegree(order in 1..25_usize) {
            let digraph = Digraph::circuit(order);

            assert!(digraph
                .vertices()
                .all(|u| { digraph.indegree(u) == usize::from(order != 1) }));
        }

        #[test]
        fn circuit_indegree_sequence(order in 1..25_usize) {
            assert!(Digraph::circuit(order)
                .indegree_sequence()
                .all(|d| d == usize::from(order != 1)));
        }

        #[test]
        fn circuit_is_balanced(order in 1..25_usize) {
            assert!(Digraph::circuit(order).is_balanced());
        }

        #[test]
        fn circuit_is_complete(order in 1..25_usize) {
            assert!((order < 3) == Digraph::circuit(order).is_complete());
        }

        #[test]
        fn circuit_is_isolated(order in 1..25_usize) {
            let digraph = Digraph::circuit(order);

            assert!(digraph
                .vertices()
                .all(|u| { (order == 1) == digraph.is_isolated(u) }));
        }

        #[test]
        fn circuit_is_oriented(order in 1..25_usize) {
            assert!((order == 2) != Digraph::circuit(order).is_oriented());
        }

        #[test]
        fn circuit_is_pendant(order in 1..25_usize) {
            let digraph = Digraph::circuit(order);

            assert!(digraph.vertices().all(|u| { !digraph.is_pendant(u) }));
        }

        #[test]
        fn circuit_is_regular(order in 1..25_usize) {
            assert!(Digraph::circuit(order).is_regular());
        }

        #[test]
        fn circuit_is_semicomplete(order in 1..25_usize) {
            assert!((order < 4) == Digraph::circuit(order).is_semicomplete());
        }

        #[test]
        fn circuit_is_simple(order in 1..25_usize) {
            assert!(Digraph::circuit(order).is_simple());
        }

        #[test]
        fn circuit_is_sink(order in 1..25_usize) {
            let digraph = Digraph::circuit(order);

            assert!(digraph
                .vertices()
                .all(|u| { (order == 1) == digraph.is_sink(u) }));
        }

        #[test]
        fn circuit_is_source(order in 1..25_usize) {
            let digraph = Digraph::circuit(order);

            assert!(digraph
                .vertices()
                .all(|u| { (order == 1) == digraph.is_source(u) }));
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
        fn circuit_is_symmetric(order in 1..25_usize) {
            assert!((order < 3) == Digraph::circuit(order).is_symmetric());
        }

        #[test]
        fn circuit_is_tournament(order in 1..25_usize) {
            assert!(
                (order == 1 || order == 3)
                    == Digraph::circuit(order).is_tournament()
            );
        }

        #[test]
        fn circuit_outdegree(order in 1..25_usize) {
            let digraph = Digraph::circuit(order);

            assert!(digraph
                .vertices()
                .all(|u| { digraph.outdegree(u) == usize::from(order != 1) }));
        }

        #[test]
        fn circuit_outdegree_sequence(order in 1..25_usize) {
            assert!(Digraph::circuit(order)
                .outdegree_sequence()
                .all(|d| d == usize::from(order != 1)));
        }

        #[test]
        fn circuit_semidegree_sequence(order in 1..25_usize) {
            assert!(Digraph::circuit(order)
                .semidegree_sequence()
                .all(|d| d == if order == 1 { (0, 0) } else { (1, 1) }));
        }

        #[test]
        fn circuit_sinks(order in 1..25_usize) {
            let digraph = Digraph::circuit(order);
            let sinks = digraph.sinks();

            assert!(if order == 1 { sinks.eq([0]) } else { sinks.eq([]) });
        }

        #[test]
        fn circuit_size(order in 1..25_usize) {
            assert_eq!(
                Digraph::circuit(order).size(),
                if order == 1 { 0 } else { order }
            );
        }

        #[test]
        fn circuit_sources(order in 1..25_usize) {
            let digraph = Digraph::circuit(order);
            let sources = digraph.sources();

            assert!(if order == 1 { sources.eq([0]) } else { sources.eq([]) });
        }

        #[test]
        fn complete_complement_equals_empty(order in 1..25_usize) {
            assert_eq!(
                Digraph::complete(order).complement(),
                Digraph::empty(order)
            );
        }

        #[test]
        fn complete_complement_size(order in 1..25_usize) {
            assert_eq!(Digraph::complete(order).complement().size(), 0);
        }

        #[test]
        fn complete_degree(order in 1..25_usize) {
            let digraph = Digraph::complete(order);
            let degree = order * 2 - 2;

            assert!(digraph
                .vertices()
                .all(|u| digraph.degree(u) == degree));
        }

        #[test]
        fn complete_degree_sequence(order in 1..25_usize) {
            let degree = order * 2 - 2;

            assert!(Digraph::complete(order)
                .degree_sequence()
                .all(|d| d == degree));
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
        fn complete_has_edge(order in 1..25_usize) {
            let digraph = Digraph::complete(order);

            assert!(digraph
                .vertices()
                .all(|u| { (u + 1..order).all(|v| digraph.has_edge(u, v)) }));
        }

        #[test]
        fn complete_indegree(order in 1..25_usize) {
            let digraph = Digraph::complete(order);
            let indegree = order - 1;

            assert!(digraph
                .vertices()
                .all(|v| digraph.indegree(v) == indegree));
        }

        #[test]
        fn complete_indegree_sequence(order in 1..25_usize) {
            let indegree = order - 1;

            assert!(Digraph::complete(order)
                .indegree_sequence()
                .all(|d| d == indegree));
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
        fn complete_is_isolated(order in 1..25_usize) {
            let digraph = Digraph::complete(order);

            assert!(digraph
                .vertices()
                .all(|u| { (order == 1) == digraph.is_isolated(u) }));
        }

        #[test]
        fn complete_is_oriented(order in 1..25_usize) {
            assert!((order == 1) == Digraph::complete(order).is_oriented());
        }

        #[test]
        fn complete_is_pendant(order in 1..25_usize) {
            let digraph = Digraph::complete(order);

            assert!(digraph.vertices().all(|u| { !digraph.is_pendant(u) }));
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
        fn complete_is_simple(order in 1..25_usize) {
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
        fn complete_is_tournament(order in 1..25_usize) {
            assert!((order == 1) == Digraph::complete(order).is_tournament());
        }

        #[test]
        fn complete_order(order in 1..25_usize) {
            assert_eq!(Digraph::complete(order).order(), order);
        }

        #[test]
        fn complete_outdegree(order in 1..25_usize) {
            let digraph = Digraph::complete(order);
            let outdegree = order - 1;

            assert!(digraph
                .vertices()
                .all(|s| digraph.outdegree(s) == outdegree));
        }

        #[test]
        fn complete_outdegree_sequence(order in 1..25_usize) {
            let outdegree = order - 1;

            assert!(Digraph::complete(order)
                .outdegree_sequence()
                .all(|d| d == outdegree));
        }

        #[test]
        fn complete_semidegree_sequence(order in 1..25_usize) {
            let degree = order - 1;
            let pair = (degree, degree);

            assert!(Digraph::complete(order)
                .semidegree_sequence()
                .all(|d| d == pair));
        }

        #[test]
        fn complete_sinks(order in 1..25_usize) {
            assert!(if order == 1 {
                Digraph::complete(order).sinks().eq([0])
            } else {
                Digraph::complete(order).sinks().eq([])
            });
        }

        #[test]
        fn complete_size(order in 1..25_usize) {
            assert_eq!(Digraph::complete(order).size(), order * (order - 1));
        }

        #[test]
        fn complete_sources(order in 1..25_usize) {
            assert!(if order == 1 {
                Digraph::complete(order).sources().eq([0])
            } else {
                Digraph::complete(order).sources().eq([])
            });
        }

        #[test]
        fn cycle_complement_size(order in 1..25_usize) {
            assert_eq!(
                Digraph::cycle(order).complement().size(),
                order * order.saturating_sub(3)
            );
        }

        #[test]
        fn cycle_degree(order in 1..25_usize) {
            let digraph = Digraph::cycle(order);

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
            assert!(Digraph::cycle(order)
                .degree_sequence()
                .all(|d| match order {
                    1 => d == 0,
                    2 => d == 2,
                    _ => d == 4,
                }));
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
        fn cycle_indegree(order in 1..25_usize) {
            let digraph = Digraph::cycle(order);

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
            assert!(Digraph::cycle(order).indegree_sequence().all(|d| d
                == match order {
                    1 => 0,
                    2 => 1,
                    _ => 2,
                }));
        }

        #[test]
        fn cycle_is_balanced(order in 1..25_usize) {
            assert!(Digraph::cycle(order).is_balanced());
        }

        #[test]
        fn cycle_is_complete(order in 1..25_usize) {
            assert!((order < 4) == Digraph::cycle(order).is_complete());
        }

        #[test]
        fn cycle_is_isolated(order in 1..25_usize) {
            let digraph = Digraph::cycle(order);

            assert!(digraph
                .vertices()
                .all(|u| { (order == 1) == digraph.is_isolated(u) }));
        }

        #[test]
        fn cycle_is_oriented(order in 1..25_usize) {
            assert!((order == 1) == Digraph::cycle(order).is_oriented());
        }

        #[test]
        fn cycle_is_pendant(order in 1..25_usize) {
            let digraph = Digraph::cycle(order);

            assert!(digraph.vertices().all(|u| { !digraph.is_pendant(u) }));
        }

        #[test]
        fn cycle_is_regular(order in 1..25_usize) {
            assert!(Digraph::cycle(order).is_regular());
        }

        #[test]
        fn cycle_is_semicomplete(order in 1..25_usize) {
            assert!((order < 4) == Digraph::cycle(order).is_semicomplete());
        }

        #[test]
        fn cycle_is_simple(order in 1..25_usize) {
            assert!(Digraph::cycle(order).is_simple());
        }

        #[test]
        fn cycle_is_sink(order in 1..25_usize) {
            let digraph = Digraph::cycle(order);

            assert!(digraph
                .vertices()
                .all(|u| { (order == 1) == digraph.is_sink(u) }));
        }

        #[test]
        fn cycle_is_source(order in 1..25_usize) {
            let digraph = Digraph::cycle(order);

            assert!(digraph
                .vertices()
                .all(|u| { (order == 1) == digraph.is_source(u) }));
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
        fn cycle_is_tournament(order in 1..25_usize) {
            assert!((order == 1) == Digraph::cycle(order).is_tournament());
        }

        #[test]
        fn cycle_outdegree(order in 1..25_usize) {
            let digraph = Digraph::cycle(order);

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
            assert!(Digraph::cycle(order).outdegree_sequence().all(|d| d
                == match order {
                    1 => 0,
                    2 => 1,
                    _ => 2,
                }));
        }

        #[test]
        fn cycle_semidegree_sequence(order in 1..25_usize) {
            assert!(Digraph::cycle(order).semidegree_sequence().all(|d| d
                == match order {
                    1 => (0, 0),
                    2 => (1, 1),
                    _ => (2, 2),
                }));
        }

        #[test]
        fn cycle_sinks(order in 1..25_usize) {
            assert!(if order == 1 {
                Digraph::cycle(order).sinks().eq([0])
            } else {
                Digraph::cycle(order).sinks().eq([])
            });
        }

        #[test]
        fn cycle_size(order in 1..25_usize) {
            assert_eq!(
                Digraph::cycle(order).size(),
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
                Digraph::cycle(order).sources().eq([0])
            } else {
                Digraph::cycle(order).sources().eq([])
            });
        }

        #[test]
        fn empty_arcs(order in 1..25_usize) {
            assert!(Digraph::empty(order).arcs().eq([]));
        }

        #[test]
        fn empty_complement_size(order in 1..25_usize) {
            assert_eq!(
                Digraph::empty(order).complement().size(),
                order * (order - 1)
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
        fn empty_degree(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            assert!(digraph.vertices().all(|u| digraph.degree(u) == 0));
        }

        #[test]
        fn empty_degree_sequence(order in 1..25_usize) {
            assert!(Digraph::empty(order).degree_sequence().all(|d| d == 0));
        }

        #[test]
        fn empty_degree_sum_equals_2size(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            assert_eq!(
                digraph.vertices().fold(0, |acc, u| acc + digraph.degree(u)),
                2 * digraph.size()
            );
        }

        #[test]
        fn empty_even_number_odd_degrees(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

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
            let digraph = Digraph::empty(order);

            assert!(digraph.vertices().all(|u| {
                digraph.vertices().all(|v| !digraph.has_arc(u, v))
            }));
        }

        #[test]
        fn empty_has_edge(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            assert!(digraph.vertices().all(|u| {
                digraph.vertices().all(|v| !digraph.has_edge(u, v))
            }));
        }

        #[test]
        fn empty_indegree(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            assert!(digraph.vertices().all(|u| digraph.indegree(u) == 0));
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
        fn empty_is_complete(order in 1..25_usize) {
            assert!((order == 1) == Digraph::empty(order).is_complete());
        }

        #[test]
        fn empty_is_isolated(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            assert!(digraph.vertices().all(|u| { digraph.is_isolated(u) }));
        }

        #[test]
        fn empty_is_oriented(order in 1..25_usize) {
            assert!(Digraph::empty(order).is_oriented());
        }

        #[test]
        fn empty_is_pendant(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            assert!(digraph.vertices().all(|u| { !digraph.is_pendant(u) }));
        }

        #[test]
        fn empty_is_regular(order in 1..25_usize) {
            assert!(Digraph::empty(order).is_regular());
        }

        #[test]
        fn empty_is_semicomplete(order in 1..25_usize) {
            assert!((order == 1) == Digraph::empty(order).is_semicomplete());
        }

        #[test]
        fn empty_is_simple(order in 1..25_usize) {
            assert!(Digraph::empty(order).is_simple());
        }

        #[test]
        fn empty_is_sink(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            assert!(digraph.vertices().all(|u| digraph.is_sink(u)));
        }

        #[test]
        fn empty_is_source(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            assert!(digraph.vertices().all(|u| digraph.is_source(u)));
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
        fn empty_is_tournament(order in 1..25_usize) {
            assert!((order == 1) == Digraph::empty(order).is_tournament());
        }

        #[test]
        fn empty_outdegree(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            assert!(digraph.vertices().all(|u| digraph.outdegree(u) == 0));
        }

        #[test]
        fn empty_outdegree_sequence(order in 1..25_usize) {
            assert!(Digraph::empty(order)
                .outdegree_sequence()
                .all(|d| d == 0));
        }

        #[test]
        fn empty_semidegree_sequence(order in 1..25_usize) {
            assert!(Digraph::empty(order)
                .semidegree_sequence()
                .all(|d| d == (0, 0)));
        }

        #[test]
        fn empty_sinks(order in 1..25_usize) {
            assert!(Digraph::empty(order).sinks().eq(0..order));
        }

        #[test]
        fn empty_size(order in 1..25_usize) {
            assert_eq!(Digraph::empty(order).size(), 0);
        }

        #[test]
        fn empty_sources(order in 1..25_usize) {
            assert!(Digraph::empty(order).sources().eq(0..order));
        }

        #[test]
        fn erdos_renyi_degree(
            order in 1..25_usize,
            p in 0.0..1.0,
            seed in 0..1000_u64
        ) {
            let digraph = Digraph::erdos_renyi(order, p, seed);
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
            let digraph = Digraph::erdos_renyi(order, p, seed);

            assert_eq!(
                digraph.vertices().fold(0, |acc, u| acc + digraph.degree(u)),
                2 * digraph.size()
            );
        }

        #[test]
        fn erdos_renyi_even_number_odd_degrees(
            order in 1..25_usize,
            p in 0.0..1.0,
            seed in 0..1000_u64
        ) {
            let digraph = Digraph::erdos_renyi(order, p, seed);

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
            let digraph = Digraph::erdos_renyi(order, p, seed);

            assert!(digraph.vertices().all(|u| !digraph.has_arc(u, u) ));
        }

        #[test]
        fn erdos_renyi_indegree(
            order in 1..25_usize,
            p in 0.0..1.0,
            seed in 0..1000_u64
        ) {
            let digraph = Digraph::erdos_renyi(order, p, seed);

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
                assert!(!Digraph::erdos_renyi(order, p, seed).is_complete());
            } else if order == 1 {
                assert!(Digraph::erdos_renyi(order, p, seed).is_complete());
            }
        }

        #[test]
        fn erdos_renyi_is_simple(
            order in 1..25_usize,
            p in 0.0..1.0,
            seed in 0..1000_u64
        ) {
            assert!(Digraph::erdos_renyi(order, p, seed).is_simple());
        }

        #[test]
        fn erdos_renyi_is_subdigraph(
            order in 1..25_usize,
            p in 0.0..1.0,
            seed in 0..1000_u64
        ) {
            let digraph = Digraph::erdos_renyi(order, p, seed);

            assert!(digraph.is_subdigraph(&digraph));
        }

        #[test]
        fn erdos_renyi_is_superdigraph(
            order in 1..25_usize,
            p in 0.0..1.0,
            seed in 0..1000_u64
        ) {
            let digraph = Digraph::erdos_renyi(order, p, seed);

            assert!(digraph.is_superdigraph(&digraph));
        }

        #[test]
        fn erdos_renyi_order(
            order in 1..25_usize,
            p in 0.0..1.0,
            seed in 0..1000_u64
        ) {
            assert_eq!(Digraph::erdos_renyi(order, p, seed).order(), order);
        }

        #[test]
        fn erdos_renyi_outdegree(
            order in 1..25_usize,
            p in 0.0..1.0,
            seed in 0..1000_u64
        ) {
            let digraph = Digraph::erdos_renyi(order, p, seed);

            assert!(digraph.vertices().all(|u| {
                (0..order).contains(&digraph.outdegree(u))
            }));
        }

        #[test]
        fn erdos_renyi_size_p_0(order in 1..25_usize, seed: u64) {
            assert_eq!(Digraph::erdos_renyi(order, 0.0, seed).size(), 0);
        }

        #[test]
        fn erdos_renyi_size_p_1(order in 1..25_usize, seed: u64) {
            assert_eq!(
                Digraph::erdos_renyi(order, 1.0, seed).size(),
                order * (order - 1)
            );
        }

        #[test]
        fn has_arc_out_of_bounds(order in 1..25_usize) {
            let digraph = Digraph::empty(order);

            assert!((0..order)
                .all(|u| !digraph.has_arc(u, order)
                    && !digraph.has_arc(order, u)));
        }

        #[test]
        fn path_complement_size(order in 1..25_usize) {
            assert_eq!(
                Digraph::path(order).complement().size(),
                (order - 1).pow(2)
            );
        }

        #[test]
        fn path_degree(order in 1..25_usize) {
            let digraph = Digraph::path(order);
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
            let digraph = Digraph::path(order);
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
        fn path_has_edge(order in 1..25_usize) {
            let digraph = Digraph::path(order);

            assert!((0..order)
                .all(|u| (0..order).all(|v| !digraph.has_edge(u, v))));
        }

        #[test]
        fn path_indegree(order in 1..25_usize) {
            let digraph = Digraph::path(order);

            assert_eq!(digraph.indegree(0), 0);
            assert!((1..order).all(|u| digraph.indegree(u) == 1));
        }

        #[test]
        fn path_indegree_sequence(order in 1..25_usize) {
            let digraph = Digraph::path(order);
            let indegree_sequence = &mut digraph.indegree_sequence();

            assert_eq!(indegree_sequence.next(), Some(0));
            assert!(indegree_sequence.all(|d| d == 1));
        }

        #[test]
        fn path_is_balanced(order in 1..25_usize) {
            assert!((order == 1) == Digraph::path(order).is_balanced());
        }

        #[test]
        fn path_is_complete(order in 1..25_usize) {
            assert!((order == 1) == Digraph::path(order).is_complete());
        }

        #[test]
        fn path_is_isolated(order in 1..25_usize) {
            let digraph = Digraph::path(order);

            assert!(digraph
                .vertices()
                .all(|u| (order == 1) == digraph.is_isolated(u)));
        }

        #[test]
        fn path_is_oriented(order in 1..25_usize) {
            assert!(Digraph::path(order).is_oriented());
        }

        #[test]
        fn path_is_pendant(order in 1..25_usize) {
            let digraph = Digraph::path(order);
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
            assert!((order == 1) == Digraph::path(order).is_regular());
        }

        #[test]
        fn path_is_semicomplete(order in 1..25_usize) {
            assert!((order < 3) == Digraph::path(order).is_semicomplete());
        }

        #[test]
        fn path_is_simple(order in 1..25_usize) {
            assert!(Digraph::path(order).is_simple());
        }

        #[test]
        fn path_is_sink(order in 1..25_usize) {
            let digraph = Digraph::path(order);
            let last = order - 1;

            assert!(
                (order == 1 && digraph.is_sink(0))
                    || ((0..last).all(|u| !digraph.is_sink(u))
                        && digraph.is_sink(last))
            );
        }

        #[test]
        fn path_is_source(order in 1..25_usize) {
            let digraph = Digraph::path(order);

            assert!(digraph.is_source(0));
            assert!((1..order).all(|u| !digraph.is_source(u)));
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
        fn path_is_symmetric(order in 1..25_usize) {
            assert!((order == 1) == Digraph::path(order).is_symmetric());
        }

        #[test]
        fn path_is_tournament(order in 1..25_usize) {
            assert!((order < 3) == Digraph::path(order).is_tournament());
        }

        #[test]
        fn path_outdegree(order in 1..25_usize) {
            let digraph = Digraph::path(order);
            let last = order - 1;

            assert!((0..last).all(|u| digraph.outdegree(u) == 1));
            assert_eq!(digraph.outdegree(last), 0);
        }

        #[test]
        fn path_outdegree_sequence(order in 1..25_usize) {
            let digraph = Digraph::path(order);
            let outdegree_sequence = &mut digraph.outdegree_sequence();

            assert!(outdegree_sequence.take(order - 1).all(|d| d == 1));
            assert_eq!(outdegree_sequence.next(), Some(0));
        }

        #[test]
        fn path_semidegree_sequence(order in 1..25_usize) {
            let digraph = Digraph::path(order);
            let semidegree_sequence = &mut digraph.semidegree_sequence();

            assert!(if order == 1 {
                semidegree_sequence.next() == Some((0, 0))
            } else {
                semidegree_sequence.next() == Some((0, 1))
                    && semidegree_sequence.take(order - 2).all(|d| d == (1, 1))
                    && semidegree_sequence.next() == Some((1, 0))
            });
        }

        #[test]
        fn path_sinks(order in 1..25_usize) {
            assert!(Digraph::path(order).sinks().eq([order - 1]));
        }

        #[test]
        fn path_size(order in 1..25_usize) {
            assert_eq!(Digraph::path(order).size(), order - 1);
        }

        #[test]
        fn path_sources(order in 1..25_usize) {
            assert!(Digraph::path(order).sources().eq([0]));
        }

        #[test]
        fn random_tournament_complement_size(
            order in 1..25_usize,
            seed in 0..1000_u64
        ) {
            assert_eq!(
                Digraph::random_tournament(order, seed).complement().size(),
                order * (order - 1) / 2
            );
        }

        #[test]
        fn random_tournament_degree(
            order in 1..25_usize,
            seed in 0..1000_u64
        ) {
            let digraph = Digraph::random_tournament(order, seed);
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

            assert!(Digraph::random_tournament(order, seed)
                .degree_sequence()
                .all(|d| d == degree));
        }

        #[test]
        fn random_tournament_degree_sum_equals_2size(
            order in 1..25_usize,
            seed in 0..1000_u64
        ) {
            let digraph = Digraph::random_tournament(order, seed);

            assert_eq!(
                digraph.vertices().fold(0, |acc, u| acc + digraph.degree(u)),
                2 * digraph.size()
            );
        }

        #[test]
        fn random_tournament_even_number_odd_degrees(
            order in 1..25_usize,
            seed in 0..1000_u64
        ) {
            let digraph = Digraph::random_tournament(order, seed);

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
            let digraph = Digraph::random_tournament(order, seed);

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
            let digraph = Digraph::random_tournament(order, seed);

            assert!(digraph.vertices().all(|u| {
                digraph.vertices().all(|v| !digraph.has_edge(u, v))
            }));
        }

        #[test]
        fn random_tournament_indegree(
            order in 1..25_usize,
            seed in 0..1000_u64
        ) {
            let digraph = Digraph::random_tournament(order, seed);

            assert!(digraph
                .vertices()
                .all(|u| (0..order).contains(&digraph.indegree(u))));
        }

        #[test]
        fn random_tournament_indegree_sequence(
            order in 1..25_usize,
            seed in 0..1000_u64
        ) {
            let digraph = Digraph::random_tournament(order, seed);
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
                    == Digraph::random_tournament(order, seed).is_complete()
            );
        }

        #[test]
        fn random_tournament_is_isolated(
            order in 1..25_usize,
            seed in 0..1000_u64
        ) {
            let digraph = Digraph::random_tournament(order, seed);

            assert!(digraph
                .vertices()
                .all(|u| { (order == 1) == digraph.is_isolated(u) }));
        }

        #[test]
        fn random_tournament_is_oriented(
            order in 1..25_usize,
            seed in 0..1000_u64
        ) {
            assert!(Digraph::random_tournament(order, seed).is_oriented());
        }

        #[test]
        fn random_tournament_is_pendant(
            order in 1..25_usize,
            seed in 0..1000_u64
        ) {
            let digraph = Digraph::random_tournament(order, seed);

            assert!(digraph
                .vertices()
                .all(|u| (order == 2) == digraph.is_pendant(u)));
        }

        #[test]
        fn random_tournament_is_semicomplete(
            order in 1..25_usize,
            seed in 0..1000_u64
        ) {
            assert!(Digraph::random_tournament(order, seed).is_semicomplete());
        }

        #[test]
        fn random_tournament_is_simple(
            order in 1..25_usize,
            seed in 0..1000_u64
        ) {
            assert!(Digraph::random_tournament(order, seed).is_simple());
        }

        #[test]
        fn random_tournament_is_spanning_subdigraph(
            order in 1..25_usize,
            seed in 0..1000_u64
        ) {
            let digraph = Digraph::random_tournament(order, seed);

            assert!(digraph.is_spanning_subdigraph(&digraph));
        }

        #[test]
        fn random_tournament_is_subdigraph(
            order in 1..25_usize,
            seed in 0..1000_u64
        ) {
            let digraph = Digraph::random_tournament(order, seed);

            assert!(digraph.is_subdigraph(&digraph));
        }

        #[test]
        fn random_tournament_is_superdigraph(
            order in 1..25_usize,
            seed in 0..1000_u64
        ) {
            let digraph = Digraph::random_tournament(order, seed);

            assert!(digraph.is_superdigraph(&digraph));
        }

        #[test]
        fn random_tournament_is_symmetric(
            order in 1..25_usize,
            seed in 0..1000_u64
        ) {
            assert!(
                (order == 1)
                    == Digraph::random_tournament(order, seed).is_symmetric()
            );
        }

        #[test]
        fn random_tournament_is_tournament(
            order in 1..25_usize,
            seed in 0..1000_u64
        ) {
            assert!(Digraph::random_tournament(order, seed).is_tournament());
        }

        #[test]
        fn random_tournament_order(
            order in 1..25_usize,
            seed in 0..1000_u64
        ) {
            assert_eq!(Digraph::random_tournament(order, seed).order(), order);
        }

        #[test]
        fn random_tournament_outdegree(
            order in 1..25_usize,
            seed in 0..1000_u64
        ) {
            let digraph = Digraph::random_tournament(order, seed);

            assert!(digraph
                .vertices()
                .all(|u| { (0..order).contains(&digraph.outdegree(u)) }));
        }

        #[test]
        fn random_tournament_outdegree_sequence(
            order in 1..25_usize,
            seed in 0..1000_u64
        ) {
            let digraph = Digraph::random_tournament(order, seed);
            let outdegree_sequence = &mut digraph.outdegree_sequence();

            assert!(outdegree_sequence.all(|d| (0..order).contains(&d)));
        }

        #[test]
        fn random_tournament_semidegree_sequence(
            order in 1..25_usize,
            seed in 0..1000_u64
        ) {
            assert_eq!(
                Digraph::random_tournament(order, seed)
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
                Digraph::random_tournament(order, seed).size(),
                order * (order - 1) / 2
            );
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
            assert!((1..order).all(|u| digraph.degree(u) == 2));
        }

        #[test]
        fn star_degree_sequence(order in 1..25_usize) {
            let digraph = Digraph::star(order);
            let degree_sequence = &mut digraph.degree_sequence();

            assert_eq!(degree_sequence.next(), Some((order - 1) * 2));
            assert!(degree_sequence.all(|d| d == 2));
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

            assert!((1..order).all(|u| digraph.has_edge(0, u)
                && (u..order).all(|v| !digraph.has_edge(u, v))));
        }

        #[test]
        fn star_indegree(order in 1..25_usize) {
            let digraph = Digraph::star(order);

            assert_eq!(digraph.indegree(0), order - 1);
            assert!((1..order).all(|u| digraph.indegree(u) == 1));
        }

        #[test]
        fn star_indegree_sequence(order in 1..25_usize) {
            let digraph = Digraph::star(order);
            let indegree_sequence = &mut digraph.indegree_sequence();

            assert_eq!(indegree_sequence.next(), Some(order - 1));
            assert!(indegree_sequence.all(|d| d == 1));
        }

        #[test]
        fn star_is_balanced(order in 1..25_usize) {
            assert!(Digraph::star(order).is_balanced());
        }

        #[test]
        fn star_is_complete(order in 1..25_usize) {
            assert!((order < 3) == Digraph::star(order).is_complete());
        }

        #[test]
        fn star_is_isolated(order in 1..25_usize) {
            let digraph = Digraph::star(order);

            assert!(digraph
                .vertices()
                .all(|u| { (order == 1) == digraph.is_isolated(u) }));
        }

        #[test]
        fn star_is_oriented(order in 1..25_usize) {
            assert!((order == 1) == Digraph::star(order).is_oriented());
        }

        #[test]
        fn star_is_pendant(order in 1..25_usize) {
            let digraph = Digraph::star(order);

            assert!(digraph.vertices().all(|u| {
                    !digraph.is_pendant(u)
            }));
        }

        #[test]
        fn star_is_regular(order in 1..25_usize) {
            assert!((order < 3) == Digraph::star(order).is_regular());
        }

        #[test]
        fn star_is_semicomplete(order in 1..25_usize) {
            assert!((order < 3) == Digraph::star(order).is_semicomplete());
        }

        #[test]
        fn star_is_simple(order in 1..25_usize) {
            assert!(Digraph::star(order).is_simple());
        }

        #[test]
        fn star_is_sink(order in 1..25_usize) {
            let digraph = Digraph::star(order);

            assert!(digraph
                .vertices()
                .all(|u| { (order == 1) == digraph.is_sink(u) }));
        }

        #[test]
        fn star_is_source(order in 1..25_usize) {
            let digraph = Digraph::star(order);

            assert!(digraph
                .vertices()
                .all(|u| { (order == 1) == digraph.is_source(u) }));
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
        fn star_is_tournament(order in 1..25_usize) {
            assert!((order == 1) == Digraph::star(order).is_tournament());
        }

        #[test]
        fn star_outdegree(order in 1..25_usize) {
            let digraph = Digraph::star(order);

            assert_eq!(digraph.outdegree(0), order - 1);
            assert!((1..order).all(|u| digraph.outdegree(u) == 1));
        }

        #[test]
        fn star_outdegree_sequence(order in 1..25_usize) {
            let digraph = Digraph::star(order);
            let outdegree_sequence = &mut digraph.outdegree_sequence();

            assert_eq!(outdegree_sequence.next(), Some(order - 1));
            assert!(outdegree_sequence.all(|d| d == 1));
        }

        #[test]
        fn star_semidegree_sequence(order in 1..25_usize) {
            let digraph = Digraph::star(order);
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
                Digraph::star(order).sinks().eq([0])
            } else {
                Digraph::star(order).sinks().eq([])
            });
        }

        #[test]
        fn star_size(order in 1..25_usize) {
            assert_eq!(Digraph::star(order).size(), (order - 1) * 2);
        }

        #[test]
        fn star_sources(order in 1..25_usize) {
            assert!(if order == 1 {
                Digraph::star(order).sources().eq([0])
            } else {
                Digraph::star(order).sources().eq([])
            });
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
        let _ = Digraph::biclique(0, 1);
    }

    #[test]
    #[should_panic(expected = "n = 0 must be greater than zero")]
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
    fn biclique_1_2() {
        assert!(Digraph::biclique(1, 2).arcs().eq([
            (0, 1),
            (0, 2),
            (1, 0),
            (2, 0)
        ]));
    }

    #[test]
    fn biclique_1_2_complement() {
        assert!(Digraph::biclique(1, 2)
            .complement()
            .arcs()
            .eq([(1, 2), (2, 1)]));
    }

    #[test]
    fn biclique_2_1() {
        assert!(Digraph::biclique(2, 1).arcs().eq([
            (0, 2),
            (1, 2),
            (2, 0),
            (2, 1)
        ]));
    }

    #[test]
    fn biclique_2_1_complement() {
        assert!(Digraph::biclique(2, 1)
            .complement()
            .arcs()
            .eq([(0, 1), (1, 0)]));
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
        assert!(Digraph::biclique(2, 2).complement().arcs().eq([
            (0, 1),
            (1, 0),
            (2, 3),
            (3, 2),
        ]));
    }

    #[test]
    fn biclique_claw() {
        assert!(Digraph::claw().arcs().eq([
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
    fn circuit_2() {
        assert!(Digraph::circuit(2).arcs().eq([(0, 1), (1, 0)]));
    }

    #[test]
    fn circuit_2_complement() {
        assert!(Digraph::circuit(2).complement().arcs().eq([]));
    }

    #[test]
    fn circuit_3() {
        assert!(Digraph::circuit(3).arcs().eq([(0, 1), (1, 2), (2, 0)]));
    }

    #[test]
    fn circuit_3_complement() {
        assert!(Digraph::circuit(3).complement().arcs().eq([
            (0, 2),
            (1, 0),
            (2, 1)
        ]));
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
    fn complete_2() {
        assert!(Digraph::complete(2).arcs().eq([(0, 1), (1, 0)]));
    }

    #[test]
    fn complete_2_complement() {
        assert!(Digraph::complete(2).complement().arcs().eq([]));
    }

    #[test]
    fn complete_3() {
        assert!(Digraph::complete(3).arcs().eq([
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
        assert!(Digraph::complete(3).complement().arcs().eq([]));
    }

    #[test]
    fn converse_bang_jensen_196() {
        assert!(bang_jensen_196().converse().arcs().eq([
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
    fn converse_kattis_cantinaofbabel_1() {
        assert!(kattis_cantinaofbabel_1().converse().arcs().eq([
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
        assert!(kattis_cantinaofbabel_2().converse().arcs().eq([
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
    fn cycle_1() {
        assert!(Digraph::cycle(1).arcs().eq([]));
    }

    #[test]
    fn cycle_1_complement() {
        assert!(Digraph::cycle(1).complement().arcs().eq([]));
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
    fn cycle_3() {
        assert!(Digraph::cycle(3).arcs().eq([
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
        assert!(Digraph::cycle(3).complement().arcs().eq([]));
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
        assert!(bang_jensen_94().degree_sequence().eq([2, 3, 5, 3, 2, 2, 1]));
    }

    #[test]
    fn degree_sequence_kattis_builddeps() {
        assert!(kattis_builddeps().degree_sequence().eq([2, 3, 3, 3, 3, 2]));
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
    fn from_vec() {
        let digraph = Digraph::from(vec![
            BTreeSet::from([1]),
            BTreeSet::from([2]),
            BTreeSet::new(),
        ]);

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
    fn in_neighbors_kattis_canitnaofbabel_2() {
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
    #[should_panic(expected = "v = 1 is out of bounds")]
    fn indegree_out_of_bounds() {
        let _ = Digraph::trivial().indegree(1);
    }

    #[test]
    fn indegree_sequence_bang_jensen_196() {
        assert!(bang_jensen_196()
            .indegree_sequence()
            .eq([1, 1, 3, 1, 2, 1, 1, 3]));
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
    #[should_panic(expected = "u = 1 is out of bounds")]
    fn out_neighbors_out_of_bounds() {
        let _ = Digraph::trivial().out_neighbors(1);
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
        assert!(digraph.out_neighbors_weighted(10).eq([(9, &1), (11, &1)]));
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
        assert!(digraph.out_neighbors_weighted(13).eq([(9, &1), (12, &1)]));
    }

    #[test]
    #[should_panic(expected = "u = 1 is out of bounds")]
    fn out_neighbors_weighted_out_of_bounds() {
        let _ = Digraph::trivial().out_neighbors_weighted(1);
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
        assert!(bang_jensen_34().outdegree_sequence().eq([1, 1, 3, 0, 0, 1]));
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
    fn path_2() {
        assert!(Digraph::path(2).arcs().eq([(0, 1)]));
    }

    #[test]
    fn path_2_complement() {
        assert!(Digraph::path(2).complement().arcs().eq([(1, 0)]));
    }

    #[test]
    fn path_3() {
        assert!(Digraph::path(3).arcs().eq([(0, 1), (1, 2)]));
    }

    #[test]
    fn path_3_complement() {
        assert!(Digraph::path(3).complement().arcs().eq([
            (0, 2),
            (1, 0),
            (2, 0),
            (2, 1)
        ]));
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
        assert!(!Digraph::trivial().remove_arc(0, 1));
        assert!(!Digraph::trivial().remove_arc(1, 0));
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
    fn star_1() {
        assert!(Digraph::star(1).arcs().eq([]));
    }

    #[test]
    fn star_1_complement() {
        assert!(Digraph::star(1).complement().arcs().eq([]));
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

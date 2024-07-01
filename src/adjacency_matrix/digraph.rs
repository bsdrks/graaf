//! An adjacency matrix representation for unweighted digraphs
//!
//! An adjacency matrix is a symmetric binary matrix where a value of `1` at
//! row `s` and column `t` indicates an arc from vertex `s` to vertex `t`. The
//! matrix is stored as a bit array, and is suited for dense digraphs with a
//! small number of vertices.

use crate::{
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
    /// Panics if `s >= V` or `t >= V`.
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

        assert!(s < v, "s is not in the digraph");
        assert!(t < v, "t is not in the digraph");

        let i = self.index(s, t);

        self.blocks[i >> 6] ^= Self::mask(i);
    }
}

impl AddArc for Digraph {
    /// # Panics
    ///
    /// Panics if `s` or `t` is not in the digraph.
    fn add_arc(&mut self, s: usize, t: usize) {
        let v = self.order;

        assert!(s < v, "s is not in the digraph");
        assert!(t < v, "t is not in the digraph");

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
        let v = self.order();
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

impl HasArc for Digraph {
    fn has_arc(&self, s: usize, t: usize) -> bool {
        if s >= self.order || t >= self.order {
            return false;
        }

        let i = self.index(s, t);

        self.blocks[i >> 6] & Self::mask(i) != 0
    }
}

impl Indegree for Digraph {
    /// # Panics
    ///
    /// Panics if `t >= V`.
    fn indegree(&self, t: usize) -> usize {
        assert!(t < self.order, "t is not in the digraph");

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
    /// Panics if `s >= V`.
    fn out_neighbors(&self, s: usize) -> impl Iterator<Item = usize> {
        assert!(s < self.order, "s is not in the digraph");

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
        assert!(s < self.order, "s is not in the digraph");

        self.vertices().filter(|&t| self.has_arc(s, t)).count()
    }
}

impl RemoveArc for Digraph {
    /// # Panics
    ///
    /// Panics if `s >= V` or `t >= V`.
    fn remove_arc(&mut self, s: usize, t: usize) -> bool {
        let v = self.order;

        assert!(s < v, "s is not in the digraph");
        assert!(t < v, "t is not in the digraph");

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
            gen::Cycle,
            op::{
                HasEdge,
                IsBalanced,
                IsIsolated,
                IsSymmetric,
            },
        },
    };

    #[test]
    fn toggle() {
        let mut digraph = Digraph::empty(3);

        digraph.toggle(0, 1);
        digraph.toggle(0, 2);

        assert_eq!(digraph.blocks, [0b110]);
    }

    #[test]
    #[should_panic(expected = "s is not in the digraph")]
    fn toggle_panic_s_gte_v() {
        let mut digraph = Digraph::empty(3);

        digraph.toggle(3, 0);
    }

    #[test]
    #[should_panic(expected = "t is not in the digraph")]
    fn toggle_panic_t_gte_v() {
        let mut digraph = Digraph::empty(3);

        digraph.toggle(0, 3);
    }

    #[test]
    fn add_arc() {
        let mut digraph = Digraph::empty(3);

        digraph.add_arc(0, 1);
        digraph.add_arc(0, 2);

        assert_eq!(digraph.blocks, [0b110]);
    }

    #[test]
    #[should_panic(expected = "s is not in the digraph")]
    fn add_arc_panic_s_gte_v() {
        let mut digraph = Digraph::empty(3);

        digraph.add_arc(3, 0);
    }

    #[test]
    #[should_panic(expected = "t is not in the digraph")]
    fn add_arc_panic_t_gte_v() {
        let mut digraph = Digraph::empty(3);

        digraph.add_arc(0, 3);
    }

    #[test]
    fn arc_weight() {
        let digraph = Digraph::cycle(3);

        assert_eq!(digraph.arc_weight(0, 0), None);
        assert_eq!(digraph.arc_weight(0, 1), Some(&1));
        assert_eq!(digraph.arc_weight(0, 2), None);
        assert_eq!(digraph.arc_weight(0, 3), None);
        assert_eq!(digraph.arc_weight(1, 0), None);
        assert_eq!(digraph.arc_weight(1, 1), None);
        assert_eq!(digraph.arc_weight(1, 2), Some(&1));
        assert_eq!(digraph.arc_weight(1, 3), None);
        assert_eq!(digraph.arc_weight(2, 0), Some(&1));
        assert_eq!(digraph.arc_weight(2, 1), None);
        assert_eq!(digraph.arc_weight(2, 2), None);
        assert_eq!(digraph.arc_weight(2, 3), None);
    }

    #[test]
    fn cycle() {
        assert!(Digraph::cycle(1).arcs().eq([(0, 0)]));
        assert!(Digraph::cycle(2).arcs().eq([(0, 1), (1, 0)]));
        assert!(Digraph::cycle(3).arcs().eq([(0, 1), (1, 2), (2, 0)]));
    }

    #[test]
    fn empty() {
        assert_eq!(Digraph::empty(1).blocks, [0]);
        assert_eq!(Digraph::empty(2).blocks, [0]);
        assert_eq!(Digraph::empty(3).blocks, [0]);
    }

    #[test]
    #[should_panic(expected = "a digraph must have at least one vertex")]
    fn empty_panic() {
        let _ = Digraph::empty(0);
    }

    #[test]
    fn has_arc() {
        let mut digraph = Digraph::empty(3);

        assert!(!digraph.has_arc(0, 1));
        assert!(!digraph.has_arc(0, 2));
        assert!(!digraph.has_arc(1, 0));
        assert!(!digraph.has_arc(1, 2));
        assert!(!digraph.has_arc(2, 0));
        assert!(!digraph.has_arc(2, 1));
        assert!(!digraph.has_arc(3, 0));
        assert!(!digraph.has_arc(0, 3));

        digraph.add_arc(0, 1);
        digraph.add_arc(0, 2);
        digraph.add_arc(2, 1);

        assert!(digraph.has_arc(0, 1));
        assert!(digraph.has_arc(0, 2));
        assert!(!digraph.has_arc(1, 0));
        assert!(!digraph.has_arc(1, 2));
        assert!(!digraph.has_arc(2, 0));
        assert!(digraph.has_arc(2, 1));
        assert!(!digraph.has_arc(3, 0));
        assert!(!digraph.has_arc(0, 3));
    }

    #[test]
    fn has_edge() {
        let mut digraph = Digraph::empty(3);

        assert!(!digraph.has_edge(0, 1));
        assert!(!digraph.has_edge(0, 2));
        assert!(!digraph.has_edge(1, 0));
        assert!(!digraph.has_edge(1, 2));
        assert!(!digraph.has_edge(2, 0));
        assert!(!digraph.has_edge(2, 1));
        assert!(!digraph.has_edge(3, 0));
        assert!(!digraph.has_edge(0, 3));

        digraph.add_arc(0, 1);
        digraph.add_arc(0, 2);
        digraph.add_arc(1, 0);
        digraph.add_arc(2, 1);

        assert!(digraph.has_edge(0, 1));
        assert!(!digraph.has_edge(0, 2));
        assert!(digraph.has_edge(1, 0));
        assert!(!digraph.has_edge(1, 2));
        assert!(!digraph.has_edge(2, 0));
        assert!(!digraph.has_edge(2, 1));
        assert!(!digraph.has_edge(3, 0));
        assert!(!digraph.has_edge(0, 3));
    }

    #[test]
    fn indegree() {
        let mut digraph = Digraph::empty(3);

        assert_eq!(digraph.indegree(0), 0);
        assert_eq!(digraph.indegree(1), 0);
        assert_eq!(digraph.indegree(2), 0);

        digraph.add_arc(0, 1);
        digraph.add_arc(0, 2);

        assert_eq!(digraph.indegree(0), 0);
        assert_eq!(digraph.indegree(1), 1);
        assert_eq!(digraph.indegree(2), 1);
    }

    #[test]
    #[should_panic(expected = "t is not in the digraph")]
    fn indegree_panic_t_gte_v() {
        let digraph = Digraph::empty(3);
        let _ = digraph.indegree(3);
    }

    #[test]
    fn is_balanced() {
        let mut digraph = Digraph::empty(3);

        assert!(digraph.is_balanced());

        digraph.add_arc(0, 1);

        assert!(!digraph.is_balanced());

        digraph.add_arc(1, 0);

        assert!(digraph.is_balanced());

        digraph.add_arc(0, 2);

        assert!(!digraph.is_balanced());

        digraph.add_arc(1, 2);

        assert!(!digraph.is_balanced());

        digraph.add_arc(2, 0);

        assert!(!digraph.is_balanced());

        digraph.add_arc(2, 1);

        assert!(digraph.is_balanced());
    }

    #[test]
    fn is_isolated() {
        let mut digraph = Digraph::empty(4);

        digraph.add_arc(0, 1);
        digraph.add_arc(0, 2);
        digraph.add_arc(1, 2);

        assert!(!digraph.is_isolated(0));
        assert!(!digraph.is_isolated(1));
        assert!(!digraph.is_isolated(2));
        assert!(digraph.is_isolated(3));
    }

    #[test]
    fn is_simple() {
        let mut digraph = Digraph::empty(3);

        digraph.add_arc(0, 1);
        digraph.add_arc(0, 2);
        digraph.add_arc(2, 1);

        assert!(digraph.is_simple());
    }

    #[test]
    fn is_simple_self_loop() {
        let mut digraph = Digraph::empty(3);

        digraph.add_arc(0, 0);

        assert!(!digraph.is_simple());
    }

    #[test]
    fn is_symmetric_simple() {
        let mut digraph = Digraph::empty(2);

        digraph.add_arc(0, 1);
        digraph.add_arc(1, 0);

        assert!(digraph.is_symmetric());

        let mut digraph = Digraph::empty(2);

        digraph.add_arc(0, 1);

        assert!(!digraph.is_symmetric());

        let mut digraph = Digraph::empty(3);

        digraph.add_arc(0, 1);
        digraph.add_arc(0, 2);
        digraph.add_arc(1, 2);
        digraph.add_arc(2, 0);

        assert!(!digraph.is_symmetric());
    }

    #[test]
    fn arcs() {
        let mut digraph = Digraph::empty(3);

        digraph.add_arc(0, 1);
        digraph.add_arc(1, 2);
        digraph.add_arc(2, 0);

        assert!(digraph.arcs().eq([(0, 1), (1, 2), (2, 0)]));
    }

    #[test]
    fn out_neighbors() {
        let mut digraph = Digraph::empty(3);

        digraph.add_arc(0, 1);
        digraph.add_arc(0, 2);
        digraph.add_arc(2, 1);

        assert!(digraph.out_neighbors(0).eq([1, 2]));
        assert!(digraph.out_neighbors(1).eq([]));
        assert!(digraph.out_neighbors(2).eq([1]));
    }

    #[test]
    #[should_panic(expected = "s is not in the digraph")]
    fn out_neighbors_panic_s_gte_v() {
        let digraph = Digraph::empty(3);
        let _ = digraph.out_neighbors(3);
    }

    #[test]
    fn vertices() {
        let digraph = Digraph::empty(3);

        assert!(digraph.vertices().eq([0, 1, 2]));
    }

    #[test]
    fn outdegree() {
        let mut digraph = Digraph::empty(3);

        assert_eq!(digraph.outdegree(0), 0);
        assert_eq!(digraph.outdegree(1), 0);
        assert_eq!(digraph.outdegree(2), 0);

        digraph.add_arc(0, 1);
        digraph.add_arc(0, 2);
        digraph.add_arc(2, 1);

        assert_eq!(digraph.outdegree(0), 2);
        assert_eq!(digraph.outdegree(1), 0);
        assert_eq!(digraph.outdegree(2), 1);
    }

    #[test]
    #[should_panic(expected = "s is not in the digraph")]
    fn outdegree_panic_s_gte_v() {
        let digraph = Digraph::empty(3);
        let _ = digraph.outdegree(3);
    }

    #[test]
    fn remove_arc() {
        let mut digraph = Digraph::empty(3);

        digraph.add_arc(0, 1);
        digraph.add_arc(0, 2);
        digraph.add_arc(1, 0);
        digraph.add_arc(2, 1);

        assert!(!digraph.has_arc(0, 0));
        assert!(digraph.has_arc(0, 1));
        assert!(digraph.has_arc(0, 2));
        assert!(digraph.has_arc(1, 0));
        assert!(!digraph.has_arc(1, 1));
        assert!(!digraph.has_arc(1, 2));
        assert!(!digraph.has_arc(2, 0));
        assert!(digraph.has_arc(2, 1));
        assert!(!digraph.has_arc(2, 2));

        assert!(digraph.remove_arc(0, 1));
        assert!(digraph.remove_arc(0, 2));
        assert!(digraph.remove_arc(1, 0));
        assert!(digraph.remove_arc(2, 1));

        assert!(!digraph.has_arc(0, 1));
        assert!(!digraph.has_arc(0, 2));
        assert!(!digraph.has_arc(1, 0));
        assert!(!digraph.has_arc(2, 1));
    }

    #[test]
    fn order() {
        let digraph = Digraph::empty(1);

        assert_eq!(digraph.order(), 1);

        let digraph = Digraph::empty(3);

        assert_eq!(digraph.order(), 3);

        let digraph = Digraph::empty(512);

        assert_eq!(digraph.order(), 512);
    }

    #[test]
    #[should_panic(expected = "s is not in the digraph")]
    fn remove_arc_panic_s_gte_v() {
        let mut digraph = Digraph::empty(3);

        let _ = digraph.remove_arc(3, 0);
    }

    #[test]
    #[should_panic(expected = "t is not in the digraph")]
    fn remove_arc_panic_t_gte_v() {
        let mut digraph = Digraph::empty(3);

        let _ = digraph.remove_arc(0, 3);
    }

    #[test]
    fn size() {
        let digraph = Digraph::empty(3);

        assert_eq!(digraph.size(), 0);

        let mut digraph = Digraph::empty(3);

        digraph.add_arc(0, 1);

        assert_eq!(digraph.size(), 1);

        digraph.add_arc(0, 2);

        assert_eq!(digraph.size(), 2);
    }
}

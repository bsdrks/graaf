//! Represent sparse unweighted digraphs.
//!
//! An [`AdjacencyList`] is a vector of sets.
//!
//! # Examples
//!
//! ## Valid digraph
//!
//! A valid digraph of order `5` and size `8`.
//!
//! ![Digraph of order `5` and size `8`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_list_1.svg?)
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Arcs,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyList::empty(5);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//! digraph.add_arc(1, 3);
//! digraph.add_arc(1, 4);
//! digraph.add_arc(3, 0);
//! digraph.add_arc(3, 2);
//! digraph.add_arc(4, 1);
//!
//! assert!(digraph.arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (1, 0),
//!     (1, 3),
//!     (1, 4),
//!     (3, 0),
//!     (3, 2),
//!     (4, 1)
//! ]));
//! ```
//!
//! ## Self-loop
//!
//! A self-loop isn't allowed. [`AdjacencyList`] can't represent this
//! pseudograph. The self-loop is red.
//!
//! ![Self-loop](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_list_self_loop.svg?)
//!
//! Adding a self-loop panics.
//!
//! ```should_panic
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyList::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(3, 2);
//!
//! // This panics.
//! digraph.add_arc(2, 2);
//! ```
//!
//! ## Parallel arcs
//!
//! Parallel arcs aren't allowed. [`AdjacencyList`] can't represent this
//! multigraph. The parallel arc is red.
//!
//! ![Parallel arcs](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_list_parallel_arcs.svg?)
//!
//! Adding a parallel arc doesn't change the digraph.
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Arcs,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyList::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(3, 2);
//!
//! assert!(digraph.arcs().eq([(0, 1), (1, 2), (3, 2)]));
//!
//! // This doesn't change the digraph.
//! digraph.add_arc(0, 1);
//!
//! assert!(digraph.arcs().eq([(0, 1), (1, 2), (3, 2)]));
//! ```

pub mod fixture;

use {
    crate::{
        op::Union,
        AddArc,
        ArcWeight,
        Arcs,
        ArcsWeighted,
        Biclique,
        Circuit,
        Converse,
        Empty,
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
    std::collections::BTreeSet,
};

/// A representation of an unweighted digraph.
///
/// # Examples
///
/// ## Valid digraph
///
/// A valid digraph of order `5` and size `8`.
///
/// ![Digraph of order `5` and size `8`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_list_1.svg?)
///
/// ```
/// use graaf::{
///     AddArc,
///     AdjacencyList,
///     Arcs,
///     Empty,
/// };
///
/// let mut digraph = AdjacencyList::empty(5);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(0, 2);
/// digraph.add_arc(1, 0);
/// digraph.add_arc(1, 3);
/// digraph.add_arc(1, 4);
/// digraph.add_arc(3, 0);
/// digraph.add_arc(3, 2);
/// digraph.add_arc(4, 1);
///
/// assert!(digraph.arcs().eq([
///     (0, 1),
///     (0, 2),
///     (1, 0),
///     (1, 3),
///     (1, 4),
///     (3, 0),
///     (3, 2),
///     (4, 1)
/// ]));
/// ```
///
/// ## Self-loop
///
/// A self-loop isn't allowed. [`AdjacencyList`] can't represent this digraph.
/// The self-loop is red.
///
/// ![Self-loop](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_list_self_loop.svg?)
///
/// Adding a self-loop panics.
///
/// ```should_panic
/// use graaf::{
///     AddArc,
///     AdjacencyList,
///     Empty,
/// };
///
/// let mut digraph = AdjacencyList::empty(4);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(3, 2);
///
/// // This panics.
/// digraph.add_arc(2, 2);
/// ```
///
/// ## Parallel arcs
///
/// Parallel arcs aren't allowed. [`AdjacencyList`] can't represent this
/// multigraph. The parallel arc is red.
///
/// ![Parallel arcs](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_list_parallel_arcs.svg?)
///
/// Adding a parallel arc doesn't change the digraph.
///
/// ```
/// use graaf::{
///     AddArc,
///     AdjacencyList,
///     Arcs,
///     Empty,
/// };
///
/// let mut digraph = AdjacencyList::empty(4);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(3, 2);
///
/// assert!(digraph.arcs().eq([(0, 1), (1, 2), (3, 2)]));
///
/// // This doesn't change the digraph.
/// digraph.add_arc(0, 1);
///
/// assert!(digraph.arcs().eq([(0, 1), (1, 2), (3, 2)]));
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AdjacencyList {
    arcs: Vec<BTreeSet<usize>>,
}

impl AddArc for AdjacencyList {
    /// # Panics
    ///
    /// * Panics if `u` equals `v`; self-loops aren't allowed.
    /// * Panics if `u` isn't in the digraph.
    /// * Panics if `v` isn't in the digraph.
    fn add_arc(&mut self, u: usize, v: usize) {
        // Self-loops aren't allowed.
        assert_ne!(u, v, "u = {u} equals v = {v}");

        let order = self.order();

        assert!(u < order, "u = {u} isn't in the digraph");
        assert!(v < order, "v = {v} isn't in the digraph");

        let _ = self.arcs[u].insert(v);
    }
}

impl ArcWeight<usize> for AdjacencyList {
    fn arc_weight(&self, u: usize, v: usize) -> Option<&usize> {
        self.has_arc(u, v).then_some(&1)
    }
}

impl Arcs for AdjacencyList {
    fn arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.arcs
            .iter()
            .enumerate()
            .flat_map(|(u, set)| set.iter().map(move |&v| (u, v)))
    }
}

impl ArcsWeighted<usize> for AdjacencyList {
    fn arcs_weighted<'a>(
        &'a self,
    ) -> impl Iterator<Item = (usize, usize, &'a usize)>
    where
        usize: 'a,
    {
        self.arcs().map(move |(u, v)| (u, v, &1))
    }
}

impl Biclique for AdjacencyList {
    /// # Panics
    ///
    /// * Panics if `m` is zero.
    /// * Panics if `n` is zero.
    fn biclique(m: usize, n: usize) -> Self {
        assert!(m > 0, "m = {m} must be greater than zero");
        assert!(n > 0, "n = {n} must be greater than zero");

        let order = m + n;
        let clique_1 = (0..m).collect::<BTreeSet<_>>();
        let clique_2 = (m..order).collect::<BTreeSet<_>>();
        let mut digraph = Self::empty(order);

        for u in 0..m {
            digraph.arcs[u].clone_from(&clique_2);
        }

        for u in m..order {
            digraph.arcs[u].clone_from(&clique_1);
        }

        digraph
    }
}

impl Circuit for AdjacencyList {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn circuit(order: usize) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        if order == 1 {
            return Self::trivial();
        }

        Self {
            arcs: (0..order)
                .map(|u| BTreeSet::from([(u + 1) % order]))
                .collect::<Vec<_>>(),
        }
    }
}

impl Converse for AdjacencyList {
    /// # Panics
    ///
    /// Panics if the digraphs' order is zero.
    fn converse(&self) -> Self {
        let order = self.order();
        let mut converse = Self::empty(order);

        for (u, v) in self.arcs() {
            converse.add_arc(v, u);
        }

        converse
    }
}

impl Empty for AdjacencyList {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn empty(order: usize) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        Self::from(vec![BTreeSet::new(); order])
    }
}

impl<I> From<I> for AdjacencyList
where
    I: IntoIterator<Item = BTreeSet<usize>>,
{
    /// # Panics
    ///
    /// * Panics if, for any arc `u -> v` in `arcs`, `u` equals `v`.
    /// * Panics if, for any arc `u -> v` in `arcs`, `v` isn't in the digraph.
    fn from(iter: I) -> Self {
        let digraph = Self {
            arcs: iter.into_iter().collect(),
        };

        for (u, v) in digraph.arcs() {
            assert_ne!(u, v, "u = {u} equals v = {v}");
            assert!(v < digraph.order(), "v = {v} isn't in the digraph");
        }

        digraph
    }
}

impl HasArc for AdjacencyList {
    fn has_arc(&self, u: usize, v: usize) -> bool {
        self.arcs.get(u).map_or(false, |set| set.contains(&v))
    }
}

impl Indegree for AdjacencyList {
    /// # Panics
    ///
    /// Panics if `v` isn't in the digraph.
    fn indegree(&self, v: usize) -> usize {
        assert!(v < self.order(), "v = {v} isn't in the digraph");

        self.arcs.iter().filter(|set| set.contains(&v)).count()
    }
}

impl IsSimple for AdjacencyList {
    fn is_simple(&self) -> bool {
        // We only check for self-loops. Parallel arcs can't exist in this
        // representation.
        self.arcs
            .iter()
            .enumerate()
            .all(|(u, set)| !set.contains(&u))
    }
}

impl Order for AdjacencyList {
    fn order(&self) -> usize {
        self.arcs.len()
    }
}

impl OutNeighbors for AdjacencyList {
    /// # Panics
    ///
    /// Panics if `u` isn't in the digraph.
    fn out_neighbors(&self, u: usize) -> impl Iterator<Item = usize> {
        assert!(u < self.order(), "u = {u} isn't in the digraph");

        self.arcs[u].iter().copied()
    }
}

impl OutNeighborsWeighted<usize> for AdjacencyList {
    /// # Panics
    ///
    /// Panics if `u` isn't in the digraph.
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

impl Outdegree for AdjacencyList {
    /// # Panics
    ///
    /// Panics if `u` isn't in the digraph.
    fn outdegree(&self, u: usize) -> usize {
        assert!(u < self.order(), "u = {u} isn't in the digraph");

        self.arcs[u].len()
    }
}

impl RemoveArc for AdjacencyList {
    fn remove_arc(&mut self, u: usize, v: usize) -> bool {
        self.arcs.get_mut(u).map_or(false, |set| set.remove(&v))
    }
}

impl Size for AdjacencyList {
    fn size(&self) -> usize {
        self.arcs.iter().map(BTreeSet::len).sum()
    }
}

impl Union for AdjacencyList {
    fn union(&self, other: &Self) -> Self {
        let (mut union, other) = if self.order() > other.order() {
            (self.clone(), other)
        } else {
            (other.clone(), self)
        };

        for (u, v) in other.arcs() {
            union.add_arc(u, v);
        }

        union
    }
}

impl Vertices for AdjacencyList {
    fn vertices(&self) -> impl Iterator<Item = usize> {
        0..self.order()
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::test_unweighted,
    };

    test_unweighted!(AdjacencyList, repr::adjacency_list::fixture);

    #[test]
    #[should_panic(expected = "v = 1 isn't in the digraph")]
    fn add_arc_out_of_bounds_u() {
        AdjacencyList::trivial().add_arc(0, 1);
    }

    #[test]
    #[should_panic(expected = "u = 1 isn't in the digraph")]
    fn add_arc_out_of_bounds_v() {
        AdjacencyList::trivial().add_arc(1, 0);
    }

    #[test]
    fn from_vec() {
        let digraph = AdjacencyList::from(vec![
            BTreeSet::from([1]),
            BTreeSet::from([2]),
            BTreeSet::new(),
        ]);

        assert_eq!(digraph.order(), 3);
        assert!(digraph.arcs().eq([(0, 1), (1, 2)]));
    }

    #[test]
    #[should_panic(expected = "v = 1 isn't in the digraph")]
    fn from_vec_out_of_bounds_v() {
        let vec = vec![BTreeSet::from([1])];
        let _ = AdjacencyList::from(vec);
    }

    #[test]
    #[should_panic(expected = "u = 0 equals v = 0")]
    fn from_vec_u_equals_v() {
        let vec = vec![BTreeSet::from([0])];
        let _ = AdjacencyList::from(vec);
    }

    #[test]
    fn is_simple_false() {
        let digraph = AdjacencyList {
            arcs: vec![BTreeSet::from([0])],
        };

        assert!(!digraph.is_simple());
    }
}

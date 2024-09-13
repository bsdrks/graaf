#![macro_use]
//! A representation of an unweighted digraph.
//!
//! # Example
//!
//! ## Valid digraph
//!
//! A valid digraph of order `5` and size `8`.
//!
//! ![digraph of order `5` and size `8`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/edge_list_1.svg?)
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     Arcs,
//!     EdgeList,
//!     Empty,
//! };
//!
//! let mut digraph = EdgeList::empty(5);
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
//! A self-loop is not allowed. The following pseudograph can not be
//! represented. The self-loop is red:
//!
//! ![self-loop](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/edge_list_self_loop.svg?)
//!
//! Adding a self-loop will panic:
//!
//! ```should_panic
//! use graaf::{
//!     AddArc,
//!     EdgeList,
//!     Empty,
//! };
//!
//! let mut digraph = EdgeList::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(3, 2);
//!
//! // This will panic.
//! digraph.add_arc(2, 2);
//! ```
//!
//! ## Parallel arcs
//!
//! Parallel arcs are not allowed. The following multigraph can not be
//! represented. The parallel arc is red:
//!
//! ![parallel arcs](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/edge_list_parallel_arcs.svg?)
//!
//! Adding a parallel arc does not change the digraph:
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     Arcs,
//!     EdgeList,
//!     Empty,
//! };
//!
//! let mut digraph = EdgeList::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(3, 2);
//!
//! assert!(digraph.arcs().eq([(0, 1), (1, 2), (3, 2)]));
//!
//! // This does not change the digraph.
//! digraph.add_arc(0, 1);
//!
//! assert!(digraph.arcs().eq([(0, 1), (1, 2), (3, 2)]));
//! ```

pub mod fixture;

use {
    crate::{
        AddArc,
        AdjacencyList,
        AdjacencyMatrix,
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
        Union,
        Vertices,
    },
    std::collections::BTreeSet,
};

/// A representation of an unweighted digraph.
///
/// # Example
///
/// ## Valid digraph
///
/// A valid digraph of order `5` and size `8`.
///
/// ![digraph of order `5` and size `8`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/edge_list_1.svg?)
///
/// ```
/// use graaf::{
///     AddArc,
///     Arcs,
///     EdgeList,
///     Empty,
/// };
///
/// let mut digraph = EdgeList::empty(5);
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
/// A self-loop is not allowed. The following pseudograph can not be
/// represented. The self-loop is red:
///
/// ![self-loop](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/edge_list_self_loop.svg?)
///
/// Adding a self-loop will panic:
///
/// ```should_panic
/// use graaf::{
///     AddArc,
///     EdgeList,
///     Empty,
/// };
///
/// let mut digraph = EdgeList::empty(4);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(3, 2);
///
/// // This will panic.
/// digraph.add_arc(2, 2);
/// ```
///
/// ## Parallel arcs
///
/// Parallel arcs are not allowed. The following multigraph can not be
/// represented. The parallel arc is red:
///
/// ![parallel arcs](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/edge_list_parallel_arcs.svg?)
///
/// Adding a parallel arc does not change the digraph:
///
/// ```
/// use graaf::{
///     AddArc,
///     Arcs,
///     EdgeList,
///     Empty,
/// };
///
/// let mut digraph = EdgeList::empty(4);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(3, 2);
///
/// assert!(digraph.arcs().eq([(0, 1), (1, 2), (3, 2)]));
///
/// // This does not change the digraph.
/// digraph.add_arc(0, 1);
///
/// assert!(digraph.arcs().eq([(0, 1), (1, 2), (3, 2)]));
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EdgeList {
    arcs: BTreeSet<(usize, usize)>,
    order: usize,
}

impl AddArc for EdgeList {
    /// # Panics
    ///
    /// * Panics if `u` equals `v`; self-loops are not allowed.
    /// * Panics if `u` is not in the digraph.
    /// * Panics if `v` is not in the digraph.
    fn add_arc(&mut self, u: usize, v: usize) {
        // Self-loops are not allowed.
        assert_ne!(u, v, "u = {u} equals v = {v}");

        assert!(u < self.order, "u = {u} is not in the digraph");
        assert!(v < self.order, "v = {v} is not in the digraph");

        let _ = self.arcs.insert((u, v));
    }
}

impl ArcWeight<usize> for EdgeList {
    fn arc_weight(&self, u: usize, v: usize) -> Option<&usize> {
        self.has_arc(u, v).then_some(&1)
    }
}

impl Arcs for EdgeList {
    fn arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.arcs.iter().copied()
    }
}

impl ArcsWeighted<usize> for EdgeList {
    fn arcs_weighted<'a>(
        &'a self,
    ) -> impl Iterator<Item = (usize, usize, &'a usize)>
    where
        usize: 'a,
    {
        self.arcs.iter().map(|&(u, v)| (u, v, &1))
    }
}

impl Biclique for EdgeList {
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

impl Circuit for EdgeList {
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

impl Converse for EdgeList {
    fn converse(&self) -> Self {
        let mut converse = Self::empty(self.order);

        for &(u, v) in &self.arcs {
            converse.add_arc(v, u);
        }

        converse
    }
}

impl Empty for EdgeList {
    /// # Panics
    ///
    /// * Panics if `order` is zero. A digraph has at least one vertex.
    fn empty(order: usize) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        Self {
            arcs: BTreeSet::new(),
            order,
        }
    }
}

impl<I> From<I> for EdgeList
where
    I: IntoIterator<Item = (usize, usize)>,
{
    fn from(iter: I) -> Self {
        let mut order = 0;
        let mut arcs = BTreeSet::new();

        for (u, v) in iter {
            assert_ne!(u, v, "u = {u} equals v = {v}");

            order = order.max(u).max(v);
            let _ = arcs.insert((u, v));
        }

        Self {
            arcs,
            order: order + 1,
        }
    }
}

impl From<AdjacencyList> for EdgeList {
    fn from(adjacency_list: AdjacencyList) -> Self {
        let order = adjacency_list.order();
        let mut edge_list = Self::empty(order);

        for (u, v) in adjacency_list.arcs() {
            edge_list.add_arc(u, v);
        }

        edge_list
    }
}

impl From<AdjacencyMatrix> for EdgeList {
    fn from(adjacency_matrix: AdjacencyMatrix) -> Self {
        let order = adjacency_matrix.order();
        let mut edge_list = Self::empty(order);

        for (u, v) in adjacency_matrix.arcs() {
            edge_list.add_arc(u, v);
        }

        edge_list
    }
}

impl HasArc for EdgeList {
    fn has_arc(&self, u: usize, v: usize) -> bool {
        self.arcs.contains(&(u, v))
    }
}

impl Indegree for EdgeList {
    /// Warning: runs in **O(a)** time, where **a** is the number of arcs.
    ///
    /// # Panics
    ///
    /// Panics if `v` is not in the digraph.
    fn indegree(&self, v: usize) -> usize {
        assert!(v < self.order, "v = {v} is not in the digraph");

        self.arcs.iter().filter(|(_, y)| v == *y).count()
    }
}

impl IsSimple for EdgeList {
    // We only check for self-loops. Parallel arcs can not exist in this
    // representation.
    fn is_simple(&self) -> bool {
        self.vertices().all(|u| !self.has_arc(u, u))
    }
}

impl Order for EdgeList {
    fn order(&self) -> usize {
        self.order
    }
}

impl OutNeighbors for EdgeList {
    /// Warning: runs in **O(a)** time, where **a** is the number of arcs,
    /// compared to **O(1)** for `adjacency_list` and
    /// `adjacency_list_weighted`.
    ///
    /// # Panics
    ///
    /// Panics if `u` is not in the digraph.
    fn out_neighbors(&self, u: usize) -> impl Iterator<Item = usize> {
        assert!(u < self.order, "u = {u} is not in the digraph");

        self.arcs
            .iter()
            .filter_map(move |&(x, y)| (x == u).then_some(y))
    }
}

impl OutNeighborsWeighted<usize> for EdgeList {
    /// Warning: runs in **O(a)**, where **a** is the number of arcs,
    /// compared to **O(1)** for `adjacency_list` and
    /// `adjacency_list_weighted`.
    ///
    /// # Panics
    ///
    /// Panics if `u` is not in the digraph.
    fn out_neighbors_weighted<'a>(
        &'a self,
        u: usize,
    ) -> impl Iterator<Item = (usize, &'a usize)>
    where
        usize: 'a,
    {
        assert!(u < self.order, "u = {u} is not in the digraph");

        self.arcs
            .iter()
            .filter_map(move |&(x, y)| (x == u).then_some((y, &1)))
    }
}

impl Outdegree for EdgeList {
    /// Warning: runs in **O(a)**, where **a** is the number of arcs, compared
    /// to **O(1)** for `adjacency_list` and `adjacency_list_weighted`.
    ///
    /// # Panics
    ///
    /// Panics if `u` is not in the digraph.
    fn outdegree(&self, u: usize) -> usize {
        assert!(u < self.order, "u = {u} is not in the digraph");

        self.arcs.iter().filter(|&(x, _)| u == *x).count()
    }
}

impl RemoveArc for EdgeList {
    fn remove_arc(&mut self, u: usize, v: usize) -> bool {
        self.arcs.remove(&(u, v))
    }
}

impl Size for EdgeList {
    fn size(&self) -> usize {
        self.arcs.len()
    }
}

impl Union for EdgeList {
    fn union(&self, other: &Self) -> Self {
        let (mut union, other) = if self.order() > other.order() {
            (self.clone(), other)
        } else {
            (other.clone(), self)
        };

        for &(u, v) in &other.arcs {
            union.add_arc(u, v);
        }

        union
    }
}

impl Vertices for EdgeList {
    fn vertices(&self) -> impl Iterator<Item = usize> {
        0..self.order
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::test_unweighted,
    };

    test_unweighted!(EdgeList, repr::edge_list::fixture);

    #[test]
    #[should_panic(expected = "v = 1 is not in the digraph")]
    fn add_arc_out_of_bounds_u() {
        EdgeList::trivial().add_arc(0, 1);
    }

    #[test]
    #[should_panic(expected = "u = 1 is not in the digraph")]
    fn add_arc_out_of_bounds_v() {
        EdgeList::trivial().add_arc(1, 0);
    }

    #[test]
    fn from_btree_set() {
        let arcs = BTreeSet::from([(0, 1), (1, 2)]);
        let digraph = EdgeList::from(arcs);

        assert_eq!(digraph.order(), 3);
        assert!(digraph.arcs().eq([(0, 1), (1, 2)]));
    }

    #[test]
    fn from_adjacency_matrix() {
        let digraph = AdjacencyMatrix::from([(0, 1), (1, 2)]);
        let digraph = EdgeList::from(digraph);

        assert_eq!(digraph.order(), 3);
        assert!(digraph.arcs().eq([(0, 1), (1, 2)]));
    }
}

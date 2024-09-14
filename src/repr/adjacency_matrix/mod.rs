//! A representation of an unweighted digraph.
//!
//! An adjacency matrix is a symmetric binary matrix where a value of `1` at
//! row `u` and column `v` indicates an arc from vertex `u` to vertex `v`. The
//! matrix is stored as a bit vector, and is suited for dense digraphs with a
//! small number of vertices.
//!
//! # Example
//!
//! ## Valid digraph
//!
//! A valid digraph of order `5` and size `8`.
//!
//! ![Digraph of order `5` and size `8`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_matrix_1-0.87.4.svg?)
//!
//! Represented as a matrix.
//!
//! ![The matrix for the above digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_matrix_matrix_1-0.87.4.svg?)
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyMatrix,
//!     Arcs,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyMatrix::empty(5);
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
//! ![Self-loop](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_matrix_self_loop-0.87.4.svg?)
//!
//! Adding a self-loop will panic:
//!
//! ```should_panic
//! use graaf::{
//!     AddArc,
//!     AdjacencyMatrix,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyMatrix::empty(4);
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
//! ![Parallel arcs](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_matrix_parallel_arcs-0.87.4.svg?)
//!
//! Adding a parallel arc does not change the digraph:
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyMatrix,
//!     Arcs,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyMatrix::empty(4);
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

use crate::{
    AddArc,
    AdjacencyList,
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
};

/// A representation of an unweighted digraph.
///
/// An adjacency matrix is a symmetric binary matrix where a value of `1` at
/// row `u` and column `v` indicates an arc from vertex `u` to vertex `v`. The
/// matrix is stored as a bit vector, and is suited for dense digraphs with a
/// small number of vertices.
///
/// # Example
///
/// ## Valid digraph
///
/// A valid digraph of order `5` and size `8`.
///
/// ![Digraph of order `5` and size `8`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_matrix_1-0.87.4.svg?)
///
/// Represented as a matrix.
///
/// ![The matrix for the above digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_matrix_matrix_1-0.87.4.svg?)
///
/// ```
/// use graaf::{
///     AddArc,
///     AdjacencyMatrix,
///     Arcs,
///     Empty,
/// };
///
/// let mut digraph = AdjacencyMatrix::empty(5);
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
/// ![Self-loop](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_matrix_self_loop-0.87.4.svg?)
///
/// Adding a self-loop will panic:
///
/// ```should_panic
/// use graaf::{
///     AddArc,
///     AdjacencyMatrix,
///     Empty,
/// };
///
/// let mut digraph = AdjacencyMatrix::empty(4);
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
/// ![Parallel arcs](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_matrix_parallel_arcs-0.87.4.svg?)
///
/// Adding a parallel arc does not change the digraph:
///
/// ```
/// use graaf::{
///     AddArc,
///     AdjacencyMatrix,
///     Arcs,
///     Empty,
/// };
///
/// let mut digraph = AdjacencyMatrix::empty(4);
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
pub struct AdjacencyMatrix {
    blocks: Vec<usize>,
    order: usize,
}

impl AdjacencyMatrix {
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
    /// * Panics if `u` is not in the digraph.
    /// * Panics if `v` is not in the digraph.
    /// * Panics if `u` equals `v`.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyMatrix,
    ///     Empty,
    ///     HasArc,
    /// };
    ///
    /// let mut digraph = AdjacencyMatrix::empty(3);
    ///
    /// assert!(!digraph.has_arc(0, 1));
    ///
    /// digraph.toggle(0, 1);
    ///
    /// assert!(digraph.has_arc(0, 1));
    /// ```
    pub fn toggle(&mut self, u: usize, v: usize) {
        assert!(u < self.order, "u = {u} is not in the digraph.");
        assert!(v < self.order, "v = {v} is not in the digraph.");
        assert_ne!(u, v, "u = {u} equals v = {v}.");

        let i = self.index(u, v);

        self.blocks[i >> 6] ^= Self::mask(i);
    }
}

impl AddArc for AdjacencyMatrix {
    /// # Panics
    ///
    /// * Panics if `u` equals `v`.
    /// * Panics if `u` is not in the digraph.
    /// * Panics if `v` is not in the digraph.
    fn add_arc(&mut self, u: usize, v: usize) {
        assert_ne!(u, v, "u = {u} equals v = {v}.");
        assert!(u < self.order, "u = {u} is not in the digraph.");
        assert!(v < self.order, "v = {v} is not in the digraph.");

        let i = self.index(u, v);

        self.blocks[i >> 6] |= Self::mask(i);
    }
}

impl ArcWeight<usize> for AdjacencyMatrix {
    fn arc_weight(&self, u: usize, v: usize) -> Option<&usize> {
        self.has_arc(u, v).then_some(&1)
    }
}

impl Arcs for AdjacencyMatrix {
    fn arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.vertices().flat_map(move |u| {
            self.vertices()
                .filter_map(move |v| self.has_arc(u, v).then_some((u, v)))
        })
    }
}

impl ArcsWeighted<usize> for AdjacencyMatrix {
    fn arcs_weighted<'a>(
        &'a self,
    ) -> impl Iterator<Item = (usize, usize, &'a usize)>
    where
        usize: 'a,
    {
        self.arcs().map(|(u, v)| (u, v, &1))
    }
}

impl Biclique for AdjacencyMatrix {
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

impl Circuit for AdjacencyMatrix {
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

impl Converse for AdjacencyMatrix {
    fn converse(&self) -> Self {
        let mut converse = Self::empty(self.order);

        for (u, v) in self.arcs() {
            converse.add_arc(v, u);
        }

        converse
    }
}

impl Empty for AdjacencyMatrix {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn empty(order: usize) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        let n = (order * order + 63) / 64;

        Self {
            blocks: vec![0; n],
            order,
        }
    }
}

impl From<AdjacencyList> for AdjacencyMatrix {
    fn from(d: AdjacencyList) -> Self {
        let order = d.order();
        let mut h = Self::empty(order);

        for (u, v) in d.arcs() {
            h.add_arc(u, v);
        }

        h
    }
}

impl<I> From<I> for AdjacencyMatrix
where
    I: IntoIterator<Item = (usize, usize)>,
{
    /// # Panics
    ///
    /// * Panics if for any arc `u -> v` in `arcs`, `u` equals `v`.
    /// * Panics if for any arc `u -> v` in `arcs`, `v` is not in the digraph.
    fn from(vec: I) -> Self {
        let mut order = 0;
        let mut arcs = Vec::new();

        for (u, v) in vec {
            assert_ne!(u, v, "u = {u} equals v = {v}.");

            order = order.max(u).max(v);

            arcs.push((u, v));
        }

        let mut digraph = Self::empty(order + 1);

        for (u, v) in arcs {
            digraph.add_arc(u, v);
        }

        digraph
    }
}

impl HasArc for AdjacencyMatrix {
    fn has_arc(&self, u: usize, v: usize) -> bool {
        if u >= self.order || v >= self.order {
            return false;
        }

        let i = self.index(u, v);

        self.blocks[i >> 6] & Self::mask(i) != 0
    }
}

impl Indegree for AdjacencyMatrix {
    /// Warning: this implementation runs in **O(v)**.
    ///
    /// # Panics
    ///
    /// Panics if `v` is not in the digraph.
    fn indegree(&self, v: usize) -> usize {
        assert!(v < self.order, "v = {v} is not in the digraph.");

        self.vertices().filter(|&u| self.has_arc(u, v)).count()
    }
}

impl IsSimple for AdjacencyMatrix {
    // We only check for self-loops. Parallel arcs can not exist in this
    // representation.
    fn is_simple(&self) -> bool {
        self.vertices().all(|u| !self.has_arc(u, u))
    }
}

impl Order for AdjacencyMatrix {
    fn order(&self) -> usize {
        self.order
    }
}

impl OutNeighbors for AdjacencyMatrix {
    /// Warning: this implementation runs in **O(v)**.
    ///
    /// # Panics
    ///
    /// Panics if `u` is not in the digraph.
    fn out_neighbors(&self, u: usize) -> impl Iterator<Item = usize> {
        assert!(u < self.order, "u = {u} is not in the digraph.");

        self.vertices().filter(move |&v| self.has_arc(u, v))
    }
}

impl OutNeighborsWeighted<usize> for AdjacencyMatrix {
    /// Warning: this implementation runs in **O(v)**.
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
        self.out_neighbors(u).map(move |v| (v, &1))
    }
}

impl Outdegree for AdjacencyMatrix {
    /// Warning: this implementation runs in **O(v)**.
    ///
    /// # Panics
    ///
    /// Panics if `u` is not in the digraph.
    fn outdegree(&self, u: usize) -> usize {
        assert!(u < self.order, "u = {u} is not in the digraph.");

        self.vertices().filter(|&v| self.has_arc(u, v)).count()
    }
}

impl RemoveArc for AdjacencyMatrix {
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

impl Size for AdjacencyMatrix {
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

impl Union for AdjacencyMatrix {
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

impl Vertices for AdjacencyMatrix {
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

    test_unweighted!(AdjacencyMatrix, repr::adjacency_matrix::fixture);

    #[test]
    #[should_panic(expected = "v = 1 is not in the digraph")]
    fn add_arc_out_of_bounds_u() {
        AdjacencyMatrix::trivial().add_arc(0, 1);
    }

    #[test]
    #[should_panic(expected = "u = 1 is not in the digraph")]
    fn add_arc_out_of_bounds_v() {
        AdjacencyMatrix::trivial().add_arc(1, 0);
    }

    #[test]
    fn from_vec() {
        let digraph = AdjacencyMatrix::from([(0, 1), (1, 2)]);

        assert_eq!(digraph.order(), 3);
        assert!(digraph.arcs().eq([(0, 1), (1, 2)]));
    }

    #[test]
    fn toggle() {
        let mut digraph = AdjacencyMatrix::empty(3);

        digraph.toggle(0, 1);
        digraph.toggle(0, 2);

        assert_eq!(digraph.blocks, [0b110]);
    }

    #[test]
    #[should_panic(expected = "u = 1 is not in the digraph.")]
    fn toggle_out_of_bounds_u() {
        let mut digraph = AdjacencyMatrix::trivial();

        digraph.toggle(1, 0);
    }

    #[test]
    #[should_panic(expected = "v = 1 is not in the digraph.")]
    fn toggle_out_of_bounds_v() {
        let mut digraph = AdjacencyMatrix::trivial();

        digraph.toggle(0, 1);
    }
}

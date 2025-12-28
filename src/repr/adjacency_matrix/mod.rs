//! Represent dense unweighted digraphs.
//!
//! An [`AdjacencyMatrix`] is a vector of 64-bit blocks.
//!
//! An adjacency matrix is a symmetric binary matrix where a value of `1` at
//! row `u` and column `v` indicates an arc from vertex `u` to vertex `v`. The
//! matrix is stored as a bit vector, and is suited for dense digraphs of small
//! order.
//!
//! # Examples
//!
//! ## Valid digraph
//!
//! A valid digraph of order `5` and size `8`.
//!
//! ![A digraph of order `5` and size `8`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_matrix_1-0.87.4.svg?)
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
//! A self-loop isn't allowed. [`AdjacencyMatrix`] can't represent this
//! pseudograph. The self-loop is red.
//!
//! ![A pseudograph with a self-loop](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_matrix_self_loop-0.87.4.svg?)
//!
//! Adding a self-loop panics.
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
//! // This panics.
//! digraph.add_arc(2, 2);
//! ```
//!
//! ## Parallel arcs
//!
//! Parallel arcs aren't allowed. [`AdjacencyMatrix`] can't represent this
//! multigraph. The parallel arc is red.
//!
//! ![A multigraph with a parallel arc](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_matrix_parallel_arcs-0.87.4.svg?)
//!
//! Adding a parallel arc doesn't change the digraph.
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
//! // This doesn't change the digraph.
//! digraph.add_arc(0, 1);
//!
//! assert!(digraph.arcs().eq([(0, 1), (1, 2), (3, 2)]));
//! ```

pub mod fixture;

use crate::{
    AddArc,
    AdjacencyList,
    AdjacencyMap,
    Arcs,
    Biclique,
    Circuit,
    Complement,
    Complete,
    ContiguousOrder,
    Converse,
    Cycle,
    Degree,
    DegreeSequence,
    EdgeList,
    Empty,
    ErdosRenyi,
    HasArc,
    HasEdge,
    HasWalk,
    InNeighbors,
    Indegree,
    IndegreeSequence,
    IsComplete,
    IsRegular,
    IsSemicomplete,
    IsSimple,
    IsTournament,
    Order,
    OutNeighbors,
    Outdegree,
    Path,
    RandomRecursiveTree,
    RandomTournament,
    RemoveArc,
    SemidegreeSequence,
    Size,
    Star,
    Union,
    Vertices,
    Wheel,
    r#gen::prng::Xoshiro256StarStar,
};

/// A representation of an unweighted digraph.
///
/// An adjacency matrix is a symmetric binary matrix where a value of `1` at
/// row `u` and column `v` indicates an arc from vertex `u` to vertex `v`. The
/// matrix is stored as a bit vector, and is suited for dense digraphs of small
/// order.
///
///
/// # Examples
///
/// ## Valid digraph
///
/// A valid digraph of order `5` and size `8`.
///
/// ![A digraph of order `5` and size `8`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_matrix_1-0.87.4.svg?)
///
/// Represented as a matrix.
///
/// ![The matrix representation of the digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_matrix_matrix_1-0.87.4.svg?)
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
/// A self-loop isn't allowed. [`AdjacencyMatrix`] can't represent this
/// pseudograph. The self-loop is red.
///
/// ![A pseudograph with a self-loop](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_matrix_self_loop-0.87.4.svg?)
///
/// Adding a self-loop panics.
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
/// // This panics.
/// digraph.add_arc(2, 2);
/// ```
///
/// ## Parallel arcs
///
/// Parallel arcs aren't allowed. [`AdjacencyMatrix`] can't represent this
/// multigraph. The parallel arc is red.
///
/// ![A multigraph with a parallel arc](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_matrix_parallel_arcs-0.87.4.svg?)
///
/// Adding a parallel arc doesn't change the digraph.
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
/// // This doesn't change the digraph.
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
    #[must_use]
    const fn mask(u: usize) -> usize {
        1 << (u & 63)
    }

    #[must_use]
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
    /// * Panics if `u` isn't in the digraph.
    /// * Panics if `v` isn't in the digraph.
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
        assert_ne!(u, v, "u = {u} equals v = {v}");
        assert!(u < self.order, "u = {u} isn't in the digraph");
        assert!(v < self.order, "v = {v} isn't in the digraph");

        let i = self.index(u, v);

        unsafe { *self.blocks.get_unchecked_mut(i >> 6) ^= Self::mask(i) };
    }
}

impl AddArc for AdjacencyMatrix {
    /// # Panics
    ///
    /// * Panics if `u` equals `v`.
    /// * Panics if `u` isn't in the digraph.
    /// * Panics if `v` isn't in the digraph.
    fn add_arc(&mut self, u: usize, v: usize) {
        assert_ne!(u, v, "u = {u} equals v = {v}");
        assert!(u < self.order, "u = {u} isn't in the digraph");
        assert!(v < self.order, "v = {v} isn't in the digraph");

        let i = self.index(u, v);

        unsafe { *self.blocks.get_unchecked_mut(i >> 6) |= Self::mask(i) };
    }
}

#[derive(Clone, Debug)]
struct ArcsIterator<'a> {
    matrix: &'a AdjacencyMatrix,
    block_index: usize,
    current_bits: usize,
    current_base: usize,
}

impl<'a> ArcsIterator<'a> {
    const fn new(matrix: &'a AdjacencyMatrix) -> Self {
        Self {
            matrix,
            block_index: 0,
            current_bits: 0,
            current_base: 0,
        }
    }
}

impl Iterator for ArcsIterator<'_> {
    type Item = (usize, usize);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.block_index < self.matrix.blocks.len()
            || self.current_bits != 0
        {
            if self.current_bits == 0 {
                unsafe {
                    self.current_bits =
                        *self.matrix.blocks.get_unchecked(self.block_index);
                }

                self.current_base = self.block_index * 64;
                self.block_index += 1;
            }

            if self.current_bits != 0 {
                let bit = self.current_bits.trailing_zeros() as usize;

                self.current_bits &= self.current_bits - 1;

                let cell_index = self.current_base + bit;

                if cell_index < self.matrix.order * self.matrix.order {
                    let u = cell_index / self.matrix.order;
                    let v = cell_index % self.matrix.order;

                    return Some((u, v));
                }
            }
        }

        None
    }
}

impl Arcs for AdjacencyMatrix {
    fn arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        ArcsIterator::new(self)
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
        if order == 1 {
            return Self::trivial();
        }

        let mut digraph = Self::empty(order);

        for u in 0..order - 1 {
            digraph.add_arc(u, u + 1);
        }

        digraph.add_arc(order - 1, 0);

        digraph
    }
}

impl Complement for AdjacencyMatrix {
    fn complement(&self) -> Self {
        let order = self.order();
        let mut digraph = Self::empty(order);

        for u in 0..order {
            for v in u + 1..order {
                if !self.has_arc(u, v) {
                    digraph.add_arc(u, v);
                }

                if !self.has_arc(v, u) {
                    digraph.add_arc(v, u);
                }
            }
        }

        digraph
    }
}

impl Complete for AdjacencyMatrix {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn complete(order: usize) -> Self {
        if order == 1 {
            return Self::trivial();
        }

        let mut digraph = Self::empty(order);

        for u in 0..order {
            for v in (u + 1)..order {
                digraph.add_arc(u, v);
                digraph.add_arc(v, u);
            }
        }

        digraph
    }
}

impl ContiguousOrder for AdjacencyMatrix {
    fn contiguous_order(&self) -> usize {
        self.order
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

impl Cycle for AdjacencyMatrix {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn cycle(order: usize) -> Self {
        if order == 1 {
            return Self::trivial();
        }

        let mut digraph = Self::empty(order);

        for u in 0..order - 1 {
            let v = u + 1;

            digraph.add_arc(u, v);
            digraph.add_arc(v, u);
        }

        let u = order - 1;

        digraph.add_arc(u, 0);
        digraph.add_arc(0, u);

        digraph
    }
}

impl DegreeSequence for AdjacencyMatrix {
    fn degree_sequence(&self) -> impl Iterator<Item = usize> {
        self.vertices().map(move |v| self.degree(v))
    }
}

impl Empty for AdjacencyMatrix {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn empty(order: usize) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        let n = (order * order).div_ceil(64);

        Self {
            blocks: vec![0; n],
            order,
        }
    }
}

impl ErdosRenyi for AdjacencyMatrix {
    /// # Panics
    ///
    /// * Panics if `order` is zero.
    /// * Panics if `p` isn't in `[0, 1]`.
    fn erdos_renyi(order: usize, p: f64, seed: u64) -> Self {
        assert!((0.0..=1.0).contains(&p), "p = {p} must be in [0, 1]");

        if order == 1 {
            return Self::trivial();
        }

        let mut digraph = Self::empty(order);
        let mut rng = Xoshiro256StarStar::new(seed);

        for u in 0..order {
            for v in (0..order).filter(|&v| u != v) {
                if rng.next_f64() < p {
                    digraph.add_arc(u, v);
                }
            }
        }

        digraph
    }
}

macro_rules! impl_from_arcs_empty_order {
    ($type:ty) => {
        /// # Panics
        ///
        /// * Panics if `digraph` is empty.
        /// * Panics if, for any arc `u -> v` in `digraph`, `u` equals `v`.
        /// * Panics if, for any arc `u -> v` in `digraph`, `v` isn't in the
        ///   digraph.
        impl From<$type> for AdjacencyMatrix {
            fn from(digraph: $type) -> Self {
                let order = digraph.order();

                assert!(order > 0, "a digraph has at least one vertex");

                let mut h = Self::empty(order);

                for (u, v) in digraph.arcs() {
                    assert_ne!(u, v, "u = {u} equals v = {v}");
                    assert!(v < order, "v = {v} isn't in the digraph");

                    h.add_arc(u, v);
                }

                h
            }
        }
    };
}

impl_from_arcs_empty_order!(AdjacencyList);
impl_from_arcs_empty_order!(AdjacencyMap);
impl_from_arcs_empty_order!(EdgeList);

impl<I> From<I> for AdjacencyMatrix
where
    I: IntoIterator<Item = (usize, usize)>,
{
    /// # Panics
    ///
    /// * Panics if `iter` is empty.
    /// * Panics if for any arc `u -> v` in `iter`, `u` equals `v`.
    fn from(iter: I) -> Self {
        let mut order = 0;
        let mut arcs = Vec::new();

        for (u, v) in iter {
            assert_ne!(u, v, "u = {u} equals v = {v}");

            order = order.max(u).max(v);

            arcs.push((u, v));
        }

        order += 1;

        assert!(!arcs.is_empty(), "a digraph has at least one vertex");

        let mut digraph = Self::empty(order);

        for (u, v) in arcs {
            digraph.add_arc(u, v);
        }

        digraph
    }
}

impl RandomRecursiveTree for AdjacencyMatrix {
    /// # Panics
    ///
    /// * Panics if `order` is zero.
    /// * Panics if conversion from `u64` to `usize` fails.
    fn random_recursive_tree(order: usize, seed: u64) -> Self {
        if order == 1 {
            return Self::trivial();
        }

        let mut digraph = Self::empty(order);
        let rng = Xoshiro256StarStar::new(seed);

        for (u, v) in (1..order).zip(rng) {
            digraph.add_arc(
                u,
                usize::try_from(v).expect("conversion failed") % u,
            );
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

impl HasEdge for AdjacencyMatrix {
    fn has_edge(&self, u: usize, v: usize) -> bool {
        self.has_arc(u, v) && self.has_arc(v, u)
    }
}

impl HasWalk for AdjacencyMatrix {
    fn has_walk(&self, walk: &[usize]) -> bool {
        walk.len() > 1
            && walk
                .iter()
                .zip(walk.iter().skip(1))
                .all(|(&u, &v)| self.has_arc(u, v))
    }
}

impl Indegree for AdjacencyMatrix {
    /// # Complexity
    ///
    /// The time complexity of this implementation is `O(v)`.
    ///
    /// # Panics
    ///
    /// Panics if `v` isn't in the digraph.
    fn indegree(&self, v: usize) -> usize {
        assert!(v < self.order, "v = {v} isn't in the digraph.");

        self.vertices().filter(|&u| self.has_arc(u, v)).count()
    }

    fn is_source(&self, v: usize) -> bool {
        self.vertices().all(|u| !self.has_arc(u, v))
    }
}

impl IndegreeSequence for AdjacencyMatrix {
    fn indegree_sequence(&self) -> impl Iterator<Item = usize> {
        self.vertices().map(move |v| self.indegree(v))
    }
}

impl InNeighbors for AdjacencyMatrix {
    fn in_neighbors(&self, v: usize) -> impl Iterator<Item = usize> {
        self.arcs().filter_map(move |(x, y)| (v == y).then_some(x))
    }
}

impl IsComplete for AdjacencyMatrix {
    fn is_complete(&self) -> bool {
        *self == Self::complete(self.order())
    }
}

impl IsRegular for AdjacencyMatrix {
    fn is_regular(&self) -> bool {
        let mut semidegrees = self.semidegree_sequence();

        let (u, v) = semidegrees
            .next()
            .expect("a digraph has at least one vertex");

        u == v && semidegrees.all(|(x, y)| x == u && y == v)
    }
}

impl IsSemicomplete for AdjacencyMatrix {
    fn is_semicomplete(&self) -> bool {
        let order = self.order();

        self.size() >= order * (order - 1) / 2
            && (0..order).all(|u| {
                (u + 1..order)
                    .all(|v| self.has_arc(u, v) || self.has_arc(v, u))
            })
    }
}

impl IsSimple for AdjacencyMatrix {
    // We only check for self-loops. Parallel arcs can't exist in this
    // representation.
    fn is_simple(&self) -> bool {
        self.vertices().all(|u| !self.has_arc(u, u))
    }
}

impl IsTournament for AdjacencyMatrix {
    fn is_tournament(&self) -> bool {
        let order = self.order();

        if self.size() != order * (order - 1) / 2 {
            return false;
        }

        (0..order).all(|u| {
            (u + 1..order).all(|v| self.has_arc(u, v) ^ self.has_arc(v, u))
        })
    }
}

impl Order for AdjacencyMatrix {
    fn order(&self) -> usize {
        self.order
    }
}

impl OutNeighbors for AdjacencyMatrix {
    /// Warning: The time complexity of this implementation is `O(v)`,
    /// compared to `O(log v + outdegree)` for `AdjacencyList`, where `v` is
    /// the digraph's order and `outdegree` is the outdegree of `u`.
    ///
    /// # Panics
    ///
    /// Panics if `u` isn't in the digraph.
    fn out_neighbors(&self, u: usize) -> impl Iterator<Item = usize> {
        assert!(u < self.order, "u = {u} isn't in the digraph.");

        self.vertices().filter(move |&v| self.has_arc(u, v))
    }
}

impl Outdegree for AdjacencyMatrix {
    /// Warning: The time complexity of this implementation is `O(v)`, where
    /// `v` is the digraph's order, compared to `O(1)` for `AdjacencyList`.
    ///
    /// # Panics
    ///
    /// Panics if `u` isn't in the digraph.
    fn outdegree(&self, u: usize) -> usize {
        assert!(u < self.order, "u = {u} isn't in the digraph.");

        self.vertices().filter(|&v| self.has_arc(u, v)).count()
    }

    fn is_sink(&self, u: usize) -> bool {
        assert!(u < self.order, "u = {u} isn't in the digraph");

        self.vertices().all(|v| !self.has_arc(u, v))
    }
}

impl Path for AdjacencyMatrix {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn path(order: usize) -> Self {
        let mut digraph = Self::empty(order);

        if order == 1 {
            return digraph;
        }

        for u in 0..order - 1 {
            digraph.add_arc(u, u + 1);
        }

        digraph
    }
}

impl RandomTournament for AdjacencyMatrix {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn random_tournament(order: usize, seed: u64) -> Self {
        let mut digraph = Self::empty(order);
        let mut rng = Xoshiro256StarStar::new(seed);

        for u in 0..order {
            for v in (u + 1)..order {
                if rng.next_bool() {
                    digraph.add_arc(u, v);
                } else {
                    digraph.add_arc(v, u);
                }
            }
        }

        digraph
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
    fn size(&self) -> usize {
        self.blocks
            .iter()
            .map(|&block| block.count_ones() as usize)
            .sum()
    }
}

impl Star for AdjacencyMatrix {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn star(order: usize) -> Self {
        if order == 1 {
            return Self::trivial();
        }

        let mut digraph = Self::empty(order);

        for u in 1..order {
            digraph.add_arc(u, 0);
            digraph.add_arc(0, u);
        }

        digraph
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

impl Wheel for AdjacencyMatrix {
    /// # Panics
    ///
    /// Panics if `order` is less than `4`.
    fn wheel(order: usize) -> Self {
        assert!(order >= 4, "a wheel digraph has at least four vertices");

        let mut digraph = Self::empty(order);

        for u in 1..order - 1 {
            let v = u + 1;

            digraph.add_arc(u, v);
            digraph.add_arc(v, u);
        }

        let u = order - 1;

        digraph.add_arc(u, 1);
        digraph.add_arc(1, u);

        for u in 1..order {
            digraph.add_arc(0, u);
            digraph.add_arc(u, 0);
        }

        digraph
    }
}

#[cfg(test)]
mod tests_add_arc_self_loop {
    use {
        super::*,
        crate::test_add_arc_self_loop,
    };

    test_add_arc_self_loop!(AdjacencyMatrix);
}

#[cfg(test)]
mod tests_add_arc_out_of_bounds {
    use {
        super::*,
        crate::test_add_arc_out_of_bounds,
    };

    test_add_arc_out_of_bounds!(AdjacencyMatrix);
}

#[cfg(test)]
mod tests_arcs {
    use {
        super::*,
        crate::test_arcs,
    };

    test_arcs!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_biclique {
    use {
        super::*,
        crate::test_biclique,
    };

    test_biclique!(AdjacencyMatrix);
}

#[cfg(test)]
mod tests_circuit {
    use {
        super::*,
        crate::test_circuit,
    };

    test_circuit!(AdjacencyMatrix);
}

#[cfg(test)]
mod tests_complete {
    use {
        super::*,
        crate::test_complete,
    };

    test_complete!(AdjacencyMatrix);
}

#[cfg(test)]
mod tests_converse {
    use {
        super::*,
        crate::test_converse,
    };

    test_converse!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_cycle {
    use {
        super::*,
        crate::test_cycle,
    };

    test_cycle!(AdjacencyMatrix);
}

#[cfg(test)]
mod tests_degree {
    use {
        super::*,
        crate::test_degree,
    };

    test_degree!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_degree_sequence {
    use {
        super::*,
        crate::test_degree_sequence,
    };

    test_degree_sequence!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_empty {
    use {
        super::*,
        crate::test_empty,
    };

    test_empty!(AdjacencyMatrix);
}

#[cfg(test)]
mod tests_erdos_renyi {
    use {
        super::*,
        crate::test_erdos_renyi,
    };

    test_erdos_renyi!(AdjacencyMatrix);
}

#[cfg(test)]
mod tests_has_walk {
    use {
        super::*,
        crate::test_has_walk,
    };

    test_has_walk!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_in_neighbors {
    use {
        super::*,
        crate::test_in_neighbors,
    };

    test_in_neighbors!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_indegree {
    use {
        super::*,
        crate::test_indegree,
    };

    test_indegree!(AdjacencyMatrix, crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_indegree_sequence {
    use {
        super::*,
        crate::test_indegree_sequence,
    };

    test_indegree_sequence!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_is_balanced {
    use crate::{
        IsBalanced,
        test_is_balanced,
    };

    test_is_balanced!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_is_complete {
    use {
        super::*,
        crate::test_is_complete,
    };

    test_is_complete!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_is_isolated {
    use crate::{
        IsIsolated,
        test_is_isolated,
    };

    test_is_isolated!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_is_oriented {
    use crate::{
        IsOriented,
        test_is_oriented,
    };

    test_is_oriented!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_is_pendant {
    use crate::{
        IsPendant,
        test_is_pendant,
    };

    test_is_pendant!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_is_regular {
    use {
        super::*,
        crate::test_is_regular,
    };

    test_is_regular!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_is_semicomplete {
    use {
        super::*,
        crate::test_is_semicomplete,
    };

    test_is_semicomplete!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_is_simple {
    use {
        super::*,
        crate::test_is_simple,
    };

    test_is_simple!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_is_symmetric {
    use crate::{
        IsSymmetric,
        test_is_symmetric,
    };

    test_is_symmetric!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_is_tournament {
    use {
        super::*,
        crate::test_is_tournament,
    };

    test_is_tournament!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_order {
    use crate::{
        Order,
        test_order,
    };

    test_order!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_out_neighbors {
    use {
        super::*,
        crate::test_out_neighbors,
    };

    test_out_neighbors!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_outdegree {
    use {
        super::*,
        crate::test_outdegree,
    };

    test_outdegree!(AdjacencyMatrix, crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_path {
    use {
        super::*,
        crate::test_path,
    };

    test_path!(AdjacencyMatrix);
}

#[cfg(test)]
mod tests_random_recursive_tree {
    use {
        super::*,
        crate::test_random_recursive_tree,
    };

    test_random_recursive_tree!(AdjacencyMatrix);
}

#[cfg(test)]
mod tests_random_tournament {
    use {
        super::*,
        crate::test_random_tournament,
    };

    test_random_tournament!(AdjacencyMatrix);
}

#[cfg(test)]
mod tests_remove_arc {
    use {
        super::*,
        crate::test_remove_arc,
    };

    test_remove_arc!(AdjacencyMatrix, crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_semidegree_sequence {
    use {
        super::*,
        crate::test_semidegree_sequence,
    };

    test_semidegree_sequence!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_sinks {
    use crate::{
        Sinks,
        test_sinks,
    };

    test_sinks!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_size {
    use crate::{
        Size,
        test_size,
    };

    test_size!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_sources {
    use crate::{
        Sources,
        test_sources,
    };

    test_sources!(crate::repr::adjacency_matrix::fixture);
}

#[cfg(test)]
mod tests_star {
    use {
        super::*,
        crate::test_star,
    };

    test_star!(AdjacencyMatrix);
}

#[cfg(test)]
mod tests_wheel {
    use {
        super::*,
        crate::test_wheel,
    };

    test_wheel!(AdjacencyMatrix);
}

#[cfg(test)]
mod proptests_add_arc {
    use {
        super::*,
        crate::proptest_add_arc,
    };

    proptest_add_arc!(AdjacencyMatrix);
}

#[cfg(test)]
mod proptests_biclique {
    use {
        super::*,
        crate::proptest_biclique,
    };

    proptest_biclique!(AdjacencyMatrix);
}

#[cfg(test)]
mod proptests_circuit {
    use {
        super::*,
        crate::proptest_circuit,
    };

    proptest_circuit!(AdjacencyMatrix);
}

#[cfg(test)]
mod proptests_complete {
    use {
        super::*,
        crate::proptest_complete,
    };

    proptest_complete!(AdjacencyMatrix);
}

#[cfg(test)]
mod proptests_contiguous_order {
    use {
        super::*,
        crate::{
            ContiguousOrder,
            proptest_contiguous_order,
        },
    };

    proptest_contiguous_order!(AdjacencyMatrix);
}

#[cfg(test)]
mod proptests_cycle {
    use {
        super::*,
        crate::proptest_cycle,
    };

    proptest_cycle!(AdjacencyMatrix);
}

#[cfg(test)]
mod proptests_empty {
    use {
        super::*,
        crate::proptest_empty,
    };

    proptest_empty!(AdjacencyMatrix);
}

#[cfg(test)]
mod proptests_empty_complement {
    use {
        super::*,
        crate::proptest_empty_complement,
    };

    proptest_empty_complement!(AdjacencyMatrix);
}

#[cfg(test)]
mod proptests_empty_complete {
    use {
        super::*,
        crate::proptest_empty_complete,
    };

    proptest_empty_complete!(AdjacencyMatrix);
}

#[cfg(test)]
mod proptests_erdos_renyi {
    use {
        super::*,
        crate::proptest_erdos_renyi,
    };

    proptest_erdos_renyi!(AdjacencyMatrix);
}

#[cfg(test)]
mod proptests_has_arc {
    use {
        super::*,
        crate::proptest_has_arc,
    };

    proptest_has_arc!(AdjacencyMatrix);
}

#[cfg(test)]
mod proptests_path {
    use {
        super::*,
        crate::proptest_path,
    };

    proptest_path!(AdjacencyMatrix);
}

#[cfg(test)]
mod proptests_random_recursive_tree {
    use {
        super::*,
        crate::proptest_random_recursive_tree,
    };

    proptest_random_recursive_tree!(AdjacencyMatrix);
}

#[cfg(test)]
mod proptests_random_tournament {
    use {
        super::*,
        crate::proptest_random_tournament,
    };

    proptest_random_tournament!(AdjacencyMatrix);
}

#[cfg(test)]
mod proptests_star {
    use {
        super::*,
        crate::proptest_star,
    };

    proptest_star!(AdjacencyMatrix);
}

#[cfg(test)]
mod proptests_union {
    use {
        super::*,
        crate::proptest_union,
    };

    proptest_union!(AdjacencyMatrix);
}

#[cfg(test)]
mod proptests_wheel {
    use {
        super::*,
        crate::proptest_wheel,
    };

    proptest_wheel!(AdjacencyMatrix);
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::collections::BTreeSet,
    };

    #[test]
    #[should_panic(expected = "v = 1 isn't in the digraph")]
    fn add_arc_out_of_bounds_u() {
        AdjacencyMatrix::trivial().add_arc(0, 1);
    }

    #[test]
    #[should_panic(expected = "u = 1 isn't in the digraph")]
    fn add_arc_out_of_bounds_v() {
        AdjacencyMatrix::trivial().add_arc(1, 0);
    }

    #[test]
    fn from_adjacency_list() {
        let digraph = AdjacencyList::from([
            BTreeSet::from([1]),
            BTreeSet::from([2]),
            BTreeSet::new(),
        ]);

        let digraph = AdjacencyMatrix::from(digraph);

        assert_eq!(digraph.order(), 3);
        assert!(digraph.arcs().eq([(0, 1), (1, 2)]));
    }

    #[test]
    fn from_adjacency_map() {
        let digraph = AdjacencyMap::from([
            BTreeSet::from([1]),
            BTreeSet::from([2]),
            BTreeSet::new(),
        ]);

        let digraph = AdjacencyMatrix::from(digraph);

        assert_eq!(digraph.order(), 3);
        assert!(digraph.arcs().eq([(0, 1), (1, 2)]));
    }

    #[test]
    fn from_edge_list() {
        let digraph = EdgeList::from([(0, 1), (1, 2)]);
        let digraph = AdjacencyMatrix::from(digraph);

        assert_eq!(digraph.order(), 3);
        assert!(digraph.arcs().eq([(0, 1), (1, 2)]));
    }

    #[test]
    fn from_iter() {
        let digraph = AdjacencyMatrix::from([(0, 1), (1, 2)]);

        assert_eq!(digraph.order(), 3);
        assert!(digraph.arcs().eq([(0, 1), (1, 2)]));
    }

    #[test]
    #[should_panic(expected = "a digraph has at least one vertex")]
    fn from_iter_empty() {
        let _ = AdjacencyMatrix::from(Vec::<(usize, usize)>::new());
    }

    #[test]
    #[should_panic(expected = "u = 1 equals v = 1")]
    fn from_iter_self_loop() {
        let _ = AdjacencyMatrix::from([(0, 1), (1, 1)]);
    }

    #[test]
    fn toggle() {
        let mut digraph = AdjacencyMatrix::empty(5);

        digraph.toggle(0, 1);
        digraph.toggle(0, 2);
        digraph.toggle(3, 1);
        digraph.toggle(3, 4);

        assert_eq!(digraph.blocks, [0b00000_10010_00000_00000_00110]);
    }

    #[test]
    #[should_panic(expected = "u = 1 isn't in the digraph")]
    fn toggle_out_of_bounds_u() {
        let mut digraph = AdjacencyMatrix::trivial();

        digraph.toggle(1, 0);
    }

    #[test]
    #[should_panic(expected = "v = 1 isn't in the digraph")]
    fn toggle_out_of_bounds_v() {
        let mut digraph = AdjacencyMatrix::trivial();

        digraph.toggle(0, 1);
    }

    #[test]
    fn is_simple_self_loop() {
        let digraph = AdjacencyMatrix {
            blocks: vec![0b1],
            order: 1,
        };

        assert!(!digraph.is_simple());
    }
}

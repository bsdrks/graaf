//! Represent sparse unweighted digraphs.
//!
//! An [`EdgeList`] is a vector of tuples.
//!
//! # Example
//!
//! ## Valid digraph
//!
//! A valid digraph of order `5` and size `8`.
//!
//! ![A digraph of order `5` and size `8`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/edge_list_1.svg?)
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
//! A self-loop isn't allowed. [`EdgeList`] can't represent this pseudograph.
//! The self-loop is red.
//!
//! ![A pseudograph with a self-loop](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/edge_list_self_loop.svg?)
//!
//! Adding a self-loop panics.
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
//! // This panics.
//! digraph.add_arc(2, 2);
//! ```
//!
//! ## Parallel arcs
//!
//! Parallel arcs aren't allowed. [`EdgeList`] can't represent this multigraph.
//! The parallel arc is red.
//!
//! ![A multigraph with a parallel arc](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/edge_list_parallel_arcs.svg?)
//!
//! Adding a parallel arc doesn't change the digraph.
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
//! // This doesn't change the digraph.
//! digraph.add_arc(0, 1);
//!
//! assert!(digraph.arcs().eq([(0, 1), (1, 2), (3, 2)]));
//! ```

pub mod fixture;

use {
    crate::{
        gen::prng::Xoshiro256StarStar,
        AddArc,
        AdjacencyList,
        AdjacencyMap,
        AdjacencyMatrix,
        ArcWeight,
        Arcs,
        ArcsWeighted,
        Biclique,
        Circuit,
        Complement,
        Complete,
        Converse,
        Cycle,
        Empty,
        ErdosRenyi,
        GrowingNetwork,
        HasArc,
        InNeighbors,
        Indegree,
        IsComplete,
        IsRegular,
        IsSemicomplete,
        IsSimple,
        IsTournament,
        Order,
        OutNeighbors,
        OutNeighborsWeighted,
        Outdegree,
        Path,
        RandomTournament,
        RemoveArc,
        SemidegreeSequence,
        Size,
        Star,
        Union,
        Vertices,
        Wheel,
    },
    std::{
        collections::BTreeSet,
        iter::once,
    },
};

/// A representation of an unweighted digraph.
///
/// # Example
///
/// ## Valid digraph
///
/// A valid digraph of order `5` and size `8`.
///
/// ![A digraph of order `5` and size `8`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/edge_list_1.svg?)
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
/// A self-loop isn't allowed. [`EdgeList`] can't represent this pseudograph.
/// The self-loop is red.
///
/// ![A pseudograph with a self-loop](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/edge_list_self_loop.svg?)
///
/// Adding a self-loop panics.
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
/// // This panics.
/// digraph.add_arc(2, 2);
/// ```
///
/// ## Parallel arcs
///
/// Parallel arcs aren't allowed. [`EdgeList`] can't represent this multigraph.
/// The parallel arc is red.
///
/// ![A multigraph with a parallel arc](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/edge_list_parallel_arcs.svg?)
///
/// Adding a parallel arc doesn't change the digraph.
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
/// // This doesn't change the digraph.
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
    /// * Panics if `u` equals `v`; self-loops aren't allowed.
    /// * Panics if `u` isn't in the digraph.
    /// * Panics if `v` isn't in the digraph.
    fn add_arc(&mut self, u: usize, v: usize) {
        // Self-loops aren't allowed.
        assert_ne!(u, v, "u = {u} equals v = {v}");

        assert!(u < self.order, "u = {u} isn't in the digraph");
        assert!(v < self.order, "v = {v} isn't in the digraph");

        let _ = self.arcs.insert((u, v));
    }
}

impl ArcWeight<usize> for EdgeList {
    type Weight = usize;

    fn arc_weight(&self, u: usize, v: usize) -> Option<&Self::Weight> {
        self.has_arc(u, v).then_some(&1)
    }
}

impl Arcs for EdgeList {
    fn arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.arcs.iter().copied()
    }
}

impl ArcsWeighted for EdgeList {
    type Weight = usize;

    fn arcs_weighted(
        &self,
    ) -> impl Iterator<Item = (usize, usize, &Self::Weight)> {
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

        Self {
            arcs: (0..m)
                .flat_map(|u| (m..order).map(move |v| (u, v)))
                .chain((m..order).flat_map(|u| (0..m).map(move |v| (u, v))))
                .collect(),
            order,
        }
    }
}

impl Circuit for EdgeList {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn circuit(order: usize) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        if order == 1 {
            return Self::trivial();
        }

        Self {
            arcs: (0..order).map(|u| (u, (u + 1) % order)).collect(),
            order,
        }
    }
}

impl Complement for EdgeList {
    fn complement(&self) -> Self {
        let order = self.order();

        Self {
            arcs: (0..order)
                .flat_map(|u| (0..u).chain(u + 1..order).map(move |v| (u, v)))
                .collect::<BTreeSet<(usize, usize)>>()
                .difference(&self.arcs)
                .copied()
                .collect(),
            order,
        }
    }
}

impl Complete for EdgeList {
    /// # Panics
    ///
    /// * Panics if `order` is zero.
    fn complete(order: usize) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        if order == 1 {
            return Self::trivial();
        }

        Self {
            arcs: (0..order)
                .flat_map(|u| {
                    (0..u).chain((u + 1)..order).map(move |v| (u, v))
                })
                .collect(),
            order,
        }
    }
}

impl Converse for EdgeList {
    fn converse(&self) -> Self {
        Self {
            arcs: self.arcs.iter().map(|&(u, v)| (v, u)).collect(),
            order: self.order,
        }
    }
}

impl Cycle for EdgeList {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn cycle(order: usize) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        if order == 1 {
            return Self::trivial();
        }

        Self {
            arcs: (0..order)
                .flat_map(|u| {
                    once((u + order - 1) % order)
                        .chain(once((u + 1) % order))
                        .map(move |v| (u, v))
                })
                .collect(),
            order,
        }
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

impl ErdosRenyi for EdgeList {
    /// # Panics
    ///
    /// * Panics if `order` is zero.
    /// * Panics if `p` isn't in `[0, 1]`.
    fn erdos_renyi(order: usize, p: f64, seed: u64) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");
        assert!((0.0..=1.0).contains(&p), "p = {p} must be in [0, 1]");

        let mut rng = Xoshiro256StarStar::new(seed);

        Self {
            arcs: (0..order)
                .flat_map(|u| {
                    (0..u)
                        .chain((u + 1)..order)
                        .filter(|_| rng.next_f64() < p)
                        .map(|v| (u, v))
                        .collect::<Vec<_>>()
                })
                .collect(),
            order,
        }
    }
}

macro_rules! impl_from_arcs_order {
    ($type:ty) => {
        /// # Panics
        ///
        /// * Panics if `digraph` is empty.
        /// * Panics if, for any arc `u -> v` in `digraph`, `u` equals `v`.
        /// * Panics if, for any arc `u -> v` in `digraph`, `v` isn't in the
        ///   digraph.
        impl From<$type> for EdgeList {
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

impl_from_arcs_order!(AdjacencyList);
impl_from_arcs_order!(AdjacencyMap);
impl_from_arcs_order!(AdjacencyMatrix);

impl<I> From<I> for EdgeList
where
    I: IntoIterator<Item = (usize, usize)>,
{
    /// # Panics
    ///
    /// * Panics if, for any arc `u -> v` in `iter`, `u` equals `v`.
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

impl GrowingNetwork for EdgeList {
    /// # Panics
    ///
    /// * Panics if `order` is zero.
    fn growing_network(order: usize, seed: u64) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        if order == 1 {
            return Self::trivial();
        }

        let mut rng = Xoshiro256StarStar::new(seed);

        Self {
            arcs: (1..order)
                .map(|u| {
                    (
                        u,
                        usize::try_from(rng.next().unwrap())
                            .expect("conversion failed")
                            % u,
                    )
                })
                .collect(),
            order,
        }
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
    /// Panics if `v` isn't in the digraph.
    fn indegree(&self, v: usize) -> usize {
        assert!(v < self.order, "v = {v} isn't in the digraph");

        self.arcs.iter().filter(|(_, y)| v == *y).count()
    }

    fn is_source(&self, v: usize) -> bool {
        self.arcs.iter().all(|&(_, y)| y != v)
    }
}

impl InNeighbors for EdgeList {
    fn in_neighbors(&self, v: usize) -> impl Iterator<Item = usize> {
        self.arcs().filter_map(move |(x, y)| (v == y).then_some(x))
    }
}

impl IsComplete for EdgeList {
    fn is_complete(&self) -> bool {
        *self == Self::complete(self.order())
    }
}

impl IsRegular for EdgeList {
    fn is_regular(&self) -> bool {
        let mut semidegrees = self.semidegree_sequence();

        let (u, v) = semidegrees
            .next()
            .expect("a digraph has at least one vertex");

        u == v && semidegrees.all(|(x, y)| x == u && y == v)
    }
}

impl IsSemicomplete for EdgeList {
    fn is_semicomplete(&self) -> bool {
        let order = self.order();

        self.size() >= order * (order - 1) / 2
            && (0..order).all(|u| {
                (u + 1..order)
                    .all(|v| self.has_arc(u, v) || self.has_arc(v, u))
            })
    }
}

impl IsSimple for EdgeList {
    // We only check for self-loops. [`EdgeList`] can't represent parallel
    // arcs.
    fn is_simple(&self) -> bool {
        self.vertices().all(|u| !self.has_arc(u, u))
    }
}

impl IsTournament for EdgeList {
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

impl Order for EdgeList {
    fn order(&self) -> usize {
        self.order
    }
}

impl OutNeighbors for EdgeList {
    /// Warning: runs in **O(a)** time, where **a** is the number of arcs,
    /// compared to **O(1)** for `AdjacencyList` and `AdjacencyListWeighted`.
    ///
    /// # Panics
    ///
    /// Panics if `u` isn't in the digraph.
    fn out_neighbors(&self, u: usize) -> impl Iterator<Item = usize> {
        assert!(u < self.order, "u = {u} isn't in the digraph");

        self.arcs
            .iter()
            .filter_map(move |&(x, y)| (x == u).then_some(y))
    }
}

impl OutNeighborsWeighted for EdgeList {
    type Weight = usize;

    /// Warning: runs in **O(a)**, where **a** is the number of arcs,
    /// compared to **O(1)** for `AdjacencyList` and `AdjacencyListWeighted`.
    ///
    /// # Panics
    ///
    /// Panics if `u` isn't in the digraph.
    fn out_neighbors_weighted(
        &self,
        u: usize,
    ) -> impl Iterator<Item = (usize, &Self::Weight)> {
        assert!(u < self.order, "u = {u} isn't in the digraph");

        self.arcs
            .iter()
            .filter_map(move |&(x, y)| (x == u).then_some((y, &1)))
    }
}

impl Outdegree for EdgeList {
    /// Warning: runs in **O(a)**, where **a** is the number of arcs, compared
    /// to **O(1)** for `AdjacencyList` and `AdjacencyListWeighted`.
    ///
    /// # Panics
    ///
    /// Panics if `u` isn't in the digraph.
    fn outdegree(&self, u: usize) -> usize {
        assert!(u < self.order, "u = {u} isn't in the digraph");

        self.arcs.iter().filter(|&(x, _)| u == *x).count()
    }

    fn is_sink(&self, u: usize) -> bool {
        self.arcs.iter().all(|&(x, _)| x != u)
    }
}

impl Path for EdgeList {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn path(order: usize) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        if order == 1 {
            return Self::trivial();
        }

        Self {
            arcs: (0..order - 1).map(|u| (u, u + 1)).collect(),
            order,
        }
    }
}

impl RandomTournament for EdgeList {
    /// # Panics
    ///
    /// * Panics if `order` is zero.
    fn random_tournament(order: usize, seed: u64) -> Self {
        if order == 1 {
            return Self::trivial();
        }

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

impl Star for EdgeList {
    /// # Panics
    ///
    /// * Panics if `order` is zero.
    fn star(order: usize) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        if order == 1 {
            return Self::trivial();
        }

        Self {
            arcs: (1..order)
                .map(|v| (0, v))
                .chain((1..order).map(|u| (u, 0)))
                .collect(),
            order,
        }
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

impl Wheel for EdgeList {
    /// # Panics
    ///
    /// * Panics if `order` is less than `4`.
    fn wheel(order: usize) -> Self {
        assert!(order >= 4, "a wheel digraph has at least four vertices");

        Self {
            arcs: (1..order)
                .map(|v| (0, v))
                .chain((1..order).flat_map(|u| {
                    let last = order - 1;

                    once(0)
                        .chain(once(if u == 1 { last } else { u - 1 }))
                        .chain(once(if u == last { 1 } else { u + 1 }))
                        .map(move |v| (u, v))
                }))
                .collect(),
            order,
        }
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
    #[should_panic(expected = "v = 1 isn't in the digraph")]
    fn add_arc_out_of_bounds_u() {
        EdgeList::trivial().add_arc(0, 1);
    }

    #[test]
    #[should_panic(expected = "u = 1 isn't in the digraph")]
    fn add_arc_out_of_bounds_v() {
        EdgeList::trivial().add_arc(1, 0);
    }

    #[test]
    fn from_adjacency_list() {
        let digraph = AdjacencyList::from([
            BTreeSet::from([1]),
            BTreeSet::from([2]),
            BTreeSet::new(),
        ]);

        let digraph = EdgeList::from(digraph);

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

        let digraph = EdgeList::from(digraph);

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

    #[test]
    fn from_iter() {
        let arcs = BTreeSet::from([(0, 1), (1, 2)]);
        let digraph = EdgeList::from(arcs);

        assert_eq!(digraph.order(), 3);
        assert!(digraph.arcs().eq([(0, 1), (1, 2)]));
    }

    #[test]
    #[should_panic(expected = "u = 1 equals v = 1")]
    fn from_iter_self_loop() {
        let _ = EdgeList::from(BTreeSet::from([(0, 1), (1, 1)]));
    }

    #[test]
    fn is_simple_self_loop() {
        let digraph = EdgeList {
            arcs: BTreeSet::from([(0, 0)]),
            order: 1,
        };

        assert!(!digraph.is_simple());
    }
}

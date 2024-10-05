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
//! ![A digraph of order `5` and size `8`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_list_1.svg?)
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
//! ![A pseudograph with a self-loop](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_list_self_loop.svg?)
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
//! ![A multigraph with a parallel arc](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_list_parallel_arcs.svg?)
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
        gen::prng::Xoshiro256StarStar,
        AddArc,
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
        EdgeList,
        Empty,
        ErdosRenyi,
        GrowingNetwork,
        HasArc,
        Indegree,
        IsComplete,
        IsSimple,
        IsTournament,
        Order,
        OutNeighbors,
        OutNeighborsWeighted,
        Outdegree,
        Path,
        RandomTournament,
        RemoveArc,
        Size,
        Star,
        Union,
        Vertices,
        Wheel,
    },
    std::{
        collections::BTreeSet,
        iter::{
            once,
            repeat,
        },
    },
};

/// A representation of an unweighted digraph.
///
/// # Examples
///
/// ## Valid digraph
///
/// A valid digraph of order `5` and size `8`.
///
/// ![A digraph of order `5` and size `8`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_list_1.svg?)
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
/// ![A pseudograph with a self-loop](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_list_self_loop.svg?)
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
/// ![A multigraph with a parallel arc](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_list_parallel_arcs.svg?)
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
    type Weight = usize;

    fn arc_weight(&self, u: usize, v: usize) -> Option<&Self::Weight> {
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

impl ArcsWeighted for AdjacencyList {
    type Weight = usize;

    fn arcs_weighted(
        &self,
    ) -> impl Iterator<Item = (usize, usize, &Self::Weight)> {
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

        Self {
            arcs: repeat(clique_2)
                .take(m)
                .chain(repeat(clique_1).take(n))
                .collect(),
        }
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

impl Complement for AdjacencyList {
    fn complement(&self) -> Self {
        let order = self.order();
        let vertices = (0..order).collect::<BTreeSet<_>>();

        Self {
            arcs: self
                .arcs
                .iter()
                .enumerate()
                .map(|(u, out_neighbors)| {
                    let mut out_neighbors = vertices
                        .clone()
                        .difference(out_neighbors)
                        .copied()
                        .collect::<BTreeSet<usize>>();

                    let _ = out_neighbors.remove(&u);

                    out_neighbors
                })
                .collect(),
        }
    }
}

impl Complete for AdjacencyList {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn complete(order: usize) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        let neighbors = (0..order).collect::<BTreeSet<usize>>();
        let mut arcs = vec![neighbors; order];

        for (u, neighbors) in arcs.iter_mut().enumerate().take(order) {
            let _ = neighbors.remove(&u);
        }

        Self { arcs }
    }
}

impl Converse for AdjacencyList {
    /// # Panics
    ///
    /// Panics if the digraphs' order is zero.
    fn converse(&self) -> Self {
        assert!(self.order() > 0, "a digraph has at least one vertex");

        Self {
            arcs: self.arcs.iter().enumerate().fold(
                vec![BTreeSet::new(); self.order()],
                |mut converse, (u, set)| {
                    for &v in set {
                        let _ = converse[v].insert(u);
                    }

                    converse
                },
            ),
        }
    }
}

impl Cycle for AdjacencyList {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn cycle(order: usize) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        if order == 1 {
            return Self {
                arcs: vec![BTreeSet::new()],
            };
        }

        Self {
            arcs: (0..order)
                .map(|u| {
                    BTreeSet::from([(u + order - 1) % order, (u + 1) % order])
                })
                .collect(),
        }
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

impl ErdosRenyi for AdjacencyList {
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
                .map(|u| {
                    (0..u)
                        .chain((u + 1)..order)
                        .filter(|_| rng.next_f64() < p)
                        .collect()
                })
                .collect(),
        }
    }
}

macro_rules! impl_from_arcs_empty_order {
    ($type:ty) => {
        impl From<$type> for AdjacencyList {
            /// # Panics
            ///
            /// * Panics if `digraph` is empty.
            /// * Panics if, for any arc `u -> v` in `digraph`, `u` equals `v`.
            /// * Panics if, for any arc `u -> v` in `digraph`, `v` isn't in
            ///   the digraph.
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

impl_from_arcs_empty_order!(AdjacencyMap);
impl_from_arcs_empty_order!(AdjacencyMatrix);
impl_from_arcs_empty_order!(EdgeList);

impl<I> From<I> for AdjacencyList
where
    I: IntoIterator<Item = BTreeSet<usize>>,
{
    /// # Panics
    ///
    /// * Panics if `iter` is empty.
    /// * Panics if, for any arc `u -> v` in `arcs`, `u` equals `v`.
    /// * Panics if, for any arc `u -> v` in `arcs`, `v` isn't in the digraph.
    fn from(iter: I) -> Self {
        let digraph = Self {
            arcs: iter.into_iter().collect(),
        };

        let order = digraph.order();

        assert!(order > 0, "a digraph has at least one vertex");

        for (u, v) in digraph.arcs() {
            assert_ne!(u, v, "u = {u} equals v = {v}");
            assert!(v < order, "v = {v} isn't in the digraph");
        }

        digraph
    }
}

impl GrowingNetwork for AdjacencyList {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn growing_network(order: usize, seed: u64) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        let mut rng = Xoshiro256StarStar::new(seed);

        Self {
            arcs: once(BTreeSet::new())
                .chain((1..order).map(|u| {
                    BTreeSet::from([usize::try_from(rng.next().unwrap())
                        .unwrap()
                        % u])
                }))
                .collect(),
        }
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

    fn is_source(&self, v: usize) -> bool {
        self.arcs.iter().all(|set| !set.contains(&v))
    }
}

impl IsComplete for AdjacencyList {
    fn is_complete(&self) -> bool {
        *self == Self::complete(self.order())
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

impl IsTournament for AdjacencyList {
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

impl OutNeighborsWeighted for AdjacencyList {
    type Weight = usize;

    /// # Panics
    ///
    /// Panics if `u` isn't in the digraph.
    fn out_neighbors_weighted(
        &self,
        u: usize,
    ) -> impl Iterator<Item = (usize, &Self::Weight)> {
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

    fn is_sink(&self, u: usize) -> bool {
        self.arcs[u].is_empty()
    }
}

impl Path for AdjacencyList {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn path(order: usize) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        Self {
            arcs: (0..order - 1)
                .map(|u| BTreeSet::from([u + 1]))
                .chain(once(BTreeSet::new()))
                .collect(),
        }
    }
}

impl RandomTournament for AdjacencyList {
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

impl Star for AdjacencyList {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn star(order: usize) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        Self {
            arcs: once((1..order).collect())
                .chain((1..order).map(|_| BTreeSet::from([0])))
                .collect(),
        }
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

impl Wheel for AdjacencyList {
    /// # Panics
    ///
    /// Panics if `order` is less than `4`.
    fn wheel(order: usize) -> Self {
        assert!(order >= 4, "a wheel digraph has at least four vertices");

        Self {
            arcs: once((1..order).collect())
                .chain((1..order).map(|u| {
                    let last = order - 1;

                    BTreeSet::from([
                        0,
                        if u == 1 { last } else { u - 1 },
                        if u == last { 1 } else { u + 1 },
                    ])
                }))
                .collect(),
        }
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
    fn from_adjacency_map() {
        let digraph = AdjacencyMap::from([
            BTreeSet::from([1]),
            BTreeSet::from([2]),
            BTreeSet::new(),
        ]);

        let digraph = AdjacencyList::from(digraph);

        assert_eq!(digraph.order(), 3);
        assert!(digraph.arcs().eq([(0, 1), (1, 2)]));
    }

    #[test]
    fn from_adjacency_matrix() {
        let digraph = AdjacencyMatrix::from([(0, 1), (1, 2)]);
        let digraph = AdjacencyList::from(digraph);

        assert_eq!(digraph.order(), 3);
        assert!(digraph.arcs().eq([(0, 1), (1, 2)]));
    }

    #[test]
    fn from_edge_list() {
        let digraph = EdgeList::from([(0, 1), (1, 2)]);
        let digraph = AdjacencyList::from(digraph);

        assert_eq!(digraph.order(), 3);
        assert!(digraph.arcs().eq([(0, 1), (1, 2)]));
    }

    #[test]
    fn from_iter() {
        let digraph = AdjacencyList::from([
            BTreeSet::from([1]),
            BTreeSet::from([2]),
            BTreeSet::new(),
        ]);

        assert_eq!(digraph.order(), 3);
        assert!(digraph.arcs().eq([(0, 1), (1, 2)]));
    }

    #[test]
    #[should_panic(expected = "a digraph has at least one vertex")]
    fn from_iter_empty() {
        let _ = AdjacencyList::from(Vec::<BTreeSet<usize>>::new());
    }

    #[test]
    #[should_panic(expected = "v = 1 isn't in the digraph")]
    fn from_iter_out_of_bounds_v() {
        let _ = AdjacencyList::from([BTreeSet::from([1])]);
    }

    #[test]
    #[should_panic(expected = "u = 0 equals v = 0")]
    fn from_iter_u_equals_v() {
        let _ = AdjacencyList::from([BTreeSet::from([0])]);
    }

    #[test]
    fn is_simple_self_loop() {
        let digraph = AdjacencyList {
            arcs: vec![BTreeSet::from([0])],
        };

        assert!(!digraph.is_simple());
    }
}

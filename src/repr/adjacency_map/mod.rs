//! Represent sparse unweighted digraphs.
//!
//! An [`AdjacencyMap`] is a map of sets.
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
//!     AdjacencyMap,
//!     Arcs,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyMap::empty(5);
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
//! A self-loop isn't allowed. [`AdjacencyMap`] can't represent this
//! pseudograph. The self-loop is red.
//!
//! ![A pseudograph with a self-loop](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_list_self_loop.svg?)
//!
//! Adding a self-loop panics.
//!
//! ```should_panic
//! use graaf::{
//!     AddArc,
//!     AdjacencyMap,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyMap::empty(4);
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
//! Parallel arcs aren't allowed. [`AdjacencyMap`] can't represent this
//! multigraph. The parallel arc is red.
//!
//! ![A multigraph with a parallel arc](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_list_parallel_arcs.svg?)
//!
//! Adding a parallel arc doesn't change the digraph.
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyMap,
//!     Arcs,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyMap::empty(4);
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
        FilterVertices,
        GrowingNetwork,
        HasArc,
        Indegree,
        IsSimple,
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
        collections::{
            BTreeMap,
            BTreeSet,
        },
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
///     AdjacencyMap,
///     Arcs,
///     Empty,
/// };
///
/// let mut digraph = AdjacencyMap::empty(5);
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
/// A self-loop isn't allowed. [`AdjacencyMap`] can't represent this
/// pseudograph. The self-loop is red.
///
/// ![A pseudograph with a self-loop](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_list_self_loop.svg?)
///
/// Adding a self-loop panics.
///
/// ```should_panic
/// use graaf::{
///     AddArc,
///     AdjacencyMap,
///     Empty,
/// };
///
/// let mut digraph = AdjacencyMap::empty(4);
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
/// Parallel arcs aren't allowed. [`AdjacencyMap`] can't represent this
/// multigraph. The parallel arc is red.
///
/// ![A multigraph with a parallel arc](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/adjacency_list_parallel_arcs.svg?)
///
/// Adding a parallel arc doesn't change the digraph.
///
/// ```
/// use graaf::{
///     AddArc,
///     AdjacencyMap,
///     Arcs,
///     Empty,
/// };
///
/// let mut digraph = AdjacencyMap::empty(4);
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
pub struct AdjacencyMap {
    arcs: BTreeMap<usize, BTreeSet<usize>>,
}

impl AddArc for AdjacencyMap {
    /// # Panics
    ///
    /// * Panics if `u` equals `v`; self-loops aren't allowed.
    /// * Panics if `u` isn't in the digraph.
    /// * Panics if `v` isn't in the digraph.
    fn add_arc(&mut self, u: usize, v: usize) {
        // Self-loops aren't allowed.
        assert_ne!(u, v, "u = {u} equals v = {v}");

        let _ = self.arcs.entry(u).or_default().insert(v);
        let _ = self.arcs.entry(v).or_default();
    }
}

impl ArcWeight<usize> for AdjacencyMap {
    type Weight = usize;

    fn arc_weight(&self, u: usize, v: usize) -> Option<&Self::Weight> {
        self.has_arc(u, v).then_some(&1)
    }
}

impl Arcs for AdjacencyMap {
    fn arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.arcs
            .iter()
            .flat_map(|(u, set)| set.iter().map(move |v| (*u, *v)))
    }
}

impl ArcsWeighted for AdjacencyMap {
    type Weight = usize;

    fn arcs_weighted(
        &self,
    ) -> impl Iterator<Item = (usize, usize, &Self::Weight)> {
        self.arcs().map(move |(u, v)| (u, v, &1))
    }
}

impl Biclique for AdjacencyMap {
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
                .enumerate()
                .collect(),
        }
    }
}

impl Circuit for AdjacencyMap {
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
                .map(|u| (u, BTreeSet::from([(u + 1) % order])))
                .collect(),
        }
    }
}

impl Complete for AdjacencyMap {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn complete(order: usize) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        Self {
            arcs: (0..order)
                .map(|u| (u, (0..u).chain((u + 1)..order).collect()))
                .collect(),
        }
    }
}

impl Complement for AdjacencyMap {
    fn complement(&self) -> Self {
        let order = self.order();
        let vertices = (0..order).collect::<BTreeSet<_>>();

        Self {
            arcs: self
                .arcs
                .iter()
                .map(|(u, out_neighbors)| {
                    let mut out_neighbors = vertices
                        .clone()
                        .difference(out_neighbors)
                        .copied()
                        .collect::<BTreeSet<_>>();

                    let _ = out_neighbors.remove(u);

                    (*u, out_neighbors)
                })
                .collect(),
        }
    }
}

impl Converse for AdjacencyMap {
    /// # Panics
    ///
    /// Panics if the digraph's order is zero.
    fn converse(&self) -> Self {
        let mut arcs = self
            .arcs
            .keys()
            .map(|u| (*u, BTreeSet::new()))
            .collect::<BTreeMap<usize, BTreeSet<usize>>>();

        for (u, v) in &self.arcs {
            for v in v {
                let _ = arcs.get_mut(v).unwrap().insert(*u);
            }
        }

        Self { arcs }
    }
}

impl Cycle for AdjacencyMap {
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
                .map(|u| {
                    (
                        u,
                        BTreeSet::from([
                            (u + order - 1) % order,
                            (u + 1) % order,
                        ]),
                    )
                })
                .collect(),
        }
    }
}

impl Empty for AdjacencyMap {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn empty(order: usize) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        Self::from(vec![BTreeSet::new(); order])
    }
}

impl ErdosRenyi for AdjacencyMap {
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
                    (
                        u,
                        (0..u)
                            .chain((u + 1)..order)
                            .filter(|_| rng.next_f64() < p)
                            .collect(),
                    )
                })
                .collect(),
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
        impl From<$type> for AdjacencyMap {
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
impl_from_arcs_order!(AdjacencyMatrix);
impl_from_arcs_order!(EdgeList);

impl<I> From<I> for AdjacencyMap
where
    I: IntoIterator<Item = BTreeSet<usize>>,
{
    /// # Panics
    ///
    /// * Panics if `iter` is empty.
    /// * Panics if, for any arc `u -> v` in `digraph`, `u` equals `v`.
    /// * Panics if, for any arc `u -> v` in `digraph`, `v` isn't in the
    ///   digraph.
    fn from(iter: I) -> Self {
        let digraph = Self {
            arcs: iter.into_iter().enumerate().collect(),
        };

        assert!(digraph.order() > 0, "a digraph has at least one vertex");

        for (u, v) in digraph.arcs() {
            assert_ne!(u, v, "u = {u} equals v = {v}");

            assert!(
                digraph.arcs.contains_key(&v),
                "v = {v} isn't in the digraph"
            );
        }

        digraph
    }
}

impl FilterVertices for AdjacencyMap {
    /// # Panics
    ///
    /// Panics if the subgraph has zero vertices.
    fn filter_vertices<P>(&self, predicate: P) -> Self
    where
        P: Fn(usize) -> bool,
    {
        let mut arcs = BTreeMap::<usize, BTreeSet<usize>>::new();

        for (u, v) in self.arcs() {
            if predicate(u) && predicate(v) {
                let _ = arcs.entry(u).or_default().insert(v);
                let _ = arcs.entry(v).or_default();
            }
        }

        Self { arcs }
    }
}

impl GrowingNetwork for AdjacencyMap {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn growing_network(order: usize, seed: u64) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        let mut rng = Xoshiro256StarStar::new(seed);

        Self {
            arcs: once((0, BTreeSet::new()))
                .chain((1..order).map(|u| {
                    (
                        u,
                        BTreeSet::from([usize::try_from(rng.next().unwrap())
                            .expect("conversion failed")
                            % u]),
                    )
                }))
                .collect(),
        }
    }
}

impl HasArc for AdjacencyMap {
    fn has_arc(&self, u: usize, v: usize) -> bool {
        self.arcs.get(&u).map_or(false, |set| set.contains(&v))
    }
}

impl Indegree for AdjacencyMap {
    /// # Panics
    ///
    /// Panics if `v` isn't in the digraph.
    fn indegree(&self, v: usize) -> usize {
        assert!(self.arcs.contains_key(&v), "v = {v} isn't in the digraph");

        self.arcs.values().filter(|set| set.contains(&v)).count()
    }
}

impl IsSimple for AdjacencyMap {
    fn is_simple(&self) -> bool {
        // We only check for self-loops. [`AdjacencyMap`] can't represent
        // parallel arcs.
        self.arcs.iter().all(|(u, set)| !set.contains(u))
    }
}

impl Order for AdjacencyMap {
    fn order(&self) -> usize {
        self.arcs.len()
    }
}

impl OutNeighbors for AdjacencyMap {
    /// # Panics
    ///
    /// Panics if `u` isn't in the digraph.
    fn out_neighbors(&self, u: usize) -> impl Iterator<Item = usize> {
        assert!(self.arcs.contains_key(&u), "u = {u} isn't in the digraph");

        self.arcs[&u].iter().copied()
    }
}

impl OutNeighborsWeighted for AdjacencyMap {
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

impl Outdegree for AdjacencyMap {
    /// # Panics
    ///
    /// Panics if `u` isn't in the digraph.
    fn outdegree(&self, u: usize) -> usize {
        assert!(self.arcs.contains_key(&u), "u = {u} isn't in the digraph");

        self.arcs[&u].len()
    }
}

impl Path for AdjacencyMap {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn path(order: usize) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        if order == 1 {
            return Self::trivial();
        }

        let last = order - 1;

        Self {
            arcs: (0..last)
                .map(|u| (u, BTreeSet::from([u + 1])))
                .chain(once((last, BTreeSet::new())))
                .collect(),
        }
    }
}

impl RandomTournament for AdjacencyMap {
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

impl RemoveArc for AdjacencyMap {
    fn remove_arc(&mut self, u: usize, v: usize) -> bool {
        self.arcs.get_mut(&u).map_or(false, |set| set.remove(&v))
    }
}

impl Size for AdjacencyMap {
    fn size(&self) -> usize {
        self.arcs.values().map(BTreeSet::len).sum()
    }
}

impl Star for AdjacencyMap {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn star(order: usize) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        Self {
            arcs: once((0, (1..order).collect()))
                .chain((1..order).map(|u| (u, BTreeSet::from([0]))))
                .collect(),
        }
    }
}

impl Union for AdjacencyMap {
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

impl Vertices for AdjacencyMap {
    fn vertices(&self) -> impl Iterator<Item = usize> {
        self.arcs.keys().copied()
    }
}

impl Wheel for AdjacencyMap {
    /// # Panics
    ///
    /// Panics if `order` is less than `4`.
    fn wheel(order: usize) -> Self {
        assert!(order >= 4, "a wheel digraph has at least four vertices");

        Self {
            arcs: once((0, (1..order).collect()))
                .chain((1..order).map(|u| {
                    let last = order - 1;

                    (
                        u,
                        BTreeSet::from([
                            0,
                            if u == 1 { last } else { u - 1 },
                            if u == last { 1 } else { u + 1 },
                        ]),
                    )
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

    test_unweighted!(AdjacencyMap, repr::adjacency_map::fixture);

    #[test]
    fn from_adjacency_list() {
        let digraph = AdjacencyList::from([
            BTreeSet::from([1]),
            BTreeSet::from([2]),
            BTreeSet::new(),
        ]);

        let digraph = AdjacencyMap::from(digraph);

        assert_eq!(digraph.order(), 3);
        assert!(digraph.arcs().eq([(0, 1), (1, 2)]));
    }

    #[test]
    fn from_adjacency_matrix() {
        let digraph = AdjacencyMatrix::from([(0, 1), (1, 2)]);
        let digraph = AdjacencyMap::from(digraph);

        assert_eq!(digraph.order(), 3);
        assert!(digraph.arcs().eq([(0, 1), (1, 2)]));
    }

    #[test]
    fn from_edge_list() {
        let digraph = EdgeList::from([(0, 1), (1, 2)]);
        let digraph = AdjacencyMap::from(digraph);

        assert_eq!(digraph.order(), 3);
        assert!(digraph.arcs().eq([(0, 1), (1, 2)]));
    }

    #[test]
    fn from_iter() {
        let digraph = AdjacencyMap::from([
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
        let _ = AdjacencyMap::from(Vec::<BTreeSet<usize>>::new());
    }

    #[test]
    #[should_panic(expected = "v = 1 isn't in the digraph")]
    fn from_iter_out_of_bounds_v() {
        let _ = AdjacencyMap::from([BTreeSet::from([1])]);
    }

    #[test]
    #[should_panic(expected = "u = 0 equals v = 0")]
    fn from_iter_u_equals_v() {
        let _ = AdjacencyMap::from([BTreeSet::from([0])]);
    }

    #[test]
    fn is_simple_self_loop() {
        let digraph = AdjacencyMap {
            arcs: BTreeMap::from([(0, BTreeSet::from([0]))]),
        };

        assert!(!digraph.is_simple());
    }
}

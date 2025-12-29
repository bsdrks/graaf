//! Represent sparse unweighted digraphs.
//!
//! An [`AdjacencyMap`] is a map of sets.
//!
//! # Non-contiguity
//!
//! The vertices don't have to be contiguous. The digraph can have vertices
//! outside the range `[0, v)`, where `v` is the digraph's order.
//!
//! # Isolated vertices
//!
//! Isolated vertices are denoted by an empty set in the map.
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
        AddArc,
        AdjacencyList,
        AdjacencyMatrix,
        Arcs,
        Biclique,
        Circuit,
        Complement,
        Complete,
        Converse,
        Cycle,
        Degree,
        DegreeSequence,
        EdgeList,
        Empty,
        ErdosRenyi,
        FilterVertices,
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
        Sources,
        Star,
        Union,
        Vertices,
        Wheel,
        r#gen::prng::Xoshiro256StarStar,
    },
    std::{
        cmp::Ordering,
        collections::{
            BTreeMap,
            BTreeSet,
        },
        iter::{
            once,
            repeat_n,
        },
        mem::ManuallyDrop,
        num::NonZero,
        ptr::read,
        sync::{
            Arc,
            Mutex,
            OnceLock,
        },
        thread::{
            self,
            available_parallelism,
            scope,
            spawn,
        },
    },
};

/// A representation of an unweighted digraph.
///
/// # Non-contiguity
///
/// The vertices don't have to be contiguous. The digraph can have vertices
/// outside the range `[0, v)`, where `v` is the digraph's order.
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

fn empty_set() -> &'static BTreeSet<usize> {
    static EMPTY: OnceLock<BTreeSet<usize>> = OnceLock::new();
    EMPTY.get_or_init(BTreeSet::new)
}

impl AddArc for AdjacencyMap {
    /// We add `u` and `v` if they aren't in the digraph.
    ///
    /// # Panics
    ///
    /// * Panics if `u` equals `v`; self-loops aren't allowed.
    ///
    /// # Complexity
    ///
    /// The time complexity is `O(log v)`, where `v` is the digraph's order.
    fn add_arc(&mut self, u: usize, v: usize) {
        // Self-loops aren't allowed.
        assert_ne!(
            u, v,
            "u = {u} equals v = {v}, but self-loops aren't allowed"
        );

        let _ = self.arcs.entry(u).or_default().insert(v);
        let _ = self.arcs.entry(v).or_default();
    }
}

impl Arcs for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity of full iteration is `O(v + a)`, where `v` is the
    /// digraph's order and `a` is the digraph's size.
    fn arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.arcs
            .iter()
            .flat_map(|(u, set)| set.iter().map(move |v| (*u, *v)))
    }
}

impl Biclique for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(v * a)`, where `v` is the digraph's order and
    /// `a` is the digraph's size.
    ///
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
            arcs: repeat_n(clique_2, m)
                .chain(repeat_n(clique_1, n))
                .enumerate()
                .collect(),
        }
    }
}

impl Circuit for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(v log v)`, where `v` is the digraph's order.
    ///
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
    /// # Complexity
    ///
    /// The time complexity is `O(v²)`, where `v` is the digraph's order.
    ///
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn complete(order: usize) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        if order == 1 {
            return Self::trivial();
        }

        let vertices = (0..order).collect::<BTreeSet<_>>();
        let mut arcs = BTreeMap::new();

        for u in 0..order {
            let mut out_neighbors = vertices.clone();
            let _ = out_neighbors.remove(&u);

            drop(arcs.insert(u, out_neighbors));
        }

        Self { arcs }
    }
}

impl Complement for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(v² log v)`, where `v` is the digraph's
    /// order.
    fn complement(&self) -> Self {
        let order = self.order();
        let vertices = (0..order).collect::<BTreeSet<_>>();

        Self {
            arcs: self
                .arcs
                .iter()
                .map(|(u, out_neighbors)| {
                    let mut out_neighbors = vertices
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
    /// # Complexity
    ///
    /// The time complexity is `O(v² log v)`, where `v` is the digraph's
    /// order.
    fn converse(&self) -> Self {
        let order = self.order();
        let mut vec = vec![BTreeSet::new(); order];

        for (u, out_neighbors) in &self.arcs {
            for v in out_neighbors {
                unsafe {
                    let _ = vec.get_unchecked_mut(*v).insert(*u);
                };
            }
        }

        Self {
            arcs: vec.into_iter().enumerate().collect(),
        }
    }
}

impl Cycle for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(v log v)`, where `v` is the digraph's order.
    ///
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

impl DegreeSequence for AdjacencyMap {
    fn degree_sequence(&self) -> impl Iterator<Item = usize> {
        self.vertices().map(move |v| self.degree(v))
    }
}

impl Empty for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(v)`, where `v` is the digraph's order.
    ///
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn empty(order: usize) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        Self::from(vec![BTreeSet::new(); order])
    }
}

impl ErdosRenyi for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(v² log v)`, where `v` is the digraph's
    /// order.
    ///
    /// # Panics
    ///
    /// * Panics if `order` is zero.
    /// * Panics if `p` isn't in `[0, 1]`.
    fn erdos_renyi(order: usize, p: f64, seed: u64) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");
        assert!((0.0..=1.0).contains(&p), "p = {p} must be in [0, 1]");

        if order == 1 {
            return Self::trivial();
        }

        if p > 0.5 {
            return Self::erdos_renyi(order, 1.0 - p, seed).complement();
        }

        let t = order.min(available_parallelism().map_or(1, NonZero::get));
        let chunk_size = order.div_ceil(t);
        let mut handles = Vec::with_capacity(t);

        for thread_id in 0..t {
            let start = thread_id * chunk_size;
            let end = order.min(start + chunk_size);

            if start >= end {
                break;
            }

            let thread_seed = seed.wrapping_add(thread_id as u64);

            let handle = thread::spawn(move || {
                let mut rng = Xoshiro256StarStar::new(thread_seed);
                let mut local_results = Vec::with_capacity(end - start);

                for u in start..end {
                    let mut out_neighbors = BTreeSet::new();

                    for v in 0..order {
                        if v != u && rng.next_f64() < p {
                            let _ = out_neighbors.insert(v);
                        }
                    }

                    local_results.push((u, out_neighbors));
                }

                local_results
            });

            handles.push(handle);
        }

        let mut results = Vec::with_capacity(order);

        for handle in handles {
            let local = unsafe { handle.join().unwrap_unchecked() };

            results.extend(local);
        }

        results.sort_by_key(|&(u, _)| u);

        Self {
            arcs: results.into_iter().collect::<BTreeMap<_, _>>(),
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
    /// # Complexity
    ///
    /// The time complexity is `O(v log v)`, where `v` is the digraph's order.
    ///
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
    /// # Complexity
    ///
    /// The time complexity is `O(a log v)`, where `v` is the digraph's order
    /// and `a` is the digraph's size.
    fn filter_vertices<P>(&self, predicate: P) -> Self
    where
        P: Fn(usize) -> bool,
    {
        let mut arcs = BTreeMap::<usize, BTreeSet<usize>>::new();

        for u in self.vertices() {
            if predicate(u) {
                let _ = arcs.entry(u).or_default();

                for v in self.out_neighbors(u) {
                    if predicate(v) {
                        let _ = arcs.entry(u).or_default().insert(v);
                        let _ = arcs.entry(v).or_default();
                    }
                }
            }
        }

        Self { arcs }
    }
}

impl RandomRecursiveTree for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(v log v)`, where `v` is the digraph's order.
    ///
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn random_recursive_tree(order: usize, seed: u64) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        if order == 1 {
            return Self::trivial();
        }

        let mut rng = Xoshiro256StarStar::new(seed);

        Self {
            arcs: once((0, BTreeSet::new()))
                .chain((1..order).map(|u| {
                    (u, unsafe {
                        BTreeSet::from([usize::try_from(
                            rng.next().unwrap_unchecked(),
                        )
                        .unwrap_unchecked()
                            % u])
                    })
                }))
                .collect(),
        }
    }
}

impl HasArc for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(log v)`, where `v` is the digraph's order.
    fn has_arc(&self, u: usize, v: usize) -> bool {
        self.arcs.get(&u).is_some_and(|set| set.contains(&v))
    }
}

impl HasEdge for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(log v)`, where `v` is the digraph's order.
    fn has_edge(&self, u: usize, v: usize) -> bool {
        self.has_arc(u, v) && self.has_arc(v, u)
    }
}

impl HasWalk for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(w)`, where `w` is the walk's length.
    fn has_walk(&self, walk: &[usize]) -> bool {
        let len = walk.len();

        if len <= 1 {
            return false;
        }

        unsafe {
            let mut ptr = walk.as_ptr();
            let end = walk.as_ptr().add(len - 1);

            while ptr < end {
                let u = *ptr;
                let v = *ptr.add(1);

                if !self.has_arc(u, v) {
                    return false;
                }

                ptr = ptr.add(1);
            }
        }

        true
    }
}

impl Indegree for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(v log v)`, where `v` is the digraph's order.
    ///
    /// # Panics
    ///
    /// Panics if `v` isn't in the digraph.
    fn indegree(&self, v: usize) -> usize {
        assert!(self.arcs.contains_key(&v), "v = {v} isn't in the digraph");

        self.arcs.values().filter(|set| set.contains(&v)).count()
    }

    /// # Complexity
    ///
    /// The time complexity is `O(v log v)`, where `v` is the digraph's order.
    fn is_source(&self, v: usize) -> bool {
        self.arcs.values().all(|set| !set.contains(&v))
    }
}

impl IndegreeSequence for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity of full iteration is `O(v² log v)`, where `v` is
    /// the digraph's order.
    fn indegree_sequence(&self) -> impl Iterator<Item = usize> {
        self.vertices().map(move |v| self.indegree(v))
    }
}

impl InNeighbors for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity of full iteration is `O(v log v)`, where `v` is the
    /// digraph's order.
    fn in_neighbors(&self, v: usize) -> impl Iterator<Item = usize> {
        self.arcs
            .iter()
            .filter_map(move |(x, set)| set.contains(&v).then_some(*x))
    }
}

impl IsComplete for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(a)`, where `a` is the digraph's size.
    fn is_complete(&self) -> bool {
        let expected_outdegree = self.order() - 1;

        for out_neighbors in self.arcs.values() {
            if out_neighbors.len() != expected_outdegree {
                return false;
            }
        }

        true
    }
}

impl IsRegular for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(v² log v)`, where `v` is the digraph's
    /// order.
    ///
    /// # Panics
    ///
    /// Panics if the digraph has no vertices.
    fn is_regular(&self) -> bool {
        let mut semidegrees = self.semidegree_sequence();

        let (u, v) = semidegrees
            .next()
            .expect("a digraph has at least one vertex");

        u == v && semidegrees.all(|(x, y)| x == u && y == v)
    }
}

impl IsSemicomplete for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(v² log v)`, where `v` is the digraph's
    /// order.
    fn is_semicomplete(&self) -> bool {
        let order = self.order();

        if self.size() < order * (order - 1) / 2 {
            return false;
        }

        let mut out_neighbors = Vec::<&BTreeSet<_>>::with_capacity(order);

        for u in self.vertices() {
            out_neighbors
                .push(self.arcs.get(&u).unwrap_or_else(|| empty_set()));
        }

        let ptr = out_neighbors.as_ptr();

        unsafe {
            for u in self.vertices() {
                for v in self.vertices() {
                    if u != v
                        && !(*ptr.add(u)).contains(&v)
                        && !(*ptr.add(v)).contains(&u)
                    {
                        return false;
                    }
                }
            }
        }

        true
    }
}

impl IsSimple for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(v log v)`, where `v` is the digraph's order.
    fn is_simple(&self) -> bool {
        // We only check for self-loops. [`AdjacencyMap`] can't represent
        // parallel arcs.
        self.arcs.iter().all(|(u, set)| !set.contains(u))
    }
}

impl IsTournament for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(v² log v)`, where `v` is the digraph's
    /// order.
    fn is_tournament(&self) -> bool {
        let order = self.order();

        if self.size() != order * (order - 1) / 2 {
            return false;
        }

        let mut out_neighbors = Vec::<&BTreeSet<_>>::with_capacity(order);

        for u in self.vertices() {
            out_neighbors
                .push(self.arcs.get(&u).unwrap_or_else(|| empty_set()));
        }

        let ptr = out_neighbors.as_ptr();

        unsafe {
            for u in self.vertices() {
                for v in self.vertices() {
                    if u != v
                        && (*ptr.add(u)).contains(&v)
                            == (*ptr.add(v)).contains(&u)
                    {
                        return false;
                    }
                }
            }
        }

        true
    }
}

impl Order for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(1)`.
    fn order(&self) -> usize {
        self.arcs.len()
    }
}

impl OutNeighbors for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity of full iteration is `O(log v + outdegree)`, where
    /// `v` is the digraph's order and `outdegree` is the outdegree of `u`.
    ///
    /// # Panics
    ///
    /// Panics if `u` isn't in the digraph.
    fn out_neighbors(&self, u: usize) -> impl Iterator<Item = usize> {
        assert!(self.arcs.contains_key(&u), "u = {u} isn't in the digraph");

        unsafe { self.arcs.get(&u).unwrap_unchecked().iter().copied() }
    }
}

impl Outdegree for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(log v)`, where `v` is the digraph's order.
    ///
    /// # Panics
    ///
    /// Panics if `u` isn't in the digraph.
    fn outdegree(&self, u: usize) -> usize {
        self.arcs.get(&u).map_or_else(
            || {
                panic!("u = {u} isn't in the digraph");
            },
            BTreeSet::len,
        )
    }

    /// # Complexity
    ///
    /// The time complexity is `O(log v)`, where `v` is the digraph's order.
    ///
    /// # Panics
    ///
    /// Panics if `u` isn't in the digraph.
    fn is_sink(&self, u: usize) -> bool {
        self.arcs.get(&u).map_or_else(
            || {
                panic!("u = {u} isn't in the digraph");
            },
            BTreeSet::is_empty,
        )
    }
}

impl Path for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(v)`, where `v` is the digraph's order.
    ///
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
    /// # Complexity
    ///
    /// The time complexity is `O(v² log v)`, where `v` is the digraph's
    /// order.
    ///
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn random_tournament(order: usize, seed: u64) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        if order == 1 {
            return Self::trivial();
        }

        let shared_arcs: Arc<Vec<_>> = Arc::new(
            (0..order).map(|_| Mutex::new(BTreeSet::new())).collect(),
        );

        let t = order.min(available_parallelism().map_or(1, NonZero::get));
        let chunk_size = order.div_ceil(t);
        let mut handles = Vec::with_capacity(t);

        for thread_id in 0..t {
            let start = thread_id * chunk_size;
            let end = order.min(start + chunk_size);

            if start >= end {
                break;
            }

            let shared_arcs = Arc::clone(&shared_arcs);
            let thread_seed = seed.wrapping_add(thread_id as u64);

            let handle = spawn(move || {
                let mut rng = Xoshiro256StarStar::new(thread_seed);

                for u in start..end {
                    for v in (u + 1)..order {
                        unsafe {
                            if rng.next_bool() {
                                let _ = shared_arcs
                                    .get_unchecked(u)
                                    .lock()
                                    .unwrap_unchecked()
                                    .insert(v);
                            } else {
                                let _ = shared_arcs
                                    .get_unchecked(v)
                                    .lock()
                                    .unwrap_unchecked()
                                    .insert(u);
                            }
                        }
                    }
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            unsafe {
                handle.join().unwrap_unchecked();
            }
        }

        Self {
            arcs: (0..order)
                .map(|u| {
                    (u, unsafe {
                        shared_arcs
                            .get_unchecked(u)
                            .lock()
                            .unwrap_unchecked()
                            .clone()
                    })
                })
                .collect::<BTreeMap<_, _>>(),
        }
    }
}

impl RemoveArc for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(log v)`, where `v` is the digraph's order.
    fn remove_arc(&mut self, u: usize, v: usize) -> bool {
        self.arcs.get_mut(&u).is_some_and(|set| set.remove(&v))
    }
}

impl Size for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(v)`, where `v` is the digraph's order.
    fn size(&self) -> usize {
        self.arcs.values().map(BTreeSet::len).sum()
    }
}

impl Star for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(v log v)`, where `v` is the digraph's order.
    ///
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn star(order: usize) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        if order == 1 {
            return Self::trivial();
        }

        Self {
            arcs: once((0, (1..order).collect()))
                .chain((1..order).map(|u| (u, BTreeSet::from([0]))))
                .collect(),
        }
    }
}

unsafe fn merge_two_sorted(lhs: &[usize], rhs: &[usize]) -> Vec<usize> {
    unsafe {
        let lhs_len = lhs.len();
        let rhs_len = rhs.len();
        let mut out = Vec::with_capacity(lhs_len + rhs_len);
        let mut i = 0;
        let mut j = 0;

        while i < lhs_len && j < rhs_len {
            let a_i = *lhs.get_unchecked(i);
            let b_j = *rhs.get_unchecked(j);

            match a_i.cmp(&b_j) {
                Ordering::Less => {
                    out.push(a_i);
                    i += 1;
                }
                Ordering::Greater => {
                    out.push(b_j);
                    j += 1;
                }
                Ordering::Equal => {
                    out.push(a_i);
                    i += 1;
                    j += 1;
                }
            }
        }

        while i < lhs.len() {
            out.push(*lhs.get_unchecked(i));
            i += 1;
        }

        while j < rhs.len() {
            out.push(*rhs.get_unchecked(j));
            j += 1;
        }

        out
    }
}

unsafe fn union_sets_unsafe(
    set_a: &BTreeSet<usize>,
    set_b: &BTreeSet<usize>,
) -> BTreeSet<usize> {
    unsafe {
        let vec_a: Vec<usize> = set_a.iter().copied().collect();
        let vec_b: Vec<usize> = set_b.iter().copied().collect();
        let merged = merge_two_sorted(&vec_a, &vec_b);

        merged.into_iter().collect()
    }
}

#[allow(clippy::suspicious_operation_groupings)]
fn find_partition(
    r: usize,
    lhs: &[(usize, BTreeSet<usize>)],
    rhs: &[(usize, BTreeSet<usize>)],
) -> (usize, usize) {
    let lhs_len = lhs.len();
    let rhs_len = rhs.len();
    let mut lo = r.saturating_sub(rhs_len);
    let mut hi = if r < lhs_len { r } else { lhs_len };

    while lo < hi {
        let mid = (lo + hi) >> 1;
        let j = r - mid;

        if j < rhs_len
            && unsafe { lhs.get_unchecked(mid) }.0
                > unsafe { rhs.get_unchecked(j) }.0
        {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }

    (lo, r - lo)
}

impl Union for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O((v1 + v2) log (v1 + v2) + U)`, where `v1` is
    /// the order of `self`, `v2` is the order of `other`, and `U` is the
    /// number of arcs in the union of `self` and `other`.
    fn union(&self, other: &Self) -> Self {
        let lhs_vec = self
            .arcs
            .iter()
            .map(|(&k, v)| (k, v.clone()))
            .collect::<Vec<_>>();

        let rhs_vec = other
            .arcs
            .iter()
            .map(|(&k, v)| (k, v.clone()))
            .collect::<Vec<_>>();

        let n1 = lhs_vec.len();
        let n2 = rhs_vec.len();
        let order = n1 + n2;

        if order == 0 {
            return Self::trivial();
        }

        let lhs_vec = ManuallyDrop::new(lhs_vec);
        let rhs_vec = ManuallyDrop::new(rhs_vec);
        let t = order.min(available_parallelism().map_or(1, NonZero::get));
        let mut partitions = Vec::with_capacity(t + 1);

        for k in 0..=t {
            partitions.push(k * order / t);
        }

        let mut boundaries = Vec::with_capacity(t + 1);
        let lhs_slice = &lhs_vec;
        let rhs_slice = &rhs_vec;

        for &r in &partitions {
            boundaries.push(find_partition(r, lhs_slice, rhs_slice));
        }

        let mut merged_entries = Vec::with_capacity(order);

        scope(|s| {
            let mut handles = Vec::with_capacity(t);

            for k in 0..t {
                let (i_start, j_start) =
                    unsafe { *boundaries.get_unchecked(k) };

                let (i_end, j_end) =
                    unsafe { *boundaries.get_unchecked(k + 1) };

                handles.push(s.spawn(move || {
                    let mut local = Vec::new();

                    unsafe {
                        let lhs_ptr = lhs_slice.as_ptr();
                        let rhs_ptr = rhs_slice.as_ptr();
                        let mut i = i_start;
                        let mut j = j_start;

                        while (i < i_end) || (j < j_end) {
                            if i < i_end && j < j_end {
                                let a = &*lhs_ptr.add(i);
                                let b = &*rhs_ptr.add(j);

                                match a.0.cmp(&b.0) {
                                    Ordering::Less => {
                                        local.push(read(lhs_ptr.add(i)));
                                        i += 1;
                                    }
                                    Ordering::Greater => {
                                        local.push(read(rhs_ptr.add(j)));
                                        j += 1;
                                    }
                                    Ordering::Equal => {
                                        let union_set =
                                            union_sets_unsafe(&a.1, &b.1);

                                        local.push((a.0, union_set));
                                        i += 1;
                                        j += 1;
                                    }
                                }
                            } else if i < i_end {
                                local.push(read(lhs_ptr.add(i)));
                                i += 1;
                            } else {
                                local.push(read(rhs_ptr.add(j)));
                                j += 1;
                            }
                        }
                    }

                    local
                }));
            }

            for h in handles {
                merged_entries.extend(h.join().unwrap());
            }
        });

        merged_entries.sort_unstable_by_key(|&(k, _)| k);

        let mut final_entries = Vec::with_capacity(merged_entries.len());

        if !merged_entries.is_empty() {
            let mut current = merged_entries.remove(0);

            for entry in merged_entries {
                if entry.0 == current.0 {
                    current.1 =
                        unsafe { union_sets_unsafe(&current.1, &entry.1) };
                } else {
                    final_entries.push(current);
                    current = entry;
                }
            }

            final_entries.push(current);
        }

        Self {
            arcs: final_entries.into_iter().collect(),
        }
    }
}

impl Vertices for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity of full iteration is `O(v)`, where `v` is the
    /// digraph's order.
    fn vertices(&self) -> impl Iterator<Item = usize> {
        self.arcs.keys().copied()
    }
}

impl Sources for AdjacencyMap {
    fn sources(&self) -> impl Iterator<Item = usize> {
        self.vertices().filter(move |&u| self.is_source(u))
    }
}

impl Wheel for AdjacencyMap {
    /// # Complexity
    ///
    /// The time complexity is `O(v log v)`, where `v` is the digraph's order.
    ///
    /// # Panics
    ///
    /// Panics if `order` is less than `4`.
    fn wheel(order: usize) -> Self {
        assert!(order >= 4, "a wheel digraph has at least four vertices");

        let last = order - 1;

        Self {
            arcs: once((0, (1..order).collect()))
                .chain(once((1, BTreeSet::from([0, last, 2]))))
                .chain(
                    (2..last).map(|u| (u, BTreeSet::from([0, u - 1, u + 1]))),
                )
                .chain(once((last, BTreeSet::from([0, order - 2, 1]))))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests_add_arc_self_loop {
    use {
        super::*,
        crate::test_add_arc_self_loop,
    };

    test_add_arc_self_loop!(AdjacencyMap);
}

#[cfg(test)]
mod tests_arcs {
    use {
        super::*,
        crate::test_arcs,
    };

    test_arcs!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_biclique {
    use {
        super::*,
        crate::test_biclique,
    };

    test_biclique!(AdjacencyMap);
}

#[cfg(test)]
mod tests_circuit {
    use {
        super::*,
        crate::test_circuit,
    };

    test_circuit!(AdjacencyMap);
}

#[cfg(test)]
mod tests_complete {
    use {
        super::*,
        crate::test_complete,
    };

    test_complete!(AdjacencyMap);
}

#[cfg(test)]
mod tests_converse {
    use {
        super::*,
        crate::test_converse,
    };

    test_converse!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_cycle {
    use {
        super::*,
        crate::test_cycle,
    };

    test_cycle!(AdjacencyMap);
}

#[cfg(test)]
mod tests_degree {
    use {
        super::*,
        crate::test_degree,
    };

    test_degree!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_degree_sequence {
    use {
        super::*,
        crate::test_degree_sequence,
    };

    test_degree_sequence!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_empty {
    use {
        super::*,
        crate::test_empty,
    };

    test_empty!(AdjacencyMap);
}

#[cfg(test)]
mod tests_erdos_renyi {
    use {
        super::*,
        crate::test_erdos_renyi,
    };

    test_erdos_renyi!(AdjacencyMap);
}

#[cfg(test)]
mod tests_has_walk {
    use {
        super::*,
        crate::test_has_walk,
    };

    test_has_walk!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_in_neighbors {
    use {
        super::*,
        crate::test_in_neighbors,
    };

    test_in_neighbors!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_indegree {
    use {
        super::*,
        crate::test_indegree,
    };

    test_indegree!(AdjacencyMap, crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_indegree_sequence {
    use {
        super::*,
        crate::test_indegree_sequence,
    };

    test_indegree_sequence!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_is_balanced {
    use crate::{
        IsBalanced,
        test_is_balanced,
    };

    test_is_balanced!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_is_complete {
    use {
        super::*,
        crate::test_is_complete,
    };

    test_is_complete!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_is_isolated {
    use crate::{
        IsIsolated,
        test_is_isolated,
    };

    test_is_isolated!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_is_oriented {
    use crate::{
        IsOriented,
        test_is_oriented,
    };

    test_is_oriented!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_is_pendant {
    use crate::{
        IsPendant,
        test_is_pendant,
    };

    test_is_pendant!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_is_regular {
    use {
        super::*,
        crate::test_is_regular,
    };

    test_is_regular!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_is_semicomplete {
    use {
        super::*,
        crate::test_is_semicomplete,
    };

    test_is_semicomplete!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_is_simple {
    use {
        super::*,
        crate::test_is_simple,
    };

    test_is_simple!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_is_symmetric {
    use crate::{
        IsSymmetric,
        test_is_symmetric,
    };

    test_is_symmetric!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_is_tournament {
    use {
        super::*,
        crate::test_is_tournament,
    };

    test_is_tournament!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_order {
    use crate::{
        Order,
        test_order,
    };

    test_order!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_out_neighbors {
    use {
        super::*,
        crate::test_out_neighbors,
    };

    test_out_neighbors!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_outdegree {
    use {
        super::*,
        crate::test_outdegree,
    };

    test_outdegree!(AdjacencyMap, crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_path {
    use {
        super::*,
        crate::test_path,
    };

    test_path!(AdjacencyMap);
}

#[cfg(test)]
mod tests_random_recursive_tree {
    use {
        super::*,
        crate::test_random_recursive_tree,
    };

    test_random_recursive_tree!(AdjacencyMap);
}

#[cfg(test)]
mod tests_random_tournament {
    use {
        super::*,
        crate::test_random_tournament,
    };

    test_random_tournament!(AdjacencyMap);
}

#[cfg(test)]
mod tests_remove_arc {
    use {
        super::*,
        crate::test_remove_arc,
    };

    test_remove_arc!(AdjacencyMap, crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_semidegree_sequence {
    use {
        super::*,
        crate::test_semidegree_sequence,
    };

    test_semidegree_sequence!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_sinks {
    use crate::{
        Sinks,
        test_sinks,
    };

    test_sinks!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_size {
    use crate::{
        Size,
        test_size,
    };

    test_size!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_sources {
    use crate::{
        Sources,
        test_sources,
    };

    test_sources!(crate::repr::adjacency_map::fixture);
}

#[cfg(test)]
mod tests_star {
    use {
        super::*,
        crate::test_star,
    };

    test_star!(AdjacencyMap);
}

#[cfg(test)]
mod tests_wheel {
    use {
        super::*,
        crate::test_wheel,
    };

    test_wheel!(AdjacencyMap);
}

#[cfg(test)]
mod proptests_add_arc {
    use {
        super::*,
        crate::proptest_add_arc,
    };

    proptest_add_arc!(AdjacencyMap);
}

#[cfg(test)]
mod proptests_biclique {
    use {
        super::*,
        crate::proptest_biclique,
    };

    proptest_biclique!(AdjacencyMap);
}

#[cfg(test)]
mod proptests_circuit {
    use {
        super::*,
        crate::proptest_circuit,
    };

    proptest_circuit!(AdjacencyMap);
}

#[cfg(test)]
mod proptests_complete {
    use {
        super::*,
        crate::proptest_complete,
    };

    proptest_complete!(AdjacencyMap);
}

#[cfg(test)]
mod proptests_cycle {
    use {
        super::*,
        crate::proptest_cycle,
    };

    proptest_cycle!(AdjacencyMap);
}

#[cfg(test)]
mod proptests_empty {
    use {
        super::*,
        crate::proptest_empty,
    };

    proptest_empty!(AdjacencyMap);
}

#[cfg(test)]
mod proptests_empty_complement {
    use {
        super::*,
        crate::proptest_empty_complement,
    };

    proptest_empty_complement!(AdjacencyMap);
}

#[cfg(test)]
mod proptests_empty_complete {
    use {
        super::*,
        crate::proptest_empty_complete,
    };

    proptest_empty_complete!(AdjacencyMap);
}

#[cfg(test)]
mod proptests_erdos_renyi {
    use {
        super::*,
        crate::proptest_erdos_renyi,
    };

    proptest_erdos_renyi!(AdjacencyMap);
}

#[cfg(test)]
mod proptests_has_arc {
    use {
        super::*,
        crate::proptest_has_arc,
    };

    proptest_has_arc!(AdjacencyMap);
}

#[cfg(test)]
mod proptests_path {
    use {
        super::*,
        crate::proptest_path,
    };

    proptest_path!(AdjacencyMap);
}

#[cfg(test)]
mod proptests_random_recursive_tree {
    use {
        super::*,
        crate::proptest_random_recursive_tree,
    };

    proptest_random_recursive_tree!(AdjacencyMap);
}

#[cfg(test)]
mod proptests_random_tournament {
    use {
        super::*,
        crate::proptest_random_tournament,
    };

    proptest_random_tournament!(AdjacencyMap);
}

#[cfg(test)]
mod proptests_star {
    use {
        super::*,
        crate::proptest_star,
    };

    proptest_star!(AdjacencyMap);
}

#[cfg(test)]
mod proptests_union {
    use {
        super::*,
        crate::proptest_union,
    };

    proptest_union!(AdjacencyMap);
}

#[cfg(test)]
mod proptests_wheel {
    use {
        super::*,
        crate::proptest_wheel,
    };

    proptest_wheel!(AdjacencyMap);
}

#[cfg(test)]
mod tests {
    use super::*;

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
        drop(AdjacencyMap::from(Vec::<BTreeSet<usize>>::new()));
    }

    #[test]
    #[should_panic(expected = "v = 1 isn't in the digraph")]
    fn from_iter_out_of_bounds_v() {
        drop(AdjacencyMap::from([BTreeSet::from([1])]));
    }

    #[test]
    #[should_panic(expected = "u = 0 equals v = 0")]
    fn from_iter_u_equals_v() {
        drop(AdjacencyMap::from([BTreeSet::from([0])]));
    }

    #[test]
    fn is_simple_self_loop() {
        let digraph = AdjacencyMap {
            arcs: BTreeMap::from([(0, BTreeSet::from([0]))]),
        };

        assert!(!digraph.is_simple());
    }
}

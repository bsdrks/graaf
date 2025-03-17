//! Represent sparse unweighted digraphs.
//!
//! An [`AdjacencyList`] is a vector of sets.
//!
//! # Contiguity
//!
//! The vertices are contiguous. The digraph has vertices in the range `[0,
//! v)`, where `v` is the digraph's order.
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
        OutNeighborsWeighted,
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
    },
    std::{
        cmp::Ordering,
        collections::{
            btree_set,
            BTreeSet,
        },
        iter::once,
        marker::PhantomData,
        num::NonZero,
        ptr::write,
        sync::{
            atomic::{
                self,
                AtomicBool,
            },
            Arc,
        },
        thread::{
            available_parallelism,
            scope,
            spawn,
        },
    },
};

/// A representation of an unweighted digraph.
///
/// # Contiguity
///
/// The vertices are contiguous. The digraph has vertices in the range `[0,
/// v)`, where `v` is the digraph's order.
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

/// The weight of an arc is `1`.
static WEIGHT: usize = 1;

impl AddArc for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity is `O(log v)`, where `v` is the digraph's order.
    ///
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

        let _ = unsafe { self.arcs.get_unchecked_mut(u) }.insert(v);
    }
}

impl ArcWeight<usize> for AdjacencyList {
    type Weight = usize;

    fn arc_weight(&self, u: usize, v: usize) -> Option<&Self::Weight> {
        self.has_arc(u, v).then_some(&WEIGHT)
    }
}

#[derive(Clone, Debug)]
struct ArcsIterator<'a> {
    arcs: &'a [BTreeSet<usize>],
    u: usize,
    inner: Option<btree_set::Iter<'a, usize>>,
}

impl Iterator for ArcsIterator<'_> {
    type Item = (usize, usize);

    /// # Complexity
    ///
    /// The time complexity is `O(v + a)`, where `v` is the digraph's order and
    /// `a` is the digraph's size.
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner) = self.inner {
                if let Some(&v) = inner.next() {
                    return Some((self.u - 1, v));
                }
            }

            if self.u >= self.arcs.len() {
                return None;
            }

            self.inner =
                Some(unsafe { self.arcs.get_unchecked(self.u) }.iter());

            self.u += 1;
        }
    }
}

impl Arcs for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity of a full iteration is `O(v + a)`, where `v` is the
    /// digraph's order and `a` is the digraph's size.
    fn arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        ArcsIterator {
            arcs: &self.arcs,
            u: 0,
            inner: None,
        }
    }
}

impl ArcsWeighted for AdjacencyList {
    type Weight = usize;

    /// # Complexity
    ///
    /// The time complexity of a full iteration is `O(v + a)`, where `v` is the
    /// digraph's order and `a` is the digraph's size.
    fn arcs_weighted(
        &self,
    ) -> impl Iterator<Item = (usize, usize, &Self::Weight)> {
        self.arcs().map(move |(u, v)| (u, v, &WEIGHT))
    }
}

impl Biclique for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity is `O(m log m + n log n + m * n)`, where `m` is the
    /// number of vertices in the first partition and `n` is the number of
    /// vertices in the second partition.
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
        let mut arcs = Vec::with_capacity(order);

        arcs.extend(vec![clique_2; m]);
        arcs.extend(vec![clique_1; n]);

        Self { arcs }
    }
}

impl Circuit for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity is `O(v)`, where `v` is the digraph's order.
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
            arcs: (1..=order)
                .map(|u| BTreeSet::from([u % order]))
                .collect::<Vec<_>>(),
        }
    }
}

impl Complement for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity is `O(v^2)`, where `v` is the digraph's order.
    fn complement(&self) -> Self {
        let order = self.order();
        let full = (0..order).collect::<Vec<_>>();
        let full_ptr = full.as_ptr();
        let full_len = full.len();
        let full_ptr_usize = full_ptr as usize;
        let arcs_arc = Arc::new(self.arcs.clone());
        let t = order.min(available_parallelism().map_or(1, NonZero::get));
        let chunk_size = order.div_ceil(t);
        let mut handles = Vec::with_capacity(t);

        for thread_id in 0..t {
            let start = thread_id * chunk_size;
            let end = ((thread_id + 1) * chunk_size).min(order);

            if start >= end {
                break;
            }

            let arcs_arc = Arc::clone(&arcs_arc);

            let handle = spawn(move || {
                let mut partial = Vec::with_capacity(end - start);

                for u in start..end {
                    let out_neighbors = unsafe { arcs_arc.get_unchecked(u) };

                    let vec =
                        out_neighbors.iter().copied().collect::<Vec<_>>();

                    let out_len = vec.len();
                    let out_ptr = vec.as_ptr();
                    let mut diff = Vec::with_capacity(full_len);

                    unsafe {
                        let full_ptr = full_ptr_usize as *const usize;
                        let mut i = 0;
                        let mut j = 0;

                        while i < full_len && j < out_len {
                            let a = *full_ptr.add(i);

                            if a == u {
                                i += 1;

                                continue;
                            }

                            let b = *out_ptr.add(j);

                            match a.cmp(&b) {
                                Ordering::Less => {
                                    diff.push(a);
                                    i += 1;
                                }
                                Ordering::Greater => {
                                    j += 1;
                                }
                                Ordering::Equal => {
                                    i += 1;
                                    j += 1;
                                }
                            }
                        }

                        while i < full_len {
                            let a = *full_ptr.add(i);

                            if a != u {
                                diff.push(a);
                            }

                            i += 1;
                        }
                    }

                    let complement_set = diff.into_iter().collect();

                    partial.push(complement_set);
                }

                partial
            });

            handles.push(handle);
        }

        let mut arcs = Vec::with_capacity(order);

        for handle in handles {
            unsafe {
                arcs.extend(handle.join().unwrap_unchecked());
            }
        }

        Self { arcs }
    }
}

impl Complete for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity is `O(v^2 log v)`, where `v` is the digraph's
    /// order.
    ///
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn complete(order: usize) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        if order == 1 {
            return Self::trivial();
        }

        let t = order.min(available_parallelism().map_or(1, NonZero::get));
        let chunk_size = order.div_ceil(t);
        let mut handles = Vec::with_capacity(t);

        for thread_id in 0..t {
            let start = thread_id * chunk_size;
            let end = ((thread_id + 1) * chunk_size).min(order);

            if start >= end {
                break;
            }

            let handle = spawn(move || {
                let mut local = Vec::with_capacity(end - start);
                let vertices = (0..order).collect::<BTreeSet<_>>();

                for u in start..end {
                    let mut out_neighbors = vertices.clone();
                    let _ = out_neighbors.remove(&u);

                    local.push((u, out_neighbors));
                }

                local
            });

            handles.push(handle);
        }

        let mut arcs = Vec::with_capacity(order);

        for handle in handles {
            unsafe {
                arcs.extend(handle.join().unwrap_unchecked());
            }
        }

        arcs.sort_unstable_by_key(|&(u, _)| u);

        Self {
            arcs: arcs.into_iter().map(|(_, s)| s).collect(),
        }
    }
}

impl Converse for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity is `O(v + a)`, where `v` is the digraph's order and
    /// `a` is the digraph's size.
    ///
    /// # Panics
    ///
    /// Panics if the digraphs' order is zero.
    fn converse(&self) -> Self {
        assert!(self.order() > 0, "a digraph has at least one vertex");

        let order = self.order();
        let mut converse = vec![BTreeSet::new(); order];
        let conv_ptr = converse.as_mut_ptr();

        for (u, set) in self.arcs.iter().enumerate() {
            for &v in set {
                unsafe {
                    let _ = (*conv_ptr.add(v)).insert(u);
                }
            }
        }

        Self { arcs: converse }
    }
}

impl Cycle for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity is `O(v)`, where `v` is the digraph's order.
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
                    BTreeSet::from([(u + order - 1) % order, (u + 1) % order])
                })
                .collect(),
        }
    }
}

impl DegreeSequence for AdjacencyList {
    /// # Complexity
    ///
    /// For sparse digraphs, the time complexity is `O(v * p)`, where `v` is
    /// the digraph's order and `p` is the number of available threads. For
    /// dense digraphs, the time complexity is `O(v^2 log v / p)`.
    fn degree_sequence(&self) -> impl Iterator<Item = usize> {
        let order = self.order();
        let t = available_parallelism().map_or(1, NonZero::get);
        let chunk_size = order.div_ceil(t);
        let mut indegree_chunks = vec![vec![0_usize; order]; t];

        scope(|s| {
            for (chunk, local_indegrees) in
                self.arcs.chunks(chunk_size).zip(indegree_chunks.iter_mut())
            {
                let _ = s.spawn(move || {
                    for out_neighbors in chunk {
                        for &v in out_neighbors {
                            unsafe {
                                *local_indegrees.get_unchecked_mut(v) += 1;
                            }
                        }
                    }
                });
            }
        });

        let mut indegrees = vec![0_usize; order];

        for local_indegrees in indegree_chunks {
            for (vertex, &local_count) in local_indegrees.iter().enumerate() {
                unsafe { *indegrees.get_unchecked_mut(vertex) += local_count };
            }
        }

        (0..order).map(move |u| unsafe {
            indegrees.get_unchecked(u) + self.arcs.get_unchecked(u).len()
        })
    }
}

impl Empty for AdjacencyList {
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

impl ErdosRenyi for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity is `O(v^2)`, where `v` is the digraph's order.
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

impl HasArc for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity is `O(log k)`, where `k` is the out-degree of `u`.
    fn has_arc(&self, u: usize, v: usize) -> bool {
        self.arcs.get(u).is_some_and(|set| set.contains(&v))
    }
}

impl HasEdge for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity is `O(log k + log l)`, where `k` is the out-degree
    /// of `u` and `l` is the out-degree of `v`.
    fn has_edge(&self, u: usize, v: usize) -> bool {
        self.has_arc(u, v) && self.has_arc(v, u)
    }
}

impl HasWalk for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity is `O(w)`, where `w` is the length of `walk`.
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

impl Indegree for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity is `O(v log v)`, where `v` is the digraph's order.
    ///
    /// # Panics
    ///
    /// Panics if `v` isn't in the digraph.
    fn indegree(&self, v: usize) -> usize {
        assert!(v < self.order(), "v = {v} isn't in the digraph");

        self.arcs.iter().filter(|set| set.contains(&v)).count()
    }

    /// # Complexity
    ///
    /// The time complexity is `O(v log v)`, where `v` is the digraph's order.
    fn is_source(&self, v: usize) -> bool {
        self.arcs.iter().all(|set| !set.contains(&v))
    }
}

impl IndegreeSequence for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity is `O(v + a)`, where `v` is the digraph's order and
    /// `a` is the digraph's size.
    fn indegree_sequence(&self) -> impl Iterator<Item = usize> {
        let order = self.order();
        let mut indegrees = vec![0; order];

        unsafe {
            let ptr = indegrees.as_mut_ptr();

            for set in &self.arcs {
                for &v in set {
                    *ptr.add(v) += 1;
                }
            }
        }

        indegrees.into_iter()
    }
}

/// An iterator over the digraph's in-neighbors.
///
/// # Examples
///
/// ```
/// use graaf::{
///     AddArc,
///     AdjacencyList,
///     Empty,
///     InNeighbors,
/// };
///
/// let mut digraph = AdjacencyList::empty(4);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(3, 2);
///
/// let mut in_neighbors = digraph.in_neighbors(2);
///
/// assert_eq!(in_neighbors.next(), Some(1));
/// assert_eq!(in_neighbors.next(), Some(3));
/// assert_eq!(in_neighbors.next(), None);
/// ```
#[derive(Clone, Debug)]
struct InNeighborsIterator<'a> {
    ptr: *const BTreeSet<usize>,
    len: usize,
    i: usize,
    v: usize,
    _marker: PhantomData<&'a BTreeSet<usize>>,
}

impl Iterator for InNeighborsIterator<'_> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            while self.i < self.len {
                let idx = self.i;
                let set = &*self.ptr.add(idx);

                self.i += 1;

                if set.contains(&self.v) {
                    return Some(idx);
                }
            }
        }

        None
    }
}

impl InNeighbors for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity of full iteration is `O(v log v)`, where `v` is the
    /// digraph's order.
    fn in_neighbors(&self, v: usize) -> impl Iterator<Item = usize> {
        InNeighborsIterator {
            ptr: self.arcs.as_ptr(),
            len: self.arcs.len(),
            i: 0,
            v,
            _marker: PhantomData,
        }
    }
}

impl IsComplete for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity is `O(a)`, where `a` is the digraph's size.
    fn is_complete(&self) -> bool {
        let expected_outdegree = self.order() - 1;

        for out_neighbors in &self.arcs {
            if out_neighbors.len() != expected_outdegree {
                return false;
            }
        }

        true
    }
}

impl IsRegular for AdjacencyList {
    /// # Panics
    ///
    /// Panics if the digraph is empty.
    fn is_regular(&self) -> bool {
        let mut semidegrees = self.semidegree_sequence();

        let (u, v) = semidegrees
            .next()
            .expect("a digraph has at least one vertex");

        u == v && semidegrees.all(|(x, y)| x == u && y == v)
    }
}

impl IsSemicomplete for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity is `O(v^2)`, where `v` is the digraph's order.
    fn is_semicomplete(&self) -> bool {
        let order = self.order();

        if order == 1 {
            return true;
        }

        if self.size() < order * (order - 1) / 2 {
            return false;
        }

        let arcs_ptr_usize = self.arcs.as_ptr() as usize;
        let result = Arc::new(AtomicBool::new(true));
        let t = available_parallelism().map_or(1, NonZero::get);
        let chunk_size = order.div_ceil(t);

        scope(|s| {
            for start in (0..order).step_by(chunk_size) {
                let end = order.min(start + chunk_size);
                let result_clone = Arc::clone(&result);

                let _ = s.spawn(move || {
                    let ptr = arcs_ptr_usize as *const BTreeSet<usize>;

                    for u in start..end {
                        if !result_clone.load(atomic::Ordering::Relaxed) {
                            break;
                        }

                        for v in (u + 1)..order {
                            if !result_clone.load(atomic::Ordering::Relaxed) {
                                break;
                            }

                            unsafe {
                                let set_u = &*ptr.add(u);
                                let set_v = &*ptr.add(v);

                                if !set_u.contains(&v) && !set_v.contains(&u) {
                                    result_clone.store(
                                        false,
                                        atomic::Ordering::Relaxed,
                                    );

                                    break;
                                }
                            }
                        }
                    }
                });
            }
        });

        result.load(atomic::Ordering::Relaxed)
    }
}

impl IsSimple for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity is `O(v log v)`, where `v` is the digraph's order.
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
    /// # Complexity
    ///
    /// The time complexity is `O(v^2 log v)`, where `v` is the digraph's
    /// order.
    fn is_tournament(&self) -> bool {
        let order = self.order();

        if self.size() != order * (order - 1) / 2 {
            return false;
        }

        let ptr = self.arcs.as_ptr();

        unsafe {
            for u in 0..order {
                for v in (u + 1)..order {
                    if (*ptr.add(u)).contains(&v) == (*ptr.add(v)).contains(&u)
                    {
                        return false;
                    }
                }
            }
        }

        true
    }
}

impl Order for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity is `O(1)`.
    fn order(&self) -> usize {
        self.arcs.len()
    }
}

impl OutNeighbors for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity of full iteration is `O(log v + outdegree)`, where
    /// `v` is the digraph's order and `outdegree` is the outdegree of `u`.
    ///
    /// # Panics
    ///
    /// Panics if `u` isn't in the digraph.
    fn out_neighbors(&self, u: usize) -> impl Iterator<Item = usize> {
        assert!(u < self.order(), "u = {u} isn't in the digraph");

        unsafe { self.arcs.get_unchecked(u).iter().copied() }
    }
}

impl OutNeighborsWeighted for AdjacencyList {
    type Weight = usize;

    /// # Complexity
    ///
    /// The time complexity of full iteration is `O(log v + outdegree)`, where
    /// `v` is the digraph's order and `outdegree` is the outdegree of `u`.
    ///
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
    /// # Complexity
    ///
    /// The time complexity is `O(1)`.
    ///
    /// # Panics
    ///
    /// Panics if `u` isn't in the digraph.
    fn outdegree(&self, u: usize) -> usize {
        self.arcs.get(u).map_or_else(
            || {
                panic!("u = {u} isn't in the digraph");
            },
            BTreeSet::len,
        )
    }

    /// # Complexity
    ///
    /// The time complexity is `O(1)`.
    ///
    /// # Panics
    ///
    /// Panics if `u` isn't in the digraph.
    fn is_sink(&self, u: usize) -> bool {
        self.arcs.get(u).map_or_else(
            || {
                panic!("u = {u} isn't in the digraph");
            },
            BTreeSet::is_empty,
        )
    }
}

impl Path for AdjacencyList {
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

        Self {
            arcs: (0..order - 1)
                .map(|u| BTreeSet::from([u + 1]))
                .chain(once(BTreeSet::new()))
                .collect(),
        }
    }
}

impl RandomRecursiveTree for AdjacencyList {
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn random_recursive_tree(order: usize, seed: u64) -> Self {
        assert!(order > 0, "a digraph has at least one vertex");

        let mut rng = Xoshiro256StarStar::new(seed);

        if order == 1 {
            return Self::trivial();
        }

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

impl RandomTournament for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity is `O(v^2 log v)`, where `v` is the digraph's
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

        let mut arcs = vec![BTreeSet::new(); order];
        let mut rng = Xoshiro256StarStar::new(seed);

        for u in 0..order {
            for v in (u + 1)..order {
                if rng.next_bool() {
                    let _ = unsafe { arcs.get_unchecked_mut(u).insert(v) };
                } else {
                    let _ = unsafe { arcs.get_unchecked_mut(v).insert(u) };
                }
            }
        }

        Self { arcs }
    }
}

impl RemoveArc for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity is `O(log k)`, where `k` is the out-degree of `u`.
    fn remove_arc(&mut self, u: usize, v: usize) -> bool {
        self.arcs.get_mut(u).is_some_and(|set| set.remove(&v))
    }
}

impl Size for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity is `O(v)` where `v` is the digraph's order.
    fn size(&self) -> usize {
        self.arcs.iter().map(BTreeSet::len).sum()
    }
}

impl Star for AdjacencyList {
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
            arcs: once((1..order).collect())
                .chain((1..order).map(|_| BTreeSet::from([0])))
                .collect(),
        }
    }
}

unsafe fn merge_two_sorted(lhs: &[usize], rhs: &[usize]) -> Vec<usize> {
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

impl Union for AdjacencyList {
    /// The time complexity is `O((v1 + v2) log (v1 + v2) + U)`, where `v1` is
    /// the order of `self`, `v2` is the order of `other`, and `U` is the
    /// number of arcs in the union of `self` and `other`.
    fn union(&self, other: &Self) -> Self {
        let order = self.order().max(other.order());
        let mut arcs: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); order];
        let self_ptr_usize = self.arcs.as_ptr() as usize;
        let other_ptr_usize = other.arcs.as_ptr() as usize;
        let arcs_ptr_usize = arcs.as_mut_ptr() as usize;
        let t = available_parallelism().map_or(1, NonZero::get);
        let chunk_size = order.div_ceil(t);

        scope(|s| {
            for chunk_start in (0..order).step_by(chunk_size) {
                let chunk_end = (chunk_start + chunk_size).min(order);

                let _ = s.spawn(move || unsafe {
                    let self_ptr = self_ptr_usize as *const BTreeSet<usize>;
                    let other_ptr = other_ptr_usize as *const BTreeSet<usize>;
                    let arcs_ptr = arcs_ptr_usize as *mut BTreeSet<usize>;

                    for u in chunk_start..chunk_end {
                        let set_a = if u < self.order() {
                            (*self_ptr.add(u))
                                .iter()
                                .copied()
                                .collect::<Vec<_>>()
                        } else {
                            vec![]
                        };

                        let set_b = if u < other.order() {
                            (*other_ptr.add(u))
                                .iter()
                                .copied()
                                .collect::<Vec<_>>()
                        } else {
                            vec![]
                        };

                        let merged = merge_two_sorted(&set_a, &set_b);

                        write(arcs_ptr.add(u), merged.into_iter().collect());
                    }
                });
            }
        });

        Self { arcs }
    }
}

impl Vertices for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity of full iteration is `O(v)`, where `v` is the
    /// digraph's order.
    fn vertices(&self) -> impl Iterator<Item = usize> {
        0..self.order()
    }
}

impl Wheel for AdjacencyList {
    /// # Complexity
    ///
    /// The time complexity is O(v log v), where v is the digraph's order.
    ///
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

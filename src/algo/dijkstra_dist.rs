//! Dijkstra's algorithm with distances.
//!
//! Dijkstra's algorithm with binary heap finds the shortest path in an
//! arc-weighted digraph.[^1]
//!
//! The time complexity is *O*(*v* log *v* + *a*).
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list_weighted::Digraph,
//!     algo::dijkstra_dist::{
//!         Dijkstra,
//!         Step,
//!     },
//!     gen::Empty,
//!     op::AddArcWeighted,
//! };
//!
//! // 0 -> {1 (2), 2 (3), 3 (4)}
//! // 1 -> {2 (5), 3 (0)}
//! // 2 -> {3 (1)}
//! // 3 -> {}
//!
//! let mut digraph = Digraph::<usize>::empty(4);
//!
//! digraph.add_arc_weighted(0, 1, 2);
//! digraph.add_arc_weighted(0, 2, 3);
//! digraph.add_arc_weighted(0, 3, 4);
//! digraph.add_arc_weighted(1, 2, 5);
//! digraph.add_arc_weighted(1, 3, 0);
//! digraph.add_arc_weighted(2, 3, 1);
//!
//! let mut dijkstra = Dijkstra::new(&digraph, &[0]);
//!
//! assert!(dijkstra.eq([
//!     Step { u: 0, dist: 0 },
//!     Step { u: 1, dist: 2 },
//!     Step { u: 3, dist: 2 },
//!     Step { u: 2, dist: 3 },
//! ]));
//! ```
//!
//! [^1]: Edsger Wybe Dijkstra. 1959. A note on two problems in connexion
//!   with graphs. Numer. Math. 1, 1 (December 1959), 269–271.
//!   <https://doi.org/10.1007/BF01386390>

use {
    crate::op::{
        Order,
        OutNeighborsWeighted,
    },
    core::cmp::Reverse,
    std::collections::{
        BTreeMap,
        BinaryHeap,
    },
};

/// Dijkstra's algorithm with distances.
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list_weighted::Digraph,
///     algo::dijkstra_dist::{
///         Dijkstra,
///         Step,
///     },
///     gen::Empty,
///     op::AddArcWeighted,
/// };
///
/// // 0 -> {1 (2), 2 (3), 3 (4)}
/// // 1 -> {2 (5), 3 (0)}
/// // 2 -> {3 (1)}
/// // 3 -> {}
///
/// let mut digraph = Digraph::<usize>::empty(4);
///
/// digraph.add_arc_weighted(0, 1, 2);
/// digraph.add_arc_weighted(0, 2, 3);
/// digraph.add_arc_weighted(0, 3, 4);
/// digraph.add_arc_weighted(1, 2, 5);
/// digraph.add_arc_weighted(1, 3, 0);
/// digraph.add_arc_weighted(2, 3, 1);
///
/// let mut dijkstra = Dijkstra::new(&digraph, &[0]);
///
/// assert!(dijkstra.eq([
///     Step { u: 0, dist: 0 },
///     Step { u: 1, dist: 2 },
///     Step { u: 3, dist: 2 },
///     Step { u: 2, dist: 3 },
/// ]));
/// ```
#[derive(Clone, Debug)]
pub struct Dijkstra<'a, D> {
    digraph: &'a D,
    dist: Vec<usize>,
    heap: BinaryHeap<(Reverse<usize>, usize)>,
}

impl<'a, D> Dijkstra<'a, D>
where
    D: Order,
{
    /// Constructs a new Dijkstra's algorithm.
    ///
    /// # Arguments
    ///
    /// * `digraph`: The digraph.
    /// * `sources`: The source vertices.
    pub fn new(digraph: &'a D, sources: &[usize]) -> Self {
        let mut dist = vec![usize::MAX; digraph.order()];

        for &u in sources {
            dist[u] = 0;
        }

        Self {
            digraph,
            dist,
            heap: sources.iter().copied().map(|u| (Reverse(0), u)).collect(),
        }
    }

    /// Finds the distances from the source vertices to all other vertices.
    ///
    /// # Panics
    ///
    /// * Panics if `self.next` panics.
    /// * Panics if a source vertex is not in the digraph.
    /// * Panics if a successor vertex is not in the digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list_weighted::Digraph,
    ///     algo::dijkstra_dist::{
    ///         Dijkstra,
    ///         Step,
    ///     },
    ///     gen::Empty,
    ///     op::AddArcWeighted,
    /// };
    ///
    /// // 0 -> {1 (2), 2 (3), 3 (4)}
    /// // 1 -> {2 (5), 3 (0)}
    /// // 2 -> {3 (1)}
    /// // 3 -> {}
    ///
    /// let mut digraph = Digraph::<usize>::empty(4);
    ///
    /// digraph.add_arc_weighted(0, 1, 2);
    /// digraph.add_arc_weighted(0, 2, 3);
    /// digraph.add_arc_weighted(0, 3, 4);
    /// digraph.add_arc_weighted(1, 2, 5);
    /// digraph.add_arc_weighted(1, 3, 0);
    /// digraph.add_arc_weighted(2, 3, 1);
    ///
    /// let mut dijkstra = Dijkstra::new(&digraph, &[0]);
    ///
    /// assert!(dijkstra.distances().into_iter().eq([
    ///     (0, 0),
    ///     (1, 2),
    ///     (2, 3),
    ///     (3, 2),
    /// ]));
    /// ```
    #[must_use]
    pub fn distances(&mut self) -> BTreeMap<usize, usize>
    where
        D: OutNeighborsWeighted<usize>,
    {
        self.map(|Step { u, dist }| (u, dist)).collect()
    }
}

/// A step in Dijkstra's algorithm.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Step {
    /// The current vertex.
    pub u: usize,
    /// The distance of `u` from the source.
    pub dist: usize,
}

impl<'a, D> Iterator for Dijkstra<'a, D>
where
    D: Order + OutNeighborsWeighted<usize>,
{
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((Reverse(dist), u)) = self.heap.pop() {
            for (v, w) in self.digraph.out_neighbors_weighted(u) {
                let dist = dist + w;

                if dist < self.dist[v] {
                    self.dist[v] = dist;
                    self.heap.push((Reverse(dist), v));
                }
            }

            if dist == self.dist[u] {
                return Some(Step { u, dist });
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::adjacency_list_weighted::fixture::{
            bang_jensen_94_usize,
            bang_jensen_96_usize,
            kattis_bryr_1_usize,
            kattis_bryr_2_usize,
            kattis_bryr_3_usize,
            kattis_crosscountry_usize,
            kattis_shortestpath1_usize,
        },
    };

    #[test]
    fn iter_bang_jensen_94() {
        let digraph = bang_jensen_94_usize();

        assert!(Dijkstra::new(&digraph, &[0]).eq([
            Step { u: 0, dist: 0 },
            Step { u: 2, dist: 1 },
            Step { u: 1, dist: 1 },
            Step { u: 5, dist: 2 },
            Step { u: 4, dist: 2 },
            Step { u: 3, dist: 2 },
            Step { u: 6, dist: 3 }
        ]));
    }

    #[test]
    fn iter_bang_jensen_96() {
        let digraph = bang_jensen_96_usize();

        assert!(Dijkstra::new(&digraph, &[0]).eq([
            Step { u: 0, dist: 0 },
            Step { u: 2, dist: 3 },
            Step { u: 4, dist: 4 },
            Step { u: 1, dist: 5 },
            Step { u: 3, dist: 6 },
            Step { u: 5, dist: 7 },
        ]));
    }

    #[test]
    fn iter_kattis_bryr_1() {
        let digraph = kattis_bryr_1_usize();

        assert!(Dijkstra::new(&digraph, &[0]).eq([
            Step { u: 0, dist: 0 },
            Step { u: 2, dist: 1 },
            Step { u: 1, dist: 1 },
        ]));
    }

    #[test]
    fn iter_kattis_bryr_2() {
        let digraph = kattis_bryr_2_usize();

        assert!(Dijkstra::new(&digraph, &[0]).eq([
            Step { u: 0, dist: 0 },
            Step { u: 3, dist: 1 },
            Step { u: 1, dist: 1 },
            Step { u: 4, dist: 2 },
            Step { u: 2, dist: 2 },
            Step { u: 5, dist: 3 },
        ]));
    }

    #[test]
    fn iter_kattis_bryr_3() {
        let digraph = kattis_bryr_3_usize();

        assert!(Dijkstra::new(&digraph, &[0]).eq([
            Step { u: 0, dist: 0 },
            Step { u: 3, dist: 0 },
            Step { u: 7, dist: 0 },
            Step { u: 5, dist: 0 },
            Step { u: 8, dist: 0 },
            Step { u: 4, dist: 0 },
            Step { u: 1, dist: 0 },
            Step { u: 9, dist: 1 },
            Step { u: 6, dist: 1 },
            Step { u: 2, dist: 1 }
        ]));
    }

    #[test]
    fn iter_kattis_crosscountry() {
        let digraph = kattis_crosscountry_usize();

        assert!(Dijkstra::new(&digraph, &[0]).eq([
            Step { u: 0, dist: 0 },
            Step { u: 1, dist: 1 },
            Step { u: 2, dist: 3 },
            Step { u: 3, dist: 10 }
        ]));
    }

    #[test]
    fn iter_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1_usize();

        assert!(Dijkstra::new(&digraph, &[0]).eq([
            Step { u: 0, dist: 0 },
            Step { u: 1, dist: 2 },
            Step { u: 2, dist: 4 }
        ]));
    }

    #[test]
    fn distances_bang_jensen_94() {
        let digraph = bang_jensen_94_usize();

        assert!(Dijkstra::new(&digraph, &[0])
            .distances()
            .values()
            .eq(&[0, 1, 1, 2, 2, 2, 3]));
    }

    #[test]
    fn distances_bang_jensen_96() {
        let digraph = bang_jensen_96_usize();

        assert!(Dijkstra::new(&digraph, &[0])
            .distances()
            .values()
            .eq(&[0, 5, 3, 6, 4, 7]));
    }

    #[test]
    fn distances_kattis_bryr_1() {
        let digraph = kattis_bryr_1_usize();

        assert!(Dijkstra::new(&digraph, &[0])
            .distances()
            .values()
            .eq(&[0, 1, 1]));
    }

    #[test]
    fn distances_kattis_bryr_2() {
        let digraph = kattis_bryr_2_usize();

        assert!(Dijkstra::new(&digraph, &[0])
            .distances()
            .values()
            .eq(&[0, 1, 2, 1, 2, 3]));
    }

    #[test]
    fn distances_kattis_bryr_3() {
        let digraph = kattis_bryr_3_usize();

        assert!(Dijkstra::new(&digraph, &[0])
            .distances()
            .values()
            .eq(&[0, 0, 1, 0, 0, 0, 1, 0, 0, 1]));
    }

    #[test]
    fn distances_kattis_crosscountry() {
        let digraph = kattis_crosscountry_usize();

        assert!(Dijkstra::new(&digraph, &[0])
            .distances()
            .values()
            .eq(&[0, 1, 3, 10]));
    }

    #[test]
    fn distances_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1_usize();

        assert!(Dijkstra::new(&digraph, &[0])
            .distances()
            .values()
            .eq(&[0, 2, 4,]));
    }
}
//! Dijkstra's algorithm
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
//!     algo::dijkstra::Dijkstra,
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
//! assert!(dijkstra.eq([0, 1, 3, 2]));
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
    std::collections::BinaryHeap,
};

/// Dijkstra's algorithm.
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list_weighted::Digraph,
///     algo::dijkstra::Dijkstra,
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
/// assert!(dijkstra.eq([0, 1, 3, 2]));
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
    /// Initializes Dijsktra's algorithm.
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
}

impl<'a, D> Iterator for Dijkstra<'a, D>
where
    D: Order + OutNeighborsWeighted<usize>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((Reverse(distance), u)) = self.heap.pop() {
            for (v, w) in self.digraph.out_neighbors_weighted(u) {
                let distance = distance + w;

                if distance < self.dist[v] {
                    self.dist[v] = distance;
                    self.heap.push((Reverse(distance), v));
                }
            }

            if distance == self.dist[u] {
                return Some(u);
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

        assert!(Dijkstra::new(&digraph, &[0]).eq([0, 2, 1, 5, 4, 3, 6]));
    }

    #[test]
    fn iter_bang_jensen_96() {
        let digraph = bang_jensen_96_usize();

        assert!(Dijkstra::new(&digraph, &[0]).eq([0, 2, 4, 1, 3, 5]));
    }

    #[test]
    fn iter_kattis_bryr_1() {
        let digraph = kattis_bryr_1_usize();

        assert!(Dijkstra::new(&digraph, &[0]).eq([0, 2, 1]));
    }

    #[test]
    fn iter_kattis_bryr_2() {
        let digraph = kattis_bryr_2_usize();

        assert!(Dijkstra::new(&digraph, &[0]).eq([0, 3, 1, 4, 2, 5]));
    }

    #[test]
    fn iter_kattis_bryr_3() {
        let digraph = kattis_bryr_3_usize();

        assert!(
            Dijkstra::new(&digraph, &[0]).eq([0, 3, 7, 5, 8, 4, 1, 9, 6, 2])
        );
    }

    #[test]
    fn iter_kattis_crosscountry() {
        let digraph = kattis_crosscountry_usize();

        assert!(Dijkstra::new(&digraph, &[0]).eq([0, 1, 2, 3]));
    }

    #[test]
    fn iter_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1_usize();

        assert!(Dijkstra::new(&digraph, &[0]).eq([0, 1, 2]));
    }
}

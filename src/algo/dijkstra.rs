//! Dijkstra's algorithm
//!
//! Dijkstra's algorithm with binary heap finds the shortest path in an
//! arc-weighted digraph.[^1]
//!
//! Runs in **O(v log v + a)** time, where **v** is the number of vertices and
//! **a** is the number of arcs.
//!
//! # Examples
//!
//! ## Single source
//!
//! Red marks the path starting at vertex `0` and `t` denotes the iteration
//! index.
//!
//! ![Dijkstra](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dijkstra_1-0.87.4.svg?)
//!
//! ```
//! use graaf::{
//!     dijkstra::Dijkstra,
//!     AddArcWeighted,
//!     AdjacencyListWeighted,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyListWeighted::<usize>::empty(7);
//!
//! digraph.add_arc_weighted(0, 1, 1);
//! digraph.add_arc_weighted(1, 2, 1);
//! digraph.add_arc_weighted(1, 6, 6);
//! digraph.add_arc_weighted(2, 4, 1);
//! digraph.add_arc_weighted(3, 0, 2);
//! digraph.add_arc_weighted(4, 5, 2);
//! digraph.add_arc_weighted(5, 6, 1);
//!
//! let mut dijkstra = Dijkstra::new(&digraph, &[0]);
//!
//! assert!(dijkstra.eq([0, 1, 2, 4, 5, 6]));
//! ```
//!
//! ## Multiple sources
//!
//! Red marks the path starting at vertex `0` and blue the path starting at
//! vertex `3`.
//!
//! ![Dijkstra](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dijkstra_multi_source_1-0.87.4.svg?)
//!
//! ```
//! use graaf::{
//!     dijkstra::Dijkstra,
//!     AddArcWeighted,
//!     AdjacencyListWeighted,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyListWeighted::<usize>::empty(7);
//!
//! digraph.add_arc_weighted(0, 1, 1);
//! digraph.add_arc_weighted(1, 2, 1);
//! digraph.add_arc_weighted(1, 6, 5);
//! digraph.add_arc_weighted(2, 4, 1);
//! digraph.add_arc_weighted(3, 0, 2);
//! digraph.add_arc_weighted(3, 5, 1);
//! digraph.add_arc_weighted(4, 5, 1);
//! digraph.add_arc_weighted(5, 6, 3);
//!
//! let mut dijkstra = Dijkstra::new(&digraph, &[0, 3]);
//!
//! assert!(dijkstra.eq([3, 0, 5, 1, 2, 4, 6]));
//! ```
//!
//! [^1]: Edsger Wybe Dijkstra. 1959. A note on two problems in connexion
//!   with graphs. Numer. Math. 1, 1 (December 1959), 269–271.
//!   <https://doi.org/10.1007/BF01386390>

use {
    crate::{
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
/// ## Single source
///
/// Red marks the path starting at vertex `0` and `t` denotes the iteration
/// index.
///
/// ![Dijkstra](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dijkstra_1-0.87.4.svg?)
///
/// ```
/// use graaf::{
///     dijkstra::Dijkstra,
///     AddArcWeighted,
///     AdjacencyListWeighted,
///     Empty,
/// };
///
/// let mut digraph = AdjacencyListWeighted::<usize>::empty(7);
///
/// digraph.add_arc_weighted(0, 1, 1);
/// digraph.add_arc_weighted(1, 2, 1);
/// digraph.add_arc_weighted(1, 6, 6);
/// digraph.add_arc_weighted(2, 4, 1);
/// digraph.add_arc_weighted(3, 0, 2);
/// digraph.add_arc_weighted(4, 5, 2);
/// digraph.add_arc_weighted(5, 6, 1);
///
/// let mut dijkstra = Dijkstra::new(&digraph, &[0]);
///
/// assert!(dijkstra.eq([0, 1, 2, 4, 5, 6]));
/// ```
///
/// ## Multiple sources
///
/// Red marks the path starting at vertex `0` and blue the path starting at
/// vertex `3`.
///
/// ![Dijkstra](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dijkstra_multi_source_1-0.87.4.svg?)
///
/// ```
/// use graaf::{
///     dijkstra::Dijkstra,
///     AddArcWeighted,
///     AdjacencyListWeighted,
///     Empty,
/// };
///
/// let mut digraph = AdjacencyListWeighted::<usize>::empty(7);
///
/// digraph.add_arc_weighted(0, 1, 1);
/// digraph.add_arc_weighted(1, 2, 1);
/// digraph.add_arc_weighted(1, 6, 5);
/// digraph.add_arc_weighted(2, 4, 1);
/// digraph.add_arc_weighted(3, 0, 2);
/// digraph.add_arc_weighted(3, 5, 1);
/// digraph.add_arc_weighted(4, 5, 1);
/// digraph.add_arc_weighted(5, 6, 1);
///
/// let mut dijkstra = Dijkstra::new(&digraph, &[0, 3]);
///
/// assert!(dijkstra.eq([3, 0, 5, 1, 6, 2, 4]));
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
    pub fn new<'b, T>(digraph: &'a D, sources: T) -> Self
    where
        T: IntoIterator<Item = &'b usize>,
    {
        let mut dist = vec![usize::MAX; digraph.order()];
        let mut heap = BinaryHeap::new();

        for &u in sources {
            dist[u] = 0;
            heap.push((Reverse(0), u));
        }

        Self {
            digraph,
            dist,
            heap,
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
        crate::repr::adjacency_list_weighted::fixture::{
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

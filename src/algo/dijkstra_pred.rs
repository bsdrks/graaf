//! Dijkstra's algorithm with predecessors.
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
//! The path from vertex `0` is red.
//!
//! ![Dijkstra](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dijkstra_pred_1-0.87.4.svg?)
//!
//! ```
//! use graaf::{
//!     AddArcWeighted,
//!     AdjacencyListWeighted,
//!     DijkstraPred,
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
//! let mut dijkstra = DijkstraPred::new(&digraph, &[0]);
//!
//! assert!(dijkstra.eq([
//!     (None, 0),
//!     (Some(0), 1),
//!     (Some(1), 2),
//!     (Some(2), 4),
//!     (Some(4), 5),
//!     (Some(5), 6),
//! ]));
//! ```
//!
//! ## Multiple sources
//!
//! The path from vertex `0` is red. The path from vertex `3` is blue.
//!
//! ![Dijkstra](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dijkstra_pred_multi_source_1-0.87.4.svg?)
//!
//! ```
//! use graaf::{
//!     AddArcWeighted,
//!     AdjacencyListWeighted,
//!     DijkstraPred,
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
//! let mut dijkstra = DijkstraPred::new(&digraph, &[0, 3]);
//!
//! assert!(dijkstra.eq([
//!     (None, 3),
//!     (None, 0),
//!     (Some(3), 5),
//!     (Some(0), 1),
//!     (Some(1), 2),
//!     (Some(2), 4),
//!     (Some(5), 6),
//! ]));
//! ```
//!
//! [^1]: Edsger Wybe Dijkstra. 1959. A note on two problems in connexion with
//!   graphs. Numer. Math. 1, 1 (December 1959), 269–271.
//!   <https://doi.org/10.1007/BF01386390>

use {
    crate::{
        Order,
        OutNeighborsWeighted,
        PredecessorTree,
    },
    core::cmp::Reverse,
    std::collections::BinaryHeap,
};

type Step = (Option<usize>, usize);

/// Dijkstra's algorithm with predecessors.
///
/// # Examples
///
/// ## Single source
///
/// The path from vertex `0` is red.
///
/// ![Dijkstra](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dijkstra_pred_1-0.87.4.svg?)
///
/// ```
/// use graaf::{
///     AddArcWeighted,
///     AdjacencyListWeighted,
///     DijkstraPred,
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
/// let mut dijkstra = DijkstraPred::new(&digraph, &[0]);
///
/// assert!(dijkstra.eq([
///     (None, 0),
///     (Some(0), 1),
///     (Some(1), 2),
///     (Some(2), 4),
///     (Some(4), 5),
///     (Some(5), 6),
/// ]));
/// ```
///
/// ## Multiple sources
///
/// The path from vertex `0` is red. The path from vertex `3` is blue.
///
/// ![Dijkstra](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dijkstra_pred_multi_source_1-0.87.4.svg?)
///
/// ```
/// use graaf::{
///     AddArcWeighted,
///     AdjacencyListWeighted,
///     DijkstraPred,
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
/// digraph.add_arc_weighted(5, 6, 3);
///
/// let mut dijkstra = DijkstraPred::new(&digraph, &[0, 3]);
///
/// assert!(dijkstra.eq([
///     (None, 3),
///     (None, 0),
///     (Some(3), 5),
///     (Some(0), 1),
///     (Some(1), 2),
///     (Some(2), 4),
///     (Some(5), 6),
/// ]));
/// ```
///
/// [^1]: Edsger Wybe Dijkstra. 1959. A note on two problems in connexion with
///   graphs. Numer. Math. 1, 1 (December 1959), 269–271.
///   <https://doi.org/10.1007/BF01386390>
#[derive(Clone, Debug)]
pub struct DijkstraPred<'a, D> {
    digraph: &'a D,
    dist: Vec<usize>,
    heap: BinaryHeap<(Reverse<usize>, Step)>,
}

impl<'a, D> DijkstraPred<'a, D>
where
    D: Order,
{
    /// Initialize Dijkstra's algorithm.
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

            heap.push((Reverse(0), (None, u)));
        }

        Self {
            digraph,
            dist,
            heap,
        }
    }

    /// Find the predecessor tree.
    ///
    /// # Panics
    ///
    /// * Panics if `self.next` panics.
    /// * Panics if a source vertex isn't in the digraph.
    /// * Panics if a successor vertex isn't in the digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArcWeighted,
    ///     AdjacencyListWeighted,
    ///     DijkstraPred,
    ///     Empty,
    /// };
    ///
    /// let mut digraph = AdjacencyListWeighted::<usize>::empty(4);
    ///
    /// digraph.add_arc_weighted(0, 1, 2);
    /// digraph.add_arc_weighted(0, 2, 3);
    /// digraph.add_arc_weighted(0, 3, 4);
    /// digraph.add_arc_weighted(1, 2, 5);
    /// digraph.add_arc_weighted(1, 3, 0);
    /// digraph.add_arc_weighted(2, 3, 1);
    ///
    /// let mut dijkstra = DijkstraPred::new(&digraph, &[0]);
    ///
    /// assert!(dijkstra.predecessors().into_iter().eq([
    ///     None,
    ///     Some(0),
    ///     Some(0),
    ///     Some(1),
    /// ]));
    /// ```
    #[must_use]
    pub fn predecessors(&mut self) -> PredecessorTree
    where
        D: Order + OutNeighborsWeighted<usize>,
    {
        let mut pred = PredecessorTree::new(self.digraph.order());

        for (u, v) in self {
            pred[v] = u;
        }

        pred
    }

    /// Find the shortest path from the source vertices to a target vertex.
    ///
    /// # Arguments
    ///
    /// * `is_target`: The function determining if the vertex is a target.
    ///
    /// # Panics
    ///
    /// * Panics if `is_target` panics.
    /// * Panics if a source vertices isn't in the digraph.
    /// * Panics if a successor vertex isn't in the digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArcWeighted,
    ///     AdjacencyListWeighted,
    ///     DijkstraPred,
    ///     Empty,
    /// };
    ///
    /// let mut digraph = AdjacencyListWeighted::<usize>::empty(4);
    ///
    /// digraph.add_arc_weighted(0, 1, 2);
    /// digraph.add_arc_weighted(0, 2, 3);
    /// digraph.add_arc_weighted(0, 3, 4);
    /// digraph.add_arc_weighted(1, 2, 5);
    /// digraph.add_arc_weighted(1, 3, 0);
    /// digraph.add_arc_weighted(2, 3, 1);
    ///
    /// let mut dijkstra = DijkstraPred::new(&digraph, &[0]);
    ///
    /// assert!(dijkstra.shortest_path(|v| v == 3).unwrap().eq(&[0, 1, 3]));
    /// ```
    #[must_use]
    pub fn shortest_path<P>(&mut self, is_target: P) -> Option<Vec<usize>>
    where
        D: Order + OutNeighborsWeighted<usize>,
        P: Fn(usize) -> bool,
    {
        let mut pred = PredecessorTree::new(self.digraph.order());

        for (u, v) in self {
            pred[v] = u;

            if is_target(v) {
                return pred.search_by(v, |_, b| b.is_none()).map(
                    |mut path| {
                        path.reverse();

                        path
                    },
                );
            }
        }

        None
    }
}

impl<'a, D> Iterator for DijkstraPred<'a, D>
where
    D: Order + OutNeighborsWeighted<usize>,
{
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((Reverse(distance), step)) = self.heap.pop() {
            let (_, v) = step;

            for (x, w) in self.digraph.out_neighbors_weighted(v) {
                let distance = distance + w;

                if distance < self.dist[x] {
                    self.dist[x] = distance;
                    self.heap.push((Reverse(distance), (Some(v), x)));
                }
            }

            if distance == self.dist[v] {
                return Some(step);
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

        assert!(DijkstraPred::new(&digraph, &[0]).eq([
            (None, 0),
            (Some(0), 2),
            (Some(0), 1),
            (Some(2), 5),
            (Some(2), 4),
            (Some(2), 3),
            (Some(4), 6),
        ]));
    }

    #[test]
    fn iter_bang_jensen_96() {
        let digraph = bang_jensen_96_usize();

        assert!(DijkstraPred::new(&digraph, &[0]).eq([
            (None, 0),
            (Some(0), 2),
            (Some(2), 4),
            (Some(2), 1),
            (Some(4), 3),
            (Some(3), 5)
        ]));
    }

    #[test]
    fn iter_kattis_bryr_1() {
        let digraph = kattis_bryr_1_usize();

        assert!(DijkstraPred::new(&digraph, &[0]).eq([
            (None, 0),
            (Some(0), 2),
            (Some(0), 1),
        ]));
    }

    #[test]
    fn iter_kattis_bryr2() {
        let digraph = kattis_bryr_2_usize();

        assert!(DijkstraPred::new(&digraph, &[0]).eq([
            (None, 0),
            (Some(0), 3),
            (Some(0), 1),
            (Some(3), 4),
            (Some(3), 2),
            (Some(4), 5),
        ]));
    }

    #[test]
    fn iter_kattis_bryr_3() {
        let digraph = kattis_bryr_3_usize();

        assert!(DijkstraPred::new(&digraph, &[0]).eq([
            (None, 0),
            (Some(0), 3),
            (Some(3), 7),
            (Some(7), 1),
            (Some(3), 5),
            (Some(5), 8),
            (Some(3), 4),
            (Some(5), 6),
            (Some(6), 2),
            (Some(1), 9)
        ]));
    }

    #[test]
    fn iter_kattis_crosscountry() {
        let digraph = kattis_crosscountry_usize();

        assert!(DijkstraPred::new(&digraph, &[0]).eq([
            (None, 0),
            (Some(0), 1),
            (Some(0), 2),
            (Some(2), 3),
        ]));
    }

    #[test]
    fn iter_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1_usize();

        assert!(DijkstraPred::new(&digraph, &[0]).eq([
            (None, 0),
            (Some(0), 1),
            (Some(1), 2),
        ]));
    }

    #[test]
    fn predecessors_bang_jensen_94() {
        let digraph = bang_jensen_94_usize();

        assert!(DijkstraPred::new(&digraph, &[0])
            .predecessors()
            .into_iter()
            .eq([None, Some(0), Some(0), Some(2), Some(2), Some(2), Some(4)]));
    }

    #[test]
    fn predecessors_bang_jensen_96() {
        let digraph = bang_jensen_96_usize();

        assert!(DijkstraPred::new(&digraph, &[0])
            .predecessors()
            .into_iter()
            .eq([None, Some(2), Some(0), Some(4), Some(2), Some(3)]));
    }

    #[test]
    fn predecessors_kattis_bryr_1() {
        let digraph = kattis_bryr_1_usize();

        assert!(DijkstraPred::new(&digraph, &[0])
            .predecessors()
            .into_iter()
            .eq([None, Some(0), Some(0)]));
    }

    #[test]
    fn predecessors_kattis_bryr_2() {
        let digraph = kattis_bryr_2_usize();

        assert!(DijkstraPred::new(&digraph, &[0])
            .predecessors()
            .into_iter()
            .eq([None, Some(0), Some(3), Some(0), Some(3), Some(4)]));
    }

    #[test]
    fn predecessors_kattis_bryr_3() {
        let digraph = kattis_bryr_3_usize();

        assert!(DijkstraPred::new(&digraph, &[0])
            .predecessors()
            .into_iter()
            .eq([
                None,
                Some(7),
                Some(6),
                Some(0),
                Some(3),
                Some(3),
                Some(5),
                Some(3),
                Some(5),
                Some(1)
            ]));
    }

    #[test]
    fn predecessors_kattis_crosscountry() {
        let digraph = kattis_crosscountry_usize();

        assert!(DijkstraPred::new(&digraph, &[0])
            .predecessors()
            .into_iter()
            .eq([None, Some(0), Some(0), Some(2)]));
    }

    #[test]
    fn predecessors_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1_usize();

        assert!(DijkstraPred::new(&digraph, &[0])
            .predecessors()
            .into_iter()
            .eq([None, Some(0), Some(1), None]));
    }

    #[test]
    fn shortest_path_bang_jensen_94() {
        let digraph = bang_jensen_94_usize();

        assert!(DijkstraPred::new(&digraph, &[0])
            .shortest_path(|v| v == 6)
            .unwrap()
            .eq(&[0, 2, 4, 6]));
    }

    #[test]
    fn shortest_path_bang_jensen_96() {
        let digraph = bang_jensen_96_usize();

        assert!(DijkstraPred::new(&digraph, &[0])
            .shortest_path(|v| v == 5)
            .unwrap()
            .eq(&[0, 2, 4, 3, 5]));
    }

    #[test]
    fn shortest_path_kattis_bryr_1() {
        let digraph = kattis_bryr_1_usize();

        assert!(DijkstraPred::new(&digraph, &[0])
            .shortest_path(|v| v == 2)
            .unwrap()
            .eq(&[0, 2]));
    }

    #[test]
    fn shortest_path_kattis_bryr_2() {
        let digraph = kattis_bryr_2_usize();

        assert!(DijkstraPred::new(&digraph, &[0])
            .shortest_path(|v| v == 5)
            .unwrap()
            .eq(&[0, 3, 4, 5]));
    }

    #[test]
    fn shortest_path_kattis_bryr_3() {
        let digraph = kattis_bryr_3_usize();

        assert!(DijkstraPred::new(&digraph, &[0])
            .shortest_path(|v| v == 9)
            .unwrap()
            .eq(&[0, 3, 7, 1, 9]));
    }

    #[test]
    fn shortest_path_kattis_crosscountry() {
        let digraph = kattis_crosscountry_usize();

        assert!(DijkstraPred::new(&digraph, &[0])
            .shortest_path(|v| v == 2)
            .unwrap()
            .eq(&[0, 2]));
    }

    #[test]
    fn shortest_path_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1_usize();

        assert!(DijkstraPred::new(&digraph, &[0])
            .shortest_path(|v| v == 3)
            .is_none());
    }
}

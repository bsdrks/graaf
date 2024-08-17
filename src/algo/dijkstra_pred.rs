//! Dijkstra's algorithm: vertices and their predecessors.
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
//!     algo::dijkstra_pred::{
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
//!     Step { u: None, v: 0 },
//!     Step { u: Some(0), v: 1 },
//!     Step { u: Some(1), v: 3 },
//!     Step { u: Some(0), v: 2 },
//! ]));
//! ```
//!
//! [^1]: Edsger Wybe Dijkstra. 1959. A note on two problems in connexion
//!   with graphs. Numer. Math. 1, 1 (December 1959), 269–271.
//!   <https://doi.org/10.1007/BF01386390>

use {
    super::PredecessorTree,
    crate::op::{
        Order,
        OutNeighborsWeighted,
    },
    core::cmp::Reverse,
    std::collections::BinaryHeap,
};

/// A step in Dijkstra's algorithm.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Step {
    /// The predecessor vertex of the current vertex, if any. The source
    /// vertices have no predecessor.
    pub u: Option<usize>,
    /// The current vertex.
    pub v: usize,
}

/// Dijkstra's algorithm: vertices and their predecessors.
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list_weighted::Digraph,
///     algo::dijkstra_pred::{
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
///     Step { u: None, v: 0 },
///     Step { u: Some(0), v: 1 },
///     Step { u: Some(1), v: 3 },
///     Step { u: Some(0), v: 2 },
/// ]));
/// ```
#[derive(Clone, Debug)]
pub struct Dijkstra<'a, D> {
    digraph: &'a D,
    dist: Vec<usize>,
    heap: BinaryHeap<(Reverse<usize>, Step)>,
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
            heap: sources
                .iter()
                .copied()
                .map(|v| (Reverse(0), Step { u: None, v }))
                .collect(),
        }
    }

    /// Finds the predecessor tree.
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
    ///     algo::dijkstra_pred::{
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

        for Step { u, v } in self {
            pred[v] = u;
        }

        pred
    }

    /// Finds the shortest path from the source vertices to a target vertex.
    ///
    /// # Arguments
    ///
    /// * `is_target`: The function determining if the vertex is a target.
    ///
    /// # Panics
    ///
    /// * Panics if `is_target` panics.
    /// * Panics if a source vertices is not in `self.digraph`.
    /// * Panics if a successor vertex is not in `self.digraph`.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list_weighted::Digraph,
    ///     algo::dijkstra_pred::{
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
    /// assert!(dijkstra.shortest_path(|v| v == 3).unwrap().eq(&[0, 1, 3]));
    /// ```
    #[must_use]
    pub fn shortest_path<P>(&mut self, is_target: P) -> Option<Vec<usize>>
    where
        D: Order + OutNeighborsWeighted<usize>,
        P: Fn(usize) -> bool,
    {
        let mut pred = PredecessorTree::new(self.digraph.order());

        for Step { u, v } in self {
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

impl<'a, D> Iterator for Dijkstra<'a, D>
where
    D: Order + OutNeighborsWeighted<usize>,
{
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((Reverse(distance), step)) = self.heap.pop() {
            let Step { v, .. } = step;

            for (x, w) in self.digraph.out_neighbors_weighted(v) {
                let distance = distance + w;

                if distance < self.dist[x] {
                    self.dist[x] = distance;
                    self.heap
                        .push((Reverse(distance), Step { u: Some(v), v: x }));
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
            Step { u: None, v: 0 },
            Step { u: Some(0), v: 2 },
            Step { u: Some(0), v: 1 },
            Step { u: Some(2), v: 5 },
            Step { u: Some(2), v: 4 },
            Step { u: Some(2), v: 3 },
            Step { u: Some(4), v: 6 },
        ]));
    }

    #[test]
    fn iter_bang_jensen_96() {
        let digraph = bang_jensen_96_usize();

        assert!(Dijkstra::new(&digraph, &[0]).eq([
            Step { u: None, v: 0 },
            Step { u: Some(0), v: 2 },
            Step { u: Some(2), v: 4 },
            Step { u: Some(2), v: 1 },
            Step { u: Some(4), v: 3 },
            Step { u: Some(3), v: 5 }
        ]));
    }

    #[test]
    fn iter_kattis_bryr_1() {
        let digraph = kattis_bryr_1_usize();

        assert!(Dijkstra::new(&digraph, &[0]).eq([
            Step { u: None, v: 0 },
            Step { u: Some(0), v: 2 },
            Step { u: Some(0), v: 1 },
        ]));
    }

    #[test]
    fn iter_kattis_bryr2() {
        let digraph = kattis_bryr_2_usize();

        assert!(Dijkstra::new(&digraph, &[0]).eq([
            Step { u: None, v: 0 },
            Step { u: Some(0), v: 3 },
            Step { u: Some(0), v: 1 },
            Step { u: Some(3), v: 4 },
            Step { u: Some(3), v: 2 },
            Step { u: Some(4), v: 5 },
        ]));
    }

    #[test]
    fn iter_kattis_bryr_3() {
        let digraph = kattis_bryr_3_usize();

        assert!(Dijkstra::new(&digraph, &[0]).eq([
            Step { u: None, v: 0 },
            Step { u: Some(0), v: 3 },
            Step { u: Some(3), v: 7 },
            Step { u: Some(7), v: 1 },
            Step { u: Some(3), v: 5 },
            Step { u: Some(5), v: 8 },
            Step { u: Some(3), v: 4 },
            Step { u: Some(5), v: 6 },
            Step { u: Some(6), v: 2 },
            Step { u: Some(1), v: 9 }
        ]));
    }

    #[test]
    fn iter_kattis_crosscountry() {
        let digraph = kattis_crosscountry_usize();

        assert!(Dijkstra::new(&digraph, &[0]).eq([
            Step { u: None, v: 0 },
            Step { u: Some(0), v: 1 },
            Step { u: Some(0), v: 2 },
            Step { u: Some(2), v: 3 },
        ]));
    }

    #[test]
    fn iter_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1_usize();

        assert!(Dijkstra::new(&digraph, &[0]).eq([
            Step { u: None, v: 0 },
            Step { u: Some(0), v: 1 },
            Step { u: Some(1), v: 2 },
        ]));
    }

    #[test]
    fn predecessors_bang_jensen_94() {
        let digraph = bang_jensen_94_usize();

        assert!(Dijkstra::new(&digraph, &[0])
            .predecessors()
            .into_iter()
            .eq([None, Some(0), Some(0), Some(2), Some(2), Some(2), Some(4)]));
    }

    #[test]
    fn predecessors_bang_jensen_96() {
        let digraph = bang_jensen_96_usize();

        assert!(Dijkstra::new(&digraph, &[0])
            .predecessors()
            .into_iter()
            .eq([None, Some(2), Some(0), Some(4), Some(2), Some(3)]));
    }

    #[test]
    fn predecessors_kattis_bryr_1() {
        let digraph = kattis_bryr_1_usize();

        assert!(Dijkstra::new(&digraph, &[0])
            .predecessors()
            .into_iter()
            .eq([None, Some(0), Some(0)]));
    }

    #[test]
    fn predecessors_kattis_bryr_2() {
        let digraph = kattis_bryr_2_usize();

        assert!(Dijkstra::new(&digraph, &[0])
            .predecessors()
            .into_iter()
            .eq([None, Some(0), Some(3), Some(0), Some(3), Some(4)]));
    }

    #[test]
    fn predecessors_kattis_bryr_3() {
        let digraph = kattis_bryr_3_usize();

        assert!(Dijkstra::new(&digraph, &[0]).predecessors().into_iter().eq(
            [
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
            ]
        ));
    }

    #[test]
    fn predecessors_kattis_crosscountry() {
        let digraph = kattis_crosscountry_usize();

        assert!(Dijkstra::new(&digraph, &[0])
            .predecessors()
            .into_iter()
            .eq([None, Some(0), Some(0), Some(2)]));
    }

    #[test]
    fn predecessors_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1_usize();

        assert!(Dijkstra::new(&digraph, &[0])
            .predecessors()
            .into_iter()
            .eq([None, Some(0), Some(1), None]));
    }

    #[test]
    fn shortest_path_bang_jensen_94() {
        let digraph = bang_jensen_94_usize();

        assert!(Dijkstra::new(&digraph, &[0])
            .shortest_path(|v| v == 6)
            .unwrap()
            .eq(&[0, 2, 4, 6]));
    }

    #[test]
    fn shortest_path_bang_jensen_96() {
        let digraph = bang_jensen_96_usize();

        assert!(Dijkstra::new(&digraph, &[0])
            .shortest_path(|v| v == 5)
            .unwrap()
            .eq(&[0, 2, 4, 3, 5]));
    }

    #[test]
    fn shortest_path_kattis_bryr_1() {
        let digraph = kattis_bryr_1_usize();

        assert!(Dijkstra::new(&digraph, &[0])
            .shortest_path(|v| v == 2)
            .unwrap()
            .eq(&[0, 2]));
    }

    #[test]
    fn shortest_path_kattis_bryr_2() {
        let digraph = kattis_bryr_2_usize();

        assert!(Dijkstra::new(&digraph, &[0])
            .shortest_path(|v| v == 5)
            .unwrap()
            .eq(&[0, 3, 4, 5]));
    }

    #[test]
    fn shortest_path_kattis_bryr_3() {
        let digraph = kattis_bryr_3_usize();

        assert!(Dijkstra::new(&digraph, &[0])
            .shortest_path(|v| v == 9)
            .unwrap()
            .eq(&[0, 3, 7, 1, 9]));
    }

    #[test]
    fn shortest_path_kattis_crosscountry() {
        let digraph = kattis_crosscountry_usize();

        assert!(Dijkstra::new(&digraph, &[0])
            .shortest_path(|v| v == 2)
            .unwrap()
            .eq(&[0, 2]));
    }

    #[test]
    fn shortest_path_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1_usize();

        assert!(Dijkstra::new(&digraph, &[0])
            .shortest_path(|v| v == 3)
            .is_none());
    }
}

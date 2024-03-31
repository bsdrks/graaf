extern crate alloc;

use {
    crate::ops::{
        CountAllVertices,
        IterEdges,
    },
    alloc::collections::BinaryHeap,
    core::cmp::Reverse,
};

/// Dijkstra's algorithm with binary-heap for unweighted graphs
pub trait DijkstraUnweighted<W> {
    /// Run Dijkstra's algorithm on a directed unweighted graph
    ///
    /// # Arguments
    ///
    /// * `step`: A function that calculates the accumulated distance.
    /// * `dist`: The distances from the source vertices.
    /// * `heap`: The vertices to visit.
    fn dijkstra(
        &self,
        step: fn(W) -> W,
        dist: &mut [W],
        heap: &mut BinaryHeap<(Reverse<W>, usize)>,
    );
}

/// Single-source shortest path
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `s`: The source vertex.
pub fn dijkstra_sssp_unweighted<G>(graph: &G, s: usize) -> Vec<usize>
where
    G: CountAllVertices + DijkstraUnweighted<usize>,
{
    let mut dist = vec![usize::MAX; graph.count_all_vertices()];
    let mut heap = BinaryHeap::from([(Reverse(0), s)]);

    dist[s] = 0;

    graph.dijkstra(|w| w + 1, &mut dist, &mut heap);

    dist
}

impl<W, T> DijkstraUnweighted<W> for T
where
    W: Copy + Ord,
    T: IterEdges,
{
    fn dijkstra(
        &self,
        step: fn(W) -> W,
        dist: &mut [W],
        heap: &mut BinaryHeap<(Reverse<W>, usize)>,
    ) {
        while let Some((Reverse(w), s)) = heap.pop() {
            let w = step(w);

            for t in self.iter_edges(s) {
                if w >= dist[t] {
                    continue;
                }

                dist[t] = w;
                heap.push((Reverse(w), t));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_graph() {
        let graph: [Vec<usize>; 0] = [];
        let mut dist = Vec::new();
        let mut heap = BinaryHeap::new();
        let () = graph.dijkstra(|w: usize| w + 1, &mut dist, &mut heap);

        assert!(dist.is_empty());
    }

    #[test]
    fn small_graph1() {
        let graph: [Vec<usize>; 8] = [
            vec![1, 3],
            vec![0, 2],
            vec![1],
            vec![0, 4, 7],
            vec![3, 5, 6, 7],
            vec![4, 6],
            vec![4, 5, 7],
            vec![3, 4, 6],
        ];

        for (i, &d) in [
            [0, 1, 2, 1, 2, 3, 3, 2],
            [1, 0, 1, 2, 3, 4, 4, 3],
            [2, 1, 0, 3, 4, 5, 5, 4],
            [1, 2, 3, 0, 1, 2, 2, 1],
            [2, 3, 4, 1, 0, 1, 1, 1],
            [3, 4, 5, 2, 1, 0, 1, 2],
            [3, 4, 5, 2, 1, 1, 0, 1],
            [2, 3, 4, 1, 1, 2, 1, 0],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(dijkstra_sssp_unweighted(&graph, i), d);
        }
    }

    #[test]
    fn small_graph2() {
        let graph: [Vec<usize>; 4] = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        for (i, &d) in [[0, 1, 3, 2], [1, 0, 2, 1], [2, 1, 0, 1], [2, 3, 1, 0]]
            .iter()
            .enumerate()
        {
            assert_eq!(dijkstra_sssp_unweighted(&graph, i), d);
        }
    }
}

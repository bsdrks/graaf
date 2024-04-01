extern crate alloc;

use {
    crate::ops::{
        CountAllVertices,
        IterWeightedEdges,
    },
    alloc::collections::BinaryHeap,
    core::cmp::Reverse,
};

/// Dijkstra's algorithm with binary-heap for weighted graphs
pub trait DijkstraWeighted<W> {
    /// Return the minimum distances from the source vertices to all other
    /// vertices.
    ///
    /// # Arguments
    ///
    /// * `step`: A function that calculates the accumulated weight.
    /// * `dist`: The distances from the source vertices.
    /// * `heap`: The vertices to visit.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate alloc;
    ///
    /// use {
    ///     alloc::collections::BinaryHeap,
    ///     core::cmp::Reverse,
    ///     graaf::algo::DijkstraWeighted,
    /// };
    ///
    /// // ╭───╮       ╭───╮
    /// // │ 0 │  2 →  │ 1 │
    /// // ╰───╯       ╰───╯
    /// //   ↑           2
    /// //   2           ↓
    /// // ╭───╮       ╭───╮
    /// // │ 3 │       │ 2 │
    /// // ╰───╯       ╰───╯
    ///
    /// let graph: [Vec<(usize, usize)>; 4] = [
    ///     vec![(1, 2)],
    ///     vec![(2, 2)],
    ///     Vec::new(),
    ///     vec![(0, 2)],
    /// ];
    ///
    /// let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
    /// let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
    ///
    /// graph.min_distances(|acc, w| acc + w, &mut dist, &mut heap);
    ///
    /// assert_eq!(dist, [0, 2, 4, usize::MAX]);
    /// ```
    fn min_distances(
        &self,
        step: fn(W, W) -> W,
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
///
/// # Example
///
/// ```
/// use graaf::algo::dijkstra_sssp_weighted;
///
/// // ╭───╮       ╭───╮
/// // │ 0 │  2 →  │ 1 │
/// // ╰───╯       ╰───╯
/// //   ↑           2
/// //   2           ↓
/// // ╭───╮       ╭───╮
/// // │ 3 │       │ 2 │
/// // ╰───╯       ╰───╯
///
/// let graph: [Vec<(usize, usize)>; 4] = [
///     vec![(1, 2)],
///     vec![(2, 2)],
///     Vec::new(),
///     vec![(0, 2)],
/// ];
///
/// assert_eq!(dijkstra_sssp_weighted(&graph, 0), [0, 2, 4, usize::MAX]);
/// ```
pub fn dijkstra_sssp_weighted<G>(graph: &G, s: usize) -> Vec<usize>
where
    G: CountAllVertices + DijkstraWeighted<usize>,
{
    let mut dist = vec![usize::MAX; graph.count_all_vertices()];
    let mut heap = BinaryHeap::from([(Reverse(0), s)]);

    dist[s] = 0;

    graph.min_distances(|acc, w| acc + w, &mut dist, &mut heap);

    dist
}

impl<W, T> DijkstraWeighted<W> for T
where
    T: IterWeightedEdges<W>,
    W: Copy + Ord,
{
    fn min_distances(
        &self,
        step: fn(W, W) -> W,
        dist: &mut [W],
        heap: &mut BinaryHeap<(Reverse<W>, usize)>,
    ) {
        while let Some((Reverse(acc), s)) = heap.pop() {
            for (t, w) in self.iter_weighted_edges(s) {
                let w = step(acc, w);

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
    use {
        super::*,
        crate::ops::AddWeightedEdge,
    };

    #[test]
    fn shortestpath1() {
        let graph: [Vec<(usize, usize)>; 4] =
            [vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]];

        for (i, &d) in [
            [0, 2, 4, usize::MAX],
            [usize::MAX, 0, 2, usize::MAX],
            [usize::MAX, usize::MAX, 0, usize::MAX],
            [2, 4, 6, 0],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(dijkstra_sssp_weighted(&graph, i), d);
        }
    }

    #[test]
    fn crosscountry() {
        let graph: [Vec<(usize, usize)>; 4] = [
            vec![(1, 1), (2, 3), (3, 14)],
            vec![(0, 2), (2, 4), (3, 22)],
            vec![(0, 3), (1, 10), (3, 7)],
            vec![(0, 13), (1, 8), (2, 2)],
        ];

        for (i, &d) in [[0, 1, 3, 10], [2, 0, 4, 11], [3, 4, 0, 7], [5, 6, 2, 0]]
            .iter()
            .enumerate()
        {
            assert_eq!(dijkstra_sssp_weighted(&graph, i), d);
        }
    }

    #[test]
    fn small_graph1() {
        let graph: [Vec<(usize, usize)>; 9] = [
            vec![(1, 4), (7, 8)],
            vec![(0, 4), (2, 8), (7, 11)],
            vec![(1, 8), (3, 7), (5, 4), (8, 2)],
            vec![(2, 7), (4, 9), (5, 14)],
            vec![(3, 9), (5, 10)],
            vec![(2, 4), (3, 14), (4, 10), (6, 2)],
            vec![(5, 2), (7, 1), (8, 6)],
            vec![(0, 8), (1, 11), (6, 1), (8, 7)],
            vec![(2, 2), (6, 6), (7, 7)],
        ];

        for (i, &d) in [
            [0, 4, 12, 19, 21, 11, 9, 8, 14],
            [4, 0, 8, 15, 22, 12, 12, 11, 10],
            [12, 8, 0, 7, 14, 4, 6, 7, 2],
            [19, 15, 7, 0, 9, 11, 13, 14, 9],
            [21, 22, 14, 9, 0, 10, 12, 13, 16],
            [11, 12, 4, 11, 10, 0, 2, 3, 6],
            [9, 12, 6, 13, 12, 2, 0, 1, 6],
            [8, 11, 7, 14, 13, 3, 1, 0, 7],
            [14, 10, 2, 9, 16, 6, 6, 7, 0],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(dijkstra_sssp_weighted(&graph, i), d);
        }
    }

    #[test]
    fn bryr1() {
        let mut graph = vec![Vec::new(); 3];

        for (s, t, w) in [(2, 0, 1), (0, 1, 1), (1, 2, 1)] {
            graph.add_weighted_edge(s, t, w);
            graph.add_weighted_edge(t, s, w);
        }

        for (s, dist) in [[0, 1, 1], [1, 0, 1], [1, 1, 0]].iter().enumerate() {
            assert_eq!(dijkstra_sssp_weighted(&graph, s), dist);
        }
    }

    #[test]
    fn bryr2() {
        let mut graph = vec![Vec::new(); 6];

        for (s, t, w) in [
            (4, 5, 1),
            (4, 3, 1),
            (1, 0, 1),
            (1, 2, 1),
            (3, 2, 1),
            (0, 3, 1),
        ] {
            graph.add_weighted_edge(s, t, w);
            graph.add_weighted_edge(t, s, w);
        }

        for (s, dist) in [
            [0, 1, 2, 1, 2, 3],
            [1, 0, 1, 2, 3, 4],
            [2, 1, 0, 1, 2, 3],
            [1, 2, 1, 0, 1, 2],
            [2, 3, 2, 1, 0, 1],
            [3, 4, 3, 2, 1, 0],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(dijkstra_sssp_weighted(&graph, s), dist);
        }
    }

    #[test]
    fn bryr3() {
        let mut graph = vec![Vec::new(); 10];

        for (s, t, w) in [
            (6, 2, 0),
            (6, 9, 1),
            (7, 1, 0),
            (9, 1, 1),
            (3, 5, 0),
            (3, 0, 0),
            (8, 4, 1),
            (5, 8, 0),
            (6, 5, 1),
            (2, 9, 0),
            (3, 4, 0),
            (4, 6, 1),
            (3, 7, 0),
        ] {
            graph.add_weighted_edge(s, t, w);
            graph.add_weighted_edge(t, s, w);
        }

        for (s, dist) in [
            [0, 0, 1, 0, 0, 0, 1, 0, 0, 1],
            [0, 0, 1, 0, 0, 0, 1, 0, 0, 1],
            [1, 1, 0, 1, 1, 1, 0, 1, 1, 0],
            [0, 0, 1, 0, 0, 0, 1, 0, 0, 1],
            [0, 0, 1, 0, 0, 0, 1, 0, 0, 1],
            [0, 0, 1, 0, 0, 0, 1, 0, 0, 1],
            [1, 1, 0, 1, 1, 1, 0, 1, 1, 0],
            [0, 0, 1, 0, 0, 0, 1, 0, 0, 1],
            [0, 0, 1, 0, 0, 0, 1, 0, 0, 1],
            [1, 1, 0, 1, 1, 1, 0, 1, 1, 0],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(dijkstra_sssp_weighted(&graph, s), dist);
        }
    }
}

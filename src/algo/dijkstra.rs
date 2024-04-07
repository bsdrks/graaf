//! Dijkstra's algorithm with binary-heap
//!
//! # Examples
//!
//! ```
//! use graaf::algo::dijkstra::predecessors_single_source;
//!
//! // ╭───╮       ╭───╮
//! // │ 0 │  2 →  │ 1 │
//! // ╰───╯       ╰───╯
//! //   ↑           2
//! //   2           ↓
//! // ╭───╮       ╭───╮
//! // │ 3 │       │ 2 │
//! // ╰───╯       ╰───╯
//!
//! let graph = [vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]];
//! let (pred, dist) = predecessors_single_source(&graph, 0);
//!
//! assert_eq!(pred, [None, Some(0), Some(1), None]);
//! assert_eq!(dist, [0, 2, 4, usize::MAX]);
//!
//! let (pred, dist) = predecessors_single_source(&graph, 1);
//!
//! assert_eq!(pred, [None, None, Some(1), None]);
//! assert_eq!(dist, [usize::MAX, 0, 2, usize::MAX]);
//!
//! let (pred, dist) = predecessors_single_source(&graph, 2);
//!
//! assert_eq!(pred, [None, None, None, None]);
//! assert_eq!(dist, [usize::MAX, usize::MAX, 0, usize::MAX]);
//!
//! let (pred, dist) = predecessors_single_source(&graph, 3);
//!
//! assert_eq!(pred, [Some(3), Some(0), Some(1), None]);
//! assert_eq!(dist, [2, 4, 6, 0]);
//! ```

extern crate alloc;

use {
    crate::op::{
        CountAllVertices,
        IterWeightedEdges,
    },
    alloc::collections::BinaryHeap,
    core::cmp::Reverse,
};

/// Calculate the minimum distances from the source vertices to all vertices in
/// a weighted directed graph. Use [`predecessors`] if you also need
/// the predecessor tree.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `step`: The function that calculates the accumulated weight.
/// * `dist`: The distances from the source vertices.
/// * `heap`: The vertices to visit.
///
/// # Examples
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BinaryHeap,
///     core::cmp::Reverse,
///     graaf::algo::dijkstra::min_distances,
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
/// let graph = [vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]];
/// let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
/// let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
///
/// min_distances(&graph, |acc, w| acc + w, &mut dist, &mut heap);
///
/// assert_eq!(dist, [0, 2, 4, usize::MAX]);
/// ```
pub fn min_distances<G, W>(
    graph: &G,
    step: fn(W, W) -> W,
    dist: &mut [W],
    heap: &mut BinaryHeap<(Reverse<W>, usize)>,
) where
    G: CountAllVertices + IterWeightedEdges<W>,
    W: Copy + Ord,
{
    while let Some((Reverse(acc), s)) = heap.pop() {
        for (t, w) in graph.iter_weighted_edges(s) {
            let w = step(acc, *w);

            if w >= dist[*t] {
                continue;
            }

            dist[*t] = w;
            heap.push((Reverse(w), *t));
        }
    }
}

/// Calculate the minimum distances from the source vertex to all vertices in a
/// weighted directed graph. Use [`predecessors_single_source`] if you also need
/// the predecessor tree.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `s`: The source vertex.
///
/// # Examples
///
/// ```
/// use graaf::algo::dijkstra::min_distances_single_source;
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
/// let graph: [Vec<(usize, usize)>; 4] = [vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]];
///
/// assert_eq!(
///     min_distances_single_source(&graph, 0),
///     [0, 2, 4, usize::MAX]
/// );
/// ```
pub fn min_distances_single_source<G>(graph: &G, s: usize) -> Vec<usize>
where
    G: CountAllVertices + IterWeightedEdges<usize>,
{
    let mut dist = vec![usize::MAX; graph.count_all_vertices()];
    let mut heap = BinaryHeap::from([(Reverse(0), s)]);

    dist[s] = 0;

    min_distances(graph, |acc, w| acc + w, &mut dist, &mut heap);

    dist
}

/// Calculate the predecessor tree and distances of the shortest paths from the
/// source vertices to all vertices in a weighted directed graph.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `step`: The function that calculates the accumulated weight.
/// * `pred`: The predecessors on the shortest paths from the source vertices.
/// * `dist`: The distances from the source vertices.
/// * `heap`: The vertices to visit.
///
/// # Examples
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BinaryHeap,
///     core::cmp::Reverse,
///     graaf::algo::dijkstra::predecessors,
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
/// let graph = [vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]];
/// let mut pred = [None, None, None, None];
/// let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
/// let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
///
/// predecessors(&graph, |acc, w| acc + w, &mut pred, &mut dist, &mut heap);
///
/// assert_eq!(pred, [None, Some(0), Some(1), None]);
/// assert_eq!(dist, [0, 2, 4, usize::MAX]);
/// ```
pub fn predecessors<G, W>(
    graph: &G,
    step: fn(W, &W) -> W,
    pred: &mut [Option<usize>],
    dist: &mut [W],
    heap: &mut BinaryHeap<(Reverse<W>, usize)>,
) where
    G: CountAllVertices + IterWeightedEdges<W>,
    W: Copy + Ord,
{
    while let Some((Reverse(acc), s)) = heap.pop() {
        for (t, w) in graph.iter_weighted_edges(s) {
            let w = step(acc, w);

            if w >= dist[*t] {
                continue;
            }

            dist[*t] = w;
            pred[*t] = Some(s);
            heap.push((Reverse(w), *t));
        }
    }
}

/// Calculate the predecessor tree and distances of the shortest paths from the
/// source vertex to all vertices in a weighted directed graph.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `s`: The source vertex.
///
/// # Examples
///
/// ```
/// use graaf::algo::dijkstra::predecessors_single_source;
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
/// let graph = [vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]];
/// let (pred, dist) = predecessors_single_source(&graph, 0);
///
/// assert_eq!(pred, [None, Some(0), Some(1), None]);
/// assert_eq!(dist, [0, 2, 4, usize::MAX]);
/// ```
pub fn predecessors_single_source<G>(graph: &G, s: usize) -> (Vec<Option<usize>>, Vec<usize>)
where
    G: CountAllVertices + IterWeightedEdges<usize>,
{
    let v = graph.count_all_vertices();
    let mut pred = vec![None; v];
    let mut dist = vec![usize::MAX; v];
    let mut heap = BinaryHeap::from([(Reverse(0), s)]);

    dist[s] = 0;

    predecessors(graph, |acc, w| acc + w, &mut pred, &mut dist, &mut heap);

    (pred, dist)
}

#[cfg(test)]
mod test {
    use super::*;

    const GRAPH_0: [&[(usize, usize)]; 0] = [];

    const GRAPH_1: [&[(usize, usize)]; 9] = [
        &[(1, 4), (7, 8)],
        &[(0, 4), (2, 8), (7, 11)],
        &[(1, 8), (3, 7), (5, 4), (8, 2)],
        &[(2, 7), (4, 9), (5, 14)],
        &[(3, 9), (5, 10)],
        &[(2, 4), (3, 14), (4, 10), (6, 2)],
        &[(5, 2), (7, 1), (8, 6)],
        &[(0, 8), (1, 11), (6, 1), (8, 7)],
        &[(2, 2), (6, 6), (7, 7)],
    ];

    const GRAPH_SHORTEST_PATH_1: [&[(usize, usize)]; 4] = [&[(1, 2)], &[(2, 2)], &[], &[(0, 2)]];

    const GRAPH_CROSS_COUNTRY: [&[(usize, usize)]; 4] = [
        &[(1, 1), (2, 3), (3, 14)],
        &[(0, 2), (2, 4), (3, 22)],
        &[(0, 3), (1, 10), (3, 7)],
        &[(0, 13), (1, 8), (2, 2)],
    ];

    const EDGES_BRYR_1: [(usize, usize, usize); 3] = [(2, 0, 1), (0, 1, 1), (1, 2, 1)];

    const EDGES_BRYR_2: [(usize, usize, usize); 6] = [
        (4, 5, 1),
        (4, 3, 1),
        (1, 0, 1),
        (1, 2, 1),
        (3, 2, 1),
        (0, 3, 1),
    ];

    const EDGES_BRYR_3: [(usize, usize, usize); 13] = [
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
    ];

    fn to_vec<T>(graph: &[&[T]]) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        graph.iter().map(|v| v.to_vec()).collect()
    }

    mod min_distances {
        use {
            super::*,
            crate::op::AddWeightedEdge,
        };

        #[test]
        fn graph_0() {
            let graph = to_vec(&GRAPH_0);
            let mut dist = Vec::new();
            let mut heap = BinaryHeap::new();

            min_distances(&graph, |acc, w| acc + w, &mut dist, &mut heap);

            assert!(dist.is_empty());
        }

        #[test]
        fn graph_1() {
            let graph = to_vec(&GRAPH_1);

            let mut dist = [
                0,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
            ];

            let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

            min_distances(&graph, |acc, w| acc + w, &mut dist, &mut heap);

            assert_eq!(dist, [0, 4, 12, 19, 21, 11, 9, 8, 14]);
        }

        #[test]
        fn graph_shortest_path_1() {
            let graph = to_vec(&GRAPH_SHORTEST_PATH_1);
            let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
            let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

            min_distances(&graph, |acc, w| acc + w, &mut dist, &mut heap);

            assert_eq!(dist, [0, 2, 4, usize::MAX]);
        }

        #[test]
        fn graph_cross_country() {
            let graph = to_vec(&GRAPH_CROSS_COUNTRY);
            let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
            let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

            min_distances(&graph, |acc, w| acc + w, &mut dist, &mut heap);

            assert_eq!(dist, [0, 1, 3, 10]);
        }

        #[test]
        fn graph_bryr_1() {
            let mut graph = vec![Vec::new(); 3];

            for (s, t, w) in EDGES_BRYR_1 {
                graph.add_weighted_edge(s, t, w);
                graph.add_weighted_edge(t, s, w);
            }

            let mut dist = [0, usize::MAX, usize::MAX];
            let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

            min_distances(&graph, |acc, w| acc + w, &mut dist, &mut heap);

            assert_eq!(dist, [0, 1, 1]);
        }

        #[test]
        fn graph_bryr_2() {
            let mut graph = vec![Vec::new(); 6];

            for (s, t, w) in EDGES_BRYR_2 {
                graph.add_weighted_edge(s, t, w);
                graph.add_weighted_edge(t, s, w);
            }

            let mut dist = [
                0,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
            ];

            let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

            min_distances(&graph, |acc, w| acc + w, &mut dist, &mut heap);

            assert_eq!(dist, [0, 1, 2, 1, 2, 3]);
        }

        #[test]
        fn graph_bryr_3() {
            let mut graph = vec![Vec::new(); 10];

            for (s, t, w) in EDGES_BRYR_3 {
                graph.add_weighted_edge(s, t, w);
                graph.add_weighted_edge(t, s, w);
            }

            let mut dist = [
                0,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
            ];

            let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

            min_distances(&graph, |acc, w| acc + w, &mut dist, &mut heap);

            assert_eq!(dist, [0, 0, 1, 0, 0, 0, 1, 0, 0, 1,]);
        }
    }

    mod min_distances_single_source {
        use {
            super::*,
            crate::op::AddWeightedEdge,
        };

        #[test]
        #[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
        fn graph_0() {
            let graph = to_vec(&GRAPH_0);
            let _ = min_distances_single_source(&graph, 0);
        }

        #[test]
        fn graph_1() {
            #[rustfmt::skip]
            const EXPECTED: [[usize; 9]; 9] = [
                [ 0,  4, 12, 19, 21, 11,  9,  8, 14],
                [ 4,  0,  8, 15, 22, 12, 12, 11, 10],
                [12,  8,  0,  7, 14,  4,  6,  7,  2],
                [19, 15,  7,  0,  9, 11, 13, 14,  9],
                [21, 22, 14,  9,  0, 10, 12, 13, 16],
                [11, 12,  4, 11, 10,  0,  2,  3,  6],
                [ 9, 12,  6, 13, 12,  2,  0,  1,  6],
                [ 8, 11,  7, 14, 13,  3,  1,  0,  7],
                [14, 10,  2,  9, 16,  6,  6,  7,  0],
            ];

            let graph = to_vec(&GRAPH_1);

            for (i, &d) in EXPECTED.iter().enumerate() {
                assert_eq!(min_distances_single_source(&graph, i), d);
            }
        }

        #[test]
        fn graph_shortest_path_1() {
            const M: usize = usize::MAX;

            #[rustfmt::skip]
            const EXPECTED: [[usize; 4]; 4] = [
                [0, 2, 4, M],
                [M, 0, 2, M],
                [M, M, 0, M],
                [2, 4, 6, 0],
            ];

            let graph = to_vec(&GRAPH_SHORTEST_PATH_1);

            for (i, &d) in EXPECTED.iter().enumerate() {
                assert_eq!(min_distances_single_source(&graph, i), d);
            }
        }

        #[test]
        fn graph_cross_country() {
            #[rustfmt::skip]
            const EXPECTED: [[usize; 4]; 4] = [
                [ 0,  1,  3, 10],
                [ 2,  0,  4, 11],
                [ 3,  4,  0,  7],
                [ 5,  6,  2,  0],
            ];

            let graph = to_vec(&GRAPH_CROSS_COUNTRY);

            for (i, &d) in EXPECTED.iter().enumerate() {
                assert_eq!(min_distances_single_source(&graph, i), d);
            }
        }

        #[test]
        fn graph_bryr_1() {
            #[rustfmt::skip]
            const EXPECTED: [[usize; 3]; 3] = [
                [0, 1, 1],
                [1, 0, 1],
                [1, 1, 0],
            ];

            let mut graph = vec![Vec::new(); 3];

            for (s, t, w) in EDGES_BRYR_1 {
                graph.add_weighted_edge(s, t, w);
                graph.add_weighted_edge(t, s, w);
            }

            for (s, dist) in EXPECTED.iter().enumerate() {
                assert_eq!(min_distances_single_source(&graph, s), dist);
            }
        }

        #[test]
        fn graph_bryr_2() {
            #[rustfmt::skip]
            const EXPECTED: [[usize; 6]; 6] = [
                [0, 1, 2, 1, 2, 3],
                [1, 0, 1, 2, 3, 4],
                [2, 1, 0, 1, 2, 3],
                [1, 2, 1, 0, 1, 2],
                [2, 3, 2, 1, 0, 1],
                [3, 4, 3, 2, 1, 0],
            ];

            let mut graph = vec![Vec::new(); 6];

            for (s, t, w) in EDGES_BRYR_2 {
                graph.add_weighted_edge(s, t, w);
                graph.add_weighted_edge(t, s, w);
            }

            for (s, dist) in EXPECTED.iter().enumerate() {
                assert_eq!(min_distances_single_source(&graph, s), dist);
            }
        }

        #[test]
        fn graph_bryr_3() {
            #[rustfmt::skip]
            const EXPECTED: [[usize; 10]; 10] = [
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
            ];

            let mut graph = vec![Vec::new(); 10];

            for (s, t, w) in EDGES_BRYR_3 {
                graph.add_weighted_edge(s, t, w);
                graph.add_weighted_edge(t, s, w);
            }

            for (s, dist) in EXPECTED.iter().enumerate() {
                assert_eq!(min_distances_single_source(&graph, s), dist);
            }
        }
    }

    mod predecessors {
        use {
            super::*,
            crate::op::AddWeightedEdge,
        };

        #[test]
        fn graph_0() {
            let graph = to_vec(&GRAPH_0);
            let mut pred = Vec::new();
            let mut dist = Vec::new();
            let mut heap = BinaryHeap::new();

            predecessors(&graph, |acc, w| acc + w, &mut pred, &mut dist, &mut heap);

            assert!(pred.is_empty());
            assert!(dist.is_empty());
        }

        #[test]
        fn graph_1() {
            let graph = to_vec(&GRAPH_1);
            let mut pred = [None; 9];

            let mut dist = [
                0,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
            ];

            let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

            predecessors(&graph, |acc, w| acc + w, &mut pred, &mut dist, &mut heap);

            assert_eq!(
                pred,
                [
                    None,
                    Some(0),
                    Some(1),
                    Some(2),
                    Some(5),
                    Some(6),
                    Some(7),
                    Some(0),
                    Some(2)
                ]
            );

            assert_eq!(dist, [0, 4, 12, 19, 21, 11, 9, 8, 14]);
        }

        #[test]
        fn graph_shortest_path_1() {
            let graph = to_vec(&GRAPH_SHORTEST_PATH_1);
            let mut pred = [None; 4];
            let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
            let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

            predecessors(&graph, |acc, w| acc + w, &mut pred, &mut dist, &mut heap);

            assert_eq!(pred, [None, Some(0), Some(1), None]);
            assert_eq!(dist, [0, 2, 4, usize::MAX]);
        }

        #[test]
        fn graph_cross_country() {
            let graph = to_vec(&GRAPH_CROSS_COUNTRY);
            let mut pred = [None; 4];
            let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
            let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

            predecessors(&graph, |acc, w| acc + w, &mut pred, &mut dist, &mut heap);

            assert_eq!(pred, [None, Some(0), Some(0), Some(2)]);
            assert_eq!(dist, [0, 1, 3, 10]);
        }

        #[test]
        fn graph_bryr_1() {
            let mut graph = vec![Vec::new(); 3];

            for (s, t, w) in EDGES_BRYR_1 {
                graph.add_weighted_edge(s, t, w);
                graph.add_weighted_edge(t, s, w);
            }

            let mut pred = [None; 3];
            let mut dist = [0, usize::MAX, usize::MAX];
            let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

            predecessors(&graph, |acc, w| acc + w, &mut pred, &mut dist, &mut heap);

            assert_eq!(pred, [None, Some(0), Some(0)]);
            assert_eq!(dist, [0, 1, 1]);
        }

        #[test]
        fn graph_bryr_2() {
            let mut graph = vec![Vec::new(); 6];

            for (s, t, w) in EDGES_BRYR_2 {
                graph.add_weighted_edge(s, t, w);
                graph.add_weighted_edge(t, s, w);
            }

            let mut pred = [None; 6];

            let mut dist = [
                0,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
            ];

            let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

            predecessors(&graph, |acc, w| acc + w, &mut pred, &mut dist, &mut heap);

            assert_eq!(pred, [None, Some(0), Some(3), Some(0), Some(3), Some(4)]);
            assert_eq!(dist, [0, 1, 2, 1, 2, 3]);
        }

        #[test]
        fn graph_bryr_3() {
            let mut graph = vec![Vec::new(); 10];

            for (s, t, w) in EDGES_BRYR_3 {
                graph.add_weighted_edge(s, t, w);
                graph.add_weighted_edge(t, s, w);
            }

            let mut pred = [None; 10];

            let mut dist = [
                0,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
            ];

            let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

            predecessors(&graph, |acc, w| acc + w, &mut pred, &mut dist, &mut heap);

            assert_eq!(
                pred,
                [
                    None,
                    Some(7),
                    Some(9),
                    Some(0),
                    Some(3),
                    Some(3),
                    Some(5),
                    Some(3),
                    Some(5),
                    Some(1)
                ],
            );

            assert_eq!(dist, [0, 0, 1, 0, 0, 0, 1, 0, 0, 1]);
        }
    }

    mod predecessors_single_source {
        use {
            super::*,
            crate::op::AddWeightedEdge,
        };

        #[test]
        #[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
        fn graph_0() {
            let graph = to_vec(&GRAPH_0);
            let _ = predecessors_single_source(&graph, 0);
        }

        #[test]
        fn graph_1() {
            #[rustfmt::skip]
            const EXPECTED_PRED: [[Option<usize>; 9]; 9] = [
                [None,    Some(0), Some(1), Some(2), Some(5), Some(6), Some(7), Some(0), Some(2)],
                [Some(1), None,    Some(1), Some(2), Some(5), Some(2), Some(7), Some(1), Some(2)],
                [Some(1), Some(2), None,    Some(2), Some(5), Some(2), Some(5), Some(6), Some(2)],
                [Some(1), Some(2), Some(3), None,    Some(3), Some(2), Some(5), Some(6), Some(2)],
                [Some(7), Some(2), Some(5), Some(4), None,    Some(4), Some(5), Some(6), Some(2)],
                [Some(7), Some(2), Some(5), Some(2), Some(5), None,    Some(5), Some(6), Some(2)],
                [Some(7), Some(7), Some(5), Some(2), Some(5), Some(6), None,    Some(6), Some(6)],
                [Some(7), Some(7), Some(5), Some(2), Some(5), Some(6), Some(7), None,    Some(7)],
                [Some(1), Some(2), Some(8), Some(2), Some(5), Some(2), Some(8), Some(8), None   ],
            ];

            #[rustfmt::skip]
            const EXPECTED_DIST: [[usize; 9]; 9] = [
                [ 0,  4, 12, 19, 21, 11,  9,  8, 14],
                [ 4,  0,  8, 15, 22, 12, 12, 11, 10],
                [12,  8,  0,  7, 14,  4,  6,  7,  2],
                [19, 15,  7,  0,  9, 11, 13, 14,  9],
                [21, 22, 14,  9,  0, 10, 12, 13, 16],
                [11, 12,  4, 11, 10,  0,  2,  3,  6],
                [ 9, 12,  6, 13, 12,  2,  0,  1,  6],
                [ 8, 11,  7, 14, 13,  3,  1,  0,  7],
                [14, 10,  2,  9, 16,  6,  6,  7,  0],
            ];

            let graph = to_vec(&GRAPH_1);

            for i in 0..9 {
                let (pred, dist) = predecessors_single_source(&graph, i);

                assert_eq!(pred, EXPECTED_PRED[i]);
                assert_eq!(dist, EXPECTED_DIST[i]);
            }
        }

        #[test]
        fn graph_shortest_path_1() {
            const M: usize = usize::MAX;

            #[rustfmt::skip]
            const EXPECTED_PRED: [[Option<usize>; 4]; 4] = [
                [None,    Some(0), Some(1), None],
                [None,    None,    Some(1), None],
                [None,    None,    None,    None],
                [Some(3), Some(0), Some(1), None],
            ];

            #[rustfmt::skip]
            const EXPECTED_DIST: [[usize; 4]; 4] = [
                [0, 2, 4, M],
                [M, 0, 2, M],
                [M, M, 0, M],
                [2, 4, 6, 0],
            ];

            let graph = to_vec(&GRAPH_SHORTEST_PATH_1);

            for i in 0..4 {
                let (pred, dist) = predecessors_single_source(&graph, i);

                assert_eq!(pred, EXPECTED_PRED[i]);
                assert_eq!(dist, EXPECTED_DIST[i]);
            }
        }

        #[test]
        fn graph_cross_country() {
            #[rustfmt::skip]
            const EXPECTED_PRED: [[Option<usize>; 4]; 4] = [
                [None,    Some(0), Some(0), Some(2)],
                [Some(1), None,    Some(1), Some(2)],
                [Some(2), Some(0), None,    Some(2)],
                [Some(2), Some(0), Some(3), None   ],
            ];

            #[rustfmt::skip]
            const EXPECTED_DIST: [[usize; 4]; 4] = [
                [ 0,  1,  3, 10],
                [ 2,  0,  4, 11],
                [ 3,  4,  0,  7],
                [ 5,  6,  2,  0],
            ];

            let graph = to_vec(&GRAPH_CROSS_COUNTRY);

            for i in 0..4 {
                let (pred, dist) = predecessors_single_source(&graph, i);

                assert_eq!(pred, EXPECTED_PRED[i]);
                assert_eq!(dist, EXPECTED_DIST[i]);
            }
        }

        #[test]
        fn graph_bryr_1() {
            #[rustfmt::skip]
            const EXPECTED_PRED: [[Option<usize>; 3]; 3] = [
                [None,    Some(0), Some(0)],
                [Some(1), None,    Some(1)],
                [Some(2), Some(2), None   ],
            ];

            #[rustfmt::skip]
            const EXPECTED_DIST: [[usize; 3]; 3] = [
                [0, 1, 1],
                [1, 0, 1],
                [1, 1, 0],
            ];

            let mut graph = vec![Vec::new(); 3];

            for (s, t, w) in EDGES_BRYR_1 {
                graph.add_weighted_edge(s, t, w);
                graph.add_weighted_edge(t, s, w);
            }

            for i in 0..3 {
                let (pred, dist) = predecessors_single_source(&graph, i);

                assert_eq!(pred, EXPECTED_PRED[i]);
                assert_eq!(dist, EXPECTED_DIST[i]);
            }
        }

        #[test]
        fn graph_bryr_2() {
            #[rustfmt::skip]
            const EXPECTED_PRED: [[Option<usize>; 6]; 6] = [
                [None,    Some(0), Some(3), Some(0), Some(3), Some(4)],
                [Some(1), None,    Some(1), Some(2), Some(3), Some(4)],
                [Some(3), Some(2), None,    Some(2), Some(3), Some(4)],
                [Some(3), Some(2), Some(3), None,    Some(3), Some(4)],
                [Some(3), Some(2), Some(3), Some(4), None,    Some(4)],
                [Some(3), Some(2), Some(3), Some(4), Some(5), None   ],
            ];

            #[rustfmt::skip]
            const EXPECTED_DIST: [[usize; 6]; 6] = [
                [0, 1, 2, 1, 2, 3],
                [1, 0, 1, 2, 3, 4],
                [2, 1, 0, 1, 2, 3],
                [1, 2, 1, 0, 1, 2],
                [2, 3, 2, 1, 0, 1],
                [3, 4, 3, 2, 1, 0],
            ];

            let mut graph = vec![Vec::new(); 6];

            for (s, t, w) in EDGES_BRYR_2 {
                graph.add_weighted_edge(s, t, w);
                graph.add_weighted_edge(t, s, w);
            }

            for i in 0..6 {
                let (pred, dist) = predecessors_single_source(&graph, i);

                assert_eq!(pred, EXPECTED_PRED[i]);
                assert_eq!(dist, EXPECTED_DIST[i]);
            }
        }

        #[test]
        fn graph_bryr_3() {
            #[rustfmt::skip]
            const EXPECTED_PRED: [[Option<usize>; 10]; 10] = [
                [None,    Some(7), Some(9), Some(0), Some(3), Some(3), Some(5), Some(3), Some(5), Some(1)],
                [Some(3), None,    Some(9), Some(7), Some(3), Some(3), Some(5), Some(1), Some(5), Some(1)],
                [Some(3), Some(9), None,    Some(5), Some(6), Some(6), Some(2), Some(3), Some(5), Some(2)],
                [Some(3), Some(7), Some(9), None,    Some(3), Some(3), Some(5), Some(3), Some(5), Some(1)],
                [Some(3), Some(7), Some(9), Some(4), None,    Some(3), Some(4), Some(3), Some(5), Some(1)],
                [Some(3), Some(7), Some(9), Some(5), Some(3), None,    Some(5), Some(3), Some(5), Some(1)],
                [Some(3), Some(9), Some(6), Some(5), Some(6), Some(6), None,    Some(3), Some(5), Some(2)],
                [Some(3), Some(7), Some(9), Some(7), Some(3), Some(3), Some(5), None,    Some(5), Some(1)],
                [Some(3), Some(7), Some(9), Some(5), Some(3), Some(8), Some(5), Some(3), None,    Some(1)],
                [Some(3), Some(9), Some(9), Some(5), Some(6), Some(6), Some(2), Some(3), Some(5), None   ],
            ];

            #[rustfmt::skip]
            const EXPECTED_DIST: [[usize; 10]; 10] = [
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
            ];

            let mut graph = vec![Vec::new(); 10];

            for (s, t, w) in EDGES_BRYR_3 {
                graph.add_weighted_edge(s, t, w);
                graph.add_weighted_edge(t, s, w);
            }

            for i in 0..10 {
                let (pred, dist) = predecessors_single_source(&graph, i);

                assert_eq!(pred, EXPECTED_PRED[i]);
                assert_eq!(dist, EXPECTED_DIST[i]);
            }
        }
    }
}

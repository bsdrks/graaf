extern crate alloc;

use {
    crate::ops::{
        CountAllVertices,
        IterEdges,
    },
    alloc::collections::BinaryHeap,
    core::cmp::Reverse,
};

/// Calculate the minimum distances from the source vertices to all other
/// vertices.
///
/// # Arguments
///
/// * `graph`: The graph.
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
///     graaf::algo::dijkstra::unweighted::min_distances,
/// };
///
/// // ╭───╮       ╭───╮
/// // │ 0 │   →   │ 1 │
/// // ╰───╯       ╰───╯
/// //   ↑           ↓
/// // ╭───╮       ╭───╮
/// // │ 3 │       │ 2 │
/// // ╰───╯       ╰───╯
///
/// let graph: [Vec<usize>; 4] = [vec![1], vec![2], Vec::new(), vec![0]];
/// let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
/// let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
///
/// min_distances(&graph, |w| w + 1, &mut dist, &mut heap);
///
/// assert_eq!(dist, [0, 1, 2, usize::MAX]);
/// ```
pub fn min_distances<G, W>(
    graph: &G,
    step: fn(W) -> W,
    dist: &mut [W],
    heap: &mut BinaryHeap<(Reverse<W>, usize)>,
) where
    G: IterEdges,
    W: Copy + Ord,
{
    while let Some((Reverse(w), s)) = heap.pop() {
        let w = step(w);

        for t in graph.iter_edges(s) {
            if w >= dist[t] {
                continue;
            }

            dist[t] = w;
            heap.push((Reverse(w), t));
        }
    }
}

/// Calculate the minimum distances from the source vertex to all other
/// vertices.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `s`: The source vertex.
///
/// # Example
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BinaryHeap,
///     core::cmp::Reverse,
///     graaf::algo::dijkstra::unweighted::min_distances_single_source,
/// };
///
/// // ╭───╮       ╭───╮
/// // │ 0 │   →   │ 1 │
/// // ╰───╯       ╰───╯
/// //   ↑           ↓
/// // ╭───╮       ╭───╮
/// // │ 3 │       │ 2 │
/// // ╰───╯       ╰───╯
///
/// let graph: [Vec<usize>; 4] = [vec![1], vec![2], Vec::new(), vec![0]];
///
/// assert_eq!(
///     min_distances_single_source(&graph, 0),
///     [0, 1, 2, usize::MAX]
/// );
/// ```
pub fn min_distances_single_source<G>(graph: &G, s: usize) -> Vec<usize>
where
    G: CountAllVertices + IterEdges,
{
    let mut dist = vec![usize::MAX; graph.count_all_vertices()];
    let mut heap = BinaryHeap::from([(Reverse(0), s)]);

    dist[s] = 0;

    min_distances(graph, |w| w + 1, &mut dist, &mut heap);

    dist
}

/// Calculate the shortest paths from the source vertices to all other
/// vertices.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `step`: A function that calculates the accumulated weight.
/// * `pred`: The predecessors on the shortest paths from the source vertices.
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
///     graaf::algo::dijkstra::unweighted::shortest_paths,
/// };
///
/// // ╭───╮       ╭───╮
/// // │ 0 │   →   │ 1 │
/// // ╰───╯       ╰───╯
/// //   ↑           ↓
/// // ╭───╮       ╭───╮
/// // │ 3 │       │ 2 │
/// // ╰───╯       ╰───╯
///
/// let graph: [Vec<usize>; 4] = [vec![1], vec![2], Vec::new(), vec![0]];
/// let mut pred = [None; 4];
/// let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
/// let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
///
/// shortest_paths(&graph, |w| w + 1, &mut pred, &mut dist, &mut heap);
///
/// assert_eq!(pred, [None, Some(0), Some(1), None]);
/// assert_eq!(dist, [0, 1, 2, usize::MAX]);
/// ```
pub fn shortest_paths<G, W>(
    graph: &G,
    step: fn(W) -> W,
    pred: &mut [Option<usize>],
    dist: &mut [W],
    heap: &mut BinaryHeap<(Reverse<W>, usize)>,
) where
    G: IterEdges,
    W: Copy + Ord,
{
    while let Some((Reverse(w), s)) = heap.pop() {
        let w = step(w);

        for t in graph.iter_edges(s) {
            if w >= dist[t] {
                continue;
            }

            dist[t] = w;
            pred[t] = Some(s);
            heap.push((Reverse(w), t));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod min_distances {
        use super::*;

        #[test]
        fn no_source() {
            let graph: [Vec<usize>; 0] = [];
            let mut dist = Vec::new();
            let mut heap = BinaryHeap::new();
            let () = min_distances(&graph, |w: usize| w + 1, &mut dist, &mut heap);

            assert!(dist.is_empty());
        }

        #[test]
        fn single_source() {
            let graph: [Vec<usize>; 4] = [vec![1], vec![2], Vec::new(), vec![0]];
            let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
            let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
            let () = min_distances(&graph, |w| w + 1, &mut dist, &mut heap);

            assert_eq!(dist, [0, 1, 2, usize::MAX]);
        }
    }

    mod min_distances_single_source {
        use super::*;

        #[test]
        fn graph_1() {
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
                assert_eq!(min_distances_single_source(&graph, i), d);
            }
        }

        #[test]
        fn graph_2() {
            let graph: [Vec<usize>; 4] = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

            for (i, &d) in [[0, 1, 1, 2], [1, 0, 1, 1], [1, 1, 0, 1], [2, 1, 1, 0]]
                .iter()
                .enumerate()
            {
                assert_eq!(min_distances_single_source(&graph, i), d);
            }
        }
    }

    mod shortest_paths {
        use super::*;

        #[test]
        fn no_source() {
            let graph: [Vec<usize>; 0] = [];
            let mut pred = Vec::new();
            let mut dist = Vec::new();
            let mut heap = BinaryHeap::new();
            let () = shortest_paths(&graph, |w: usize| w + 1, &mut pred, &mut dist, &mut heap);

            assert!(pred.is_empty());
            assert!(dist.is_empty());
        }

        #[test]
        fn conquestcampaign() {
            let graph: [Vec<usize>; 12] = [
                vec![1, 4],
                vec![0, 2, 5],
                vec![1, 3, 6],
                vec![2, 7],
                vec![0, 5, 8],
                vec![1, 4, 6, 9],
                vec![2, 5, 7, 10],
                vec![3, 6, 11],
                vec![4, 9],
                vec![5, 8, 10],
                vec![6, 9, 11],
                vec![7, 10],
            ];

            let mut pred = [None; 12];

            let mut dist = [
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                0,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                0,
            ];

            let mut heap = BinaryHeap::from([(Reverse(0), 5), (Reverse(0), 11)]);
            let () = shortest_paths(&graph, |w| w + 1, &mut pred, &mut dist, &mut heap);

            assert_eq!(dist, [2, 1, 2, 2, 1, 0, 1, 1, 2, 1, 1, 0]);

            assert_eq!(
                pred,
                [
                    Some(4),
                    Some(5),
                    Some(6),
                    Some(7),
                    Some(5),
                    None,
                    Some(5),
                    Some(11),
                    Some(9),
                    Some(5),
                    Some(11),
                    None
                ]
            );
        }
    }
}

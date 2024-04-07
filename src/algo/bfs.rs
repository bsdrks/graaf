//! Breadth-first search
//!
//! # Examples
//!
//! ```
//! use graaf::algo::bfs::predecessors_single_source;
//!
//! // ╭───╮       ╭───╮
//! // │ 0 │   →   │ 1 │
//! // ╰───╯       ╰───╯
//! //   ↑           ↓
//! // ╭───╮       ╭───╮
//! // │ 3 │       │ 2 │
//! // ╰───╯       ╰───╯
//!
//! let graph = [vec![1], vec![2], Vec::new(), vec![0]];
//! let (pred, dist) = predecessors_single_source(&graph, 0);
//!
//! assert_eq!(pred, [None, Some(0), Some(1), None]);
//! assert_eq!(dist, [0, 1, 2, usize::MAX]);
//!
//! let (pred, dist) = predecessors_single_source(&graph, 1);
//!
//! assert_eq!(pred, [None, None, Some(1), None]);
//! assert_eq!(dist, [usize::MAX, 0, 1, usize::MAX]);
//!
//! let (pred, dist) = predecessors_single_source(&graph, 2);
//!
//! assert_eq!(pred, [None, None, None, None]);
//! assert_eq!(dist, [usize::MAX, usize::MAX, 0, usize::MAX]);
//!
//! let (pred, dist) = predecessors_single_source(&graph, 3);
//!
//! assert_eq!(pred, [Some(3), Some(0), Some(1), None]);
//! assert_eq!(dist, [1, 2, 3, 0]);
//! ```

extern crate alloc;

use {
    crate::op::{
        CountAllVertices,
        IterEdges,
    },
    alloc::collections::VecDeque,
};

/// Calculate the minimum distances from the source vertices to all vertices in
/// an unweighted graph. Use [`predecessors`] if you also need the predecessor
/// tree.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `step`: The function to calculate the new weight.
/// * `dist`: The distances from the source vertices.
/// * `queue`: The vertices to visit.
///
/// # Examples
///
/// ```
/// use {
///     graaf::algo::bfs::min_distances,
///     std::collections::VecDeque,
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
/// let graph = [vec![1], vec![2], Vec::new(), vec![0]];
/// let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
/// let mut queue = VecDeque::from(vec![(0, 0)]);
///
/// min_distances(&graph, |w| w + 1, &mut dist, &mut queue);
///
/// assert_eq!(dist, [0, 1, 2, usize::MAX]);
/// ```
pub fn min_distances<G, W>(
    graph: &G,
    step: fn(W) -> W,
    dist: &mut [W],
    queue: &mut VecDeque<(usize, W)>,
) where
    G: IterEdges,
    W: Copy + Ord,
{
    while let Some((s, w)) = queue.pop_front() {
        let w = step(w);

        for t in graph.iter_edges(s) {
            if w >= dist[t] {
                continue;
            }

            dist[t] = w;
            queue.push_back((t, w));
        }
    }
}

/// Calculate the minimum distances from the source vertex to all vertices in an
/// unweighted directed graph. Use [`predecessors_single_source`] if you also
/// need the predecessor tree.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `s`: The source vertex.
///
/// # Examples
///
/// ```
/// use graaf::algo::bfs::min_distances_single_source;
///
/// // ╭───╮       ╭───╮
/// // │ 0 │   →   │ 1 │
/// // ╰───╯       ╰───╯
/// //   ↑           ↓
/// // ╭───╮       ╭───╮
/// // │ 3 │       │ 2 │
/// // ╰───╯       ╰───╯
///
/// let graph = [vec![1], vec![2], Vec::new(), vec![0]];
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
    let mut queue = VecDeque::from(vec![(s, 0)]);

    dist[s] = 0;

    min_distances(graph, |w| w + 1, &mut dist, &mut queue);

    dist
}

/// Calculate the predecessor tree and distances of the shortest paths from the
/// source vertices to all vertices in an unweighted directed graph.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `step`: The function that calculates the accumulated weight.
/// * `pred`: The predecessors on the shortest paths from the source vertices.
/// * `dist`: The distances from the source vertices.
/// * `queue`: The vertices to visit.
///
/// # Examples
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::VecDeque,
///     core::cmp::Reverse,
///     graaf::algo::bfs::predecessors,
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
/// let graph = [vec![1], vec![2], Vec::new(), vec![0]];
/// let mut pred = [None, None, None, None];
/// let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
/// let mut queue = VecDeque::from([(0, 0)]);
///
/// predecessors(&graph, |w| w + 1, &mut pred, &mut dist, &mut queue);
///
/// assert_eq!(pred, [None, Some(0), Some(1), None]);
/// assert_eq!(dist, [0, 1, 2, usize::MAX]);
/// ```
pub fn predecessors<G, W>(
    graph: &G,
    step: fn(W) -> W,
    pred: &mut [Option<usize>],
    dist: &mut [W],
    queue: &mut VecDeque<(usize, W)>,
) where
    G: IterEdges,
    W: Copy + Ord,
{
    while let Some((s, w)) = queue.pop_front() {
        let w = step(w);

        for t in graph.iter_edges(s) {
            if w >= dist[t] {
                continue;
            }

            dist[t] = w;
            pred[t] = Some(s);
            queue.push_back((t, w));
        }
    }
}

/// Calculate the predecessor tree and distances of the shortest paths from the
/// source vertex to all vertices in an unweighted directed graph.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `s`: The source vertex.
///
/// # Examples
///
/// ```
/// use graaf::algo::bfs::predecessors_single_source;
///
/// // ╭───╮       ╭───╮
/// // │ 0 │   →   │ 1 │
/// // ╰───╯       ╰───╯
/// //   ↑           ↓
/// // ╭───╮       ╭───╮
/// // │ 3 │       │ 2 │
/// // ╰───╯       ╰───╯
///
/// let graph = [vec![1], vec![2], Vec::new(), vec![0]];
/// let (pred, dist) = predecessors_single_source(&graph, 0);
///
/// assert_eq!(pred, [None, Some(0), Some(1), None]);
/// assert_eq!(dist, [0, 1, 2, usize::MAX]);
/// ```
pub fn predecessors_single_source<G>(graph: &G, s: usize) -> (Vec<Option<usize>>, Vec<usize>)
where
    G: CountAllVertices + IterEdges,
{
    let mut pred = vec![None; graph.count_all_vertices()];
    let mut dist = vec![usize::MAX; graph.count_all_vertices()];
    let mut queue = VecDeque::from(vec![(s, 0)]);

    dist[s] = 0;

    predecessors(graph, |w| w + 1, &mut pred, &mut dist, &mut queue);

    (pred, dist)
}

#[cfg(test)]
mod test {
    use super::*;

    const GRAPH_0: [Vec<usize>; 0] = [];

    const GRAPH_1: &[&[usize]] = &[&[1], &[2], &[], &[0]];

    const GRAPH_2: &[&[usize]] = &[&[1, 2], &[0, 2, 3], &[0, 1, 3], &[1, 2]];

    const GRAPH_3: &[&[usize]] = &[
        &[1, 3],
        &[0, 2],
        &[1],
        &[0, 4, 7],
        &[3, 5, 6, 7],
        &[4, 6],
        &[4, 5, 7],
        &[3, 4, 6],
    ];

    fn to_vec<T>(graph: &[&[T]]) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        graph.iter().map(|v| v.to_vec()).collect()
    }

    mod min_distances {
        use super::*;

        #[test]
        fn graph_0() {
            let mut dist = Vec::new();
            let mut queue = VecDeque::new();
            min_distances(&GRAPH_0, |w: usize| w + 1, &mut dist, &mut queue);

            assert!(dist.is_empty());
        }

        #[test]
        fn graph_1() {
            let graph = to_vec(GRAPH_1);
            let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
            let mut queue = VecDeque::from([(0, 0)]);
            min_distances(&graph, |w| w + 1, &mut dist, &mut queue);

            assert_eq!(dist, [0, 1, 2, usize::MAX]);
        }

        #[test]
        fn graph_2() {
            let graph = to_vec(GRAPH_2);
            let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
            let mut queue = VecDeque::from([(0, 0)]);
            min_distances(&graph, |w| w + 1, &mut dist, &mut queue);

            assert_eq!(dist, [0, 1, 1, 2]);
        }

        #[test]
        fn graph_3() {
            let graph = to_vec(GRAPH_3);

            let mut dist = [
                0,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
            ];

            let mut queue = VecDeque::from([(0, 0)]);
            min_distances(&graph, |w| w + 1, &mut dist, &mut queue);

            assert_eq!(dist, [0, 1, 2, 1, 2, 3, 3, 2]);
        }
    }

    mod min_distances_single_source {
        use super::*;

        #[test]
        #[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
        fn graph_0() {
            let _ = min_distances_single_source(&GRAPH_0, 0);
        }

        #[test]
        fn graph_1() {
            const M: usize = usize::MAX;

            #[rustfmt::skip]
            const EXPECTED: [[usize; 4]; 4] = [
                [0, 1, 2, M],
                [M, 0, 1, M],
                [M, M, 0, M],
                [1, 2, 3, 0],
            ];

            let graph = to_vec(GRAPH_1);

            for (i, &d) in EXPECTED.iter().enumerate() {
                assert_eq!(min_distances_single_source(&graph, i), d);
            }
        }

        #[test]
        fn graph_2() {
            #[rustfmt::skip]
            const EXPECTED: [[usize; 4]; 4] = [
                [0, 1, 1, 2],
                [1, 0, 1, 1],
                [1, 1, 0, 1],
                [2, 1, 1, 0],
            ];

            let graph = to_vec(GRAPH_2);

            for (i, &d) in EXPECTED.iter().enumerate() {
                assert_eq!(min_distances_single_source(&graph, i), d);
            }
        }

        #[test]
        fn graph_3() {
            #[rustfmt::skip]
            const EXPECTED: [[usize; 8]; 8] = [
                [0, 1, 2, 1, 2, 3, 3, 2],
                [1, 0, 1, 2, 3, 4, 4, 3],
                [2, 1, 0, 3, 4, 5, 5, 4],
                [1, 2, 3, 0, 1, 2, 2, 1],
                [2, 3, 4, 1, 0, 1, 1, 1],
                [3, 4, 5, 2, 1, 0, 1, 2],
                [3, 4, 5, 2, 1, 1, 0, 1],
                [2, 3, 4, 1, 1, 2, 1, 0],
            ];

            let graph = to_vec(GRAPH_3);

            for (i, &d) in EXPECTED.iter().enumerate() {
                assert_eq!(min_distances_single_source(&graph, i), d);
            }
        }
    }

    mod predecessors {
        use super::*;

        #[test]
        fn graph_0() {
            let mut pred = Vec::new();
            let mut dist = Vec::new();
            let mut queue = VecDeque::new();

            predecessors(&GRAPH_0, |w: usize| w + 1, &mut pred, &mut dist, &mut queue);

            assert!(pred.is_empty());
            assert!(dist.is_empty());
        }

        #[test]
        fn graph_1() {
            let graph = to_vec(GRAPH_1);
            let mut pred = [None, None, None, None];
            let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
            let mut queue = VecDeque::from([(0, 0)]);

            predecessors(&graph, |w| w + 1, &mut pred, &mut dist, &mut queue);

            assert_eq!(pred, [None, Some(0), Some(1), None]);
            assert_eq!(dist, [0, 1, 2, usize::MAX]);
        }

        #[test]
        fn graph_2() {
            let graph = to_vec(GRAPH_2);
            let mut pred = [None, None, None, None];
            let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
            let mut queue = VecDeque::from([(0, 0)]);

            predecessors(&graph, |w| w + 1, &mut pred, &mut dist, &mut queue);

            assert_eq!(pred, [None, Some(0), Some(0), Some(1)]);
            assert_eq!(dist, [0, 1, 1, 2]);
        }

        #[test]
        fn graph_3() {
            let graph = to_vec(GRAPH_3);
            let mut pred = [None, None, None, None, None, None, None, None];

            let mut dist = [
                0,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
            ];

            let mut queue = VecDeque::from([(0, 0)]);

            predecessors(&graph, |w| w + 1, &mut pred, &mut dist, &mut queue);

            assert_eq!(
                pred,
                [
                    None,
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(3),
                    Some(4),
                    Some(4),
                    Some(3)
                ]
            );

            assert_eq!(dist, [0, 1, 2, 1, 2, 3, 3, 2]);
        }
    }

    mod predecessors_single_source {
        use super::*;

        #[test]
        #[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
        fn graph_0() {
            let _ = predecessors_single_source(&GRAPH_0, 0);
        }

        #[test]
        fn graph_1() {
            let graph = to_vec(GRAPH_1);
            let (pred, dist) = predecessors_single_source(&graph, 0);

            assert_eq!(pred, [None, Some(0), Some(1), None]);
            assert_eq!(dist, [0, 1, 2, usize::MAX]);
        }

        #[test]
        fn graph_2() {
            let graph = to_vec(GRAPH_2);
            let (pred, dist) = predecessors_single_source(&graph, 0);

            assert_eq!(pred, [None, Some(0), Some(0), Some(1)]);
            assert_eq!(dist, [0, 1, 1, 2]);
        }

        #[test]
        fn graph_3() {
            let graph = to_vec(GRAPH_3);
            let (pred, dist) = predecessors_single_source(&graph, 0);

            assert_eq!(
                pred,
                [
                    None,
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(3),
                    Some(4),
                    Some(4),
                    Some(3)
                ]
            );

            assert_eq!(dist, [0, 1, 2, 1, 2, 3, 3, 2]);
        }
    }
}

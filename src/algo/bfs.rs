//! Breadth-first search
//!
//! Use [`single_source_distances`] if you:
//! - need the distances from a single source vertex to all other vertices.
//!
//! Use [`single_source_predecessors`] if you:
//! - need the predecessor tree for the shortest paths from a single source
//!   vertex to all other vertices.
//!
//! Use [`distances`] if you:
//! - need the distances from multiple source vertices to all other vertices,
//! - have predefined starting distances,
//! - have predefined distances for any non-source vertices,
//! - want to use your own step function.
//!
//! Use [`predecessors`] if you:
//! - need the predecessor tree *and* distances,
//! - have multiple source vertices,
//! - have predefined starting distances,
//! - have predefined distances for any non-source vertices,
//! - want to use your own step function.
//!
//! Use [`shortest_path`] if you:
//! - need the shortest path,
//! - have multiple source vertices,
//! - have predefined starting distances,
//! - have predefined distances for any non-source vertices,
//! - want to use your own step function,
//! - want to use your own target predicate.
//!
//! The implementations use distances instead of a set or boolean array to check
//! if a vertex has been visited because we already calculate these distances
//! during traversal.
//!
//! # Examples
//!
//! This is for illustrative purposes only; use [`predecessors`] if you need the
//! predecessor tree *and* distances.
//!
//! ```
//! use graaf::algo::bfs::{
//!     single_source_distances,
//!     single_source_predecessors,
//! };
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
//! let dist = single_source_distances(&graph, 0);
//! let pred = single_source_predecessors(&graph, 0);
//!
//! assert_eq!(pred, [None, Some(0), Some(1), None]);
//! assert_eq!(dist, [0, 1, 2, usize::MAX]);
//!
//! let dist = single_source_distances(&graph, 1);
//! let pred = single_source_predecessors(&graph, 1);
//!
//! assert_eq!(pred, [None, None, Some(1), None]);
//! assert_eq!(dist, [usize::MAX, 0, 1, usize::MAX]);
//!
//! let dist = single_source_distances(&graph, 2);
//! let pred = single_source_predecessors(&graph, 2);
//!
//! assert_eq!(pred, [None, None, None, None]);
//! assert_eq!(dist, [usize::MAX, usize::MAX, 0, usize::MAX]);
//!
//! let dist = single_source_distances(&graph, 3);
//! let pred = single_source_predecessors(&graph, 3);
//!
//! assert_eq!(pred, [Some(3), Some(0), Some(1), None]);
//! assert_eq!(dist, [1, 2, 3, 0]);
//! ```

extern crate alloc;

use {
    crate::{
        algo::predecessor,
        op::{
            CountAllVertices,
            IterEdges,
        },
    },
    alloc::collections::VecDeque,
};

/// Calculate all distances from the source vertices.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `step`: The function to calculate the new weight.
/// * `dist`: The distances from the source vertices.
/// * `queue`: The source vertices.
///
/// # Examples
///
/// ```
/// use {
///     graaf::algo::bfs::distances,
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
/// distances(&graph, |w| w + 1, &mut dist, &mut queue);
///
/// assert_eq!(dist, [0, 1, 2, usize::MAX]);
/// ```
pub fn distances<G, W>(
    graph: &G,
    step: fn(W) -> W,
    dist: &mut [W],
    queue: &mut VecDeque<(usize, W)>,
) where
    G: IterEdges + ?Sized,
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

/// Calculate all distances from a single source vertex.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `s`: The source vertex.
///
/// # Examples
///
/// ```
/// use graaf::algo::bfs::single_source_distances;
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
/// assert_eq!(single_source_distances(&graph, 0), [0, 1, 2, usize::MAX]);
/// ```
pub fn single_source_distances<G>(graph: &G, s: usize) -> Vec<usize>
where
    G: CountAllVertices + IterEdges,
{
    let mut dist = vec![usize::MAX; graph.count_all_vertices()];
    let mut queue = VecDeque::from(vec![(s, 0)]);

    dist[s] = 0;

    distances(graph, |w| w + 1, &mut dist, &mut queue);

    dist
}

/// Calculate the predecessor tree and distances from the source vertices.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `step`: The function that calculates the accumulated weight.
/// * `pred`: The predecessors on the shortest paths from the source vertices.
/// * `dist`: The distances from the source vertices.
/// * `queue`: The source vertices.
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
    G: IterEdges + ?Sized,
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

/// Calculate the predecessor tree for the shortest paths from a single source
/// vertex.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `s`: The source vertex.
///
/// # Examples
///
/// ```
/// use graaf::algo::bfs::single_source_predecessors;
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
/// let pred = single_source_predecessors(&graph, 0);
///
/// assert_eq!(pred, [None, Some(0), Some(1), None]);
/// ```
pub fn single_source_predecessors<G>(graph: &G, s: usize) -> Vec<Option<usize>>
where
    G: CountAllVertices + IterEdges,
{
    let mut pred = vec![None; graph.count_all_vertices()];
    let mut dist = vec![usize::MAX; graph.count_all_vertices()];
    let mut queue = VecDeque::from(vec![(s, 0)]);

    dist[s] = 0;

    predecessors(graph, |w| w + 1, &mut pred, &mut dist, &mut queue);

    pred
}

/// Calculate the shortest path from the source vertex to the target vertex.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `step`: The function that calculates the accumulated weight.
/// * `is_target`: The function that checks if the vertex is the target vertex.
/// * `pred`: The predecessors on the shortest paths from the source vertices.
/// * `dist`: The distances from the source vertices.
/// * `source`: The source vertices.
///
/// # Examples
///
/// ```
/// use {
///     graaf::algo::bfs::shortest_path,
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
/// let mut pred = [None, None, None, None];
/// let mut dist = [usize::MAX, usize::MAX, usize::MAX, 0];
/// let mut queue = VecDeque::from([(3, 0)]);
///
/// let path = shortest_path(
///     &graph,
///     |w| w + 1,
///     |t| t == 2,
///     &mut pred,
///     &mut dist,
///     &mut queue,
/// );
///
/// assert_eq!(path, Some(vec![3, 0, 1, 2]));
/// ```
pub fn shortest_path<G>(
    graph: &G,
    step: fn(usize) -> usize,
    is_target: fn(usize) -> bool,
    pred: &mut [Option<usize>],
    dist: &mut [usize],
    queue: &mut VecDeque<(usize, usize)>,
) -> Option<Vec<usize>>
where
    G: IterEdges,
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

            if is_target(t) {
                return predecessor::search_by(pred, t, |_, b| b.is_none()).map(|mut path| {
                    path.reverse();
                    path
                });
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const GRAPH_0: [&[usize]; 0] = [];

    const GRAPH_1: [&[usize]; 4] = [&[1], &[2], &[], &[0]];

    const GRAPH_2: [&[usize]; 4] = [&[1, 2], &[0, 2, 3], &[0, 1, 3], &[1, 2]];

    const GRAPH_3: [&[usize]; 8] = [
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

    #[test]
    fn distances_graph_0() {
        let graph = to_vec(&GRAPH_0);
        let mut dist = Vec::new();
        let mut queue = VecDeque::new();

        distances(&graph, |w: usize| w + 1, &mut dist, &mut queue);

        assert!(dist.is_empty());
    }

    #[test]
    fn distances_graph_1() {
        let graph = to_vec(&GRAPH_1);
        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut queue = VecDeque::from([(0, 0)]);

        distances(&graph, |w| w + 1, &mut dist, &mut queue);

        assert_eq!(dist, [0, 1, 2, usize::MAX]);
    }

    #[test]
    fn distances_graph_2() {
        let graph = to_vec(&GRAPH_2);
        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut queue = VecDeque::from([(0, 0)]);

        distances(&graph, |w| w + 1, &mut dist, &mut queue);

        assert_eq!(dist, [0, 1, 1, 2]);
    }

    #[test]
    fn distances_graph_3() {
        let graph = to_vec(&GRAPH_3);

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

        distances(&graph, |w| w + 1, &mut dist, &mut queue);

        assert_eq!(dist, [0, 1, 2, 1, 2, 3, 3, 2]);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
    fn graph_0() {
        let graph = to_vec(&GRAPH_0);
        let _ = single_source_distances(&graph, 0);
    }

    #[test]
    fn single_source_distances_graph_1() {
        const M: usize = usize::MAX;

        #[rustfmt::skip]
        const EXPECTED: [[usize; 4]; 4] = [
            [0, 1, 2, M],
            [M, 0, 1, M],
            [M, M, 0, M],
            [1, 2, 3, 0],
        ];

        let graph = to_vec(&GRAPH_1);

        for (i, &d) in EXPECTED.iter().enumerate() {
            assert_eq!(single_source_distances(&graph, i), d);
        }
    }

    #[test]
    fn single_source_distances_graph_2() {
        #[rustfmt::skip]
        const EXPECTED: [[usize; 4]; 4] = [
            [0, 1, 1, 2],
            [1, 0, 1, 1],
            [1, 1, 0, 1],
            [2, 1, 1, 0],
        ];

        let graph = to_vec(&GRAPH_2);

        for (i, &d) in EXPECTED.iter().enumerate() {
            assert_eq!(single_source_distances(&graph, i), d);
        }
    }

    #[test]
    fn single_source_distances_graph_3() {
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

        let graph = to_vec(&GRAPH_3);

        for (i, &d) in EXPECTED.iter().enumerate() {
            assert_eq!(single_source_distances(&graph, i), d);
        }
    }

    #[test]
    fn predecessors_graph_0() {
        let graph = to_vec(&GRAPH_0);
        let mut pred = Vec::new();
        let mut dist = Vec::new();
        let mut queue = VecDeque::new();

        predecessors(&graph, |w: usize| w + 1, &mut pred, &mut dist, &mut queue);

        assert!(pred.is_empty());
        assert!(dist.is_empty());
    }

    #[test]
    fn predecessors_graph_1() {
        let graph = to_vec(&GRAPH_1);
        let mut pred = [None, None, None, None];
        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut queue = VecDeque::from([(0, 0)]);

        predecessors(&graph, |w| w + 1, &mut pred, &mut dist, &mut queue);

        assert_eq!(pred, [None, Some(0), Some(1), None]);
        assert_eq!(dist, [0, 1, 2, usize::MAX]);
    }

    #[test]
    fn predecessors_graph_2() {
        let graph = to_vec(&GRAPH_2);
        let mut pred = [None, None, None, None];
        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut queue = VecDeque::from([(0, 0)]);

        predecessors(&graph, |w| w + 1, &mut pred, &mut dist, &mut queue);

        assert_eq!(pred, [None, Some(0), Some(0), Some(1)]);
        assert_eq!(dist, [0, 1, 1, 2]);
    }

    #[test]
    fn predecessors_graph_3() {
        let graph = to_vec(&GRAPH_3);
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

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
    fn single_source_predecessors_graph_0() {
        let graph = to_vec(&GRAPH_0);
        let _ = single_source_predecessors(&graph, 0);
    }

    #[test]
    fn single_source_predecessors_graph_1() {
        let graph = to_vec(&GRAPH_1);
        let pred = single_source_predecessors(&graph, 0);

        assert_eq!(pred, [None, Some(0), Some(1), None]);
    }

    #[test]
    fn single_source_predecessors_graph_2() {
        let graph = to_vec(&GRAPH_2);
        let pred = single_source_predecessors(&graph, 0);

        assert_eq!(pred, [None, Some(0), Some(0), Some(1)]);
    }

    #[test]
    fn single_source_predecessors_graph_3() {
        let graph = to_vec(&GRAPH_3);
        let pred = single_source_predecessors(&graph, 0);

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
    }

    #[test]
    fn shortest_path_graph_0() {
        let graph = to_vec(&GRAPH_0);
        let pred = &mut Vec::new();
        let dist = &mut Vec::new();
        let queue = &mut VecDeque::new();
        let path = shortest_path(&graph, |w: usize| w + 1, |t| t == 0, pred, dist, queue);

        assert!(path.is_none());
    }

    #[test]
    fn shortest_path_graph_1() {
        let graph = to_vec(&GRAPH_1);
        let pred = &mut [None, None, None, None];
        let dist = &mut [0, usize::MAX, usize::MAX, usize::MAX];
        let queue = &mut VecDeque::from([(0, 0)]);
        let path = shortest_path(&graph, |w| w + 1, |t| t == 1, pred, dist, queue);

        assert_eq!(path, Some(vec![0, 1]));
    }

    #[test]
    fn shortest_path_graph_2() {
        let graph = to_vec(&GRAPH_2);
        let pred = &mut [None, None, None, None];
        let dist = &mut [0, usize::MAX, usize::MAX, usize::MAX];
        let queue = &mut VecDeque::from([(0, 0)]);
        let path = shortest_path(&graph, |w| w + 1, |t| t == 2, pred, dist, queue);

        assert_eq!(path, Some(vec![0, 2]));
    }

    #[test]
    fn shortest_path_graph_3() {
        let graph = to_vec(&GRAPH_3);
        let pred = &mut [None, None, None, None, None, None, None, None];

        let dist = &mut [
            0,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
        ];

        let queue = &mut VecDeque::from([(0, 0)]);
        let path = shortest_path(&graph, |w| w + 1, |t| t == 2, pred, dist, queue);

        assert_eq!(path, Some(vec![0, 1, 2]));
    }
}

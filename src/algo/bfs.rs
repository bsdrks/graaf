//! Breadth-first search
//!
//! Breadth-first search is a graph traversal algorithm that visits
//! vertices of an unweighted digraph in order of their distance from the source
//! vertex. Use [`dijkstra`] for weighted digraphs.
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
//! // ╭───╮     ╭───╮
//! // │ 0 │  →  │ 1 │
//! // ╰───╯     ╰───╯
//! //   ↑         ↓
//! // ╭───╮     ╭───╮
//! // │ 3 │     │ 2 │
//! // ╰───╯     ╰───╯
//!
//! let digraph = [vec![1], vec![2], Vec::new(), vec![0]];
//! let dist = single_source_distances(&digraph, 0);
//! let pred = single_source_predecessors(&digraph, 0);
//!
//! assert_eq!(pred, [None, Some(0), Some(1), None]);
//! assert_eq!(dist, [0, 1, 2, usize::MAX]);
//!
//! let dist = single_source_distances(&digraph, 1);
//! let pred = single_source_predecessors(&digraph, 1);
//!
//! assert_eq!(pred, [None, None, Some(1), None]);
//! assert_eq!(dist, [usize::MAX, 0, 1, usize::MAX]);
//!
//! let dist = single_source_distances(&digraph, 2);
//! let pred = single_source_predecessors(&digraph, 2);
//!
//! assert_eq!(pred, [None, None, None, None]);
//! assert_eq!(dist, [usize::MAX, usize::MAX, 0, usize::MAX]);
//!
//! let dist = single_source_distances(&digraph, 3);
//! let pred = single_source_predecessors(&digraph, 3);
//!
//! assert_eq!(pred, [Some(3), Some(0), Some(1), None]);
//! assert_eq!(dist, [1, 2, 3, 0]);
//! ```
//!
//! [`dijkstra`]: crate::algo::dijkstra

extern crate alloc;

use {
    crate::{
        algo::predecessor,
        op::{
            IterOutNeighbors,
            Order,
        },
    },
    alloc::collections::VecDeque,
};

/// Calculates all distances from the source vertices.
///
/// # Arguments
///
/// * `graph`: The digraph.
/// * `step`: The function to calculate the new weight.
/// * `dist`: The distances from the source vertices.
/// * `queue`: The source vertices.
///
/// # Panics
///
/// * Panics if `step` panics.
/// * Panics if a source or successor vertex is not in `dist`.
/// * Panics if a source or successor vertex is not in `graph`.
///
/// # Examples
///
/// ```
/// use {
///     graaf::algo::bfs::distances,
///     std::collections::VecDeque,
/// };
///
/// // ╭───╮     ╭───╮
/// // │ 0 │  →  │ 1 │
/// // ╰───╯     ╰───╯
/// //   ↑         ↓
/// // ╭───╮     ╭───╮
/// // │ 3 │     │ 2 │
/// // ╰───╯     ╰───╯
///
/// let digraph = [vec![1], vec![2], Vec::new(), vec![0]];
/// let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
/// let mut queue = VecDeque::from(vec![(0, 0)]);
///
/// distances(&digraph, |w| w + 1, &mut dist, &mut queue);
///
/// assert_eq!(dist, [0, 1, 2, usize::MAX]);
/// ```
pub fn distances<D, S, W>(digraph: &D, step: S, dist: &mut [W], queue: &mut VecDeque<(usize, W)>)
where
    D: IterOutNeighbors,
    S: Fn(W) -> W,
    W: Copy + Ord,
{
    while let Some((s, w)) = queue.pop_front() {
        let w = step(w);

        for t in digraph.iter_out_neighbors(s) {
            if w >= dist[t] {
                continue;
            }

            dist[t] = w;
            queue.push_back((t, w));
        }
    }
}

/// Calculates all distances from a single source vertex.
///
/// # Arguments
///
/// * `graph`: The digraph.
/// * `s`: The source vertex.
///
/// # Panics
///
/// Panics if `s` is out of bounds.
///
/// # Examples
///
/// ```
/// use graaf::algo::bfs::single_source_distances;
///
/// // ╭───╮     ╭───╮
/// // │ 0 │  →  │ 1 │
/// // ╰───╯     ╰───╯
/// //   ↑         ↓
/// // ╭───╮     ╭───╮
/// // │ 3 │     │ 2 │
/// // ╰───╯     ╰───╯
///
/// let digraph = [vec![1], vec![2], Vec::new(), vec![0]];
///
/// assert_eq!(single_source_distances(&digraph, 0), [0, 1, 2, usize::MAX]);
/// ```
pub fn single_source_distances<D>(digraph: &D, s: usize) -> Vec<usize>
where
    D: Order + IterOutNeighbors,
{
    let mut dist = vec![usize::MAX; digraph.order()];
    let mut queue = VecDeque::from(vec![(s, 0)]);

    dist[s] = 0;

    distances(digraph, |w| w + 1, &mut dist, &mut queue);

    dist
}

/// Calculates the predecessor tree and distances from the source vertices.
///
/// # Arguments
///
/// * `graph`: The digraph.
/// * `step`: The function that calculates the accumulated weight.
/// * `pred`: The predecessors on the shortest paths from the source vertices.
/// * `dist`: The distances from the source vertices.
/// * `queue`: The source vertices.
///
/// # Panics
///
/// * Panics if `step` panics.
/// * Panics if a source or successor vertex is not in `dist`.
/// * Panics if a source or successor vertex is not in `graph`.
/// * Panics if a source or successor vertex is not in `pred`.
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
/// // ╭───╮     ╭───╮
/// // │ 0 │  →  │ 1 │
/// // ╰───╯     ╰───╯
/// //   ↑         ↓
/// // ╭───╮     ╭───╮
/// // │ 3 │     │ 2 │
/// // ╰───╯     ╰───╯
///
/// let digraph = [vec![1], vec![2], Vec::new(), vec![0]];
/// let mut pred = [None, None, None, None];
/// let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
/// let mut queue = VecDeque::from([(0, 0)]);
///
/// predecessors(&digraph, |w| w + 1, &mut pred, &mut dist, &mut queue);
///
/// assert_eq!(pred, [None, Some(0), Some(1), None]);
/// assert_eq!(dist, [0, 1, 2, usize::MAX]);
/// ```
pub fn predecessors<D, S, W>(
    digraph: &D,
    step: S,
    pred: &mut [Option<usize>],
    dist: &mut [W],
    queue: &mut VecDeque<(usize, W)>,
) where
    D: IterOutNeighbors,
    S: Fn(W) -> W,
    W: Copy + Ord,
{
    while let Some((s, w)) = queue.pop_front() {
        let w = step(w);

        for t in digraph.iter_out_neighbors(s) {
            if w >= dist[t] {
                continue;
            }

            dist[t] = w;
            pred[t] = Some(s);
            queue.push_back((t, w));
        }
    }
}

/// Calculates the predecessor tree for the shortest paths from a single source
/// vertex.
///
/// # Arguments
///
/// * `graph`: The digraph.
/// * `s`: The source vertex.
///
/// # Panics
///
/// Panics if `s` is out of bounds.
///
/// # Examples
///
/// ```
/// use graaf::algo::bfs::single_source_predecessors;
///
/// // ╭───╮     ╭───╮
/// // │ 0 │  →  │ 1 │
/// // ╰───╯     ╰───╯
/// //   ↑         ↓
/// // ╭───╮     ╭───╮
/// // │ 3 │     │ 2 │
/// // ╰───╯     ╰───╯
///
/// let digraph = [vec![1], vec![2], Vec::new(), vec![0]];
/// let pred = single_source_predecessors(&digraph, 0);
///
/// assert_eq!(pred, [None, Some(0), Some(1), None]);
/// ```
pub fn single_source_predecessors<D>(digraph: &D, s: usize) -> Vec<Option<usize>>
where
    D: Order + IterOutNeighbors,
{
    let mut pred = vec![None; digraph.order()];
    let mut dist = vec![usize::MAX; digraph.order()];
    let mut queue = VecDeque::from(vec![(s, 0)]);

    dist[s] = 0;

    predecessors(digraph, |w| w + 1, &mut pred, &mut dist, &mut queue);

    pred
}

/// Calculates the shortest path from the source vertex to a target vertex.
///
/// # Arguments
///
/// * `graph`: The digraph.
/// * `step`: The function that calculates the accumulated distance.
/// * `is_target`: The function that determines if the vertex is a target.
/// * `pred`: The predecessors on the shortest paths from the source vertices.
/// * `dist`: The distances from the source vertices.
/// * `source`: The source vertices.
///
/// # Panics
///
/// * Panics if `is_target` panics.
/// * Panics if `step` panics.
/// * Panics if a source or successor vertex is not in `dist`.
/// * Panics if a source or successor vertex is not in `graph`.
/// * Panics if a source or successor vertex is not in `pred`.
///
/// # Examples
///
/// ```
/// use {
///     graaf::algo::bfs::shortest_path,
///     std::collections::VecDeque,
/// };
///
/// // ╭───╮     ╭───╮
/// // │ 0 │  →  │ 1 │
/// // ╰───╯     ╰───╯
/// //   ↑         ↓
/// // ╭───╮     ╭───╮
/// // │ 3 │     │ 2 │
/// // ╰───╯     ╰───╯
///
/// let digraph = [vec![1], vec![2], Vec::new(), vec![0]];
/// let mut pred = [None, None, None, None];
/// let mut dist = [usize::MAX, usize::MAX, usize::MAX, 0];
/// let mut queue = VecDeque::from([(3, 0)]);
///
/// let path = shortest_path(
///     &digraph,
///     |w| w + 1,
///     |t| t == 2,
///     &mut pred,
///     &mut dist,
///     &mut queue,
/// );
///
/// assert_eq!(path, Some(vec![3, 0, 1, 2]));
/// ```
pub fn shortest_path<D, S, T>(
    digraph: &D,
    step: S,
    is_target: T,
    pred: &mut [Option<usize>],
    dist: &mut [usize],
    queue: &mut VecDeque<(usize, usize)>,
) -> Option<Vec<usize>>
where
    D: IterOutNeighbors,
    S: Fn(usize) -> usize,
    T: Fn(usize) -> bool,
{
    while let Some((s, w)) = queue.pop_front() {
        let w = step(w);

        for t in digraph.iter_out_neighbors(s) {
            if w >= dist[t] {
                continue;
            }

            dist[t] = w;
            pred[t] = Some(s);

            if is_target(t) {
                return predecessor::search_by(pred, t, |_, b| b.is_none()).map(|mut path| {
                    path.reverse();

                    path
                });
            }

            queue.push_back((t, w));
        }
    }

    None
}

/// Calculates the shortest path from a single source vertex to a single target
/// vertex.
///
/// In an unweighted digraph, the shortest path is the path with the fewest
/// arcs. There can be multiple shortest paths in a digraph, but this function
/// only returns one.
///
/// # Arguments
///
/// * `graph`: The digraph.
/// * `s`: The source vertex.
/// * `t`: The target vertex.
///
/// # Panics
///
/// Panics if `s`, `t`, or an intermediate vertex is out of bounds.
///
/// # Examples
///
/// ```
/// use graaf::algo::bfs::single_pair_shortest_path as spsp;
///
/// // ╭───╮     ╭───╮
/// // │ 0 │  →  │ 1 │
/// // ╰───╯     ╰───╯
/// //   ↑         ↓
/// // ╭───╮     ╭───╮
/// // │ 3 │     │ 2 │
/// // ╰───╯     ╰───╯
///
/// let digraph = [vec![1], vec![2], Vec::new(), vec![0]];
/// let path = spsp(&digraph, 3, 2);
///
/// assert_eq!(path, Some(vec![3, 0, 1, 2]));
///
/// // ╭───╮     ╭───╮
/// // │ 0 │  ←  │ 1 │
/// // ╰───╯     ╰───╯
/// //   ↑         ↑
/// // ╭───╮     ╭───╮
/// // │ 3 │  →  │ 2 │
/// // ╰───╯     ╰───╯
///
/// let digraph = [Vec::new(), vec![0], vec![1], vec![0, 2]];
///
/// assert_eq!(spsp(&digraph, 3, 0), Some(vec![3, 0]));
/// assert_eq!(spsp(&digraph, 3, 1), Some(vec![3, 2, 1]));
/// assert_eq!(spsp(&digraph, 3, 2), Some(vec![3, 2]));
/// assert_eq!(spsp(&digraph, 0, 3), None);
/// ```
#[doc(alias = "spsp")]
pub fn single_pair_shortest_path<D>(digraph: &D, s: usize, t: usize) -> Option<Vec<usize>>
where
    D: Order + IterOutNeighbors,
{
    let mut pred = vec![None; digraph.order()];
    let mut dist = vec![usize::MAX; digraph.order()];
    let mut queue = VecDeque::from(vec![(s, 0)]);

    dist[s] = 0;

    shortest_path(
        digraph,
        |w| w + 1,
        |v| v == t,
        &mut pred,
        &mut dist,
        &mut queue,
    )
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

    fn to_vec<T>(digraph: &[&[T]]) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        digraph.iter().map(|v| v.to_vec()).collect()
    }

    #[test]
    fn distances_digraph_0() {
        let digraph = to_vec(&GRAPH_0);
        let mut dist = Vec::new();
        let mut queue = VecDeque::new();

        distances(&digraph, |w: usize| w + 1, &mut dist, &mut queue);

        assert!(dist.is_empty());
    }

    #[test]
    fn distances_digraph_1() {
        let digraph = to_vec(&GRAPH_1);
        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut queue = VecDeque::from([(0, 0)]);

        distances(&digraph, |w| w + 1, &mut dist, &mut queue);

        assert_eq!(dist, [0, 1, 2, usize::MAX]);
    }

    #[test]
    fn distances_digraph_2() {
        let digraph = to_vec(&GRAPH_2);
        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut queue = VecDeque::from([(0, 0)]);

        distances(&digraph, |w| w + 1, &mut dist, &mut queue);

        assert_eq!(dist, [0, 1, 1, 2]);
    }

    #[test]
    fn distances_digraph_3() {
        let digraph = to_vec(&GRAPH_3);

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

        distances(&digraph, |w| w + 1, &mut dist, &mut queue);

        assert_eq!(dist, [0, 1, 2, 1, 2, 3, 3, 2]);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
    fn digraph_0() {
        let digraph = to_vec(&GRAPH_0);
        let _ = single_source_distances(&digraph, 0);
    }

    #[test]
    fn single_source_distances_digraph_1() {
        const M: usize = usize::MAX;

        #[rustfmt::skip]
        const EXPECTED: [[usize; 4]; 4] = [
            [0, 1, 2, M],
            [M, 0, 1, M],
            [M, M, 0, M],
            [1, 2, 3, 0],
        ];

        let digraph = to_vec(&GRAPH_1);

        for (i, &d) in EXPECTED.iter().enumerate() {
            assert_eq!(single_source_distances(&digraph, i), d);
        }
    }

    #[test]
    fn single_source_distances_digraph_2() {
        #[rustfmt::skip]
        const EXPECTED: [[usize; 4]; 4] = [
            [0, 1, 1, 2],
            [1, 0, 1, 1],
            [1, 1, 0, 1],
            [2, 1, 1, 0],
        ];

        let digraph = to_vec(&GRAPH_2);

        for (i, &d) in EXPECTED.iter().enumerate() {
            assert_eq!(single_source_distances(&digraph, i), d);
        }
    }

    #[test]
    fn single_source_distances_digraph_3() {
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

        let digraph = to_vec(&GRAPH_3);

        for (i, &d) in EXPECTED.iter().enumerate() {
            assert_eq!(single_source_distances(&digraph, i), d);
        }
    }

    #[test]
    fn predecessors_digraph_0() {
        let digraph = to_vec(&GRAPH_0);
        let mut pred = Vec::new();
        let mut dist = Vec::new();
        let mut queue = VecDeque::new();

        predecessors(&digraph, |w: usize| w + 1, &mut pred, &mut dist, &mut queue);

        assert!(pred.is_empty());
        assert!(dist.is_empty());
    }

    #[test]
    fn predecessors_digraph_1() {
        let digraph = to_vec(&GRAPH_1);
        let mut pred = [None, None, None, None];
        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut queue = VecDeque::from([(0, 0)]);

        predecessors(&digraph, |w| w + 1, &mut pred, &mut dist, &mut queue);

        assert_eq!(pred, [None, Some(0), Some(1), None]);
        assert_eq!(dist, [0, 1, 2, usize::MAX]);
    }

    #[test]
    fn predecessors_digraph_2() {
        let digraph = to_vec(&GRAPH_2);
        let mut pred = [None, None, None, None];
        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut queue = VecDeque::from([(0, 0)]);

        predecessors(&digraph, |w| w + 1, &mut pred, &mut dist, &mut queue);

        assert_eq!(pred, [None, Some(0), Some(0), Some(1)]);
        assert_eq!(dist, [0, 1, 1, 2]);
    }

    #[test]
    fn predecessors_digraph_3() {
        let digraph = to_vec(&GRAPH_3);
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

        predecessors(&digraph, |w| w + 1, &mut pred, &mut dist, &mut queue);

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
    fn single_source_predecessors_digraph_0() {
        let digraph = to_vec(&GRAPH_0);
        let _ = single_source_predecessors(&digraph, 0);
    }

    #[test]
    fn single_source_predecessors_digraph_1() {
        let digraph = to_vec(&GRAPH_1);
        let pred = single_source_predecessors(&digraph, 0);

        assert_eq!(pred, [None, Some(0), Some(1), None]);
    }

    #[test]
    fn single_source_predecessors_digraph_2() {
        let digraph = to_vec(&GRAPH_2);
        let pred = single_source_predecessors(&digraph, 0);

        assert_eq!(pred, [None, Some(0), Some(0), Some(1)]);
    }

    #[test]
    fn single_source_predecessors_digraph_3() {
        let digraph = to_vec(&GRAPH_3);
        let pred = single_source_predecessors(&digraph, 0);

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
    fn shortest_path_digraph_0() {
        let digraph = to_vec(&GRAPH_0);
        let mut pred = Vec::new();
        let mut dist = Vec::new();
        let mut queue = VecDeque::new();

        let path = shortest_path(
            &digraph,
            |w: usize| w + 1,
            |t| t == 0,
            &mut pred,
            &mut dist,
            &mut queue,
        );

        assert!(pred.is_empty());
        assert!(dist.is_empty());
        assert!(path.is_none());
    }

    #[test]
    fn shortest_path_digraph_1() {
        let digraph = to_vec(&GRAPH_1);
        let mut pred = [None, None, None, None];
        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut queue = VecDeque::from([(0, 0)]);

        let path = shortest_path(
            &digraph,
            |w| w + 1,
            |t| t == 1,
            &mut pred,
            &mut dist,
            &mut queue,
        );

        assert_eq!(pred, [None, Some(0), None, None]);
        assert_eq!(dist, [0, 1, usize::MAX, usize::MAX]);
        assert_eq!(path, Some(vec![0, 1]));
    }

    #[test]
    fn shortest_path_digraph_2() {
        let digraph = to_vec(&GRAPH_2);
        let mut pred = [None, None, None, None];
        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut queue = VecDeque::from([(0, 0)]);

        let path = shortest_path(
            &digraph,
            |w| w + 1,
            |t| t == 2,
            &mut pred,
            &mut dist,
            &mut queue,
        );

        assert_eq!(pred, [None, Some(0), Some(0), None]);
        assert_eq!(dist, [0, 1, 1, usize::MAX]);
        assert_eq!(path, Some(vec![0, 2]));
    }

    #[test]
    fn shortest_path_digraph_3() {
        let digraph = to_vec(&GRAPH_3);
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

        let path = shortest_path(
            &digraph,
            |w| w + 1,
            |t| t == 2,
            &mut pred,
            &mut dist,
            &mut queue,
        );

        assert_eq!(
            pred,
            [None, Some(0), Some(1), Some(0), None, None, None, None]
        );

        assert_eq!(
            dist,
            [0, 1, 2, 1, usize::MAX, usize::MAX, usize::MAX, usize::MAX]
        );

        assert_eq!(path, Some(vec![0, 1, 2]));
    }

    #[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
    #[test]
    fn single_pair_shortest_path_digraph_0() {
        let digraph = to_vec(&GRAPH_0);
        let _ = single_pair_shortest_path(&digraph, 0, 0);
    }

    #[test]
    fn single_pair_shortest_path_digraph_1() {
        let digraph = to_vec(&GRAPH_1);

        for (s, t, p) in &[
            (0, 0, None),
            (0, 1, Some(vec![0, 1])),
            (0, 2, Some(vec![0, 1, 2])),
            (0, 3, None),
            (1, 0, None),
            (1, 1, None),
            (1, 2, Some(vec![1, 2])),
            (1, 3, None),
            (2, 0, None),
            (2, 1, None),
            (2, 2, None),
            (2, 3, None),
            (3, 0, Some(vec![3, 0])),
            (3, 1, Some(vec![3, 0, 1])),
            (3, 2, Some(vec![3, 0, 1, 2])),
            (3, 3, None),
        ] {
            assert_eq!(single_pair_shortest_path(&digraph, *s, *t), *p);
        }
    }

    #[test]
    fn single_pair_shortest_path_digraph_2() {
        let digraph = to_vec(&GRAPH_2);

        for (s, t, p) in &[
            (0, 0, None),
            (0, 1, Some(vec![0, 1])),
            (0, 2, Some(vec![0, 2])),
            (0, 3, Some(vec![0, 1, 3])),
            (1, 0, Some(vec![1, 0])),
            (1, 1, None),
            (1, 2, Some(vec![1, 2])),
            (1, 3, Some(vec![1, 3])),
            (2, 0, Some(vec![2, 0])),
            (2, 1, Some(vec![2, 1])),
            (2, 2, None),
            (2, 3, Some(vec![2, 3])),
            (3, 0, Some(vec![3, 1, 0])),
            (3, 1, Some(vec![3, 1])),
            (3, 2, Some(vec![3, 2])),
            (3, 3, None),
        ] {
            assert_eq!(single_pair_shortest_path(&digraph, *s, *t), *p);
        }
    }

    #[test]
    fn single_pair_shortest_path_digraph_3() {
        let digraph = to_vec(&GRAPH_3);

        for (s, t, p) in &[
            (0, 0, None),
            (0, 1, Some(vec![0, 1])),
            (0, 2, Some(vec![0, 1, 2])),
            (0, 3, Some(vec![0, 3])),
            (0, 4, Some(vec![0, 3, 4])),
            (0, 5, Some(vec![0, 3, 4, 5])),
            (0, 6, Some(vec![0, 3, 4, 6])),
            (0, 7, Some(vec![0, 3, 7])),
            (1, 0, Some(vec![1, 0])),
            (1, 1, None),
            (1, 2, Some(vec![1, 2])),
            (1, 3, Some(vec![1, 0, 3])),
            (1, 4, Some(vec![1, 0, 3, 4])),
            (1, 5, Some(vec![1, 0, 3, 4, 5])),
            (1, 6, Some(vec![1, 0, 3, 4, 6])),
            (1, 7, Some(vec![1, 0, 3, 7])),
            (2, 0, Some(vec![2, 1, 0])),
            (2, 1, Some(vec![2, 1])),
            (2, 2, None),
            (2, 3, Some(vec![2, 1, 0, 3])),
            (2, 4, Some(vec![2, 1, 0, 3, 4])),
            (2, 5, Some(vec![2, 1, 0, 3, 4, 5])),
            (2, 6, Some(vec![2, 1, 0, 3, 4, 6])),
            (2, 7, Some(vec![2, 1, 0, 3, 7])),
            (3, 0, Some(vec![3, 0])),
            (3, 1, Some(vec![3, 0, 1])),
            (3, 2, Some(vec![3, 0, 1, 2])),
            (3, 3, None),
            (3, 4, Some(vec![3, 4])),
            (3, 5, Some(vec![3, 4, 5])),
            (3, 6, Some(vec![3, 4, 6])),
            (3, 7, Some(vec![3, 7])),
            (4, 0, Some(vec![4, 3, 0])),
            (4, 1, Some(vec![4, 3, 0, 1])),
            (4, 2, Some(vec![4, 3, 0, 1, 2])),
            (4, 3, Some(vec![4, 3])),
            (4, 4, None),
            (4, 5, Some(vec![4, 5])),
            (4, 6, Some(vec![4, 6])),
            (4, 7, Some(vec![4, 7])),
            (5, 0, Some(vec![5, 4, 3, 0])),
            (5, 1, Some(vec![5, 4, 3, 0, 1])),
            (5, 2, Some(vec![5, 4, 3, 0, 1, 2])),
            (5, 3, Some(vec![5, 4, 3])),
            (5, 4, Some(vec![5, 4])),
            (5, 5, None),
            (5, 6, Some(vec![5, 6])),
            (5, 7, Some(vec![5, 4, 7])),
            (6, 0, Some(vec![6, 4, 3, 0])),
            (6, 1, Some(vec![6, 4, 3, 0, 1])),
            (6, 2, Some(vec![6, 4, 3, 0, 1, 2])),
            (6, 3, Some(vec![6, 4, 3])),
            (6, 4, Some(vec![6, 4])),
            (6, 5, Some(vec![6, 5])),
            (6, 6, None),
            (6, 7, Some(vec![6, 7])),
            (7, 0, Some(vec![7, 3, 0])),
            (7, 1, Some(vec![7, 3, 0, 1])),
            (7, 2, Some(vec![7, 3, 0, 1, 2])),
            (7, 3, Some(vec![7, 3])),
            (7, 4, Some(vec![7, 4])),
            (7, 5, Some(vec![7, 4, 5])),
            (7, 6, Some(vec![7, 6])),
            (7, 7, None),
        ] {
            assert_eq!(single_pair_shortest_path(&digraph, *s, *t), *p);
        }
    }
}

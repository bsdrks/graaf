//! Dijkstra's algorithm with binary-heap
//!
//! Dijkstra's algorithm[^citation] finds the shortest path in a weighted
//! digraph. Use [`bfs`] for unweighted digraphs.
//!
//! # Examples
//!
//! This is for illustrative purposes only; use [`predecessors`] if you need the
//! predecessor tree *and* distances.
//!
//! ```
//! use graaf::algo::dijkstra::{
//!     single_source_distances,
//!     single_source_predecessors,
//! };
//!
//! // ╭───╮     ╭───╮
//! // │ 0 │ 2 → │ 1 │
//! // ╰───╯     ╰───╯
//! //  ↑ 2       ↓ 2
//! // ╭───╮     ╭───╮
//! // │ 3 │     │ 2 │
//! // ╰───╯     ╰───╯
//!
//! let digraph = [vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]];
//! let dist = single_source_distances(&digraph, 0);
//! let pred = single_source_predecessors(&digraph, 0);
//!
//! assert_eq!(pred, [None, Some(0), Some(1), None]);
//! assert_eq!(dist, [0, 2, 4, usize::MAX]);
//!
//! let dist = single_source_distances(&digraph, 1);
//! let pred = single_source_predecessors(&digraph, 1);
//!
//! assert_eq!(pred, [None, None, Some(1), None]);
//! assert_eq!(dist, [usize::MAX, 0, 2, usize::MAX]);
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
//! assert_eq!(dist, [2, 4, 6, 0]);
//! ```
//!
//! [`bfs`]: crate::algo::bfs
//! [^citation]: Dijkstra, E.W. A note on two problems in connexion with graphs. Numer. Math. 1, 269–271 (1959)

extern crate alloc;

use {
    super::predecessor,
    crate::op::{
        IterWeightedArcs,
        Order,
    },
    alloc::collections::BinaryHeap,
    core::cmp::Reverse,
};

/// Calculates the distances from the source vertices to all vertices in a
/// weighted digraph.
///
/// # Arguments
///
/// * `digraph`: The digraph.
/// * `step`: The function that calculates the accumulated weight.
/// * `dist`: The distances from the source vertices.
/// * `heap`: The source vertices.
///
/// # Panics
///
/// * Panics if `step` panics.
/// * Panics if a source or successor vertex is not in `dist`.
/// * Panics if a source or successor vertex is not in `heap`.
///
/// # Examples
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BinaryHeap,
///     core::cmp::Reverse,
///     graaf::algo::dijkstra::distances,
/// };
///
/// // ╭───╮     ╭───╮
/// // │ 0 │ 2 → │ 1 │
/// // ╰───╯     ╰───╯
/// //  ↑ 2       ↓ 2
/// // ╭───╮     ╭───╮
/// // │ 3 │     │ 2 │
/// // ╰───╯     ╰───╯
///
/// let digraph = [vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]];
/// let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
/// let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
///
/// distances(&digraph, |acc, w| acc + w, &mut dist, &mut heap);
///
/// assert_eq!(dist, [0, 2, 4, usize::MAX]);
/// ```
pub fn distances<D, S, W>(
    digraph: &D,
    step: S,
    dist: &mut [W],
    heap: &mut BinaryHeap<(Reverse<W>, usize)>,
) where
    D: Order + IterWeightedArcs<W>,
    S: Fn(W, &W) -> W,
    W: Copy + Ord,
{
    while let Some((Reverse(acc), s)) = heap.pop() {
        for (t, w) in digraph.iter_weighted_arcs(s) {
            let w = step(acc, w);

            if w >= dist[t] {
                continue;
            }

            dist[t] = w;
            heap.push((Reverse(w), t));
        }
    }
}

/// Calculates all distances from a single source vertex in a weighted directed
/// digraph.
///
/// # Arguments
///
/// * `digraph`: The digraph.
/// * `s`: The source vertex.
///
/// # Panics
///
/// Panics if `s` is out of bounds.
///
/// # Examples
///
/// ```
/// use graaf::algo::dijkstra::single_source_distances;
///
/// // ╭───╮     ╭───╮
/// // │ 0 │ 2 → │ 1 │
/// // ╰───╯     ╰───╯
/// //  ↑ 2       ↓ 2
/// // ╭───╮     ╭───╮
/// // │ 3 │     │ 2 │
/// // ╰───╯     ╰───╯
///
/// let digraph: [Vec<(usize, usize)>; 4] = [vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]];
///
/// assert_eq!(single_source_distances(&digraph, 0), [0, 2, 4, usize::MAX]);
/// ```
pub fn single_source_distances<D>(digraph: &D, s: usize) -> Vec<usize>
where
    D: Order + IterWeightedArcs<usize>,
{
    let mut dist = vec![usize::MAX; digraph.order()];
    let mut heap = BinaryHeap::from([(Reverse(0), s)]);

    dist[s] = 0;

    distances(digraph, |acc, w| acc + w, &mut dist, &mut heap);

    dist
}

/// Calculates the predecessor tree and distances of the shortest paths from the
/// source vertices to all vertices in a weighted digraph.
///
/// # Arguments
///
/// * `digraph`: The digraph.
/// * `step`: The function that calculates the accumulated weight.
/// * `pred`: The predecessors on the shortest paths from the source vertices.
/// * `dist`: The distances from the source vertices.
/// * `heap`: The source vertices.
///
/// # Panics
///
/// * Panics if `step` panics.
/// * Panics if a source or successor vertex is not in `dist`.
/// * Panics if a source or successor vertex is not in `heap`.
/// * Panics if a source or successor vertex is not in `pred`.
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
/// // ╭───╮     ╭───╮
/// // │ 0 │ 2 → │ 1 │
/// // ╰───╯     ╰───╯
/// //  ↑ 2       ↓ 2
/// // ╭───╮     ╭───╮
/// // │ 3 │     │ 2 │
/// // ╰───╯     ╰───╯
///
/// let digraph = [vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]];
/// let mut pred = [None, None, None, None];
/// let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
/// let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
///
/// predecessors(&digraph, |acc, w| acc + w, &mut pred, &mut dist, &mut heap);
///
/// assert_eq!(pred, [None, Some(0), Some(1), None]);
/// assert_eq!(dist, [0, 2, 4, usize::MAX]);
/// ```
pub fn predecessors<D, S, W>(
    digraph: &D,
    step: S,
    pred: &mut [Option<usize>],
    dist: &mut [W],
    heap: &mut BinaryHeap<(Reverse<W>, usize)>,
) where
    D: Order + IterWeightedArcs<W>,
    S: Fn(W, &W) -> W,
    W: Copy + Ord,
{
    while let Some((Reverse(acc), s)) = heap.pop() {
        for (t, w) in digraph.iter_weighted_arcs(s) {
            let w = step(acc, w);

            if w >= dist[t] {
                continue;
            }

            dist[t] = w;
            pred[t] = Some(s);
            heap.push((Reverse(w), t));
        }
    }
}

/// Calculates the predecessor tree for the shortest paths from a single source
/// vertex in a weighted digraph.
///
/// # Arguments
///
/// * `digraph`: The digraph.
/// * `s`: The source vertex.
///
/// # Panics
///
/// Panics if `s` is out of bounds.
///
/// # Examples
///
/// ```
/// use graaf::algo::dijkstra::single_source_predecessors;
///
/// // ╭───╮     ╭───╮
/// // │ 0 │ 2 → │ 1 │
/// // ╰───╯     ╰───╯
/// //  ↑ 2       ↓ 2
/// // ╭───╮     ╭───╮
/// // │ 3 │     │ 2 │
/// // ╰───╯     ╰───╯
///
/// let digraph = [vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]];
/// let pred = single_source_predecessors(&digraph, 0);
///
/// assert_eq!(pred, [None, Some(0), Some(1), None]);
/// ```
pub fn single_source_predecessors<D>(digraph: &D, s: usize) -> Vec<Option<usize>>
where
    D: Order + IterWeightedArcs<usize>,
{
    let v = digraph.order();
    let mut pred = vec![None; v];
    let mut dist = vec![usize::MAX; v];
    let mut heap = BinaryHeap::from([(Reverse(0), s)]);

    dist[s] = 0;

    predecessors(digraph, |acc, w| acc + w, &mut pred, &mut dist, &mut heap);

    pred
}

/// Calculates the shortest path from the source vertex to a target vertex.
///
/// In a weighted digraph, the shortest path is the path with the
/// smallest sum of weights. There can be multiple shortest paths in a digraph,
/// but this function only returns one.
///
/// # Arguments
///
/// * `digraph`: The digraph.
/// * `step`: The function that calculates the accumulated weight.
/// * `is_target`: The function that determines if the vertex is a target.
/// * `pred`: The predecessors on the shortest paths from the source vertices.
/// * `dist`: The distances from the source vertices.
/// * `heap`: The source vertices and their initial distances.
///
/// # Panics
///
/// * Panics if `step` panics.
/// * Panics if `is_target` panics.
/// * Panics if a source or successor vertex is not in `dist`.
/// * Panics if a source or successor vertex is not in `heap`.
/// * Panics if a source or successor vertex is not in `pred`.
///
/// # Examples
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BinaryHeap,
///     core::cmp::Reverse,
///     graaf::algo::dijkstra::shortest_path,
/// };
///
/// // ╭───╮     ╭───╮
/// // │ 0 │ 2 → │ 1 │
/// // ╰───╯     ╰───╯
/// //  ↑ 2       ↓ 2
/// // ╭───╮     ╭───╮
/// // │ 3 │     │ 2 │
/// // ╰───╯     ╰───╯
///
/// let digraph = [vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]];
/// let mut pred = [None, None, None, None];
/// let mut dist = [usize::MAX, usize::MAX, usize::MAX, 0];
/// let mut heap = BinaryHeap::from([(Reverse(0), 3)]);
///
/// let path = shortest_path(
///     &digraph,
///     |acc, w| acc + w,
///     |v, _| v == 2,
///     &mut pred,
///     &mut dist,
///     &mut heap,
/// );
///
/// assert_eq!(pred, [Some(3), Some(0), Some(1), None]);
/// assert_eq!(dist, [2, 4, 6, 0]);
/// assert_eq!(path, Some(vec![3, 0, 1, 2]));
/// ```
pub fn shortest_path<D, S, T, W>(
    digraph: &D,
    step: S,
    is_target: T,
    pred: &mut [Option<usize>],
    dist: &mut [W],
    heap: &mut BinaryHeap<(Reverse<W>, usize)>,
) -> Option<Vec<usize>>
where
    D: Order + IterWeightedArcs<W>,
    S: Fn(W, &W) -> W,
    T: Fn(usize, &W) -> bool,
    W: Copy + Ord,
{
    while let Some((Reverse(acc), s)) = heap.pop() {
        if is_target(s, &acc) {
            return predecessor::search_by(pred, s, |_, u| u.is_none()).map(|mut path| {
                path.reverse();

                path
            });
        }

        for (t, w) in digraph.iter_weighted_arcs(s) {
            let w = step(acc, w);

            if w >= dist[t] {
                continue;
            }

            dist[t] = w;
            pred[t] = Some(s);
            heap.push((Reverse(w), t));
        }
    }

    None
}

/// Calculates the shortest path from a single source vertex to a single target
/// vertex.
///
/// In a weighted digraph, the shortest path is the path with the
/// smallest sum of weights. There can be multiple shortest paths in a digraph,
/// but this function only returns one.
///
/// # Arguments
///
/// * `digraph`: The digraph.
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
/// use graaf::algo::dijkstra::single_pair_shortest_path as spsp;
///
/// // ╭───╮     ╭───╮
/// // │ 0 │ 2 → │ 1 │
/// // ╰───╯     ╰───╯
/// //  ↑ 2       ↓ 2
/// // ╭───╮     ╭───╮
/// // │ 3 │     │ 2 │
/// // ╰───╯     ╰───╯
///
/// let digraph = [vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]];
/// let path = spsp(&digraph, 3, 2);
///
/// assert_eq!(path, Some(vec![3, 0, 1, 2]));
///
/// // ╭───╮     ╭───╮
/// // │ 0 │ ← 1 │ 1 │
/// // ╰───╯     ╰───╯
/// //  ↑ 4       ↑ 1
/// // ╭───╮     ╭───╮
/// // │ 3 │ 1 → │ 2 │
/// // ╰───╯     ╰───╯
///
/// let digraph = [Vec::new(), vec![(0, 1)], vec![(1, 1)], vec![(0, 4), (2, 1)]];
/// let path = spsp(&digraph, 3, 0);
///
/// assert_eq!(path, Some(vec![3, 2, 1, 0]));
/// ```
#[doc(alias = "spsp")]
pub fn single_pair_shortest_path<D>(digraph: &D, s: usize, t: usize) -> Option<Vec<usize>>
where
    D: Order + IterWeightedArcs<usize>,
{
    let v = digraph.order();
    let pred = &mut vec![None; v];
    let dist = &mut vec![usize::MAX; v];
    let heap = &mut BinaryHeap::from([(Reverse(0), s)]);

    dist[s] = 0;

    shortest_path(digraph, |acc, w| acc + w, |v, _| v == t, pred, dist, heap)
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::op::AddWeightedArc,
    };

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
        let mut heap = BinaryHeap::new();

        distances(&digraph, |acc, w| acc + w, &mut dist, &mut heap);

        assert!(dist.is_empty());
    }

    #[test]
    fn distances_digraph_1() {
        let digraph = to_vec(&GRAPH_1);

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

        distances(&digraph, |acc, w| acc + w, &mut dist, &mut heap);

        assert_eq!(dist, [0, 4, 12, 19, 21, 11, 9, 8, 14]);
    }

    #[test]
    fn distances_shortest_path_1() {
        let digraph = to_vec(&GRAPH_SHORTEST_PATH_1);
        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        distances(&digraph, |acc, w| acc + w, &mut dist, &mut heap);

        assert_eq!(dist, [0, 2, 4, usize::MAX]);
    }

    #[test]
    fn distances_cross_country() {
        let digraph = to_vec(&GRAPH_CROSS_COUNTRY);
        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        distances(&digraph, |acc, w| acc + w, &mut dist, &mut heap);

        assert_eq!(dist, [0, 1, 3, 10]);
    }

    #[test]
    fn distances_bryr_1() {
        let mut digraph = vec![Vec::new(); 3];

        for (s, t, w) in EDGES_BRYR_1 {
            digraph.add_weighted_arc(s, t, w);
            digraph.add_weighted_arc(t, s, w);
        }

        let mut dist = [0, usize::MAX, usize::MAX];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        distances(&digraph, |acc, w| acc + w, &mut dist, &mut heap);

        assert_eq!(dist, [0, 1, 1]);
    }

    #[test]
    fn distances_bryr_2() {
        let mut digraph = vec![Vec::new(); 6];

        for (s, t, w) in EDGES_BRYR_2 {
            digraph.add_weighted_arc(s, t, w);
            digraph.add_weighted_arc(t, s, w);
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

        distances(&digraph, |acc, w| acc + w, &mut dist, &mut heap);

        assert_eq!(dist, [0, 1, 2, 1, 2, 3]);
    }

    #[test]
    fn distances_bryr_3() {
        let mut digraph = vec![Vec::new(); 10];

        for (s, t, w) in EDGES_BRYR_3 {
            digraph.add_weighted_arc(s, t, w);
            digraph.add_weighted_arc(t, s, w);
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

        distances(&digraph, |acc, w| acc + w, &mut dist, &mut heap);

        assert_eq!(dist, [0, 0, 1, 0, 0, 0, 1, 0, 0, 1]);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
    fn single_source_distances_digraph_0() {
        let digraph = to_vec(&GRAPH_0);
        let _ = single_source_distances(&digraph, 0);
    }

    #[test]
    fn single_source_distances_digraph_1() {
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

        let digraph = to_vec(&GRAPH_1);

        for (i, &d) in EXPECTED.iter().enumerate() {
            assert_eq!(single_source_distances(&digraph, i), d);
        }
    }

    #[test]
    fn single_source_distances_shortest_path_1() {
        const M: usize = usize::MAX;

        #[rustfmt::skip]
        const EXPECTED: [[usize; 4]; 4] = [
            [0, 2, 4, M],
            [M, 0, 2, M],
            [M, M, 0, M],
            [2, 4, 6, 0],
        ];

        let digraph = to_vec(&GRAPH_SHORTEST_PATH_1);

        for (i, &d) in EXPECTED.iter().enumerate() {
            assert_eq!(single_source_distances(&digraph, i), d);
        }
    }

    #[test]
    fn single_source_distances_cross_country() {
        #[rustfmt::skip]
        const EXPECTED: [[usize; 4]; 4] = [
            [ 0,  1,  3, 10],
            [ 2,  0,  4, 11],
            [ 3,  4,  0,  7],
            [ 5,  6,  2,  0],
        ];

        let digraph = to_vec(&GRAPH_CROSS_COUNTRY);

        for (i, &d) in EXPECTED.iter().enumerate() {
            assert_eq!(single_source_distances(&digraph, i), d);
        }
    }

    #[test]
    fn single_source_distances_bryr_1() {
        #[rustfmt::skip]
        const EXPECTED: [[usize; 3]; 3] = [
            [0, 1, 1],
            [1, 0, 1],
            [1, 1, 0],
        ];

        let mut digraph = vec![Vec::new(); 3];

        for (s, t, w) in EDGES_BRYR_1 {
            digraph.add_weighted_arc(s, t, w);
            digraph.add_weighted_arc(t, s, w);
        }

        for (s, dist) in EXPECTED.iter().enumerate() {
            assert_eq!(single_source_distances(&digraph, s), dist);
        }
    }

    #[test]
    fn single_source_distances_bryr_2() {
        #[rustfmt::skip]
        const EXPECTED: [[usize; 6]; 6] = [
            [0, 1, 2, 1, 2, 3],
            [1, 0, 1, 2, 3, 4],
            [2, 1, 0, 1, 2, 3],
            [1, 2, 1, 0, 1, 2],
            [2, 3, 2, 1, 0, 1],
            [3, 4, 3, 2, 1, 0],
        ];

        let mut digraph = vec![Vec::new(); 6];

        for (s, t, w) in EDGES_BRYR_2 {
            digraph.add_weighted_arc(s, t, w);
            digraph.add_weighted_arc(t, s, w);
        }

        for (s, dist) in EXPECTED.iter().enumerate() {
            assert_eq!(single_source_distances(&digraph, s), dist);
        }
    }

    #[test]
    fn single_source_distances_bryr_3() {
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

        let mut digraph = vec![Vec::new(); 10];

        for (s, t, w) in EDGES_BRYR_3 {
            digraph.add_weighted_arc(s, t, w);
            digraph.add_weighted_arc(t, s, w);
        }

        for (s, dist) in EXPECTED.iter().enumerate() {
            assert_eq!(single_source_distances(&digraph, s), dist);
        }
    }

    #[test]
    fn predecessors_digraph_0() {
        let digraph = to_vec(&GRAPH_0);
        let mut pred = Vec::new();
        let mut dist = Vec::new();
        let mut heap = BinaryHeap::new();

        predecessors(&digraph, |acc, w| acc + w, &mut pred, &mut dist, &mut heap);

        assert!(pred.is_empty());
        assert!(dist.is_empty());
    }

    #[test]
    fn predecessors_digraph_1() {
        let digraph = to_vec(&GRAPH_1);
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

        predecessors(&digraph, |acc, w| acc + w, &mut pred, &mut dist, &mut heap);

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
    fn predecessors_shortest_path_1() {
        let digraph = to_vec(&GRAPH_SHORTEST_PATH_1);
        let mut pred = [None; 4];
        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        predecessors(&digraph, |acc, w| acc + w, &mut pred, &mut dist, &mut heap);

        assert_eq!(pred, [None, Some(0), Some(1), None]);
        assert_eq!(dist, [0, 2, 4, usize::MAX]);
    }

    #[test]
    fn predecessors_cross_country() {
        let digraph = to_vec(&GRAPH_CROSS_COUNTRY);
        let mut pred = [None; 4];
        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        predecessors(&digraph, |acc, w| acc + w, &mut pred, &mut dist, &mut heap);

        assert_eq!(pred, [None, Some(0), Some(0), Some(2)]);
        assert_eq!(dist, [0, 1, 3, 10]);
    }

    #[test]
    fn predecessors_bryr_1() {
        let mut digraph = vec![Vec::new(); 3];

        for (s, t, w) in EDGES_BRYR_1 {
            digraph.add_weighted_arc(s, t, w);
            digraph.add_weighted_arc(t, s, w);
        }

        let mut pred = [None; 3];
        let mut dist = [0, usize::MAX, usize::MAX];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        predecessors(&digraph, |acc, w| acc + w, &mut pred, &mut dist, &mut heap);

        assert_eq!(pred, [None, Some(0), Some(0)]);
        assert_eq!(dist, [0, 1, 1]);
    }

    #[test]
    fn predecessors_bryr_2() {
        let mut digraph = vec![Vec::new(); 6];

        for (s, t, w) in EDGES_BRYR_2 {
            digraph.add_weighted_arc(s, t, w);
            digraph.add_weighted_arc(t, s, w);
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

        predecessors(&digraph, |acc, w| acc + w, &mut pred, &mut dist, &mut heap);

        assert_eq!(pred, [None, Some(0), Some(3), Some(0), Some(3), Some(4)]);
        assert_eq!(dist, [0, 1, 2, 1, 2, 3]);
    }

    #[test]
    fn predecessors_bryr_3() {
        let mut digraph = vec![Vec::new(); 10];

        for (s, t, w) in EDGES_BRYR_3 {
            digraph.add_weighted_arc(s, t, w);
            digraph.add_weighted_arc(t, s, w);
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

        predecessors(&digraph, |acc, w| acc + w, &mut pred, &mut dist, &mut heap);

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

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
    fn single_source_predecessors_digraph_0() {
        let digraph = to_vec(&GRAPH_0);
        let _ = single_source_predecessors(&digraph, 0);
    }

    #[test]
    fn single_source_predecessors_digraph_1() {
        #[rustfmt::skip]
        const EXPECTED: [[Option<usize>; 9]; 9] = [
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

        let digraph = to_vec(&GRAPH_1);

        for (i, expected) in EXPECTED.iter().enumerate() {
            let pred = single_source_predecessors(&digraph, i);

            assert_eq!(pred, expected);
        }
    }

    #[test]
    fn single_source_predecessors_digraph_shortest_path_1() {
        #[rustfmt::skip]
        const EXPECTED: [[Option<usize>; 4]; 4] = [
            [None,    Some(0), Some(1), None],
            [None,    None,    Some(1), None],
            [None,    None,    None,    None],
            [Some(3), Some(0), Some(1), None],
        ];

        let digraph = to_vec(&GRAPH_SHORTEST_PATH_1);

        for (i, expected) in EXPECTED.iter().enumerate() {
            let pred = single_source_predecessors(&digraph, i);

            assert_eq!(pred, expected);
        }
    }

    #[test]
    fn single_source_predecessors_digraph_cross_country() {
        #[rustfmt::skip]
        const EXPECTED: [[Option<usize>; 4]; 4] = [
            [None,    Some(0), Some(0), Some(2)],
            [Some(1), None,    Some(1), Some(2)],
            [Some(2), Some(0), None,    Some(2)],
            [Some(2), Some(0), Some(3), None   ],
        ];

        let digraph = to_vec(&GRAPH_CROSS_COUNTRY);

        for (i, expected) in EXPECTED.iter().enumerate() {
            let pred = single_source_predecessors(&digraph, i);

            assert_eq!(pred, expected);
        }
    }

    #[test]
    fn single_source_predecessors_digraph_bryr_1() {
        #[rustfmt::skip]
        const EXPECTED: [[Option<usize>; 3]; 3] = [
            [None,    Some(0), Some(0)],
            [Some(1), None,    Some(1)],
            [Some(2), Some(2), None   ],
        ];

        let mut digraph = vec![Vec::new(); 3];

        for (s, t, w) in EDGES_BRYR_1 {
            digraph.add_weighted_arc(s, t, w);
            digraph.add_weighted_arc(t, s, w);
        }

        for (i, expected) in EXPECTED.iter().enumerate() {
            let pred = single_source_predecessors(&digraph, i);

            assert_eq!(pred, expected);
        }
    }

    #[test]
    fn single_source_predecessors_digraph_bryr_2() {
        #[rustfmt::skip]
        const EXPECTED: [[Option<usize>; 6]; 6] = [
            [None,    Some(0), Some(3), Some(0), Some(3), Some(4)],
            [Some(1), None,    Some(1), Some(2), Some(3), Some(4)],
            [Some(3), Some(2), None,    Some(2), Some(3), Some(4)],
            [Some(3), Some(2), Some(3), None,    Some(3), Some(4)],
            [Some(3), Some(2), Some(3), Some(4), None,    Some(4)],
            [Some(3), Some(2), Some(3), Some(4), Some(5), None   ],
        ];

        let mut digraph = vec![Vec::new(); 6];

        for (s, t, w) in EDGES_BRYR_2 {
            digraph.add_weighted_arc(s, t, w);
            digraph.add_weighted_arc(t, s, w);
        }

        for (i, expected) in EXPECTED.iter().enumerate() {
            let pred = single_source_predecessors(&digraph, i);

            assert_eq!(pred, expected);
        }
    }

    #[test]
    fn single_source_predecessors_digraph_bryr_3() {
        #[rustfmt::skip]
        const EXPECTED: [[Option<usize>; 10]; 10] = [
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

        let mut digraph = vec![Vec::new(); 10];

        for (s, t, w) in EDGES_BRYR_3 {
            digraph.add_weighted_arc(s, t, w);
            digraph.add_weighted_arc(t, s, w);
        }

        for (i, expected) in EXPECTED.iter().enumerate() {
            let pred = single_source_predecessors(&digraph, i);

            assert_eq!(pred, expected);
        }
    }

    #[test]
    fn shortest_path_digraph_0() {
        let digraph = to_vec(&GRAPH_0);
        let mut pred = Vec::new();
        let mut dist = Vec::new();
        let mut heap = BinaryHeap::new();

        let path = shortest_path(
            &digraph,
            |acc, w| acc + w,
            |_, _| false,
            &mut pred,
            &mut dist,
            &mut heap,
        );

        assert!(pred.is_empty());
        assert!(dist.is_empty());
        assert!(path.is_none());
    }

    #[test]
    fn shortest_path_digraph_1() {
        let digraph = to_vec(&GRAPH_1);
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

        let path = shortest_path(
            &digraph,
            |acc, w| acc + w,
            |v, _| v == 8,
            &mut pred,
            &mut dist,
            &mut heap,
        );

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
        assert_eq!(path, Some(vec![0, 1, 2, 8]));
    }

    #[test]
    fn shortest_path_shortest_path_1() {
        let digraph = to_vec(&GRAPH_SHORTEST_PATH_1);
        let mut pred = [None; 4];
        let mut dist = [usize::MAX, usize::MAX, usize::MAX, 0];
        let mut heap = BinaryHeap::from([(Reverse(0), 3)]);

        let path = shortest_path(
            &digraph,
            |acc, w| acc + w,
            |v, _| v == 2,
            &mut pred,
            &mut dist,
            &mut heap,
        );

        assert_eq!(pred, [Some(3), Some(0), Some(1), None]);
        assert_eq!(dist, [2, 4, 6, 0]);
        assert_eq!(path, Some(vec![3, 0, 1, 2]));
    }

    #[test]
    fn shortest_path_cross_country() {
        let digraph = to_vec(&GRAPH_CROSS_COUNTRY);
        let mut pred = [None; 4];
        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        let path = shortest_path(
            &digraph,
            |acc, w| acc + w,
            |v, _| v == 2,
            &mut pred,
            &mut dist,
            &mut heap,
        );

        assert_eq!(path, Some(vec![0, 2]));
        assert_eq!(pred, [None, Some(0), Some(0), Some(0)]);
        assert_eq!(dist, [0, 1, 3, 14]);
    }

    #[test]
    fn shortest_path_bryr_1() {
        let mut digraph = vec![Vec::new(); 3];

        for (s, t, w) in EDGES_BRYR_1 {
            digraph.add_weighted_arc(s, t, w);
            digraph.add_weighted_arc(t, s, w);
        }

        let mut pred = [None; 3];
        let mut dist = [0, usize::MAX, usize::MAX];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        let path = shortest_path(
            &digraph,
            |acc, w| acc + w,
            |v, _| v == 2,
            &mut pred,
            &mut dist,
            &mut heap,
        );

        assert_eq!(path, Some(vec![0, 2]));
        assert_eq!(pred, [None, Some(0), Some(0)]);
        assert_eq!(dist, [0, 1, 1]);
    }

    #[test]
    fn shortest_path_bryr_2() {
        let mut digraph = vec![Vec::new(); 6];

        for (s, t, w) in EDGES_BRYR_2 {
            digraph.add_weighted_arc(s, t, w);
            digraph.add_weighted_arc(t, s, w);
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

        let path = shortest_path(
            &digraph,
            |acc, w| acc + w,
            |v, _| v == 5,
            &mut pred,
            &mut dist,
            &mut heap,
        );

        assert_eq!(path, Some(vec![0, 3, 4, 5]));
        assert_eq!(pred, [None, Some(0), Some(3), Some(0), Some(3), Some(4)]);
        assert_eq!(dist, [0, 1, 2, 1, 2, 3]);
    }

    #[test]
    fn shortest_path_bryr_3() {
        let mut digraph = vec![Vec::new(); 10];

        for (s, t, w) in EDGES_BRYR_3 {
            digraph.add_weighted_arc(s, t, w);
            digraph.add_weighted_arc(t, s, w);
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

        let path = shortest_path(
            &digraph,
            |acc, w| acc + w,
            |v, _| v == 9,
            &mut pred,
            &mut dist,
            &mut heap,
        );

        assert_eq!(path, Some(vec![0, 3, 7, 1, 9]));

        assert_eq!(
            pred,
            [
                None,
                Some(7),
                None,
                Some(0),
                Some(3),
                Some(3),
                Some(5),
                Some(3),
                Some(5),
                Some(1)
            ],
        );

        assert_eq!(dist, [0, 0, usize::MAX, 0, 0, 0, 1, 0, 0, 1]);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
    fn single_pair_shortest_path_digraph_0() {
        let digraph = to_vec(&GRAPH_0);
        let _ = single_pair_shortest_path(&digraph, 0, 0);
    }

    #[test]
    fn single_pair_shortest_path_digraph_1() {
        let digraph = to_vec(&GRAPH_1);
        let path = single_pair_shortest_path(&digraph, 0, 8);

        assert_eq!(path, Some(vec![0, 1, 2, 8]));
    }

    #[test]
    fn single_pair_shortest_path_shortest_path_1() {
        let digraph = to_vec(&GRAPH_SHORTEST_PATH_1);
        let path = single_pair_shortest_path(&digraph, 0, 2);

        assert_eq!(path, Some(vec![0, 1, 2]));
    }

    #[test]
    fn single_pair_shortest_path_cross_country() {
        let digraph = to_vec(&GRAPH_CROSS_COUNTRY);
        let path = single_pair_shortest_path(&digraph, 0, 2);

        assert_eq!(path, Some(vec![0, 2]));
    }

    #[test]
    fn single_pair_shortest_path_bryr_1() {
        let mut digraph = vec![Vec::new(); 3];

        for (s, t, w) in EDGES_BRYR_1 {
            digraph.add_weighted_arc(s, t, w);
            digraph.add_weighted_arc(t, s, w);
        }

        let path = single_pair_shortest_path(&digraph, 0, 2);

        assert_eq!(path, Some(vec![0, 2]));
    }

    #[test]
    fn single_pair_shortest_path_bryr_2() {
        let mut digraph = vec![Vec::new(); 6];

        for (s, t, w) in EDGES_BRYR_2 {
            digraph.add_weighted_arc(s, t, w);
            digraph.add_weighted_arc(t, s, w);
        }

        let path = single_pair_shortest_path(&digraph, 0, 5);

        assert_eq!(path, Some(vec![0, 3, 4, 5]));
    }

    #[test]
    fn single_pair_shortest_path_bryr_3() {
        let mut digraph = vec![Vec::new(); 10];

        for (s, t, w) in EDGES_BRYR_3 {
            digraph.add_weighted_arc(s, t, w);
            digraph.add_weighted_arc(t, s, w);
        }

        let path = single_pair_shortest_path(&digraph, 0, 9);

        assert_eq!(path, Some(vec![0, 3, 7, 1, 9]));
    }
}

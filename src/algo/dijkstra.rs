//! Dijkstra's algorithm with binary heap
//!
//! Dijkstra's algorithm with binary heap finds the shortest path in a weighted
//! digraph.[^1]
//!
//! The time complexity is *O*(*v* log *v* + *a*).
//!
//! # Examples
//!
//! The separate calls to `single_source_distances` and
//! `single_source_predecessors` in the example are for illustrative purposes
//! only; use [`predecessors`] if you need the breadth-first tree *and*
//! distances.
//!
//! ```
//! use graaf::{
//!     algo::dijkstra::{
//!         single_source_distances,
//!         single_source_predecessors,
//!     },
//!     gen::Empty,
//!     op::AddWeightedArc,
//! };
//!
//! // 0 -> {1 (2)}
//! // 1 -> {2 (2)}
//! // 2 -> {}
//! // 3 -> {0 (2)}
//!
//! let mut digraph = Vec::<Vec<(usize, usize)>>::empty(4);
//!
//! digraph.add_weighted_arc(0, 1, 2);
//! digraph.add_weighted_arc(1, 2, 2);
//! digraph.add_weighted_arc(3, 0, 2);
//!
//! let dist = single_source_distances(&digraph, 0);
//! let pred = single_source_predecessors(&digraph, 0);
//!
//! assert!(pred.into_iter().eq([None, Some(0), Some(1), None]));
//! assert_eq!(dist, [0, 2, 4, usize::MAX]);
//!
//! let dist = single_source_distances(&digraph, 1);
//! let pred = single_source_predecessors(&digraph, 1);
//!
//! assert!(pred.into_iter().eq([None, None, Some(1), None]));
//! assert_eq!(dist, [usize::MAX, 0, 2, usize::MAX]);
//!
//! let dist = single_source_distances(&digraph, 2);
//! let pred = single_source_predecessors(&digraph, 2);
//!
//! assert!(pred.into_iter().eq([None, None, None, None]));
//! assert_eq!(dist, [usize::MAX, usize::MAX, 0, usize::MAX]);
//!
//! let dist = single_source_distances(&digraph, 3);
//! let pred = single_source_predecessors(&digraph, 3);
//!
//! assert!(pred.into_iter().eq([Some(3), Some(0), Some(1), None]));
//! assert_eq!(dist, [2, 4, 6, 0]);
//! ```
//!
//! # Related
//!
//! * Use [`bfs`] to find the shortest path in an unweighted digraph.
//! * Use [`floyd_warshall`] to find the shortest path between all pairs of
//!   vertices in small, dense, weighted digraphs.
//!
//! [`bfs`]: crate::algo::bfs
//! [`floyd_warshall`]: crate::algo::floyd_warshall
//! [^1]: Edsger Wybe Dijkstra. 1959. A note on two problems in connexion
//!   with graphs. Numer. Math. 1, 1 (December 1959), 269–271.
//!   <https://doi.org/10.1007/BF01386390>

use {
    crate::{
        algo::bfs_tree::BfsTree,
        op::{
            IterOutWeightedNeighbors,
            Order,
        },
    },
    core::cmp::Reverse,
    std::collections::BinaryHeap,
};

/// Calculates the distances from the source vertices to all vertices in a
/// weighted digraph.[^1]
///
/// # Arguments
///
/// * `digraph`: The digraph.
/// * `step`: The function that calculates the accumulated weight.
/// * `dist`: The distances from the source vertices.
/// * `heap`: The source vertices.
///
/// # Returns
///
/// Returns the distances from the source vertices to all other vertices.
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
/// use {
///     core::cmp::Reverse,
///     graaf::{
///         algo::dijkstra::distances,
///         gen::Empty,
///         op::AddWeightedArc,
///     },
///     std::collections::BinaryHeap,
/// };
///
/// // 0 -> {1 (2)}
/// // 1 -> {2 (2)}
/// // 2 -> {}
/// // 3 -> {0 (2)}
///
/// let mut digraph = Vec::<Vec<(usize, usize)>>::empty(4);
///
/// digraph.add_weighted_arc(0, 1, 2);
/// digraph.add_weighted_arc(1, 2, 2);
/// digraph.add_weighted_arc(3, 0, 2);
///
/// let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
/// let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
///
/// distances(&digraph, |acc, w| acc + w, &mut dist, &mut heap);
///
/// assert_eq!(dist, [0, 2, 4, usize::MAX]);
/// ```
///
/// [^1]: Edsger Wybe Dijkstra. 1959. A note on two problems in connexion
///   with graphs. Numer. Math. 1, 1 (December 1959), 269–271.
///   <https://doi.org/10.1007/BF01386390>
pub fn distances<D, S, W>(
    digraph: &D,
    step: S,
    dist: &mut [W],
    heap: &mut BinaryHeap<(Reverse<W>, usize)>,
) where
    D: Order + IterOutWeightedNeighbors<W>,
    S: Fn(W, &W) -> W,
    W: Copy + Ord,
{
    while let Some((Reverse(acc), s)) = heap.pop() {
        for (t, w) in digraph.iter_out_weighted_neighbors(s) {
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
/// # Returns
///
/// Returns the distances from the source vertex to all other vertices.
///
/// # Panics
///
/// Panics if `s` is out of bounds.
///
/// # Examples
///
/// ```
/// use graaf::{
///     algo::dijkstra::single_source_distances,
///     gen::Empty,
///     op::AddWeightedArc,
/// };
///
/// // 0 -> {1 (2)}
/// // 1 -> {2 (2)}
/// // 2 -> {}
/// // 3 -> {0 (2)}
///
/// let mut digraph = Vec::<Vec<(usize, usize)>>::empty(4);
///
/// digraph.add_weighted_arc(0, 1, 2);
/// digraph.add_weighted_arc(1, 2, 2);
/// digraph.add_weighted_arc(3, 0, 2);
///
/// assert_eq!(single_source_distances(&digraph, 0), [0, 2, 4, usize::MAX]);
/// ```
pub fn single_source_distances<D>(digraph: &D, s: usize) -> Vec<usize>
where
    D: Order + IterOutWeightedNeighbors<usize>,
{
    let mut dist = vec![usize::MAX; digraph.order()];
    let mut heap = BinaryHeap::from([(Reverse(0), s)]);

    dist[s] = 0;

    distances(digraph, |acc, w| acc + w, &mut dist, &mut heap);

    dist
}

/// Calculates the breadth-first tree and distances of the shortest paths from
/// the source vertices to all vertices in a weighted digraph.
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
/// use {
///     core::cmp::Reverse,
///     graaf::{
///         algo::{
///             bfs_tree::BfsTree,
///             dijkstra::predecessors,
///         },
///         gen::Empty,
///         op::AddWeightedArc,
///     },
///     std::collections::BinaryHeap,
/// };
///
/// // 0 -> {1 (2)}
/// // 1 -> {2 (2)}
/// // 2 -> {}
/// // 3 -> {0 (2)}
///
/// let mut digraph = Vec::<Vec<(usize, usize)>>::empty(4);
///
/// digraph.add_weighted_arc(0, 1, 2);
/// digraph.add_weighted_arc(1, 2, 2);
/// digraph.add_weighted_arc(3, 0, 2);
///
/// let mut pred = BfsTree::new(4);
/// let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
/// let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
///
/// predecessors(&digraph, |acc, w| acc + w, &mut pred, &mut dist, &mut heap);
///
/// assert!(pred.into_iter().eq([None, Some(0), Some(1), None]));
/// assert_eq!(dist, [0, 2, 4, usize::MAX]);
/// ```
pub fn predecessors<D, S, W>(
    digraph: &D,
    step: S,
    pred: &mut BfsTree,
    dist: &mut [W],
    heap: &mut BinaryHeap<(Reverse<W>, usize)>,
) where
    D: Order + IterOutWeightedNeighbors<W>,
    S: Fn(W, &W) -> W,
    W: Copy + Ord,
{
    while let Some((Reverse(acc), s)) = heap.pop() {
        for (t, w) in digraph.iter_out_weighted_neighbors(s) {
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

/// Calculates the breadth-first tree for the shortest paths from a single
/// source vertex in a weighted digraph.
///
/// # Arguments
///
/// * `digraph`: The digraph.
/// * `s`: The source vertex.
///
/// # Returns
///
/// Returns the breadth-first tree.
///
/// # Panics
///
/// Panics if `s` is out of bounds.
///
/// # Examples
///
/// ```
/// use graaf::{
///     algo::dijkstra::single_source_predecessors,
///     gen::Empty,
///     op::AddWeightedArc,
/// };
///
/// // 0 -> {1 (2)}
/// // 1 -> {2 (2)}
/// // 2 -> {}
/// // 3 -> {0 (2)}
///
/// let mut digraph = Vec::<Vec<(usize, usize)>>::empty(4);
///
/// digraph.add_weighted_arc(0, 1, 2);
/// digraph.add_weighted_arc(1, 2, 2);
/// digraph.add_weighted_arc(3, 0, 2);
///
/// let pred = single_source_predecessors(&digraph, 0);
///
/// assert!(pred.into_iter().eq([None, Some(0), Some(1), None]));
/// ```
pub fn single_source_predecessors<D>(digraph: &D, s: usize) -> BfsTree
where
    D: Order + IterOutWeightedNeighbors<usize>,
{
    let v = digraph.order();
    let mut pred = BfsTree::new(v);
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
/// * `is_target`: The function determining whether the vertex is a target.
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
/// use {
///     core::cmp::Reverse,
///     graaf::{
///         algo::{
///             bfs_tree::BfsTree,
///             dijkstra::shortest_path,
///         },
///         gen::EmptyConst,
///         op::AddWeightedArc,
///     },
///     std::collections::BinaryHeap,
/// };
///
/// // 0 -> {1 (2)}
/// // 1 -> {2 (2)}
/// // 2 -> {}
/// // 3 -> {0 (2)}
///
/// let mut digraph = <[Vec<(usize, usize)>; 4]>::empty();
///
/// digraph.add_weighted_arc(0, 1, 2);
/// digraph.add_weighted_arc(1, 2, 2);
/// digraph.add_weighted_arc(3, 0, 2);
///
/// let mut pred = BfsTree::new(4);
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
/// assert!(pred.into_iter().eq([Some(3), Some(0), Some(1), None]));
/// assert_eq!(dist, [2, 4, 6, 0]);
/// assert_eq!(path, Some(vec![3, 0, 1, 2]));
/// ```
pub fn shortest_path<D, S, T, W>(
    digraph: &D,
    step: S,
    is_target: T,
    pred: &mut BfsTree,
    dist: &mut [W],
    heap: &mut BinaryHeap<(Reverse<W>, usize)>,
) -> Option<Vec<usize>>
where
    D: Order + IterOutWeightedNeighbors<W>,
    S: Fn(W, &W) -> W,
    T: Fn(usize, &W) -> bool,
    W: Copy + Ord,
{
    while let Some((Reverse(acc), s)) = heap.pop() {
        if is_target(s, &acc) {
            return pred.search_by(s, |_, u| u.is_none()).map(|mut path| {
                path.reverse();

                path
            });
        }

        for (t, w) in digraph.iter_out_weighted_neighbors(s) {
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
/// # Returns
///
/// Returns the shortest path from the source vertex to the target vertex.
///
/// # Panics
///
/// Panics if `s`, `t`, or an intermediate vertex is out of bounds.
///
/// # Examples
///
/// ```
/// use graaf::{
///     algo::dijkstra::single_pair_shortest_path as spsp,
///     gen::EmptyConst,
///     op::AddWeightedArc,
/// };
///
/// // 0 -> {1 (2)}
/// // 1 -> {2 (2)}
/// // 2 -> {}
/// // 3 -> {0 (2)}
///
/// let mut digraph = <[Vec<(usize, usize)>; 4]>::empty();
///
/// digraph.add_weighted_arc(0, 1, 2);
/// digraph.add_weighted_arc(1, 2, 2);
/// digraph.add_weighted_arc(3, 0, 2);
///
/// let path = spsp(&digraph, 3, 2);
///
/// assert_eq!(path, Some(vec![3, 0, 1, 2]));
///
/// // 0 -> {}
/// // 1 -> {0 (1)}
/// // 2 -> {1 (1)}
/// // 3 -> {0 (4), 2 (1)}
///
/// let digraph = [Vec::new(), vec![(0, 1)], vec![(1, 1)], vec![(0, 4), (2, 1)]];
/// let path = spsp(&digraph, 3, 0);
///
/// assert_eq!(path, Some(vec![3, 2, 1, 0]));
/// ```
#[doc(alias = "spsp")]
pub fn single_pair_shortest_path<D>(digraph: &D, s: usize, t: usize) -> Option<Vec<usize>>
where
    D: Order + IterOutWeightedNeighbors<usize>,
{
    let v = digraph.order();
    let pred = &mut BfsTree::new(v);
    let dist = &mut vec![usize::MAX; v];
    let heap = &mut BinaryHeap::from([(Reverse(0), s)]);

    dist[s] = 0;

    shortest_path(digraph, |acc, w| acc + w, |v, _| v == t, pred, dist, heap)
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            algo::fixture,
            gen::Empty,
        },
    };

    macro_rules! test_distances {
        ($digraph:expr, $dist:expr) => {
            let mut dist = vec![usize::MAX; $dist.len()];
            let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

            dist[0] = 0;

            distances(&$digraph, |acc, w| acc + w, &mut dist, &mut heap);

            assert!(dist.iter().eq($dist));
        };
    }

    macro_rules! test_predecessors {
        ($digraph:expr, $dist:expr, $pred:expr) => {
            let mut pred = BfsTree::new($dist.len());
            let mut dist = vec![usize::MAX; $dist.len()];
            let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

            dist[0] = 0;

            predecessors(&$digraph, |acc, w| acc + w, &mut pred, &mut dist, &mut heap);

            assert!(dist.iter().eq($dist));
            assert!(pred.into_iter().eq($pred));
        };
    }

    macro_rules! test_shortest_path {
        ($digraph:expr, $t:expr, $dist:expr, $pred:expr, $path:expr) => {
            let mut pred = BfsTree::new($dist.len());
            let mut dist = vec![usize::MAX; $dist.len()];
            let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

            dist[0] = 0;

            let path = shortest_path(
                &$digraph,
                |acc, w| acc + w,
                |v, _| v == $t,
                &mut pred,
                &mut dist,
                &mut heap,
            );

            assert!(dist.iter().eq($dist));
            assert!(pred.into_iter().eq($pred));
            assert_eq!(path, $path);
        };
    }

    #[test]
    fn distances_trivial() {
        test_distances!(Vec::<Vec<(usize, usize)>>::trivial(), &[0]);
    }

    #[test]
    fn distances_bang_jensen_94() {
        test_distances!(
            fixture::bang_jensen_94_weighted_usize(),
            &[0, 1, 1, 2, 2, 2, 3]
        );
    }

    #[test]
    fn distances_bang_jensen_96() {
        test_distances!(fixture::bang_jensen_96(), &[0, 5, 3, 6, 4, 7]);
    }

    #[test]
    fn distances_kattis_bryr_1() {
        test_distances!(fixture::kattis_bryr_1(), &[0, 1, 1]);
    }

    #[test]
    fn distances_kattis_bryr_2() {
        test_distances!(fixture::kattis_bryr_2(), &[0, 1, 2, 1, 2, 3]);
    }

    #[test]
    fn distances_kattis_bryr_3() {
        test_distances!(fixture::kattis_bryr_3(), &[0, 0, 1, 0, 0, 0, 1, 0, 0, 1]);
    }

    #[test]
    fn distances_kattis_crosscountry() {
        test_distances!(fixture::kattis_crosscountry(), &[0, 1, 3, 10]);
    }

    #[test]
    fn distances_kattis_shortestpath1() {
        test_distances!(fixture::kattis_shortestpath1(), &[0, 2, 4, usize::MAX]);
    }

    #[test]
    fn single_source_distances_trivial() {
        assert!(
            single_source_distances(&Vec::<Vec<(usize, usize)>>::trivial(), 0)
                .iter()
                .eq(&[0])
        );
    }

    #[test]
    fn single_source_distances_bang_jensen_94() {
        assert!(
            single_source_distances(&fixture::bang_jensen_94_weighted_usize(), 0)
                .iter()
                .eq(&[0, 1, 1, 2, 2, 2, 3])
        );
    }

    #[test]
    fn single_source_distances_bang_jensen_96() {
        assert!(single_source_distances(&fixture::bang_jensen_96(), 0)
            .iter()
            .eq(&[0, 5, 3, 6, 4, 7]));
    }

    #[test]
    fn single_source_distances_kattis_bryr_1() {
        assert!(single_source_distances(&fixture::kattis_bryr_1(), 0)
            .iter()
            .eq(&[0, 1, 1]));
    }

    #[test]
    fn single_source_distances_kattis_bryr_2() {
        assert!(single_source_distances(&fixture::kattis_bryr_2(), 0)
            .iter()
            .eq(&[0, 1, 2, 1, 2, 3]));
    }

    #[test]
    fn single_source_distances_kattis_bryr_3() {
        assert!(single_source_distances(&fixture::kattis_bryr_3(), 0)
            .iter()
            .eq(&[0, 0, 1, 0, 0, 0, 1, 0, 0, 1]));
    }

    #[test]
    fn single_source_distances_kattis_crosscountry() {
        assert!(single_source_distances(&fixture::kattis_crosscountry(), 0)
            .iter()
            .eq(&[0, 1, 3, 10]));
    }

    #[test]
    fn single_source_distances_kattis_shortestpath1() {
        assert!(single_source_distances(&fixture::kattis_shortestpath1(), 0)
            .iter()
            .eq(&[0, 2, 4, usize::MAX]));
    }

    #[test]
    fn predecessors_trivial() {
        test_predecessors!(Vec::<Vec<(usize, usize)>>::trivial(), &[0], [None]);
    }

    #[test]
    fn predecessors_bang_jensen_94() {
        test_predecessors!(
            fixture::bang_jensen_94_weighted_usize(),
            &[0, 1, 1, 2, 2, 2, 3],
            [None, Some(0), Some(0), Some(2), Some(2), Some(2), Some(4)]
        );
    }

    #[test]
    fn predecessors_bang_jensen_96() {
        test_predecessors!(
            fixture::bang_jensen_96(),
            &[0, 5, 3, 6, 4, 7],
            [None, Some(2), Some(0), Some(4), Some(2), Some(3)]
        );
    }

    #[test]
    fn predecessors_kattis_bryr_1() {
        test_predecessors!(
            fixture::kattis_bryr_1(),
            &[0, 1, 1],
            [None, Some(0), Some(0)]
        );
    }

    #[test]
    fn predecessors_kattis_bryr_2() {
        test_predecessors!(
            fixture::kattis_bryr_2(),
            &[0, 1, 2, 1, 2, 3],
            [None, Some(0), Some(3), Some(0), Some(3), Some(4)]
        );
    }

    #[test]
    fn predecessors_kattis_bryr_3() {
        test_predecessors!(
            fixture::kattis_bryr_3(),
            &[0, 0, 1, 0, 0, 0, 1, 0, 0, 1],
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
                Some(1),
            ]
        );
    }

    #[test]
    fn predecessors_kattis_crosscountry() {
        test_predecessors!(
            fixture::kattis_crosscountry(),
            &[0, 1, 3, 10],
            [None, Some(0), Some(0), Some(2)]
        );
    }

    #[test]
    fn predecessors_kattis_shortestpath1() {
        test_predecessors!(
            fixture::kattis_shortestpath1(),
            &[0, 2, 4, usize::MAX],
            [None, Some(0), Some(1), None]
        );
    }

    #[test]
    fn single_source_predecessors_trivial() {
        assert!(
            single_source_predecessors(&Vec::<Vec<(usize, usize)>>::trivial(), 0)
                .into_iter()
                .eq([None])
        );
    }

    #[test]
    fn single_source_predecessors_bang_jensen_94() {
        assert!(
            single_source_predecessors(&fixture::bang_jensen_94_weighted_usize(), 0)
                .into_iter()
                .eq([None, Some(0), Some(0), Some(2), Some(2), Some(2), Some(4)])
        );
    }

    #[test]
    fn single_source_predecessors_bang_jensen_96() {
        assert!(single_source_predecessors(&fixture::bang_jensen_96(), 0)
            .into_iter()
            .eq([None, Some(2), Some(0), Some(4), Some(2), Some(3)]));
    }

    #[test]
    fn single_source_predecessors_kattis_bryr_1() {
        assert!(single_source_predecessors(&fixture::kattis_bryr_1(), 0)
            .into_iter()
            .eq([None, Some(0), Some(0)]));
    }

    #[test]
    fn single_source_predecessors_kattis_bryr_2() {
        assert!(single_source_predecessors(&fixture::kattis_bryr_2(), 0)
            .into_iter()
            .eq([None, Some(0), Some(3), Some(0), Some(3), Some(4)]));
    }

    #[test]
    fn single_source_predecessors_kattis_bryr_3() {
        assert!(single_source_predecessors(&fixture::kattis_bryr_3(), 0)
            .into_iter()
            .eq([
                None,
                Some(7),
                Some(9),
                Some(0),
                Some(3),
                Some(3),
                Some(5),
                Some(3),
                Some(5),
                Some(1),
            ]));
    }

    #[test]
    fn single_source_predecessors_kattis_crosscountry() {
        assert!(
            single_source_predecessors(&fixture::kattis_crosscountry(), 0)
                .into_iter()
                .eq([None, Some(0), Some(0), Some(2)])
        );
    }

    #[test]
    fn single_source_predecessors_kattis_shortestpath1() {
        assert!(
            single_source_predecessors(&fixture::kattis_shortestpath1(), 0)
                .into_iter()
                .eq([None, Some(0), Some(1), None])
        );
    }

    #[test]
    fn shortest_path_trivial() {
        test_shortest_path!(
            Vec::<Vec<(usize, usize)>>::trivial(),
            0,
            &[0],
            [None],
            Some(vec![0])
        );
    }

    #[test]
    fn shortest_path_bang_jensen_94() {
        test_shortest_path!(
            fixture::bang_jensen_94_weighted_usize(),
            6,
            &[0, 1, 1, 2, 2, 2, 3],
            [None, Some(0), Some(0), Some(2), Some(2), Some(2), Some(4)],
            Some(vec![0, 2, 4, 6])
        );
    }

    #[test]
    fn shortest_path_bang_jensen_96() {
        test_shortest_path!(
            fixture::bang_jensen_96(),
            5,
            &[0, 5, 3, 6, 4, 7],
            [None, Some(2), Some(0), Some(4), Some(2), Some(3)],
            Some(vec![0, 2, 4, 3, 5])
        );
    }

    #[test]
    fn shortest_path_kattis_bryr_1() {
        test_shortest_path!(
            fixture::kattis_bryr_1(),
            2,
            &[0, 1, 1],
            [None, Some(0), Some(0)],
            Some(vec![0, 2])
        );
    }

    #[test]
    fn shortest_path_kattis_bryr_2() {
        test_shortest_path!(
            fixture::kattis_bryr_2(),
            5,
            &[0, 1, 2, 1, 2, 3],
            [None, Some(0), Some(3), Some(0), Some(3), Some(4)],
            Some(vec![0, 3, 4, 5])
        );
    }

    #[test]
    fn shortest_path_kattis_bryr_3() {
        test_shortest_path!(
            fixture::kattis_bryr_3(),
            9,
            &[0, 0, usize::MAX, 0, 0, 0, 1, 0, 0, 1],
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
                Some(1),
            ],
            Some(vec![0, 3, 7, 1, 9])
        );
    }

    #[test]
    fn shortest_path_kattis_crosscountry() {
        test_shortest_path!(
            fixture::kattis_crosscountry(),
            2,
            &[0, 1, 3, 14],
            [None, Some(0), Some(0), Some(0)],
            Some(vec![0, 2])
        );
    }

    #[test]
    fn shortest_path_kattis_shortestpath1() {
        test_shortest_path!(
            fixture::kattis_shortestpath1(),
            3,
            &[0, 2, 4, usize::MAX],
            [None, Some(0), Some(1), None],
            None
        );
    }

    #[test]
    fn single_pair_shortest_path_trivial() {
        assert!(
            single_pair_shortest_path(&Vec::<Vec<(usize, usize)>>::trivial(), 0, 0)
                .unwrap()
                .iter()
                .eq(&[0])
        );
    }

    #[test]
    fn single_pair_shortest_path_bang_jensen_94() {
        assert!(
            single_pair_shortest_path(&fixture::bang_jensen_94_weighted_usize(), 0, 6)
                .unwrap()
                .iter()
                .eq(&[0, 2, 4, 6])
        );
    }

    #[test]
    fn single_pair_shortest_path_bang_jensen_96() {
        assert!(single_pair_shortest_path(&fixture::bang_jensen_96(), 0, 5)
            .unwrap()
            .iter()
            .eq(&[0, 2, 4, 3, 5]));
    }

    #[test]
    fn single_pair_shortest_path_kattis_bryr_1() {
        assert!(single_pair_shortest_path(&fixture::kattis_bryr_1(), 0, 2)
            .unwrap()
            .iter()
            .eq(&[0, 2]));
    }

    #[test]
    fn single_pair_shortest_path_kattis_bryr_2() {
        assert!(single_pair_shortest_path(&fixture::kattis_bryr_2(), 0, 5)
            .unwrap()
            .iter()
            .eq(&[0, 3, 4, 5]));
    }

    #[test]
    fn single_pair_shortest_path_kattis_bryr_3() {
        assert!(single_pair_shortest_path(&fixture::kattis_bryr_3(), 0, 9)
            .unwrap()
            .iter()
            .eq(&[0, 3, 7, 1, 9]));
    }

    #[test]
    fn single_pair_shortest_path_kattis_crosscountry() {
        assert!(
            single_pair_shortest_path(&fixture::kattis_crosscountry(), 0, 2)
                .unwrap()
                .iter()
                .eq(&[0, 2])
        );
    }

    #[test]
    fn single_pair_shortest_path_kattis_shortestpath1() {
        assert_eq!(
            single_pair_shortest_path(&fixture::kattis_shortestpath1(), 0, 3),
            None
        );
    }
}

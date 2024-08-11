//! Dijkstra
//!
//! Dijkstra's algorithm with binary heap finds the shortest path in an
//! arc-weighted digraph.[^1]
//!
//! The time complexity is *O*(*v* log *v* + *a*).
//!
//! # Examples
//!
//! The separate calls to `single_source_distances` and
//! `single_source_predecessors` in the example are for illustrative purposes
//! only; use [`predecessors`] if you need the predecessor tree *and*
//! distances.
//!
//! ```
//! use graaf::{
//!     adjacency_list_weighted::Digraph,
//!     algo::dijkstra::{
//!         single_source_distances,
//!         single_source_predecessors,
//!     },
//!     gen::Empty,
//!     op::AddArcWeighted,
//! };
//!
//! // 0 -> {1 (2)}
//! // 1 -> {2 (2)}
//! // 2 -> {}
//! // 3 -> {0 (2)}
//!
//! let mut digraph = Digraph::<usize>::empty(4);
//!
//! digraph.add_arc_weighted(0, 1, 2);
//! digraph.add_arc_weighted(1, 2, 2);
//! digraph.add_arc_weighted(3, 0, 2);
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

/// Finds the distances from the source vertices to all vertices in an
/// arc-weighted digraph.[^1]
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
///         adjacency_list_weighted::Digraph,
///         algo::dijkstra::distances,
///         gen::Empty,
///         op::AddArcWeighted,
///     },
///     std::collections::BinaryHeap,
/// };
///
/// // 0 -> {1 (2)}
/// // 1 -> {2 (2)}
/// // 2 -> {}
/// // 3 -> {0 (2)}
///
/// let mut digraph = Digraph::<usize>::empty(4);
///
/// digraph.add_arc_weighted(0, 1, 2);
/// digraph.add_arc_weighted(1, 2, 2);
/// digraph.add_arc_weighted(3, 0, 2);
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
    D: Order + OutNeighborsWeighted<W>,
    S: Fn(W, &W) -> W,
    W: Copy + Ord,
{
    while let Some((Reverse(acc), s)) = heap.pop() {
        for (t, w) in digraph.out_neighbors_weighted(s) {
            let w = step(acc, w);

            if w >= dist[t] {
                continue;
            }

            dist[t] = w;
            heap.push((Reverse(w), t));
        }
    }
}

/// Finds all distances from a source vertex in an arc-weighted digraph.
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
///     adjacency_list_weighted::Digraph,
///     algo::dijkstra::single_source_distances,
///     gen::Empty,
///     op::AddArcWeighted,
/// };
///
/// // 0 -> {1 (2)}
/// // 1 -> {2 (2)}
/// // 2 -> {}
/// // 3 -> {0 (2)}
///
/// let mut digraph = Digraph::<usize>::empty(4);
///
/// digraph.add_arc_weighted(0, 1, 2);
/// digraph.add_arc_weighted(1, 2, 2);
/// digraph.add_arc_weighted(3, 0, 2);
///
/// assert_eq!(single_source_distances(&digraph, 0), [0, 2, 4, usize::MAX]);
/// ```
#[must_use]
pub fn single_source_distances<D>(digraph: &D, s: usize) -> Vec<usize>
where
    D: Order + OutNeighborsWeighted<usize>,
{
    let v = digraph.order();

    assert!(s < v, "s = {s} is out of bounds.");

    let mut dist = vec![usize::MAX; v];
    let mut heap = BinaryHeap::from([(Reverse(0), s)]);

    dist[s] = 0;

    distances(digraph, |acc, w| acc + w, &mut dist, &mut heap);

    dist
}

/// Finds the predecessor tree and distances of the shortest paths from
/// the source vertices to all vertices in an arc-weighted digraph.
///
/// # Arguments
///
/// * `digraph`: The digraph.
/// * `step`: The function that calculates the accumulated weight.
/// * `pred`: The predecessors of the vertices.
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
///         adjacency_list_weighted::Digraph,
///         algo::{
///             dijkstra::predecessors,
///             PredecessorTree,
///         },
///         gen::Empty,
///         op::AddArcWeighted,
///     },
///     std::collections::BinaryHeap,
/// };
///
/// // 0 -> {1 (2)}
/// // 1 -> {2 (2)}
/// // 2 -> {}
/// // 3 -> {0 (2)}
///
/// let mut digraph = Digraph::<usize>::empty(4);
///
/// digraph.add_arc_weighted(0, 1, 2);
/// digraph.add_arc_weighted(1, 2, 2);
/// digraph.add_arc_weighted(3, 0, 2);
///
/// let mut pred = PredecessorTree::new(4);
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
    pred: &mut PredecessorTree,
    dist: &mut [W],
    heap: &mut BinaryHeap<(Reverse<W>, usize)>,
) where
    D: Order + OutNeighborsWeighted<W>,
    S: Fn(W, &W) -> W,
    W: Copy + Ord,
{
    while let Some((Reverse(acc), s)) = heap.pop() {
        for (t, w) in digraph.out_neighbors_weighted(s) {
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

/// Finds the predecessor tree for the shortest paths from a source vertex in
/// an arc-weighted digraph.
///
/// # Arguments
///
/// * `digraph`: The digraph.
/// * `s`: The source vertex.
///
/// # Returns
///
/// Returns the predecessor tree.
///
/// # Panics
///
/// Panics if `s` is out of bounds.
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list_weighted::Digraph,
///     algo::dijkstra::single_source_predecessors,
///     gen::Empty,
///     op::AddArcWeighted,
/// };
///
/// // 0 -> {1 (2)}
/// // 1 -> {2 (2)}
/// // 2 -> {}
/// // 3 -> {0 (2)}
///
/// let mut digraph = Digraph::<usize>::empty(4);
///
/// digraph.add_arc_weighted(0, 1, 2);
/// digraph.add_arc_weighted(1, 2, 2);
/// digraph.add_arc_weighted(3, 0, 2);
///
/// let pred = single_source_predecessors(&digraph, 0);
///
/// assert!(pred.into_iter().eq([None, Some(0), Some(1), None]));
/// ```
#[must_use]
pub fn single_source_predecessors<D>(digraph: &D, s: usize) -> PredecessorTree
where
    D: Order + OutNeighborsWeighted<usize>,
{
    let v = digraph.order();

    assert!(s < v, "s = {s} is out of bounds.");

    let mut pred = PredecessorTree::new(v);
    let mut dist = vec![usize::MAX; v];
    let mut heap = BinaryHeap::from([(Reverse(0), s)]);

    dist[s] = 0;

    predecessors(digraph, |acc, w| acc + w, &mut pred, &mut dist, &mut heap);

    pred
}

/// Finds the shortest path from the source vertex to a target vertex.
///
/// In an arc-weighted digraph, the shortest path is the path with the
/// smallest sum of weights. There can be multiple shortest paths in a digraph,
/// but this function only returns one.
///
/// # Arguments
///
/// * `digraph`: The digraph.
/// * `step`: The function that calculates the accumulated weight.
/// * `is_target`: The function determining if the vertex is a target.
/// * `pred`: The predecessors of the vertices.
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
///         adjacency_list_weighted::Digraph,
///         algo::{
///             dijkstra::shortest_path,
///             PredecessorTree,
///         },
///         gen::Empty,
///         op::AddArcWeighted,
///     },
///     std::collections::BinaryHeap,
/// };
///
/// // 0 -> {1 (2)}
/// // 1 -> {2 (2)}
/// // 2 -> {}
/// // 3 -> {0 (2)}
///
/// let mut digraph = Digraph::<usize>::empty(4);
///
/// digraph.add_arc_weighted(0, 1, 2);
/// digraph.add_arc_weighted(1, 2, 2);
/// digraph.add_arc_weighted(3, 0, 2);
///
/// let mut pred = PredecessorTree::new(4);
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
#[must_use]
pub fn shortest_path<D, S, T, W>(
    digraph: &D,
    step: S,
    is_target: T,
    pred: &mut PredecessorTree,
    dist: &mut [W],
    heap: &mut BinaryHeap<(Reverse<W>, usize)>,
) -> Option<Vec<usize>>
where
    D: Order + OutNeighborsWeighted<W>,
    S: Fn(W, &W) -> W,
    T: Fn(usize, &W) -> bool,
    W: Copy + Ord,
{
    while let Some((Reverse(acc), u)) = heap.pop() {
        if is_target(u, &acc) {
            return pred.search_by(u, |_, u| u.is_none()).map(|mut path| {
                path.reverse();

                path
            });
        }

        for (v, w) in digraph.out_neighbors_weighted(u) {
            let w = step(acc, w);

            if w >= dist[v] {
                continue;
            }

            dist[v] = w;
            pred[v] = Some(u);
            heap.push((Reverse(w), v));
        }
    }

    None
}

/// Finds the shortest path from a source vertex to a target vertex.
///
/// In an arc-weighted digraph, the shortest path is the path with the
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
/// * Panics if `s` is out of bounds.
/// * Panics if `t` is out of bounds.
/// * Panics if an intermediate vertex is out of bounds.
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list_weighted::Digraph,
///     algo::dijkstra::single_pair_shortest_path as spsp,
///     gen::Empty,
///     op::AddArcWeighted,
/// };
///
/// // 0 -> {1 (2)}
/// // 1 -> {2 (2)}
/// // 2 -> {}
/// // 3 -> {0 (2)}
///
/// let mut digraph = Digraph::<usize>::empty(4);
///
/// digraph.add_arc_weighted(0, 1, 2);
/// digraph.add_arc_weighted(1, 2, 2);
/// digraph.add_arc_weighted(3, 0, 2);
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
/// let mut digraph = Digraph::<usize>::empty(4);
///
/// digraph.add_arc_weighted(1, 0, 1);
/// digraph.add_arc_weighted(2, 1, 1);
/// digraph.add_arc_weighted(3, 0, 4);
/// digraph.add_arc_weighted(3, 2, 1);
///
/// let path = spsp(&digraph, 3, 0);
///
/// assert_eq!(path, Some(vec![3, 2, 1, 0]));
/// ```
#[doc(alias = "spsp")]
#[must_use]
pub fn single_pair_shortest_path<D>(
    digraph: &D,
    s: usize,
    t: usize,
) -> Option<Vec<usize>>
where
    D: Order + OutNeighborsWeighted<usize>,
{
    let v = digraph.order();

    assert!(s < v, "s = {s} is out of bounds.");
    assert!(t < v, "t = {t} is out of bounds.");

    let pred = &mut PredecessorTree::new(v);
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
            adjacency_list_weighted::{
                fixture::{
                    bang_jensen_94_weighted_usize,
                    bang_jensen_96_usize,
                    kattis_bryr_1_usize,
                    kattis_bryr_2_usize,
                    kattis_bryr_3_usize,
                    kattis_crosscountry_usize,
                    kattis_shortestpath1_usize,
                },
                Digraph,
            },
            gen::Empty,
        },
    };

    #[test]
    fn distances_trivial() {
        let mut dist = vec![usize::MAX; 1];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        distances(&Digraph::trivial(), |acc, w| acc + w, &mut dist, &mut heap);

        assert!(dist.iter().eq(&[0]));
    }

    #[test]
    fn distances_bang_jensen_94_weighted() {
        let mut dist = vec![usize::MAX; 7];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        distances(
            &bang_jensen_94_weighted_usize(),
            |acc, w| acc + w,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0, 1, 1, 2, 2, 2, 3]));
    }

    #[test]
    fn distances_bang_jensen_96() {
        let mut dist = vec![usize::MAX; 6];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        distances(
            &bang_jensen_96_usize(),
            |acc, w| acc + w,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0, 5, 3, 6, 4, 7]));
    }

    #[test]
    fn distances_kattis_bryr_1() {
        let mut dist = vec![usize::MAX; 3];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        distances(
            &kattis_bryr_1_usize(),
            |acc, w| acc + w,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0, 1, 1]));
    }

    #[test]
    fn distances_kattis_bryr_2() {
        let mut dist = vec![usize::MAX; 6];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        distances(
            &kattis_bryr_2_usize(),
            |acc, w| acc + w,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0, 1, 2, 1, 2, 3]));
    }

    #[test]
    fn distances_kattis_bryr_3() {
        let mut dist = vec![usize::MAX; 10];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        distances(
            &kattis_bryr_3_usize(),
            |acc, w| acc + w,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0, 0, 1, 0, 0, 0, 1, 0, 0, 1]));
    }

    #[test]
    fn distances_kattis_crosscountry() {
        let mut dist = vec![usize::MAX; 4];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        distances(
            &kattis_crosscountry_usize(),
            |acc, w| acc + w,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0, 1, 3, 10]));
    }

    #[test]
    fn distances_kattis_shortestpath1() {
        let mut dist = vec![usize::MAX; 4];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        distances(
            &kattis_shortestpath1_usize(),
            |acc, w| acc + w,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0, 2, 4, usize::MAX]));
    }

    #[test]
    fn single_source_distances_trivial() {
        assert!(single_source_distances(&Digraph::trivial(), 0)
            .iter()
            .eq(&[0]));
    }

    #[test]
    fn single_source_distances_bang_jensen_94_weighted() {
        assert!(single_source_distances(&bang_jensen_94_weighted_usize(), 0)
            .iter()
            .eq(&[0, 1, 1, 2, 2, 2, 3]));
    }

    #[test]
    fn single_source_distances_bang_jensen_96() {
        assert!(single_source_distances(&bang_jensen_96_usize(), 0)
            .iter()
            .eq(&[0, 5, 3, 6, 4, 7]));
    }

    #[test]
    fn single_source_distances_kattis_bryr_1() {
        assert!(single_source_distances(&kattis_bryr_1_usize(), 0)
            .iter()
            .eq(&[0, 1, 1]));
    }

    #[test]
    fn single_source_distances_kattis_bryr_2() {
        assert!(single_source_distances(&kattis_bryr_2_usize(), 0)
            .iter()
            .eq(&[0, 1, 2, 1, 2, 3]));
    }

    #[test]
    fn single_source_distances_kattis_bryr_3() {
        assert!(single_source_distances(&kattis_bryr_3_usize(), 0)
            .iter()
            .eq(&[0, 0, 1, 0, 0, 0, 1, 0, 0, 1]));
    }

    #[test]
    fn single_source_distances_kattis_crosscountry() {
        assert!(single_source_distances(&kattis_crosscountry_usize(), 0)
            .iter()
            .eq(&[0, 1, 3, 10]));
    }

    #[test]
    fn single_source_distances_kattis_shortestpath1() {
        assert!(single_source_distances(&kattis_shortestpath1_usize(), 0)
            .iter()
            .eq(&[0, 2, 4, usize::MAX]));
    }

    #[test]
    fn predecessors_trivial() {
        let mut pred = PredecessorTree::new(1);
        let mut dist = vec![usize::MAX; 1];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        predecessors(
            &Digraph::trivial(),
            |acc, w| acc + w,
            &mut pred,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0]));
        assert!(pred.into_iter().eq([None]));
    }

    #[test]
    fn predecessors_bang_jensen_94_weighted() {
        let mut pred = PredecessorTree::new(7);
        let mut dist = vec![usize::MAX; 7];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        predecessors(
            &bang_jensen_94_weighted_usize(),
            |acc, w| acc + w,
            &mut pred,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0, 1, 1, 2, 2, 2, 3]));

        assert!(pred.into_iter().eq([
            None,
            Some(0),
            Some(0),
            Some(2),
            Some(2),
            Some(2),
            Some(4)
        ]));
    }

    #[test]
    fn predecessors_bang_jensen_96() {
        let mut pred = PredecessorTree::new(6);
        let mut dist = vec![usize::MAX; 6];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        predecessors(
            &bang_jensen_96_usize(),
            |acc, w| acc + w,
            &mut pred,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0, 5, 3, 6, 4, 7]));

        assert!(pred.into_iter().eq([
            None,
            Some(2),
            Some(0),
            Some(4),
            Some(2),
            Some(3)
        ]));
    }

    #[test]
    fn predecessors_kattis_bryr_1() {
        let mut pred = PredecessorTree::new(3);
        let mut dist = vec![usize::MAX; 3];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        predecessors(
            &kattis_bryr_1_usize(),
            |acc, w| acc + w,
            &mut pred,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0, 1, 1]));
        assert!(pred.into_iter().eq([None, Some(0), Some(0)]));
    }

    #[test]
    fn predecessors_kattis_bryr_2() {
        let mut pred = PredecessorTree::new(6);
        let mut dist = vec![usize::MAX; 6];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        predecessors(
            &kattis_bryr_2_usize(),
            |acc, w| acc + w,
            &mut pred,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0, 1, 2, 1, 2, 3]));

        assert!(pred.into_iter().eq([
            None,
            Some(0),
            Some(3),
            Some(0),
            Some(3),
            Some(4)
        ]));
    }

    #[test]
    fn predecessors_kattis_bryr_3() {
        let mut pred = PredecessorTree::new(10);
        let mut dist = vec![usize::MAX; 10];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        predecessors(
            &kattis_bryr_3_usize(),
            |acc, w| acc + w,
            &mut pred,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0, 0, 1, 0, 0, 0, 1, 0, 0, 1]));

        assert!(pred.into_iter().eq([
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
    fn predecessors_kattis_crosscountry() {
        let mut pred = PredecessorTree::new(4);
        let mut dist = vec![usize::MAX; 4];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        predecessors(
            &kattis_crosscountry_usize(),
            |acc, w| acc + w,
            &mut pred,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0, 1, 3, 10]));
        assert!(pred.into_iter().eq([None, Some(0), Some(0), Some(2)]));
    }

    #[test]
    fn predecessors_kattis_shortestpath1() {
        let mut pred = PredecessorTree::new(4);
        let mut dist = vec![usize::MAX; 4];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        predecessors(
            &kattis_shortestpath1_usize(),
            |acc, w| acc + w,
            &mut pred,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0, 2, 4, usize::MAX]));
        assert!(pred.into_iter().eq([None, Some(0), Some(1), None]));
    }

    #[test]
    fn single_source_predecessors_trivial() {
        assert!(single_source_predecessors(&Digraph::trivial(), 0)
            .into_iter()
            .eq([None]));
    }

    #[test]
    fn single_source_predecessors_bang_jensen_94_weighted() {
        assert!(single_source_predecessors(
            &bang_jensen_94_weighted_usize(),
            0
        )
        .into_iter()
        .eq([
            None,
            Some(0),
            Some(0),
            Some(2),
            Some(2),
            Some(2),
            Some(4)
        ]));
    }

    #[test]
    fn single_source_predecessors_bang_jensen_96() {
        assert!(single_source_predecessors(&bang_jensen_96_usize(), 0)
            .into_iter()
            .eq([None, Some(2), Some(0), Some(4), Some(2), Some(3)]));
    }

    #[test]
    fn single_source_predecessors_kattis_bryr_1() {
        assert!(single_source_predecessors(&kattis_bryr_1_usize(), 0)
            .into_iter()
            .eq([None, Some(0), Some(0)]));
    }

    #[test]
    fn single_source_predecessors_kattis_bryr_2() {
        assert!(single_source_predecessors(&kattis_bryr_2_usize(), 0)
            .into_iter()
            .eq([None, Some(0), Some(3), Some(0), Some(3), Some(4)]));
    }

    #[test]
    fn single_source_predecessors_kattis_bryr_3() {
        assert!(single_source_predecessors(&kattis_bryr_3_usize(), 0)
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
        assert!(single_source_predecessors(&kattis_crosscountry_usize(), 0)
            .into_iter()
            .eq([None, Some(0), Some(0), Some(2)]));
    }

    #[test]
    fn single_source_predecessors_kattis_shortestpath1() {
        assert!(single_source_predecessors(&kattis_shortestpath1_usize(), 0)
            .into_iter()
            .eq([None, Some(0), Some(1), None]));
    }

    #[test]
    fn shortest_path_trivial() {
        let mut pred = PredecessorTree::new(1);
        let mut dist = vec![usize::MAX; 1];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        let path = shortest_path(
            &Digraph::trivial(),
            |acc, w| acc + w,
            |v, _| v == 0,
            &mut pred,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0]));
        assert!(pred.into_iter().eq([None]));
        assert!(path.unwrap().iter().eq(&[0]));
    }

    #[test]
    fn shortest_path_bang_jensen_94_weighted() {
        let mut pred = PredecessorTree::new(7);
        let mut dist = vec![usize::MAX; 7];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        let path = shortest_path(
            &bang_jensen_94_weighted_usize(),
            |acc, w| acc + w,
            |v, _| v == 6,
            &mut pred,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0, 1, 1, 2, 2, 2, 3]));

        assert!(pred.into_iter().eq([
            None,
            Some(0),
            Some(0),
            Some(2),
            Some(2),
            Some(2),
            Some(4)
        ]));

        assert!(path.unwrap().iter().eq(&[0, 2, 4, 6]));
    }

    #[test]
    fn shortest_path_bang_jensen_96() {
        let mut pred = PredecessorTree::new(6);
        let mut dist = vec![usize::MAX; 6];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        let path = shortest_path(
            &bang_jensen_96_usize(),
            |acc, w| acc + w,
            |v, _| v == 5,
            &mut pred,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0, 5, 3, 6, 4, 7]));

        assert!(pred.into_iter().eq([
            None,
            Some(2),
            Some(0),
            Some(4),
            Some(2),
            Some(3)
        ]));

        assert!(path.unwrap().iter().eq(&[0, 2, 4, 3, 5]));
    }

    #[test]
    fn shortest_path_kattis_bryr_1() {
        let mut pred = PredecessorTree::new(3);
        let mut dist = vec![usize::MAX; 3];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        let path = shortest_path(
            &kattis_bryr_1_usize(),
            |acc, w| acc + w,
            |v, _| v == 2,
            &mut pred,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0, 1, 1]));
        assert!(pred.into_iter().eq([None, Some(0), Some(0)]));
        assert!(path.unwrap().iter().eq(&[0, 2]));
    }

    #[test]
    fn shortest_path_kattis_bryr_2() {
        let mut pred = PredecessorTree::new(6);
        let mut dist = vec![usize::MAX; 6];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        let path = shortest_path(
            &kattis_bryr_2_usize(),
            |acc, w| acc + w,
            |v, _| v == 5,
            &mut pred,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0, 1, 2, 1, 2, 3]));

        assert!(pred.into_iter().eq([
            None,
            Some(0),
            Some(3),
            Some(0),
            Some(3),
            Some(4)
        ]));

        assert!(path.unwrap().iter().eq(&[0, 3, 4, 5]));
    }

    #[test]
    fn shortest_path_kattis_bryr_3() {
        let mut pred = PredecessorTree::new(10);
        let mut dist = vec![usize::MAX; 10];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        let path = shortest_path(
            &kattis_bryr_3_usize(),
            |acc, w| acc + w,
            |v, _| v == 9,
            &mut pred,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0, 0, usize::MAX, 0, 0, 0, 1, 0, 0, 1]));

        assert!(pred.into_iter().eq([
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
        ]));

        assert!(path.unwrap().iter().eq(&[0, 3, 7, 1, 9]));
    }

    #[test]
    fn shortest_path_kattis_crosscountry() {
        let mut pred = PredecessorTree::new(4);
        let mut dist = vec![usize::MAX; 4];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        let path = shortest_path(
            &kattis_crosscountry_usize(),
            |acc, w| acc + w,
            |v, _| v == 2,
            &mut pred,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0, 1, 3, 14]));
        assert!(pred.into_iter().eq([None, Some(0), Some(0), Some(0)]));
        assert!(path.unwrap().iter().eq(&[0, 2]));
    }

    #[test]
    fn shortest_path_kattis_shortestpath1() {
        let mut pred = PredecessorTree::new(4);
        let mut dist = vec![usize::MAX; 4];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        dist[0] = 0;

        let path = shortest_path(
            &kattis_shortestpath1_usize(),
            |acc, w| acc + w,
            |v, _| v == 3,
            &mut pred,
            &mut dist,
            &mut heap,
        );

        assert!(dist.iter().eq(&[0, 2, 4, usize::MAX]));
        assert!(pred.into_iter().eq([None, Some(0), Some(1), None]));
        assert_eq!(path, None);
    }

    #[test]
    fn single_pair_shortest_path_trivial() {
        assert!(single_pair_shortest_path(&Digraph::trivial(), 0, 0)
            .unwrap()
            .iter()
            .eq(&[0]));
    }

    #[test]
    fn single_pair_shortest_path_bang_jensen_94_weighted() {
        assert!(single_pair_shortest_path(
            &bang_jensen_94_weighted_usize(),
            0,
            6
        )
        .unwrap()
        .iter()
        .eq(&[0, 2, 4, 6]));
    }

    #[test]
    fn single_pair_shortest_path_bang_jensen_96() {
        assert!(single_pair_shortest_path(&bang_jensen_96_usize(), 0, 5)
            .unwrap()
            .iter()
            .eq(&[0, 2, 4, 3, 5]));
    }

    #[test]
    fn single_pair_shortest_path_kattis_bryr_1() {
        assert!(single_pair_shortest_path(&kattis_bryr_1_usize(), 0, 2)
            .unwrap()
            .iter()
            .eq(&[0, 2]));
    }

    #[test]
    fn single_pair_shortest_path_kattis_bryr_2() {
        assert!(single_pair_shortest_path(&kattis_bryr_2_usize(), 0, 5)
            .unwrap()
            .iter()
            .eq(&[0, 3, 4, 5]));
    }

    #[test]
    fn single_pair_shortest_path_kattis_bryr_3() {
        assert!(single_pair_shortest_path(&kattis_bryr_3_usize(), 0, 9)
            .unwrap()
            .iter()
            .eq(&[0, 3, 7, 1, 9]));
    }

    #[test]
    fn single_pair_shortest_path_kattis_crosscountry() {
        assert!(
            single_pair_shortest_path(&kattis_crosscountry_usize(), 0, 2)
                .unwrap()
                .iter()
                .eq(&[0, 2])
        );
    }

    #[test]
    fn single_pair_shortest_path_kattis_shortestpath1() {
        assert_eq!(
            single_pair_shortest_path(&kattis_shortestpath1_usize(), 0, 3),
            None
        );
    }
}

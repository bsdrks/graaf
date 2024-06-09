//! Breadth-first search
//!
//! Breadth-first search is a graph traversal algorithm that visits
//! vertices of an unweighted digraph in order of their distance from the source
//! vertex. Use [`dijkstra`] for weighted digraphs.
//!
//! The implementations use distances instead of a set or boolean array to check
//! if it has already visited a vertex because it already calculates these
//! distances during traversal.
//!
//! The time complexity is *O*(*v* + *a*).
//!
//! # Examples
//!
//! The separate calls to `single_source_distances` and
//! `single_source_predecessors` in the example are for illustrative purposes
//! only; use [`predecessors`] if you need the predecessor tree *and* distances.
//!
//! ```
//! use graaf::{
//!     algo::bfs::{
//!         single_source_distances,
//!         single_source_predecessors,
//!     },
//!     gen::EmptyConst,
//!     op::AddArc,
//! };
//!
//! // 0 -> {1}
//! // 1 -> {2}
//! // 2 -> {}
//! // 3 -> {0}
//!
//! let mut digraph = <[Vec<usize>; 4]>::empty();
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(3, 0);
//!
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

use {
    crate::{
        algo::predecessor,
        op::{
            IterOutNeighbors,
            Order,
        },
    },
    std::collections::VecDeque,
};

/// Calculates all distances from the source vertices.
///
/// # Arguments
///
/// * `digraph`: The digraph.
/// * `step`: The function to calculate the new weight.
/// * `dist`: The distances from the source vertices.
/// * `queue`: The source vertices.
///
/// # Panics
///
/// * Panics if `step` panics.
/// * Panics if a source or successor vertex is not in `dist`.
/// * Panics if a source or successor vertex is not in `digraph`.
///
/// # Examples
///
/// ```
/// use {
///     graaf::{
///         algo::bfs::distances,
///         gen::Empty,
///         op::AddArc,
///     },
///     std::collections::VecDeque,
/// };
///
/// // 0 -> {1}
/// // 1 -> {2}
/// // 2 -> {}
/// // 3 -> {0}
///
/// let mut digraph = Vec::<Vec<usize>>::empty(4);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(3, 0);
///
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
/// * `digraph`: The digraph.
/// * `s`: The source vertex.
///
/// # Returns
///
/// Returns the BFS tree.
///
/// # Panics
///
/// Panics if `s` is out of bounds.
///
/// # Examples
///
/// ```
/// use graaf::{
///     algo::bfs::single_source_distances,
///     gen::EmptyConst,
///     op::AddArc,
/// };
///
/// // 0 -> {1}
/// // 1 -> {2}
/// // 2 -> {}
/// // 3 -> {0}
///
/// let mut digraph = <[Vec<usize>; 4]>::empty();
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(3, 0);
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
/// * `digraph`: The digraph.
/// * `step`: The function that calculates the accumulated weight.
/// * `pred`: The predecessors on the shortest paths from the source vertices.
/// * `dist`: The distances from the source vertices.
/// * `queue`: The source vertices.
///
/// # Panics
///
/// * Panics if `step` panics.
/// * Panics if a source or successor vertex is not in `dist`.
/// * Panics if a source or successor vertex is not in `digraph`.
/// * Panics if a source or successor vertex is not in `pred`.
///
/// # Examples
///
/// ```
/// use {
///     core::cmp::Reverse,
///     graaf::{
///         algo::bfs::predecessors,
///         gen::Empty,
///         op::AddArc,
///     },
///     std::collections::VecDeque,
/// };
///
/// // 0 -> {1}
/// // 1 -> {2}
/// // 2 -> {}
/// // 3 -> {0}
///
/// let mut digraph = <[Vec<usize>; 4]>::empty();
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(3, 0);
///
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
///     algo::bfs::single_source_predecessors,
///     gen::EmptyConst,
///     op::AddArc,
/// };
///
/// // 0 -> {1}
/// // 1 -> {2}
/// // 2 -> {}
/// // 3 -> {0}
///
/// let mut digraph = <[Vec<usize>; 4]>::empty();
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(3, 0);
///
/// let pred = single_source_predecessors(&digraph, 0);
///
/// assert_eq!(pred, [None, Some(0), Some(1), None]);
/// ```
pub fn single_source_predecessors<D>(digraph: &D, s: usize) -> Vec<Option<usize>>
where
    D: Order + IterOutNeighbors,
{
    let v = digraph.order();
    let mut pred = vec![None; v];
    let mut dist = vec![usize::MAX; v];
    let mut queue = VecDeque::from(vec![(s, 0)]);

    dist[s] = 0;

    predecessors(digraph, |w| w + 1, &mut pred, &mut dist, &mut queue);

    pred
}

/// Calculates the shortest path from the source vertex to a target vertex.
///
/// # Arguments
///
/// * `digraph`: The digraph.
/// * `step`: The function that calculates the accumulated distance.
/// * `is_target`: The function determining whether the vertex is a target.
/// * `pred`: The predecessors on the shortest paths from the source vertices.
/// * `dist`: The distances from the source vertices.
/// * `source`: The source vertices.
///
/// # Returns
///
/// If it finds a target vertex, the function returns the shortest path from the
/// source vertex to this target vertex. Otherwise, it returns `None`.
///
/// # Panics
///
/// * Panics if `is_target` panics.
/// * Panics if `step` panics.
/// * Panics if a source or successor vertex is not in `dist`.
/// * Panics if a source or successor vertex is not in `digraph`.
/// * Panics if a source or successor vertex is not in `pred`.
///
/// # Examples
///
/// ```
/// use {
///     graaf::{
///         algo::bfs::shortest_path,
///         gen::EmptyConst,
///         op::AddArc,
///     },
///     std::collections::VecDeque,
/// };
///
/// // 0 -> {1}
/// // 1 -> {2}
/// // 2 -> {}
/// // 3 -> {0}
///
/// let mut digraph = <[Vec<usize>; 4]>::empty();
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(3, 0);
///
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
/// * `digraph`: The digraph.
/// * `s`: The source vertex.
/// * `t`: The target vertex.
///
/// # Returns
///
/// If a path from `s` to `t` exists, the function returns the shortest path
/// from `s` to `t`. Otherwise, it returns `None`.
///
/// # Panics
///
/// Panics if `s`, `t`, or an intermediate vertex is out of bounds.
///
/// # Examples
///
/// ```
/// use graaf::{
///     algo::bfs::single_pair_shortest_path as spsp,
///     gen::EmptyConst,
///     op::AddArc,
/// };
///
/// // 0 -> {1}
/// // 1 -> {2}
/// // 2 -> {}
/// // 3 -> {0}
///
/// let mut digraph = <[Vec<usize>; 4]>::empty();
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(3, 0);
///
/// let path = spsp(&digraph, 3, 2);
///
/// assert_eq!(path, Some(vec![3, 0, 1, 2]));
///
/// // 0 -> {}
/// // 1 -> {0}
/// // 2 -> {1}
/// // 3 -> {0, 2}
///
/// let mut digraph = <[Vec<usize>; 4]>::empty();
///
/// digraph.add_arc(1, 0);
/// digraph.add_arc(2, 1);
/// digraph.add_arc(3, 0);
/// digraph.add_arc(3, 2);
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
    let v = digraph.order();
    let mut pred = vec![None; v];
    let mut dist = vec![usize::MAX; v];
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
    use crate::{
        algo::fixture,
        gen::Empty,
    };

    use super::*;

    #[test]
    fn distances_trivial() {
        let digraph = Vec::<Vec<usize>>::trivial();
        let mut dist = vec![0];
        let mut queue = VecDeque::new();

        distances(&digraph, |w: usize| w + 1, &mut dist, &mut queue);

        assert!(dist.iter().eq(&[0]));
    }

    #[test]
    fn distances_bang_jensen_94() {
        let digraph = fixture::bang_jensen_94();
        let mut dist = [usize::MAX; 7];
        let mut queue = VecDeque::from([(0, 0)]);

        dist[0] = 0;

        distances(&digraph, |w| w + 1, &mut dist, &mut queue);

        assert!(dist.iter().eq(&[0, 1, 1, 2, 2, 2, 3]));
    }

    #[test]
    fn single_source_distances_trivial() {
        assert!(single_source_distances(&Vec::<Vec<usize>>::trivial(), 0)
            .iter()
            .eq(&[0]));
    }

    #[test]
    fn single_source_distancesbang_jensen_94() {
        assert!(single_source_distances(&fixture::bang_jensen_94(), 0)
            .iter()
            .eq(&[0, 1, 1, 2, 2, 2, 3]));
    }

    #[test]
    fn predecessors_trivial() {
        let digraph = Vec::<Vec<usize>>::trivial();
        let mut pred = vec![None];
        let mut dist = vec![0];
        let mut queue = VecDeque::new();

        predecessors(&digraph, |w: usize| w + 1, &mut pred, &mut dist, &mut queue);

        assert!(pred.iter().eq(&[None]));
        assert!(dist.iter().eq(&[0]));
    }

    #[test]
    fn predecessors_bang_jensen_94() {
        let digraph = fixture::bang_jensen_94();
        let mut pred = [None; 7];
        let mut dist = [usize::MAX; 7];
        let mut queue = VecDeque::from([(0, 0)]);

        dist[0] = 0;

        predecessors(&digraph, |w| w + 1, &mut pred, &mut dist, &mut queue);

        assert_eq!(
            pred,
            [None, Some(0), Some(0), Some(1), Some(2), Some(2), Some(4)]
        );

        assert_eq!(dist, [0, 1, 1, 2, 2, 2, 3]);
    }

    #[test]
    fn single_source_predecessors_trivial() {
        assert!(single_source_predecessors(&Vec::<Vec<usize>>::trivial(), 0)
            .iter()
            .eq(&[None]));
    }

    #[test]
    fn single_source_predecessors_bang_jensen_94() {
        assert!(single_source_predecessors(&fixture::bang_jensen_94(), 0)
            .iter()
            .eq(&[None, Some(0), Some(0), Some(1), Some(2), Some(2), Some(4)]));
    }

    #[test]
    fn shortest_path_trivial() {
        let digraph = Vec::<Vec<usize>>::trivial();
        let mut pred = vec![None];
        let mut dist = vec![0];
        let mut queue = VecDeque::new();

        let path = shortest_path(
            &digraph,
            |w: usize| w + 1,
            |t| t == 0,
            &mut pred,
            &mut dist,
            &mut queue,
        );

        assert!(pred.iter().eq(&[None]));
        assert!(dist.iter().eq(&[0]));
        assert!(path.is_none());
    }

    #[test]
    fn shortest_path_bang_jensen_94() {
        let digraph = fixture::bang_jensen_94();
        let mut pred = [None; 7];
        let mut dist = [usize::MAX; 7];
        let mut queue = VecDeque::from([(0, 0)]);

        dist[0] = 0;

        let path = shortest_path(
            &digraph,
            |w| w + 1,
            |t| t == 6,
            &mut pred,
            &mut dist,
            &mut queue,
        );

        assert!(pred
            .iter()
            .eq(&[None, Some(0), Some(0), Some(1), Some(2), Some(2), Some(4)]));

        assert!(dist.iter().eq(&[0, 1, 1, 2, 2, 2, 3]));
        assert!(path.unwrap().iter().eq(&[0, 2, 4, 6]));
    }

    #[test]
    fn single_pair_shortest_path_trivial() {
        assert_eq!(
            single_pair_shortest_path(&Vec::<Vec<usize>>::trivial(), 0, 0),
            None
        );
    }

    #[test]
    fn single_pair_shortest_path_bang_jensen_94() {
        assert!(single_pair_shortest_path(&fixture::bang_jensen_94(), 0, 6)
            .unwrap()
            .iter()
            .eq(&[0, 2, 4, 6]));
    }
}

//! Breadth-first search
//!
//! Breadth-first search is a digraph traversal algorithm that visits
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
//! only; use [`predecessors`] if you need the breadth-first tree *and*
//! distances.
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     algo::bfs::{
//!         single_source_distances,
//!         single_source_predecessors,
//!     },
//!     gen::Empty,
//!     op::AddArc,
//! };
//!
//! // 0 -> {1}
//! // 1 -> {2}
//! // 2 -> {}
//! // 3 -> {0}
//!
//! let mut digraph = Digraph::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(3, 0);
//!
//! let dist = single_source_distances(&digraph, 0);
//! let pred = single_source_predecessors(&digraph, 0);
//!
//! assert!(pred.into_iter().eq([None, Some(0), Some(1), None]));
//! assert_eq!(dist, [0, 1, 2, usize::MAX]);
//!
//! let dist = single_source_distances(&digraph, 1);
//! let pred = single_source_predecessors(&digraph, 1);
//!
//! assert!(pred.into_iter().eq([None, None, Some(1), None]));
//! assert_eq!(dist, [usize::MAX, 0, 1, usize::MAX]);
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
//! assert_eq!(dist, [1, 2, 3, 0]);
//! ```
//!
//! [`dijkstra`]: crate::algo::dijkstra

use {
    crate::{
        algo::breadth_first_tree::BreadthFirstTree,
        op::{
            Order,
            OutNeighbors,
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
///         adjacency_list::Digraph,
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
/// let mut digraph = Digraph::empty(4);
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
    D: OutNeighbors,
    S: Fn(W) -> W,
    W: Copy + Ord,
{
    while let Some((s, w)) = queue.pop_front() {
        let w = step(w);

        for t in digraph.out_neighbors(s) {
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
///     adjacency_list::Digraph,
///     algo::bfs::single_source_distances,
///     gen::Empty,
///     op::AddArc,
/// };
///
/// // 0 -> {1}
/// // 1 -> {2}
/// // 2 -> {}
/// // 3 -> {0}
///
/// let mut digraph = Digraph::empty(4);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(3, 0);
///
/// assert_eq!(single_source_distances(&digraph, 0), [0, 1, 2, usize::MAX]);
/// ```
pub fn single_source_distances<D>(digraph: &D, s: usize) -> Vec<usize>
where
    D: Order + OutNeighbors,
{
    let mut dist = vec![usize::MAX; digraph.order()];
    let mut queue = VecDeque::from(vec![(s, 0)]);

    dist[s] = 0;

    distances(digraph, |w| w + 1, &mut dist, &mut queue);

    dist
}

/// Calculates the breadth-first tree and distances from the source vertices.
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
///         adjacency_list::Digraph,
///         algo::{
///             bfs::predecessors,
///             breadth_first_tree::BreadthFirstTree,
///         },
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
/// let mut digraph = Digraph::empty(4);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(3, 0);
///
/// let mut pred = BreadthFirstTree::new(4);
/// let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
/// let mut queue = VecDeque::from([(0, 0)]);
///
/// predecessors(&digraph, |w| w + 1, &mut pred, &mut dist, &mut queue);
///
/// assert!(pred.into_iter().eq([None, Some(0), Some(1), None]));
/// assert_eq!(dist, [0, 1, 2, usize::MAX]);
/// ```
pub fn predecessors<D, S, W>(
    digraph: &D,
    step: S,
    pred: &mut BreadthFirstTree,
    dist: &mut [W],
    queue: &mut VecDeque<(usize, W)>,
) where
    D: OutNeighbors,
    S: Fn(W) -> W,
    W: Copy + Ord,
{
    while let Some((s, w)) = queue.pop_front() {
        let w = step(w);

        for t in digraph.out_neighbors(s) {
            if w >= dist[t] {
                continue;
            }

            dist[t] = w;
            pred[t] = Some(s);
            queue.push_back((t, w));
        }
    }
}

/// Calculates the breadth-first tree for the shortest paths from a single
/// source vertex.
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
///     adjacency_list::Digraph,
///     algo::bfs::single_source_predecessors,
///     gen::Empty,
///     op::AddArc,
/// };
///
/// // 0 -> {1}
/// // 1 -> {2}
/// // 2 -> {}
/// // 3 -> {0}
///
/// let mut digraph = Digraph::empty(4);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(3, 0);
///
/// let pred = single_source_predecessors(&digraph, 0);
///
/// assert!(pred.into_iter().eq([None, Some(0), Some(1), None]));
/// ```
pub fn single_source_predecessors<D>(digraph: &D, s: usize) -> BreadthFirstTree
where
    D: Order + OutNeighbors,
{
    let v = digraph.order();
    let mut pred = BreadthFirstTree::new(v);
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
/// * `pred`: The breadth-first tree.
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
///         adjacency_list::Digraph,
///         algo::{
///             bfs::shortest_path,
///             breadth_first_tree::BreadthFirstTree,
///         },
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
/// let mut digraph = Digraph::empty(4);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(3, 0);
///
/// let mut pred = BreadthFirstTree::new(4);
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
    pred: &mut BreadthFirstTree,
    dist: &mut [usize],
    queue: &mut VecDeque<(usize, usize)>,
) -> Option<Vec<usize>>
where
    D: OutNeighbors,
    S: Fn(usize) -> usize,
    T: Fn(usize) -> bool,
{
    while let Some((s, w)) = queue.pop_front() {
        if is_target(s) {
            return pred.search_by(s, |_, b| b.is_none()).map(|mut path| {
                path.reverse();

                path
            });
        }

        let w = step(w);

        for t in digraph.out_neighbors(s) {
            if w >= dist[t] {
                continue;
            }

            dist[t] = w;
            pred[t] = Some(s);

            if is_target(t) {
                return pred.search_by(t, |_, b| b.is_none()).map(|mut path| {
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
///     adjacency_list::Digraph,
///     algo::bfs::single_pair_shortest_path as spsp,
///     gen::Empty,
///     op::AddArc,
/// };
///
/// // 0 -> {1}
/// // 1 -> {2}
/// // 2 -> {}
/// // 3 -> {0}
///
/// let mut digraph = Digraph::empty(4);
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
/// let mut digraph = Digraph::empty(4);
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
    D: Order + OutNeighbors,
{
    let v = digraph.order();
    let mut pred = BreadthFirstTree::new(v);
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
    use {
        crate::{
            adjacency_list::{
                fixture,
                Digraph,
            },
            gen::Empty,
        },
        std::collections::BTreeSet,
    };

    use super::*;

    #[test]
    fn distances_trivial() {
        let digraph = Digraph::trivial();
        let mut dist = vec![0];
        let mut queue = VecDeque::new();

        distances(&digraph, |w: usize| w + 1, &mut dist, &mut queue);

        assert!(dist.iter().eq(&[0]));
    }

    #[test]
    fn distances_bang_jensen_94() {
        let digraph = fixture::bang_jensen_94!();
        let mut dist = [usize::MAX; 7];
        let mut queue = VecDeque::from([(0, 0)]);

        dist[0] = 0;

        distances(&digraph, |w| w + 1, &mut dist, &mut queue);

        assert!(dist.iter().eq(&[0, 1, 1, 2, 2, 2, 3]));
    }

    #[test]
    fn distances_kattis_escapewallmaria_1() {
        let digraph = fixture::kattis_escapewallmaria_1!();
        let mut dist = [usize::MAX; 16];
        let mut queue = VecDeque::from([(5, 0)]);

        dist[5] = 0;

        distances(&digraph, |w| w + 1, &mut dist, &mut queue);

        let mut dist_expected = [usize::MAX; 16];

        dist_expected[5] = 0;
        dist_expected[6] = 1;
        dist_expected[9] = 1;
        dist_expected[12] = 3;
        dist_expected[13] = 2;

        assert!(dist.iter().eq(&dist_expected));
    }

    #[test]
    fn distances_kattis_escapewallmaria_2() {
        let digraph = fixture::kattis_escapewallmaria_2!();
        let mut dist = [usize::MAX; 16];
        let mut queue = VecDeque::from([(5, 0)]);

        dist[5] = 0;

        distances(&digraph, |w| w + 1, &mut dist, &mut queue);

        let mut dist_expected = [usize::MAX; 16];

        dist_expected[5] = 0;
        dist_expected[6] = 1;
        dist_expected[9] = 1;

        assert!(dist.iter().eq(&dist_expected));
    }

    #[test]
    fn distances_kattis_escapewallmaria_3() {
        let digraph = fixture::kattis_escapewallmaria_3!();
        let mut dist = [usize::MAX; 16];
        let mut queue = VecDeque::from([(1, 0)]);

        dist[1] = 0;

        distances(&digraph, |w| w + 1, &mut dist, &mut queue);

        let mut dist_expected = [usize::MAX; 16];

        dist_expected[1] = 0;
        dist_expected[2] = 1;
        dist_expected[5] = 1;
        dist_expected[6] = 2;
        dist_expected[9] = 2;
        dist_expected[12] = 4;
        dist_expected[13] = 3;

        assert!(dist.iter().eq(&dist_expected));
    }

    #[test]
    fn single_source_distances_trivial() {
        assert!(single_source_distances(&Digraph::trivial(), 0)
            .iter()
            .eq(&[0]));
    }

    #[test]
    fn single_source_distancesbang_jensen_94() {
        assert!(single_source_distances(&fixture::bang_jensen_94!(), 0)
            .iter()
            .eq(&[0, 1, 1, 2, 2, 2, 3]));
    }

    #[test]
    fn predecessors_trivial() {
        let digraph = Digraph::trivial();
        let mut pred = BreadthFirstTree::new(1);
        let mut dist = vec![0];
        let mut queue = VecDeque::new();

        predecessors(&digraph, |w: usize| w + 1, &mut pred, &mut dist, &mut queue);

        assert!(pred.into_iter().eq([None]));
        assert!(dist.iter().eq(&[0]));
    }

    #[test]
    fn predecessors_bang_jensen_94() {
        let digraph = fixture::bang_jensen_94!();
        let mut pred = BreadthFirstTree::new(7);
        let mut dist = [usize::MAX; 7];
        let mut queue = VecDeque::from([(0, 0)]);

        dist[0] = 0;

        predecessors(&digraph, |w| w + 1, &mut pred, &mut dist, &mut queue);

        assert!(pred
            .into_iter()
            .eq([None, Some(0), Some(0), Some(1), Some(2), Some(2), Some(4)]));

        assert_eq!(dist, [0, 1, 1, 2, 2, 2, 3]);
    }

    #[test]
    fn predecessors_kattis_escapewallmaria_1() {
        let digraph = fixture::kattis_escapewallmaria_1!();
        let mut pred = BreadthFirstTree::new(16);
        let mut dist = [usize::MAX; 16];
        let mut queue = VecDeque::from([(5, 0)]);

        dist[5] = 0;

        predecessors(&digraph, |w| w + 1, &mut pred, &mut dist, &mut queue);

        let mut pred_expected = [None; 16];

        pred_expected[6] = Some(5);
        pred_expected[9] = Some(5);
        pred_expected[12] = Some(13);
        pred_expected[13] = Some(9);

        assert!(pred.into_iter().eq(pred_expected));

        let mut dist_expected = [usize::MAX; 16];

        dist_expected[5] = 0;
        dist_expected[6] = 1;
        dist_expected[9] = 1;
        dist_expected[12] = 3;
        dist_expected[13] = 2;

        assert!(dist.iter().eq(&dist_expected));
    }

    #[test]
    fn predecessors_kattis_escapewallmaria_2() {
        let digraph = fixture::kattis_escapewallmaria_2!();
        let mut pred = BreadthFirstTree::new(16);
        let mut dist = [usize::MAX; 16];
        let mut queue = VecDeque::from([(5, 0)]);

        dist[5] = 0;

        predecessors(&digraph, |w| w + 1, &mut pred, &mut dist, &mut queue);

        let mut pred_expected = [None; 16];

        pred_expected[6] = Some(5);
        pred_expected[9] = Some(5);

        assert!(pred.into_iter().eq(pred_expected));

        let mut dist_expected = [usize::MAX; 16];

        dist_expected[5] = 0;
        dist_expected[6] = 1;
        dist_expected[9] = 1;

        assert!(dist.iter().eq(&dist_expected));
    }

    #[test]
    fn predecessors_kattis_escapewallmaria_3() {
        let digraph = fixture::kattis_escapewallmaria_3!();
        let mut pred = BreadthFirstTree::new(16);
        let mut dist = [usize::MAX; 16];
        let mut queue = VecDeque::from([(1, 0)]);

        dist[1] = 0;

        predecessors(&digraph, |w| w + 1, &mut pred, &mut dist, &mut queue);

        let mut pred_expected = [None; 16];

        pred_expected[2] = Some(1);
        pred_expected[5] = Some(1);
        pred_expected[6] = Some(2);
        pred_expected[9] = Some(5);
        pred_expected[12] = Some(13);
        pred_expected[13] = Some(9);

        assert!(pred.into_iter().eq(pred_expected));

        let mut dist_expected = [usize::MAX; 16];

        dist_expected[1] = 0;
        dist_expected[2] = 1;
        dist_expected[5] = 1;
        dist_expected[6] = 2;
        dist_expected[9] = 2;
        dist_expected[12] = 4;
        dist_expected[13] = 3;

        assert!(dist.iter().eq(&dist_expected));
    }

    #[test]
    fn single_source_predecessors_trivial() {
        assert!(single_source_predecessors(&Digraph::trivial(), 0)
            .into_iter()
            .eq([None]));
    }

    #[test]
    fn single_source_predecessors_bang_jensen_94() {
        assert!(single_source_predecessors(&fixture::bang_jensen_94!(), 0)
            .into_iter()
            .eq([None, Some(0), Some(0), Some(1), Some(2), Some(2), Some(4)]));
    }

    #[test]
    fn shortest_path_trivial() {
        let digraph = Digraph::trivial();
        let mut pred = BreadthFirstTree::new(1);
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

        assert!(pred.into_iter().eq([None]));
        assert!(dist.iter().eq(&[0]));
        assert!(path.is_none());
    }

    #[test]
    fn shortest_path_bang_jensen_94() {
        let digraph = fixture::bang_jensen_94!();
        let mut pred = BreadthFirstTree::new(7);
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
            .into_iter()
            .eq([None, Some(0), Some(0), Some(1), Some(2), Some(2), Some(4)]));

        assert!(dist.iter().eq(&[0, 1, 1, 2, 2, 2, 3]));
        assert!(path.unwrap().iter().eq(&[0, 2, 4, 6]));
    }

    #[test]
    fn shortest_path_kattis_escapewallmaria_1() {
        let digraph = fixture::kattis_escapewallmaria_1!();
        let border = BTreeSet::from([0, 1, 2, 3, 4, 7, 8, 11, 12, 13, 14, 15]);
        let mut pred = BreadthFirstTree::new(16);
        let mut dist = [usize::MAX; 16];
        let mut queue = VecDeque::from([(5, 0)]);

        dist[5] = 0;

        let path = shortest_path(
            &digraph,
            |w| w + 1,
            |t| border.contains(&t),
            &mut pred,
            &mut dist,
            &mut queue,
        );

        let mut pred_expected = [None; 16];

        pred_expected[6] = Some(5);
        pred_expected[9] = Some(5);
        pred_expected[13] = Some(9);

        assert!(pred.into_iter().eq(pred_expected));

        let mut dist_expected = [usize::MAX; 16];

        dist_expected[5] = 0;
        dist_expected[6] = 1;
        dist_expected[9] = 1;
        dist_expected[13] = 2;

        assert!(dist.iter().eq(&dist_expected));
        assert!(path.unwrap().iter().eq(&[5, 9, 13]));
    }

    #[test]
    fn shortest_path_kattis_escapewallmaria_2() {
        let digraph = fixture::kattis_escapewallmaria_2!();
        let border = BTreeSet::from([0, 1, 2, 3, 4, 7, 8, 11, 12, 13, 14, 15]);
        let mut pred = BreadthFirstTree::new(16);
        let mut dist = [usize::MAX; 16];
        let mut queue = VecDeque::from([(5, 0)]);

        dist[5] = 0;

        let path = shortest_path(
            &digraph,
            |w| w + 1,
            |t| border.contains(&t),
            &mut pred,
            &mut dist,
            &mut queue,
        );

        let mut pred_expected = [None; 16];

        pred_expected[6] = Some(5);
        pred_expected[9] = Some(5);

        assert!(pred.into_iter().eq(pred_expected));

        let mut dist_expected = [usize::MAX; 16];

        dist_expected[5] = 0;
        dist_expected[6] = 1;
        dist_expected[9] = 1;

        assert!(dist.iter().eq(&dist_expected));
        assert_eq!(path, None);
    }

    #[test]
    fn shortest_path_kattis_escapewallmaria_3() {
        let digraph = fixture::kattis_escapewallmaria_3!();
        let border = BTreeSet::from([0, 1, 2, 3, 4, 7, 8, 11, 12, 13, 14, 15]);
        let mut pred = BreadthFirstTree::new(16);
        let mut dist = [usize::MAX; 16];
        let mut queue = VecDeque::from([(1, 0)]);

        dist[1] = 0;

        let path = shortest_path(
            &digraph,
            |w| w + 1,
            |t| border.contains(&t),
            &mut pred,
            &mut dist,
            &mut queue,
        );

        assert!(pred.into_iter().eq([None; 16]));

        let mut dist_expected = [usize::MAX; 16];

        dist_expected[1] = 0;

        assert!(dist.iter().eq(&dist_expected));
        assert_eq!(path, Some(vec![1]));
    }

    #[test]
    fn single_pair_shortest_path_trivial() {
        assert_eq!(
            single_pair_shortest_path(&Digraph::trivial(), 0, 0),
            Some(vec![0])
        );
    }

    #[test]
    fn single_pair_shortest_path_bang_jensen_94() {
        assert!(single_pair_shortest_path(&fixture::bang_jensen_94!(), 0, 6)
            .unwrap()
            .iter()
            .eq(&[0, 2, 4, 6]));
    }
}

//! The Bellman-Ford-Moore algorithm
//!
//! The Bellman-Ford-Moore algorithm is a single-source shortest-path algorithm
//! for weighted digraphs.
//!
//! The time complexity is *O*(*ve*).
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     algo::bellman_ford_moore::single_source_distances,
//!     gen::Empty,
//!     op::AddWeightedArc,
//! };
//!
//! // 0 -> {1 (8), 2 (4)}
//! // 1 -> {2 (-5)}
//! // 2 -> {3 (-2), 4 (4)}
//! // 3 -> {5 (-2)}
//! // 4 -> {3 (10), 5 (9)}
//! // 5 -> {3 (5), 4 (-3)}
//!
//! let mut digraph = Vec::<Vec<(usize, isize)>>::empty(6);
//!
//! digraph.add_weighted_arc(0, 1, 8);
//! digraph.add_weighted_arc(0, 2, 4);
//! digraph.add_weighted_arc(1, 2, -5);
//! digraph.add_weighted_arc(2, 3, -2);
//! digraph.add_weighted_arc(2, 4, 4);
//! digraph.add_weighted_arc(3, 5, -2);
//! digraph.add_weighted_arc(4, 3, 10);
//! digraph.add_weighted_arc(4, 5, 9);
//! digraph.add_weighted_arc(5, 3, 5);
//! digraph.add_weighted_arc(5, 4, -3);
//!
//! assert!(single_source_distances(&digraph, 0)
//!     .unwrap()
//!     .eq(&[0, 8, 3, 1, -4, -1]));
//!
//! // 0 -> {1 (-2)}
//! // 1 -> {2 (-1)}
//! // 2 -> {0 (-1)}
//!
//! let mut digraph = Vec::<Vec<(usize, isize)>>::empty(3);
//!
//! digraph.add_weighted_arc(0, 1, -2);
//! digraph.add_weighted_arc(1, 2, -1);
//! digraph.add_weighted_arc(2, 0, -1);
//!
//! assert_eq!(single_source_distances(&digraph, 0), None);
//! ```

use crate::op::{
    IterWeightedArcs,
    Order,
};

/// Computes the distances from a source vertex to all other vertices in a
/// weighted digraph.
///
/// # Arguments
///
/// * `digraph`: The weighted digraph.
/// * `s`: The source vertex.
///
/// # Returns
///
/// Returns the distances from the source vertex to all other vertices. Returns
/// `None` if the digraph contains a negative cycle.
///
/// # Examples
///
/// ```
/// use graaf::{
///     algo::bellman_ford_moore::single_source_distances,
///     gen::Empty,
///     op::AddWeightedArc,
/// };
///
/// // 0 -> {1 (8), 2 (4)}
/// // 1 -> {2 (-5)}
/// // 2 -> {3 (-2), 4 (4)}
/// // 3 -> {5 (-2)}
/// // 4 -> {3 (10), 5 (9)}
/// // 5 -> {3 (5), 4 (-3)}
///
/// let mut digraph = Vec::<Vec<(usize, isize)>>::empty(6);
///
/// digraph.add_weighted_arc(0, 1, 8);
/// digraph.add_weighted_arc(0, 2, 4);
/// digraph.add_weighted_arc(1, 2, -5);
/// digraph.add_weighted_arc(2, 3, -2);
/// digraph.add_weighted_arc(2, 4, 4);
/// digraph.add_weighted_arc(3, 5, -2);
/// digraph.add_weighted_arc(4, 3, 10);
/// digraph.add_weighted_arc(4, 5, 9);
/// digraph.add_weighted_arc(5, 3, 5);
/// digraph.add_weighted_arc(5, 4, -3);
///
/// assert!(single_source_distances(&digraph, 0)
///     .unwrap()
///     .eq(&[0, 8, 3, 1, -4, -1]));
///
/// // 0 -> {1 (-2)}
/// // 1 -> {2 (-1)}
/// // 2 -> {0 (-1)}
///
/// let mut digraph = Vec::<Vec<(usize, isize)>>::empty(3);
///
/// digraph.add_weighted_arc(0, 1, -2);
/// digraph.add_weighted_arc(1, 2, -1);
/// digraph.add_weighted_arc(2, 0, -1);
///
/// assert_eq!(single_source_distances(&digraph, 0), None);
/// ```
pub fn single_source_distances<D>(digraph: &D, s: usize) -> Option<Vec<isize>>
where
    D: IterWeightedArcs<isize> + Order,
{
    let v = digraph.order();
    let mut dist = vec![isize::MAX; v];

    dist[s] = 0;

    for _ in 1..v {
        for (s, t, w) in digraph.iter_weighted_arcs() {
            dist[t] = dist[t].min(dist[s].saturating_add(*w));
        }
    }

    for (s, t, w) in digraph.iter_weighted_arcs() {
        if dist[t] > dist[s].saturating_add(*w) {
            return None;
        }
    }

    Some(dist)
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

    #[test]
    fn trivial() {
        assert!(
            single_source_distances(&Vec::<Vec<(usize, isize)>>::trivial(), 0)
                .unwrap()
                .iter()
                .eq(&[0])
        );
    }

    #[test]
    fn bang_jensen_94() {
        assert!(
            single_source_distances(&fixture::bang_jensen_94_weighted!(), 0)
                .unwrap()
                .eq(&[0, 1, 1, 2, 2, 2, 3])
        );
    }

    #[test]
    fn bang_jensen_96() {
        assert!(single_source_distances(&fixture::bang_jensen_96!(), 0)
            .unwrap()
            .eq(&[0, 5, 3, 6, 4, 7]));
    }

    #[test]
    fn bang_jensen_99() {
        assert!(single_source_distances(&fixture::bang_jensen_99!(), 0)
            .unwrap()
            .eq(&[0, 8, 3, 1, -4, -1]));
    }

    #[test]
    fn kattis_bryr_1() {
        assert!(single_source_distances(&fixture::kattis_bryr_1!(), 0)
            .unwrap()
            .eq(&[0, 1, 1]));
    }

    #[test]
    fn kattis_bryr_2() {
        assert!(single_source_distances(&fixture::kattis_bryr_2!(), 0)
            .unwrap()
            .eq(&[0, 1, 2, 1, 2, 3]));
    }

    #[test]
    fn kattis_bryr_3() {
        assert!(single_source_distances(&fixture::kattis_bryr_3!(), 0)
            .unwrap()
            .eq(&[0, 0, 1, 0, 0, 0, 1, 0, 0, 1]));
    }

    #[test]
    fn kattis_crosscountry() {
        assert!(single_source_distances(&fixture::kattis_crosscountry!(), 0)
            .unwrap()
            .eq(&[0, 1, 3, 10]));
    }

    #[test]
    fn kattis_shortestpath1() {
        assert!(
            single_source_distances(&fixture::kattis_shortestpath1!(), 0)
                .unwrap()
                .eq(&[0, 2, 4, isize::MAX])
        );
    }

    #[test]
    fn test_negative_cycle() {
        let digraph = vec![vec![(1, -2)], vec![(2, -1)], vec![(0, -1)]];

        assert_eq!(single_source_distances(&digraph, 0), None);
    }
}

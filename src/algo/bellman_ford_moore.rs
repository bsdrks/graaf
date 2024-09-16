//! The Bellman-Ford-Moore algorithm.
//!
//! Find the shortest distances from a source vertex to all other vertices in
//! an arc-weighted digraph with negative weights.
//!
//! Runs in **O(va)** time (worst-case), where **v** is the number of vertices
//! and **a** is the number of arcs.
//!
//! # Examples
//!
//! ## Shortest distances
//!
//! The shortest path from vertex `0` to `4` is red. The dashed arcs represent
//! the other shortest distances.
//!
//! ![Bellman-Ford-Moore](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bellman_ford_moore_1-0.87.4.svg?)
//!
//! ```
//! use graaf::{
//!     bellman_ford_moore::single_source_distances,
//!     AddArcWeighted,
//!     AdjacencyListWeighted,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyListWeighted::<isize>::empty(6);
//!
//! digraph.add_arc_weighted(0, 1, 8);
//! digraph.add_arc_weighted(0, 2, 4);
//! digraph.add_arc_weighted(1, 2, -5);
//! digraph.add_arc_weighted(2, 3, -2);
//! digraph.add_arc_weighted(2, 4, 4);
//! digraph.add_arc_weighted(3, 5, -2);
//! digraph.add_arc_weighted(4, 3, 10);
//! digraph.add_arc_weighted(4, 5, 9);
//! digraph.add_arc_weighted(5, 3, 5);
//! digraph.add_arc_weighted(5, 4, -3);
//!
//! assert!(single_source_distances(&digraph, 0)
//!     .unwrap()
//!     .eq(&[0, 8, 3, 1, -4, -1]));
//! ```
//!
//! ## Negative cycle
//!
//! There is no shortest path between vertices `0` and the other vertices due
//! to the negative cycle.
//!
//! ![Bellman-Ford-Moore](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bellman_ford_moore_2-0.87.4.svg)
//!
//! ```
//! use graaf::{
//!     bellman_ford_moore::single_source_distances,
//!     AddArcWeighted,
//!     AdjacencyListWeighted,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyListWeighted::empty(3);
//!
//! digraph.add_arc_weighted(0, 1, -2);
//! digraph.add_arc_weighted(1, 2, -1);
//! digraph.add_arc_weighted(2, 0, -1);
//!
//! assert_eq!(single_source_distances(&digraph, 0), None);
//! ```
#![doc(alias = "bellman_ford")]

use crate::{
    ArcsWeighted,
    Order,
};

/// Find the shortest distances from a source vertex to all other vertices in
/// an arc-weighted digraph with negative weights.
///
/// Runs in **O(va)** time (worst-case), where **v** is the number of vertices
/// and **a** is the number of arcs.
///
/// # Arguments
///
/// * `digraph`: The digraph.
/// * `s`: The source vertex.
///
/// # Returns
///
/// The distances from the source vertex to all other vertices. Returns `None`
/// if the digraph contains a negative circuit.
///
/// # Examples
///
/// ## Shortest distances
///
/// The shortest path from vertex `0` to `4` is red. The dashed arcs represent
/// the other shortest distances.
///
/// ![Bellman-Ford-Moore](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bellman_ford_moore_1-0.87.4.svg?)
///
/// ```
/// use graaf::{
///     bellman_ford_moore::single_source_distances,
///     AddArcWeighted,
///     AdjacencyListWeighted,
///     Empty,
/// };
///
/// let mut digraph = AdjacencyListWeighted::<isize>::empty(6);
///
/// digraph.add_arc_weighted(0, 1, 8);
/// digraph.add_arc_weighted(0, 2, 4);
/// digraph.add_arc_weighted(1, 2, -5);
/// digraph.add_arc_weighted(2, 3, -2);
/// digraph.add_arc_weighted(2, 4, 4);
/// digraph.add_arc_weighted(3, 5, -2);
/// digraph.add_arc_weighted(4, 3, 10);
/// digraph.add_arc_weighted(4, 5, 9);
/// digraph.add_arc_weighted(5, 3, 5);
/// digraph.add_arc_weighted(5, 4, -3);
///
/// assert!(single_source_distances(&digraph, 0)
///     .unwrap()
///     .eq(&[0, 8, 3, 1, -4, -1]));
/// ```
///
/// ## Negative cycle
///
/// There is no shortest path between vertices `0` and the other vertices due
/// to the negative cycle.
///
/// ![Bellman-Ford-Moore](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bellman_ford_moore_2-0.87.4.svg)
///
/// ```
/// use graaf::{
///     bellman_ford_moore::single_source_distances,
///     AddArcWeighted,
///     AdjacencyListWeighted,
///     Empty,
/// };
///
/// let mut digraph = AdjacencyListWeighted::empty(3);
///
/// digraph.add_arc_weighted(0, 1, -2);
/// digraph.add_arc_weighted(1, 2, -1);
/// digraph.add_arc_weighted(2, 0, -1);
///
/// assert_eq!(single_source_distances(&digraph, 0), None);
/// ```
#[must_use]
pub fn single_source_distances<D>(digraph: &D, s: usize) -> Option<Vec<isize>>
where
    D: ArcsWeighted<isize> + Order,
{
    let order = digraph.order();
    let mut dist = vec![isize::MAX; order];

    dist[s] = 0;

    for _ in 1..order {
        for (u, v, &w) in digraph.arcs_weighted() {
            dist[v] = dist[v].min(dist[u].saturating_add(w));
        }
    }

    for (u, v, &w) in digraph.arcs_weighted() {
        if dist[v] > dist[u].saturating_add(w) {
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
            repr::adjacency_list_weighted::fixture::{
                bang_jensen_94_isize,
                bang_jensen_96_isize,
                bang_jensen_99,
                kattis_bryr_1_isize,
                kattis_bryr_2_isize,
                kattis_bryr_3_isize,
                kattis_crosscountry_isize,
                kattis_shortestpath1_isize,
                kattis_shortestpath3,
            },
            AddArcWeighted,
            AdjacencyListWeighted,
            Empty,
        },
    };

    #[test]
    fn single_source_distances_trivial() {
        assert!(single_source_distances(
            &AdjacencyListWeighted::<isize>::trivial(),
            0
        )
        .unwrap()
        .iter()
        .eq(&[0]));
    }

    #[test]
    fn single_source_distances_bang_jensen_94_weighted() {
        assert!(single_source_distances(&bang_jensen_94_isize(), 0)
            .unwrap()
            .eq(&[0, 1, 1, 2, 2, 2, 3]));
    }

    #[test]
    fn single_source_distances_bang_jensen_96() {
        assert!(single_source_distances(&bang_jensen_96_isize(), 0)
            .unwrap()
            .eq(&[0, 5, 3, 6, 4, 7]));
    }

    #[test]
    fn single_source_distances_bang_jensen_99() {
        assert!(single_source_distances(&bang_jensen_99(), 0)
            .unwrap()
            .eq(&[0, 8, 3, 1, -4, -1]));
    }

    #[test]
    fn single_source_distances_kattis_bryr_1() {
        assert!(single_source_distances(&kattis_bryr_1_isize(), 0)
            .unwrap()
            .eq(&[0, 1, 1]));
    }

    #[test]
    fn single_source_distances_kattis_bryr_2() {
        assert!(single_source_distances(&kattis_bryr_2_isize(), 0)
            .unwrap()
            .eq(&[0, 1, 2, 1, 2, 3]));
    }

    #[test]
    fn single_source_distances_kattis_bryr_3() {
        assert!(single_source_distances(&kattis_bryr_3_isize(), 0)
            .unwrap()
            .eq(&[0, 0, 1, 0, 0, 0, 1, 0, 0, 1]));
    }

    #[test]
    fn single_source_distances_kattis_crosscountry() {
        assert!(single_source_distances(&kattis_crosscountry_isize(), 0)
            .unwrap()
            .eq(&[0, 1, 3, 10]));
    }

    #[test]
    fn single_source_distances_kattis_shortestpath1() {
        assert!(single_source_distances(&kattis_shortestpath1_isize(), 0)
            .unwrap()
            .eq(&[0, 2, 4, isize::MAX]));
    }

    #[test]
    fn single_source_distances_kattis_shortestpath3() {
        assert_eq!(single_source_distances(&kattis_shortestpath3(), 0), None);
    }

    #[test]
    fn test_negative_circuit() {
        let mut digraph = AdjacencyListWeighted::<isize>::empty(3);

        digraph.add_arc_weighted(0, 1, -2);
        digraph.add_arc_weighted(1, 2, -1);
        digraph.add_arc_weighted(2, 0, -1);

        assert_eq!(single_source_distances(&digraph, 0), None);
    }
}

//! The Bellman-Ford-Moore algorithm
//!
//! The Bellman-Ford-Moore algorithm is a single-source shortest path algorithm
//! for weighted digraphs without negative cycles.
//!
//! # Examples
//!
//! ```
//! use graaf::algo::bellman_ford_moore;
//!
//! let digraph = vec![
//!     vec![(1, 1), (2, 2)],
//!     vec![(2, 1)],
//!     Vec::new(),
//!     vec![(1, -1)],
//! ];
//!
//! assert!(bellman_ford_moore(&digraph, 0).eq(&[0, 1, 2, isize::MAX]));
//! assert!(bellman_ford_moore(&digraph, 1).eq(&[isize::MAX, 0, 1, isize::MAX]));
//! assert!(bellman_ford_moore(&digraph, 2).eq(&[isize::MAX, isize::MAX, 0, isize::MAX]));
//! assert!(bellman_ford_moore(&digraph, 3).eq(&[isize::MAX, -1, 0, 0]));
//! ```

use crate::op::{
    IterAllWeightedArcs,
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
/// # Examples
///
/// ```
/// use graaf::algo::bellman_ford_moore;
///
/// let digraph = vec![
///     vec![(1, 1), (2, 2)],
///     vec![(2, 1)],
///     Vec::new(),
///     vec![(1, -1)],
/// ];
///
/// assert!(bellman_ford_moore(&digraph, 0).eq(&[0, 1, 2, isize::MAX]));
/// assert!(bellman_ford_moore(&digraph, 1).eq(&[isize::MAX, 0, 1, isize::MAX]));
/// assert!(bellman_ford_moore(&digraph, 2).eq(&[isize::MAX, isize::MAX, 0, isize::MAX]));
/// assert!(bellman_ford_moore(&digraph, 3).eq(&[isize::MAX, -1, 0, 0]));
/// ```
pub fn bellman_ford_moore<D>(digraph: &D, s: usize) -> Vec<isize>
where
    D: IterAllWeightedArcs<isize> + Order,
{
    let v = digraph.order();
    let mut dist = vec![isize::MAX; v];

    dist[s] = 0;

    for _ in 1..v {
        for (s, t, w) in digraph.iter_all_weighted_arcs() {
            if dist[s] == isize::MAX {
                continue;
            }

            dist[t] = dist[t].min(dist[s].saturating_add(*w));
        }
    }

    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doctest() {
        let digraph = vec![
            vec![(1, 1), (2, 2)],
            vec![(2, 1)],
            Vec::new(),
            vec![(1, -1)],
        ];

        assert!(bellman_ford_moore(&digraph, 0).eq(&[0, 1, 2, isize::MAX]));
        assert!(bellman_ford_moore(&digraph, 1).eq(&[isize::MAX, 0, 1, isize::MAX]));
        assert!(bellman_ford_moore(&digraph, 2).eq(&[isize::MAX, isize::MAX, 0, isize::MAX]));
        assert!(bellman_ford_moore(&digraph, 3).eq(&[isize::MAX, -1, 0, 0]));
    }

    #[test]
    fn bang_jensen_96() {
        let digraph = vec![
            vec![(1, 9), (2, 3)],
            vec![(2, 6), (3, 2)],
            vec![(1, 2), (4, 1)],
            vec![(5, 1)],
            vec![(2, 2), (3, 2), (5, 7)],
            vec![(3, 2)],
        ];

        assert!(bellman_ford_moore(&digraph, 0).eq(&[0, 5, 3, 6, 4, 7]));
    }

    #[test]
    fn bang_jensen_99() {
        let digraph = vec![
            vec![(1, 9), (2, 10)],
            vec![(0, -3), (2, 5)],
            vec![(1, -2)],
            vec![(0, 4), (2, -2)],
            vec![(2, 2), (3, -5)],
            vec![(3, 4), (4, 8)],
        ];

        assert!(bellman_ford_moore(&digraph, 5).eq(&[-4, -1, 1, 3, 8, 0]));
    }
}

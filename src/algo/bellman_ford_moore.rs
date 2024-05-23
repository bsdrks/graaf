//! The Bellman-Ford-Moore algorithm
//!
//! The Bellman-Ford-Moore algorithm is a single-source shortest path algorithm
//! for weighted digraphs without negative cycles.
//!
//! # Examples
//!
//! ```
//! use graaf::algo::bellman_ford_moore::distances;
//!
//! let digraph = vec![
//!     vec![(1, 8), (2, 4)],
//!     vec![(2, -5)],
//!     vec![(3, -2), (4, 4)],
//!     vec![(5, -2)],
//!     vec![(3, 10), (5, 9)],
//!     vec![(3, 5), (4, -3)],
//! ];
//!
//! assert!(distances(&digraph, 0).eq(&[0, 8, 3, 1, -4, -1]));
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
/// use graaf::algo::bellman_ford_moore::distances;
///
/// let digraph = vec![
///     vec![(1, 8), (2, 4)],
///     vec![(2, -5)],
///     vec![(3, -2), (4, 4)],
///     vec![(5, -2)],
///     vec![(3, 10), (5, 9)],
///     vec![(3, 5), (4, -3)],
/// ];
///
/// assert!(distances(&digraph, 0).eq(&[0, 8, 3, 1, -4, -1]));
/// ```
pub fn distances<D>(digraph: &D, s: usize) -> Vec<isize>
where
    D: IterAllWeightedArcs<isize> + Order,
{
    let v = digraph.order();
    let mut dist = vec![isize::MAX; v];

    dist[s] = 0;

    for _ in 1..v {
        for (s, t, w) in digraph.iter_all_weighted_arcs() {
            dist[t] = dist[t].min(dist[s].saturating_add(*w));
        }
    }

    dist
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::algo::fixture,
    };

    #[test]
    fn bang_jensen_96() {
        assert!(distances(&fixture::bang_jensen_96_isize(), 0).eq(&[0, 5, 3, 6, 4, 7]));
    }

    #[test]
    fn bang_jensen_99() {
        assert!(distances(&fixture::bang_jensen_99(), 0).eq(&[0, 8, 3, 1, -4, -1]));
    }
}

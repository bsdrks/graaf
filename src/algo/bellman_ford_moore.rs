//! The Bellman-Ford-Moore algorithm
//!
//! The Bellman-Ford-Moore algorithm is a single-source shortest path algorithm
//! for weighted digraphs.
//!
//! The time complexity is *O*(*ve*).
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
//! assert!(distances(&digraph, 0).unwrap().eq(&[0, 8, 3, 1, -4, -1]));
//!
//! let digraph = vec![vec![(1, -2)], vec![(2, -1)], vec![(0, -1)]];
//!
//! assert_eq!(distances(&digraph, 0), None);
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
/// assert!(distances(&digraph, 0).unwrap().eq(&[0, 8, 3, 1, -4, -1]));
///
/// let digraph = vec![vec![(1, -2)], vec![(2, -1)], vec![(0, -1)]];
///
/// assert_eq!(distances(&digraph, 0), None);
/// ```
pub fn distances<D>(digraph: &D, s: usize) -> Option<Vec<isize>>
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
        crate::algo::fixture,
    };

    #[test]
    fn bang_jensen_96() {
        assert!(distances(&fixture::bang_jensen_96_isize(), 0)
            .unwrap()
            .eq(&[0, 5, 3, 6, 4, 7]));
    }

    #[test]
    fn bang_jensen_99() {
        assert!(distances(&fixture::bang_jensen_99(), 0)
            .unwrap()
            .eq(&[0, 8, 3, 1, -4, -1]));
    }

    #[test]
    fn test_negative_cycle() {
        let digraph = vec![vec![(1, -2)], vec![(2, -1)], vec![(0, -1)]];

        assert_eq!(distances(&digraph, 0), None);
    }
}

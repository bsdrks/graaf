//! The Floyd-Warshall algorithm
//!
//! The Floyd-Warshall algorithm finds the shortest paths between all pairs of
//! vertices in a weighted digraph.
//!
//! The time complexity is *O*(*v*³).
//!
//! # Examples
//!
//! ```
//! use graaf::algo::floyd_warshall::distances;
//!
//! // 0 -> {2 (-2)}
//! // 1 -> {0 (4), 2 (3)}
//! // 2 -> {3 (2)}
//! // 3 -> {1 (-1)}
//!
//! let digraph = vec![
//!     vec![(2, -2)],
//!     vec![(0, 4), (2, 3)],
//!     vec![(3, 2)],
//!     vec![(1, -1)],
//! ];
//!
//! assert!(distances(&digraph).eq(&[
//!     vec![0, -1, -2, 0],
//!     vec![4, 0, 2, 4],
//!     vec![5, 1, 0, 2],
//!     vec![3, -1, 1, 0],
//! ]));
//! ```

use crate::op::{
    IterVertices,
    IterWeightedArcs,
    Order,
};

/// Computes the distances between all pairs of vertices in a weighted digraph.
///
/// # Arguments
///
/// * `digraph`: The weighted digraph.
///
/// # Returns
///
/// Returns the distances between all pairs of vertices.
///
/// # Examples
///
/// ```
/// use graaf::algo::floyd_warshall::distances;
///
/// // 0 -> {2 (-2)}
/// // 1 -> {0 (4), 2 (3)}
/// // 2 -> {3 (2)}
/// // 3 -> {1 (-1)}
///
/// let digraph = vec![
///     vec![(2, -2)],
///     vec![(0, 4), (2, 3)],
///     vec![(3, 2)],
///     vec![(1, -1)],
/// ];
///
/// assert!(distances(&digraph).eq(&[
///     vec![0, -1, -2, 0],
///     vec![4, 0, 2, 4],
///     vec![5, 1, 0, 2],
///     vec![3, -1, 1, 0],
/// ]));
/// ```
pub fn distances<D>(digraph: &D) -> Vec<Vec<isize>>
where
    D: IterVertices + IterWeightedArcs<isize> + Order,
{
    let v = digraph.order();
    let mut dist = vec![vec![isize::MAX; v]; v];

    for (s, t, w) in digraph.iter_weighted_arcs() {
        dist[s][t] = *w;
    }

    for (i, vec) in dist.iter_mut().enumerate().take(v) {
        vec[i] = 0;
    }

    for k in 0..v {
        for i in 0..v {
            for j in 0..v {
                if dist[i][k] == isize::MAX || dist[k][j] == isize::MAX {
                    continue;
                }

                let w = dist[i][k].saturating_add(dist[k][j]);

                if dist[i][j] > w {
                    dist[i][j] = w;
                }
            }
        }
    }

    dist
}

#[cfg(test)]
mod tests {
    use crate::{
        algo::fixture,
        gen::Empty,
    };

    use super::*;

    #[test]
    fn doctest() {
        let digraph = vec![
            vec![(2, -2)],
            vec![(0, 4), (2, 3)],
            vec![(3, 2)],
            vec![(1, -1)],
        ];

        assert!(distances(&digraph).eq(&[
            vec![0, -1, -2, 0],
            vec![4, 0, 2, 4],
            vec![5, 1, 0, 2],
            vec![3, -1, 1, 0],
        ]));
    }

    #[test]
    fn trivial() {
        assert!(distances(&Vec::<Vec<(usize, isize)>>::trivial())
            .iter()
            .eq(&[vec![0]]));
    }

    #[test]
    fn bang_jensen_96() {
        assert!(distances(&fixture::bang_jensen_96_isize())[0]
            .iter()
            .eq(&[0, 5, 3, 6, 4, 7]));
    }

    #[test]
    fn bang_jensen_99() {
        assert!(distances(&fixture::bang_jensen_99())[0]
            .iter()
            .eq(&[0, 8, 3, 1, -4, -1]));
    }
}

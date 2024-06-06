//! The Floyd-Warshall algorithm
//!
//! The Floyd[^citation]-Warshall algorithm finds the shortest paths between all
//! pairs of vertices in a weighted digraph.
//!
//! The time complexity is *O*(*v³*).
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
//!
//! [^citation]: Robert W. Floyd. 1962. Algorithm 97: Shortest path. Commun. ACM 5, 6 (June 1962), 345. <https://doi.org/10.1145/367766.368168>

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
#[doc(alias = "apsp")]
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

    for i in digraph.iter_vertices() {
        for j in digraph.iter_vertices() {
            let a = dist[j][i];

            if a == isize::MAX {
                continue;
            }

            for k in digraph.iter_vertices() {
                let b = dist[i][k];

                if b == isize::MAX {
                    continue;
                }

                let s = a + b;

                if s < dist[j][k] {
                    dist[j][k] = s;
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
    fn bang_jensen_94() {
        assert!(distances(&fixture::bang_jensen_94_weighted_isize())[0]
            .iter()
            .eq(&[0, 1, 1, 2, 2, 2, 3]));
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

    #[test]
    fn kattis_bryr_1() {
        assert!(distances(&fixture::kattis_bryr_1_isize())[0]
            .iter()
            .eq(&[0, 1, 1]));
    }

    #[test]
    fn kattis_bryr_2() {
        assert!(distances(&fixture::kattis_bryr_2_isize())[0]
            .iter()
            .eq(&[0, 1, 2, 1, 2, 3]));
    }

    #[test]
    fn kattis_bryr_3() {
        assert!(distances(&fixture::kattis_bryr_3_isize())[0]
            .iter()
            .eq(&[0, 0, 1, 0, 0, 0, 1, 0, 0, 1]));
    }

    #[test]
    fn kattis_crosscountry() {
        assert!(distances(&fixture::kattis_crosscountry_isize())[0]
            .iter()
            .eq(&[0, 1, 3, 10]));
    }

    #[test]
    fn kattis_shortestpath1() {
        assert!(distances(&fixture::kattis_shortestpath1_isize())[0]
            .iter()
            .eq(&[0, 2, 4, isize::MAX]));
    }
}

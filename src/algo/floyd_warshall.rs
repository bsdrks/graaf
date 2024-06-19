//! The Floyd-Warshall algorithm
//!
//! The Floyd[^1]-Warshall algorithm finds the shortest paths between all
//! pairs of vertices in a weighted digraph.
//!
//! The time complexity is *O*(*v³*).
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     algo::{
//!         distance_matrix::DistanceMatrix,
//!         floyd_warshall::distances,
//!     },
//!     gen::Empty,
//!     op::AddWeightedArc,
//! };
//!
//! // 0 -> {2 (-2)}
//! // 1 -> {0 (4), 2 (3)}
//! // 2 -> {3 (2)}
//! // 3 -> {1 (-1)}
//!
//! let mut digraph = Vec::<Vec<(usize, isize)>>::empty(4);
//!
//! digraph.add_weighted_arc(0, 2, -2);
//! digraph.add_weighted_arc(1, 0, 4);
//! digraph.add_weighted_arc(1, 2, 3);
//! digraph.add_weighted_arc(2, 3, 2);
//! digraph.add_weighted_arc(3, 1, -1);
//!
//! assert!(distances(&digraph).eq(DistanceMatrix::from(vec![
//!     vec![0, -1, -2, 0],
//!     vec![4, 0, 2, 4],
//!     vec![5, 1, 0, 2],
//!     vec![3, -1, 1, 0],
//! ])));
//! ```
//!
//! [^1]: Robert W. Floyd. 1962. Algorithm 97: Shortest path. Commun.
//!   ACM 5, 6 (June 1962), 345. <https://doi.org/10.1145/367766.368168>

use crate::{
    algo::distance_matrix::DistanceMatrix,
    op::{
        IterVertices,
        IterWeightedArcs,
        Order,
    },
};

/// Computes the distances between all pairs of vertices in a weighted
/// digraph.[^1]
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
/// use graaf::{
///     algo::{
///         distance_matrix::DistanceMatrix,
///         floyd_warshall::distances,
///     },
///     gen::Empty,
///     op::AddWeightedArc,
/// };
///
/// // 0 -> {2 (-2)}
/// // 1 -> {0 (4), 2 (3)}
/// // 2 -> {3 (2)}
/// // 3 -> {1 (-1)}
///
/// let mut digraph = Vec::<Vec<(usize, isize)>>::empty(4);
///
/// digraph.add_weighted_arc(0, 2, -2);
/// digraph.add_weighted_arc(1, 0, 4);
/// digraph.add_weighted_arc(1, 2, 3);
/// digraph.add_weighted_arc(2, 3, 2);
/// digraph.add_weighted_arc(3, 1, -1);
///
/// assert!(distances(&digraph).eq(DistanceMatrix::from(vec!([
///     vec![0, -1, -2, 0],
///     vec![4, 0, 2, 4],
///     vec![5, 1, 0, 2],
///     vec![3, -1, 1, 0],
/// ])));
/// ```
///
/// # Panics
///
/// Never
///
/// [^1]: Robert W. Floyd. 1962. Algorithm 97: Shortest path. Commun.
///   ACM 5, 6 (June 1962), 345. <https://doi.org/10.1145/367766.368168>
#[doc(alias = "apsp")]
pub fn distances<D>(digraph: &D) -> DistanceMatrix<isize>
where
    D: IterVertices + IterWeightedArcs<isize> + Order,
{
    let v = digraph.order();
    let mut dist = DistanceMatrix::<isize>::new(v, isize::MAX);

    for (s, t, w) in digraph.iter_weighted_arcs() {
        dist[s][t] = *w;
    }

    for i in 0..v {
        dist[i][i] = 0;
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

        let dist = distances(&digraph);

        assert_eq!(dist[0][0], 0);
        assert_eq!(dist[0][1], -1);
        assert_eq!(dist[0][2], -2);
        assert_eq!(dist[0][3], 0);
        assert_eq!(dist[1][0], 4);
        assert_eq!(dist[1][1], 0);
        assert_eq!(dist[1][2], 2);
        assert_eq!(dist[1][3], 4);
        assert_eq!(dist[2][0], 5);
        assert_eq!(dist[2][1], 1);
        assert_eq!(dist[2][2], 0);
        assert_eq!(dist[2][3], 2);
        assert_eq!(dist[3][0], 3);
        assert_eq!(dist[3][1], -1);
        assert_eq!(dist[3][2], 1);
        assert_eq!(dist[3][3], 0);
    }

    #[test]
    fn trivial() {
        let dist = distances(&Vec::<Vec<(usize, isize)>>::trivial());

        assert_eq!(dist[0][0], 0);
    }

    #[test]
    fn bang_jensen_94() {
        let dist = distances(&fixture::bang_jensen_94_weighted_isize());

        assert_eq!(dist[0][0], 0);
        assert_eq!(dist[0][1], 1);
        assert_eq!(dist[0][2], 1);
        assert_eq!(dist[0][3], 2);
        assert_eq!(dist[0][4], 2);
        assert_eq!(dist[0][5], 2);
        assert_eq!(dist[0][6], 3);
    }

    #[test]
    fn bang_jensen_96() {
        let dist = distances(&fixture::bang_jensen_96_isize());

        assert_eq!(dist[0][0], 0);
        assert_eq!(dist[0][1], 5);
        assert_eq!(dist[0][2], 3);
        assert_eq!(dist[0][3], 6);
        assert_eq!(dist[0][4], 4);
        assert_eq!(dist[0][5], 7);
    }

    #[test]
    fn bang_jensen_99() {
        let dist = distances(&fixture::bang_jensen_99());

        assert_eq!(dist[0][0], 0);
        assert_eq!(dist[0][1], 8);
        assert_eq!(dist[0][2], 3);
        assert_eq!(dist[0][3], 1);
        assert_eq!(dist[0][4], -4);
        assert_eq!(dist[0][5], -1);
    }

    #[test]
    fn kattis_bryr_1() {
        let dist = distances(&fixture::kattis_bryr_1_isize());

        assert_eq!(dist[0][0], 0);
        assert_eq!(dist[0][1], 1);
        assert_eq!(dist[0][2], 1);
    }

    #[test]
    fn kattis_bryr_2() {
        let dist = distances(&fixture::kattis_bryr_2_isize());

        assert_eq!(dist[0][0], 0);
        assert_eq!(dist[0][1], 1);
        assert_eq!(dist[0][2], 2);
        assert_eq!(dist[0][3], 1);
        assert_eq!(dist[0][4], 2);
        assert_eq!(dist[0][5], 3);
    }

    #[test]
    fn kattis_bryr_3() {
        let dist = distances(&fixture::kattis_bryr_3_isize());

        assert_eq!(dist[0][0], 0);
        assert_eq!(dist[0][1], 0);
        assert_eq!(dist[0][2], 1);
        assert_eq!(dist[0][3], 0);
        assert_eq!(dist[0][4], 0);
        assert_eq!(dist[0][5], 0);
        assert_eq!(dist[0][6], 1);
        assert_eq!(dist[0][7], 0);
        assert_eq!(dist[0][8], 0);
        assert_eq!(dist[0][9], 1);
    }

    #[test]
    fn kattis_crosscountry() {
        let dist = distances(&fixture::kattis_crosscountry_isize());

        assert_eq!(dist[0][0], 0);
        assert_eq!(dist[0][1], 1);
        assert_eq!(dist[0][2], 3);
        assert_eq!(dist[0][3], 10);
    }

    #[test]
    fn kattis_shortestpath1() {
        let dist = distances(&fixture::kattis_shortestpath1_isize());

        assert_eq!(dist[0][0], 0);
        assert_eq!(dist[0][1], 2);
        assert_eq!(dist[0][2], 4);
        assert_eq!(dist[0][3], dist.max);
    }
}

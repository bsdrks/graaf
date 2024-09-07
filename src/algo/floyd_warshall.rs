//! The Floyd-Warshall algorithm.
//!
//! The Floyd[^1]-Warshall algorithm finds the shortest paths between all
//! pairs of vertices in an arc-weighted digraph.
//!
//! Runs in **O(v³)** time, where **v** is the number of vertices.
//!
//! # Examples
//!
//! ## A digraph
//!
//! A digraph of order `7` and size `15`.
//!
//! ![The digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/distance_matrix_digraph_1-0.87.4.svg?)
//!
//! ### The distance matrix
//!
//! The corresponding [`DistanceMatrix`].
//!
//! ![The distance matrix](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/distance_matrix_matrix_1-0.87.4.svg?)
//!
//! ```
//! use graaf::{
//!     adjacency_list_weighted::Digraph,
//!     algo::{
//!         floyd_warshall::distances,
//!         DistanceMatrix,
//!     },
//!     gen::Empty,
//!     op::AddArcWeighted,
//! };
//!
//! let mut digraph = Digraph::<isize>::empty(7);
//!
//! digraph.add_arc_weighted(0, 1, 5);
//! digraph.add_arc_weighted(0, 2, 3);
//! digraph.add_arc_weighted(0, 3, 2);
//! digraph.add_arc_weighted(0, 4, 4);
//! digraph.add_arc_weighted(1, 0, 3);
//! digraph.add_arc_weighted(1, 3, 1);
//! digraph.add_arc_weighted(1, 4, 2);
//! digraph.add_arc_weighted(2, 6, 4);
//! digraph.add_arc_weighted(3, 4, 1);
//! digraph.add_arc_weighted(3, 5, 1);
//! digraph.add_arc_weighted(4, 2, 3);
//! digraph.add_arc_weighted(5, 6, 1);
//! digraph.add_arc_weighted(6, 0, 9);
//! digraph.add_arc_weighted(6, 1, 8);
//! digraph.add_arc_weighted(6, 2, 5);
//!
//! let dist = distances(&digraph);
//!
//! assert!(dist[0].eq(&[0, 5, 3, 2, 3, 3, 4]));
//! assert!(dist[1].eq(&[3, 0, 5, 1, 2, 2, 3]));
//! assert!(dist[2].eq(&[13, 12, 0, 13, 14, 14, 4]));
//! assert!(dist[3].eq(&[11, 10, 4, 0, 1, 1, 2]));
//! assert!(dist[4].eq(&[16, 15, 3, 16, 0, 17, 7]));
//! assert!(dist[5].eq(&[10, 9, 6, 10, 11, 0, 1]));
//! assert!(dist[6].eq(&[9, 8, 5, 9, 10, 10, 0]));
//! ```
//!
//! [^1]: Robert W. Floyd. 1962. Algorithm 97: Shortest path. Commun.
//!   ACM 5, 6 (June 1962), 345. <https://doi.org/10.1145/367766.368168>

use {
    super::DistanceMatrix,
    crate::op::{
        ArcsWeighted,
        Order,
        Vertices,
    },
};

/// Computes the distances between all pairs of vertices in an arc-weighted
/// digraph.[^1]
///
/// # Arguments
///
/// * `digraph`: The digraph.
///
/// # Examples
///
/// A digraph of order `7` and size `15`.
///
/// ![The digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/distance_matrix_digraph_1-0.87.4.svg?)
///
/// ### The distance matrix
///
/// The corresponding [`DistanceMatrix`].
///
/// ![The distance matrix](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/distance_matrix_matrix_1-0.87.4.svg?)
///
/// ```
/// use graaf::{
///     adjacency_list_weighted::Digraph,
///     algo::{
///         floyd_warshall::distances,
///         DistanceMatrix,
///     },
///     gen::Empty,
///     op::AddArcWeighted,
/// };
///
/// let mut digraph = Digraph::<isize>::empty(7);
///
/// digraph.add_arc_weighted(0, 1, 5);
/// digraph.add_arc_weighted(0, 2, 3);
/// digraph.add_arc_weighted(0, 3, 2);
/// digraph.add_arc_weighted(0, 4, 4);
/// digraph.add_arc_weighted(1, 0, 3);
/// digraph.add_arc_weighted(1, 3, 1);
/// digraph.add_arc_weighted(1, 4, 2);
/// digraph.add_arc_weighted(2, 6, 4);
/// digraph.add_arc_weighted(3, 4, 1);
/// digraph.add_arc_weighted(3, 5, 1);
/// digraph.add_arc_weighted(4, 2, 3);
/// digraph.add_arc_weighted(5, 6, 1);
/// digraph.add_arc_weighted(6, 0, 9);
/// digraph.add_arc_weighted(6, 1, 8);
/// digraph.add_arc_weighted(6, 2, 5);
///
/// let dist = distances(&digraph);
///
/// assert!(dist[0].eq(&[0, 5, 3, 2, 3, 3, 4]));
/// assert!(dist[1].eq(&[3, 0, 5, 1, 2, 2, 3]));
/// assert!(dist[2].eq(&[13, 12, 0, 13, 14, 14, 4]));
/// assert!(dist[3].eq(&[11, 10, 4, 0, 1, 1, 2]));
/// assert!(dist[4].eq(&[16, 15, 3, 16, 0, 17, 7]));
/// assert!(dist[5].eq(&[10, 9, 6, 10, 11, 0, 1]));
/// assert!(dist[6].eq(&[9, 8, 5, 9, 10, 10, 0]));
/// ```
///
/// [^1]: Robert W. Floyd. 1962. Algorithm 97: Shortest path. Commun.
///   ACM 5, 6 (June 1962), 345. <https://doi.org/10.1145/367766.368168>
#[doc(alias = "apsp")]
#[must_use]
pub fn distances<D>(digraph: &D) -> DistanceMatrix<isize>
where
    D: ArcsWeighted<isize> + Order + Vertices,
{
    let order = digraph.order();
    let mut dist = DistanceMatrix::<isize>::new(order, isize::MAX);

    for (u, v, &w) in digraph.arcs_weighted() {
        dist[u][v] = w;
    }

    for i in 0..order {
        dist[i][i] = 0;
    }

    for i in digraph.vertices() {
        for j in digraph.vertices() {
            let a = dist[j][i];

            if a == isize::MAX {
                continue;
            }

            for k in digraph.vertices() {
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
    use {
        super::*,
        crate::{
            adjacency_list_weighted::{
                fixture::{
                    bang_jensen_94_isize,
                    bang_jensen_96_isize,
                    bang_jensen_99,
                    kattis_bryr_1_isize,
                    kattis_bryr_2_isize,
                    kattis_bryr_3_isize,
                    kattis_crosscountry_isize,
                    kattis_shortestpath1_isize,
                },
                Digraph,
            },
            gen::Empty,
            op::AddArcWeighted,
        },
    };

    #[test]
    fn distances_doctest() {
        let mut digraph = Digraph::empty(4);

        digraph.add_arc_weighted(0, 2, -2);
        digraph.add_arc_weighted(1, 0, 4);
        digraph.add_arc_weighted(1, 2, 3);
        digraph.add_arc_weighted(2, 3, 2);
        digraph.add_arc_weighted(3, 1, -1);

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
    fn distances_trivial() {
        let dist = distances(&Digraph::<isize>::trivial());

        assert_eq!(dist[0][0], 0);
    }

    #[test]
    fn distances_bang_jensen_94_weighted() {
        let dist = distances(&bang_jensen_94_isize());

        assert_eq!(dist[0][0], 0);
        assert_eq!(dist[0][1], 1);
        assert_eq!(dist[0][2], 1);
        assert_eq!(dist[0][3], 2);
        assert_eq!(dist[0][4], 2);
        assert_eq!(dist[0][5], 2);
        assert_eq!(dist[0][6], 3);
    }

    #[test]
    fn distances_bang_jensen_96() {
        let dist = distances(&bang_jensen_96_isize());

        assert_eq!(dist[0][0], 0);
        assert_eq!(dist[0][1], 5);
        assert_eq!(dist[0][2], 3);
        assert_eq!(dist[0][3], 6);
        assert_eq!(dist[0][4], 4);
        assert_eq!(dist[0][5], 7);
    }

    #[test]
    fn distances_bang_jensen_99() {
        let dist = distances(&bang_jensen_99());

        assert_eq!(dist[0][0], 0);
        assert_eq!(dist[0][1], 8);
        assert_eq!(dist[0][2], 3);
        assert_eq!(dist[0][3], 1);
        assert_eq!(dist[0][4], -4);
        assert_eq!(dist[0][5], -1);
    }

    #[test]
    fn distances_kattis_bryr_1() {
        let dist = distances(&kattis_bryr_1_isize());

        assert_eq!(dist[0][0], 0);
        assert_eq!(dist[0][1], 1);
        assert_eq!(dist[0][2], 1);
    }

    #[test]
    fn distances_kattis_bryr_2() {
        let dist = distances(&kattis_bryr_2_isize());

        assert_eq!(dist[0][0], 0);
        assert_eq!(dist[0][1], 1);
        assert_eq!(dist[0][2], 2);
        assert_eq!(dist[0][3], 1);
        assert_eq!(dist[0][4], 2);
        assert_eq!(dist[0][5], 3);
    }

    #[test]
    fn distances_kattis_bryr_3() {
        let dist = distances(&kattis_bryr_3_isize());

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
    fn distances_kattis_crosscountry() {
        let dist = distances(&kattis_crosscountry_isize());

        assert_eq!(dist[0][0], 0);
        assert_eq!(dist[0][1], 1);
        assert_eq!(dist[0][2], 3);
        assert_eq!(dist[0][3], 10);
    }

    #[test]
    fn distances_kattis_shortestpath1() {
        let dist = distances(&kattis_shortestpath1_isize());

        assert_eq!(dist[0][0], 0);
        assert_eq!(dist[0][1], 2);
        assert_eq!(dist[0][2], 4);
        assert_eq!(dist[0][3], dist.infinity);
    }
}

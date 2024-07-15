//! Distance matrices.
//!
//! A distance matrix contains the distance between each pair of vertices in a
//! digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list_weighted::Digraph,
//!     algo::{
//!         distance_matrix::DistanceMatrix,
//!         floyd_warshall::distances,
//!     },
//!     gen::Empty,
//!     op::{
//!         AddArcWeighted,
//!         Arcs,
//!     },
//! };
//!
//! // 0 -> {1 (1), 2 (3), 3 (14)}
//! // 1 -> {0 (2), 2 (4), 3 (22)}
//! // 2 -> {0 (3), 1 (10), 3 (7)}
//! // 3 -> {0 (13), 1 (8), 2 (2)}
//!
//! let mut digraph = Digraph::<isize>::empty(4);
//!
//! digraph.add_arc_weighted(0, 1, 1);
//! digraph.add_arc_weighted(0, 2, 3);
//! digraph.add_arc_weighted(0, 3, 14);
//! digraph.add_arc_weighted(1, 0, 2);
//! digraph.add_arc_weighted(1, 2, 4);
//! digraph.add_arc_weighted(1, 3, 22);
//! digraph.add_arc_weighted(2, 0, 3);
//! digraph.add_arc_weighted(2, 1, 10);
//! digraph.add_arc_weighted(2, 3, 7);
//! digraph.add_arc_weighted(3, 0, 13);
//! digraph.add_arc_weighted(3, 1, 8);
//! digraph.add_arc_weighted(3, 2, 2);
//!
//! let dist = distances(&digraph);
//!
//! assert!(dist[0].iter().eq(&[0, 1, 3, 10]));
//! assert!(dist[1].iter().eq(&[2, 0, 4, 11]));
//! assert!(dist[2].iter().eq(&[3, 4, 0, 7]));
//! assert!(dist[3].iter().eq(&[5, 6, 2, 0]));
//! ```

use std::{
    cmp::Ordering::{
        Equal,
        Greater,
        Less,
    },
    ops::{
        Index,
        IndexMut,
    },
};

/// A distance matrix.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DistanceMatrix<W> {
    dist: Vec<Vec<W>>,
    /// The distance between unconnected vertices.
    pub infinity: W,
}

impl<W> DistanceMatrix<W> {
    /// Creates a distance matrix from a vector of vectors.
    ///
    /// # Arguments
    ///
    /// * `order`: The number of vertices.
    /// * `infinity`: The distance between unconnected vertices.
    ///
    /// # Panics
    ///
    /// Panics if `order` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::algo::distance_matrix::DistanceMatrix;
    ///
    /// let dist = DistanceMatrix::new(4, 0);
    ///
    /// assert_eq!(dist.infinity, 0);
    /// assert_eq!(dist[0], vec![0; 4]);
    /// assert_eq!(dist[1], vec![0; 4]);
    /// assert_eq!(dist[2], vec![0; 4]);
    /// assert_eq!(dist[3], vec![0; 4]);
    /// ```
    #[must_use]
    pub fn new(order: usize, infinity: W) -> Self
    where
        W: Copy,
    {
        assert!(order > 0, "a distance matrix must have at least one vertex");

        Self {
            dist: vec![vec![infinity; order]; order],
            infinity,
        }
    }

    /// Finds the center of a distance matrix.
    ///
    /// The center of a digraph is the set of vertices with the smallest
    /// eccentricity. The center is also known as the Jordan center.
    ///
    /// # Returns
    ///
    /// Returns the vertices with the smallest eccentricity.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list_weighted::Digraph,
    ///     algo::{
    ///         distance_matrix::DistanceMatrix,
    ///         floyd_warshall::distances,
    ///     },
    ///     gen::Empty,
    ///     op::AddArcWeighted,
    /// };
    ///
    /// // 0 -> {1 (1), 2 (3), 3 (14)}
    /// // 1 -> {0 (2), 2 (4), 3 (22)}
    /// // 2 -> {0 (3), 1 (10), 3 (7)}
    /// // 3 -> {0 (13), 1 (8), 2 (2)}
    ///
    /// let mut digraph = Digraph::<isize>::empty(4);
    ///
    /// digraph.add_arc_weighted(0, 1, 1);
    /// digraph.add_arc_weighted(0, 2, 3);
    /// digraph.add_arc_weighted(0, 3, 14);
    /// digraph.add_arc_weighted(1, 0, 2);
    /// digraph.add_arc_weighted(1, 2, 4);
    /// digraph.add_arc_weighted(1, 3, 22);
    /// digraph.add_arc_weighted(2, 0, 3);
    /// digraph.add_arc_weighted(2, 1, 10);
    /// digraph.add_arc_weighted(2, 3, 7);
    /// digraph.add_arc_weighted(3, 0, 13);
    /// digraph.add_arc_weighted(3, 1, 8);
    /// digraph.add_arc_weighted(3, 2, 2);
    ///
    /// let dist = distances(&digraph);
    ///
    /// assert!(dist.center().iter().eq(&[3]));
    /// ```
    #[doc(alias = "centre")]
    #[doc(alias = "jordan_center")]
    #[doc(alias = "jordan_centre")]
    #[must_use]
    pub fn center(&self) -> Vec<usize>
    where
        W: Copy + Ord,
    {
        let ecc = self.eccentricities();
        let mut center = Vec::new();
        let mut min = self.infinity;

        for (i, &e) in ecc.iter().enumerate() {
            match e.cmp(&min) {
                Less => {
                    center.clear();
                    center.push(i);
                    min = e;
                }
                Equal => center.push(i),
                Greater => (),
            }
        }

        center
    }

    /// Returns the diameter of the digraph.
    ///
    /// The diameter of a digraph is the maximum eccentricity of any vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list_weighted::Digraph,
    ///     algo::{
    ///         distance_matrix::DistanceMatrix,
    ///         floyd_warshall::distances,
    ///     },
    ///     gen::Empty,
    ///     op::AddArcWeighted,
    /// };
    ///
    /// // 0 -> {1 (1), 2 (3), 3 (14)}
    /// // 1 -> {0 (2), 2 (4), 3 (22)}
    /// // 2 -> {0 (3), 1 (10), 3 (7)}
    /// // 3 -> {0 (13), 1 (8), 2 (2)}
    ///
    /// let mut digraph = Digraph::<isize>::empty(4);
    ///
    /// digraph.add_arc_weighted(0, 1, 1);
    /// digraph.add_arc_weighted(0, 2, 3);
    /// digraph.add_arc_weighted(0, 3, 14);
    /// digraph.add_arc_weighted(1, 0, 2);
    /// digraph.add_arc_weighted(1, 2, 4);
    /// digraph.add_arc_weighted(1, 3, 22);
    /// digraph.add_arc_weighted(2, 0, 3);
    /// digraph.add_arc_weighted(2, 1, 10);
    /// digraph.add_arc_weighted(2, 3, 7);
    /// digraph.add_arc_weighted(3, 0, 13);
    /// digraph.add_arc_weighted(3, 1, 8);
    /// digraph.add_arc_weighted(3, 2, 2);
    ///
    /// let dist = distances(&digraph);
    ///
    /// assert_eq!(dist.diameter(), 11);
    /// ```
    #[must_use]
    pub fn diameter(&self) -> W
    where
        W: Copy + Ord,
    {
        self.eccentricities()
            .iter()
            .copied()
            .max()
            .unwrap_or(self.infinity)
    }

    /// Returns the eccentricities of the vertices.
    ///
    /// The eccentricity of a vertex is the maximum distance to any other
    /// vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list_weighted::Digraph,
    ///     algo::{
    ///         distance_matrix::DistanceMatrix,
    ///         floyd_warshall::distances,
    ///     },
    ///     gen::Empty,
    ///     op::AddArcWeighted,
    /// };
    ///
    /// // 0 -> {1 (1), 2 (3), 3 (14)}
    /// // 1 -> {0 (2), 2 (4), 3 (22)}
    /// // 2 -> {0 (3), 1 (10), 3 (7)}
    /// // 3 -> {0 (13), 1 (8), 2 (2)}
    ///
    /// let mut digraph = Digraph::<isize>::empty(4);
    ///
    /// digraph.add_arc_weighted(0, 1, 1);
    /// digraph.add_arc_weighted(0, 2, 3);
    /// digraph.add_arc_weighted(0, 3, 14);
    /// digraph.add_arc_weighted(1, 0, 2);
    /// digraph.add_arc_weighted(1, 2, 4);
    /// digraph.add_arc_weighted(1, 3, 22);
    /// digraph.add_arc_weighted(2, 0, 3);
    /// digraph.add_arc_weighted(2, 1, 10);
    /// digraph.add_arc_weighted(2, 3, 7);
    /// digraph.add_arc_weighted(3, 0, 13);
    /// digraph.add_arc_weighted(3, 1, 8);
    /// digraph.add_arc_weighted(3, 2, 2);
    ///
    /// let dist = distances(&digraph);
    ///
    /// assert!(dist.eccentricities().iter().eq(&[10, 11, 7, 6]));
    /// ```
    #[must_use]
    pub fn eccentricities(&self) -> Vec<W>
    where
        W: Copy + Ord,
    {
        self.dist
            .iter()
            .map(|row| {
                row.iter()
                    .reduce(|acc, x| acc.max(x))
                    .unwrap_or(&self.infinity)
            })
            .copied()
            .collect()
    }

    /// Returns whether the distance matrix is connected.
    ///
    /// A distance matrix is connected if the eccentricity of every vertex is
    /// finite.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list_weighted::Digraph,
    ///     algo::{
    ///         distance_matrix::DistanceMatrix,
    ///         floyd_warshall::distances,
    ///     },
    ///     gen::Empty,
    ///     op::AddArcWeighted,
    /// };
    ///
    /// // 0 -> {1 (1), 2 (1)}
    /// // 1 -> {0 (1), 2 (1)}
    /// // 2 -> {0 (1), 1 (1)}
    ///
    /// let mut digraph = Digraph::<isize>::empty(3);
    ///
    /// digraph.add_arc_weighted(0, 1, 1);
    /// digraph.add_arc_weighted(0, 2, 1);
    /// digraph.add_arc_weighted(1, 0, 1);
    /// digraph.add_arc_weighted(1, 2, 1);
    /// digraph.add_arc_weighted(2, 0, 1);
    /// digraph.add_arc_weighted(2, 1, 1);
    ///
    /// let dist = distances(&digraph);
    ///
    /// assert!(dist.is_connected());
    ///
    /// // 0 -> {1 (1), 2 (1)}
    /// // 1 -> {0 (1), 2 (1)}
    /// // 2 -> {0 (1), 1 (1)}
    /// // 3 -> {}
    ///
    /// let mut digraph = Digraph::<isize>::empty(4);
    ///
    /// digraph.add_arc_weighted(0, 1, 1);
    /// digraph.add_arc_weighted(0, 2, 1);
    /// digraph.add_arc_weighted(1, 0, 1);
    /// digraph.add_arc_weighted(1, 2, 1);
    /// digraph.add_arc_weighted(2, 0, 1);
    /// digraph.add_arc_weighted(2, 1, 1);
    ///
    /// let dist = distances(&digraph);
    ///
    /// assert!(!dist.is_connected());
    /// ```
    #[must_use]
    pub fn is_connected(&self) -> bool
    where
        W: Copy + Ord,
    {
        self.eccentricities().iter().all(|&e| e != self.infinity)
    }

    /// Returns the periphery of the digraph.
    ///
    /// The periphery of a digraph is the set of vertices with an eccentricity
    /// equal to the diameter.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list_weighted::Digraph,
    ///     algo::{
    ///         distance_matrix::DistanceMatrix,
    ///         floyd_warshall::distances,
    ///     },
    ///     gen::Empty,
    ///     op::AddArcWeighted,
    /// };
    ///
    /// // 0 -> {1 (1), 2 (3), 3 (14)}
    /// // 1 -> {0 (2), 2 (4), 3 (22)}
    /// // 2 -> {0 (3), 1 (10), 3 (7)}
    /// // 3 -> {0 (13), 1 (8), 2 (2)}
    ///
    /// let mut digraph = Digraph::<isize>::empty(4);
    ///
    /// digraph.add_arc_weighted(0, 1, 1);
    /// digraph.add_arc_weighted(0, 2, 3);
    /// digraph.add_arc_weighted(0, 3, 14);
    /// digraph.add_arc_weighted(1, 0, 2);
    /// digraph.add_arc_weighted(1, 2, 4);
    /// digraph.add_arc_weighted(1, 3, 22);
    /// digraph.add_arc_weighted(2, 0, 3);
    /// digraph.add_arc_weighted(2, 1, 10);
    /// digraph.add_arc_weighted(2, 3, 7);
    /// digraph.add_arc_weighted(3, 0, 13);
    /// digraph.add_arc_weighted(3, 1, 8);
    /// digraph.add_arc_weighted(3, 2, 2);
    ///
    /// let dist = distances(&digraph);
    ///
    /// assert!(dist.periphery().iter().eq(&[1]));
    /// ```
    #[must_use]
    pub fn periphery(&self) -> Vec<usize>
    where
        W: Copy + Ord,
    {
        let ecc = self.eccentricities();
        let diameter = ecc.iter().max().unwrap_or(&self.infinity);

        ecc.iter()
            .enumerate()
            .filter_map(|(i, &e)| (e == *diameter).then_some(i))
            .collect()
    }
}

impl<W> Index<usize> for DistanceMatrix<W> {
    type Output = Vec<W>;

    fn index(&self, u: usize) -> &Self::Output {
        &self.dist[u]
    }
}

impl<W> IndexMut<usize> for DistanceMatrix<W> {
    fn index_mut(&mut self, u: usize) -> &mut Self::Output {
        &mut self.dist[u]
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            adjacency_list_weighted::{
                fixture::{
                    kattis_bryr_1_isize,
                    kattis_bryr_2_isize,
                    kattis_bryr_3_isize,
                    kattis_crosscountry_isize,
                },
                Digraph,
            },
            algo::floyd_warshall::distances,
            gen::Empty,
        },
    };

    #[test]
    fn center_kattis_bryr_1() {
        assert!(distances(&kattis_bryr_1_isize())
            .center()
            .iter()
            .eq(&[0, 1, 2]));
    }

    #[test]
    fn center_kattis_bryr_2() {
        assert!(distances(&kattis_bryr_2_isize()).center().iter().eq(&[3]));
    }

    #[test]
    fn center_kattis_bryr_3() {
        assert!(distances(&kattis_bryr_3_isize())
            .center()
            .iter()
            .eq(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
    }

    #[test]
    fn center_kattis_crosscountry() {
        assert!(distances(&kattis_crosscountry_isize())
            .center()
            .iter()
            .eq(&[3]));
    }

    #[test]
    fn center_trivial() {
        assert!(distances(&Digraph::<isize>::trivial())
            .center()
            .iter()
            .eq(&[0]));
    }

    #[test]
    fn diameter_kattis_bryr_1() {
        assert_eq!(distances(&kattis_bryr_1_isize()).diameter(), 1);
    }

    #[test]
    fn diameter_kattis_bryr_2() {
        assert_eq!(distances(&kattis_bryr_2_isize()).diameter(), 4);
    }

    #[test]
    fn diameter_kattis_bryr_3() {
        assert_eq!(distances(&kattis_bryr_3_isize()).diameter(), 1);
    }

    #[test]
    fn diameter_kattis_crosscountry() {
        assert_eq!(distances(&kattis_crosscountry_isize()).diameter(), 11);
    }

    #[test]
    fn diameter_trivial() {
        assert_eq!(distances(&Digraph::<isize>::trivial()).diameter(), 0);
    }

    #[test]
    fn eccentricities_kattis_bryr_1() {
        assert!(distances(&kattis_bryr_1_isize())
            .eccentricities()
            .iter()
            .eq(&[1, 1, 1]));
    }

    #[test]
    fn eccentricities_kattis_bryr_2() {
        assert!(distances(&kattis_bryr_2_isize())
            .eccentricities()
            .iter()
            .eq(&[3, 4, 3, 2, 3, 4]));
    }

    #[test]
    fn eccentricities_kattis_bryr_3() {
        assert!(distances(&kattis_bryr_3_isize())
            .eccentricities()
            .iter()
            .eq(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]));
    }

    #[test]
    fn eccentricities_kattis_crosscountry() {
        assert!(distances(&kattis_crosscountry_isize())
            .eccentricities()
            .iter()
            .eq(&[10, 11, 7, 6]));
    }

    #[test]
    fn eccentricities_trivial() {
        assert!(distances(&Digraph::<isize>::trivial())
            .eccentricities()
            .iter()
            .eq(&[0]));
    }

    #[test]
    fn is_connected_kattis_bryr_1() {
        assert!(distances(&kattis_bryr_1_isize()).is_connected());
    }

    #[test]
    fn is_connected_kattis_bryr_2() {
        assert!(distances(&kattis_bryr_2_isize()).is_connected());
    }

    #[test]
    fn is_connected_kattis_bryr_3() {
        assert!(distances(&kattis_bryr_3_isize()).is_connected());
    }

    #[test]
    fn is_connected_kattis_crosscountry() {
        assert!(distances(&kattis_crosscountry_isize()).is_connected());
    }

    #[test]
    fn is_connected_trivial() {
        assert!(distances(&Digraph::<isize>::trivial()).is_connected());
    }

    #[test]
    fn index() {
        let dist = DistanceMatrix::new(4, isize::MAX);

        assert_eq!(dist[0][0], isize::MAX);
        assert_eq!(dist[0][1], isize::MAX);
        assert_eq!(dist[0][2], isize::MAX);
        assert_eq!(dist[0][3], isize::MAX);
        assert_eq!(dist[1][0], isize::MAX);
        assert_eq!(dist[1][1], isize::MAX);
        assert_eq!(dist[1][2], isize::MAX);
        assert_eq!(dist[1][3], isize::MAX);
        assert_eq!(dist[2][0], isize::MAX);
        assert_eq!(dist[2][1], isize::MAX);
        assert_eq!(dist[2][2], isize::MAX);
        assert_eq!(dist[2][3], isize::MAX);
        assert_eq!(dist[3][0], isize::MAX);
        assert_eq!(dist[3][1], isize::MAX);
        assert_eq!(dist[3][2], isize::MAX);
        assert_eq!(dist[3][3], isize::MAX);
    }

    #[test]
    fn index_mut() {
        let mut dist = DistanceMatrix::new(4, isize::MAX);

        dist[0][0] = 1;
        dist[0][1] = 2;
        dist[0][2] = 3;
        dist[0][3] = 4;
        dist[1][0] = 5;
        dist[1][1] = 6;
        dist[1][2] = 7;
        dist[1][3] = 8;
        dist[2][0] = 9;
        dist[2][1] = 10;
        dist[2][2] = 11;
        dist[2][3] = 12;
        dist[3][0] = 13;
        dist[3][1] = 14;
        dist[3][2] = 15;
        dist[3][3] = 16;

        assert!(dist[0].iter().eq(&[1, 2, 3, 4]));
        assert!(dist[1].iter().eq(&[5, 6, 7, 8]));
        assert!(dist[2].iter().eq(&[9, 10, 11, 12]));
        assert!(dist[3].iter().eq(&[13, 14, 15, 16]));
    }

    #[test]
    fn new() {
        let dist = DistanceMatrix::new(4, isize::MAX);

        assert_eq!(dist.infinity, isize::MAX);
        assert!(dist[0].iter().eq(&[isize::MAX; 4]));
        assert!(dist[1].iter().eq(&[isize::MAX; 4]));
        assert!(dist[2].iter().eq(&[isize::MAX; 4]));
        assert!(dist[3].iter().eq(&[isize::MAX; 4]));
    }

    #[test]
    #[should_panic(expected = "a distance matrix must have at least one vertex")]
    fn new_0() {
        let _ = DistanceMatrix::new(0, isize::MAX);
    }

    #[test]
    fn periphery_kattis_bryr_1() {
        assert!(distances(&kattis_bryr_1_isize())
            .periphery()
            .iter()
            .eq(&[0, 1, 2]));
    }

    #[test]
    fn periphery_kattis_bryr_2() {
        assert!(distances(&kattis_bryr_2_isize())
            .periphery()
            .iter()
            .eq(&[1, 5]));
    }

    #[test]
    fn periphery_kattis_bryr_3() {
        assert!(distances(&kattis_bryr_3_isize())
            .periphery()
            .iter()
            .eq(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
    }

    #[test]
    fn periphery_kattis_crosscountry() {
        assert!(distances(&kattis_crosscountry_isize())
            .periphery()
            .iter()
            .eq(&[1]));
    }

    #[test]
    fn periphery_trivial() {
        assert!(distances(&Digraph::<isize>::trivial())
            .periphery()
            .iter()
            .eq(&[0]));
    }
}

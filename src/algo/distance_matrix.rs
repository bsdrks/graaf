//! A distance matrix
//!
//! A distance matrix contains the distance between each pair of vertices in a
//! digraph.
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
//! let dist = distances(&digraph);
//!
//! assert!(dist[0].iter().eq(&[0, -1, -2, 0]));
//! assert!(dist[1].iter().eq(&[4, 0, 2, 4]));
//! assert!(dist[2].iter().eq(&[5, 1, 0, 2]));
//! assert!(dist[3].iter().eq(&[3, -1, 1, 0]));
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
    /// The distance between each pair of vertices.
    dist: Vec<Vec<W>>,
    /// The maximum distance between two vertices.
    pub max: W,
}

impl<W> DistanceMatrix<W> {
    /// Creates a distance matrix from a vector of vectors.
    ///
    /// # Arguments
    ///
    /// * `v`: The number of vertices.
    /// * `max`: The maximum distance between two vertices.
    ///
    /// # Panics
    ///
    /// Panics if `v` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::algo::distance_matrix::DistanceMatrix;
    ///
    /// let dist = DistanceMatrix::new(4, 0);
    ///
    /// assert_eq!(dist.max, 0);
    /// assert_eq!(dist[0], vec![0; 4]);
    /// assert_eq!(dist[1], vec![0; 4]);
    /// assert_eq!(dist[2], vec![0; 4]);
    /// assert_eq!(dist[3], vec![0; 4]);
    /// ```
    pub fn new(v: usize, max: W) -> Self
    where
        W: Copy,
    {
        assert!(v > 0, "a distance matrix must have at least one vertex");

        Self {
            dist: vec![vec![max; v]; v],
            max,
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
    ///     algo::{
    ///         distance_matrix::DistanceMatrix,
    ///         floyd_warshall::distances,
    ///     },
    ///     gen::Empty,
    ///     op::AddWeightedArc,
    /// };
    ///
    /// // 0 -> {2}
    /// // 1 -> {3}
    /// // 2 -> {0, 3, 4}
    /// // 3 -> {1, 2, 5}
    /// // 4 -> {2, 5, 6}
    /// // 5 -> {3, 4, 7}
    /// // 6 -> {5, 8}
    /// // 7 -> {5}
    /// // 8 -> {6}
    ///
    /// let mut digraph = Vec::<Vec<(usize, isize)>>::empty(9);
    ///
    /// digraph.add_weighted_arc(0, 2, 1);
    /// digraph.add_weighted_arc(1, 3, 1);
    /// digraph.add_weighted_arc(2, 0, 1);
    /// digraph.add_weighted_arc(2, 3, 1);
    /// digraph.add_weighted_arc(2, 4, 1);
    /// digraph.add_weighted_arc(3, 1, 1);
    /// digraph.add_weighted_arc(3, 2, 1);
    /// digraph.add_weighted_arc(3, 5, 1);
    /// digraph.add_weighted_arc(4, 2, 1);
    /// digraph.add_weighted_arc(4, 5, 1);
    /// digraph.add_weighted_arc(4, 6, 1);
    /// digraph.add_weighted_arc(5, 3, 1);
    /// digraph.add_weighted_arc(5, 4, 1);
    /// digraph.add_weighted_arc(5, 7, 1);
    /// digraph.add_weighted_arc(6, 5, 1);
    /// digraph.add_weighted_arc(6, 8, 1);
    /// digraph.add_weighted_arc(7, 5, 1);
    /// digraph.add_weighted_arc(8, 6, 1);
    ///
    /// let dist = distances(&digraph);
    ///
    /// assert!(dist.center().iter().eq(&[2, 4, 5]));
    /// ```
    #[doc(alias = "jordan_center")]
    pub fn center(&self) -> Vec<usize>
    where
        W: Copy + Ord,
    {
        let ecc = self.eccentricities();
        let mut center = Vec::new();
        let mut min = self.max;

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
    ///     algo::{
    ///         distance_matrix::DistanceMatrix,
    ///         floyd_warshall::distances,
    ///     },
    ///     gen::Empty,
    ///     op::AddWeightedArc,
    /// };
    ///
    /// // 0 -> {2}
    /// // 1 -> {3}
    /// // 2 -> {0, 3, 4}
    /// // 3 -> {1, 2, 5}
    /// // 4 -> {2, 5, 6}
    /// // 5 -> {3, 4, 7}
    /// // 6 -> {5, 8}
    /// // 7 -> {5}
    /// // 8 -> {6}
    ///
    /// let mut digraph = Vec::<Vec<(usize, isize)>>::empty(9);
    ///
    /// digraph.add_weighted_arc(0, 2, 1);
    /// digraph.add_weighted_arc(1, 3, 1);
    /// digraph.add_weighted_arc(2, 0, 1);
    /// digraph.add_weighted_arc(2, 3, 1);
    /// digraph.add_weighted_arc(2, 4, 1);
    /// digraph.add_weighted_arc(3, 1, 1);
    /// digraph.add_weighted_arc(3, 2, 1);
    /// digraph.add_weighted_arc(3, 5, 1);
    /// digraph.add_weighted_arc(4, 2, 1);
    /// digraph.add_weighted_arc(4, 5, 1);
    /// digraph.add_weighted_arc(4, 6, 1);
    /// digraph.add_weighted_arc(5, 3, 1);
    /// digraph.add_weighted_arc(5, 4, 1);
    /// digraph.add_weighted_arc(5, 7, 1);
    /// digraph.add_weighted_arc(6, 5, 1);
    /// digraph.add_weighted_arc(6, 8, 1);
    /// digraph.add_weighted_arc(7, 5, 1);
    /// digraph.add_weighted_arc(8, 6, 1);
    ///
    /// let dist = distances(&digraph);
    ///
    /// assert_eq!(dist.diameter(), 5);
    /// ```
    pub fn diameter(&self) -> W
    where
        W: Copy + Ord,
    {
        self.eccentricities()
            .iter()
            .copied()
            .max()
            .unwrap_or(self.max)
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
    ///     algo::{
    ///         distance_matrix::DistanceMatrix,
    ///         floyd_warshall::distances,
    ///     },
    ///     gen::Empty,
    ///     op::AddWeightedArc,
    /// };
    ///
    /// // 0 -> {2}
    /// // 1 -> {3}
    /// // 2 -> {0, 3, 4}
    /// // 3 -> {1, 2, 5}
    /// // 4 -> {2, 5, 6}
    /// // 5 -> {3, 4, 7}
    /// // 6 -> {5, 8}
    /// // 7 -> {5}
    /// // 8 -> {6}
    ///
    /// let mut digraph = Vec::<Vec<(usize, isize)>>::empty(9);
    ///
    /// digraph.add_weighted_arc(0, 2, 1);
    /// digraph.add_weighted_arc(1, 3, 1);
    /// digraph.add_weighted_arc(2, 0, 1);
    /// digraph.add_weighted_arc(2, 3, 1);
    /// digraph.add_weighted_arc(2, 4, 1);
    /// digraph.add_weighted_arc(3, 1, 1);
    /// digraph.add_weighted_arc(3, 2, 1);
    /// digraph.add_weighted_arc(3, 5, 1);
    /// digraph.add_weighted_arc(4, 2, 1);
    /// digraph.add_weighted_arc(4, 5, 1);
    /// digraph.add_weighted_arc(4, 6, 1);
    /// digraph.add_weighted_arc(5, 3, 1);
    /// digraph.add_weighted_arc(5, 4, 1);
    /// digraph.add_weighted_arc(5, 7, 1);
    /// digraph.add_weighted_arc(6, 5, 1);
    /// digraph.add_weighted_arc(6, 8, 1);
    /// digraph.add_weighted_arc(7, 5, 1);
    /// digraph.add_weighted_arc(8, 6, 1);
    ///
    /// let dist = distances(&digraph);
    ///
    /// assert!(dist
    ///     .eccentricities()
    ///     .iter()
    ///     .eq(&[4, 5, 3, 4, 3, 3, 4, 4, 5]));
    /// ```
    pub fn eccentricities(&self) -> Vec<W>
    where
        W: Copy + Ord,
    {
        self.dist
            .iter()
            .map(|row| row.iter().reduce(|acc, x| acc.max(x)).unwrap_or(&self.max))
            .copied()
            .collect()
    }
}

impl<W> Index<usize> for DistanceMatrix<W> {
    type Output = Vec<W>;

    fn index(&self, s: usize) -> &Self::Output {
        &self.dist[s]
    }
}

impl<W> IndexMut<usize> for DistanceMatrix<W> {
    fn index_mut(&mut self, s: usize) -> &mut Self::Output {
        &mut self.dist[s]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let dist = DistanceMatrix::new(4, isize::MAX);

        assert_eq!(dist.max, isize::MAX);
        assert!(dist[0].iter().eq(&[isize::MAX; 4]));
        assert!(dist[1].iter().eq(&[isize::MAX; 4]));
        assert!(dist[2].iter().eq(&[isize::MAX; 4]));
        assert!(dist[3].iter().eq(&[isize::MAX; 4]));
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
    fn center() {
        use crate::{
            algo::floyd_warshall::distances,
            gen::Empty,
            op::AddWeightedArc,
        };

        // 0 -> {2}
        // 1 -> {3}
        // 2 -> {0, 3, 4}
        // 3 -> {1, 2, 5}
        // 4 -> {2, 5, 6}
        // 5 -> {3, 4, 7}
        // 6 -> {5, 8}
        // 7 -> {5}
        // 8 -> {6}

        let mut digraph = Vec::<Vec<(usize, isize)>>::empty(9);

        digraph.add_weighted_arc(0, 2, 1);
        digraph.add_weighted_arc(1, 3, 1);
        digraph.add_weighted_arc(2, 0, 1);
        digraph.add_weighted_arc(2, 3, 1);
        digraph.add_weighted_arc(2, 4, 1);
        digraph.add_weighted_arc(3, 1, 1);
        digraph.add_weighted_arc(3, 2, 1);
        digraph.add_weighted_arc(3, 5, 1);
        digraph.add_weighted_arc(4, 2, 1);
        digraph.add_weighted_arc(4, 5, 1);
        digraph.add_weighted_arc(4, 6, 1);
        digraph.add_weighted_arc(5, 3, 1);
        digraph.add_weighted_arc(5, 4, 1);
        digraph.add_weighted_arc(5, 7, 1);
        digraph.add_weighted_arc(6, 5, 1);
        digraph.add_weighted_arc(6, 8, 1);
        digraph.add_weighted_arc(7, 5, 1);
        digraph.add_weighted_arc(8, 6, 1);

        let dist = distances(&digraph);

        assert!(dist.center().iter().eq(&[2, 4, 5]));
    }

    #[test]
    fn eccentricities() {
        use crate::{
            algo::floyd_warshall::distances,
            gen::Empty,
            op::AddWeightedArc,
        };

        // 0 -> {2}
        // 1 -> {3}
        // 2 -> {0, 3, 4}
        // 3 -> {1, 2, 5}
        // 4 -> {2, 5, 6}
        // 5 -> {3, 4, 7}
        // 6 -> {5, 8}
        // 7 -> {5}
        // 8 -> {6}

        let mut digraph = Vec::<Vec<(usize, isize)>>::empty(9);

        digraph.add_weighted_arc(0, 2, 1);
        digraph.add_weighted_arc(1, 3, 1);
        digraph.add_weighted_arc(2, 0, 1);
        digraph.add_weighted_arc(2, 3, 1);
        digraph.add_weighted_arc(2, 4, 1);
        digraph.add_weighted_arc(3, 1, 1);
        digraph.add_weighted_arc(3, 2, 1);
        digraph.add_weighted_arc(3, 5, 1);
        digraph.add_weighted_arc(4, 2, 1);
        digraph.add_weighted_arc(4, 5, 1);
        digraph.add_weighted_arc(4, 6, 1);
        digraph.add_weighted_arc(5, 3, 1);
        digraph.add_weighted_arc(5, 4, 1);
        digraph.add_weighted_arc(5, 7, 1);
        digraph.add_weighted_arc(6, 5, 1);
        digraph.add_weighted_arc(6, 8, 1);
        digraph.add_weighted_arc(7, 5, 1);
        digraph.add_weighted_arc(8, 6, 1);

        let dist = distances(&digraph);

        assert!(dist
            .eccentricities()
            .iter()
            .eq(&[4, 5, 3, 4, 3, 3, 4, 4, 5]));
    }
}

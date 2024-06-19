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
    /// # Returns
    ///
    /// A distance matrix.
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
        Self {
            dist: vec![vec![max; v]; v],
            max,
        }
    }

    /// Finds the center of a distance matrix.
    ///
    /// # Returns
    ///
    /// The vertices with the smallest eccentricity.
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
    pub fn center(&self) -> Vec<usize>
    where
        W: Copy + Ord,
    {
        let mut center = Vec::new();
        let mut ecc_min = self.max;

        for (i, row) in self.dist.iter().enumerate() {
            if let Some(&ecc_next) = row.iter().max() {
                match ecc_next.cmp(&ecc_min) {
                    Less => {
                        ecc_min = ecc_next;

                        center.clear();
                        center.push(i);
                    }
                    Equal => {
                        center.push(i);
                    }
                    Greater => {}
                }
            }
        }

        center
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
    fn test_new() {
        let dist = DistanceMatrix::new(4, isize::MAX);

        assert_eq!(dist.max, isize::MAX);
        assert!(dist[0].iter().eq(&[0; 4]));
        assert!(dist[1].iter().eq(&[0; 4]));
        assert!(dist[2].iter().eq(&[0; 4]));
        assert!(dist[3].iter().eq(&[0; 4]));
    }

    #[test]
    fn test_index() {
        let dist = DistanceMatrix::new(4, isize::MAX);

        assert_eq!(dist[0][0], 0);
        assert_eq!(dist[0][1], 0);
        assert_eq!(dist[0][2], 0);
        assert_eq!(dist[0][3], 0);
        assert_eq!(dist[1][0], 0);
        assert_eq!(dist[1][1], 0);
        assert_eq!(dist[1][2], 0);
        assert_eq!(dist[1][3], 0);
        assert_eq!(dist[2][0], 0);
        assert_eq!(dist[2][1], 0);
        assert_eq!(dist[2][2], 0);
        assert_eq!(dist[2][3], 0);
        assert_eq!(dist[3][0], 0);
        assert_eq!(dist[3][1], 0);
        assert_eq!(dist[3][2], 0);
        assert_eq!(dist[3][3], 0);
    }

    #[test]
    fn test_index_mut() {
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
}

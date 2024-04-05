//! A trait to count all edges in a graph
//!
//! # Examples
//!
//! ```
//! use graaf::ops::CountAllEdges;
//!
//! let graph: Vec<Vec<usize>> = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
//!
//! assert_eq!(graph.count_all_edges(), 10);
//! ```

use {
    core::hash::BuildHasher,
    std::collections::{
        HashMap,
        HashSet,
    },
};

/// A trait to count all edges in a graph
///
/// # Examples
///
/// ```
/// use graaf::ops::CountAllEdges;
///
/// let graph: Vec<Vec<usize>> = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
///
/// assert_eq!(graph.count_all_edges(), 10);
/// ```
pub trait CountAllEdges {
    /// Counts all edges.
    fn count_all_edges(&self) -> usize;
}

// Vec

impl<T> CountAllEdges for Vec<Vec<T>> {
    /// # Panics
    ///
    /// Panics if the number of edges exceeds `usize::MAX`.
    fn count_all_edges(&self) -> usize {
        self.iter().map(Vec::len).sum()
    }
}

impl<T, H> CountAllEdges for Vec<HashSet<T, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if the number of edges exceeds `usize::MAX`.
    fn count_all_edges(&self) -> usize {
        self.iter().map(HashSet::len).sum()
    }
}

impl<K, W, H> CountAllEdges for Vec<HashMap<K, W, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if the number of edges exceeds `usize::MAX`.
    fn count_all_edges(&self) -> usize {
        self.iter().map(HashMap::len).sum()
    }
}

// Arr

impl<const V: usize, T> CountAllEdges for [Vec<T>; V] {
    /// # Panics
    ///
    /// Panics if the number of edges exceeds `usize::MAX`.
    fn count_all_edges(&self) -> usize {
        self.iter().map(Vec::len).sum()
    }
}

impl<const V: usize, T, H> CountAllEdges for [HashSet<T, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if the number of edges exceeds `usize::MAX`.
    fn count_all_edges(&self) -> usize {
        self.iter().map(HashSet::len).sum()
    }
}

impl<const V: usize, K, W, H> CountAllEdges for [HashMap<K, W, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if the number of edges exceeds `usize::MAX`.
    fn count_all_edges(&self) -> usize {
        self.iter().map(HashMap::len).sum()
    }
}

// HashMap

impl<K, T, H> CountAllEdges for HashMap<K, Vec<T>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if the number of edges exceeds `usize::MAX`.
    fn count_all_edges(&self) -> usize {
        self.values().map(Vec::len).sum()
    }
}

impl<K, T, H> CountAllEdges for HashMap<K, HashSet<T, H>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if the number of edges exceeds `usize::MAX`.
    fn count_all_edges(&self) -> usize {
        self.values().map(HashSet::len).sum()
    }
}

impl<K, W, H> CountAllEdges for HashMap<K, HashMap<K, W, H>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if the number of edges exceeds `usize::MAX`.
    fn count_all_edges(&self) -> usize {
        self.values().map(HashMap::len).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_vec() {
        let graph: Vec<Vec<usize>> = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        assert_eq!(graph.count_all_edges(), 10);
    }

    #[test]
    fn vec_hash_set() {
        let graph: Vec<HashSet<usize>> = vec![
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        assert_eq!(graph.count_all_edges(), 10);
    }

    #[test]
    fn vec_hash_map() {
        let graph: Vec<HashMap<usize, usize>> = vec![
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.count_all_edges(), 5);
    }

    #[test]
    fn arr_vec() {
        let graph: [Vec<usize>; 4] = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        assert_eq!(graph.count_all_edges(), 10);
    }

    #[test]
    fn arr_hash_set() {
        let graph: [HashSet<usize>; 4] = [
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        assert_eq!(graph.count_all_edges(), 10);
    }

    #[test]
    fn arr_hash_map() {
        let graph: [HashMap<usize, usize>; 3] = [
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.count_all_edges(), 5);
    }

    #[test]
    fn hash_map_vec() {
        let graph: HashMap<usize, Vec<usize>> =
            HashMap::from([(0, vec![1, 2]), (1, vec![0, 2]), (2, vec![0, 1])]);

        assert_eq!(graph.count_all_edges(), 6);
    }

    #[test]
    fn hash_map_hash_set() {
        let graph: HashMap<usize, HashSet<usize>> = HashMap::from([
            (0, HashSet::from([1, 2])),
            (1, HashSet::from([0, 2])),
            (2, HashSet::from([0, 1])),
        ]);

        assert_eq!(graph.count_all_edges(), 6);
    }

    #[test]
    fn hash_map_hash_map() {
        let graph: HashMap<usize, HashMap<usize, usize>> = HashMap::from([
            (0, HashMap::from([(1, 2), (2, 3)])),
            (1, HashMap::from([(0, 4)])),
            (2, HashMap::from([(0, 7), (1, 8)])),
        ]);

        assert_eq!(graph.count_all_edges(), 5);
    }
}

//! A trait to get the outdegree of a given vertex
//!
//! # Examples
//!
//! ```
//! use graaf::op::Outdegree;
//!
//! let graph = vec![vec![1, 2], vec![0], vec![1]];
//!
//! assert_eq!(graph.outdegree(0), 2);
//! assert_eq!(graph.outdegree(1), 1);
//! assert_eq!(graph.outdegree(2), 1);
//! ```

use {
    core::hash::BuildHasher,
    std::collections::{
        HashMap,
        HashSet,
    },
};

/// A trait to get the outdegree of a given vertex
///
/// # How can I implement `Outdegree`?
///
/// Provide an implementation of `outdegree` that returns the outdegree of the
/// target vertex.
///
/// ```
/// use graaf::op::Outdegree;
///
/// struct Graph {
///     edges: Vec<Vec<usize>>,
/// }
///
/// impl Outdegree for Graph {
///     fn outdegree(&self, s: usize) -> usize {
///         self.edges[s].len()
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::Outdegree;
///
/// let graph = vec![vec![1, 2], vec![0], vec![1]];
///
/// assert_eq!(graph.outdegree(0), 2);
/// assert_eq!(graph.outdegree(1), 1);
/// assert_eq!(graph.outdegree(2), 1);
/// ```
pub trait Outdegree {
    /// Return the outdegree of a vertex.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    fn outdegree(&self, s: usize) -> usize;
}

impl<T> Outdegree for Vec<Vec<T>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<T, H> Outdegree for Vec<HashSet<T, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<K, W, H> Outdegree for Vec<HashMap<K, W, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<T> Outdegree for [Vec<T>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<T, H> Outdegree for [HashSet<T, H>]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<K, W, H> Outdegree for [HashMap<K, W, H>]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<const V: usize, T> Outdegree for [Vec<T>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<const V: usize, T, H> Outdegree for [HashSet<T, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<const V: usize, K, W, H> Outdegree for [HashMap<K, W, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<H> Outdegree for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[&s].len()
    }
}

impl<H> Outdegree for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[&s].len()
    }
}

impl<H> Outdegree for HashMap<usize, HashMap<usize, usize, H>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[&s].len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_vec() {
        let graph = vec![vec![1, 2], vec![0], vec![1]];

        assert_eq!(graph.outdegree(0), 2);
        assert_eq!(graph.outdegree(1), 1);
        assert_eq!(graph.outdegree(2), 1);
    }

    #[test]
    fn vec_hash_set() {
        let graph = vec![
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([1]),
        ];

        assert_eq!(graph.outdegree(0), 2);
        assert_eq!(graph.outdegree(1), 1);
        assert_eq!(graph.outdegree(2), 1);
    }

    #[test]
    fn vec_hash_map() {
        let graph = vec![
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(1, 1)]),
        ];

        assert_eq!(graph.outdegree(0), 2);
        assert_eq!(graph.outdegree(1), 1);
        assert_eq!(graph.outdegree(2), 1);
    }

    #[test]
    fn slice_vec() {
        let graph: &[Vec<usize>] = &[vec![1, 2], vec![0], vec![1]];

        assert_eq!(graph.outdegree(0), 2);
        assert_eq!(graph.outdegree(1), 1);
        assert_eq!(graph.outdegree(2), 1);
    }

    #[test]
    fn slice_hash_set() {
        let graph: &[HashSet<usize>] = &[
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([1]),
        ];

        assert_eq!(graph.outdegree(0), 2);
        assert_eq!(graph.outdegree(1), 1);
        assert_eq!(graph.outdegree(2), 1);
    }

    #[test]
    fn slice_hash_map() {
        let graph: &[HashMap<usize, i32>] = &[
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(1, 1)]),
        ];

        assert_eq!(graph.outdegree(0), 2);
        assert_eq!(graph.outdegree(1), 1);
        assert_eq!(graph.outdegree(2), 1);
    }

    #[test]
    fn arr_vec() {
        let graph = [vec![1, 2], vec![0], vec![1]];

        assert_eq!(graph.outdegree(0), 2);
        assert_eq!(graph.outdegree(1), 1);
        assert_eq!(graph.outdegree(2), 1);
    }

    #[test]
    fn arr_hash_set() {
        let graph = [
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([1]),
        ];

        assert_eq!(graph.outdegree(0), 2);
        assert_eq!(graph.outdegree(1), 1);
        assert_eq!(graph.outdegree(2), 1);
    }

    #[test]
    fn arr_hash_map() {
        let graph = [
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(1, 1)]),
        ];

        assert_eq!(graph.outdegree(0), 2);
        assert_eq!(graph.outdegree(1), 1);
        assert_eq!(graph.outdegree(2), 1);
    }

    #[test]
    fn hash_map_vec() {
        let graph = HashMap::from([(0, vec![1, 2]), (1, vec![0]), (2, vec![1])]);

        assert_eq!(graph.outdegree(0), 2);
        assert_eq!(graph.outdegree(1), 1);
        assert_eq!(graph.outdegree(2), 1);
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = HashMap::from([
            (0, HashSet::from([1, 2])),
            (1, HashSet::from([0])),
            (2, HashSet::from([1])),
        ]);

        assert_eq!(graph.outdegree(0), 2);
        assert_eq!(graph.outdegree(1), 1);
        assert_eq!(graph.outdegree(2), 1);
    }

    #[test]
    fn hash_map_hash_map() {
        let graph = HashMap::from([
            (0, HashMap::from([(1, 1), (2, 1)])),
            (1, HashMap::from([(0, 1)])),
            (2, HashMap::from([(1, 1)])),
        ]);

        assert_eq!(graph.outdegree(0), 2);
        assert_eq!(graph.outdegree(1), 1);
        assert_eq!(graph.outdegree(2), 1);
    }
}

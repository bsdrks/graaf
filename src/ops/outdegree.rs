use {
    core::hash::BuildHasher,
    std::collections::{
        HashMap,
        HashSet,
    },
};

/// A trait to get the outdegree of a given vertex
pub trait Outdegree {
    /// Returns the outdegree of a vertex.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    fn outdegree(&self, s: usize) -> usize;
}

// Vec

impl<T> Outdegree for Vec<Vec<T>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<H> Outdegree for Vec<HashSet<usize, H>>
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

// Arr

impl<const V: usize, T> Outdegree for [Vec<T>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<const V: usize, H> Outdegree for [HashSet<usize, H>; V]
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

// HashMap

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

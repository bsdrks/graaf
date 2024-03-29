use std::{
    collections::{
        HashMap,
        HashSet,
    },
    hash::BuildHasher,
};

/// A trait for getting the in-degree of a vertex.
pub trait InDegree {
    /// Returns the in-degree of a vertex.
    ///
    /// # Arguments
    ///
    /// * `t`: The target vertex.
    fn in_degree(&self, t: usize) -> usize;
}

// Vec

impl<H> InDegree for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    fn in_degree(&self, t: usize) -> usize {
        self.iter().filter(|set| set.contains(&t)).count()
    }
}

impl<W, H> InDegree for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn in_degree(&self, t: usize) -> usize {
        self.iter().filter(|map| map.contains_key(&t)).count()
    }
}

// Arr

impl<const V: usize, H> InDegree for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    fn in_degree(&self, t: usize) -> usize {
        self.iter().filter(|set| set.contains(&t)).count()
    }
}

impl<const V: usize, W, H> InDegree for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn in_degree(&self, t: usize) -> usize {
        self.iter().filter(|map| map.contains_key(&t)).count()
    }
}

// HashMap

impl<H> InDegree for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    fn in_degree(&self, t: usize) -> usize {
        self.values()
            .map(|set| set.iter().filter(|&&u| u == t).count())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_hash_set() {
        let graph: Vec<HashSet<usize>> =
            vec![HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];

        assert_eq!(graph.in_degree(0), 0);
        assert_eq!(graph.in_degree(1), 1);
        assert_eq!(graph.in_degree(2), 2);
    }

    #[test]
    fn vec_hash_map() {
        let graph: Vec<HashMap<usize, usize>> = vec![
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(2, 1)]),
            HashMap::from([]),
        ];

        assert_eq!(graph.in_degree(0), 0);
        assert_eq!(graph.in_degree(1), 1);
        assert_eq!(graph.in_degree(2), 2);
    }

    #[test]
    fn arr_hash_set() {
        let graph: [HashSet<usize>; 3] =
            [HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];

        assert_eq!(graph.in_degree(0), 0);
        assert_eq!(graph.in_degree(1), 1);
        assert_eq!(graph.in_degree(2), 2);
    }

    #[test]
    fn arr_hash_map() {
        let graph: [HashMap<usize, usize>; 3] = [
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(2, 1)]),
            HashMap::from([]),
        ];

        assert_eq!(graph.in_degree(0), 0);
        assert_eq!(graph.in_degree(1), 1);
        assert_eq!(graph.in_degree(2), 2);
    }

    #[test]
    fn hash_map() {
        let graph: HashMap<usize, HashSet<usize>> = HashMap::from([
            (0, HashSet::from([1, 2])),
            (1, HashSet::from([2])),
            (2, HashSet::new()),
        ]);

        assert_eq!(graph.in_degree(0), 0);
        assert_eq!(graph.in_degree(1), 1);
        assert_eq!(graph.in_degree(2), 2);
    }
}

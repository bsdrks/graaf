use std::{
    collections::{
        HashMap,
        HashSet,
    },
    hash::BuildHasher,
};

/// A trait for getting the edges of a vertex in a graph.
///
/// # Example
///
/// ```
/// use graph::IterEdges;
///
/// let graph: [Vec<usize>; 4] = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
/// let mut iter = graph.iter_edges(0);
///
/// assert_eq!(iter.next(), Some(1));
/// assert_eq!(iter.next(), Some(2));
/// assert_eq!(iter.next(), None);
///
/// let mut iter = graph.iter_edges(1);
///
/// assert_eq!(iter.next(), Some(0));
/// assert_eq!(iter.next(), Some(2));
/// assert_eq!(iter.next(), Some(3));
/// assert_eq!(iter.next(), None);
///
/// let mut iter = graph.iter_edges(2);
///
/// assert_eq!(iter.next(), Some(0));
/// assert_eq!(iter.next(), Some(1));
/// assert_eq!(iter.next(), Some(3));
/// assert_eq!(iter.next(), None);
///
/// let mut iter = graph.iter_edges(3);
///
/// assert_eq!(iter.next(), Some(1));
/// assert_eq!(iter.next(), Some(2));
/// assert_eq!(iter.next(), None);
/// ```
pub trait IterEdges {
    /// Returns an iterator over the edges of the vertex `s`.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize>;
}

// Vec

impl IterEdges for Vec<Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl<H> IterEdges for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

// Arr

impl<const V: usize> IterEdges for [Vec<usize>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl<const V: usize, H> IterEdges for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

// HashMap

impl<H> IterEdges for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[&s].iter().copied()
    }
}

impl<H> IterEdges for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[&s].iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use {
        crate::IterEdges,
        std::{
            assert_matches::assert_matches,
            collections::{
                HashMap,
                HashSet,
            },
        },
    };

    #[test]
    fn vec_vec() {
        let graph: Vec<Vec<usize>> = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
        let mut iter = graph.iter_edges(0);

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);

        let mut iter = graph.iter_edges(1);

        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);

        let mut iter = graph.iter_edges(2);

        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);

        let mut iter = graph.iter_edges(3);

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn vec_hash_set() {
        let graph = vec![
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        let mut iter = graph.iter_edges(0);

        assert_matches!(iter.next(), Some(1 | 2));
        assert_matches!(iter.next(), Some(1 | 2));
        assert_eq!(iter.next(), None);

        let mut iter = graph.iter_edges(1);

        assert_matches!(iter.next(), Some(0 | 2 | 3));
        assert_matches!(iter.next(), Some(0 | 2 | 3));
        assert_matches!(iter.next(), Some(0 | 2 | 3));
        assert_eq!(iter.next(), None);

        let mut iter = graph.iter_edges(2);

        assert_matches!(iter.next(), Some(0 | 1 | 3));
        assert_matches!(iter.next(), Some(0 | 1 | 3));
        assert_matches!(iter.next(), Some(0 | 1 | 3));
        assert_eq!(iter.next(), None);

        let mut iter = graph.iter_edges(3);

        assert_matches!(iter.next(), Some(1 | 2));
        assert_matches!(iter.next(), Some(1 | 2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn arr_vec() {
        let graph: [Vec<usize>; 4] = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
        let mut iter = graph.iter_edges(0);

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);

        let mut iter = graph.iter_edges(1);

        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);

        let mut iter = graph.iter_edges(2);

        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);

        let mut iter = graph.iter_edges(3);

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn arr_hash_set() {
        let graph = [
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        let mut iter = graph.iter_edges(0);

        assert_matches!(iter.next(), Some(1 | 2));
        assert_matches!(iter.next(), Some(1 | 2));
        assert_eq!(iter.next(), None);

        let mut iter = graph.iter_edges(1);

        assert_matches!(iter.next(), Some(0 | 2 | 3));
        assert_matches!(iter.next(), Some(0 | 2 | 3));
        assert_matches!(iter.next(), Some(0 | 2 | 3));
        assert_eq!(iter.next(), None);

        let mut iter = graph.iter_edges(2);

        assert_matches!(iter.next(), Some(0 | 1 | 3));
        assert_matches!(iter.next(), Some(0 | 1 | 3));
        assert_matches!(iter.next(), Some(0 | 1 | 3));
        assert_eq!(iter.next(), None);

        let mut iter = graph.iter_edges(3);

        assert_matches!(iter.next(), Some(1 | 2));
        assert_matches!(iter.next(), Some(1 | 2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn hash_map_vec() {
        let graph: HashMap<usize, Vec<usize>> = [
            (0, vec![1, 2]),
            (1, vec![0, 2, 3]),
            (2, vec![0, 1, 3]),
            (3, vec![1, 2]),
        ]
        .iter()
        .cloned()
        .collect();

        let mut iter = graph.iter_edges(0);

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);

        let mut iter = graph.iter_edges(1);

        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);

        let mut iter = graph.iter_edges(2);

        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);

        let mut iter = graph.iter_edges(3);

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn hash_map_hash_set() {
        let graph: HashMap<usize, HashSet<usize>> = [
            (0, HashSet::from([1, 2])),
            (1, HashSet::from([0, 2, 3])),
            (2, HashSet::from([0, 1, 3])),
            (3, HashSet::from([1, 2])),
        ]
        .iter()
        .cloned()
        .collect();

        let mut iter = graph.iter_edges(0);

        assert_matches!(iter.next(), Some(1 | 2));
        assert_matches!(iter.next(), Some(1 | 2));
        assert_eq!(iter.next(), None);

        let mut iter = graph.iter_edges(1);

        assert_matches!(iter.next(), Some(0 | 2 | 3));
        assert_matches!(iter.next(), Some(0 | 2 | 3));
        assert_matches!(iter.next(), Some(0 | 2 | 3));
        assert_eq!(iter.next(), None);

        let mut iter = graph.iter_edges(2);

        assert_matches!(iter.next(), Some(0 | 1 | 3));
        assert_matches!(iter.next(), Some(0 | 1 | 3));
        assert_matches!(iter.next(), Some(0 | 1 | 3));
        assert_eq!(iter.next(), None);

        let mut iter = graph.iter_edges(3);

        assert_matches!(iter.next(), Some(1 | 2));
        assert_matches!(iter.next(), Some(1 | 2));
        assert_eq!(iter.next(), None);
    }
}

use std::{
    collections::HashSet,
    hash::BuildHasher,
};

pub trait IterEdges {
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize>;
}

impl IterEdges for Vec<Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(V)
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl<const V: usize> IterEdges for [Vec<usize>; V] {
    /// # Panics
    ///
    /// Panics if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(V)
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
    /// Panics if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(V)
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
    /// Panics if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(V)
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use {
        crate::IterEdges,
        std::{
            assert_matches::assert_matches,
            collections::HashSet,
        },
    };

    #[test]
    fn hash_set() {
        let graph: Vec<HashSet<usize>> = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]]
            .into_iter()
            .map(|v| v.into_iter().collect())
            .collect();

        let mut iter = graph.iter_edges(0);

        assert_matches!(iter.next(), Some(1) | Some(2));
        assert_matches!(iter.next(), Some(1) | Some(2));
        assert_eq!(iter.next(), None);

        let mut iter = graph.iter_edges(1);

        assert_matches!(iter.next(), Some(0) | Some(2) | Some(3));
        assert_matches!(iter.next(), Some(0) | Some(2) | Some(3));
        assert_matches!(iter.next(), Some(0) | Some(2) | Some(3));
        assert_eq!(iter.next(), None);

        let mut iter = graph.iter_edges(2);

        assert_matches!(iter.next(), Some(0) | Some(1) | Some(3));
        assert_matches!(iter.next(), Some(0) | Some(1) | Some(3));
        assert_matches!(iter.next(), Some(0) | Some(1) | Some(3));
        assert_eq!(iter.next(), None);

        let mut iter = graph.iter_edges(3);

        assert_matches!(iter.next(), Some(1) | Some(2));
        assert_matches!(iter.next(), Some(1) | Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn vec() {
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
}

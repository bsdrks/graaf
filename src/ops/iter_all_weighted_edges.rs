use std::{
    collections::HashSet,
    hash::BuildHasher,
};

/// A trait for graph representations to iterate over all weighted edges in a
/// graph.
pub trait IterAllWeightedEdges<W> {
    /// Returns an iterator that iterates over all weighted edges in a graph.
    fn iter_all_weighted_edges<'a>(&'a self) -> impl Iterator<Item = &(usize, usize, W)>
    where
        W: 'a;
}

// Vec

impl<W> IterAllWeightedEdges<W> for Vec<(usize, usize, W)> {
    fn iter_all_weighted_edges<'a>(&'a self) -> impl Iterator<Item = &(usize, usize, W)>
    where
        W: 'a,
    {
        self.iter()
    }
}

// Arr

impl<const V: usize, W> IterAllWeightedEdges<W> for [(usize, usize, W); V] {
    fn iter_all_weighted_edges<'a>(&'a self) -> impl Iterator<Item = &(usize, usize, W)>
    where
        W: 'a,
    {
        self.iter()
    }
}

// HashSet

impl<W, H> IterAllWeightedEdges<W> for HashSet<(usize, usize, W), H>
where
    H: BuildHasher,
{
    fn iter_all_weighted_edges<'a>(&'a self) -> impl Iterator<Item = &(usize, usize, W)>
    where
        W: 'a,
    {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::assert_matches::assert_matches,
    };

    #[test]
    fn vec() {
        let graph = vec![(0, 1, 2), (1, 2, 3), (2, 0, 4)];
        let mut iter = graph.iter_all_weighted_edges();

        assert_eq!(iter.next(), Some(&(0, 1, 2)));
        assert_eq!(iter.next(), Some(&(1, 2, 3)));
        assert_eq!(iter.next(), Some(&(2, 0, 4)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn arr() {
        let graph = [(0, 1, 2), (1, 2, 3), (2, 0, 4)];
        let mut iter = graph.iter_all_weighted_edges();

        assert_eq!(iter.next(), Some(&(0, 1, 2)));
        assert_eq!(iter.next(), Some(&(1, 2, 3)));
        assert_eq!(iter.next(), Some(&(2, 0, 4)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn hash_set() {
        let graph: HashSet<(usize, usize, usize)> =
            HashSet::from([(0, 1, 2), (1, 2, 3), (2, 0, 4)]);

        let mut iter = graph.iter_all_weighted_edges();

        assert_matches!(iter.next(), Some(&(0, 1, 2) | &(1, 2, 3) | &(2, 0, 4)));
        assert_matches!(iter.next(), Some(&(0, 1, 2) | &(1, 2, 3) | &(2, 0, 4)));
        assert_matches!(iter.next(), Some(&(0, 1, 2) | &(1, 2, 3) | &(2, 0, 4)));
        assert_matches!(iter.next(), None);
    }
}

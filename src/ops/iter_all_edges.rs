/// A trait for graph representations to iterate over all edges in a graph.
pub trait IterAllEdges {
    /// Returns an iterator that iterates over all edges in a graph.
    fn iter_all_edges(&self) -> impl Iterator<Item = &(usize, usize)>;
}

// Vec

impl IterAllEdges for Vec<(usize, usize)> {
    fn iter_all_edges(&self) -> impl Iterator<Item = &(usize, usize)> {
        self.iter()
    }
}

// Arr

impl<const V: usize> IterAllEdges for [(usize, usize); V] {
    fn iter_all_edges(&self) -> impl Iterator<Item = &(usize, usize)> {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec() {
        let graph = vec![(0, 1), (1, 2), (2, 0)];
        let mut iter = graph.iter_all_edges();

        assert_eq!(iter.next(), Some(&(0, 1)));
        assert_eq!(iter.next(), Some(&(1, 2)));
        assert_eq!(iter.next(), Some(&(2, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn arr() {
        let graph = [(0, 1), (1, 2), (2, 0)];
        let mut iter = graph.iter_all_edges();

        assert_eq!(iter.next(), Some(&(0, 1)));
        assert_eq!(iter.next(), Some(&(1, 2)));
        assert_eq!(iter.next(), Some(&(2, 0)));
        assert_eq!(iter.next(), None);
    }
}

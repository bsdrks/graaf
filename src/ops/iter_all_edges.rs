pub trait IterAllEdges {
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)>;
}

impl IterAllEdges for Vec<(usize, usize)> {
    /// # Complexity
    ///
    /// O(V)
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter().copied()
    }
}

impl<const V: usize> IterAllEdges for [(usize, usize); V] {
    /// # Complexity
    ///
    /// O(V)
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter().copied()
    }
}

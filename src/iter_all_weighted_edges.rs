pub trait IterAllWeightedEdges<W> {
    fn iter_all_weighted_edges(&self) -> impl Iterator<Item = (usize, usize, W)>;
}

impl<W> IterAllWeightedEdges<W> for Vec<(usize, usize, W)>
where
    W: Copy,
{
    /// # Complexity
    ///
    /// O(V)
    fn iter_all_weighted_edges(&self) -> impl Iterator<Item = (usize, usize, W)> {
        self.iter().copied()
    }
}

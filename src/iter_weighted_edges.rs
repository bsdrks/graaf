pub trait IterWeightedEdges<W> {
    fn iter_weighted_edges(&self, s: usize) -> impl Iterator<Item = (usize, W)>;
}

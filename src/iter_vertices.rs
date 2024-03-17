pub trait IterVertices {
    fn iter_vertices(&self) -> impl Iterator<Item = usize>;
}

impl<E> IterVertices for Vec<E> {
    /// # Complexity
    ///
    /// O(V)
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

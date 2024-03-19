pub trait IterVertices {
    fn iter_vertices(&self) -> impl Iterator<Item = usize>;
}

impl<T> IterVertices for Vec<T> {
    /// # Complexity
    ///
    /// O(V)
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<T, const V: usize> IterVertices for [T; V] {
    /// # Complexity
    ///
    /// O(V)
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..V
    }
}

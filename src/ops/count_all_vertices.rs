pub trait CountAllVertices {
    fn count_all_vertices(&self) -> usize;
}

impl<T> CountAllVertices for Vec<T> {
    /// # Complexity
    ///
    /// O(1)
    fn count_all_vertices(&self) -> usize {
        self.len()
    }
}

impl<const V: usize, T> CountAllVertices for [T; V] {
    /// # Complexity
    ///
    /// O(1)
    fn count_all_vertices(&self) -> usize {
        V
    }
}

pub trait CountAllVertices {
    fn count_all_vertices(&self) -> usize;
}

impl<E> CountAllVertices for Vec<E> {
    /// # Complexity
    ///
    /// O(1)
    fn count_all_vertices(&self) -> usize {
        self.len()
    }
}

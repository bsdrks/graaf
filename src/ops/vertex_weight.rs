/// A trait to get the weight of a given vertex
pub trait VertexWeight<W> {
    /// Get the weight of vertex `s`.
    ///
    /// # Arguments
    ///
    /// * `s`: The vertex.
    fn vertex_weight(&self, s: usize) -> Option<&W>;
}

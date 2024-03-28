/// A trait for getting the weight of a vertex in a graph.
pub trait VertexWeight<W> {
    /// Get the weight of vertex `s`.
    ///
    /// # Arguments
    ///
    /// * `s`: The vertex.
    fn vertex_weight(&self, s: usize) -> Option<&W>;
}

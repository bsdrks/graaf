pub trait VertexWeight<W> {
    fn vertex_weight(&self, s: usize) -> W;
}

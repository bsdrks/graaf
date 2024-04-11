//! Adding an edge with [`crate::op::AddEdge`] should be reflected by
//! [`crate::op::IsEdge`].
use crate::op::{
    AddEdge,
    IsEdge,
};

/// Types that implement [`crate::op::AddEdge`] and [`crate::op::IsEdge`] should
/// ensure that the following property holds for every `graph`, `s`, and `t` of
/// the given types.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `s`: The source vertex.
/// * `t`: The target vertex.
pub fn add_edge_is_edge<G, W>(graph: &mut G, s: usize, t: usize) -> bool
where
    G: AddEdge + IsEdge,
{
    graph.add_edge(s, t);

    graph.is_edge(s, t)
}

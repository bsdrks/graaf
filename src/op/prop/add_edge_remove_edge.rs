//! Adding an edge with [`crate::op::AddEdge`] and then removing it with
//! [`crate::op::RemoveEdge`] should keep the graph unchanged.
use crate::op::{
    AddEdge,
    RemoveEdge,
};

/// Types that implement [`crate::op::AddEdge`] and [`crate::op::RemoveEdge`]
/// should ensure that the following property holds for every `graph`, `s`, and
/// `t` of the given types.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `s`: The source vertex.
/// * `t`: The target vertex.
pub fn add_edge_remove_edge<G, W>(graph: &G, s: usize, t: usize) -> bool
where
    G: AddEdge + Clone + PartialEq + RemoveEdge,
{
    let mut clone = graph.clone();

    clone.add_edge(s, t);
    clone.remove_edge(s, t);

    *graph == clone
}

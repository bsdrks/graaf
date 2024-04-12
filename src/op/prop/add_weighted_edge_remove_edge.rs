//! Adding a weighted edge with [`crate::op::AddWeightedEdge`] and then removing
//! it with [`crate::op::RemoveEdge`] should keep the graph unchanged.
use crate::op::{
    AddWeightedEdge,
    RemoveEdge,
};

/// Adding a weighted edge with [`crate::op::AddWeightedEdge`] and then removing
/// it with [`crate::op::RemoveEdge`] should keep the graph unchanged.
///
/// Types that implement [`crate::op::AddWeightedEdge`] and
/// [`crate::op::RemoveEdge`] should ensure that the following property holds
/// for every `graph`, `s`, and `t` of the given types.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `s`: The source vertex.
/// * `t`: The target vertex.
/// * `w`: The weight of the edge.
pub fn add_weighted_edge_remove_edge<G, W>(graph: &G, s: usize, t: usize, w: W) -> bool
where
    G: AddWeightedEdge<W> + Clone + PartialEq + RemoveEdge,
{
    let mut clone = graph.clone();

    clone.add_weighted_edge(s, t, w);
    clone.remove_edge(s, t);

    *graph == clone
}

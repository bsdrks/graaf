//! Adding a weighted edge with [`crate::op::AddWeightedEdge`] should be
//! reflected by [`crate::op::IsEdge`].
use crate::op::{
    AddWeightedEdge,
    IsEdge,
};

/// Types that implement [`crate::op::AddWeightedEdge`] and
/// [`crate::op::IsEdge`] should ensure that the following property holds for
/// every `graph`, `s`, `t`, and `w` of the given types.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `s`: The source vertex.
/// * `t`: The target vertex.
/// * `w`: The weight of the edge.
pub fn add_weighted_edge_is_edge<G, W>(graph: &mut G, s: usize, t: usize, w: W) -> bool
where
    G: AddWeightedEdge<W> + IsEdge,
{
    graph.add_weighted_edge(s, t, w);

    graph.is_edge(s, t)
}

//! An edge removed with [`crate::op::RemoveEdge`] should no longer in the
//! graph, as reflected by [`crate::op::IsEdge`].
use crate::op::{
    IsEdge,
    RemoveEdge,
};

/// An edge removed with [`crate::op::RemoveEdge`] should no longer in the
/// graph, as reflected by [`crate::op::IsEdge`].
///
/// Types that implement [`crate::op::RemoveEdge`] and [`crate::op::IsEdge`]
/// should ensure that the following property holds for every `graph`, `s`, and
/// `t` of the given types.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `s`: The source vertex.
/// * `t`: The target vertex.
pub fn remove_edge_is_edge<G, W>(graph: &mut G, s: usize, t: usize) -> bool
where
    G: IsEdge + RemoveEdge,
{
    let _ = graph.remove_edge(s, t);

    !graph.is_edge(s, t)
}

//! Properties of traits in [`crate::op`].
use super::{
    AddEdge,
    AddWeightedEdge,
    IsEdge,
    RemoveEdge,
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
pub fn add_weighted_edge_remove_edge<G, W>(graph: &G, s: usize, t: usize, w: W) -> bool
where
    G: AddWeightedEdge<W> + Clone + PartialEq + RemoveEdge,
{
    let mut clone = graph.clone();

    clone.add_weighted_edge(s, t, w);
    clone.remove_edge(s, t);

    *graph == clone
}

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
    graph.remove_edge(s, t);

    !graph.is_edge(s, t)
}

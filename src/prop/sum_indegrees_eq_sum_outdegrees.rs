//! The sum of the in-degrees of all vertices is equal to the sum of the
//! out-degrees of all vertices.

use crate::op::{
    Indegree,
    IterVertices,
    Outdegree,
};

/// Returns whether the sum of the in-degrees of all vertices is equal to the
/// sum of the out-degrees of all vertices.
///
/// Types that implement [`Indegree`] and [`Outdegree`] should ensure that the
/// property holds for every `digraph` of the given type.
///
/// # Arguments
///
/// * `digraph`: The digraph.
///
/// [`Indegree`]: crate::op::Indegree
/// [`Outdegree`]: crate::op::Outdegree
pub fn sum_indegrees_eq_sum_outdegrees<D>(digraph: &D) -> bool
where
    D: Indegree + IterVertices + Outdegree,
{
    digraph
        .iter_vertices()
        .map(|v| digraph.indegree(v))
        .sum::<usize>()
        == digraph
            .iter_vertices()
            .map(|v| digraph.outdegree(v))
            .sum::<usize>()
}

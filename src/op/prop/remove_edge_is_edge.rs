//! An edge removed with [`RemoveEdge`](crate::op::RemoveEdge) should no longer
//! in the graph, as reflected by [`IsEdge`](crate::op::IsEdge).

use crate::op::{
    IsEdge,
    RemoveEdge,
};

/// An edge removed with [`RemoveEdge`](crate::op::RemoveEdge) should no longer
/// in the graph, as reflected by [`IsEdge`](crate::op::IsEdge).
///
/// Types that implement [`RemoveEdge`](crate::op::RemoveEdge)and
/// [`IsEdge`](crate::op::IsEdge) should ensure that the property holds for
/// every `graph`, `s`, and `t` of the given types.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `s`: The source vertex.
/// * `t`: The target vertex.
pub fn remove_edge_is_edge<G>(graph: &mut G, s: usize, t: usize) -> bool
where
    G: IsEdge + RemoveEdge,
{
    let _ = graph.remove_edge(s, t);

    !graph.is_edge(s, t)
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::collections::HashSet,
    };

    macro_rules! test_remove_edge_is_edge {
        ($graph:expr) => {
            assert!(remove_edge_is_edge($graph, 0, 1));
            assert!(remove_edge_is_edge($graph, 0, 2));
            assert!(remove_edge_is_edge($graph, 1, 0));
            assert!(remove_edge_is_edge($graph, 1, 2));
            assert!(remove_edge_is_edge($graph, 2, 0));
            assert!(remove_edge_is_edge($graph, 2, 1));
        };
    }

    #[test]
    fn vec_hash_set() {
        let graph: &mut Vec<HashSet<usize>> = &mut vec![HashSet::new(); 3];

        test_remove_edge_is_edge!(graph);
    }
}

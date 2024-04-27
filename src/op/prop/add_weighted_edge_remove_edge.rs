//! Adding a weighted edge with [`AddWeightedEdge`](crate::op::AddWeightedEdge)
//! and then removing it with [`RemoveEdge`](crate::op::RemoveEdge) should keep
//! the graph unchanged.

use crate::op::{
    AddWeightedEdge,
    RemoveEdge,
};

/// Adding a weighted edge with [`AddWeightedEdge`](crate::op::AddWeightedEdge)
/// and then removing it with [`RemoveEdge`](crate::op::RemoveEdge) should keep
/// the graph unchanged.
///
/// Types that implement [`AddWeightedEdge`](crate::op::AddWeightedEdge) and
/// [`RemoveEdge`](crate::op::RemoveEdge) should ensure that the property holds
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

    let _ = clone.remove_edge(s, t);

    *graph == clone
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::collections::HashMap,
    };

    macro_rules! test_add_weighted_edge_remove_edge {
        ($graph:expr) => {
            assert!(add_weighted_edge_remove_edge($graph, 0, 1, 1));
            assert!(add_weighted_edge_remove_edge($graph, 0, 2, 2));
            assert!(add_weighted_edge_remove_edge($graph, 1, 0, 3));
            assert!(add_weighted_edge_remove_edge($graph, 1, 2, 4));
            assert!(add_weighted_edge_remove_edge($graph, 2, 0, 5));
            assert!(add_weighted_edge_remove_edge($graph, 2, 1, 6));
        };
    }

    #[test]
    fn vec_hash_map() {
        let graph: Vec<HashMap<usize, i32>> = vec![HashMap::new(), HashMap::new(), HashMap::new()];

        test_add_weighted_edge_remove_edge!(&graph);
    }
}

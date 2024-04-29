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
    extern crate alloc;

    use {
        super::*,
        crate::prop::strategy::binop_vertices,
        alloc::collections::BTreeMap,
        proptest::prelude::*,
        std::collections::HashMap,
    };

    proptest! {
        #[test]
        fn vec_btree_map((v, s, t) in binop_vertices(1, 100), w in -100..100_i32) {
            let graph = vec![BTreeMap::new(); v];

            assert!(add_weighted_edge_remove_edge(&graph, s, t, w));
        }

        #[test]
        fn vec_hash_map((v, s, t) in binop_vertices(1, 100), w in -100..100_i32) {
            let graph = vec![HashMap::new(); v];

            assert!(add_weighted_edge_remove_edge(&graph, s, t, w));
        }
    }

    #[test]
    fn arr_btree_map() {
        let graph = [BTreeMap::new(), BTreeMap::new(), BTreeMap::new()];

        assert!(add_weighted_edge_remove_edge(&graph, 0, 0, 0));
        assert!(add_weighted_edge_remove_edge(&graph, 0, 1, 0));
        assert!(add_weighted_edge_remove_edge(&graph, 0, 2, 0));
        assert!(add_weighted_edge_remove_edge(&graph, 1, 0, 0));
        assert!(add_weighted_edge_remove_edge(&graph, 1, 1, 0));
        assert!(add_weighted_edge_remove_edge(&graph, 1, 2, 0));
        assert!(add_weighted_edge_remove_edge(&graph, 2, 0, 0));
        assert!(add_weighted_edge_remove_edge(&graph, 2, 1, 0));
        assert!(add_weighted_edge_remove_edge(&graph, 2, 2, 0));
    }

    #[test]
    fn arr_hash_map() {
        let graph = [HashMap::new(), HashMap::new(), HashMap::new()];

        assert!(add_weighted_edge_remove_edge(&graph, 0, 0, 0));
        assert!(add_weighted_edge_remove_edge(&graph, 0, 1, 0));
        assert!(add_weighted_edge_remove_edge(&graph, 0, 2, 0));
        assert!(add_weighted_edge_remove_edge(&graph, 1, 0, 0));
        assert!(add_weighted_edge_remove_edge(&graph, 1, 1, 0));
        assert!(add_weighted_edge_remove_edge(&graph, 1, 2, 0));
        assert!(add_weighted_edge_remove_edge(&graph, 2, 0, 0));
        assert!(add_weighted_edge_remove_edge(&graph, 2, 1, 0));
        assert!(add_weighted_edge_remove_edge(&graph, 2, 2, 0));
    }
}

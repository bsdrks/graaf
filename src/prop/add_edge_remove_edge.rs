//! Adding an edge with [`AddEdge`](crate::op::AddEdge) and then removing it
//! with [`RemoveEdge`](crate::op::RemoveEdge) should keep the graph unchanged.

use crate::op::{
    AddEdge,
    RemoveEdge,
};

/// Adding an edge with [`AddEdge`](crate::op::AddEdge) and then removing it
/// with [`RemoveEdge`](crate::op::RemoveEdge) should keep the graph unchanged.
///
/// Types that implement [`AddEdge`](crate::op::AddEdge) and
/// [`RemoveEdge`](crate::op::RemoveEdge) should ensure that the property holds
/// for every `graph`, `s`, and `t` of the given types.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `s`: The source vertex.
/// * `t`: The target vertex.
pub fn add_edge_remove_edge<G>(graph: &G, s: usize, t: usize) -> bool
where
    G: AddEdge + Clone + PartialEq + RemoveEdge,
{
    let mut clone = graph.clone();

    clone.add_edge(s, t);

    let _ = clone.remove_edge(s, t);

    *graph == clone
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use {
        super::*,
        crate::prop::strategy::binop_vertices,
        alloc::collections::BTreeSet,
        proptest::prelude::*,
        std::collections::HashSet,
    };

    proptest! {
        #[test]
        fn vec_btree_set((v, s, t) in binop_vertices(1, 100)) {
            let graph = vec![HashSet::new(); v];

            assert!(add_edge_remove_edge(&graph, s, t));
        }

        #[test]
        fn vec_hash_set((v, s, t) in binop_vertices(1, 100)) {
            let graph = vec![HashSet::new(); v];

            assert!(add_edge_remove_edge(&graph, s, t));
        }
    }

    #[test]
    fn arr_btree_set() {
        let graph = [BTreeSet::new(), BTreeSet::new(), BTreeSet::new()];

        assert!(add_edge_remove_edge(&graph, 0, 0));
        assert!(add_edge_remove_edge(&graph, 0, 1));
        assert!(add_edge_remove_edge(&graph, 0, 2));
        assert!(add_edge_remove_edge(&graph, 1, 0));
        assert!(add_edge_remove_edge(&graph, 1, 1));
        assert!(add_edge_remove_edge(&graph, 1, 2));
        assert!(add_edge_remove_edge(&graph, 2, 0));
        assert!(add_edge_remove_edge(&graph, 2, 1));
        assert!(add_edge_remove_edge(&graph, 2, 2));
    }

    #[test]
    fn arr_hash_set() {
        let graph = [HashSet::new(), HashSet::new(), HashSet::new()];

        assert!(add_edge_remove_edge(&graph, 0, 0));
        assert!(add_edge_remove_edge(&graph, 0, 1));
        assert!(add_edge_remove_edge(&graph, 0, 2));
        assert!(add_edge_remove_edge(&graph, 1, 0));
        assert!(add_edge_remove_edge(&graph, 1, 1));
        assert!(add_edge_remove_edge(&graph, 1, 2));
        assert!(add_edge_remove_edge(&graph, 2, 0));
        assert!(add_edge_remove_edge(&graph, 2, 1));
        assert!(add_edge_remove_edge(&graph, 2, 2));
    }

    #[cfg(feature = "adjacency_matrix")]
    #[test]
    fn adjacency_matrix() {
        use crate::repr::AdjacencyMatrix;

        let graph = AdjacencyMatrix::<3>::new();

        assert!(add_edge_remove_edge(&graph, 0, 0));
        assert!(add_edge_remove_edge(&graph, 0, 1));
        assert!(add_edge_remove_edge(&graph, 0, 2));
        assert!(add_edge_remove_edge(&graph, 1, 0));
        assert!(add_edge_remove_edge(&graph, 1, 1));
        assert!(add_edge_remove_edge(&graph, 1, 2));
        assert!(add_edge_remove_edge(&graph, 2, 0));
        assert!(add_edge_remove_edge(&graph, 2, 1));
        assert!(add_edge_remove_edge(&graph, 2, 2));
    }
}

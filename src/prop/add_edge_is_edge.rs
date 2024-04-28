//! Adding an edge with [`AddEdge`](crate::op::AddEdge) should be reflected by
//! [`IsEdge`](crate::op::IsEdge).

use crate::op::{
    AddEdge,
    IsEdge,
};

/// Adding an edge with [`AddEdge`](crate::op::AddEdge) should be reflected
/// by [`IsEdge`](crate::op::IsEdge).
///
/// Types that implement [`AddEdge`](crate::op::AddEdge) and
/// [`IsEdge`](crate::op::IsEdge) should ensure that this property holds for
/// every `graph`, `s`, and `t` of the given types.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `s`: The source vertex.
/// * `t`: The target vertex.
pub fn add_edge_is_edge<G>(graph: &mut G, s: usize, t: usize) -> bool
where
    G: AddEdge + IsEdge + ?Sized,
{
    graph.add_edge(s, t);

    graph.is_edge(s, t)
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use {
        super::*,
        crate::prop::strategy::binop_vertices,
        alloc::collections::{
            BTreeMap,
            BTreeSet,
        },
        proptest::prelude::*,
        std::collections::{
            HashMap,
            HashSet,
        },
    };

    proptest! {
        #[test]
        fn vec_btree_set((v, s, t) in binop_vertices(1, 10_000)) {
            let mut graph = vec![BTreeSet::new(); v];

            assert!(add_edge_is_edge(&mut graph, s, t));
        }

        #[test]
        fn vec_hash_set((v, s, t) in binop_vertices(1, 10_000)) {
            let mut graph = vec![HashSet::new(); v];

            assert!(add_edge_is_edge(&mut graph, s, t));
        }

        #[test]
        fn slice_btree_set((v, s, t) in binop_vertices(1, 10_000)) {
            let graph = &mut vec![BTreeSet::new(); v][..];

            assert!(add_edge_is_edge(graph, s, t));
        }

        #[test]
        fn slice_hash_set((v, s, t) in binop_vertices(1, 10_000)) {
            let graph = &mut vec![HashSet::new(); v][..];

            assert!(add_edge_is_edge(graph, s, t));
        }

        #[test]
        fn btree_map_btree_set((v, s, t) in binop_vertices(1, 10_000)) {
            let mut graph = (0..v)
                .map(|v| (v, BTreeSet::new()))
                .collect::<BTreeMap<_, _>>();

            assert!(add_edge_is_edge(&mut graph, s, t));
        }

        #[test]
        fn hash_map_hash_set((v, s, t) in binop_vertices(1, 10_000)) {
            let mut graph = (0..v)
                .map(|v| (v, HashSet::new()))
                .collect::<HashMap<_, _>>();

            assert!(add_edge_is_edge(&mut graph, s, t));
        }
    }

    #[test]
    fn arr_btree_set() {
        let graph = &mut [BTreeSet::new(), BTreeSet::new(), BTreeSet::new()];

        assert!(add_edge_is_edge(graph, 0, 0));
        assert!(add_edge_is_edge(graph, 0, 1));
        assert!(add_edge_is_edge(graph, 0, 2));
        assert!(add_edge_is_edge(graph, 1, 0));
        assert!(add_edge_is_edge(graph, 1, 1));
        assert!(add_edge_is_edge(graph, 1, 2));
        assert!(add_edge_is_edge(graph, 2, 0));
        assert!(add_edge_is_edge(graph, 2, 1));
        assert!(add_edge_is_edge(graph, 2, 2));
    }

    #[test]
    fn arr_hash_set() {
        let graph = &mut [HashSet::new(), HashSet::new(), HashSet::new()];

        assert!(add_edge_is_edge(graph, 0, 0));
        assert!(add_edge_is_edge(graph, 0, 1));
        assert!(add_edge_is_edge(graph, 0, 2));
        assert!(add_edge_is_edge(graph, 1, 0));
        assert!(add_edge_is_edge(graph, 1, 1));
        assert!(add_edge_is_edge(graph, 1, 2));
        assert!(add_edge_is_edge(graph, 2, 0));
        assert!(add_edge_is_edge(graph, 2, 1));
        assert!(add_edge_is_edge(graph, 2, 2));
    }

    #[cfg(feature = "adjacency_matrix")]
    #[test]
    fn adjacency_matrix() {
        use crate::repr::AdjacencyMatrix;

        let graph = &mut AdjacencyMatrix::<3>::new();

        assert!(add_edge_is_edge(graph, 0, 0));
        assert!(add_edge_is_edge(graph, 0, 1));
        assert!(add_edge_is_edge(graph, 0, 2));
        assert!(add_edge_is_edge(graph, 1, 0));
        assert!(add_edge_is_edge(graph, 1, 1));
        assert!(add_edge_is_edge(graph, 1, 2));
        assert!(add_edge_is_edge(graph, 2, 0));
        assert!(add_edge_is_edge(graph, 2, 1));
        assert!(add_edge_is_edge(graph, 2, 2));
    }
}

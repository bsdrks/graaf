//! Adding a weighted edge with [`AddWeightedEdge`](crate::op::AddWeightedEdge)
//! should be reflected by [`IsEdge`](crate::op::IsEdge).

use crate::op::{
    AddWeightedEdge,
    IsEdge,
};

/// Returns whether adding a weighted edge with
/// [`AddWeightedEdge`](crate::op::AddWeightedEdge) is reflected by
/// [`IsEdge`](crate::op::IsEdge).
///
/// Types that implement [`AddWeightedEdge`](crate::op::AddWeightedEdge) and
/// [`IsEdge`](crate::op::IsEdge) should ensure that the property holds for
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
    G: AddWeightedEdge<W> + IsEdge + ?Sized,
{
    graph.add_weighted_edge(s, t, w);

    graph.is_edge(s, t)
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
            let mut graph = vec![BTreeMap::new(); v];

            assert!(add_weighted_edge_is_edge(&mut graph, s, t, w));
        }

        #[test]
        fn vec_hash_map((v, s, t) in binop_vertices(1, 100), w in -100..100_i32) {
            let graph: &mut Vec<HashMap<usize, i32>> = &mut vec![HashMap::new(); v];

            assert!(add_weighted_edge_is_edge(graph, s, t, w));
        }

        #[test]
        fn slice_btree_map((v, s, t) in binop_vertices(1, 100), w in -100..100_i32) {
            let graph = &mut vec![BTreeMap::new(); v][..];

            assert!(add_weighted_edge_is_edge(graph, s, t, w));
        }

        #[test]
        fn slice_hash_map((v, s, t) in binop_vertices(1, 100), w in -100..100_i32) {
            let graph = &mut vec![HashMap::new(); v][..];

            assert!(add_weighted_edge_is_edge(graph, s, t, w));
        }

        #[test]
        fn btree_map_btree_map((v, s, t) in binop_vertices(1, 100), w in -100..100_i32) {
            let mut graph = (0..v)
                .map(|v| (v, BTreeMap::new()))
                .collect::<BTreeMap<_, _>>();

            assert!(add_weighted_edge_is_edge(&mut graph, s, t, w));
        }

        #[test]
        fn hash_map_hash_map((v, s, t) in binop_vertices(1, 100), w in -100..100_i32) {
            let mut graph = (0..v)
                .map(|v| (v, HashMap::new()))
                .collect::<HashMap<_, _>>();

            assert!(add_weighted_edge_is_edge(&mut graph, s, t, w));
        }
    }

    #[test]
    fn arr_btree_map() {
        let mut graph = [BTreeMap::new(), BTreeMap::new(), BTreeMap::new()];

        assert!(add_weighted_edge_is_edge(&mut graph, 0, 0, 0));
        assert!(add_weighted_edge_is_edge(&mut graph, 0, 1, 0));
        assert!(add_weighted_edge_is_edge(&mut graph, 0, 2, 0));
        assert!(add_weighted_edge_is_edge(&mut graph, 1, 0, 0));
        assert!(add_weighted_edge_is_edge(&mut graph, 1, 1, 0));
        assert!(add_weighted_edge_is_edge(&mut graph, 1, 2, 0));
        assert!(add_weighted_edge_is_edge(&mut graph, 2, 0, 0));
        assert!(add_weighted_edge_is_edge(&mut graph, 2, 1, 0));
        assert!(add_weighted_edge_is_edge(&mut graph, 2, 2, 0));
    }

    #[test]
    fn arr_hash_map() {
        let mut graph = [HashMap::new(), HashMap::new(), HashMap::new()];

        assert!(add_weighted_edge_is_edge(&mut graph, 0, 0, 0));
        assert!(add_weighted_edge_is_edge(&mut graph, 0, 1, 0));
        assert!(add_weighted_edge_is_edge(&mut graph, 0, 2, 0));
        assert!(add_weighted_edge_is_edge(&mut graph, 1, 0, 0));
        assert!(add_weighted_edge_is_edge(&mut graph, 1, 1, 0));
        assert!(add_weighted_edge_is_edge(&mut graph, 1, 2, 0));
        assert!(add_weighted_edge_is_edge(&mut graph, 2, 0, 0));
        assert!(add_weighted_edge_is_edge(&mut graph, 2, 1, 0));
        assert!(add_weighted_edge_is_edge(&mut graph, 2, 2, 0));
    }
}

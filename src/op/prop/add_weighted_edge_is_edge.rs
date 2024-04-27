//! Adding a weighted edge with [`AddWeightedEdge`](crate::op::AddWeightedEdge)
//! should be reflected by [`IsEdge`](crate::op::IsEdge).

use crate::op::{
    AddWeightedEdge,
    IsEdge,
};

/// Adding a weighted edge with [`AddWeightedEdge`](crate::op::AddWeightedEdge)
/// should be reflected by [`IsEdge`](crate::op::IsEdge).
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
    G: AddWeightedEdge<W> + IsEdge,
{
    graph.add_weighted_edge(s, t, w);

    graph.is_edge(s, t)
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::collections::HashMap,
    };

    macro_rules! add_weighted_edge_is_edge {
        ($graph:expr) => {
            assert!(add_weighted_edge_is_edge($graph, 0, 1, 1));
            assert!(add_weighted_edge_is_edge($graph, 0, 2, 2));
            assert!(add_weighted_edge_is_edge($graph, 1, 0, 3));
            assert!(add_weighted_edge_is_edge($graph, 1, 2, 4));
            assert!(add_weighted_edge_is_edge($graph, 2, 0, 5));
            assert!(add_weighted_edge_is_edge($graph, 2, 1, 6));
        };
    }

    #[test]
    fn vec_hash_map() {
        let graph: &mut Vec<HashMap<usize, i32>> = &mut vec![HashMap::new(); 3];

        add_weighted_edge_is_edge!(graph);
    }
}

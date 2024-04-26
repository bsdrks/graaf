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
    use {
        super::*,
        std::{
            collections::{
                HashMap,
                HashSet,
            },
            hash::RandomState,
        },
    };

    macro_rules! test_add_edge_is_edge {
        ($graph:expr) => {
            assert!(add_edge_is_edge($graph, 0, 1));
            assert!(add_edge_is_edge($graph, 0, 2));
            assert!(add_edge_is_edge($graph, 1, 0));
            assert!(add_edge_is_edge($graph, 1, 2));
            assert!(add_edge_is_edge($graph, 2, 0));
            assert!(add_edge_is_edge($graph, 2, 1));
        };
    }

    #[test]
    fn vec_hash_set() {
        let graph = &mut vec![HashSet::new(), HashSet::new(), HashSet::new()];

        test_add_edge_is_edge!(graph);
    }

    #[test]
    fn slice_hash_set() {
        let graph: &mut [HashSet<usize, RandomState>] =
            &mut [HashSet::new(), HashSet::new(), HashSet::new()];

        test_add_edge_is_edge!(graph);
    }

    #[test]
    fn arr_hash_set() {
        let graph = &mut [HashSet::new(), HashSet::new(), HashSet::new()];

        test_add_edge_is_edge!(graph);
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = &mut HashMap::from([
            (0, HashSet::new()),
            (1, HashSet::new()),
            (2, HashSet::new()),
        ]);

        test_add_edge_is_edge!(graph);
    }

    #[cfg(feature = "adjacency_matrix")]
    #[test]
    fn adjacency_matrix() {
        use crate::repr::AdjacencyMatrix;

        let graph = &mut AdjacencyMatrix::<3>::new();

        test_add_edge_is_edge!(graph);
    }
}

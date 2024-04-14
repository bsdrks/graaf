//! Adding an edge with [`crate::op::AddEdge`] and then removing it with
//! [`crate::op::RemoveEdge`] should keep the graph unchanged.
use crate::op::{
    AddEdge,
    RemoveEdge,
};

/// Adding an edge with [`crate::op::AddEdge`] and then removing it with
/// [`crate::op::RemoveEdge`] should keep the graph unchanged.
///
/// Types that implement [`crate::op::AddEdge`] and [`crate::op::RemoveEdge`]
/// should ensure that the following property holds for every `graph`, `s`, and
/// `t` of the given types.
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
    clone.remove_edge(s, t);

    *graph == clone
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::repr::AdjacencyMatrix,
        std::{
            collections::HashSet,
            hash::RandomState,
        },
    };

    #[test]
    fn vec_hash_set() {
        #[allow(clippy::useless_vec)]
        let graph: Vec<HashSet<usize, RandomState>> =
            vec![HashSet::new(), HashSet::new(), HashSet::new()];

        assert!(add_edge_remove_edge(&graph, 0, 1));
        assert!(add_edge_remove_edge(&graph, 0, 2));
        assert!(add_edge_remove_edge(&graph, 1, 0));
        assert!(add_edge_remove_edge(&graph, 1, 2));
        assert!(add_edge_remove_edge(&graph, 2, 0));
        assert!(add_edge_remove_edge(&graph, 2, 1));
    }

    #[test]
    fn arr_hash_set() {
        let graph = [HashSet::new(), HashSet::new(), HashSet::new()];

        assert!(add_edge_remove_edge(&graph, 0, 1));
        assert!(add_edge_remove_edge(&graph, 0, 2));
        assert!(add_edge_remove_edge(&graph, 1, 0));
        assert!(add_edge_remove_edge(&graph, 1, 2));
        assert!(add_edge_remove_edge(&graph, 2, 0));
        assert!(add_edge_remove_edge(&graph, 2, 1));
    }

    #[test]
    fn adjacency_matrix() {
        let graph = AdjacencyMatrix::<3>::new();

        assert!(add_edge_remove_edge(&graph, 0, 1));
        assert!(add_edge_remove_edge(&graph, 0, 2));
        assert!(add_edge_remove_edge(&graph, 1, 0));
        assert!(add_edge_remove_edge(&graph, 1, 2));
        assert!(add_edge_remove_edge(&graph, 2, 0));
        assert!(add_edge_remove_edge(&graph, 2, 1));
    }
}

//! Adding an edge with [`crate::op::AddEdge`] should be reflected by
//! [`crate::op::IsEdge`].
use crate::op::{
    AddEdge,
    IsEdge,
};

/// Types that implement [`crate::op::AddEdge`] and [`crate::op::IsEdge`] should
/// ensure that the following property holds for every `graph`, `s`, and `t` of
/// the given types.
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
mod test {
    use {
        super::*,
        crate::repr::AdjacencyMatrix,
        std::{
            collections::{
                HashMap,
                HashSet,
            },
            hash::RandomState,
        },
    };

    #[test]
    fn vec_hash_set() {
        type G = [HashSet<usize, RandomState>];

        #[allow(clippy::useless_vec)]
        let graph = &mut vec![
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([0, 1]),
        ];

        assert!(add_edge_is_edge::<G>(graph, 0, 1));
        assert!(add_edge_is_edge::<G>(graph, 0, 2));
        assert!(add_edge_is_edge::<G>(graph, 1, 0));
        assert!(add_edge_is_edge::<G>(graph, 1, 2));
        assert!(add_edge_is_edge::<G>(graph, 2, 0));
        assert!(add_edge_is_edge::<G>(graph, 2, 1));
    }

    #[test]
    fn slice_hash_set() {
        type G = [HashSet<usize, RandomState>];

        let graph: &mut G = &mut [
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([0, 1]),
        ];

        assert!(add_edge_is_edge::<G>(graph, 0, 1));
        assert!(add_edge_is_edge::<G>(graph, 0, 2));
        assert!(add_edge_is_edge::<G>(graph, 1, 0));
        assert!(add_edge_is_edge::<G>(graph, 1, 2));
        assert!(add_edge_is_edge::<G>(graph, 2, 0));
        assert!(add_edge_is_edge::<G>(graph, 2, 1));
    }

    #[test]
    fn arr_hash_set() {
        type G = [HashSet<usize, RandomState>];

        let graph = &mut [
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([0, 1]),
        ];

        assert!(add_edge_is_edge::<G>(graph, 0, 1));
        assert!(add_edge_is_edge::<G>(graph, 0, 2));
        assert!(add_edge_is_edge::<G>(graph, 1, 0));
        assert!(add_edge_is_edge::<G>(graph, 1, 2));
        assert!(add_edge_is_edge::<G>(graph, 2, 0));
        assert!(add_edge_is_edge::<G>(graph, 2, 1));
    }

    #[test]
    fn hash_map_hash_set() {
        type G = HashMap<usize, HashSet<usize, RandomState>>;

        let graph: &mut G = &mut HashMap::from([
            (0, HashSet::from([1, 2])),
            (1, HashSet::from([0])),
            (2, HashSet::from([0, 1])),
        ]);

        assert!(add_edge_is_edge::<G>(graph, 0, 1));
        assert!(add_edge_is_edge::<G>(graph, 0, 2));
        assert!(add_edge_is_edge::<G>(graph, 1, 0));
        assert!(add_edge_is_edge::<G>(graph, 1, 2));
        assert!(add_edge_is_edge::<G>(graph, 2, 0));
        assert!(add_edge_is_edge::<G>(graph, 2, 1));
    }

    #[test]
    fn adjacency_matrix() {
        type G = AdjacencyMatrix<3>;

        let mut graph = AdjacencyMatrix::<3>::new();

        assert!(add_edge_is_edge::<G>(&mut graph, 0, 1));
        assert!(add_edge_is_edge::<G>(&mut graph, 0, 2));
        assert!(add_edge_is_edge::<G>(&mut graph, 1, 0));
        assert!(add_edge_is_edge::<G>(&mut graph, 1, 2));
        assert!(add_edge_is_edge::<G>(&mut graph, 2, 0));
        assert!(add_edge_is_edge::<G>(&mut graph, 2, 1));
    }
}

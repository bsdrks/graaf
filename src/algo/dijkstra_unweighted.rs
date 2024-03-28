use {
    crate::IterEdges,
    std::{
        cmp::Reverse,
        collections::BinaryHeap,
    },
};

/// A trait for representations of unweighted directed graphs that implement
/// Dijkstra's algorithm.
pub trait DijkstraUnweighted<W> {
    /// Dijkstra's algorithm for a directed unweighted graph.
    ///
    /// # Arguments
    ///
    /// * `self`: The graph.
    /// * `step`: A function that calculates the accumulated distance.
    /// * `dist`: The distances from the source vertices.
    /// * `heap`: The vertices to visit.
    fn dijkstra(
        &self,
        step: fn(W) -> W,
        dist: &mut [W],
        heap: &mut BinaryHeap<(Reverse<W>, usize)>,
    );
}

impl<W, T> DijkstraUnweighted<W> for T
where
    W: Copy + Ord,
    T: IterEdges,
{
    fn dijkstra(
        &self,
        step: fn(W) -> W,
        dist: &mut [W],
        heap: &mut BinaryHeap<(Reverse<W>, usize)>,
    ) {
        while let Some((Reverse(w), s)) = heap.pop() {
            let w = step(w);

            for t in self.iter_edges(s) {
                if w >= dist[t] {
                    continue;
                }

                dist[t] = w;
                heap.push((Reverse(w), t));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use {
        super::*,
        crate::{
            AddEdge,
            AdjacencyMatrix,
        },
        std::collections::HashSet,
    };

    #[test]
    fn adjacency_list_arr_vec_empty() {
        let graph: [Vec<usize>; 0] = [];
        let mut dist = Vec::new();
        let mut heap = BinaryHeap::new();
        let () = graph.dijkstra(|w: usize| w + 1, &mut dist, &mut heap);

        assert!(dist.is_empty());
    }

    #[test]
    fn adjacency_list_arr_vec_small() {
        let graph: [Vec<usize>; 4] = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
        let mut dist = vec![0, usize::MAX, usize::MAX, usize::MAX];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
        let () = graph.dijkstra(|w: usize| w + 1, &mut dist, &mut heap);

        assert_eq!(dist, vec![0, 1, 1, 2]);
    }

    #[test]
    fn adjacency_list_arr_hash_set_empty() {
        let graph: [Vec<usize>; 0] = [];
        let mut dist = Vec::new();
        let mut heap = BinaryHeap::new();
        let () = graph.dijkstra(|w: usize| w + 1, &mut dist, &mut heap);

        assert!(dist.is_empty());
    }

    #[test]
    fn adjacency_list_arr_hash_set_small() {
        let graph = [
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];
        let mut dist = vec![0, usize::MAX, usize::MAX, usize::MAX];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
        let () = graph.dijkstra(|w: usize| w + 1, &mut dist, &mut heap);

        assert_eq!(dist, vec![0, 1, 1, 2]);
    }

    #[test]
    fn adjacency_matrix_empty() {
        let graph = AdjacencyMatrix::<0>::new();
        let mut dist = Vec::new();
        let mut heap = BinaryHeap::new();
        let () = graph.dijkstra(|w: usize| w + 1, &mut dist, &mut heap);

        assert!(dist.is_empty());
    }

    #[test]
    fn adjacency_matrix_small() {
        let mut graph = AdjacencyMatrix::<4>::new();

        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 0);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 0);
        graph.add_edge(2, 1);
        graph.add_edge(2, 3);
        graph.add_edge(3, 1);
        graph.add_edge(3, 2);

        let mut dist = vec![0, usize::MAX, usize::MAX, usize::MAX];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
        let () = graph.dijkstra(|w: usize| w + 1, &mut dist, &mut heap);

        assert_eq!(dist, vec![0, 1, 1, 2]);
    }
}

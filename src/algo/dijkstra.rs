use {
    crate::IterEdges,
    std::{
        cmp::Reverse,
        collections::BinaryHeap,
        ops::Add,
    },
};

#[must_use]
pub fn dijkstra<W, G>(
    increment: W,
    graph: &G,
    dist: &mut [W],
    heap: &mut BinaryHeap<(Reverse<W>, usize)>,
) where
    G: IterEdges,
    W: Add<Output = W> + Copy + Ord,
{
    while let Some((Reverse(w), s)) = heap.pop() {
        let w = w + increment;

        for t in graph.iter_edges(s) {
            if w >= dist[t] {
                continue;
            }

            dist[t] = w;
            heap.push((Reverse(w), t));
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
    fn adjacency_list_vec_empty() {
        let graph = [Vec::new(); 0];
        let mut dist = Vec::new();
        let mut heap = BinaryHeap::new();
        let _ = dijkstra(1, &graph, &mut dist, &mut heap);

        assert!(dist.is_empty());
    }

    #[test]
    fn adjacency_list_vec_small() {
        let graph: [Vec<usize>; 4] = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
        let mut dist = vec![0, usize::MAX, usize::MAX, usize::MAX];
        let mut heap = BinaryHeap::new();

        dist[0] = 0;
        heap.push((Reverse(0), 0));

        let _ = dijkstra(1, &graph, &mut dist, &mut heap);

        assert_eq!(dist, vec![0, 1, 1, 2]);
    }

    #[test]
    fn adjacency_list_hash_set_empty() {
        let graph = [HashSet::new(); 0];
        let mut dist = Vec::new();
        let mut heap = BinaryHeap::new();
        let _ = dijkstra(1, &graph, &mut dist, &mut heap);

        assert!(dist.is_empty());
    }

    #[test]
    fn adjacency_list_hash_set_small() {
        let graph = [
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];
        let mut dist = vec![0, usize::MAX, usize::MAX, usize::MAX];
        let mut heap = BinaryHeap::new();

        dist[0] = 0;
        heap.push((Reverse(0), 0));

        let _ = dijkstra(1, &graph, &mut dist, &mut heap);

        assert_eq!(dist, vec![0, 1, 1, 2]);
    }

    #[test]
    fn adjacency_matrix_empty() {
        let graph = AdjacencyMatrix::<0>::new();
        let mut dist = Vec::new();
        let mut heap = BinaryHeap::new();
        let _ = dijkstra(1, &&graph, &mut dist, &mut heap);

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
        let mut heap = BinaryHeap::new();

        dist[0] = 0;
        heap.push((Reverse(0), 0));

        let _ = dijkstra(1, &&graph, &mut dist, &mut heap);

        assert_eq!(dist, vec![0, 1, 1, 2]);
    }
}

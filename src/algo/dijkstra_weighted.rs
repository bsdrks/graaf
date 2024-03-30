use {
    crate::IterWeightedEdges,
    std::{
        cmp::Reverse,
        collections::BinaryHeap,
    },
};

/// A trait for representations of weighted graphs that implement
/// Dijkstra's algorithm.
pub trait DijkstraWeighted<W> {
    /// Dijkstra's algorithm for a weighted graph.
    ///
    /// # Arguments
    ///
    /// * `step`: A function that calculates the accumulated weight.
    /// * `dist`: The distances from the source vertices.
    /// * `heap`: The vertices to visit.
    fn dijkstra(
        &self,
        step: fn(W, W) -> W,
        dist: &mut [W],
        heap: &mut BinaryHeap<(Reverse<W>, usize)>,
    );
}

impl<W, T> DijkstraWeighted<W> for T
where
    T: IterWeightedEdges<W>,
    W: Copy + Ord,
{
    fn dijkstra(
        &self,
        step: fn(W, W) -> W,
        dist: &mut [W],
        heap: &mut BinaryHeap<(Reverse<W>, usize)>,
    ) {
        while let Some((Reverse(acc), s)) = heap.pop() {
            for (t, w) in self.iter_weighted_edges(s) {
                let w = step(acc, w);

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
    use super::*;

    #[test]
    fn shortestpath1() {
        let graph: [Vec<(usize, usize)>; 4] =
            [vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]];

        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        graph.dijkstra(|acc, w| acc + w, &mut dist, &mut heap);

        assert_eq!(dist, [0, 2, 4, usize::MAX]);
    }

    #[test]
    fn crosscountry() {
        let graph: [Vec<(usize, usize)>; 4] = [
            vec![(1, 1), (2, 3), (3, 14)],
            vec![(0, 2), (2, 4), (3, 22)],
            vec![(0, 3), (1, 10), (3, 7)],
            vec![(0, 13), (1, 8), (2, 2)],
        ];

        let mut dist = [usize::MAX, 0, usize::MAX, usize::MAX];
        let mut heap = BinaryHeap::from([(Reverse(0), 1)]);

        graph.dijkstra(|acc, w| acc + w, &mut dist, &mut heap);

        assert_eq!(dist, [2, 0, 4, 11]);
    }

    #[test]
    fn small_graph_1() {
        let graph: [Vec<(usize, usize)>; 9] = [
            vec![(1, 4), (7, 8)],
            vec![(0, 4), (2, 8), (7, 11)],
            vec![(1, 8), (3, 7), (5, 4), (8, 2)],
            vec![(2, 7), (4, 9), (5, 14)],
            vec![(3, 9), (5, 10)],
            vec![(2, 4), (3, 14), (4, 10), (6, 2)],
            vec![(5, 2), (7, 1), (8, 6)],
            vec![(0, 8), (1, 11), (6, 1), (8, 7)],
            vec![(2, 2), (6, 6), (7, 7)],
        ];

        let mut dist = [
            0,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
        ];

        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        graph.dijkstra(|acc, w| acc + w, &mut dist, &mut heap);

        assert_eq!(dist, [0, 4, 12, 19, 21, 11, 9, 8, 14]);
    }
}

//! Dijkstra's algorithm with distances.
//!
//! Dijkstra's algorithm with binary heap finds the shortest path in an
//! arc-weighted digraph.[^1]
//!
//! Runs in **O(v log v + a)** time, where **v** is the number of vertices and
//! **a** is the number of arcs.
//!
//! # Examples
//!
//! ## Single source
//!
//! The path from vertex `0` is red. `t` denotes the distances.
//!
//! ![A digraph and the distances between the source vertex and the other vertices obtained by Dijkstra's algorithm](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dijkstra_dist_1-0.87.4.svg?)
//!
//! ```
//! use {
//!     graaf::{
//!         AddArcWeighted,
//!         AdjacencyListWeighted,
//!         DijkstraDist,
//!         Empty,
//!     },
//!     std::iter::once,
//! };
//!
//! let mut digraph = AdjacencyListWeighted::<usize>::empty(7);
//!
//! digraph.add_arc_weighted(0, 1, 1);
//! digraph.add_arc_weighted(1, 2, 1);
//! digraph.add_arc_weighted(1, 6, 6);
//! digraph.add_arc_weighted(2, 4, 1);
//! digraph.add_arc_weighted(3, 0, 2);
//! digraph.add_arc_weighted(4, 5, 2);
//! digraph.add_arc_weighted(5, 6, 1);
//!
//! let mut dijkstra = DijkstraDist::new(&digraph, once(0));
//!
//! assert!(dijkstra.eq([(0, 0), (1, 1), (2, 2), (4, 3), (5, 5), (6, 6)]));
//! ```
//!
//! ## Multiple sources
//!
//! The path from vertex `0` is red. The path from vertex `3` is blue.
//!
//! ![A digraph and the distances between the source vertices and the other vertices obtained by Dijkstra's algorithm](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dijkstra_dist_multi_source_1-0.87.4.svg?)
//!
//! ```
//! use graaf::{
//!     AddArcWeighted,
//!     AdjacencyListWeighted,
//!     DijkstraDist,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyListWeighted::<usize>::empty(7);
//!
//! digraph.add_arc_weighted(0, 1, 1);
//! digraph.add_arc_weighted(1, 2, 1);
//! digraph.add_arc_weighted(1, 6, 5);
//! digraph.add_arc_weighted(2, 4, 1);
//! digraph.add_arc_weighted(3, 0, 2);
//! digraph.add_arc_weighted(3, 5, 1);
//! digraph.add_arc_weighted(4, 5, 1);
//! digraph.add_arc_weighted(5, 6, 3);
//!
//! let mut dijkstra = DijkstraDist::new(&digraph, [0, 3].into_iter());
//!
//! assert!(dijkstra.eq([
//!     (3, 0),
//!     (0, 0),
//!     (5, 1),
//!     (1, 1),
//!     (2, 2),
//!     (4, 3),
//!     (6, 4),
//! ]));
//! ```
//!
//! [^1]: Edsger Wybe Dijkstra. 1959. A note on two problems in connexion with
//!   graphs. Numer. Math. 1, 1 (December 1959), 269–271.
//!   <https://doi.org/10.1007/BF01386390>

use {
    crate::{
        Order,
        OutNeighborsWeighted,
    },
    core::cmp::Reverse,
    std::collections::BinaryHeap,
};

type Step = (usize, usize);

/// Dijkstra's algorithm with distances.
///
/// # Examples
///
/// ## Single source
///
/// The path from vertex `0` is red. `d` denotes the distances.
///
/// ![A digraph and the distances between the source vertex and the other vertices obtained by Dijkstra's algorithm](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dijkstra_dist_1-0.87.4.svg?)
///
/// ```
/// use {
///     graaf::{
///         AddArcWeighted,
///         AdjacencyListWeighted,
///         DijkstraDist,
///         Empty,
///     },
///     std::iter::once,
/// };
///
/// let mut digraph = AdjacencyListWeighted::<usize>::empty(7);
///
/// digraph.add_arc_weighted(0, 1, 1);
/// digraph.add_arc_weighted(1, 2, 1);
/// digraph.add_arc_weighted(1, 6, 6);
/// digraph.add_arc_weighted(2, 4, 1);
/// digraph.add_arc_weighted(3, 0, 2);
/// digraph.add_arc_weighted(4, 5, 2);
/// digraph.add_arc_weighted(5, 6, 1);
///
/// let mut dijkstra = DijkstraDist::new(&digraph, once(0));
///
/// assert!(dijkstra.eq([(0, 0), (1, 1), (2, 2), (4, 3), (5, 5), (6, 6)]));
/// ```
///
/// ## Multiple sources
///
/// The path from vertex `0` is red. The path from vertex `3` is blue.
///
/// ![A digraph and the distances between the source vertices and the other vertices obtained by Dijkstra's algorithm](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dijkstra_dist_multi_source_1-0.87.4.svg?)
///
/// ```
/// use graaf::{
///     AddArcWeighted,
///     AdjacencyListWeighted,
///     DijkstraDist,
///     Empty,
/// };
///
/// let mut digraph = AdjacencyListWeighted::<usize>::empty(7);
///
/// digraph.add_arc_weighted(0, 1, 1);
/// digraph.add_arc_weighted(1, 2, 1);
/// digraph.add_arc_weighted(1, 6, 5);
/// digraph.add_arc_weighted(2, 4, 1);
/// digraph.add_arc_weighted(3, 0, 2);
/// digraph.add_arc_weighted(3, 5, 1);
/// digraph.add_arc_weighted(4, 5, 1);
/// digraph.add_arc_weighted(5, 6, 3);
///
/// let mut dijkstra = DijkstraDist::new(&digraph, [0, 3].into_iter());
///
/// assert!(dijkstra.eq([
///     (3, 0),
///     (0, 0),
///     (5, 1),
///     (1, 1),
///     (2, 2),
///     (4, 3),
///     (6, 4),
/// ]));
/// ```
///
/// [^1]: Edsger Wybe Dijkstra. 1959. A note on two problems in connexion with
///   graphs. Numer. Math. 1, 1 (December 1959), 269–271.
///   <https://doi.org/10.1007/BF01386390>
#[derive(Clone, Debug)]
pub struct DijkstraDist<'a, D> {
    digraph: &'a D,
    dist: Vec<usize>,
    heap: BinaryHeap<(Reverse<usize>, usize)>,
}

impl<'a, D> DijkstraDist<'a, D>
where
    D: Order,
{
    /// Initialize Dijkstra's algorithm.
    ///
    /// # Arguments
    ///
    /// * `digraph`: The digraph.
    /// * `sources`: The source vertices.
    #[must_use]
    pub fn new<T>(digraph: &'a D, sources: T) -> Self
    where
        T: Iterator<Item = usize> + Clone,
    {
        Self {
            digraph,
            dist: sources.clone().fold(
                vec![usize::MAX; digraph.order()],
                |mut dist, u| {
                    dist[u] = 0;

                    dist
                },
            ),
            heap: sources.map(|u| (Reverse(0), u)).collect(),
        }
    }

    /// Find the distances from the source vertices to all other vertices.
    ///
    /// # Panics
    ///
    /// * Panics if a source vertex isn't in the digraph.
    /// * Panics if a successor vertex isn't in the digraph.
    ///
    /// # Examples
    ///
    /// ## Single source
    ///
    /// The path from vertex `0` is red. The dashed arcs represent the shortest
    /// distances from the source. The gray arcs are not traversed.
    ///
    /// ![A digraph and the distances between the source vertex and the other vertices obtained by Dijkstra's algorithm](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dijkstra_dist_distances_1-0.93.1.svg?)
    ///
    /// ```
    /// use {
    ///     graaf::{
    ///         AddArcWeighted,
    ///         AdjacencyListWeighted,
    ///         DijkstraDist,
    ///         Empty,
    ///     },
    ///     std::iter::once,
    /// };
    ///
    /// let mut digraph = AdjacencyListWeighted::empty(7);
    ///
    /// digraph.add_arc_weighted(0, 1, 1);
    /// digraph.add_arc_weighted(1, 2, 1);
    /// digraph.add_arc_weighted(1, 6, 6);
    /// digraph.add_arc_weighted(2, 4, 1);
    /// digraph.add_arc_weighted(3, 0, 2);
    /// digraph.add_arc_weighted(4, 5, 2);
    /// digraph.add_arc_weighted(5, 6, 1);
    ///
    /// assert!(DijkstraDist::new(&digraph, once(0)).distances().eq(&[
    ///     0,
    ///     1,
    ///     2,
    ///     usize::MAX,
    ///     3,
    ///     5,
    ///     6
    /// ]));
    /// ```
    ///
    /// ## Multiple sources
    ///
    /// The path from vertex `0` is red. The path from vertex `3` is blue. The
    /// dashed arcs represent the shortest distances from the sources. The gray
    /// arcs are not traversed.
    ///
    /// ![A digraph and the distances between the source vertex and the other vertices obtained by Dijkstra's algorithm](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dijkstra_dist_distances_multi_source_1-0.93.1.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AddArcWeighted,
    ///     AdjacencyListWeighted,
    ///     DijkstraDist,
    ///     Empty,
    /// };
    ///
    /// let mut digraph = AdjacencyListWeighted::empty(7);
    ///
    /// digraph.add_arc_weighted(0, 1, 1);
    /// digraph.add_arc_weighted(1, 2, 1);
    /// digraph.add_arc_weighted(1, 6, 6);
    /// digraph.add_arc_weighted(2, 4, 1);
    /// digraph.add_arc_weighted(3, 0, 2);
    /// digraph.add_arc_weighted(3, 5, 1);
    /// digraph.add_arc_weighted(4, 5, 1);
    /// digraph.add_arc_weighted(5, 6, 3);
    ///
    /// assert!(DijkstraDist::new(&digraph, [0, 3].into_iter())
    ///     .distances()
    ///     .eq(&[0, 1, 2, 0, 3, 1, 4]));
    /// ```
    /// Find the distances from the source vertices to all other vertices.
    ///
    /// # Panics
    ///
    /// * Panics if a source vertex isn't in the digraph.
    /// * Panics if a successor vertex isn't in the digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use {
    ///     graaf::{
    ///         AddArcWeighted,
    ///         AdjacencyListWeighted,
    ///         DijkstraDist,
    ///         Empty,
    ///     },
    ///     std::iter::once,
    /// };
    ///
    /// let mut digraph = AdjacencyListWeighted::<usize>::empty(4);
    ///
    /// digraph.add_arc_weighted(0, 1, 2);
    /// digraph.add_arc_weighted(0, 2, 3);
    /// digraph.add_arc_weighted(0, 3, 4);
    /// digraph.add_arc_weighted(1, 2, 5);
    /// digraph.add_arc_weighted(1, 3, 0);
    /// digraph.add_arc_weighted(2, 3, 1);
    ///
    /// let mut dijkstra = DijkstraDist::new(&digraph, once(0));
    ///
    /// assert!(dijkstra.distances().into_iter().eq([0, 2, 3, 2]));
    /// ```
    #[must_use]
    pub fn distances(&mut self) -> Vec<usize>
    where
        D: OutNeighborsWeighted<Weight = usize>,
    {
        self.fold(
            vec![usize::MAX; self.digraph.order()],
            |mut distances, (u, dist)| {
                distances[u] = dist;

                distances
            },
        )
    }
}

impl<'a, D> Iterator for DijkstraDist<'a, D>
where
    D: Order + OutNeighborsWeighted<Weight = usize>,
{
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        let (Reverse(dist), u) = self.heap.pop()?;

        for (v, w) in self.digraph.out_neighbors_weighted(u) {
            let dist = dist + w;

            if dist < self.dist[v] {
                self.dist[v] = dist;
                self.heap.push((Reverse(dist), v));
            }
        }

        if dist == self.dist[u] {
            return Some((u, dist));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::repr::adjacency_list_weighted::fixture::{
            bang_jensen_94_usize,
            bang_jensen_96_usize,
            kattis_bryr_1_usize,
            kattis_bryr_2_usize,
            kattis_bryr_3_usize,
            kattis_crosscountry_usize,
            kattis_shortestpath1_usize,
        },
        std::iter::once,
    };

    #[test]
    fn iter_bang_jensen_94() {
        let digraph = bang_jensen_94_usize();

        assert!(DijkstraDist::new(&digraph, once(0)).eq([
            (0, 0),
            (2, 1),
            (1, 1),
            (5, 2),
            (4, 2),
            (3, 2),
            (6, 3)
        ]));
    }

    #[test]
    fn iter_bang_jensen_96() {
        let digraph = bang_jensen_96_usize();

        assert!(DijkstraDist::new(&digraph, once(0)).eq([
            (0, 0),
            (2, 3),
            (4, 4),
            (1, 5),
            (3, 6),
            (5, 7),
        ]));
    }

    #[test]
    fn iter_kattis_bryr_1() {
        let digraph = kattis_bryr_1_usize();

        assert!(DijkstraDist::new(&digraph, once(0)).eq([
            (0, 0),
            (2, 1),
            (1, 1),
        ]));
    }

    #[test]
    fn iter_kattis_bryr_2() {
        let digraph = kattis_bryr_2_usize();

        assert!(DijkstraDist::new(&digraph, once(0)).eq([
            (0, 0),
            (3, 1),
            (1, 1),
            (4, 2),
            (2, 2),
            (5, 3),
        ]));
    }

    #[test]
    fn iter_kattis_bryr_3() {
        let digraph = kattis_bryr_3_usize();

        assert!(DijkstraDist::new(&digraph, once(0)).eq([
            (0, 0),
            (3, 0),
            (7, 0),
            (5, 0),
            (8, 0),
            (4, 0),
            (1, 0),
            (9, 1),
            (6, 1),
            (2, 1)
        ]));
    }

    #[test]
    fn iter_kattis_crosscountry() {
        let digraph = kattis_crosscountry_usize();

        assert!(DijkstraDist::new(&digraph, once(0)).eq([
            (0, 0),
            (1, 1),
            (2, 3),
            (3, 10)
        ]));
    }

    #[test]
    fn iter_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1_usize();

        assert!(DijkstraDist::new(&digraph, once(0)).eq([
            (0, 0),
            (1, 2),
            (2, 4)
        ]));
    }

    #[test]
    fn distances_bang_jensen_94() {
        let digraph = bang_jensen_94_usize();

        assert!(DijkstraDist::new(&digraph, once(0))
            .distances()
            .iter()
            .eq(&[0, 1, 1, 2, 2, 2, 3]));
    }

    #[test]
    fn distances_bang_jensen_96() {
        let digraph = bang_jensen_96_usize();

        assert!(DijkstraDist::new(&digraph, once(0))
            .distances()
            .iter()
            .eq(&[0, 5, 3, 6, 4, 7]));
    }

    #[test]
    fn distances_kattis_bryr_1() {
        let digraph = kattis_bryr_1_usize();

        assert!(DijkstraDist::new(&digraph, once(0))
            .distances()
            .iter()
            .eq(&[0, 1, 1]));
    }

    #[test]
    fn distances_kattis_bryr_2() {
        let digraph = kattis_bryr_2_usize();

        assert!(DijkstraDist::new(&digraph, once(0))
            .distances()
            .iter()
            .eq(&[0, 1, 2, 1, 2, 3]));
    }

    #[test]
    fn distances_kattis_bryr_3() {
        let digraph = kattis_bryr_3_usize();

        assert!(DijkstraDist::new(&digraph, once(0))
            .distances()
            .iter()
            .eq(&[0, 0, 1, 0, 0, 0, 1, 0, 0, 1]));
    }

    #[test]
    fn distances_kattis_crosscountry() {
        let digraph = kattis_crosscountry_usize();

        assert!(DijkstraDist::new(&digraph, once(0))
            .distances()
            .iter()
            .eq(&[0, 1, 3, 10]));
    }

    #[test]
    fn distances_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1_usize();

        assert!(DijkstraDist::new(&digraph, once(0))
            .distances()
            .iter()
            .eq(&[0, 2, 4, usize::MAX]));
    }
}

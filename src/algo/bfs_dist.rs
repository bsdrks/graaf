//! Breadth-first search with distances.
//!
//! Breadth-first search explores an unweighted digraph's vertices in order of
//! their distance from a source.
//!
//! Runs in **O(v + a)** time, where **v** is the number of vertices and **a**
//! is the number of arcs.
//!
//! # Examples
//!
//! ## Single source
//!
//! The path from vertex `0` is red. `d` denotes the distances.
//!
//! ![BFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_dist_1-0.87.4.svg?)
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     BfsDist,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyList::empty(6);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(1, 4);
//! digraph.add_arc(2, 5);
//! digraph.add_arc(3, 0);
//!
//! assert!(BfsDist::new(&digraph, &[0]).eq([
//!     (0, 0),
//!     (1, 1),
//!     (2, 2),
//!     (4, 2),
//!     (5, 3),
//! ]));
//! ```
//!
//! ## Multiple sources
//!
//! The path from vertex `3` is red. The path from vertex `7` is blue.
//!
//! ![BFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_dist_multi_source_1-0.87.4.svg?)
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     BfsDist,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyList::empty(8);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(1, 4);
//! digraph.add_arc(2, 3);
//! digraph.add_arc(2, 5);
//! digraph.add_arc(2, 6);
//! digraph.add_arc(3, 0);
//! digraph.add_arc(6, 5);
//! digraph.add_arc(6, 7);
//! digraph.add_arc(7, 6);
//!
//! assert!(BfsDist::new(&digraph, &[3, 7]).eq([
//!     (3, 0),
//!     (7, 0),
//!     (0, 1),
//!     (6, 1),
//!     (1, 2),
//!     (5, 2),
//!     (2, 3),
//!     (4, 3),
//! ]));
//! ```

use {
    crate::{
        Order,
        OutNeighbors,
    },
    std::collections::{
        HashSet,
        VecDeque,
    },
};

type Step = (usize, usize);

/// Breadth-first search with distances.
///
/// Breadth-first search explores an unweighted digraph's vertices in order of
/// their distance from a source.
///
/// Runs in **O(v + a)** time, where **v** is the number of vertices and **a**
/// is the number of arcs.
///
/// # Examples
///
/// ## Single source
///
/// The path from vertex `0` is red. `d` denotes the distances.
///
/// ![BFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_dist_1-0.87.4.svg?)
///
/// ```
/// use graaf::{
///     AddArc,
///     AdjacencyList,
///     BfsDist,
///     Empty,
/// };
///
/// let mut digraph = AdjacencyList::empty(6);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(1, 4);
/// digraph.add_arc(2, 5);
/// digraph.add_arc(3, 0);
///
/// assert!(BfsDist::new(&digraph, &[0]).eq([
///     (0, 0),
///     (1, 1),
///     (2, 2),
///     (4, 2),
///     (5, 3)
/// ]));
/// ```
///
/// ## Multiple sources
///
/// The path from vertex `3` is red. The path from vertex `7` is blue.
///
/// ![BFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_dist_multi_source_1-0.87.4.svg?)
///
/// ```
/// use graaf::{
///     AddArc,
///     AdjacencyList,
///     BfsDist,
///     Empty,
/// };
///
/// let mut digraph = AdjacencyList::empty(8);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(1, 4);
/// digraph.add_arc(2, 3);
/// digraph.add_arc(2, 5);
/// digraph.add_arc(2, 6);
/// digraph.add_arc(3, 0);
/// digraph.add_arc(6, 5);
/// digraph.add_arc(6, 7);
/// digraph.add_arc(7, 6);
///
/// assert!(BfsDist::new(&digraph, &[3, 7]).eq([
///     (3, 0),
///     (7, 0),
///     (0, 1),
///     (6, 1),
///     (1, 2),
///     (5, 2),
///     (2, 3),
///     (4, 3)
/// ]));
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BfsDist<'a, D> {
    digraph: &'a D,
    queue: VecDeque<Step>,
    visited: HashSet<usize>,
}

impl<'a, D> BfsDist<'a, D> {
    /// Construct a new breadth-first search.
    ///
    /// # Arguments
    ///
    /// * `digraph`: The digraph.
    /// * `sources`: The source vertices.
    pub fn new<'b, T>(digraph: &'a D, sources: T) -> Self
    where
        T: IntoIterator<Item = &'b usize>,
    {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        for &source in sources {
            queue.push_back((source, 0));

            let _ = visited.insert(source);
        }

        Self {
            digraph,
            queue,
            visited,
        }
    }

    /// Find the distances from the source vertices to all other vertices.
    ///
    /// # Panics
    ///
    /// * Panics if `self.next` panics.
    /// * Panics if a source vertex isn't in `self.digraph`.
    /// * Panics if a successor vertex isn't in `self.digraph`.
    ///
    /// # Examples
    ///
    /// ## Single source
    ///
    /// The path from vertex `0` is red. `d` denotes the distances.
    ///
    /// ![BFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_dist_distances_1-0.91.3.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     BfsDist,
    ///     Empty,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(6);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(1, 4);
    /// digraph.add_arc(2, 5);
    /// digraph.add_arc(3, 0);
    ///
    /// assert!(BfsDist::new(&digraph, &[0]).distances().eq(&[
    ///     0,
    ///     1,
    ///     2,
    ///     usize::MAX,
    ///     2,
    ///     3
    /// ]));
    /// ```
    ///
    /// ## Multiple sources
    ///
    /// The path from vertex `3` is red. The path from vertex `7` is blue.
    ///
    /// ![BFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_dist_distances_multi_source_1-0.91.3.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     BfsDist,
    ///     Empty,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(8);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(1, 4);
    /// digraph.add_arc(2, 3);
    /// digraph.add_arc(2, 5);
    /// digraph.add_arc(2, 6);
    /// digraph.add_arc(3, 0);
    /// digraph.add_arc(6, 5);
    /// digraph.add_arc(6, 7);
    /// digraph.add_arc(7, 6);
    ///
    /// assert!(BfsDist::new(&digraph, &[3, 7])
    ///     .distances()
    ///     .eq(&[1, 2, 3, 0, 3, 2, 1, 0]));
    /// ```
    #[must_use]
    pub fn distances(&mut self) -> Vec<usize>
    where
        D: Order + OutNeighbors,
    {
        let mut distances = vec![usize::MAX; self.digraph.order()];

        for (u, dist) in self {
            distances[u] = dist;
        }

        distances
    }
}

impl<'a, D> Iterator for BfsDist<'a, D>
where
    D: OutNeighbors,
{
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((u, dist)) = self.queue.pop_front() {
            {
                let dist = dist + 1;

                for v in self
                    .digraph
                    .out_neighbors(u)
                    .filter(|v| self.visited.insert(*v))
                {
                    self.queue.push_back((v, dist));
                }
            }

            return Some((u, dist));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            repr::adjacency_list::fixture::{
                bang_jensen_196,
                bang_jensen_34,
                bang_jensen_94,
                kattis_builddeps,
                kattis_cantinaofbabel_1,
                kattis_cantinaofbabel_2,
                kattis_escapewallmaria_1,
                kattis_escapewallmaria_2,
                kattis_escapewallmaria_3,
            },
            AdjacencyList,
            Empty,
        },
    };

    #[test]
    fn distances_trivial() {
        let digraph = AdjacencyList::trivial();

        assert!(BfsDist::new(&digraph, &[0]).distances().eq(&[0]));
    }

    #[test]
    fn distances_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(BfsDist::new(&digraph, &[0])
            .distances()
            .eq(&[0, 1, 1, 2, 2, 2, 3]));
    }

    #[test]
    fn distances_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(BfsDist::new(&digraph, &[5]).distances().eq(&[
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            0,
            1,
            usize::MAX,
            usize::MAX,
            1,
            usize::MAX,
            usize::MAX,
            3,
            2,
            usize::MAX,
            usize::MAX,
        ]));
    }

    #[test]
    fn distances_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(BfsDist::new(&digraph, &[5]).distances().eq(&[
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            0,
            1,
            usize::MAX,
            usize::MAX,
            1,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
        ]));
    }

    #[test]
    fn distances_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(BfsDist::new(&digraph, &[1]).distances().eq(&[
            usize::MAX,
            0,
            1,
            usize::MAX,
            usize::MAX,
            1,
            2,
            usize::MAX,
            usize::MAX,
            2,
            usize::MAX,
            usize::MAX,
            4,
            3,
            usize::MAX,
            usize::MAX,
        ]));
    }

    #[test]
    fn iter_bang_jensen_196() {
        let digraph = bang_jensen_196();

        assert!(BfsDist::new(&digraph, &[0]).eq([
            (0, 0),
            (1, 1),
            (4, 1),
            (7, 1),
            (2, 2),
            (5, 2),
            (3, 3),
            (6, 3)
        ]));
    }

    #[test]
    fn iter_bang_jensen_34() {
        let digraph = bang_jensen_34();

        assert!(BfsDist::new(&digraph, &[0]).eq([(0, 0), (4, 1)]));
    }

    #[test]
    fn iter_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(BfsDist::new(&digraph, &[0]).eq([
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 2),
            (5, 2),
            (6, 3),
        ]));
    }

    #[test]
    fn iter_kattis_builddeps() {
        let digraph = kattis_builddeps();

        assert!(BfsDist::new(&digraph, &[0]).eq([
            (0, 0),
            (3, 1),
            (4, 1),
            (1, 2),
        ]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_1() {
        let digraph = kattis_cantinaofbabel_1();

        assert!(BfsDist::new(&digraph, &[0]).eq([
            (0, 0),
            (1, 1),
            (2, 2),
            (4, 2),
            (3, 3),
            (5, 4),
            (7, 4),
            (10, 4),
            (11, 4),
            (6, 5),
            (9, 5),
        ]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_2() {
        let digraph = kattis_cantinaofbabel_2();

        assert!(BfsDist::new(&digraph, &[0]).eq([
            (0, 0),
            (1, 1),
            (7, 2),
            (2, 3),
            (5, 4),
            (3, 5),
            (6, 5),
            (4, 6),
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(BfsDist::new(&digraph, &[5]).eq([
            (5, 0),
            (6, 1),
            (9, 1),
            (13, 2),
            (12, 3),
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(BfsDist::new(&digraph, &[5]).eq([(5, 0), (6, 1), (9, 1),]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(BfsDist::new(&digraph, &[1]).eq([
            (1, 0),
            (2, 1),
            (5, 1),
            (6, 2),
            (9, 2),
            (13, 3),
            (12, 4),
        ]));
    }
}

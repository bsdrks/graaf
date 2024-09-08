//! Breadth-first search with distances.
//!
//! Breadth-first search explores the vertices of an unweighted digraph in
//! order of their distance from a source.
//!
//! Runs in **O(v + a)** time, where **v** is the number of vertices and **a**
//! is the number of arcs.
//!
//! # Examples
//!
//! ## Single source
//!
//! Red marks the path starting at vertex `0` and `d` denotes the distance.
//!
//! ![BFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_dist_1-0.87.4.svg?)
//!
//! ```
//! use graaf::{
//!     bfs_dist::{
//!         BfsDist,
//!         Step,
//!     },
//!     AddArc,
//!     AdjacencyList,
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
//!     Step { u: 0, dist: 0 },
//!     Step { u: 1, dist: 1 },
//!     Step { u: 2, dist: 2 },
//!     Step { u: 4, dist: 2 },
//!     Step { u: 5, dist: 3 },
//! ]));
//! ```
//!
//! ## Multiple sources
//!
//! Red marks the path starting at vertex `3` and blue the path starting at
//! vertex `7`.
//!
//! ![BFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_dist_multi_source_1-0.87.4.svg?)
//!
//! ```
//! use graaf::{
//!     bfs_dist::{
//!         BfsDist,
//!         Step,
//!     },
//!     AddArc,
//!     AdjacencyList,
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
//!     Step { u: 3, dist: 0 },
//!     Step { u: 7, dist: 0 },
//!     Step { u: 0, dist: 1 },
//!     Step { u: 6, dist: 1 },
//!     Step { u: 1, dist: 2 },
//!     Step { u: 5, dist: 2 },
//!     Step { u: 2, dist: 3 },
//!     Step { u: 4, dist: 3 },
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

/// Breadth-first search with distances.
///
/// Breadth-first search explores the vertices of an unweighted digraph in
/// order of their distance from a source.
///
/// Runs in **O(v + a)** time, where **v** is the number of vertices and **a**
/// is the number of arcs.
///
/// # Examples
///
/// ## Single source
///
/// Red marks the path starting at vertex `0` and `d` denotes the distance.
///
/// ![BFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_dist_1-0.87.4.svg?)
///
/// ```
/// use graaf::{
///     bfs_dist::{
///         BfsDist,
///         Step,
///     },
///     AddArc,
///     AdjacencyList,
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
///     Step { u: 0, dist: 0 },
///     Step { u: 1, dist: 1 },
///     Step { u: 2, dist: 2 },
///     Step { u: 4, dist: 2 },
///     Step { u: 5, dist: 3 },
/// ]));
/// ```
///
/// ## Multiple sources
///
/// Red marks the path starting at vertex `3` and blue the path starting at
/// vertex `7`.
///
/// ![BFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_dist_multi_source_1-0.87.4.svg?)
///
/// ```
/// use graaf::{
///     bfs_dist::{
///         BfsDist,
///         Step,
///     },
///     AddArc,
///     AdjacencyList,
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
///     Step { u: 3, dist: 0 },
///     Step { u: 7, dist: 0 },
///     Step { u: 0, dist: 1 },
///     Step { u: 6, dist: 1 },
///     Step { u: 1, dist: 2 },
///     Step { u: 5, dist: 2 },
///     Step { u: 2, dist: 3 },
///     Step { u: 4, dist: 3 },
/// ]));
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BfsDist<'a, D> {
    digraph: &'a D,
    queue: VecDeque<(usize, usize)>,
    visited: HashSet<usize>,
}

/// A step in the breadth-first search.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Step {
    /// The current vertex.
    pub u: usize,
    /// The distance of `u` from the source vertex.
    pub dist: usize,
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
    /// * Panics if a source vertex is not in `self.digraph`.
    /// * Panics if a successor vertex is not in `self.digraph`.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     bfs_dist::BfsDist,
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(4);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(3, 0);
    ///
    /// assert!(BfsDist::new(&digraph, &[0]).distances().eq(&[
    ///     0,
    ///     1,
    ///     2,
    ///     usize::MAX
    /// ]));
    /// ```
    #[must_use]
    pub fn distances(&mut self) -> Vec<usize>
    where
        D: Order + OutNeighbors,
    {
        let mut distances = vec![usize::MAX; self.digraph.order()];

        for Step { u, dist } in self {
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

            return Some(Step { u, dist });
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
            Step { u: 0, dist: 0 },
            Step { u: 1, dist: 1 },
            Step { u: 4, dist: 1 },
            Step { u: 7, dist: 1 },
            Step { u: 2, dist: 2 },
            Step { u: 5, dist: 2 },
            Step { u: 3, dist: 3 },
            Step { u: 6, dist: 3 }
        ]));
    }

    #[test]
    fn iter_bang_jensen_34() {
        let digraph = bang_jensen_34();

        assert!(BfsDist::new(&digraph, &[0])
            .eq([Step { u: 0, dist: 0 }, Step { u: 4, dist: 1 }]));
    }

    #[test]
    fn iter_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(BfsDist::new(&digraph, &[0]).eq([
            Step { u: 0, dist: 0 },
            Step { u: 1, dist: 1 },
            Step { u: 2, dist: 1 },
            Step { u: 3, dist: 2 },
            Step { u: 4, dist: 2 },
            Step { u: 5, dist: 2 },
            Step { u: 6, dist: 3 }
        ]));
    }

    #[test]
    fn iter_kattis_builddeps() {
        let digraph = kattis_builddeps();

        assert!(BfsDist::new(&digraph, &[0]).eq([
            Step { u: 0, dist: 0 },
            Step { u: 3, dist: 1 },
            Step { u: 4, dist: 1 },
            Step { u: 1, dist: 2 }
        ]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_1() {
        let digraph = kattis_cantinaofbabel_1();

        assert!(BfsDist::new(&digraph, &[0]).eq([
            Step { u: 0, dist: 0 },
            Step { u: 1, dist: 1 },
            Step { u: 2, dist: 2 },
            Step { u: 4, dist: 2 },
            Step { u: 3, dist: 3 },
            Step { u: 5, dist: 4 },
            Step { u: 7, dist: 4 },
            Step { u: 10, dist: 4 },
            Step { u: 11, dist: 4 },
            Step { u: 6, dist: 5 },
            Step { u: 9, dist: 5 }
        ]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_2() {
        let digraph = kattis_cantinaofbabel_2();

        assert!(BfsDist::new(&digraph, &[0]).eq([
            Step { u: 0, dist: 0 },
            Step { u: 1, dist: 1 },
            Step { u: 7, dist: 2 },
            Step { u: 2, dist: 3 },
            Step { u: 5, dist: 4 },
            Step { u: 3, dist: 5 },
            Step { u: 6, dist: 5 },
            Step { u: 4, dist: 6 }
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(BfsDist::new(&digraph, &[5]).eq([
            Step { u: 5, dist: 0 },
            Step { u: 6, dist: 1 },
            Step { u: 9, dist: 1 },
            Step { u: 13, dist: 2 },
            Step { u: 12, dist: 3 }
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(BfsDist::new(&digraph, &[5]).eq([
            Step { u: 5, dist: 0 },
            Step { u: 6, dist: 1 },
            Step { u: 9, dist: 1 }
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(BfsDist::new(&digraph, &[1]).eq([
            Step { u: 1, dist: 0 },
            Step { u: 2, dist: 1 },
            Step { u: 5, dist: 1 },
            Step { u: 6, dist: 2 },
            Step { u: 9, dist: 2 },
            Step { u: 13, dist: 3 },
            Step { u: 12, dist: 4 }
        ]));
    }
}

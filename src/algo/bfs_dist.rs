//! Breadth-first search iterator with distances
//!
//! Breadth-first search explores the vertices of an unweighted digraph in
//! order of their distance from a source. The time complexity is
//! *O*(*v* + *a*).
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     algo::bfs_dist::{
//!         Bfs,
//!         Step,
//!     },
//!     gen::Empty,
//!     op::AddArc,
//! };
//!
//! // 0 -> {1}
//! // 1 -> {2}
//! // 2 -> {}
//! // 3 -> {0}
//!
//! let mut digraph = Digraph::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(3, 0);
//!
//! assert!(Bfs::new(&digraph, &[0]).eq([
//!     Step { u: 0, dist: 0 },
//!     Step { u: 1, dist: 1 },
//!     Step { u: 2, dist: 2 },
//! ]));
//!
//! assert!(Bfs::new(&digraph, &[0]).distances().into_iter().eq([
//!     (0, 0),
//!     (1, 1),
//!     (2, 2),
//! ]));
//! ```

use {
    crate::op::OutNeighbors,
    std::collections::{
        BTreeMap,
        BTreeSet,
        VecDeque,
    },
};

/// Breadth-first search iterator with distances.
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     algo::bfs_dist::{
///         Bfs,
///         Step,
///     },
///     gen::Empty,
///     op::AddArc,
/// };
///
/// // 0 -> {1}
/// // 1 -> {2}
/// // 2 -> {}
/// // 3 -> {0}
///
/// let mut digraph = Digraph::empty(4);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(3, 0);
///
/// assert!(Bfs::new(&digraph, &[0]).eq([
///     Step { u: 0, dist: 0 },
///     Step { u: 1, dist: 1 },
///     Step { u: 2, dist: 2 },
/// ]));
/// ```
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     algo::bfs_dist::{
///         Bfs,
///         Step,
///     },
///     gen::Complete,
/// };
///
/// // 0 -> {1, 2, 3}
/// // 1 -> {0, 2, 3}
/// // 2 -> {0, 1, 3}
/// // 3 -> {0, 1, 2}
///
/// let digraph = Digraph::complete(4);
///
/// assert!(Bfs::new(&digraph, &[0]).eq([
///     Step { u: 0, dist: 0 },
///     Step { u: 1, dist: 1 },
///     Step { u: 2, dist: 1 },
///     Step { u: 3, dist: 1 },
/// ]));
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bfs<'a, D> {
    digraph: &'a D,
    queue: VecDeque<(usize, usize)>,
    visited: BTreeSet<usize>,
}

/// A step in the breadth-first search.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Step {
    /// The current vertex.
    pub u: usize,
    /// The distance of `u` from the source vertex.
    pub dist: usize,
}

impl<'a, D> Bfs<'a, D> {
    /// Constructs a new breadth-first search.
    ///
    /// # Arguments
    ///
    /// * `digraph`: The digraph.
    /// * `sources`: The source vertices.
    pub fn new<'b, T>(digraph: &'a D, sources: T) -> Self
    where
        T: IntoIterator<Item = &'b usize>,
    {
        let mut visited = BTreeSet::new();
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

    /// Finds the distances from the source vertices to all other vertices.
    ///
    /// # Panics
    ///
    /// * Panics if `self.next` panics.
    /// * Panics if a source vertex is not in the digraph.
    /// * Panics if a successor vertex is not in the digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     algo::bfs_dist::Bfs,
    ///     gen::Empty,
    ///     op::AddArc,
    /// };
    ///
    /// // 0 -> {1}
    /// // 1 -> {2}
    /// // 2 -> {}
    /// // 3 -> {0}
    ///
    /// let mut digraph = Digraph::empty(4);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(3, 0);
    ///
    /// assert!(Bfs::new(&digraph, &[0]).distances().into_iter().eq([
    ///     (0, 0),
    ///     (1, 1),
    ///     (2, 2),
    /// ]));
    /// ```
    #[must_use]
    pub fn distances(&mut self) -> BTreeMap<usize, usize>
    where
        D: OutNeighbors,
    {
        self.map(|Step { u, dist }| (u, dist)).collect()
    }
}

impl<'a, D> Iterator for Bfs<'a, D>
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
            adjacency_list::{
                fixture::{
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
                Digraph,
            },
            gen::Empty,
        },
    };

    #[test]
    fn distances_trivial() {
        let digraph = Digraph::trivial();

        assert!(Bfs::new(&digraph, &[0])
            .distances()
            .into_iter()
            .eq([(0, 0)]));
    }

    #[test]
    fn distances_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(Bfs::new(&digraph, &[0]).distances().into_iter().eq([
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 2),
            (5, 2),
            (6, 3)
        ]));
    }

    #[test]
    fn distances_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(Bfs::new(&digraph, &[5]).distances().into_iter().eq([
            (5, 0),
            (6, 1),
            (9, 1),
            (12, 3),
            (13, 2)
        ]));
    }

    #[test]
    fn distances_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(Bfs::new(&digraph, &[5]).distances().into_iter().eq([
            (5, 0),
            (6, 1),
            (9, 1),
        ]));
    }

    #[test]
    fn distances_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(Bfs::new(&digraph, &[1]).distances().into_iter().eq([
            (1, 0),
            (2, 1),
            (5, 1),
            (6, 2),
            (9, 2),
            (12, 4),
            (13, 3)
        ]));
    }

    #[test]
    fn iter_bang_jensen_196() {
        let digraph = bang_jensen_196();

        assert!(Bfs::new(&digraph, &[0]).eq([
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

        assert!(Bfs::new(&digraph, &[0])
            .eq([Step { u: 0, dist: 0 }, Step { u: 4, dist: 1 }]));
    }

    #[test]
    fn iter_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(Bfs::new(&digraph, &[0]).eq([
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

        assert!(Bfs::new(&digraph, &[0]).eq([
            Step { u: 0, dist: 0 },
            Step { u: 3, dist: 1 },
            Step { u: 4, dist: 1 },
            Step { u: 1, dist: 2 }
        ]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_1() {
        let digraph = kattis_cantinaofbabel_1();

        assert!(Bfs::new(&digraph, &[0]).eq([
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

        assert!(Bfs::new(&digraph, &[0]).eq([
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

        assert!(Bfs::new(&digraph, &[5]).eq([
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

        assert!(Bfs::new(&digraph, &[5]).eq([
            Step { u: 5, dist: 0 },
            Step { u: 6, dist: 1 },
            Step { u: 9, dist: 1 }
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(Bfs::new(&digraph, &[1]).eq([
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

//! Breadth-first search iterator with depth
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
//!     algo::bfs_depth::{
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
//!     Step { u: 0, depth: 0 },
//!     Step { u: 1, depth: 1 },
//!     Step { u: 2, depth: 2 },
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

/// Breadth-first search iterator with depth.
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     algo::bfs_depth::{
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
///     Step { u: 0, depth: 0 },
///     Step { u: 1, depth: 1 },
///     Step { u: 2, depth: 2 },
/// ]));
/// ```
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     algo::bfs_depth::{
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
///     Step { u: 0, depth: 0 },
///     Step { u: 1, depth: 1 },
///     Step { u: 2, depth: 1 },
///     Step { u: 3, depth: 1 },
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
    /// The depth of `u` in the breadth-first search tree.
    pub depth: usize,
}

impl<'a, D> Bfs<'a, D> {
    /// Constructs a new breadth-first search.
    ///
    /// # Arguments
    ///
    /// * `digraph`: The digraph.
    /// * `sources`: The source vertices.
    pub fn new(digraph: &'a D, sources: &[usize]) -> Self {
        Self {
            digraph,
            queue: sources.iter().map(|u| (*u, 0)).collect(),
            visited: sources.iter().copied().collect(),
        }
    }

    /// Finds the distances from the source vertices to all other vertices.
    ///
    /// # Panics
    ///
    /// * Panics if `self.next` panics.
    /// * Panics if a source or successor vertex is not in `digraph`.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     algo::bfs_depth::Bfs,
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
        self.map(|Step { u, depth }| (u, depth)).collect()
    }
}

impl<'a, D> Iterator for Bfs<'a, D>
where
    D: OutNeighbors,
{
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((u, depth)) = self.queue.pop_front() {
            {
                let depth = depth + 1;

                for v in self
                    .digraph
                    .out_neighbors(u)
                    .filter(|v| self.visited.insert(*v))
                {
                    self.queue.push_back((v, depth));
                }
            }

            return Some(Step { u, depth });
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
            Step { u: 0, depth: 0 },
            Step { u: 1, depth: 1 },
            Step { u: 4, depth: 1 },
            Step { u: 7, depth: 1 },
            Step { u: 2, depth: 2 },
            Step { u: 5, depth: 2 },
            Step { u: 3, depth: 3 },
            Step { u: 6, depth: 3 }
        ]));
    }

    #[test]
    fn iter_bang_jensen_34() {
        let digraph = bang_jensen_34();

        assert!(Bfs::new(&digraph, &[0])
            .eq([Step { u: 0, depth: 0 }, Step { u: 4, depth: 1 }]));
    }

    #[test]
    fn iter_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(Bfs::new(&digraph, &[0]).eq([
            Step { u: 0, depth: 0 },
            Step { u: 1, depth: 1 },
            Step { u: 2, depth: 1 },
            Step { u: 3, depth: 2 },
            Step { u: 4, depth: 2 },
            Step { u: 5, depth: 2 },
            Step { u: 6, depth: 3 }
        ]));
    }

    #[test]
    fn iter_kattis_builddeps() {
        let digraph = kattis_builddeps();

        assert!(Bfs::new(&digraph, &[0]).eq([
            Step { u: 0, depth: 0 },
            Step { u: 3, depth: 1 },
            Step { u: 4, depth: 1 },
            Step { u: 1, depth: 2 }
        ]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_1() {
        let digraph = kattis_cantinaofbabel_1();

        assert!(Bfs::new(&digraph, &[0]).eq([
            Step { u: 0, depth: 0 },
            Step { u: 1, depth: 1 },
            Step { u: 2, depth: 2 },
            Step { u: 4, depth: 2 },
            Step { u: 3, depth: 3 },
            Step { u: 5, depth: 4 },
            Step { u: 7, depth: 4 },
            Step { u: 10, depth: 4 },
            Step { u: 11, depth: 4 },
            Step { u: 6, depth: 5 },
            Step { u: 9, depth: 5 }
        ]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_2() {
        let digraph = kattis_cantinaofbabel_2();

        assert!(Bfs::new(&digraph, &[0]).eq([
            Step { u: 0, depth: 0 },
            Step { u: 1, depth: 1 },
            Step { u: 7, depth: 2 },
            Step { u: 2, depth: 3 },
            Step { u: 5, depth: 4 },
            Step { u: 3, depth: 5 },
            Step { u: 6, depth: 5 },
            Step { u: 4, depth: 6 }
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(Bfs::new(&digraph, &[5]).eq([
            Step { u: 5, depth: 0 },
            Step { u: 6, depth: 1 },
            Step { u: 9, depth: 1 },
            Step { u: 13, depth: 2 },
            Step { u: 12, depth: 3 }
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(Bfs::new(&digraph, &[5]).eq([
            Step { u: 5, depth: 0 },
            Step { u: 6, depth: 1 },
            Step { u: 9, depth: 1 }
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(Bfs::new(&digraph, &[1]).eq([
            Step { u: 1, depth: 0 },
            Step { u: 2, depth: 1 },
            Step { u: 5, depth: 1 },
            Step { u: 6, depth: 2 },
            Step { u: 9, depth: 2 },
            Step { u: 13, depth: 3 },
            Step { u: 12, depth: 4 }
        ]));
    }
}

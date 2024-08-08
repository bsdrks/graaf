//! Breadth-first search with successors
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
//!     algo::bfs_successors::{
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
//!     Step { u: 0, v: vec![1] },
//!     Step { u: 1, v: vec![2] },
//!     Step { u: 2, v: vec![] },
//! ]));
//!
//! assert!(Bfs::new(&digraph, &[0]).predecessors().into_iter().eq([
//!     None,
//!     Some(0),
//!     Some(1),
//!     None
//! ]));
//! ```

use {
    super::PredecessorTree,
    crate::op::{
        Order,
        OutNeighbors,
    },
    std::collections::{
        BTreeSet,
        VecDeque,
    },
};

/// Breadth-first iterator with successors.
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     algo::bfs_successors::{
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
///     Step { u: 0, v: vec![1] },
///     Step { u: 1, v: vec![2] },
///     Step { u: 2, v: vec![] },
/// ]));
/// ```
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     algo::bfs_successors::{
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
///     Step {
///         u: 0,
///         v: vec![1, 2, 3]
///     },
///     Step { u: 1, v: vec![] },
///     Step { u: 2, v: vec![] },
///     Step { u: 3, v: vec![] },
/// ]));
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bfs<'a, D> {
    digraph: &'a D,
    queue: VecDeque<usize>,
    visited: BTreeSet<usize>,
}

/// A step in the breadth-first search.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Step {
    /// The current vertex.
    pub u: usize,
    /// The successors of `u`.
    pub v: Vec<usize>,
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
            queue: sources.iter().copied().collect(),
            visited: sources.iter().copied().collect(),
        }
    }

    /// Finds the predecessor tree.
    ///
    /// # Panics
    ///
    /// * Panics if `self.next` panics.
    /// * Panics if a source or successor vertex is not in `pred`.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     algo::{
    ///         bfs_successors::Bfs,
    ///         PredecessorTree,
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
    /// assert!(Bfs::new(&digraph, &[0]).predecessors().into_iter().eq([
    ///     None,
    ///     Some(0),
    ///     Some(1),
    ///     None
    /// ]));
    /// ```
    pub fn predecessors(&mut self) -> PredecessorTree
    where
        D: Order + OutNeighbors,
    {
        let mut pred = PredecessorTree::new(self.digraph.order());

        for Step { u, v: successors } in self {
            for v in successors {
                pred[v] = Some(u);
            }
        }

        pred
    }

    /// Finds the shortest path from the source vertex to a target vertex.
    ///
    /// # Arguments
    ///
    /// * `is_target`: The function determining if the vertex is a target.
    ///
    /// # Returns
    ///
    /// If it finds a target vertex, the function returns the shortest path
    /// from the source vertex to this target vertex. Otherwise, it returns
    /// `None`.
    ///
    /// # Panics
    ///
    /// * Panics if `is_target` panics.
    /// * Panics if a source or successor vertex is not in `self.digraph`.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     algo::bfs_successors::Bfs,
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
    /// assert!(Bfs::new(&digraph, &[0])
    ///     .shortest_path(|v| v == 2)
    ///     .unwrap()
    ///     .eq(&[0, 1, 2]));
    /// ```
    #[must_use]
    pub fn shortest_path<P>(&mut self, is_target: P) -> Option<Vec<usize>>
    where
        D: Order + OutNeighbors,
        P: Fn(usize) -> bool,
    {
        let mut pred = PredecessorTree::new(self.digraph.order());

        for Step { u, v: successors } in self.by_ref() {
            if is_target(u) {
                return pred.search_by(u, |_, b| b.is_none()).map(
                    |mut path| {
                        path.reverse();

                        path
                    },
                );
            }

            for v in successors {
                pred[v] = Some(u);

                if is_target(v) {
                    return pred.search_by(v, |_, b| b.is_none()).map(
                        |mut path| {
                            path.reverse();

                            path
                        },
                    );
                }
            }
        }

        None
    }
}

impl<'a, D> Iterator for Bfs<'a, D>
where
    D: OutNeighbors,
{
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(u) = self.queue.pop_front() {
            return Some(Step {
                u,
                v: {
                    self.digraph
                        .out_neighbors(u)
                        .filter(|v| self.visited.insert(*v))
                        .inspect(|&v| {
                            self.queue.push_back(v);
                        })
                        .collect()
                },
            });
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
        std::collections::BTreeSet,
    };

    #[test]
    fn iter_bang_jensen_196() {
        let digraph = bang_jensen_196();

        assert!(Bfs::new(&digraph, &[0]).eq([
            Step {
                u: 0,
                v: vec![1, 4, 7]
            },
            Step { u: 1, v: vec![2] },
            Step { u: 4, v: vec![] },
            Step { u: 7, v: vec![5] },
            Step { u: 2, v: vec![3] },
            Step { u: 5, v: vec![6] },
            Step { u: 3, v: vec![] },
            Step { u: 6, v: vec![] },
        ]));
    }

    #[test]
    fn iter_bang_jensen_34() {
        let digraph = bang_jensen_34();

        assert!(Bfs::new(&digraph, &[0])
            .eq([Step { u: 0, v: vec![4] }, Step { u: 4, v: vec![] }]));
    }

    #[test]
    fn iter_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(Bfs::new(&digraph, &[0]).eq([
            Step {
                u: 0,
                v: vec![1, 2]
            },
            Step { u: 1, v: vec![3] },
            Step {
                u: 2,
                v: vec![4, 5]
            },
            Step { u: 3, v: vec![] },
            Step { u: 4, v: vec![6] },
            Step { u: 5, v: vec![] },
            Step { u: 6, v: vec![] },
        ]));
    }

    #[test]
    fn iter_kattis_builddeps() {
        let digraph = kattis_builddeps();

        assert!(Bfs::new(&digraph, &[0]).eq([
            Step {
                u: 0,
                v: vec![3, 4]
            },
            Step { u: 3, v: vec![1] },
            Step { u: 4, v: vec![] },
            Step { u: 1, v: vec![] },
        ]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_1() {
        let digraph = kattis_cantinaofbabel_1();

        assert!(Bfs::new(&digraph, &[0]).eq([
            Step { u: 0, v: vec![1] },
            Step {
                u: 1,
                v: vec![2, 4]
            },
            Step { u: 2, v: vec![] },
            Step { u: 4, v: vec![3] },
            Step {
                u: 3,
                v: vec![5, 7, 10, 11]
            },
            Step { u: 5, v: vec![6] },
            Step { u: 7, v: vec![] },
            Step { u: 10, v: vec![] },
            Step { u: 11, v: vec![9] },
            Step { u: 6, v: vec![] },
            Step { u: 9, v: vec![] },
        ]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_2() {
        let digraph = kattis_cantinaofbabel_2();

        assert!(Bfs::new(&digraph, &[0]).eq([
            Step { u: 0, v: vec![1] },
            Step { u: 1, v: vec![7] },
            Step { u: 7, v: vec![2] },
            Step { u: 2, v: vec![5] },
            Step {
                u: 5,
                v: vec![3, 6]
            },
            Step { u: 3, v: vec![4] },
            Step { u: 6, v: vec![] },
            Step { u: 4, v: vec![] },
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(Bfs::new(&digraph, &[5]).eq([
            Step {
                u: 5,
                v: vec![6, 9]
            },
            Step { u: 6, v: vec![] },
            Step { u: 9, v: vec![13] },
            Step { u: 13, v: vec![12] },
            Step { u: 12, v: vec![] },
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(Bfs::new(&digraph, &[5]).eq([
            Step {
                u: 5,
                v: vec![6, 9]
            },
            Step { u: 6, v: vec![] },
            Step { u: 9, v: vec![] },
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(Bfs::new(&digraph, &[1]).eq([
            Step {
                u: 1,
                v: vec![2, 5]
            },
            Step { u: 2, v: vec![6] },
            Step { u: 5, v: vec![9] },
            Step { u: 6, v: vec![] },
            Step { u: 9, v: vec![13] },
            Step { u: 13, v: vec![12] },
            Step { u: 12, v: vec![] },
        ]));
    }

    #[test]
    fn predecessors_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(Bfs::new(&digraph, &[0]).predecessors().into_iter().eq([
            None,
            Some(0),
            Some(0),
            Some(1),
            Some(2),
            Some(2),
            Some(4)
        ]));
    }

    #[test]
    fn predecessors_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(Bfs::new(&digraph, &[5]).predecessors().into_iter().eq([
            None,
            None,
            None,
            None,
            None,
            None,
            Some(5),
            None,
            None,
            Some(5),
            None,
            None,
            Some(13),
            Some(9),
            None,
            None,
        ]));
    }

    #[test]
    fn predecessors_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(Bfs::new(&digraph, &[5]).predecessors().into_iter().eq([
            None,
            None,
            None,
            None,
            None,
            None,
            Some(5),
            None,
            None,
            Some(5),
            None,
            None,
            None,
            None,
            None,
            None
        ]));
    }

    #[test]
    fn predecessors_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(Bfs::new(&digraph, &[1]).predecessors().into_iter().eq([
            None,
            None,
            Some(1),
            None,
            None,
            Some(1),
            Some(2),
            None,
            None,
            Some(5),
            None,
            None,
            Some(13),
            Some(9),
            None,
            None,
        ]));
    }

    #[test]
    fn predecessors_trivial() {
        let digraph = Digraph::trivial();

        assert!(Bfs::new(&digraph, &[0])
            .predecessors()
            .into_iter()
            .eq([None]));
    }

    #[test]
    fn shortest_path_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(Bfs::new(&digraph, &[0])
            .shortest_path(|t| t == 6)
            .unwrap()
            .eq(&[0, 2, 4, 6]));
    }

    #[test]
    fn shortest_path_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();
        let border = BTreeSet::from([0, 1, 2, 3, 4, 7, 8, 11, 12, 13, 14, 15]);

        assert!(Bfs::new(&digraph, &[5])
            .shortest_path(|t| border.contains(&t))
            .unwrap()
            .eq(&[5, 9, 13]));
    }

    #[test]
    fn shortest_path_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();
        let border = BTreeSet::from([0, 1, 2, 3, 4, 7, 8, 11, 12, 13, 14, 15]);

        assert_eq!(
            Bfs::new(&digraph, &[5]).shortest_path(|t| border.contains(&t)),
            None
        );
    }

    #[test]
    fn shortest_path_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();
        let border = BTreeSet::from([0, 1, 2, 3, 4, 7, 8, 11, 12, 13, 14, 15]);

        assert!(Bfs::new(&digraph, &[1])
            .shortest_path(|t| border.contains(&t))
            .unwrap()
            .eq(&[1]));
    }

    #[test]
    fn shortest_path_trivial() {
        let digraph = Digraph::trivial();

        assert!(Bfs::new(&digraph, &[0])
            .shortest_path(|t| t == 0)
            .unwrap()
            .eq(&[0]));
    }
}

//! Breadth-first search
//!
//! Breadth-first search explores the vertices of an unweighted digraph in
//! order of their distance from a source. The time complexity is
//! *O*(*v* + *a*).
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::{
//!         adjacency_list::Digraph,
//!         algo::{
//!             bfs::Step,
//!             Bfs,
//!         },
//!         gen::Empty,
//!         op::AddArc,
//!     },
//!     std::collections::{
//!         BTreeSet,
//!         VecDeque,
//!     },
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
//! let mut bfs = Bfs::new(
//!     &digraph,
//!     VecDeque::from(vec![(0, 0)]),
//!     |w| w + 1,
//!     BTreeSet::from([0]),
//! );
//!
//! assert_eq!(
//!     bfs.next(),
//!     Some(Step {
//!         out_neighbors: vec![1],
//!         u: 0,
//!         w: 0,
//!         w_next: 1,
//!     })
//! );
//!
//! assert_eq!(
//!     bfs.next(),
//!     Some(Step {
//!         out_neighbors: vec![2],
//!         u: 1,
//!         w: 1,
//!         w_next: 2,
//!     })
//! );
//!
//! assert_eq!(
//!     bfs.next(),
//!     Some(Step {
//!         out_neighbors: vec![],
//!         u: 2,
//!         w: 2,
//!         w_next: 3,
//!     })
//! );
//!
//! assert_eq!(bfs.next(), None);
//!
//! let mut bfs = Bfs::new(
//!     &digraph,
//!     VecDeque::from([(0, 0)]),
//!     |w| w + 1,
//!     BTreeSet::from([0]),
//! );
//!
//! assert!(bfs.distances().into_iter().eq([(0, 0), (1, 1), (2, 2)]));
//! ```

use {
    super::BreadthFirstTree,
    crate::op::{
        Order,
        OutNeighbors,
    },
    std::collections::{
        BTreeMap,
        BTreeSet,
        VecDeque,
    },
};

/// Breadth-first search.
///
/// # Examples
///
/// ```
/// use {
///     graaf::{
///         adjacency_list::Digraph,
///         algo::{
///             bfs::Step,
///             Bfs,
///         },
///         gen::Empty,
///         op::AddArc,
///     },
///     std::collections::{
///         BTreeSet,
///         VecDeque,
///     },
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
/// let mut bfs = Bfs::new(
///     &digraph,
///     VecDeque::from(vec![(0, 0)]),
///     |w| w + 1,
///     BTreeSet::from([0]),
/// );
///
/// assert_eq!(
///     bfs.next(),
///     Some(Step {
///         out_neighbors: vec![1],
///         u: 0,
///         w: 0,
///         w_next: 1,
///     })
/// );
///
/// assert_eq!(
///     bfs.next(),
///     Some(Step {
///         out_neighbors: vec![2],
///         u: 1,
///         w: 1,
///         w_next: 2,
///     })
/// );
///
/// assert_eq!(
///     bfs.next(),
///     Some(Step {
///         out_neighbors: vec![],
///         u: 2,
///         w: 2,
///         w_next: 3,
///     })
/// );
///
/// assert_eq!(bfs.next(), None);
/// ```
///
/// ```
/// use {
///     graaf::{
///         adjacency_list::Digraph,
///         algo::{
///             bfs::Step,
///             Bfs,
///         },
///         gen::Complete,
///     },
///     std::collections::{
///         BTreeSet,
///         VecDeque,
///     },
/// };
///
/// // 0 -> {1, 2, 3}
/// // 1 -> {0, 2, 3}
/// // 2 -> {0, 1, 3}
/// // 3 -> {0, 1, 2}
///
/// let digraph = Digraph::complete(4);
///
/// let mut bfs = Bfs::new(
///     &digraph,
///     VecDeque::from(vec![(0, 0)]),
///     |w| w + 1,
///     BTreeSet::from([0]),
/// );
///
/// assert_eq!(
///     bfs.next(),
///     Some(Step {
///         out_neighbors: vec![1, 2, 3],
///         u: 0,
///         w: 0,
///         w_next: 1,
///     })
/// );
///
/// assert_eq!(
///     bfs.next(),
///     Some(Step {
///         out_neighbors: vec![],
///         u: 1,
///         w: 1,
///         w_next: 2,
///     })
/// );
///
/// assert_eq!(
///     bfs.next(),
///     Some(Step {
///         out_neighbors: vec![],
///         u: 2,
///         w: 1,
///         w_next: 2,
///     })
/// );
///
/// assert_eq!(
///     bfs.next(),
///     Some(Step {
///         out_neighbors: vec![],
///         u: 3,
///         w: 1,
///         w_next: 2,
///     })
/// );
///
/// assert_eq!(bfs.next(), None);
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bfs<'a, D, F, W> {
    digraph: &'a D,
    queue: VecDeque<(usize, W)>,
    update: F,
    visited: BTreeSet<usize>,
}

/// A step in the breadth-first search.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Step<W> {
    /// The out neighbors of `u` that will be visited next.
    pub out_neighbors: Vec<usize>,
    /// The current tail vertex.
    pub u: usize,
    /// The accumulated distance to `u`.
    pub w: W,
    /// The accumulated distance to the out neighbors of `u`.
    pub w_next: W,
}

impl<'a, D, F, W> Iterator for Bfs<'a, D, F, W>
where
    D: OutNeighbors,
    F: Fn(W) -> W,
    W: Copy + Ord,
{
    type Item = Step<W>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr) = self.queue.pop_front() {
            let (u, w) = curr;
            let w_next = (self.update)(w);

            return Some(Step {
                out_neighbors: self
                    .digraph
                    .out_neighbors(u)
                    .filter(|v| self.visited.insert(*v))
                    .inspect(|&v| {
                        self.queue.push_back((v, w_next));
                    })
                    .collect(),
                u,
                w,
                w_next,
            });
        }

        None
    }
}

impl<'a, D, F, W> Bfs<'a, D, F, W>
where
    D: Order + OutNeighbors,
    F: Fn(W) -> W,
    W: Copy + Ord,
{
    /// Constructs a new breadth-first search.
    ///
    /// # Arguments
    ///
    /// * `digraph`: The digraph.
    /// * `queue`: The source vertices.
    /// * `update`: The function that calculates the accumulated distance.
    /// * `visited`: The set of visited vertices.
    pub const fn new(
        digraph: &'a D,
        queue: VecDeque<(usize, W)>,
        update: F,
        visited: BTreeSet<usize>,
    ) -> Self {
        Self {
            digraph,
            queue,
            update,
            visited,
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
    /// use {
    ///     graaf::{
    ///         adjacency_list::Digraph,
    ///         algo::Bfs,
    ///         gen::Empty,
    ///         op::AddArc,
    ///     },
    ///     std::collections::{
    ///         BTreeSet,
    ///         VecDeque,
    ///     },
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
    /// let mut bfs = Bfs::new(
    ///     &digraph,
    ///     VecDeque::from(vec![(0, 0)]),
    ///     |w| w + 1,
    ///     BTreeSet::from([0]),
    /// );
    ///
    /// assert!(bfs.distances().into_iter().eq([(0, 0), (1, 1), (2, 2)]));
    /// ```
    #[must_use]
    pub fn distances(&mut self) -> BTreeMap<usize, W> {
        let mut dist = BTreeMap::new();

        for Step { u, w, .. } in self {
            let _ = dist.insert(u, w);
        }

        dist
    }

    /// Finds the breadth-first tree.
    ///
    /// # Panics
    ///
    /// * Panics if `self.next` panics.
    /// * Panics if a source or successor vertex is not in `pred`.
    ///
    /// # Examples
    ///
    /// ```
    /// use {
    ///     graaf::{
    ///         adjacency_list::Digraph,
    ///         algo::{
    ///             Bfs,
    ///             BreadthFirstTree,
    ///         },
    ///         gen::Empty,
    ///         op::AddArc,
    ///     },
    ///     std::collections::{
    ///         BTreeSet,
    ///         VecDeque,
    ///     },
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
    /// let mut bfs = Bfs::new(
    ///     &digraph,
    ///     VecDeque::from(vec![(0, 0)]),
    ///     |w| w + 1,
    ///     BTreeSet::from([0]),
    /// );
    ///
    /// let pred = bfs.predecessors();
    ///
    /// assert!(pred.into_iter().eq([None, Some(0), Some(1), None]));
    /// ```
    pub fn predecessors(&mut self) -> BreadthFirstTree {
        let mut pred = BreadthFirstTree::new(self.digraph.order());

        for Step {
            u, out_neighbors, ..
        } in self
        {
            for v in out_neighbors {
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
    /// use {
    ///     graaf::{
    ///         adjacency_list::Digraph,
    ///         algo::Bfs,
    ///         gen::Empty,
    ///         op::AddArc,
    ///     },
    ///     std::collections::{
    ///         BTreeSet,
    ///         VecDeque,
    ///     },
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
    /// let mut bfs = Bfs::new(
    ///     &digraph,
    ///     VecDeque::from(vec![(0, 0)]),
    ///     |w| w + 1,
    ///     BTreeSet::from([0]),
    /// );
    ///
    /// assert!(bfs.shortest_path(|v| v == 2).unwrap().eq(&[0, 1, 2]));
    /// ```
    #[must_use]
    pub fn shortest_path<P>(&mut self, is_target: P) -> Option<Vec<usize>>
    where
        P: Fn(usize) -> bool,
    {
        let mut pred = BreadthFirstTree::new(self.digraph.order());

        for Step {
            u, out_neighbors, ..
        } in self.by_ref()
        {
            if is_target(u) {
                return pred.search_by(u, |_, b| b.is_none()).map(
                    |mut path| {
                        path.reverse();

                        path
                    },
                );
            }

            for v in out_neighbors {
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

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            adjacency_list::{
                fixture::{
                    bang_jensen_94,
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
    fn distances_trivial() {
        let digraph = Digraph::trivial();

        let mut bfs = Bfs::new(
            &digraph,
            VecDeque::new(),
            |w: usize| w + 1,
            BTreeSet::new(),
        );

        assert!(bfs.distances().is_empty());
    }

    #[test]
    fn distances_bang_jensen_94() {
        let digraph = bang_jensen_94();

        let mut bfs = Bfs::new(
            &digraph,
            VecDeque::from([(0, 0)]),
            |w| w + 1,
            BTreeSet::from([0]),
        );

        assert!(bfs.distances().into_iter().eq([
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

        let mut bfs = Bfs::new(
            &digraph,
            VecDeque::from([(5, 0)]),
            |w| w + 1,
            BTreeSet::from([5]),
        );

        assert!(bfs.distances().into_iter().eq([
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

        let mut bfs = Bfs::new(
            &digraph,
            VecDeque::from([(5, 0)]),
            |w| w + 1,
            BTreeSet::from([5]),
        );

        assert!(bfs.distances().into_iter().eq([(5, 0), (6, 1), (9, 1),]));
    }

    #[test]
    fn distances_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        let mut bfs = Bfs::new(
            &digraph,
            VecDeque::from([(1, 0)]),
            |w| w + 1,
            BTreeSet::from([1]),
        );

        assert!(bfs.distances().into_iter().eq([
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
    fn predecessors_trivial() {
        let digraph = Digraph::trivial();

        let mut bfs = Bfs::new(
            &digraph,
            VecDeque::new(),
            |w: usize| w + 1,
            BTreeSet::new(),
        );

        assert!(bfs.predecessors().into_iter().eq([None]));
    }

    #[test]
    fn predecessors_bang_jensen_94() {
        let digraph = bang_jensen_94();

        let mut bfs = Bfs::new(
            &digraph,
            VecDeque::from([(0, 0)]),
            |w| w + 1,
            BTreeSet::from([0]),
        );

        assert!(bfs.predecessors().into_iter().eq([
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

        let mut bfs = Bfs::new(
            &digraph,
            VecDeque::from([(5, 0)]),
            |w| w + 1,
            BTreeSet::from([5]),
        );

        assert!(bfs.predecessors().into_iter().eq([
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

        let mut bfs = Bfs::new(
            &digraph,
            VecDeque::from([(5, 0)]),
            |w| w + 1,
            BTreeSet::from([5]),
        );

        assert!(bfs.predecessors().into_iter().eq([
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

        let mut bfs = Bfs::new(
            &digraph,
            VecDeque::from([(1, 0)]),
            |w| w + 1,
            BTreeSet::from([1]),
        );

        assert!(bfs.predecessors().into_iter().eq([
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
    fn shortest_path_trivial() {
        let digraph = Digraph::trivial();

        let mut bfs = Bfs::new(
            &digraph,
            VecDeque::new(),
            |w: usize| w + 1,
            BTreeSet::new(),
        );

        assert!(bfs.shortest_path(|t| t == 0).is_none());
    }

    #[test]
    fn shortest_path_bang_jensen_94() {
        let digraph = bang_jensen_94();

        let mut bfs = Bfs::new(
            &digraph,
            VecDeque::from([(0, 0)]),
            |w| w + 1,
            BTreeSet::from([0]),
        );

        assert!(bfs.shortest_path(|t| t == 6).unwrap().eq(&[0, 2, 4, 6]));
    }

    #[test]
    fn shortest_path_kattis_escapewallmaria_1() {
        let border = BTreeSet::from([0, 1, 2, 3, 4, 7, 8, 11, 12, 13, 14, 15]);
        let digraph = kattis_escapewallmaria_1();

        let mut bfs = Bfs::new(
            &digraph,
            VecDeque::from([(5, 0)]),
            |w| w + 1,
            BTreeSet::from([5]),
        );

        assert!(bfs
            .shortest_path(|t| border.contains(&t))
            .unwrap()
            .eq(&[5, 9, 13]));
    }

    #[test]
    fn shortest_path_kattis_escapewallmaria_2() {
        let border = BTreeSet::from([0, 1, 2, 3, 4, 7, 8, 11, 12, 13, 14, 15]);
        let digraph = kattis_escapewallmaria_2();

        let mut bfs = Bfs::new(
            &digraph,
            VecDeque::from([(5, 0)]),
            |w| w + 1,
            BTreeSet::from([5]),
        );

        assert_eq!(bfs.shortest_path(|t| border.contains(&t)), None);
    }

    #[test]
    fn shortest_path_kattis_escapewallmaria_3() {
        let border = BTreeSet::from([0, 1, 2, 3, 4, 7, 8, 11, 12, 13, 14, 15]);
        let digraph = kattis_escapewallmaria_3();

        let mut bfs = Bfs::new(
            &digraph,
            VecDeque::from([(1, 0)]),
            |w| w + 1,
            BTreeSet::from([1]),
        );

        assert!(bfs.shortest_path(|t| border.contains(&t)).unwrap().eq(&[1]));
    }
}

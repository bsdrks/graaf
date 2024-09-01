//! Breadth-first search: vertices and their predecessors.
//!
//! Breadth-first search explores the vertices of an unweighted digraph in
//! order of their distance from a source. The time complexity is
//! *O*(*v* + *a*).
//!
//! # Examples
//!
//! ## Single source
//!
//! Red marks the path starting at vertex `0`, with `p` denoting the
//! predecessor. Note that, in the digraph, vertex `3` preceeds vertex `0`, but
//! the BFS algorithm starts at vertex `0`.
//!
//! ![BFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_pred_1.svg?)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     algo::bfs_pred::{
//!         BfsPred,
//!         Step,
//!     },
//!     gen::Empty,
//!     op::AddArc,
//! };
//!
//! let mut digraph = Digraph::empty(6);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(1, 4);
//! digraph.add_arc(2, 5);
//! digraph.add_arc(3, 0);
//!
//! assert!(BfsPred::new(&digraph, &[0]).eq([
//!     Step { u: None, v: 0 },
//!     Step { u: Some(0), v: 1 },
//!     Step { u: Some(1), v: 2 },
//!     Step { u: Some(1), v: 4 },
//!     Step { u: Some(2), v: 5 },
//! ]));
//! ```
//!
//! ## Multiple sources
//!
//! Red marks the path starting at vertex `3` and blue the path starting at
//! vertex `7`.
//!
//! ![BFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_pred_multi_source_1.svg?)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     algo::bfs_pred::{
//!         BfsPred,
//!         Step,
//!     },
//!     gen::Empty,
//!     op::AddArc,
//! };
//!
//! let mut digraph = Digraph::empty(8);
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
//! assert!(BfsPred::new(&digraph, &[3, 7]).eq([
//!     Step { u: None, v: 3 },
//!     Step { u: None, v: 7 },
//!     Step { u: Some(3), v: 0 },
//!     Step { u: Some(7), v: 6 },
//!     Step { u: Some(0), v: 1 },
//!     Step { u: Some(6), v: 5 },
//!     Step { u: Some(1), v: 2 },
//!     Step { u: Some(1), v: 4 },
//! ]));
//! ```

use {
    super::PredecessorTree,
    crate::op::{
        Order,
        OutNeighbors,
    },
    std::collections::{
        HashSet,
        VecDeque,
    },
};

/// A step in the breadth-first search.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Step {
    /// The predecessor vertex of the current vertex, if any. The source
    /// vertices have no predecessor.
    pub u: Option<usize>,
    /// The current vertex.
    pub v: usize,
}

/// Breadth-first search: vertices and their predecessors.
///
/// # Examples
///
/// ## Single source
///
/// Red marks the path starting at vertex `0`, with `p` denoting the
/// predecessor. Note that, in the digraph, vertex `3` preceeds vertex `0`, but
/// the BFS algorithm starts at vertex `0`.
///
/// ![BFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_pred_1.svg?)
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     algo::bfs_pred::{
///         BfsPred,
///         Step,
///     },
///     gen::Empty,
///     op::AddArc,
/// };
///
/// let mut digraph = Digraph::empty(6);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(1, 4);
/// digraph.add_arc(2, 5);
/// digraph.add_arc(3, 0);
///
/// assert!(BfsPred::new(&digraph, &[0]).eq([
///     Step { u: None, v: 0 },
///     Step { u: Some(0), v: 1 },
///     Step { u: Some(1), v: 2 },
///     Step { u: Some(1), v: 4 },
///     Step { u: Some(2), v: 5 },
/// ]));
/// ```
///
/// ## Multiple sources
///
/// Red marks the path starting at vertex `3` and blue the path starting at
/// vertex `7`.
///
/// ![BFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_pred_multi_source_1.svg?)
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     algo::bfs_pred::{
///         BfsPred,
///         Step,
///     },
///     gen::Empty,
///     op::AddArc,
/// };
///
/// let mut digraph = Digraph::empty(8);
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
/// assert!(BfsPred::new(&digraph, &[3, 7]).eq([
///     Step { u: None, v: 3 },
///     Step { u: None, v: 7 },
///     Step { u: Some(3), v: 0 },
///     Step { u: Some(7), v: 6 },
///     Step { u: Some(0), v: 1 },
///     Step { u: Some(6), v: 5 },
///     Step { u: Some(1), v: 2 },
///     Step { u: Some(1), v: 4 },
/// ]));
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BfsPred<'a, D> {
    digraph: &'a D,
    queue: VecDeque<Step>,
    visited: HashSet<usize>,
}

impl<'a, D> BfsPred<'a, D> {
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
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        for &source in sources {
            queue.push_back(Step { u: None, v: source });
            let _ = visited.insert(source);
        }

        Self {
            digraph,
            queue,
            visited,
        }
    }

    /// Finds the predecessor tree.
    ///
    /// # Panics
    ///
    /// * Panics if `self.next` panics.
    /// * Panics if a source vertex is not in the digraph.
    /// * Panics if a successor vertex is not in the digraph.
    ///
    /// # Examples
    ///
    /// ## Single source
    ///
    /// Red marks the traversal starting at vertex `0` and the dashed arcs mark
    /// the predecessor tree.
    ///
    /// ![BFS and the predecessor tree](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_pred_predecessors_1.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     algo::{
    ///         bfs_pred::BfsPred,
    ///         PredecessorTree,
    ///     },
    ///     gen::Empty,
    ///     op::AddArc,
    /// };
    ///
    /// let mut digraph = Digraph::empty(6);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(1, 4);
    /// digraph.add_arc(2, 5);
    ///
    /// assert!(BfsPred::new(&digraph, &[0]).predecessors().into_iter().eq([
    ///     None,
    ///     Some(0),
    ///     Some(1),
    ///     None,
    ///     Some(1),
    ///     Some(2),
    /// ]));
    /// ```
    ///
    /// ## Multiple sources
    ///
    /// Red marks the traversal starting at vertex `3` and blue marks the
    /// traversal starting at vertex `7`. The dashed arcs mark the predecessor
    /// tree.
    ///
    /// ![BFS and the predecessor tree](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_pred_predecessors_multi_source_1.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     algo::{
    ///         bfs_pred::BfsPred,
    ///         PredecessorTree,
    ///     },
    ///     gen::Empty,
    ///     op::AddArc,
    /// };
    ///
    /// let mut digraph = Digraph::empty(8);
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
    /// assert!(BfsPred::new(&digraph, &[3, 7])
    ///     .predecessors()
    ///     .into_iter()
    ///     .eq([
    ///         Some(3),
    ///         Some(0),
    ///         Some(1),
    ///         None,
    ///         Some(1),
    ///         Some(6),
    ///         Some(7),
    ///         None
    ///     ]));
    /// ```
    pub fn predecessors(&mut self) -> PredecessorTree
    where
        D: Order + OutNeighbors,
    {
        let mut pred = PredecessorTree::new(self.digraph.order());

        for Step { u, v } in self {
            pred[v] = u;
        }

        pred
    }

    /// Finds the shortest path from the source vertices to a target vertex.
    ///
    /// # Arguments
    ///
    /// * `is_target`: The function determining if the vertex is a target.
    ///
    /// # Returns
    ///
    /// If it finds a target vertex, the function returns the shortest path
    /// to this target vertex. Otherwise, it returns `None`.
    ///
    /// # Panics
    ///
    /// * Panics if `is_target` panics.
    /// * Panics if a source vertices is not in `self.digraph`.
    /// * Panics if a successor vertex is not in `self.digraph`.
    ///
    /// # Examples
    ///
    /// ## Single source
    ///
    /// Red marks the path matching `v > 4`.
    ///
    /// ![BFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_pred_shortest_path_1.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     algo::bfs_pred::BfsPred,
    ///     gen::Empty,
    ///     op::AddArc,
    /// };
    ///
    /// let mut digraph = Digraph::empty(6);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(1, 4);
    /// digraph.add_arc(2, 5);
    /// digraph.add_arc(3, 0);
    ///
    /// assert!(BfsPred::new(&digraph, &[0])
    ///     .shortest_path(|v| v > 4)
    ///     .unwrap()
    ///     .eq(&[0, 1, 2, 5]));
    /// ```
    ///
    /// ## Multiple sources
    ///
    /// Red marks the path starting at vertex `3` matching `v == 2` and blue
    /// marks the path starting at vertex `7`  matching `v == 5`.
    ///
    /// ![BFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_pred_shortest_path_multi_source_1.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     algo::bfs_pred::BfsPred,
    ///     gen::Empty,
    ///     op::AddArc,
    /// };
    ///
    /// let mut digraph = Digraph::empty(8);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(1, 4);
    /// digraph.add_arc(2, 5);
    /// digraph.add_arc(2, 6);
    /// digraph.add_arc(3, 0);
    /// digraph.add_arc(6, 5);
    /// digraph.add_arc(6, 7);
    /// digraph.add_arc(7, 6);
    ///
    /// assert!(BfsPred::new(&digraph, &[3, 7])
    ///     .shortest_path(|v| v == 2)
    ///     .unwrap()
    ///     .eq(&[3, 0, 1, 2]));
    ///
    /// assert!(BfsPred::new(&digraph, &[3, 7])
    ///     .shortest_path(|v| v == 5)
    ///     .unwrap()
    ///     .eq(&[7, 6, 5]));
    /// ```
    #[must_use]
    pub fn shortest_path<P>(&mut self, is_target: P) -> Option<Vec<usize>>
    where
        D: Order + OutNeighbors,
        P: Fn(usize) -> bool,
    {
        let mut pred = PredecessorTree::new(self.digraph.order());

        for Step { u, v } in self.by_ref() {
            pred[v] = u;

            if is_target(v) {
                return pred.search_by(v, |_, b| b.is_none()).map(
                    |mut path| {
                        path.reverse();

                        path
                    },
                );
            }
        }

        None
    }
}

impl<'a, D> Iterator for BfsPred<'a, D>
where
    D: OutNeighbors,
{
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(step) = self.queue.pop_front() {
            let Step { v, .. } = step;

            for x in self.digraph.out_neighbors(v) {
                if self.visited.insert(x) {
                    self.queue.push_back(Step { u: Some(v), v: x });
                }
            }

            return Some(step);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::adjacency_list::fixture::{
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
    };

    #[test]
    fn iter_bang_jensen_196() {
        let digraph = bang_jensen_196();

        assert!(BfsPred::new(&digraph, &[0]).eq([
            Step { u: None, v: 0 },
            Step { u: Some(0), v: 1 },
            Step { u: Some(0), v: 4 },
            Step { u: Some(0), v: 7 },
            Step { u: Some(1), v: 2 },
            Step { u: Some(7), v: 5 },
            Step { u: Some(2), v: 3 },
            Step { u: Some(5), v: 6 },
        ]));
    }

    #[test]
    fn iter_bang_jensen_34() {
        let digraph = bang_jensen_34();

        assert!(BfsPred::new(&digraph, &[0])
            .eq([Step { u: None, v: 0 }, Step { u: Some(0), v: 4 }]));
    }

    #[test]
    fn iter_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(BfsPred::new(&digraph, &[0]).eq([
            Step { u: None, v: 0 },
            Step { u: Some(0), v: 1 },
            Step { u: Some(0), v: 2 },
            Step { u: Some(1), v: 3 },
            Step { u: Some(2), v: 4 },
            Step { u: Some(2), v: 5 },
            Step { u: Some(4), v: 6 }
        ]));
    }

    #[test]
    fn iter_kattis_builddeps() {
        let digraph = kattis_builddeps();

        assert!(BfsPred::new(&digraph, &[0]).eq([
            Step { u: None, v: 0 },
            Step { u: Some(0), v: 3 },
            Step { u: Some(0), v: 4 },
            Step { u: Some(3), v: 1 }
        ]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_1() {
        let digraph = kattis_cantinaofbabel_1();

        assert!(BfsPred::new(&digraph, &[0]).eq([
            Step { u: None, v: 0 },
            Step { u: Some(0), v: 1 },
            Step { u: Some(1), v: 2 },
            Step { u: Some(1), v: 4 },
            Step { u: Some(4), v: 3 },
            Step { u: Some(3), v: 5 },
            Step { u: Some(3), v: 7 },
            Step { u: Some(3), v: 10 },
            Step { u: Some(3), v: 11 },
            Step { u: Some(5), v: 6 },
            Step { u: Some(11), v: 9 }
        ]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_2() {
        let digraph = kattis_cantinaofbabel_2();

        assert!(BfsPred::new(&digraph, &[0]).eq([
            Step { u: None, v: 0 },
            Step { u: Some(0), v: 1 },
            Step { u: Some(1), v: 7 },
            Step { u: Some(7), v: 2 },
            Step { u: Some(2), v: 5 },
            Step { u: Some(5), v: 3 },
            Step { u: Some(5), v: 6 },
            Step { u: Some(3), v: 4 }
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(BfsPred::new(&digraph, &[5]).eq([
            Step { u: None, v: 5 },
            Step { u: Some(5), v: 6 },
            Step { u: Some(5), v: 9 },
            Step { u: Some(9), v: 13 },
            Step { u: Some(13), v: 12 }
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(BfsPred::new(&digraph, &[5]).eq([
            Step { u: None, v: 5 },
            Step { u: Some(5), v: 6 },
            Step { u: Some(5), v: 9 }
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(BfsPred::new(&digraph, &[5]).eq([
            Step { u: None, v: 5 },
            Step { u: Some(5), v: 1 },
            Step { u: Some(5), v: 6 },
            Step { u: Some(5), v: 9 },
            Step { u: Some(1), v: 2 },
            Step { u: Some(9), v: 13 },
            Step { u: Some(13), v: 12 }
        ]));
    }

    #[test]
    fn predecessors_bang_jensen_196() {
        let digraph = bang_jensen_196();

        assert!(BfsPred::new(&digraph, &[0]).predecessors().into_iter().eq([
            None,
            Some(0),
            Some(1),
            Some(2),
            Some(0),
            Some(7),
            Some(5),
            Some(0)
        ]));
    }

    #[test]
    fn predecessors_bang_jensen_34() {
        let digraph = bang_jensen_34();

        assert!(BfsPred::new(&digraph, &[0]).predecessors().into_iter().eq([
            None,
            None,
            None,
            None,
            Some(0),
            None
        ]));
    }

    #[test]
    fn predecessors_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(BfsPred::new(&digraph, &[0]).predecessors().into_iter().eq([
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
    fn predecessors_kattis_builddeps() {
        let digraph = kattis_builddeps();

        assert!(BfsPred::new(&digraph, &[0]).predecessors().into_iter().eq([
            None,
            Some(3),
            None,
            Some(0),
            Some(0),
            None
        ]));
    }

    #[test]
    fn predecessors_kattis_cantinaofbabel_1() {
        let digraph = kattis_cantinaofbabel_1();

        assert!(BfsPred::new(&digraph, &[0]).predecessors().into_iter().eq([
            None,
            Some(0),
            Some(1),
            Some(4),
            Some(1),
            Some(3),
            Some(5),
            Some(3),
            None,
            Some(11),
            Some(3),
            Some(3)
        ]));
    }

    #[test]
    fn predecessors_kattis_cantinaofbabel_2() {
        let digraph = kattis_cantinaofbabel_2();

        assert!(BfsPred::new(&digraph, &[0]).predecessors().into_iter().eq([
            None,
            Some(0),
            Some(7),
            Some(5),
            Some(3),
            Some(2),
            Some(5),
            Some(1),
            None,
            None,
            None,
            None
        ]));
    }

    #[test]
    fn predecessors_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(BfsPred::new(&digraph, &[5]).predecessors().into_iter().eq([
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
            None
        ]));
    }

    #[test]
    fn predecessors_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(BfsPred::new(&digraph, &[5]).predecessors().into_iter().eq([
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

        assert!(BfsPred::new(&digraph, &[5]).predecessors().into_iter().eq([
            None,
            Some(5),
            Some(1),
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
            None
        ]));
    }

    #[test]
    fn shortest_path_bang_jensen_196() {
        let digraph = bang_jensen_196();

        assert!(BfsPred::new(&digraph, &[0])
            .shortest_path(|v| v == 6)
            .unwrap()
            .eq(&[0, 7, 5, 6]));
    }

    #[test]
    fn shortest_path_bang_jensen_34() {
        let digraph = bang_jensen_34();

        assert!(BfsPred::new(&digraph, &[0])
            .shortest_path(|v| v == 4)
            .unwrap()
            .eq(&[0, 4]));
    }

    #[test]
    fn shortest_path_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(BfsPred::new(&digraph, &[0])
            .shortest_path(|v| v == 6)
            .unwrap()
            .eq(&[0, 2, 4, 6]));
    }

    #[test]
    fn shortest_path_kattis_builddeps() {
        let digraph = kattis_builddeps();

        assert!(BfsPred::new(&digraph, &[0])
            .shortest_path(|v| v == 1)
            .unwrap()
            .eq(&[0, 3, 1]));
    }

    #[test]
    fn shortest_path_kattis_cantinaofbabel_1() {
        let digraph = kattis_cantinaofbabel_1();

        assert!(BfsPred::new(&digraph, &[0])
            .shortest_path(|v| v == 9)
            .unwrap()
            .eq(&[0, 1, 4, 3, 11, 9]));
    }

    #[test]
    fn shortest_path_kattis_cantinaofbabel_2() {
        let digraph = kattis_cantinaofbabel_2();

        assert!(BfsPred::new(&digraph, &[0])
            .shortest_path(|v| v == 7)
            .unwrap()
            .eq(&[0, 1, 7]));
    }

    #[test]
    fn shortest_path_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(BfsPred::new(&digraph, &[5])
            .shortest_path(
                |v| [0, 1, 2, 3, 4, 7, 8, 11, 12, 13, 14, 15].contains(&v)
            )
            .unwrap()
            .eq(&[5, 9, 13]));
    }

    #[test]
    fn shortest_path_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(BfsPred::new(&digraph, &[5])
            .shortest_path(
                |v| [0, 1, 2, 3, 4, 7, 8, 11, 12, 13, 14, 15].contains(&v)
            )
            .is_none());
    }

    #[test]
    fn shortest_path_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(BfsPred::new(&digraph, &[5])
            .shortest_path(
                |v| [0, 1, 2, 3, 4, 7, 8, 11, 12, 13, 14, 15].contains(&v)
            )
            .unwrap()
            .eq(&[5, 1]));
    }
}

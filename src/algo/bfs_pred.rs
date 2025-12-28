//! Breadth-first search with predecessors.
//!
//! Breadth-first search explores an unweighted digraph's vertices in order of
//! their distance from a source.
//!
//! The time complexity is `O(v + a)`, where `v` is the digraph's order and `a`
//! is the digraph's size.
//!
//! # Examples
//!
//! ## Single source
//!
//! The path from vertex `0` is red. `p` denotes the predecessors. Note that,
//! in the digraph, vertex `3` precedes `0`, but the BFS algorithm starts at
//! `0`.
//!
//! ![A digraph and the predecessors along the breadth-first traversal](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_pred_1-0.87.4.svg?)
//!
//! ```
//! use {
//!     graaf::{
//!         AddArc,
//!         AdjacencyList,
//!         BfsPred,
//!         Empty,
//!     },
//!     std::iter::once,
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
//! assert!(BfsPred::new(&digraph, once(0)).eq([
//!     (None, 0),
//!     (Some(0), 1),
//!     (Some(1), 2),
//!     (Some(1), 4),
//!     (Some(2), 5),
//! ]));
//! ```
//!
//! ## Multiple sources
//!
//! The path from vertex `3` is red. The path from vertex `7` is blue.
//!
//! ![A digraph and the predecessors along the breadth-first traversal](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_pred_multi_source_1-0.87.4.svg?)
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     BfsPred,
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
//! assert!(BfsPred::new(&digraph, [3, 7].into_iter()).eq([
//!     (None, 3),
//!     (None, 7),
//!     (Some(3), 0),
//!     (Some(7), 6),
//!     (Some(0), 1),
//!     (Some(6), 5),
//!     (Some(1), 2),
//!     (Some(1), 4),
//! ]));
//! ```

use {
    crate::{
        Order,
        OutNeighbors,
        PredecessorTree,
    },
    std::collections::VecDeque,
};

type Step = (Option<usize>, usize);

/// Breadth-first search with predecessors.
///
/// Breadth-first search explores an unweighted digraph's vertices in order of
/// their distance from a source.
///
/// # Examples
///
/// ## Single source
///
/// The path from vertex `0` is red. `p` denotes the predecessors. Note that,
/// in the digraph, vertex `3` precedes `0`, but the BFS algorithm starts at
/// `0`.
///
/// ![A digraph and the predecessors along the breadth-first traversal](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_pred_1-0.87.4.svg?)
///
/// ```
/// use {
///     graaf::{
///         AddArc,
///         AdjacencyList,
///         BfsPred,
///         Empty,
///     },
///     std::iter::once,
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
/// assert!(BfsPred::new(&digraph, once(0)).eq([
///     (None, 0),
///     (Some(0), 1),
///     (Some(1), 2),
///     (Some(1), 4),
///     (Some(2), 5),
/// ]));
/// ```
///
/// ## Multiple sources
///
/// The path from vertex `3` is red. The path from vertex `7` is blue.
///
/// ![A digraph and the predecessors along the breadth-first traversal](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_pred_multi_source_1-0.87.4.svg?)
///
/// ```
/// use graaf::{
///     AddArc,
///     AdjacencyList,
///     BfsPred,
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
/// assert!(BfsPred::new(&digraph, [3, 7].into_iter()).eq([
///     (None, 3),
///     (None, 7),
///     (Some(3), 0),
///     (Some(7), 6),
///     (Some(0), 1),
///     (Some(6), 5),
///     (Some(1), 2),
///     (Some(1), 4),
/// ]));
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BfsPred<'a, D> {
    digraph: &'a D,
    queue: VecDeque<Step>,
    visited: Vec<bool>,
}

impl<'a, D> BfsPred<'a, D> {
    /// Construct a new breadth-first search.
    ///
    /// # Arguments
    ///
    /// * `digraph`: The digraph.
    /// * `sources`: The source vertices.
    #[must_use]
    pub fn new<T>(digraph: &'a D, sources: T) -> Self
    where
        D: Order,
        T: Iterator<Item = usize> + Clone,
    {
        let order = digraph.order();
        let mut queue = VecDeque::with_capacity(order);
        let mut visited = vec![false; order];
        let visited_ptr = visited.as_mut_ptr();

        for u in sources {
            queue.push_back((None, u));

            unsafe {
                *visited_ptr.add(u) = true;
            }
        }

        Self {
            digraph,
            queue,
            visited,
        }
    }

    /// Return cycles along the shortest path.
    ///
    /// Warning: This method doesn't find all cycles sharing a vertex with
    /// multiple predecessors.
    ///
    /// # Panics
    ///
    /// * Panics if a source vertex isn't in the digraph.
    /// * Panics if a successor vertex isn't in the digraph.
    ///
    /// # Examples
    ///
    /// ## Single source, connected
    ///
    /// ![Cycles along the breadth-first traversal in a connected digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_pred_cycles_1-0.88.6.svg?)
    ///
    /// ```
    /// use {
    ///     graaf::{
    ///         AddArc,
    ///         AdjacencyList,
    ///         BfsPred,
    ///         Empty,
    ///     },
    ///     std::iter::once,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(10);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 5);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(1, 3);
    /// digraph.add_arc(2, 0);
    /// digraph.add_arc(3, 4);
    /// digraph.add_arc(3, 5);
    /// digraph.add_arc(4, 5);
    /// digraph.add_arc(4, 6);
    /// digraph.add_arc(5, 8);
    /// digraph.add_arc(6, 7);
    /// digraph.add_arc(6, 9);
    /// digraph.add_arc(7, 3);
    /// digraph.add_arc(8, 9);
    /// digraph.add_arc(9, 5);
    ///
    /// assert!(BfsPred::new(&digraph, once(0)).cycles().iter().eq(&[
    ///     vec![0, 1, 2],
    ///     vec![5, 8, 9],
    ///     vec![3, 4, 6, 7]
    /// ]));
    /// ```
    ///
    /// ## Single source, disconnected
    ///
    /// ![Cycles along the breadth-first traversal in a disconnect digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_pred_cycles_disconnected_1-0.88.6.svg?)
    ///
    /// ```
    /// use {
    ///     graaf::{
    ///         AddArc,
    ///         AdjacencyList,
    ///         BfsPred,
    ///         Empty,
    ///     },
    ///     std::iter::once,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(10);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(2, 0);
    /// digraph.add_arc(3, 4);
    /// digraph.add_arc(4, 6);
    /// digraph.add_arc(5, 8);
    /// digraph.add_arc(6, 7);
    /// digraph.add_arc(7, 3);
    /// digraph.add_arc(8, 9);
    /// digraph.add_arc(9, 5);
    ///
    /// assert!(
    ///     BfsPred::new(&digraph, once(0))
    ///         .cycles()
    ///         .iter()
    ///         .eq(&[vec![0, 1, 2]])
    /// );
    /// ```
    ///
    /// ## Multiple sources, disconnected
    ///
    /// ![Cycles along a multi-source breadth-first traversal along a disconnected digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_pred_cycles_multi_source_1-0.88.6.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     BfsPred,
    ///     Empty,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(10);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(2, 0);
    /// digraph.add_arc(3, 4);
    /// digraph.add_arc(4, 6);
    /// digraph.add_arc(5, 8);
    /// digraph.add_arc(6, 7);
    /// digraph.add_arc(7, 3);
    /// digraph.add_arc(8, 9);
    /// digraph.add_arc(9, 5);
    ///
    /// assert!(
    ///     BfsPred::new(&digraph, [1, 8, 4].into_iter())
    ///         .cycles()
    ///         .iter()
    ///         .eq(&[vec![1, 2, 0], vec![8, 9, 5], vec![4, 6, 7, 3]])
    /// );
    /// ```
    #[must_use]
    pub fn cycles(&mut self) -> Vec<Vec<usize>>
    where
        D: Order + OutNeighbors,
    {
        let order = self.digraph.order();
        let mut pred = PredecessorTree::new(order);
        let mut cycles = Vec::new();
        let pred_ptr = pred.pred.as_mut_ptr();

        while let Some((u, v)) = self.next() {
            unsafe {
                *pred_ptr.add(v) = u;
            }

            for x in self.digraph.out_neighbors(v) {
                if let Some(mut path) = pred.search(v, x) {
                    path.reverse();
                    cycles.push(path);
                }
            }
        }

        cycles
    }

    /// Find the predecessor tree.
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
    /// The traversal from vertex `0` is red. The dashed arcs mark the
    /// predecessor tree.
    ///
    /// ![A digraph and the predecessor tree generated by a breadth-first search from a single source](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_pred_predecessors_1-0.87.4.svg?)
    ///
    /// ```
    /// use {
    ///     graaf::{
    ///         AddArc,
    ///         AdjacencyList,
    ///         BfsPred,
    ///         Empty,
    ///     },
    ///     std::iter::once,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(6);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(1, 4);
    /// digraph.add_arc(2, 5);
    ///
    /// assert!(
    ///     BfsPred::new(&digraph, once(0))
    ///         .predecessors()
    ///         .into_iter()
    ///         .eq([None, Some(0), Some(1), None, Some(1), Some(2)])
    /// );
    /// ```
    ///
    /// ## Multiple sources
    ///
    /// The traversal from vertex `3` is red. The traversal from vertex `7`
    /// is blue. The dashed arcs mark the predecessor trees.
    ///
    /// ![A digraph and the predecessor tree generated by a breadth-first search from multiple sources](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_pred_predecessors_multi_source_1-0.87.4.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     BfsPred,
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
    /// assert!(
    ///     BfsPred::new(&digraph, [3, 7].into_iter())
    ///         .predecessors()
    ///         .into_iter()
    ///         .eq([
    ///             Some(3),
    ///             Some(0),
    ///             Some(1),
    ///             None,
    ///             Some(1),
    ///             Some(6),
    ///             Some(7),
    ///             None
    ///         ])
    /// );
    /// ```
    #[must_use]
    pub fn predecessors(&mut self) -> PredecessorTree
    where
        D: Order + OutNeighbors,
    {
        let order = self.digraph.order();
        let mut pred = PredecessorTree::new(order);
        let pred_ptr = pred.pred.as_mut_ptr();

        for (u, v) in self.by_ref() {
            unsafe {
                *pred_ptr.add(v) = u;
            }
        }

        pred
    }

    /// Find the shortest path from the source vertices to a target vertex.
    ///
    /// # Arguments
    ///
    /// * `is_target`: The function determining if the vertex is a target.
    ///
    /// # Returns
    ///
    /// If `is_target` is `true`, the function returns the shortest path to
    /// this target vertex. Otherwise, it returns `None`.
    ///
    /// # Panics
    ///
    /// * Panics if `is_target` panics.
    /// * Panics if a source vertices isn't in the digraph.
    /// * Panics if a successor vertex isn't in the digraph.
    ///
    /// # Examples
    ///
    /// ## Single source
    ///
    /// The path from vertex `0` to the first vertex matching `v > 4` is red.
    ///
    /// ![A digraph and the shortest path from vertex `0` to the first vertex matching `v > 4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_pred_shortest_path_1-0.87.4.svg?)
    ///
    /// ```
    /// use {
    ///     graaf::{
    ///         AddArc,
    ///         AdjacencyList,
    ///         BfsPred,
    ///         Empty,
    ///     },
    ///     std::iter::once,
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
    /// assert!(
    ///     BfsPred::new(&digraph, once(0))
    ///         .shortest_path(|v| v > 4)
    ///         .unwrap()
    ///         .eq(&[0, 1, 2, 5])
    /// );
    /// ```
    ///
    /// ## Multiple sources
    ///
    /// The path from vertex `3` to the vertex matching `v == 2` is red. The
    /// path from vertex `7` to the vertex matching `v == 5` is blue.
    ///
    /// ![A digraph and two shortest paths](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_pred_shortest_path_multi_source_1-0.87.4.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     BfsPred,
    ///     Empty,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(8);
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
    /// assert!(
    ///     BfsPred::new(&digraph, [3, 7].into_iter())
    ///         .shortest_path(|v| v == 2)
    ///         .unwrap()
    ///         .eq(&[3, 0, 1, 2])
    /// );
    ///
    /// assert!(
    ///     BfsPred::new(&digraph, [3, 7].into_iter())
    ///         .shortest_path(|v| v == 5)
    ///         .unwrap()
    ///         .eq(&[7, 6, 5])
    /// );
    /// ```
    #[must_use]
    pub fn shortest_path<P>(&mut self, is_target: P) -> Option<Vec<usize>>
    where
        D: Order + OutNeighbors,
        P: Fn(usize) -> bool,
    {
        let order = self.digraph.order();
        let mut pred = PredecessorTree::new(order);
        let pred_ptr = pred.pred.as_mut_ptr();

        for (u, v) in self.by_ref() {
            unsafe {
                *pred_ptr.add(v) = u;
            }

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

impl<D> Iterator for BfsPred<'_, D>
where
    D: OutNeighbors,
{
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        let step @ (_, v) = self.queue.pop_front()?;
        let visited_ptr = self.visited.as_mut_ptr();

        for u in self.digraph.out_neighbors(v) {
            let visited_u = unsafe { visited_ptr.add(u) };

            unsafe {
                if !*visited_u {
                    *visited_u = true;

                    self.queue.push_back((Some(v), u));
                }
            }
        }

        Some(step)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::repr::adjacency_list::fixture::{
            bang_jensen_34,
            bang_jensen_94,
            bang_jensen_196,
            kattis_builddeps,
            kattis_cantinaofbabel_1,
            kattis_cantinaofbabel_2,
            kattis_escapewallmaria_1,
            kattis_escapewallmaria_2,
            kattis_escapewallmaria_3,
        },
        std::iter::once,
    };

    #[test]
    fn cycles_bang_jensen_196() {
        let digraph = bang_jensen_196();

        assert!(BfsPred::new(&digraph, once(0)).cycles().iter().eq(&[
            vec![0, 1],
            vec![2, 3],
            vec![7, 5, 6]
        ]));
    }

    #[test]
    fn cycles_bang_jensen_34() {
        let digraph = bang_jensen_34();

        assert!(BfsPred::new(&digraph, once(0)).cycles().iter().eq::<&[Vec<
            usize,
        >]>(
            &[]
        ));
    }

    #[test]
    fn cycles_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(BfsPred::new(&digraph, once(0)).cycles().iter().eq::<&[Vec<
            usize,
        >]>(
            &[]
        ));
    }

    #[test]
    fn cycles_kattis_builddeps() {
        let digraph = kattis_builddeps();

        assert!(BfsPred::new(&digraph, once(0)).cycles().iter().eq::<&[Vec<
            usize,
        >]>(
            &[]
        ));
    }

    #[test]
    fn cycles_kattis_cantinaofbabel_1() {
        let digraph = kattis_cantinaofbabel_1();

        assert!(BfsPred::new(&digraph, once(0)).cycles().iter().eq(&[
            vec![0, 1],
            vec![1, 2],
            vec![4, 3],
            vec![3, 7],
            vec![5, 6],
            vec![11, 9]
        ]));
    }

    #[test]
    fn cycles_kattis_cantinaofbabel_2() {
        let digraph = kattis_cantinaofbabel_2();

        assert!(BfsPred::new(&digraph, once(0)).cycles().iter().eq(&[
            vec![0, 1],
            vec![0, 1, 7, 2],
            vec![7, 2],
            vec![5, 6],
            vec![3, 4]
        ]));
    }

    #[test]
    fn cycles_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(BfsPred::new(&digraph, once(5)).cycles().iter().eq(&[
            vec![5, 6],
            vec![5, 9],
            vec![9, 13]
        ]));
    }

    #[test]
    fn cycles_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(
            BfsPred::new(&digraph, once(5))
                .cycles()
                .iter()
                .eq(&[vec![5, 6], vec![5, 9]])
        );
    }

    #[test]
    fn cycles_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(BfsPred::new(&digraph, once(5)).cycles().iter().eq(&[
            vec![5, 1],
            vec![5, 6],
            vec![5, 9],
            vec![1, 2],
            vec![9, 13],
            vec![13, 12]
        ]));
    }

    #[test]
    fn iter_bang_jensen_196() {
        let digraph = bang_jensen_196();

        assert!(BfsPred::new(&digraph, once(0)).eq([
            (None, 0),
            (Some(0), 1),
            (Some(0), 4),
            (Some(0), 7),
            (Some(1), 2),
            (Some(7), 5),
            (Some(2), 3),
            (Some(5), 6),
        ]));
    }

    #[test]
    fn iter_bang_jensen_34() {
        let digraph = bang_jensen_34();

        assert!(BfsPred::new(&digraph, once(0)).eq([(None, 0), (Some(0), 4)]));
    }

    #[test]
    fn iter_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(BfsPred::new(&digraph, once(0)).eq([
            (None, 0),
            (Some(0), 1),
            (Some(0), 2),
            (Some(1), 3),
            (Some(2), 4),
            (Some(2), 5),
            (Some(4), 6)
        ]));
    }

    #[test]
    fn iter_kattis_builddeps() {
        let digraph = kattis_builddeps();

        assert!(BfsPred::new(&digraph, once(0)).eq([
            (None, 0),
            (Some(0), 3),
            (Some(0), 4),
            (Some(3), 1)
        ]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_1() {
        let digraph = kattis_cantinaofbabel_1();

        assert!(BfsPred::new(&digraph, once(0)).eq([
            (None, 0),
            (Some(0), 1),
            (Some(1), 2),
            (Some(1), 4),
            (Some(4), 3),
            (Some(3), 5),
            (Some(3), 7),
            (Some(3), 10),
            (Some(3), 11),
            (Some(5), 6),
            (Some(11), 9)
        ]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_2() {
        let digraph = kattis_cantinaofbabel_2();

        assert!(BfsPred::new(&digraph, once(0)).eq([
            (None, 0),
            (Some(0), 1),
            (Some(1), 7),
            (Some(7), 2),
            (Some(2), 5),
            (Some(5), 3),
            (Some(5), 6),
            (Some(3), 4)
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(BfsPred::new(&digraph, once(5)).eq([
            (None, 5),
            (Some(5), 6),
            (Some(5), 9),
            (Some(9), 13),
            (Some(13), 12)
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(BfsPred::new(&digraph, once(5)).eq([
            (None, 5),
            (Some(5), 6),
            (Some(5), 9)
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(BfsPred::new(&digraph, once(5)).eq([
            (None, 5),
            (Some(5), 1),
            (Some(5), 6),
            (Some(5), 9),
            (Some(1), 2),
            (Some(9), 13),
            (Some(13), 12)
        ]));
    }

    #[test]
    fn predecessors_bang_jensen_196() {
        let digraph = bang_jensen_196();

        assert!(
            BfsPred::new(&digraph, once(0))
                .predecessors()
                .into_iter()
                .eq([
                    None,
                    Some(0),
                    Some(1),
                    Some(2),
                    Some(0),
                    Some(7),
                    Some(5),
                    Some(0)
                ])
        );
    }

    #[test]
    fn predecessors_bang_jensen_34() {
        let digraph = bang_jensen_34();

        assert!(
            BfsPred::new(&digraph, once(0))
                .predecessors()
                .into_iter()
                .eq([None, None, None, None, Some(0), None])
        );
    }

    #[test]
    fn predecessors_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(
            BfsPred::new(&digraph, once(0))
                .predecessors()
                .into_iter()
                .eq([
                    None,
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(2),
                    Some(2),
                    Some(4)
                ])
        );
    }

    #[test]
    fn predecessors_kattis_builddeps() {
        let digraph = kattis_builddeps();

        assert!(
            BfsPred::new(&digraph, once(0))
                .predecessors()
                .into_iter()
                .eq([None, Some(3), None, Some(0), Some(0), None])
        );
    }

    #[test]
    fn predecessors_kattis_cantinaofbabel_1() {
        let digraph = kattis_cantinaofbabel_1();

        assert!(
            BfsPred::new(&digraph, once(0))
                .predecessors()
                .into_iter()
                .eq([
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
                ])
        );
    }

    #[test]
    fn predecessors_kattis_cantinaofbabel_2() {
        let digraph = kattis_cantinaofbabel_2();

        assert!(
            BfsPred::new(&digraph, once(0))
                .predecessors()
                .into_iter()
                .eq([
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
                ])
        );
    }

    #[test]
    fn predecessors_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(
            BfsPred::new(&digraph, once(5))
                .predecessors()
                .into_iter()
                .eq([
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
                ])
        );
    }

    #[test]
    fn predecessors_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(
            BfsPred::new(&digraph, once(5))
                .predecessors()
                .into_iter()
                .eq([
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
                ])
        );
    }

    #[test]
    fn predecessors_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(
            BfsPred::new(&digraph, once(5))
                .predecessors()
                .into_iter()
                .eq([
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
                ])
        );
    }

    #[test]
    fn shortest_path_bang_jensen_196() {
        let digraph = bang_jensen_196();

        assert!(
            BfsPred::new(&digraph, once(0))
                .shortest_path(|v| v == 6)
                .unwrap()
                .eq(&[0, 7, 5, 6])
        );
    }

    #[test]
    fn shortest_path_bang_jensen_34() {
        let digraph = bang_jensen_34();

        assert!(
            BfsPred::new(&digraph, once(0))
                .shortest_path(|v| v == 4)
                .unwrap()
                .eq(&[0, 4])
        );
    }

    #[test]
    fn shortest_path_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(
            BfsPred::new(&digraph, once(0))
                .shortest_path(|v| v == 6)
                .unwrap()
                .eq(&[0, 2, 4, 6])
        );
    }

    #[test]
    fn shortest_path_kattis_builddeps() {
        let digraph = kattis_builddeps();

        assert!(
            BfsPred::new(&digraph, once(0))
                .shortest_path(|v| v == 1)
                .unwrap()
                .eq(&[0, 3, 1])
        );
    }

    #[test]
    fn shortest_path_kattis_cantinaofbabel_1() {
        let digraph = kattis_cantinaofbabel_1();

        assert!(
            BfsPred::new(&digraph, once(0))
                .shortest_path(|v| v == 9)
                .unwrap()
                .eq(&[0, 1, 4, 3, 11, 9])
        );
    }

    #[test]
    fn shortest_path_kattis_cantinaofbabel_2() {
        let digraph = kattis_cantinaofbabel_2();

        assert!(
            BfsPred::new(&digraph, once(0))
                .shortest_path(|v| v == 7)
                .unwrap()
                .eq(&[0, 1, 7])
        );
    }

    #[test]
    fn shortest_path_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(
            BfsPred::new(&digraph, once(5))
                .shortest_path(
                    |v| [0, 1, 2, 3, 4, 7, 8, 11, 12, 13, 14, 15].contains(&v)
                )
                .unwrap()
                .eq(&[5, 9, 13])
        );
    }

    #[test]
    fn shortest_path_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(
            BfsPred::new(&digraph, once(5))
                .shortest_path(
                    |v| [0, 1, 2, 3, 4, 7, 8, 11, 12, 13, 14, 15].contains(&v)
                )
                .is_none()
        );
    }

    #[test]
    fn shortest_path_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(
            BfsPred::new(&digraph, once(5))
                .shortest_path(
                    |v| [0, 1, 2, 3, 4, 7, 8, 11, 12, 13, 14, 15].contains(&v)
                )
                .unwrap()
                .eq(&[5, 1])
        );
    }
}

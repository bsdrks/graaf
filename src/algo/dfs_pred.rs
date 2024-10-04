//! Depth-first search with predecessors.
//!
//! Depth-first search is a digraph traversal algorithm that explores a digraph
//! by following a path as far as possible before backtracking.
//!
//! Runs in **O(v + a)** time, where **v** is the number of vertices and **a**
//! is the number of arcs.
//!
//! # Examples
//!
//! ## Single source
//!
//! The path from vertex `0` is red. `p` denotes the predecessors.
//!
//! ![A digraph and the predecessors along the depth-first traversal](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dfs_pred_1-0.87.4.svg?)
//!
//! ```
//! use {
//!     graaf::{
//!         AddArc,
//!         AdjacencyList,
//!         DfsPred,
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
//! assert!(DfsPred::new(&digraph, once(0)).eq([
//!     (None, 0),
//!     (Some(0), 1),
//!     (Some(1), 4),
//!     (Some(1), 2),
//!     (Some(2), 5),
//! ]));
//! ```
//!
//! ## Multiple sources
//!
//! The path from vertex `3` is red. The path from vertex `7` is blue.
//!
//! ![A digraph and the predecessors along the depth-first traversal](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dfs_pred_multi_source_1-0.87.4.svg?)
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     DfsPred,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyList::empty(8);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 4);
//! digraph.add_arc(2, 3);
//! digraph.add_arc(2, 5);
//! digraph.add_arc(2, 6);
//! digraph.add_arc(3, 0);
//! digraph.add_arc(6, 5);
//! digraph.add_arc(6, 7);
//! digraph.add_arc(7, 6);
//!
//! assert!(DfsPred::new(&digraph, [3, 7].into_iter()).eq([
//!     (None, 7),
//!     (Some(7), 6),
//!     (Some(6), 5),
//!     (None, 3),
//!     (Some(3), 0),
//!     (Some(0), 1),
//!     (Some(1), 4),
//! ]));
//! ```

use {
    crate::{
        Order,
        OutNeighbors,
        PredecessorTree,
    },
    std::collections::HashSet,
};

type Step = (Option<usize>, usize);

/// Depth-first search with predecessors.
///
/// # Examples
///
/// ## Single source
///
/// The path from vertex `0` is red. `p` denotes the predecessors.
///
/// ![A digraph and the predecessors along the depth-first traversal](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dfs_pred_1-0.87.4.svg?)
///
/// ```
/// use {
///     graaf::{
///         AddArc,
///         AdjacencyList,
///         DfsPred,
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
/// assert!(DfsPred::new(&digraph, once(0)).eq([
///     (None, 0),
///     (Some(0), 1),
///     (Some(1), 4),
///     (Some(1), 2),
///     (Some(2), 5),
/// ]));
/// ```
///
/// ## Multiple sources
///
/// The path from vertex `3` is red. The path from vertex `7` is blue.
///
/// ![A digraph and the predecessors along the depth-first traversal](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dfs_pred_multi_source_1-0.87.4.svg?)
///
/// ```
/// use graaf::{
///     AddArc,
///     AdjacencyList,
///     DfsPred,
///     Empty,
/// };
///
/// let mut digraph = AdjacencyList::empty(8);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 4);
/// digraph.add_arc(2, 3);
/// digraph.add_arc(2, 5);
/// digraph.add_arc(2, 6);
/// digraph.add_arc(3, 0);
/// digraph.add_arc(6, 5);
/// digraph.add_arc(6, 7);
/// digraph.add_arc(7, 6);
///
/// assert!(DfsPred::new(&digraph, [3, 7].into_iter()).eq([
///     (None, 7),
///     (Some(7), 6),
///     (Some(6), 5),
///     (None, 3),
///     (Some(3), 0),
///     (Some(0), 1),
///     (Some(1), 4),
/// ]));
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DfsPred<'a, D> {
    digraph: &'a D,
    stack: Vec<Step>,
    visited: HashSet<usize>,
}

impl<'a, D> DfsPred<'a, D> {
    /// Construct a new depth-first search.
    ///
    /// # Arguments
    ///
    /// * `digraph`: The digraph.
    /// * `sources`: The source vertices.
    #[must_use]
    pub fn new<T>(digraph: &'a D, sources: T) -> Self
    where
        T: Iterator<Item = usize>,
    {
        Self {
            digraph,
            stack: sources.map(|u| (None, u)).collect(),
            visited: HashSet::new(),
        }
    }

    /// Find the predecessors tree.
    ///
    /// # Panics
    ///
    /// * Panics if the `self.next` panics.
    /// * Panics if a source vertex isn't in the digraph.
    /// * Panics if a successor vertex isn't in the digraph.
    ///
    /// # Examples
    ///
    /// ## Single source
    ///
    /// The path from vertex `0` is red. The dashed arcs mark the predecessor
    /// trees.
    ///
    /// ![A digraph and the predecessor tree generated by a depth-first search from a single source](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dfs_pred_predecessors_1-0.87.4.svg?)
    ///
    /// ```
    /// use {
    ///     graaf::{
    ///         AddArc,
    ///         AdjacencyList,
    ///         DfsPred,
    ///         Empty,
    ///         PredecessorTree,
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
    /// assert!(DfsPred::new(&digraph, once(0))
    ///     .predecessors()
    ///     .into_iter()
    ///     .eq([None, Some(0), Some(1), None, Some(1), Some(2)]));
    /// ```
    ///
    /// ## Multiple sources
    ///
    /// The path from vertex `3` is red. The path from vertex `7` is blue. The
    /// dashed arcs mark the predecessor tree.
    ///
    /// ![A digraph and the predecessor tree generated by a depth-first search from multiple sources](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dfs_pred_predecessors_multi_source_1-0.87.4.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     DfsPred,
    ///     Empty,
    ///     PredecessorTree,
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
    /// assert!(DfsPred::new(&digraph, [3, 7].into_iter())
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
    #[must_use]
    pub fn predecessors(&mut self) -> PredecessorTree
    where
        D: Order + OutNeighbors,
    {
        self.fold(
            PredecessorTree::new(self.digraph.order()),
            |mut pred, step| {
                pred[step.1] = step.0;

                pred
            },
        )
    }
}

impl<'a, D> Iterator for DfsPred<'a, D>
where
    D: OutNeighbors,
{
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        let step @ (_, v) = self.stack.pop()?;

        if self.visited.insert(v) {
            self.stack
                .extend(self.digraph.out_neighbors(v).filter_map(|x| {
                    (!self.visited.contains(&x)).then_some((Some(v), x))
                }));

            return Some(step);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::repr::adjacency_list::fixture::{
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
        std::iter::once,
    };

    #[test]
    fn iter_bang_jensen_196() {
        let digraph = bang_jensen_196();

        assert!(DfsPred::new(&digraph, once(0)).eq([
            (None, 0),
            (Some(0), 7),
            (Some(7), 5),
            (Some(5), 6),
            (Some(0), 4),
            (Some(4), 2),
            (Some(2), 3),
            (Some(0), 1),
        ]));
    }

    #[test]
    fn iter_bang_jensen_34() {
        let digraph = bang_jensen_34();

        assert!(DfsPred::new(&digraph, once(0)).eq([(None, 0), (Some(0), 4)]));
    }

    #[test]
    fn iter_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(DfsPred::new(&digraph, once(0)).eq([
            (None, 0),
            (Some(0), 2),
            (Some(2), 5),
            (Some(2), 4),
            (Some(4), 6),
            (Some(2), 3),
            (Some(2), 1),
        ]));
    }

    #[test]
    fn iter_kattis_builddeps() {
        let digraph = kattis_builddeps();

        assert!(DfsPred::new(&digraph, once(0)).eq([
            (None, 0),
            (Some(0), 4),
            (Some(4), 1),
            (Some(0), 3),
        ]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_1() {
        let digraph = kattis_cantinaofbabel_1();

        assert!(DfsPred::new(&digraph, once(0)).eq([
            (None, 0),
            (Some(0), 1),
            (Some(1), 4),
            (Some(4), 3),
            (Some(3), 11),
            (Some(11), 9),
            (Some(9), 7),
            (Some(3), 10),
            (Some(10), 6),
            (Some(6), 5),
        ]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_2() {
        let digraph = kattis_cantinaofbabel_2();

        assert!(DfsPred::new(&digraph, once(0)).eq([
            (None, 0),
            (Some(0), 1),
            (Some(1), 7),
            (Some(7), 2),
            (Some(2), 5),
            (Some(5), 6),
            (Some(5), 3),
            (Some(3), 4),
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(DfsPred::new(&digraph, once(5)).eq([
            (None, 5),
            (Some(5), 9),
            (Some(9), 13),
            (Some(13), 12),
            (Some(5), 6),
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(DfsPred::new(&digraph, once(5)).eq([
            (None, 5),
            (Some(5), 9),
            (Some(5), 6),
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(DfsPred::new(&digraph, once(1)).eq([
            (None, 1),
            (Some(1), 5),
            (Some(5), 9),
            (Some(9), 13),
            (Some(13), 12),
            (Some(5), 6),
            (Some(6), 2),
        ]));
    }

    #[test]
    fn predecessors_bang_jensen_196() {
        let digraph = bang_jensen_196();

        assert!(DfsPred::new(&digraph, once(0))
            .predecessors()
            .into_iter()
            .eq([
                None,
                Some(0),
                Some(4),
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

        assert!(DfsPred::new(&digraph, once(0))
            .predecessors()
            .into_iter()
            .eq([None, None, None, None, Some(0), None]));
    }

    #[test]
    fn predecessors_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(DfsPred::new(&digraph, once(0))
            .predecessors()
            .into_iter()
            .eq([None, Some(2), Some(0), Some(2), Some(2), Some(2), Some(4)]));
    }

    #[test]
    fn predecessors_kattis_builddeps() {
        let digraph = kattis_builddeps();

        assert!(DfsPred::new(&digraph, once(0))
            .predecessors()
            .into_iter()
            .eq([None, Some(4), None, Some(0), Some(0), None]));
    }

    #[test]
    fn predecessors_kattis_cantinaofbabel_1() {
        let digraph = kattis_cantinaofbabel_1();

        assert!(DfsPred::new(&digraph, once(0))
            .predecessors()
            .into_iter()
            .eq([
                None,
                Some(0),
                None,
                Some(4),
                Some(1),
                Some(6),
                Some(10),
                Some(9),
                None,
                Some(11),
                Some(3),
                Some(3)
            ]));
    }

    #[test]
    fn predecessors_kattis_cantinaofbabel_2() {
        let digraph = kattis_cantinaofbabel_2();

        assert!(DfsPred::new(&digraph, once(0))
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
            ]));
    }

    #[test]
    fn predecessors_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(DfsPred::new(&digraph, once(5))
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
            ]));
    }

    #[test]
    fn predecessors_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(DfsPred::new(&digraph, once(5))
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
            ]));
    }

    #[test]
    fn predecessors_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(DfsPred::new(&digraph, once(5))
            .predecessors()
            .into_iter()
            .eq([
                None,
                Some(2),
                Some(6),
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
}

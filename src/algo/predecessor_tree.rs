//! A predecessor tree.
//!
//! A [`PredecessorTree`] contains each vertex's predecessor on the shortest
//! path from the source vertex.
//!
//! # Examples
//!
//! This [`PredecessorTree`] is generated by
//! [`BfsPred::predecessors`](crate::BfsPred::predecessors).
//!
//! The path from vertex `0` is red and the dashed arcs mark the
//! [`PredecessorTree`].
//!
//! ![A digraph and the predecessor tree generated by breadth-first search](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_pred_predecessors_1-0.87.4.svg?)
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
//!
//! assert!(BfsPred::new(&digraph, once(0))
//!     .predecessors()
//!     .into_iter()
//!     .eq([None, Some(0), Some(1), None, Some(1), Some(2)]));
//! ```
use std::{
    ops::{
        Index,
        IndexMut,
    },
    vec::IntoIter,
};

/// A predecessor tree.
///
/// A [`PredecessorTree`] contains each vertex's predecessor on the shortest
/// path from the source vertex.
///
/// # Examples
///
/// This [`PredecessorTree`] is generated by
/// [`BfsPred::predecessors`](crate::BfsPred::predecessors).
///
/// The path from vertex `0` is red. The dashed arcs mark the
/// [`PredecessorTree`].
///
/// ![A digraph and the predecessor tree generated by breadth-first search](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_pred_predecessors_1-0.87.4.svg?)
///
/// ```
/// use {
///     graaf::{
///         AddArc,
///         AdjacencyList,
///         BfsPred,
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
/// assert!(BfsPred::new(&digraph, once(0))
///     .predecessors()
///     .into_iter()
///     .eq([None, Some(0), Some(1), None, Some(1), Some(2)]));
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct PredecessorTree {
    /// The predecessors of each vertex.
    pub pred: Vec<Option<usize>>,
}

impl PredecessorTree {
    /// Construct a [`PredecessorTree`] with a given order.
    ///
    /// # Arguments
    ///
    /// * `order`: The number of vertices in the [`PredecessorTree`].
    ///
    /// # Panics
    ///
    /// Panics if `order` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::PredecessorTree;
    ///
    /// assert!(PredecessorTree::new(4)
    ///     .into_iter()
    ///     .eq([None, None, None, None]));
    /// ```
    #[must_use]
    pub fn new(order: usize) -> Self {
        assert!(order > 0, "a predecessor tree has at least one vertex");

        Self {
            pred: vec![None; order],
        }
    }

    /// Search a [`PredecessorTree`] for a path from a source vertex to a
    /// target vertex.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    /// * `t`: The target vertex.
    ///
    /// # Returns
    ///
    /// If a path from `s` to `t` is found, the function returns it. Otherwise,
    /// returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::PredecessorTree;
    ///
    /// let pred = PredecessorTree::from(vec![Some(1), Some(2), Some(3), None]);
    ///
    /// assert!(pred.search(0, 3).into_iter().eq(Some(vec![0, 1, 2, 3])));
    /// ```
    #[must_use]
    pub fn search(&self, s: usize, t: usize) -> Option<Vec<usize>> {
        self.search_by(s, |&v, _| v == t)
    }

    /// Search a [`PredecessorTree`] for a path from a source vertex to a
    /// vertex that satisfies a predicate.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    /// * `is_target`: A predicate determining if a vertex is the target.
    ///
    /// # Returns
    ///
    /// If it finds a target, it returns the path from the source to the
    /// target. Otherwise, it returns `None`.
    ///
    /// # Panics
    ///
    /// Panics if `s` isn't in the path.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::PredecessorTree;
    ///
    /// let pred = PredecessorTree::from(vec![Some(1), Some(2), Some(3), None]);
    ///
    /// assert!(pred
    ///     .search_by(0, |&v, _| v > 1)
    ///     .into_iter()
    ///     .eq(Some(vec![0, 1, 2])));
    ///
    /// let pred =
    ///     PredecessorTree::from(vec![Some(1), Some(2), Some(3), None, Some(0)]);
    ///
    /// assert!(pred
    ///     .search_by(0, |_, v| v.is_none())
    ///     .into_iter()
    ///     .eq(Some(vec![0, 1, 2, 3])));
    /// ```
    #[must_use]
    pub fn search_by<F>(
        &self,
        mut s: usize,
        is_target: F,
    ) -> Option<Vec<usize>>
    where
        F: Fn(&usize, &Option<usize>) -> bool,
    {
        if is_target(&s, &self.pred[s]) {
            return Some(vec![s]);
        }

        let mut visited = vec![false; self.pred.len()];
        let visited_ptr = visited.as_mut_ptr();
        let mut path = vec![s];

        while let Some(&v) = self.pred.get(s) {
            if is_target(&s, &v) {
                return Some(path);
            }

            if let Some(v) = v {
                if unsafe { *visited_ptr.add(v) } {
                    break;
                }

                unsafe {
                    *visited_ptr.add(v) = true;
                }

                if v != s {
                    path.push(v);
                }

                s = v;
            } else {
                break;
            }
        }

        None
    }
}

impl From<Vec<Option<usize>>> for PredecessorTree {
    fn from(pred: Vec<Option<usize>>) -> Self {
        Self { pred }
    }
}

impl Index<usize> for PredecessorTree {
    type Output = Option<usize>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pred[index]
    }
}

impl IndexMut<usize> for PredecessorTree {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pred[index]
    }
}

impl IntoIterator for PredecessorTree {
    type IntoIter = IntoIter<Self::Item>;
    type Item = Option<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.pred.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index() {
        let pred =
            PredecessorTree::from(vec![Some(1), Some(2), Some(3), None]);

        assert_eq!(pred[0], Some(1));
        assert_eq!(pred[1], Some(2));
        assert_eq!(pred[2], Some(3));
        assert_eq!(pred[3], None);
    }

    #[test]
    fn index_mut() {
        let mut pred =
            PredecessorTree::from(vec![Some(1), Some(2), Some(3), None]);

        pred[0] = Some(0);
        pred[1] = None;
        pred[2] = None;
        pred[3] = Some(2);

        assert_eq!(pred[0], Some(0));
        assert_eq!(pred[1], None);
        assert_eq!(pred[2], None);
        assert_eq!(pred[3], Some(2));
    }

    #[test]
    fn new() {
        let pred = PredecessorTree::new(1);

        assert_eq!(pred.search(0, 0), Some(vec![0]));
    }

    #[test]
    #[should_panic(expected = "a predecessor tree has at least one vertex")]
    fn new_0() {
        let _ = PredecessorTree::new(0);
    }

    #[test]
    fn search_singleton_s_ne_t() {
        assert_eq!(PredecessorTree::from(vec![Some(0)]).search(0, 1), None);
    }

    #[test]
    fn search_singleton_s_equals_t() {
        assert!(PredecessorTree::from(vec![Some(0)])
            .search(0, 0)
            .unwrap()
            .into_iter()
            .eq([0]));
    }

    #[test]
    fn search_no_path() {
        assert_eq!(
            PredecessorTree::from(vec![Some(1), Some(2), None, None])
                .search(0, 3),
            None
        );
    }

    #[test]
    fn search_circuit() {
        assert_eq!(
            PredecessorTree::from(vec![Some(1), Some(2), Some(0), None])
                .search(0, 3),
            None
        );
    }

    #[test]
    fn search_path_s_equals_t() {
        assert!(PredecessorTree::from(vec![Some(1), Some(2), Some(0), None])
            .search(0, 0)
            .unwrap()
            .into_iter()
            .eq([0]));
    }

    #[test]
    fn search_path_s_ne_t() {
        assert!(PredecessorTree::from(vec![Some(1), Some(2), Some(3), None])
            .search(1, 3)
            .unwrap()
            .into_iter()
            .eq([1, 2, 3]));
    }

    #[test]
    fn search_by_singleton_s_equals_t() {
        assert!(PredecessorTree::from(vec![Some(0)])
            .search_by(0, |&t, _| t == 0)
            .unwrap()
            .into_iter()
            .eq([0]));
    }

    #[test]
    fn search_by_singleton_s_ne_t() {
        assert_eq!(
            PredecessorTree::from(vec![Some(0)]).search_by(0, |&t, _| t == 1),
            None
        );
    }

    #[test]
    fn search_by_no_path() {
        assert_eq!(
            PredecessorTree::from(vec![Some(1), Some(2), None, None])
                .search_by(0, |&t, _| t == 3),
            None
        );
    }

    #[test]
    fn search_by_circuit() {
        assert_eq!(
            PredecessorTree::from(vec![Some(1), Some(2), Some(0), None])
                .search_by(0, |&t, _| t == 3),
            None
        );
    }

    #[test]
    fn search_by_path_s_equals_t() {
        assert!(PredecessorTree::from(vec![Some(1), Some(2), Some(0), None])
            .search_by(0, |&t, _| t == 0)
            .unwrap()
            .into_iter()
            .eq([0]));
    }

    #[test]
    fn search_by_path_s_ne_t() {
        assert!(PredecessorTree::from(vec![Some(1), Some(2), Some(3), None])
            .search_by(1, |&t, _| t == 3)
            .unwrap()
            .into_iter()
            .eq([1, 2, 3]));
    }
}

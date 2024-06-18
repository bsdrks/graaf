//! Breadth-first tree.
//!
//! A breadth-first tree is obtained by running a [breadth-first search] on a
//! digraph. The tree contains the predecessor of each vertex on the shortest
//! path from the source vertex.
//!
//! [breadth-first search]: `crate::algo::bfs`

use std::{
    collections::HashSet,
    ops::{
        Index,
        IndexMut,
    },
    vec::IntoIter,
};

/// A breadth-first tree.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BfsTree {
    /// The predecessor of each vertex on the shortest path from the source.
    pred: Vec<Option<usize>>,
}

impl BfsTree {
    /// Creates a breadth-first tree.
    ///
    /// # Arguments
    ///
    /// * `pred`: The predecessor of each vertex on the shortest path from the
    ///   source.
    #[must_use]
    pub fn new(v: usize) -> Self {
        Self {
            pred: vec![None; v],
        }
    }

    /// Searches a breadth-first tree for a path from a source vertex to a
    /// target vertex.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex of the path.
    /// * `t`: The target vertex of the path.
    ///
    /// # Returns
    ///
    /// If a path from `s` to `t` is found, the function returns it. Otherwise,
    /// returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::algo::bfs_tree::BfsTree;
    ///
    /// let pred = BfsTree::from(vec![Some(1), Some(2), Some(3), None]);
    ///
    /// assert!(pred.search(0, 3).into_iter().eq(Some(vec![0, 1, 2, 3])));
    /// ```
    #[must_use]
    pub fn search(&self, s: usize, t: usize) -> Option<Vec<usize>> {
        self.search_by(s, |&v, _| v == t)
    }

    /// Searches a breadth-first tree for a path from a source vertex to a
    /// target vertex that satisfies a predicate.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex of the path.
    /// * `is_target`: A predicate determining whether a vertex is the target.
    ///
    /// # Returns
    ///
    /// If it finds a target, it returns the path from the source to the target.
    /// Otherwise, it returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::algo::bfs_tree::BfsTree;
    ///
    /// let pred = BfsTree::from(vec![Some(1), Some(2), Some(3), None]);
    ///
    /// assert!(pred
    ///     .search_by(0, |&v, _| v > 1)
    ///     .into_iter()
    ///     .eq(Some(vec![0, 1, 2])));
    /// ```
    ///
    /// ```
    /// use graaf::algo::bfs_tree::BfsTree;
    ///
    /// let pred = BfsTree::from(vec![Some(1), Some(2), Some(3), None, Some(0)]);
    ///
    /// assert!(pred
    ///     .search_by(0, |_, u| u.is_none())
    ///     .into_iter()
    ///     .eq(Some(vec![0, 1, 2, 3])));
    /// ```
    #[must_use]
    pub fn search_by<F>(&self, mut s: usize, is_target: F) -> Option<Vec<usize>>
    where
        F: Fn(&usize, &Option<usize>) -> bool,
    {
        let mut visited = HashSet::new();
        let mut path = vec![s];

        while let Some(&u) = self.pred.get(s) {
            if is_target(&s, &u) {
                return Some(path);
            }

            if let Some(v) = u {
                if !visited.insert(v) {
                    break;
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

impl From<Vec<Option<usize>>> for BfsTree {
    fn from(pred: Vec<Option<usize>>) -> Self {
        Self { pred }
    }
}

impl Index<usize> for BfsTree {
    type Output = Option<usize>;

    fn index(&self, s: usize) -> &Self::Output {
        &self.pred[s]
    }
}

impl IndexMut<usize> for BfsTree {
    fn index_mut(&mut self, s: usize) -> &mut Self::Output {
        &mut self.pred[s]
    }
}

impl IntoIterator for BfsTree {
    type IntoIter = IntoIter<Self::Item>;
    type Item = Option<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.pred.into_iter()
    }
}

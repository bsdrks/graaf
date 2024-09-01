//! Depth-first search with predecessors
//!
//! Depth-first search is a digraph traversal algorithm that explores a digraph
//! by following a path as far as possible before backtracking.
//!
//! The time complexity is *O*(*v* + *a*).
//!
//! # Examples
//!
//! ## Single source
//!
//! Red marks the path starting at vertex `0` and `p` denotes the predecessor.
//!
//! ![DFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dfs_pred_1-0.87.0.svg?)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     algo::dfs_pred::{
//!         DfsPred,
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
//! assert!(DfsPred::new(&digraph, &[0]).eq([
//!     Step { u: None, v: 0 },
//!     Step { u: Some(0), v: 1 },
//!     Step { u: Some(1), v: 4 },
//!     Step { u: Some(1), v: 2 },
//!     Step { u: Some(2), v: 5 },
//! ]));
//! ```
//!
//! ## Multiple sources
//!
//! Red marks the path starting at vertex `3` and blue the path starting at
//! vertex `7`.
//!
//! ![DFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dfs_pred_multi_source_1-0.87.0.svg?)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     algo::dfs_pred::{
//!         DfsPred,
//!         Step,
//!     },
//!     gen::Empty,
//!     op::AddArc,
//! };
//!
//! let mut digraph = Digraph::empty(8);
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
//! assert!(DfsPred::new(&digraph, &[3, 7]).eq([
//!     Step { u: None, v: 7 },
//!     Step { u: Some(7), v: 6 },
//!     Step { u: Some(6), v: 5 },
//!     Step { u: None, v: 3 },
//!     Step { u: Some(3), v: 0 },
//!     Step { u: Some(0), v: 1 },
//!     Step { u: Some(1), v: 4 },
//! ]));
//! ```

use {
    crate::op::OutNeighbors,
    std::collections::HashSet,
};

/// A step in the depth-first search.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Step {
    /// The predecessor vertex of the current vertex, if any. The source
    /// vertices have no predecessor.
    pub u: Option<usize>,
    /// The current vertex.
    pub v: usize,
}

/// Depth-first search with predecessors.
///
/// # Examples
///
/// ## Single source
///
/// Red marks the path starting at vertex `0` and `p` denotes the predecessor.
///
/// ![DFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dfs_pred_1-0.87.0.svg?)
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     algo::dfs_pred::{
///         DfsPred,
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
/// assert!(DfsPred::new(&digraph, &[0]).eq([
///     Step { u: None, v: 0 },
///     Step { u: Some(0), v: 1 },
///     Step { u: Some(1), v: 4 },
///     Step { u: Some(1), v: 2 },
///     Step { u: Some(2), v: 5 },
/// ]));
/// ```
///
/// ## Multiple sources
///
/// Red marks the path starting at vertex `3` and blue the path starting at
/// vertex `7`.
///
/// ![DFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dfs_pred_multi_source_1-0.87.0.svg?)
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     algo::dfs_pred::{
///         DfsPred,
///         Step,
///     },
///     gen::Empty,
///     op::AddArc,
/// };
///
/// let mut digraph = Digraph::empty(8);
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
/// assert!(DfsPred::new(&digraph, &[3, 7]).eq([
///     Step { u: None, v: 7 },
///     Step { u: Some(7), v: 6 },
///     Step { u: Some(6), v: 5 },
///     Step { u: None, v: 3 },
///     Step { u: Some(3), v: 0 },
///     Step { u: Some(0), v: 1 },
///     Step { u: Some(1), v: 4 },
/// ]));
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DfsPred<'a, D> {
    digraph: &'a D,
    stack: Vec<Step>,
    visited: HashSet<usize>,
}

impl<'a, D> DfsPred<'a, D> {
    /// Constructs a new depth-first search.
    ///
    /// # Arguments
    ///
    /// * `digraph`: The digraph.
    /// * `sources`: The source vertices.
    pub fn new<'b, T>(digraph: &'a D, sources: T) -> Self
    where
        T: IntoIterator<Item = &'b usize>,
    {
        let mut stack = Vec::new();

        for &source in sources {
            stack.push(Step { u: None, v: source });
        }

        Self {
            digraph,
            stack,
            visited: HashSet::new(),
        }
    }
}

impl<'a, D> Iterator for DfsPred<'a, D>
where
    D: OutNeighbors,
{
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(step) = self.stack.pop() {
            let Step { v, .. } = step;

            if self.visited.insert(v) {
                for x in self
                    .digraph
                    .out_neighbors(v)
                    .filter(|x| !self.visited.contains(x))
                {
                    self.stack.push(Step { u: Some(v), v: x });
                }

                return Some(step);
            }
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

        assert!(DfsPred::new(&digraph, &[0]).eq([
            Step { u: None, v: 0 },
            Step { u: Some(0), v: 7 },
            Step { u: Some(7), v: 5 },
            Step { u: Some(5), v: 6 },
            Step { u: Some(0), v: 4 },
            Step { u: Some(4), v: 2 },
            Step { u: Some(2), v: 3 },
            Step { u: Some(0), v: 1 },
        ]));
    }

    #[test]
    fn iter_bang_jensen_34() {
        let digraph = bang_jensen_34();

        assert!(DfsPred::new(&digraph, &[0])
            .eq([Step { u: None, v: 0 }, Step { u: Some(0), v: 4 },]));
    }

    #[test]
    fn iter_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(DfsPred::new(&digraph, &[0]).eq([
            Step { u: None, v: 0 },
            Step { u: Some(0), v: 2 },
            Step { u: Some(2), v: 5 },
            Step { u: Some(2), v: 4 },
            Step { u: Some(4), v: 6 },
            Step { u: Some(2), v: 3 },
            Step { u: Some(2), v: 1 },
        ]));
    }

    #[test]
    fn iter_kattis_builddeps() {
        let digraph = kattis_builddeps();

        assert!(DfsPred::new(&digraph, &[0]).eq([
            Step { u: None, v: 0 },
            Step { u: Some(0), v: 4 },
            Step { u: Some(4), v: 1 },
            Step { u: Some(0), v: 3 },
        ]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_1() {
        let digraph = kattis_cantinaofbabel_1();

        assert!(DfsPred::new(&digraph, &[0]).eq([
            Step { u: None, v: 0 },
            Step { u: Some(0), v: 1 },
            Step { u: Some(1), v: 4 },
            Step { u: Some(4), v: 3 },
            Step { u: Some(3), v: 11 },
            Step { u: Some(11), v: 9 },
            Step { u: Some(9), v: 7 },
            Step { u: Some(3), v: 10 },
            Step { u: Some(10), v: 6 },
            Step { u: Some(6), v: 5 },
        ]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_2() {
        let digraph = kattis_cantinaofbabel_2();

        assert!(DfsPred::new(&digraph, &[0]).eq([
            Step { u: None, v: 0 },
            Step { u: Some(0), v: 1 },
            Step { u: Some(1), v: 7 },
            Step { u: Some(7), v: 2 },
            Step { u: Some(2), v: 5 },
            Step { u: Some(5), v: 6 },
            Step { u: Some(5), v: 3 },
            Step { u: Some(3), v: 4 },
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(DfsPred::new(&digraph, &[5]).eq([
            Step { u: None, v: 5 },
            Step { u: Some(5), v: 9 },
            Step { u: Some(9), v: 13 },
            Step { u: Some(13), v: 12 },
            Step { u: Some(5), v: 6 },
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(DfsPred::new(&digraph, &[5]).eq([
            Step { u: None, v: 5 },
            Step { u: Some(5), v: 9 },
            Step { u: Some(5), v: 6 },
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(DfsPred::new(&digraph, &[1]).eq([
            Step { u: None, v: 1 },
            Step { u: Some(1), v: 5 },
            Step { u: Some(5), v: 9 },
            Step { u: Some(9), v: 13 },
            Step { u: Some(13), v: 12 },
            Step { u: Some(5), v: 6 },
            Step { u: Some(6), v: 2 },
        ]));
    }
}

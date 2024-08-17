//! Depth-first search with distances
//!
//! Depth-first search is a digraph traversal algorithm that explores a digraph
//! by following a path as far as possible before backtracking.
//!
//! The time complexity is *O*(*v* + *a*).
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     algo::dfs_dist::{
//!         Dfs,
//!         Step,
//!     },
//!     gen::Empty,
//!     op::AddArc,
//! };
//!
//! // 0 -> {1, 2}
//! // 1 -> {4}
//! // 2 -> {3, 4}
//! // 3 -> {4}
//! // 4 -> {}
//!
//! let mut digraph = Digraph::empty(5);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 4);
//! digraph.add_arc(2, 3);
//! digraph.add_arc(2, 4);
//! digraph.add_arc(3, 4);
//!
//! assert!(Dfs::new(&digraph, 0).eq([
//!     Step { u: 0, dist: 0 },
//!     Step { u: 2, dist: 1 },
//!     Step { u: 4, dist: 2 },
//!     Step { u: 3, dist: 2 },
//!     Step { u: 1, dist: 1 },
//! ]));
//! ```

use {
    crate::op::{
        Order,
        OutNeighbors,
    },
    std::{
        collections::BTreeSet,
        vec,
    },
};

/// Depth-first search with distances.
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     algo::dfs_dist::{
///         Dfs,
///         Step,
///     },
///     gen::Empty,
///     op::AddArc,
/// };
///
/// // 0 -> {1, 2}
/// // 1 -> {4}
/// // 2 -> {3, 4}
/// // 3 -> {4}
/// // 4 -> {}
///
/// let mut digraph = Digraph::empty(5);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(0, 2);
/// digraph.add_arc(1, 4);
/// digraph.add_arc(2, 3);
/// digraph.add_arc(2, 4);
/// digraph.add_arc(3, 4);
///
/// assert!(Dfs::new(&digraph, 0).eq([
///     Step { u: 0, dist: 0 },
///     Step { u: 2, dist: 1 },
///     Step { u: 4, dist: 2 },
///     Step { u: 3, dist: 2 },
///     Step { u: 1, dist: 1 },
/// ]));
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Dfs<'a, D> {
    digraph: &'a D,
    stack: Vec<(usize, usize)>,
    visited: BTreeSet<usize>,
}

/// A step in the depth-first search.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Step {
    /// The current vertex.
    pub u: usize,
    /// The dist of `u` from the source vertex.
    pub dist: usize,
}

impl<'a, D> Dfs<'a, D> {
    /// Constructs a new depth-first search.
    ///
    /// # Arguments
    ///
    /// * `digraph`: The digraph.
    /// * `source`: The source vertex.
    pub fn new(digraph: &'a D, source: usize) -> Self
    where
        D: Order + OutNeighbors,
    {
        Self {
            digraph,
            stack: vec![(source, 0)],
            visited: BTreeSet::new(),
        }
    }
}

impl<'a, D> Iterator for Dfs<'a, D>
where
    D: OutNeighbors,
{
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((u, dist)) = self.stack.pop() {
            if self.visited.insert(u) {
                {
                    let dist = dist + 1;

                    for v in self
                        .digraph
                        .out_neighbors(u)
                        .filter(|v| !self.visited.contains(v))
                    {
                        self.stack.push((v, dist));
                    }
                }

                return Some(Step { u, dist });
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

        assert!(Dfs::new(&digraph, 0).eq([
            Step { u: 0, dist: 0 },
            Step { u: 7, dist: 1 },
            Step { u: 5, dist: 2 },
            Step { u: 6, dist: 3 },
            Step { u: 4, dist: 1 },
            Step { u: 2, dist: 2 },
            Step { u: 3, dist: 3 },
            Step { u: 1, dist: 1 }
        ]));
    }

    #[test]
    fn iter_bang_jensen_34() {
        let digraph = bang_jensen_34();

        assert!(Dfs::new(&digraph, 0)
            .eq([Step { u: 0, dist: 0 }, Step { u: 4, dist: 1 }]));
    }

    #[test]
    fn iter_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(Dfs::new(&digraph, 0).eq([
            Step { u: 0, dist: 0 },
            Step { u: 2, dist: 1 },
            Step { u: 5, dist: 2 },
            Step { u: 4, dist: 2 },
            Step { u: 6, dist: 3 },
            Step { u: 3, dist: 2 },
            Step { u: 1, dist: 2 }
        ]));
    }

    #[test]
    fn iter_kattis_builddeps() {
        let digraph = kattis_builddeps();

        assert!(Dfs::new(&digraph, 0).eq([
            Step { u: 0, dist: 0 },
            Step { u: 4, dist: 1 },
            Step { u: 1, dist: 2 },
            Step { u: 3, dist: 1 }
        ]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_1() {
        let digraph = kattis_cantinaofbabel_1();

        assert!(Dfs::new(&digraph, 0).eq([
            Step { u: 0, dist: 0 },
            Step { u: 1, dist: 1 },
            Step { u: 4, dist: 2 },
            Step { u: 3, dist: 3 },
            Step { u: 11, dist: 4 },
            Step { u: 9, dist: 5 },
            Step { u: 7, dist: 6 },
            Step { u: 10, dist: 4 },
            Step { u: 6, dist: 5 },
            Step { u: 5, dist: 6 }
        ]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_2() {
        let digraph = kattis_cantinaofbabel_2();

        assert!(Dfs::new(&digraph, 0).eq([
            Step { u: 0, dist: 0 },
            Step { u: 1, dist: 1 },
            Step { u: 7, dist: 2 },
            Step { u: 2, dist: 3 },
            Step { u: 5, dist: 4 },
            Step { u: 6, dist: 5 },
            Step { u: 3, dist: 5 },
            Step { u: 4, dist: 6 }
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(Dfs::new(&digraph, 5).eq([
            Step { u: 5, dist: 0 },
            Step { u: 9, dist: 1 },
            Step { u: 13, dist: 2 },
            Step { u: 12, dist: 3 },
            Step { u: 6, dist: 1 }
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(Dfs::new(&digraph, 5).eq([
            Step { u: 5, dist: 0 },
            Step { u: 9, dist: 1 },
            Step { u: 6, dist: 1 }
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(Dfs::new(&digraph, 5).eq([
            Step { u: 5, dist: 0 },
            Step { u: 9, dist: 1 },
            Step { u: 13, dist: 2 },
            Step { u: 12, dist: 3 },
            Step { u: 6, dist: 1 },
            Step { u: 2, dist: 2 },
            Step { u: 1, dist: 3 }
        ]));
    }
}

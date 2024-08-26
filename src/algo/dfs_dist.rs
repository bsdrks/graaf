//! Depth-first search with distances
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
//! Red marks the path starting at vertex `0` and `d` denotes the distance.
//!
//! ![BFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_dist_1.svg?)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     algo::bfs_dist::{
//!         Bfs,
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
//! assert!(Bfs::new(&digraph, &[0]).eq([
//!     Step { u: 0, dist: 0 },
//!     Step { u: 1, dist: 1 },
//!     Step { u: 2, dist: 2 },
//!     Step { u: 4, dist: 2 },
//!     Step { u: 5, dist: 3 },
//! ]));
//! ```
//!
//! ## Multiple sources
//!
//! Red marks the path starting at vertex `3` and blue the path starting at
//! vertex `7`.
//!
//! ![DFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dfs_dist_multi_source_1.svg?)
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
//! assert!(Dfs::new(&digraph, &[3, 7]).eq([
//!     Step { u: 7, dist: 0 },
//!     Step { u: 6, dist: 1 },
//!     Step { u: 5, dist: 2 },
//!     Step { u: 3, dist: 0 },
//!     Step { u: 0, dist: 1 },
//!     Step { u: 1, dist: 2 },
//!     Step { u: 4, dist: 3 },
//! ]));
//! ```

use {
    crate::op::{
        Order,
        OutNeighbors,
    },
    std::collections::BTreeSet,
};

/// Depth-first search with distances.
///
/// # Examples
///
/// ## Single source
///
/// Red marks the path starting at vertex `0` and `d` denotes the distance.
///
/// ![BFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_dist_1.svg?)
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     algo::bfs_dist::{
///         Bfs,
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
/// assert!(Bfs::new(&digraph, &[0]).eq([
///     Step { u: 0, dist: 0 },
///     Step { u: 1, dist: 1 },
///     Step { u: 2, dist: 2 },
///     Step { u: 4, dist: 2 },
///     Step { u: 5, dist: 3 },
/// ]));
/// ```
///
/// ## Multiple sources
///
/// Red marks the path starting at vertex `3` and blue the path starting at
/// vertex `7`.
///
/// ![DFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dfs_dist_multi_source_1.svg?)
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
/// assert!(Dfs::new(&digraph, &[3, 7]).eq([
///     Step { u: 7, dist: 0 },
///     Step { u: 6, dist: 1 },
///     Step { u: 5, dist: 2 },
///     Step { u: 3, dist: 0 },
///     Step { u: 0, dist: 1 },
///     Step { u: 1, dist: 2 },
///     Step { u: 4, dist: 3 },
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
    /// * `sources`: The source vertices.
    pub fn new<'b, T>(digraph: &'a D, sources: T) -> Self
    where
        D: Order + OutNeighbors,
        T: IntoIterator<Item = &'b usize>,
    {
        Self {
            digraph,
            stack: sources.into_iter().map(|&u| (u, 0)).collect(),
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

        assert!(Dfs::new(&digraph, &[0]).eq([
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

        assert!(Dfs::new(&digraph, &[0])
            .eq([Step { u: 0, dist: 0 }, Step { u: 4, dist: 1 }]));
    }

    #[test]
    fn iter_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(Dfs::new(&digraph, &[0]).eq([
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

        assert!(Dfs::new(&digraph, &[0]).eq([
            Step { u: 0, dist: 0 },
            Step { u: 4, dist: 1 },
            Step { u: 1, dist: 2 },
            Step { u: 3, dist: 1 }
        ]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_1() {
        let digraph = kattis_cantinaofbabel_1();

        assert!(Dfs::new(&digraph, &[0]).eq([
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

        assert!(Dfs::new(&digraph, &[0]).eq([
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

        assert!(Dfs::new(&digraph, &[5]).eq([
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

        assert!(Dfs::new(&digraph, &[5]).eq([
            Step { u: 5, dist: 0 },
            Step { u: 9, dist: 1 },
            Step { u: 6, dist: 1 }
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(Dfs::new(&digraph, &[5]).eq([
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

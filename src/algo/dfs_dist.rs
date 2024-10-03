//! Depth-first search with distances.
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
//! The path from vertex `0` is red. `d` denotes the distances.
//!
//! ![A digraph and the distances between the source vertex and the other vertices along the depth-first traversal](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dfs_dist_1-0.87.4.svg?)
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     DfsDist,
//!     Empty,
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
//! assert!(DfsDist::new(&digraph, &[0]).eq([
//!     (0, 0),
//!     (1, 1),
//!     (4, 2),
//!     (2, 2),
//!     (5, 3),
//! ]));
//! ```
//!
//! ## Multiple sources
//!
//! The path from vertex `3` is red. The path from vertex `7` is blue.
//!
//! ![A digraph and the distances between the source vertices and the other vertices along the depth-first traversal](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dfs_dist_multi_source_1-0.87.4.svg?)
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     DfsDist,
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
//! assert!(DfsDist::new(&digraph, &[3, 7]).eq([
//!     (7, 0),
//!     (6, 1),
//!     (5, 2),
//!     (3, 0),
//!     (0, 1),
//!     (1, 2),
//!     (4, 3),
//! ]));
//! ```

use {
    crate::OutNeighbors,
    std::collections::HashSet,
};

type Step = (usize, usize);

/// Depth-first search with distances.
///
/// # Examples
///
/// ## Single source
///
/// The path from vertex `0` is red. `d` denotes the distances.
///
/// ![A digraph and the distances between the source vertex and the other vertices along the depth-first traversal](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dfs_dist_1-0.87.4.svg?)
///
/// ```
/// use graaf::{
///     AddArc,
///     AdjacencyList,
///     DfsDist,
///     Empty,
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
/// assert!(DfsDist::new(&digraph, &[0]).eq([
///     (0, 0),
///     (1, 1),
///     (4, 2),
///     (2, 2),
///     (5, 3),
/// ]));
/// ```
///
/// ## Multiple sources
///
/// The path from vertex `3` is red. The path from vertex `7` is blue.
///
/// ![A digraph and the distances between the source vertices and the other vertices along the depth-first traversal](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dfs_dist_multi_source_1-0.87.4.svg?)
///
/// ```
/// use graaf::{
///     AddArc,
///     AdjacencyList,
///     DfsDist,
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
/// assert!(DfsDist::new(&digraph, &[3, 7]).eq([
///     (7, 0),
///     (6, 1),
///     (5, 2),
///     (3, 0),
///     (0, 1),
///     (1, 2),
///     (4, 3),
/// ]));
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DfsDist<'a, D> {
    digraph: &'a D,
    stack: Vec<Step>,
    visited: HashSet<usize>,
}

impl<'a, D> DfsDist<'a, D> {
    /// Construct a new depth-first search.
    ///
    /// # Arguments
    ///
    /// * `digraph`: The digraph.
    /// * `sources`: The source vertices.
    #[must_use]
    pub fn new<'b, T>(digraph: &'a D, sources: T) -> Self
    where
        T: IntoIterator<Item = &'b usize>,
    {
        Self {
            digraph,
            stack: sources.into_iter().map(|&u| (u, 0)).collect(),
            visited: HashSet::new(),
        }
    }
}

impl<'a, D> Iterator for DfsDist<'a, D>
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

                return Some((u, dist));
            }
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
    };

    #[test]
    fn iter_bang_jensen_196() {
        let digraph = bang_jensen_196();

        assert!(DfsDist::new(&digraph, &[0]).eq([
            (0, 0),
            (7, 1),
            (5, 2),
            (6, 3),
            (4, 1),
            (2, 2),
            (3, 3),
            (1, 1)
        ]));
    }

    #[test]
    fn iter_bang_jensen_34() {
        let digraph = bang_jensen_34();

        assert!(DfsDist::new(&digraph, &[0]).eq([(0, 0), (4, 1)]));
    }

    #[test]
    fn iter_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(DfsDist::new(&digraph, &[0]).eq([
            (0, 0),
            (2, 1),
            (5, 2),
            (4, 2),
            (6, 3),
            (3, 2),
            (1, 2)
        ]));
    }

    #[test]
    fn iter_kattis_builddeps() {
        let digraph = kattis_builddeps();

        assert!(DfsDist::new(&digraph, &[0]).eq([
            (0, 0),
            (4, 1),
            (1, 2),
            (3, 1)
        ]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_1() {
        let digraph = kattis_cantinaofbabel_1();

        assert!(DfsDist::new(&digraph, &[0]).eq([
            (0, 0),
            (1, 1),
            (4, 2),
            (3, 3),
            (11, 4),
            (9, 5),
            (7, 6),
            (10, 4),
            (6, 5),
            (5, 6)
        ]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_2() {
        let digraph = kattis_cantinaofbabel_2();

        assert!(DfsDist::new(&digraph, &[0]).eq([
            (0, 0),
            (1, 1),
            (7, 2),
            (2, 3),
            (5, 4),
            (6, 5),
            (3, 5),
            (4, 6)
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(DfsDist::new(&digraph, &[5]).eq([
            (5, 0),
            (9, 1),
            (13, 2),
            (12, 3),
            (6, 1)
        ]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(DfsDist::new(&digraph, &[5]).eq([(5, 0), (9, 1), (6, 1)]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(DfsDist::new(&digraph, &[5]).eq([
            (5, 0),
            (9, 1),
            (13, 2),
            (12, 3),
            (6, 1),
            (2, 2),
            (1, 3)
        ]));
    }
}

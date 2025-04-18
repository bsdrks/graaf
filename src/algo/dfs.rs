//! Depth-first search.
//!
//! Depth-first search is a digraph traversal algorithm that explores a digraph
//! by following a path as far as possible before backtracking.
//!
//! The time complexity is `O(v + a)`, where `v` is the digraph's order and `a`
//! is the digraph's size.
//!
//! # Examples
//!
//! ## Single source
//!
//! The path from vertex `0` is red. `t` denotes the iteration indices.
//!
//! ![A digraph and the distances between the source vertex and the vertices along the depth-first traversal](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dfs_1-0.87.4.svg?)
//!
//! ```
//! use {
//!     graaf::{
//!         AddArc,
//!         AdjacencyList,
//!         Dfs,
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
//! assert!(Dfs::new(&digraph, once(0)).eq([0, 1, 4, 2, 5]));
//! ```
//!
//! ## Multiple sources
//!
//! The path from vertex `3` is red. The path from vertex `7` is blue.
//!
//! ![A digraph and the distances between the source vertices and the other vertices along the depth-first traversal](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dfs_multi_source_1-0.87.4.svg?)
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Dfs,
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
//! assert!(Dfs::new(&digraph, [3, 7].into_iter()).eq([7, 6, 5, 3, 0, 1, 4]));
//! ```

use crate::{
    Order,
    OutNeighbors,
};

/// Depth-first search.
///
/// # Examples
///
/// ## Single source
///
/// The path from vertex `0` is red. `t` denotes the iteration indices.
///
/// ![A digraph and the distances between the source vertex and the vertices along the depth-first traversal](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dfs_1-0.87.4.svg?)
///
/// ```
/// use {
///     graaf::{
///         AddArc,
///         AdjacencyList,
///         Dfs,
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
/// assert!(Dfs::new(&digraph, once(0)).eq([0, 1, 4, 2, 5]));
/// ```
///
/// ## Multiple sources
///
/// The path from vertex `3` is red. The path from vertex `7` is blue.
///
/// ![A digraph and the distances between the source vertices and the other vertices along the depth-first traversal](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dfs_multi_source_1-0.87.4.svg?)
///
/// ```
/// use graaf::{
///     AddArc,
///     AdjacencyList,
///     Dfs,
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
/// assert!(Dfs::new(&digraph, [3, 7].into_iter()).eq([7, 6, 5, 3, 0, 1, 4]));
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dfs<'a, D> {
    digraph: &'a D,
    stack: Vec<usize>,
    visited: Vec<bool>,
}

impl<'a, D> Dfs<'a, D> {
    /// Construct a new depth-first search.
    ///
    /// # Arguments
    ///
    /// * `digraph`: The digraph.
    /// * `sources`: The source vertices.
    #[must_use]
    pub fn new<T>(digraph: &'a D, sources: T) -> Self
    where
        D: Order,
        T: Iterator<Item = usize>,
    {
        Self {
            digraph,
            stack: sources.collect(),
            visited: vec![false; digraph.order()],
        }
    }
}

impl<D> Iterator for Dfs<'_, D>
where
    D: OutNeighbors,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let u = self.stack.pop()?;
        let visited_ptr = self.visited.as_mut_ptr();

        if unsafe { *visited_ptr.add(u) } {
            return None;
        }

        unsafe {
            *visited_ptr.add(u) = true;
        }

        for v in self.digraph.out_neighbors(u) {
            if !unsafe { *visited_ptr.add(v) } {
                self.stack.push(v);
            }
        }

        Some(u)
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

        assert!(Dfs::new(&digraph, once(0)).eq([0, 7, 5, 6, 4, 2, 3, 1]));
    }

    #[test]
    fn iter_bang_jensen_34() {
        let digraph = bang_jensen_34();

        assert!(Dfs::new(&digraph, once(0)).eq([0, 4]));
    }

    #[test]
    fn iter_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(Dfs::new(&digraph, once(0)).eq([0, 2, 5, 4, 6, 3, 1]));
    }

    #[test]
    fn iter_kattis_builddeps() {
        let digraph = kattis_builddeps();

        assert!(Dfs::new(&digraph, once(0)).eq([0, 4, 1, 3]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_1() {
        let digraph = kattis_cantinaofbabel_1();

        assert!(
            Dfs::new(&digraph, once(0)).eq([0, 1, 4, 3, 11, 9, 7, 10, 6, 5])
        );
    }

    #[test]
    fn iter_kattis_cantinaofbabel_2() {
        let digraph = kattis_cantinaofbabel_2();

        assert!(Dfs::new(&digraph, once(0)).eq([0, 1, 7, 2, 5, 6, 3, 4]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(Dfs::new(&digraph, once(5)).eq([5, 9, 13, 12, 6]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(Dfs::new(&digraph, once(5)).eq([5, 9, 6]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(Dfs::new(&digraph, once(1)).eq([1, 5, 9, 13, 12, 6, 2]));
    }
}

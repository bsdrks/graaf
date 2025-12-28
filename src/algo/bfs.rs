//! Breadth-first search.
//!
//! Breadth-first search explores an unweighted digraph's vertices in the order
//! of their distance from a source.
//!
//! The time complexity is `O(v + a)`, where `v` is the digraph's order and
//! `a` is the digraph's size.
//!
//! # Examples
//!
//! ## Single source
//!
//! The path from vertex `0` is red. `t` denotes the iteration indices.
//!
//! ![A digraph and the breadth-first search from vertex `0`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_1-0.87.4.svg?)
//!
//! ```
//! use {
//!     graaf::{
//!         AddArc,
//!         AdjacencyList,
//!         Bfs,
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
//! assert!(Bfs::new(&digraph, once(0)).eq([0, 1, 2, 4, 5]));
//! ```
//!
//! ## Multiple sources
//!
//! The path from vertex `0` is red. The path from vertex `1` is blue.
//!
//! ![A digraph and a breadth-first traversal from two source vertices](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_multi_source_1-0.87.4.svg?)
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Bfs,
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
//! assert!(
//!     Bfs::new(&digraph, [3, 7].into_iter()).eq([3, 7, 0, 6, 1, 5, 2, 4])
//! );
//! ```

use {
    crate::{
        Order,
        OutNeighbors,
    },
    std::collections::VecDeque,
};

/// Breadth-first search.
///
/// Breadth-first search explores an unweighted digraph's vertices in the order
/// of their distance from a source.
///
/// # Examples
///
/// The path from vertex `0` is red. `t` denotes the iteration indices.
///
/// ![A digraph and the breadth-first search from vertex `0`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_1-0.87.4.svg?)
///
/// ```
/// use {
///     graaf::{
///         AddArc,
///         AdjacencyList,
///         Bfs,
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
/// assert!(Bfs::new(&digraph, once(0)).eq([0, 1, 2, 4, 5]));
/// ```
///
/// ## Multiple sources
///
/// The path from vertex `0` is red. The path from vertex `1` is blue.
///
/// ![A digraph and a breadth-first traversal from two source vertices](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_multi_source_1-0.87.4.svg?)
///
/// ```
/// use graaf::{
///     AddArc,
///     AdjacencyList,
///     Bfs,
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
///     Bfs::new(&digraph, [3, 7].into_iter()).eq([3, 7, 0, 6, 1, 5, 2, 4])
/// );
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bfs<'a, D> {
    digraph: &'a D,
    queue: VecDeque<usize>,
    visited: Vec<bool>,
}

impl<'a, D> Bfs<'a, D> {
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
            queue.push_back(u);

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
}

impl<D> Iterator for Bfs<'_, D>
where
    D: OutNeighbors,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let u = self.queue.pop_front()?;
        let visited_ptr = self.visited.as_mut_ptr();

        for v in self.digraph.out_neighbors(u) {
            let visited_v = unsafe { visited_ptr.add(v) };

            unsafe {
                if !*visited_v {
                    *visited_v = true;

                    self.queue.push_back(v);
                }
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
    fn iter_bang_jensen_196() {
        let digraph = bang_jensen_196();

        assert!(Bfs::new(&digraph, once(0)).eq([0, 1, 4, 7, 2, 5, 3, 6]));
    }

    #[test]
    fn iter_bang_jensen_34() {
        let digraph = bang_jensen_34();

        assert!(Bfs::new(&digraph, once(0)).eq([0, 4]));
    }

    #[test]
    fn iter_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(Bfs::new(&digraph, once(0)).eq([0, 1, 2, 3, 4, 5, 6]));
    }

    #[test]
    fn iter_kattis_builddeps() {
        let digraph = kattis_builddeps();

        assert!(Bfs::new(&digraph, once(0)).eq([0, 3, 4, 1]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_1() {
        let digraph = kattis_cantinaofbabel_1();

        assert!(
            Bfs::new(&digraph, once(0))
                .eq([0, 1, 2, 4, 3, 5, 7, 10, 11, 6, 9])
        );
    }

    #[test]
    fn iter_kattis_cantinaofbabel_2() {
        let digraph = kattis_cantinaofbabel_2();

        assert!(Bfs::new(&digraph, once(0)).eq([0, 1, 7, 2, 5, 3, 6, 4]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(Bfs::new(&digraph, once(5)).eq([5, 6, 9, 13, 12]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(Bfs::new(&digraph, once(5)).eq([5, 6, 9]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(Bfs::new(&digraph, once(1)).eq([1, 2, 5, 6, 9, 13, 12]));
    }
}

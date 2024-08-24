//! Breadth-first search
//!
//! Breadth-first search explores the vertices of an unweighted digraph in
//! order of their distance from a source. The time complexity is
//! *O*(*v* + *a*).
//!
//! # Examples
//!
//! ## Single source
//!
//! Red marks the path:
//!
//! ![BFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_1.svg?)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     algo::bfs::Bfs,
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
//! assert!(Bfs::new(&digraph, &[0]).eq([0, 1, 2, 4, 5]));
//! ```
//!
//! ## Multiple sources
//!
//! Red marks the path starting at `3` and blue the path starting at `7`:
//!
//! ![BFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_multi_source_1.svg?)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     algo::bfs::Bfs,
//!     gen::Empty,
//!     op::AddArc,
//! };
//!
//! let mut digraph = Digraph::empty(8);
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
//! assert!(Bfs::new(&digraph, &[3, 7]).eq([3, 7, 0, 6, 1, 5, 2, 4]));
//! ```

use {
    crate::op::OutNeighbors,
    std::collections::{
        BTreeSet,
        VecDeque,
    },
};

/// Breadth-first search.
///
/// # Examples
///
/// Red marks the path:
///
/// ![BFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_1.svg?)
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     algo::bfs::Bfs,
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
/// assert!(Bfs::new(&digraph, &[0]).eq([0, 1, 2, 4, 5]));
/// ```
///
/// ## Multiple sources
///
/// Red marks the path starting at `3` and blue the path starting at `7`:
///
/// ![BFS](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bfs_multi_source_1.svg?)
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     algo::bfs::Bfs,
///     gen::Empty,
///     op::AddArc,
/// };
///
/// let mut digraph = Digraph::empty(8);
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
/// assert!(Bfs::new(&digraph, &[3, 7]).eq([3, 7, 0, 6, 1, 5, 2, 4]));
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bfs<'a, D> {
    digraph: &'a D,
    queue: VecDeque<usize>,
    visited: BTreeSet<usize>,
}

impl<'a, D> Bfs<'a, D> {
    /// Constructs a new breadth-first search.
    ///
    /// # Arguments
    ///
    /// * `digraph`: The digraph.
    /// * `sources`: The source vertices.
    pub fn new<'b, T>(digraph: &'a D, sources: T) -> Self
    where
        T: IntoIterator<Item = &'b usize>,
    {
        let mut queue = VecDeque::new();
        let mut visited = BTreeSet::new();

        for &source in sources {
            queue.push_back(source);
            let _ = visited.insert(source);
        }

        Self {
            digraph,
            queue,
            visited,
        }
    }
}

impl<'a, D> Iterator for Bfs<'a, D>
where
    D: OutNeighbors,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(u) = self.queue.pop_front() {
            for v in self
                .digraph
                .out_neighbors(u)
                .filter(|v| self.visited.insert(*v))
            {
                self.queue.push_back(v);
            }

            return Some(u);
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

        assert!(Bfs::new(&digraph, &[0]).eq([0, 1, 4, 7, 2, 5, 3, 6]));
    }

    #[test]
    fn iter_bang_jensen_34() {
        let digraph = bang_jensen_34();

        assert!(Bfs::new(&digraph, &[0]).eq([0, 4]));
    }

    #[test]
    fn iter_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(Bfs::new(&digraph, &[0]).eq([0, 1, 2, 3, 4, 5, 6]));
    }

    #[test]
    fn iter_kattis_builddeps() {
        let digraph = kattis_builddeps();

        assert!(Bfs::new(&digraph, &[0]).eq([0, 3, 4, 1]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_1() {
        let digraph = kattis_cantinaofbabel_1();

        assert!(
            Bfs::new(&digraph, &[0]).eq([0, 1, 2, 4, 3, 5, 7, 10, 11, 6, 9])
        );
    }

    #[test]
    fn iter_kattis_cantinaofbabel_2() {
        let digraph = kattis_cantinaofbabel_2();

        assert!(Bfs::new(&digraph, &[0]).eq([0, 1, 7, 2, 5, 3, 6, 4]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(Bfs::new(&digraph, &[5]).eq([5, 6, 9, 13, 12]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(Bfs::new(&digraph, &[5]).eq([5, 6, 9]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(Bfs::new(&digraph, &[1]).eq([1, 2, 5, 6, 9, 13, 12]));
    }
}

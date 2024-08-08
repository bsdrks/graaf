//! Depth-first search
//!
//! Depth-first search is a digraph traversal algorithm that explores a digraph
//! by following a path as far as possible before backtracking.
//!
//! The time complexity is *O*(*v* + *a*).

use {
    crate::op::{
        Order,
        OutNeighbors,
    },
    std::collections::BTreeSet,
};

/// Depth-first search.
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     algo::dfs::Dfs,
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
/// assert!(Dfs::new(&digraph, 0).eq([0, 2, 4, 3, 1]));
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Dfs<'a, D> {
    digraph: &'a D,
    stack: Vec<usize>,
    visited: BTreeSet<usize>,
}

impl<'a, D> Iterator for Dfs<'a, D>
where
    D: OutNeighbors,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(u) = self.stack.pop() {
            if self.visited.insert(u) {
                for v in self
                    .digraph
                    .out_neighbors(u)
                    .filter(|v| !self.visited.contains(v))
                {
                    self.stack.push(v);
                }

                return Some(u);
            }
        }

        None
    }
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
            stack: vec![source],
            visited: BTreeSet::new(),
        }
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

        assert!(Dfs::new(&digraph, 0).eq([0, 7, 5, 6, 4, 2, 3, 1]));
    }

    #[test]
    fn iter_bang_jensen_34() {
        let digraph = bang_jensen_34();

        assert!(Dfs::new(&digraph, 0).eq([0, 4]));
    }

    #[test]
    fn iter_bang_jensen_94() {
        let digraph = bang_jensen_94();

        assert!(Dfs::new(&digraph, 0).eq([0, 2, 5, 4, 6, 3, 1]));
    }

    #[test]
    fn iter_kattis_builddeps() {
        let digraph = kattis_builddeps();

        assert!(Dfs::new(&digraph, 0).eq([0, 4, 1, 3]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_1() {
        let digraph = kattis_cantinaofbabel_1();

        assert!(Dfs::new(&digraph, 0).eq([0, 1, 4, 3, 11, 9, 7, 10, 6, 5]));
    }

    #[test]
    fn iter_kattis_cantinaofbabel_2() {
        let digraph = kattis_cantinaofbabel_2();

        assert!(Dfs::new(&digraph, 0).eq([0, 1, 7, 2, 5, 6, 3, 4]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_1() {
        let digraph = kattis_escapewallmaria_1();

        assert!(Dfs::new(&digraph, 5).eq([5, 9, 13, 12, 6]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_2() {
        let digraph = kattis_escapewallmaria_2();

        assert!(Dfs::new(&digraph, 5).eq([5, 9, 6]));
    }

    #[test]
    fn iter_kattis_escapewallmaria_3() {
        let digraph = kattis_escapewallmaria_3();

        assert!(Dfs::new(&digraph, 1).eq([1, 5, 9, 13, 12, 6, 2]));
    }
}

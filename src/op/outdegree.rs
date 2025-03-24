//! Return a vertex's outdegree.
//!
//! The outdegree is the digraph's size incident out of a vertex.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     Outdegree,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//!
//! assert_eq!(digraph.outdegree(0), 2);
//! assert_eq!(digraph.outdegree(1), 1);
//! assert_eq!(digraph.outdegree(2), 0);
//! ```
#![doc(alias = "out_degree")]

use crate::Vertices;

/// Vertex outdegree
pub trait Outdegree {
    /// Return the vertex's outdegree.
    ///
    /// # Arguments
    ///
    /// * `u`: The vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     Outdegree,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 0);
    /// digraph.add_arc(2, 1);
    ///
    /// assert_eq!(digraph.outdegree(0), 2);
    /// assert_eq!(digraph.outdegree(1), 1);
    /// assert_eq!(digraph.outdegree(2), 1);
    /// ```
    #[doc(alias = "out_degree")]
    #[must_use]
    fn outdegree(&self, u: usize) -> usize;

    /// Check whether a vertex is a digraph's sink.
    ///
    /// A sink is a vertex without out-neighbors.
    ///
    /// # Arguments
    ///
    /// * `u`: The vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     Outdegree,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 0);
    ///
    /// assert!(!digraph.is_sink(0));
    /// assert!(!digraph.is_sink(1));
    /// assert!(digraph.is_sink(2));
    /// ```
    #[must_use]
    fn is_sink(&self, u: usize) -> bool {
        self.outdegree(u) == 0
    }

    /// Return the digraph's maximum outdegree.
    ///
    /// # Examples
    ///
    /// The maximum outdegree of this digraph is `3`. The vertex with the
    /// maximum outdegree is red.
    ///
    /// ![A digraph and its maximum outdegree](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/max_outdegree-0.88.5.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     Outdegree,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(4);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(0, 3);
    /// digraph.add_arc(1, 0);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(2, 0);
    /// digraph.add_arc(3, 0);
    ///
    /// assert_eq!(digraph.max_outdegree(), 3);
    /// ```
    #[must_use]
    fn max_outdegree(&self) -> usize
    where
        Self: Vertices,
    {
        self.vertices()
            .map(|u| self.outdegree(u))
            .max()
            .unwrap_or(0)
    }

    /// Return the digraph's minimum outdegree.
    ///
    /// # Examples
    ///
    /// The minimum outdegree of this digraph is `1`. The vertices with the
    /// minimum outdegree are red.
    ///
    /// ![A digraph and its minimum outdegree](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/min_outdegree-0.88.5.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     Outdegree,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(4);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(0, 3);
    /// digraph.add_arc(1, 0);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(2, 0);
    /// digraph.add_arc(3, 0);
    ///
    /// assert_eq!(digraph.min_outdegree(), 1);
    /// ```
    #[must_use]
    fn min_outdegree(&self) -> usize
    where
        Self: Vertices,
    {
        self.vertices()
            .map(|u| self.outdegree(u))
            .min()
            .unwrap_or(0)
    }
}

/// `Outdegree` tests
#[macro_export]
macro_rules! test_outdegree {
    ($type:ty, $fixture:path) => {
        use $fixture::{
            bang_jensen_196,
            bang_jensen_34,
            bang_jensen_94,
            kattis_builddeps,
            kattis_cantinaofbabel_1,
            kattis_cantinaofbabel_2,
            kattis_escapewallmaria_1,
            kattis_escapewallmaria_2,
            kattis_escapewallmaria_3,
        };

        #[test]
        #[should_panic(expected = "u = 1 isn't in the digraph")]
        fn is_sink_out_of_bounds() {
            let _ = <$type>::trivial().is_sink(1);
        }

        #[test]
        fn max_outdegree_bang_jensen_196() {
            assert_eq!(bang_jensen_196().max_outdegree(), 3);
        }

        #[test]
        fn max_outdegree_bang_jensen_34() {
            assert_eq!(bang_jensen_34().max_outdegree(), 3);
        }

        #[test]
        fn max_outdegree_bang_jensen_94() {
            assert_eq!(bang_jensen_94().max_outdegree(), 4);
        }

        #[test]
        fn max_outdegree_kattis_builddeps() {
            assert_eq!(kattis_builddeps().max_outdegree(), 3);
        }

        #[test]
        fn max_outdegree_kattis_cantinaofbabel_1() {
            assert_eq!(kattis_cantinaofbabel_1().max_outdegree(), 6);
        }

        #[test]
        fn max_outdegree_kattis_cantinaofbabel_2() {
            assert_eq!(kattis_cantinaofbabel_2().max_outdegree(), 3);
        }

        #[test]
        fn max_outdegree_kattis_escapewallmaria_1() {
            assert_eq!(kattis_escapewallmaria_1().max_outdegree(), 2);
        }

        #[test]
        fn max_outdegree_kattis_escapewallmaria_2() {
            assert_eq!(kattis_escapewallmaria_2().max_outdegree(), 2);
        }

        #[test]
        fn max_outdegree_kattis_escapewallmaria_3() {
            assert_eq!(kattis_escapewallmaria_3().max_outdegree(), 3);
        }

        #[test]
        fn min_outdegree_bang_jensen_196() {
            assert_eq!(bang_jensen_196().min_outdegree(), 1);
        }

        #[test]
        fn min_outdegree_bang_jensen_34() {
            assert_eq!(bang_jensen_34().min_outdegree(), 0);
        }

        #[test]
        fn min_outdegree_bang_jensen_94() {
            assert_eq!(bang_jensen_94().min_outdegree(), 0);
        }

        #[test]
        fn min_outdegree_kattis_builddeps() {
            assert_eq!(kattis_builddeps().min_outdegree(), 0);
        }

        #[test]
        fn min_outdegree_kattis_cantinaofbabel_1() {
            assert_eq!(kattis_cantinaofbabel_1().min_outdegree(), 1);
        }

        #[test]
        fn min_outdegree_kattis_cantinaofbabel_2() {
            assert_eq!(kattis_cantinaofbabel_2().min_outdegree(), 1);
        }

        #[test]
        fn min_outdegree_kattis_escapewallmaria_1() {
            assert_eq!(kattis_escapewallmaria_1().min_outdegree(), 0);
        }

        #[test]
        fn min_outdegree_kattis_escapewallmaria_2() {
            assert_eq!(kattis_escapewallmaria_2().min_outdegree(), 0);
        }

        #[test]
        fn min_outdegree_kattis_escapewallmaria_3() {
            assert_eq!(kattis_escapewallmaria_3().min_outdegree(), 0);
        }

        #[test]
        fn outdegree_bang_jensen_196() {
            let digraph = bang_jensen_196();

            assert_eq!(digraph.outdegree(0), 3);
            assert_eq!(digraph.outdegree(1), 3);
            assert_eq!(digraph.outdegree(2), 1);
            assert_eq!(digraph.outdegree(3), 2);
            assert_eq!(digraph.outdegree(4), 1);
            assert_eq!(digraph.outdegree(5), 1);
            assert_eq!(digraph.outdegree(6), 1);
            assert_eq!(digraph.outdegree(7), 1);
        }

        #[test]
        fn outdegree_bang_jensen_34() {
            let digraph = bang_jensen_34();

            assert_eq!(digraph.outdegree(0), 1);
            assert_eq!(digraph.outdegree(1), 1);
            assert_eq!(digraph.outdegree(2), 3);
            assert_eq!(digraph.outdegree(3), 0);
            assert_eq!(digraph.outdegree(4), 0);
            assert_eq!(digraph.outdegree(5), 1);
        }

        #[test]
        fn outdegree_bang_jensen_94() {
            let digraph = bang_jensen_94();

            assert_eq!(digraph.outdegree(0), 2);
            assert_eq!(digraph.outdegree(1), 1);
            assert_eq!(digraph.outdegree(2), 4);
            assert_eq!(digraph.outdegree(3), 1);
            assert_eq!(digraph.outdegree(4), 1);
            assert_eq!(digraph.outdegree(5), 0);
            assert_eq!(digraph.outdegree(6), 0);
        }

        #[test]
        fn outdegree_kattis_builddeps() {
            let digraph = kattis_builddeps();

            assert_eq!(digraph.outdegree(0), 2);
            assert_eq!(digraph.outdegree(1), 0);
            assert_eq!(digraph.outdegree(2), 3);
            assert_eq!(digraph.outdegree(3), 1);
            assert_eq!(digraph.outdegree(4), 1);
            assert_eq!(digraph.outdegree(5), 1);
        }

        #[test]
        fn outdegree_kattis_cantinaofbabel_1() {
            let digraph = kattis_cantinaofbabel_1();

            assert_eq!(digraph.outdegree(0), 1);
            assert_eq!(digraph.outdegree(1), 3);
            assert_eq!(digraph.outdegree(2), 1);
            assert_eq!(digraph.outdegree(3), 6);
            assert_eq!(digraph.outdegree(4), 1);
            assert_eq!(digraph.outdegree(5), 1);
            assert_eq!(digraph.outdegree(6), 2);
            assert_eq!(digraph.outdegree(7), 1);
            assert_eq!(digraph.outdegree(8), 2);
            assert_eq!(digraph.outdegree(9), 2);
            assert_eq!(digraph.outdegree(10), 1);
            assert_eq!(digraph.outdegree(11), 1);
        }

        #[test]
        fn outdegree_kattis_cantinaofbabel_2() {
            let digraph = kattis_cantinaofbabel_2();

            assert_eq!(digraph.outdegree(0), 1);
            assert_eq!(digraph.outdegree(1), 2);
            assert_eq!(digraph.outdegree(2), 3);
            assert_eq!(digraph.outdegree(3), 1);
            assert_eq!(digraph.outdegree(4), 1);
            assert_eq!(digraph.outdegree(5), 2);
            assert_eq!(digraph.outdegree(6), 1);
            assert_eq!(digraph.outdegree(7), 1);
            assert_eq!(digraph.outdegree(8), 3);
            assert_eq!(digraph.outdegree(9), 1);
            assert_eq!(digraph.outdegree(10), 2);
            assert_eq!(digraph.outdegree(11), 1);
        }

        #[test]
        fn outdegree_kattis_escapewallmaria_1() {
            let digraph = kattis_escapewallmaria_1();

            assert_eq!(digraph.outdegree(0), 0);
            assert_eq!(digraph.outdegree(1), 0);
            assert_eq!(digraph.outdegree(2), 0);
            assert_eq!(digraph.outdegree(3), 0);
            assert_eq!(digraph.outdegree(4), 0);
            assert_eq!(digraph.outdegree(5), 2);
            assert_eq!(digraph.outdegree(6), 1);
            assert_eq!(digraph.outdegree(7), 0);
            assert_eq!(digraph.outdegree(8), 0);
            assert_eq!(digraph.outdegree(9), 2);
            assert_eq!(digraph.outdegree(10), 0);
            assert_eq!(digraph.outdegree(11), 0);
            assert_eq!(digraph.outdegree(12), 0);
            assert_eq!(digraph.outdegree(13), 2);
            assert_eq!(digraph.outdegree(14), 0);
            assert_eq!(digraph.outdegree(15), 0);
        }

        #[test]
        fn outdegree_kattis_escapewallmaria_2() {
            let digraph = kattis_escapewallmaria_2();

            assert_eq!(digraph.outdegree(0), 0);
            assert_eq!(digraph.outdegree(1), 0);
            assert_eq!(digraph.outdegree(2), 0);
            assert_eq!(digraph.outdegree(3), 0);
            assert_eq!(digraph.outdegree(4), 0);
            assert_eq!(digraph.outdegree(5), 2);
            assert_eq!(digraph.outdegree(6), 1);
            assert_eq!(digraph.outdegree(7), 0);
            assert_eq!(digraph.outdegree(8), 0);
            assert_eq!(digraph.outdegree(9), 1);
            assert_eq!(digraph.outdegree(10), 0);
            assert_eq!(digraph.outdegree(11), 0);
            assert_eq!(digraph.outdegree(12), 1);
            assert_eq!(digraph.outdegree(13), 2);
            assert_eq!(digraph.outdegree(14), 0);
            assert_eq!(digraph.outdegree(15), 0);
        }

        #[test]
        fn outdegree_kattis_escapewallmaria_3() {
            let digraph = kattis_escapewallmaria_3();

            assert_eq!(digraph.outdegree(0), 0);
            assert_eq!(digraph.outdegree(1), 2);
            assert_eq!(digraph.outdegree(2), 2);
            assert_eq!(digraph.outdegree(3), 0);
            assert_eq!(digraph.outdegree(4), 0);
            assert_eq!(digraph.outdegree(5), 3);
            assert_eq!(digraph.outdegree(6), 2);
            assert_eq!(digraph.outdegree(7), 0);
            assert_eq!(digraph.outdegree(8), 0);
            assert_eq!(digraph.outdegree(9), 2);
            assert_eq!(digraph.outdegree(10), 0);
            assert_eq!(digraph.outdegree(11), 0);
            assert_eq!(digraph.outdegree(12), 1);
            assert_eq!(digraph.outdegree(13), 2);
            assert_eq!(digraph.outdegree(14), 0);
            assert_eq!(digraph.outdegree(15), 0);
        }

        #[test]
        #[should_panic(expected = "u = 1 isn't in the digraph")]
        fn outdegree_out_of_bounds() {
            let _ = <$type>::trivial().outdegree(1);
        }
    };
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::collections::BTreeSet,
    };

    #[test]
    fn is_sink() {
        struct AdjacencyList {
            arcs: Vec<BTreeSet<usize>>,
        }

        impl Outdegree for AdjacencyList {
            fn outdegree(&self, u: usize) -> usize {
                self.arcs.get(u).map_or(0, BTreeSet::len)
            }
        }

        let digraph = AdjacencyList {
            arcs: vec![
                BTreeSet::from([1, 2]),
                BTreeSet::new(),
                BTreeSet::new(),
                BTreeSet::from([0]),
            ],
        };

        assert!(!digraph.is_sink(0));
        assert!(digraph.is_sink(1));
        assert!(digraph.is_sink(2));
        assert!(!digraph.is_sink(3));
    }
}

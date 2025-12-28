//! Return a vertex's indegree.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     Indegree,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//!
//! assert_eq!(digraph.indegree(0), 0);
//! assert_eq!(digraph.indegree(1), 1);
//! assert_eq!(digraph.indegree(2), 2);
//! ```
#![doc(alias = "in_degree")]

use crate::Vertices;

/// Vertex indegree
#[doc(alias = "InDegree")]
pub trait Indegree {
    /// Return the vertex's indegree.
    ///
    /// # Arguments
    ///
    /// * `v`: The vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     Indegree,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 2);
    ///
    /// assert_eq!(digraph.indegree(0), 0);
    /// assert_eq!(digraph.indegree(1), 1);
    /// assert_eq!(digraph.indegree(2), 2);
    /// ```
    #[doc(alias = "in_degree")]
    #[must_use]
    fn indegree(&self, v: usize) -> usize;

    /// Check whether a vertex is a digraph's source.
    ///
    /// A source is a vertex with an indegree of 0.
    ///
    /// # Arguments
    ///
    /// * `v`: The vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     Indegree,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 2);
    ///
    /// assert!(digraph.is_source(0));
    /// assert!(!digraph.is_source(1));
    /// assert!(!digraph.is_source(2));
    /// ```
    #[must_use]
    fn is_source(&self, v: usize) -> bool {
        self.indegree(v) == 0
    }

    /// Return a digraph's maximum indegree.
    ///
    /// # Examples
    ///
    /// The maximum indegree of this digraph is `3`. The vertex with the
    /// maximum indegree is red.
    ///
    /// ![A digraph and its maximum indegree](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/max_indegree-0.88.5.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     Indegree,
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
    /// assert_eq!(digraph.max_indegree(), 3);
    /// ```
    #[must_use]
    fn max_indegree(&self) -> usize
    where
        Self: Vertices,
    {
        self.vertices().map(|u| self.indegree(u)).max().unwrap_or(0)
    }

    /// Return a digraph's minimum indegree.
    ///
    /// # Examples
    ///
    /// The minimum indegree of this digraph is `1`. The vertices with the
    /// minimum indegree are red.
    ///
    /// ![A digraph and its minimum indegree](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/min_indegree-0.88.5.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     Indegree,
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
    /// assert_eq!(digraph.min_indegree(), 1);
    /// ```
    #[must_use]
    fn min_indegree(&self) -> usize
    where
        Self: Vertices,
    {
        self.vertices().map(|u| self.indegree(u)).min().unwrap_or(0)
    }
}

/// `Indegree` tests
#[macro_export]
macro_rules! test_indegree {
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
        fn indegree_bang_jensen_196() {
            let digraph = bang_jensen_196();

            assert!(digraph.indegree(0) == 1);
            assert!(digraph.indegree(1) == 1);
            assert!(digraph.indegree(2) == 3);
            assert!(digraph.indegree(3) == 1);
            assert!(digraph.indegree(4) == 2);
            assert!(digraph.indegree(5) == 1);
            assert!(digraph.indegree(6) == 1);
            assert!(digraph.indegree(7) == 3);
        }

        #[test]
        fn indegree_bang_jensen_34() {
            let digraph = bang_jensen_34();

            assert!(digraph.indegree(0) == 1);
            assert!(digraph.indegree(1) == 1);
            assert!(digraph.indegree(2) == 0);
            assert!(digraph.indegree(3) == 1);
            assert!(digraph.indegree(4) == 2);
            assert!(digraph.indegree(5) == 1);
        }

        #[test]
        fn indegree_bang_jensen_94() {
            let digraph = bang_jensen_94();

            assert!(digraph.indegree(0) == 0);
            assert!(digraph.indegree(1) == 2);
            assert!(digraph.indegree(2) == 1);
            assert!(digraph.indegree(3) == 2);
            assert!(digraph.indegree(4) == 1);
            assert!(digraph.indegree(5) == 2);
            assert!(digraph.indegree(6) == 1);
        }

        #[test]
        fn indegree_kattis_builddeps() {
            let digraph = kattis_builddeps();

            assert!(digraph.indegree(0) == 0);
            assert!(digraph.indegree(1) == 3);
            assert!(digraph.indegree(2) == 0);
            assert!(digraph.indegree(3) == 2);
            assert!(digraph.indegree(4) == 2);
            assert!(digraph.indegree(5) == 1);
        }

        #[test]
        fn indegree_kattis_cantinaofbabel_1() {
            let digraph = kattis_cantinaofbabel_1();

            assert!(digraph.indegree(0) == 1);
            assert!(digraph.indegree(1) == 2);
            assert!(digraph.indegree(2) == 2);
            assert!(digraph.indegree(3) == 2);
            assert!(digraph.indegree(4) == 2);
            assert!(digraph.indegree(5) == 2);
            assert!(digraph.indegree(6) == 2);
            assert!(digraph.indegree(7) == 3);
            assert!(digraph.indegree(8) == 0);
            assert!(digraph.indegree(9) == 1);
            assert!(digraph.indegree(10) == 3);
            assert!(digraph.indegree(11) == 2);
        }

        #[test]
        fn indegree_kattis_cantinaofbabel_2() {
            let digraph = kattis_cantinaofbabel_2();

            assert!(digraph.indegree(0) == 2);
            assert!(digraph.indegree(1) == 1);
            assert!(digraph.indegree(2) == 1);
            assert!(digraph.indegree(3) == 2);
            assert!(digraph.indegree(4) == 1);
            assert!(digraph.indegree(5) == 2);
            assert!(digraph.indegree(6) == 1);
            assert!(digraph.indegree(7) == 3);
            assert!(digraph.indegree(8) == 1);
            assert!(digraph.indegree(9) == 2);
            assert!(digraph.indegree(10) == 1);
            assert!(digraph.indegree(11) == 2);
        }

        #[test]
        fn indegree_kattis_escapewallmaria_1() {
            let digraph = kattis_escapewallmaria_1();

            assert!(digraph.indegree(0) == 0);
            assert!(digraph.indegree(1) == 0);
            assert!(digraph.indegree(2) == 0);
            assert!(digraph.indegree(3) == 0);
            assert!(digraph.indegree(4) == 0);
            assert!(digraph.indegree(5) == 2);
            assert!(digraph.indegree(6) == 1);
            assert!(digraph.indegree(7) == 0);
            assert!(digraph.indegree(8) == 0);
            assert!(digraph.indegree(9) == 2);
            assert!(digraph.indegree(10) == 0);
            assert!(digraph.indegree(11) == 0);
            assert!(digraph.indegree(12) == 1);
            assert!(digraph.indegree(13) == 1);
            assert!(digraph.indegree(14) == 0);
            assert!(digraph.indegree(15) == 0);
        }

        #[test]
        fn indegree_kattis_escapewallmaria_2() {
            let digraph = kattis_escapewallmaria_2();

            assert!(digraph.indegree(0) == 0);
            assert!(digraph.indegree(1) == 0);
            assert!(digraph.indegree(2) == 0);
            assert!(digraph.indegree(3) == 0);
            assert!(digraph.indegree(4) == 0);
            assert!(digraph.indegree(5) == 2);
            assert!(digraph.indegree(6) == 1);
            assert!(digraph.indegree(7) == 0);
            assert!(digraph.indegree(8) == 0);
            assert!(digraph.indegree(9) == 2);
            assert!(digraph.indegree(10) == 0);
            assert!(digraph.indegree(11) == 0);
            assert!(digraph.indegree(12) == 1);
            assert!(digraph.indegree(13) == 1);
            assert!(digraph.indegree(14) == 0);
            assert!(digraph.indegree(15) == 0);
        }

        #[test]
        fn indegree_kattis_escapewallmaria_3() {
            let digraph = kattis_escapewallmaria_3();

            assert!(digraph.indegree(0) == 0);
            assert!(digraph.indegree(1) == 2);
            assert!(digraph.indegree(2) == 2);
            assert!(digraph.indegree(3) == 0);
            assert!(digraph.indegree(4) == 0);
            assert!(digraph.indegree(5) == 3);
            assert!(digraph.indegree(6) == 2);
            assert!(digraph.indegree(7) == 0);
            assert!(digraph.indegree(8) == 0);
            assert!(digraph.indegree(9) == 2);
            assert!(digraph.indegree(10) == 0);
            assert!(digraph.indegree(11) == 0);
            assert!(digraph.indegree(12) == 1);
            assert!(digraph.indegree(13) == 2);
            assert!(digraph.indegree(14) == 0);
            assert!(digraph.indegree(15) == 0);
        }

        #[test]
        #[should_panic(expected = "v = 1 isn't in the digraph")]
        fn indegree_out_of_bounds() {
            let _ = <$type>::trivial().indegree(1);
        }

        #[test]
        fn max_indegree_bang_jensen_196() {
            assert_eq!(bang_jensen_196().max_indegree(), 3);
        }

        #[test]
        fn max_indegree_bang_jensen_34() {
            assert_eq!(bang_jensen_34().max_indegree(), 2);
        }

        #[test]
        fn max_indegree_bang_jensen_94() {
            assert_eq!(bang_jensen_94().max_indegree(), 2);
        }

        #[test]
        fn max_indegree_kattis_builddeps() {
            assert_eq!(kattis_builddeps().max_indegree(), 3);
        }

        #[test]
        fn max_indegree_kattis_cantinaofbabel_1() {
            assert_eq!(kattis_cantinaofbabel_1().max_indegree(), 3);
        }

        #[test]
        fn max_indegree_kattis_cantinaofbabel_2() {
            assert_eq!(kattis_cantinaofbabel_2().max_indegree(), 3);
        }

        #[test]
        fn max_indegree_kattis_escapewallmaria_1() {
            assert_eq!(kattis_escapewallmaria_1().max_indegree(), 2);
        }

        #[test]
        fn max_indegree_kattis_escapewallmaria_2() {
            assert_eq!(kattis_escapewallmaria_2().max_indegree(), 2);
        }

        #[test]
        fn max_indegree_kattis_escapewallmaria_3() {
            assert_eq!(kattis_escapewallmaria_3().max_indegree(), 3);
        }

        #[test]
        fn min_indegree_bang_jensen_196() {
            assert_eq!(bang_jensen_196().min_indegree(), 1);
        }

        #[test]
        fn min_indegree_bang_jensen_34() {
            assert_eq!(bang_jensen_34().min_indegree(), 0);
        }

        #[test]
        fn min_indegree_bang_jensen_94() {
            assert_eq!(bang_jensen_94().min_indegree(), 0);
        }

        #[test]
        fn min_indegree_kattis_builddeps() {
            assert_eq!(kattis_builddeps().min_indegree(), 0);
        }

        #[test]
        fn min_indegree_kattis_cantinaofbabel_1() {
            assert_eq!(kattis_cantinaofbabel_1().min_indegree(), 0);
        }

        #[test]
        fn min_indegree_kattis_cantinaofbabel_2() {
            assert_eq!(kattis_cantinaofbabel_2().min_indegree(), 1);
        }

        #[test]
        fn min_indegree_kattis_escapewallmaria_1() {
            assert_eq!(kattis_escapewallmaria_1().min_indegree(), 0);
        }

        #[test]
        fn min_indegree_kattis_escapewallmaria_2() {
            assert_eq!(kattis_escapewallmaria_2().min_indegree(), 0);
        }

        #[test]
        fn min_indegree_kattis_escapewallmaria_3() {
            assert_eq!(kattis_escapewallmaria_3().min_indegree(), 0);
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
    fn is_source() {
        struct AdjacencyList {
            arcs: Vec<BTreeSet<usize>>,
        }

        impl Indegree for AdjacencyList {
            fn indegree(&self, v: usize) -> usize {
                self.arcs.iter().filter(|set| set.contains(&v)).count()
            }
        }

        let digraph = AdjacencyList {
            arcs: vec![
                BTreeSet::from([1, 2]),
                BTreeSet::new(),
                BTreeSet::new(),
                BTreeSet::from([1]),
            ],
        };

        assert!(digraph.is_source(0));
        assert!(!digraph.is_source(1));
        assert!(!digraph.is_source(2));
        assert!(digraph.is_source(3));
    }
}

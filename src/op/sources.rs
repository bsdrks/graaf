//! Iterate a digraph's sources.
//!
//! A source is a vertex without in-neighbors.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     Sources,
//! };
//!
//! let mut digraph = AdjacencyList::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//!
//! assert!(digraph.sources().eq([0, 3]));
//! ```


/// Digraph sources
pub trait Sources {
    /// Iterate the sources in the digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     Sources,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(4);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 2);
    ///
    /// assert!(digraph.sources().eq([0, 3]));
    /// ```
    #[must_use]
    fn sources(&self) -> impl Iterator<Item = usize>;
}

/// `Sources` tests
#[macro_export]
macro_rules! test_sources {
    ($fixture:path) => {
        use $fixture::{
            bang_jensen_34,
            bang_jensen_94,
            bang_jensen_196,
            kattis_builddeps,
            kattis_cantinaofbabel_1,
            kattis_cantinaofbabel_2,
            kattis_escapewallmaria_1,
            kattis_escapewallmaria_2,
            kattis_escapewallmaria_3,
        };

        #[test]
        fn sources_bang_jensen_196() {
            assert!(bang_jensen_196().sources().eq([]));
        }

        #[test]
        fn sources_bang_jensen_34() {
            assert!(bang_jensen_34().sources().eq([2]));
        }

        #[test]
        fn sources_bang_jensen_94() {
            assert!(bang_jensen_94().sources().eq([0]));
        }

        #[test]
        fn sources_kattis_builddeps() {
            assert!(kattis_builddeps().sources().eq([0, 2]));
        }

        #[test]
        fn sources_kattis_cantinaofbabel_1() {
            assert!(kattis_cantinaofbabel_1().sources().eq([8]));
        }

        #[test]
        fn sources_kattis_cantinaofbabel_2() {
            assert!(kattis_cantinaofbabel_2().sources().eq([]));
        }

        #[test]
        fn sources_kattis_escapewallmaria_1() {
            assert!(
                kattis_escapewallmaria_1()
                    .sources()
                    .eq([0, 1, 2, 3, 4, 7, 8, 10, 11, 14, 15])
            );
        }

        #[test]
        fn sources_kattis_escapewallmaria_2() {
            assert!(
                kattis_escapewallmaria_2()
                    .sources()
                    .eq([0, 1, 2, 3, 4, 7, 8, 10, 11, 14, 15])
            );
        }

        #[test]
        fn sources_kattis_escapewallmaria_3() {
            assert!(
                kattis_escapewallmaria_3()
                    .sources()
                    .eq([0, 3, 4, 7, 8, 10, 11, 14, 15])
            );
        }
    };
}

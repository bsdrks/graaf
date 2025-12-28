//! Iterate a digraph's sinks.
//!
//! A sink is a vertex without out-neighbors.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     Sinks,
//! };
//!
//! let mut digraph = AdjacencyList::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//!
//! assert!(digraph.sinks().eq([2, 3]));
//! ```

use crate::{
    Outdegree,
    Vertices,
};

/// Digraph sinks
pub trait Sinks {
    /// Iterate a digraph's sinks.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     Sinks,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(4);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 2);
    ///
    /// assert!(digraph.sinks().eq([2, 3]));
    /// ```
    #[must_use]
    fn sinks(&self) -> impl Iterator<Item = usize>;
}

impl<D> Sinks for D
where
    D: Outdegree + Vertices,
{
    fn sinks(&self) -> impl Iterator<Item = usize> {
        self.vertices().filter(move |&u| self.is_sink(u))
    }
}

/// `Sinks` tests
#[macro_export]
macro_rules! test_sinks {
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
        fn sinks_bang_jensen_196() {
            assert!(bang_jensen_196().sinks().eq([]));
        }

        #[test]
        fn sinks_bang_jensen_34() {
            assert!(bang_jensen_34().sinks().eq([3, 4]));
        }

        #[test]
        fn sinks_bang_jensen_94() {
            assert!(bang_jensen_94().sinks().eq([5, 6]));
        }

        #[test]
        fn sinks_kattis_builddeps() {
            assert!(kattis_builddeps().sinks().eq([1]));
        }

        #[test]
        fn sinks_kattis_cantinaofbabel_1() {
            assert!(kattis_cantinaofbabel_1().sinks().eq([]));
        }

        #[test]
        fn sinks_kattis_cantinaofbabel_2() {
            assert!(kattis_cantinaofbabel_2().sinks().eq([]));
        }

        #[test]
        fn sinks_kattis_escapewallmaria_1() {
            assert!(
                kattis_escapewallmaria_1()
                    .sinks()
                    .eq([0, 1, 2, 3, 4, 7, 8, 10, 11, 12, 14, 15])
            );
        }

        #[test]
        fn sinks_kattis_escapewallmaria_2() {
            assert!(
                kattis_escapewallmaria_2()
                    .sinks()
                    .eq([0, 1, 2, 3, 4, 7, 8, 10, 11, 14, 15])
            );
        }

        #[test]
        fn sinks_kattis_escapewallmaria_3() {
            assert!(
                kattis_escapewallmaria_3()
                    .sinks()
                    .eq([0, 3, 4, 7, 8, 10, 11, 14, 15])
            );
        }
    };
}

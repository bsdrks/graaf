//! Generate biclique digraphs.
//!
//! Bicliques are also known as complete bipartite digraphs.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Biclique,
//!     op::Arcs,
//! };
//!
//! // 0 -> {1}
//! // 1 -> {0}
//!
//! assert!(Digraph::biclique(1, 1).arcs().eq([(0, 1), (1, 0)]));
//!
//! // 0 -> {2, 3, 4}
//! // 1 -> {2, 3, 4}
//! // 2 -> {0, 1}
//! // 3 -> {0, 1}
//! // 4 -> {0, 1}
//!
//! assert!(Digraph::biclique(2, 3).arcs().eq([
//!     (0, 2),
//!     (0, 3),
//!     (0, 4),
//!     (1, 2),
//!     (1, 3),
//!     (1, 4),
//!     (2, 0),
//!     (2, 1),
//!     (3, 0),
//!     (3, 1),
//!     (4, 0),
//!     (4, 1),
//! ]));
//!
//! // 0 -> {4, 5}
//! // 1 -> {4, 5}
//! // 2 -> {4, 5}
//! // 3 -> {4, 5}
//! // 4 -> {0, 1, 2, 3}
//! // 5 -> {0, 1, 2, 3}
//!
//! assert!(Digraph::biclique(4, 2).arcs().eq([
//!     (0, 4),
//!     (0, 5),
//!     (1, 4),
//!     (1, 5),
//!     (2, 4),
//!     (2, 5),
//!     (3, 4),
//!     (3, 5),
//!     (4, 0),
//!     (4, 1),
//!     (4, 2),
//!     (4, 3),
//!     (5, 0),
//!     (5, 1),
//!     (5, 2),
//!     (5, 3),
//! ]));
//! ```
#![doc(alias = "complete_bipartite")]

use crate::{
    gen::Empty,
    op::AddArc,
};

/// Generate biclique digraphs.
///
/// # How can I implement `Biclique`?
///
/// Provide an implementation of `biclique` that generates a complete bipartite
/// digraph with two partitions of `m` and `n` vertices OR implement `AddArc`
/// and `Empty`.
///
/// ```
/// use {
///     graaf::{
///         gen::{
///             Biclique,
///             Empty,
///         },
///         op::AddArc,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl AddArc for Digraph {
///     fn add_arc(&mut self, u: usize, v: usize) {
///         self.arcs[u].insert(v);
///     }
/// }
///
/// impl Empty for Digraph {
///     fn empty(order: usize) -> Self {
///         Self {
///             arcs: vec![BTreeSet::new(); order],
///         }
///     }
/// }
///
/// // 0 -> {3, 4, 5}
/// // 1 -> {3, 4, 5}
/// // 2 -> {3, 4, 5}
/// // 3 -> {0, 1, 2}
/// // 4 -> {0, 1, 2}
/// // 5 -> {0, 1, 2}
///
/// let digraph = Digraph::biclique(3, 3);
///
/// assert_eq!(
///     digraph.arcs,
///     vec![
///         BTreeSet::from([3, 4, 5]),
///         BTreeSet::from([3, 4, 5]),
///         BTreeSet::from([3, 4, 5]),
///         BTreeSet::from([0, 1, 2]),
///         BTreeSet::from([0, 1, 2]),
///         BTreeSet::from([0, 1, 2]),
///     ]
/// );
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     gen::Biclique,
///     op::Arcs,
/// };
///
/// // 0 -> {1}
/// // 1 -> {0}
///
/// assert!(Digraph::biclique(1, 1).arcs().eq([(0, 1), (1, 0)]));
///
/// // 0 -> {2, 3, 4}
/// // 1 -> {2, 3, 4}
/// // 2 -> {0, 1}
/// // 3 -> {0, 1}
/// // 4 -> {0, 1}
///
/// assert!(Digraph::biclique(2, 3).arcs().eq([
///     (0, 2),
///     (0, 3),
///     (0, 4),
///     (1, 2),
///     (1, 3),
///     (1, 4),
///     (2, 0),
///     (2, 1),
///     (3, 0),
///     (3, 1),
///     (4, 0),
///     (4, 1)
/// ]));
///
/// // 0 -> {4, 5}
/// // 1 -> {4, 5}
/// // 2 -> {4, 5}
/// // 3 -> {4, 5}
/// // 4 -> {0, 1, 2, 3}
/// // 5 -> {0, 1, 2, 3}
///
/// assert!(Digraph::biclique(4, 2).arcs().eq([
///     (0, 4),
///     (0, 5),
///     (1, 4),
///     (1, 5),
///     (2, 4),
///     (2, 5),
///     (3, 4),
///     (3, 5),
///     (4, 0),
///     (4, 1),
///     (4, 2),
///     (4, 3),
///     (5, 0),
///     (5, 1),
///     (5, 2),
///     (5, 3)
/// ]));
/// ```
#[doc(alias = "CompleteBipartite")]
pub trait Biclique {
    /// Generates a biclique digraph.
    ///
    /// # Arguments
    ///
    /// * `m` - The number of vertices in the first partition.
    /// * `n` - The number of vertices in the second partition.
    #[doc(alias = "complete_bipartite")]
    #[must_use]
    fn biclique(m: usize, n: usize) -> Self;

    /// Generates a claw digraph.
    ///
    /// The claw digraph is also known as K{1, 3}.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Biclique,
    ///     op::Arcs,
    /// };
    ///
    /// // 0 -> {1, 2, 3}
    /// // 1 -> {0}
    /// // 2 -> {0}
    /// // 3 -> {0}
    ///
    /// assert!(Digraph::claw()
    ///     .arcs()
    ///     .eq([(0, 1), (0, 2), (0, 3), (1, 0), (2, 0), (3, 0)]));
    /// ```
    #[must_use]
    fn claw() -> Self
    where
        Self: Sized,
    {
        Self::biclique(1, 3)
    }

    /// Generates a utility digraph.
    ///
    /// The utility digraph is also known as K{3, 3}.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Biclique,
    ///     op::Arcs,
    /// };
    ///
    /// // 0 -> {3, 4, 5}
    /// // 1 -> {3, 4, 5}
    /// // 2 -> {3, 4, 5}
    /// // 3 -> {0, 1, 2}
    /// // 4 -> {0, 1, 2}
    /// // 5 -> {0, 1, 2}
    ///
    /// assert!(Digraph::utility().arcs().eq([
    ///     (0, 3),
    ///     (0, 4),
    ///     (0, 5),
    ///     (1, 3),
    ///     (1, 4),
    ///     (1, 5),
    ///     (2, 3),
    ///     (2, 4),
    ///     (2, 5),
    ///     (3, 0),
    ///     (3, 1),
    ///     (3, 2),
    ///     (4, 0),
    ///     (4, 1),
    ///     (4, 2),
    ///     (5, 0),
    ///     (5, 1),
    ///     (5, 2),
    /// ]));
    /// ```
    #[must_use]
    fn utility() -> Self
    where
        Self: Sized,
    {
        Self::biclique(3, 3)
    }
}

impl<D> Biclique for D
where
    D: AddArc + Empty,
{
    /// # Panics
    ///
    /// Panics if `m` or `n` is zero.
    fn biclique(m: usize, n: usize) -> Self {
        assert!(m > 0, "m must be greater than zero");
        assert!(n > 0, "n must be greater than zero");

        let order = m + n;
        let mut digraph = D::empty(order);

        for u in 0..m {
            for v in m..order {
                digraph.add_arc(u, v);
                digraph.add_arc(v, u);
            }
        }

        digraph
    }
}

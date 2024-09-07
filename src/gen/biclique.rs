//! Generate biclique digraphs.
//!
//! Bicliques are also known as complete bipartite digraphs.
//!
//! # Examples
//!
//! ## m = 2, n = 3
//!
//! Generate a biclique digraph with `m = 2` and `n = 3`.
//!
//! ![Biclique digraph with m = 2 and n = 3](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/biclique_2_3.svg?)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Biclique,
//!     op::Arcs,
//! };
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
//! ```
//!
//! ## m = 4, n = 2
//!
//! Generate a biclique digraph with `m = 4` and `n = 2`.
//!
//! ![Biclique digraph with m = 4 and n = 2](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/biclique_4_2.svg?)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Biclique,
//!     op::Arcs,
//! };
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

/// Generate biclique digraphs.
///
/// Bicliques are also known as complete bipartite digraphs.
///
/// # Implementing `Biclique`
///
/// Provide an implementation of `biclique` that generates a complete bipartite
/// digraph with two partitions of `m` and `n` vertices.
///
/// ```
/// use {
///     graaf::gen::Biclique,
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Biclique for Digraph {
///     fn biclique(m: usize, n: usize) -> Self {
///         let order = m + n;
///         let clique_1 = (0..m).collect::<BTreeSet<_>>();
///         let clique_2 = (m..order).collect::<BTreeSet<_>>();
///         let mut arcs = vec![BTreeSet::new(); order];
///
///         for u in 0..m {
///             arcs[u].clone_from(&clique_2);
///         }
///
///         for v in m..order {
///             arcs[v].clone_from(&clique_1);
///         }
///
///         Self { arcs }
///     }
/// }
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
#[doc(alias = "CompleteBipartite")]
pub trait Biclique {
    /// Generates a biclique digraph.
    ///
    /// # Arguments
    ///
    /// * `m` - The number of vertices in the first partition.
    /// * `n` - The number of vertices in the second partition.
    ///
    /// # Examples
    ///
    /// ## m = 2, n = 3
    ///
    /// Generate a biclique digraph with `m = 2` and `n = 3`.
    ///
    /// ![Biclique digraph with m = 2 and n = 3](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/biclique_2_3.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Biclique,
    ///     op::Arcs,
    /// };
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
    ///     (4, 1),
    /// ]));
    /// ```
    ///
    /// ## m = 4, n = 2
    ///
    /// Generate a biclique digraph with `m = 4` and `n = 2`.
    ///
    /// ![Biclique digraph with m = 4 and n = 2](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/biclique_4_2.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Biclique,
    ///     op::Arcs,
    /// };
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
    ///     (5, 3),
    /// ]));
    /// ```
    #[doc(alias = "complete_bipartite")]
    #[must_use]
    fn biclique(m: usize, n: usize) -> Self;

    /// Generates a claw digraph.
    ///
    /// The claw digraph is also known as K{1, 3}.
    ///
    /// ![Claw digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/biclique_claw.svg)
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
    /// assert!(Digraph::claw().arcs().eq([
    ///     (0, 1),
    ///     (0, 2),
    ///     (0, 3),
    ///     (1, 0),
    ///     (2, 0),
    ///     (3, 0)
    /// ]));
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
    /// ![Utility digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/biclique_utility.svg)
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

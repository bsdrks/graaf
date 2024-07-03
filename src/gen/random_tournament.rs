//! Generate random tournaments.
//!
//! A tournament is a digraph in which an arc connects every unordered pair of
//! distinct vertices.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::RandomTournament,
//!     op::{
//!         Degree,
//!         Indegree,
//!         Order,
//!         Outdegree,
//!         Size,
//!         Vertices,
//!     },
//! };
//!
//! let tournament = Digraph::random_tournament(4);
//!
//! assert_eq!(tournament.size(), 6);
//! assert_eq!(tournament.order(), 4);
//!
//! for s in tournament.vertices() {
//!     assert_eq!(tournament.degree(s), 3);
//!     assert!((0..3).contains(&tournament.outdegree(s)));
//!     assert!((0..3).contains(&tournament.indegree(s)));
//! }
//! ```

use {
    super::prng::Xoshiro256StarStar,
    crate::{
        gen::Empty,
        op::AddArc,
    },
};

/// Generate random tournaments.
///
/// # How can I implement `RandomTournament`?
///
/// Provide an implementation of `random_tournament` that generates a random
/// tournament with `v` vertices OR implement `AddArc` and `Empty`.
///
/// ```
/// use {
///     graaf::{
///         gen::{
///             prng::Xoshiro256StarStar,
///             Empty,
///             RandomTournament,
///         },
///         op::{
///             AddArc,
///             Degree,
///             Indegree,
///             Order,
///             Outdegree,
///             Size,
///             Vertices,
///         },
///     },
///     std::collections::BTreeSet,
/// };
///
/// pub struct Tournament {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl RandomTournament for Tournament {
///     fn random_tournament(v: usize) -> Self {
///         let mut rng = Xoshiro256StarStar::new(v as u64);
///
///         let mut tournament = Self {
///             arcs: vec![BTreeSet::new(); v],
///         };
///
///         for s in 0..v {
///             for t in (s + 1)..v {
///                 if rng.next_bool() {
///                     tournament.arcs[s].insert(t);
///                 } else {
///                     tournament.arcs[t].insert(s);
///                 }
///             }
///         }
///
///         tournament
///     }
/// }
///
/// let tournament = Tournament::random_tournament(4);
///
/// assert_eq!(tournament.arcs.len(), 4);
///
/// assert_eq!(
///     tournament.arcs.iter().map(|set| set.len()).sum::<usize>(),
///     6
/// );
///
/// for s in 0..tournament.arcs.len() {
///     assert!((0..3).contains(&tournament.arcs[s].len()));
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     gen::RandomTournament,
///     op::{
///         Degree,
///         Indegree,
///         Order,
///         Outdegree,
///         Size,
///         Vertices,
///     },
/// };
///
/// let tournament = Digraph::random_tournament(4);
///
/// assert_eq!(tournament.size(), 6);
/// assert_eq!(tournament.order(), 4);
///
/// for s in tournament.vertices() {
///     assert_eq!(tournament.degree(s), 3);
///     assert!((0..3).contains(&tournament.outdegree(s)));
///     assert!((0..3).contains(&tournament.indegree(s)));
/// }
/// ```
pub trait RandomTournament {
    /// Generates a random tournament.
    #[must_use]
    fn random_tournament(v: usize) -> Self;
}

impl<D> RandomTournament for D
where
    D: AddArc + Empty,
{
    fn random_tournament(v: usize) -> Self {
        let mut digraph = Self::empty(v);
        let mut rng = Xoshiro256StarStar::new(v as u64);

        for s in 0..v {
            for t in (s + 1)..v {
                if rng.next_bool() {
                    digraph.add_arc(s, t);
                } else {
                    digraph.add_arc(t, s);
                }
            }
        }

        digraph
    }
}

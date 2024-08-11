//! Digraph generators.
//!
//! Generate parameterized and random digraphs.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Circuit,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::circuit(4)
//!     .arcs()
//!     .eq([(0, 1), (1, 2), (2, 3), (3, 0)]));
//! ```
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Complete,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::complete(4).arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (0, 3),
//!     (1, 0),
//!     (1, 2),
//!     (1, 3),
//!     (2, 0),
//!     (2, 1),
//!     (2, 3),
//!     (3, 0),
//!     (3, 1),
//!     (3, 2),
//! ]));
//! ```
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         Arcs,
//!         Order,
//!     },
//! };
//!
//! // 0 -> {}
//! // 1 -> {}
//! // 2 -> {}
//! // 3 -> {}
//!
//! let digraph = Digraph::empty(4);
//!
//! assert!(digraph.arcs().eq([]));
//! assert_eq!(digraph.order(), 4);
//! ```
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
//! let tournament = Digraph::random_tournament(4, 0);
//!
//! assert_eq!(tournament.size(), 6);
//! assert_eq!(tournament.order(), 4);
//!
//! for s in tournament.vertices() {
//!     assert_eq!(tournament.degree(s), 3);
//!     assert!((0..=3).contains(&tournament.outdegree(s)));
//!     assert!((0..=3).contains(&tournament.indegree(s)));
//! }
//! ```

pub mod biclique;
pub mod circuit;
pub mod complete;
pub mod cycle;
pub mod empty;
pub mod erdos_renyi;
pub mod path;
pub mod prng;
pub mod random_tournament;
pub mod star;

pub use {
    biclique::Biclique,
    circuit::Circuit,
    complete::Complete,
    cycle::Cycle,
    empty::Empty,
    erdos_renyi::ErdosRenyi,
    path::Path,
    random_tournament::RandomTournament,
    star::Star,
};

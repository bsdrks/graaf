//! Generate parameterized and random digraphs.
//!
//! # Examples
//!
//! ## Biclique digraph
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
//! ## Circuit digraph
//!
//! Generate a circuit digraph of order `4`.
//!
//! ![Circuit digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/circuit_4.svg?)
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
//! ## Complete digraph
//!
//! Generate a complete digraph of order `4`.
//!
//! ![Complete digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/complete_4.svg?)
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
//! ## Cycle digraph
//!
//! Generate a cycle digraph of order `4`.
//!
//! ![Cycle digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/cycle_4.svg?)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Cycle,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::cycle(4).arcs().eq([
//!     (0, 1),
//!     (0, 3),
//!     (1, 0),
//!     (1, 2),
//!     (2, 1),
//!     (2, 3),
//!     (3, 0),
//!     (3, 2),
//! ]));
//! ```
//!
//! ## Empty digraph
//!
//! Generate an empty digraph of order `4`.
//!
//! ![Empty digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/empty_4.svg?)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::empty(4).arcs().eq([]));
//! ```
//!
//! ## Erdős-Rényi digraph
//!
//! Generate a random Erdős-Rényi digraph of order `4` and probability 0.5.
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::ErdosRenyi,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::erdos_renyi(4, 0.5, 0).arcs().count() <= 12);
//! ```
//!
//! ## Path digraph
//!
//! Generate a path digraph of order `4`.
//!
//! ![Path digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_4.svg?)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Path,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::path(4).arcs().eq([(0, 1), (1, 2), (2, 3),]));
//! ```
//!
//! ## Random tournament digraph
//!
//! Generate a random tournament digraph of order `4`.
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::RandomTournament,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::random_tournament(4, 0).arcs().count() <= 12);
//! ```
//!
//! ## Star digraph
//!
//! Generate a star digraph of order `4`.
//!
//! ![Star digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_4.svg?)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Star,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::star(4).arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (0, 3),
//!     (1, 0),
//!     (2, 0),
//!     (3, 0),
//! ]));
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

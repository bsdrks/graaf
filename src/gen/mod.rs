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
//!     AdjacencyList,
//!     Arcs,
//!     Biclique,
//! };
//!
//! assert!(AdjacencyList::biclique(2, 3).arcs().eq([
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
//!     AdjacencyList,
//!     Arcs,
//!     Circuit,
//! };
//!
//! assert!(AdjacencyList::circuit(4).arcs().eq([
//!     (0, 1),
//!     (1, 2),
//!     (2, 3),
//!     (3, 0)
//! ]));
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
//!     AdjacencyList,
//!     Arcs,
//!     Complete,
//! };
//!
//! assert!(AdjacencyList::complete(4).arcs().eq([
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
//!     AdjacencyList,
//!     Arcs,
//!     Cycle,
//! };
//!
//! assert!(AdjacencyList::cycle(4).arcs().eq([
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
//!     AdjacencyList,
//!     Arcs,
//!     Empty,
//! };
//!
//! assert!(AdjacencyList::empty(4).arcs().eq([]));
//! ```
//!
//! ## Erdős-Rényi digraph
//!
//! Generate a random Erdős-Rényi digraph of order `4` and probability 0.5.
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     ErdosRenyi,
//! };
//!
//! assert!(AdjacencyList::erdos_renyi(4, 0.5, 0).arcs().count() <= 12);
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
//!     AdjacencyList,
//!     Arcs,
//!     Path,
//! };
//!
//! assert!(AdjacencyList::path(4).arcs().eq([(0, 1), (1, 2), (2, 3),]));
//! ```
//!
//! ## Random tournament digraph
//!
//! Generate a random tournament digraph of order `4`.
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     RandomTournament,
//! };
//!
//! assert!(AdjacencyList::random_tournament(4, 0).arcs().count() <= 12);
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
//!     AdjacencyList,
//!     Arcs,
//!     Star,
//! };
//!
//! assert!(AdjacencyList::star(4).arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (0, 3),
//!     (1, 0),
//!     (2, 0),
//!     (3, 0),
//! ]));
//! ```
//!
//! ## Wheel digraph
//!
//! Generate a wheel digraph of order `4`.
//!
//! ![Wheel digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/wheel_4-0.87.5.svg?)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Wheel,
//! };
//!
//! assert!(AdjacencyList::wheel(4).arcs().eq([
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
pub mod wheel;

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
    wheel::Wheel,
};

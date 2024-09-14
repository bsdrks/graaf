//! Generate parameterized and random digraphs.
//!
//! # Examples
//!
//! ## Biclique digraph
//!
//! Generate a [`biclique`](Biclique] digraph with `m = 2` and `n = 3`.
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
//! Generate a [`circuit`](Circuit) digraph of order `4`.
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
//! Generate a [`complete`](Complete) digraph of order `4`.
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
//! Generate a [`cycle`](Cycle) digraph of order `4`.
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
//! Generate an [`empty`](Empty) digraph of order `4`.
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
//! Generate a random [`Erdős-Rényi`](ErdosRenyi) digraph of order `6` with a
//! probability of `0.5`.
//!
//! ![Random Erdős-Rényi digraph of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/erdos_renyi_1-0.89.2.svg?)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     ErdosRenyi,
//!     Order,
//!     Size,
//! };
//!
//! let digraph = AdjacencyList::erdos_renyi(6, 0.5, 0);
//!
//! assert!(digraph.arcs().eq([
//!     (0, 3),
//!     (0, 4),
//!     (1, 2),
//!     (2, 0),
//!     (2, 1),
//!     (2, 3),
//!     (2, 5),
//!     (3, 0),
//!     (3, 2),
//!     (3, 5),
//!     (4, 0),
//!     (4, 1),
//!     (4, 2),
//!     (5, 0)
//! ]));
//! ```
//!
//! ## Path digraph
//!
//! Generate a [`path`](Path) digraph of order `4`.
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
//! assert!(AdjacencyList::path(4).arcs().eq([(0, 1), (1, 2), (2, 3)]));
//! ```
//!
//! ## Random tournament digraph
//!
//! Generate a [`random tournament`](RandomTournament) digraph of order `6`.
//!
//! ![Random tournament of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/random_tournament_1-0.89.2.svg?)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     RandomTournament,
//! };
//!
//! let tournament = AdjacencyList::random_tournament(6, 0);
//!
//! assert!(tournament.arcs().eq([
//!     (0, 5),
//!     (1, 0),
//!     (1, 4),
//!     (1, 5),
//!     (2, 0),
//!     (2, 1),
//!     (2, 3),
//!     (2, 5),
//!     (3, 0),
//!     (3, 1),
//!     (3, 5),
//!     (4, 0),
//!     (4, 2),
//!     (4, 3),
//!     (5, 4)
//! ]));
//! ```
//!
//! ## Star digraph
//!
//! Generate a [`star`](Star) digraph of order `4`.
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
//! Generate a [`wheel`](Wheel) digraph of order `4`.
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
// pub mod growing_network;
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
    // growing_network::GrowingNetwork,
    path::Path,
    random_tournament::RandomTournament,
    star::Star,
    wheel::Wheel,
};

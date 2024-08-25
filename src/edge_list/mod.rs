//! An edge list representation of an unweighted digraph
//!
//! # Example
//!
//! ## Valid digraph
//!
//! A valid digraph of order 5 and size 8.
//!
//! ![digraph of order 5 and size 8](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/edge_list_1.svg?)
//!
//! ```
//! use graaf::{
//!     edge_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         Arcs,
//!     },
//! };
//!
//! let mut digraph = Digraph::empty(5);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//! digraph.add_arc(1, 3);
//! digraph.add_arc(1, 4);
//! digraph.add_arc(3, 0);
//! digraph.add_arc(3, 2);
//! digraph.add_arc(4, 1);
//!
//! assert!(digraph.arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (1, 0),
//!     (1, 3),
//!     (1, 4),
//!     (3, 0),
//!     (3, 2),
//!     (4, 1)
//! ]));
//! ```
//!
//! ## Self-loop
//!
//! A self-loop is not allowed. The following pseudograph can not be
//! represented. The self-loop is marked in red.
//!
//! ![self-loop](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/edge_list_self_loop.svg?)
//!
//! Adding a self-loop will panic:
//!
//! ```should_panic
//! use graaf::{
//!     edge_list::Digraph,
//!     gen::Empty,
//!     op::AddArc,
//! };
//!
//! let mut digraph = Digraph::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(3, 2);
//!
//! // This will panic.
//! digraph.add_arc(2, 2);
//! ```
//!
//! ## Parallel arcs
//!
//! Parallel arcs are not allowed. The following multigraph can not be
//! represented. The parallel arc is marked in red:
//!
//! ![parallel arcs](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/edge_list_parallel_arcs.svg?)
//!
//! Adding a parallel arc does not change the digraph:
//!
//! ```
//! use graaf::{
//!     edge_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         Arcs,
//!     },
//! };
//!
//! let mut digraph = Digraph::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(3, 2);
//!
//! assert!(digraph.arcs().eq([(0, 1), (1, 2), (3, 2),]));
//!
//! // This does not change the digraph.
//! digraph.add_arc(0, 1);
//!
//! assert!(digraph.arcs().eq([(0, 1), (1, 2), (3, 2),]));
//! ```

pub mod digraph;
pub mod fixture;

pub use digraph::Digraph;

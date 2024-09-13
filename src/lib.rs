//! # Graaf
//!
//! Rust-powered directed graphs.
//!
//! # Table of Contents
//!
//! - [Representations](#representations)
//! - [Generators](#generators)
//! - [Operations](#operations)
//! - [Algorithms](#algorithms)
//!    - [Bellman-Ford-Moore](#bellman-ford-moore)
//!    - [Breadth-First Search](#breadth-first-search)
//!    - [Depth-First Search](#depth-first-search)
//!    - [Dijkstra](#dijkstra)
//!    - [Distance Matrix](#distance-matrix)
//!    - [Floyd-Warshall](#floyd-warshall)
//!    - [Johnson's Circuit-Finding
//!      Algorithm](#johnsons-circuit-finding-algorithm)
//!    - [Predecessor Tree](#predecessor-tree)
//!    - [Tarjan](#tarjan)
//!
//! # Representations
//!
//! ## Arc-Weighted Sparse Digraphs
//!
//! - [`AdjacencyListWeighted`] represents digraphs as a vector of maps.
//!
//! ## Unweighted Dense Digraphs
//!
//! - [`AdjacencyMatrix`] represents digraphs as a matrix using a bit vector.
//!
//! ## Unweighted Sparse Digraphs
//!
//! - [`AdjacencyList`] represents digraphs as a vector of sets.
//! - [`AdjacencyMap`] represents digraphs as a map of sets.
//! - [`EdgeList`] represents digraphs as a vector of tuples.
//!
//! # Generators
//!
//! - [`Biclique`] generates a complete bipartite digraph.
//! - [`Circuit`] generates a circuit digraph.
//! - [`Complete`] generates a complete digraph.
//! - [`Cycle`] generates a bidirectional circuit.
//! - [`Empty`] generates a digraph with no arcs.
//! - [`ErdosRenyi`] generates a random digraph.
//! - [`RandomTournament`] generates a random tournament.
//! - [`Star`] generates a star digraph.
//! - [`Path`] generates a path digraph.
//! - [`Wheel`] generates a wheel digraph.
//!
//! # Operations
//!
//! - [`AddArcWeighted`] adds an arc to an arc-weighted digraph.
//! - [`AddArc`] adds an arc to an unweighted digraph.
//! - [`ArcWeight`] returns an arc's weight.
//! - [`ArcsWeighted`] iterates a digraph's weighted arcs.
//! - [`Arcs`] iterates a digraph's arcs.
//! - [`Complement`] returns a digraph's complement.
//! - [`Converse`] returns a digraph's converse.
//! - [`DegreeSequence`] iterates a digraph's degrees.
//! - [`Degree`] returns a vertex's degree.
//! - [`FilterVertices`] filters a digraph's vertices.
//! - [`HasArc`] checks whether a digraph contains an arc.
//! - [`HasEdge`] checks whether a digraph contains an edge.
//! - [`HasWalk`] checks whether a digraph contains a walk.
//! - [`InNeighbors`] iterates a vertex's in-neighbors.
//! - [`IndegreeSequence`] iterates a digraph's indegrees.
//! - [`Indegree`] a vertex's indegree.
//! - [`IsBalanced`] checks whether a digraph is balanced.
//! - [`IsComplete`] checks whether a digraph is complete.
//! - [`IsIsolated`] checks whether a vertex is isolated.
//! - [`IsOriented`] checks whether a digraph is oriented.
//! - [`IsPendant`] checks whether a vertex is a pendant.
//! - [`IsRegular`] checks whether a digraph is regular.
//! - [`IsSemicomplete`] checks whether a digraph is semicomplete.
//! - [`IsSimple`] checks whether a digraph is simple.
//! - [`IsSpanningSubdigraph`] checks whether a digraph spans a superdigraph.
//! - [`IsSubdigraph`] checks whether a digraph is a subdigraph.
//! - [`IsSuperdigraph`] checks whether a digraph is a superdigraph.
//! - [`IsSymmetric`] checks whether a digraph is symmetric.
//! - [`IsTournament`] checks whether a digraph is a tournament.
//! - [`Order`] returns the number of vertices in a digraph.
//! - [`OutNeighborsWeighted`] iterates a vertex's weighted out-neighbors.
//! - [`OutNeighbors`] iterates a vertex's out-neighbors.
//! - [`OutdegreeSequence`] iterates a digraph's outdegrees.
//! - [`Outdegree`] returns a vertex's outdegree.
//! - [`RemoveArc`] removes an arc from a digraph.
//! - [`SemidegreeSequence`] iterates a digraph's semidegrees.
//! - [`Sinks`] iterates a digraph's sinks.
//! - [`Size`] returns the number of arcs in a digraph.
//! - [`Sources`] iterates a digraph's sources.
//! - [`Union`] returns the union of two digraphs.
//! - [`Vertices`] iterates a digraph's vertices.
//!
//! # Algorithms
//!
//! ## Bellman-Ford-Moore
//!
//! Find the shortest distances from a source vertex to all other vertices in
//! an arc-weighted digraph with negative weights.

//! - [`bellman_ford_moore::single_source_distances`] finds the shortest
//!   distances.
//!
//! ## Breadth-First Search
//!
//! A breadth-first search explores the vertices of an unweighted digraph in
//! order of their distance from a source.
//!
//! - [`Bfs`](bfs::Bfs) iterates the vertices.
//! - [`BfsDist`](bfs_dist::BfsDist) iterates the vertices and their distance
//!   from the source.
//! - [`BfsPred`](bfs_pred::BfsPred) iterates the vertices and their
//!   predecessors.
//! - [`BfsDist::distances`](bfs_dist::BfsDist::distances) finds the shortest
//!   distances.
//! - [`BfsPred::cycles`](bfs_pred::BfsPred::predecessors) returns the cycles
//!   along the shortest path.
//! - [`BfsPred::predecessors`](bfs_pred::BfsPred::predecessors) finds the
//!   predecessors.
//! - [`BfsPred::shortest_path`](bfs_pred::BfsPred::shortest_path) finds the
//!   shortest path.
//!
//! ## Depth-First Search
//!
//! A depth-first search explores the vertices of an unweighted digraph in
//! order of their distance from a source.
//!
//! - [`Dfs`](dfs::Dfs) iterates the vertices.
//! - [`DfsDist`](dfs_dist::DfsDist) iterates the vertices and their distance
//!   from the source.
//! - [`DfsPred`](dfs_pred::DfsPred) iterates the vertices and their
//!   predecessors.
//! - [`DfsPred::predecessors`](dfs_pred::DfsPred::predecessors) finds the
//!   predecessors.
//!
//! ## Dijkstra
//!
//! Dijkstra's algorithm finds the shortest paths from one or more source
//! vertices in an arc-weighted digraph.
//!
//! - [`Dijkstra`](dijkstra::Dijkstra) iterates the vertices.
//! - [`DijkstraDist`](dijkstra_dist::DijkstraDist) iterates the vertices and
//!   their distance from the source.
//! - [`DijkstraPred`](dijkstra_pred::DijkstraPred) iterates the vertices and
//!   their predecessors.
//! - [`DijkstraDist::distances`](dijkstra_dist::DijkstraDist::distances) finds
//!   the shortest distances.
//! - [`DijkstraPred::predecessors`](dijkstra_pred::DijkstraPred::predecessors)
//!   finds the predecessors.
//! - [`DijkstraPred::shortest_path`](dijkstra_pred::DijkstraPred::shortest_path) finds the shortest path.
//!
//! ## Distance Matrix
//!
//! A [`DistanceMatrix`] contains the shortest distances between all pairs of
//! vertices in a digraph.
//!
//! - [`DistanceMatrix::center`](DistanceMatrix::center) finds the center of
//!   the digraph.
//! - [`DistanceMatrix::diameter`](DistanceMatrix::diameter) finds the diameter
//!   of the digraph.
//! - [`DistanceMatrix::eccentricities`](DistanceMatrix::eccentricities)
//!   returns the eccentricities of the vertices.
//! - [`DistanceMatrix::is_connected`](DistanceMatrix::is_connected) checks if
//!   the digraph is connected.
//! - [`DistanceMatrix::periphery`](DistanceMatrix::periphery) finds the
//!   periphery of the digraph.
//!
//! ## Floyd-Warshall
//!
//! The Floyd-Warshall algorithm finds the distance between each pair
//! of vertices in an arc-weighted digraph.
//!
//! - [`floyd_warshall::distances`] finds the shortest distances.
//!
//! ## Johnson's Circuit-Finding Algorithm
//!
//! Johnson's circuit-finding algorithm finds all circuits in a digraph.
//!
//! - [`Johnson75::find_circuits`](johnson_75::Johnson75::find_circuits) finds
//!   all circuits.
//!
//! ## Predecessor Tree
//!
//! A [`PredecessorTree`] is the result of a search and contains the
//! predecessors of the vertices.
//!
//! - [`PredecessorTree::search`] finds a vertex by value.
//! - [`PredecessorTree::search_by`] finds a vertex by predicate.
//!
//! ## Tarjan
//!
//! Tarjan's algorithm finds strongly connected components in a digraph.
//!
//! - [`tarjan::strongly_connected_components`] finds strongly connected
//!   components.

// Clippy lint groups
#![deny(clippy::all, clippy::cargo, clippy::pedantic, clippy::nursery)]
// Clippy restriction lints
#![deny(
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::impl_trait_in_params,
    clippy::missing_assert_message,
    clippy::multiple_inherent_impl,
    clippy::panic_in_result_fn,
    clippy::redundant_type_annotations,
    clippy::renamed_function_params,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::self_named_module_files,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::unwrap_in_result
)]
// Rustc lint groups
#![deny(rust_2018_idioms)]
// Rustc lints
#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    variant_size_differences
)]
// Rustdoc lints
#![deny(rustdoc::all)]
// Overwrites
#![allow(clippy::large_stack_frames)]

pub mod algo;
pub mod gen;
pub mod op;
#[cfg(test)]
pub mod proptest_strategy;
pub mod repr;

pub use repr::{
    AdjacencyList,
    AdjacencyListWeighted,
    AdjacencyMap,
    AdjacencyMatrix,
    EdgeList,
};

pub use op::{
    AddArc,
    AddArcWeighted,
    ArcWeight,
    Arcs,
    ArcsWeighted,
    Complement,
    Converse,
    Degree,
    DegreeSequence,
    FilterVertices,
    HasArc,
    HasEdge,
    HasWalk,
    InNeighbors,
    Indegree,
    IndegreeSequence,
    IsBalanced,
    IsComplete,
    IsIsolated,
    IsOriented,
    IsPendant,
    IsRegular,
    IsSemicomplete,
    IsSimple,
    IsSpanningSubdigraph,
    IsSubdigraph,
    IsSuperdigraph,
    IsSymmetric,
    IsTournament,
    Order,
    OutNeighbors,
    OutNeighborsWeighted,
    Outdegree,
    OutdegreeSequence,
    RemoveArc,
    SemidegreeSequence,
    Sinks,
    Size,
    Sources,
    Union,
    Vertices,
};

pub use gen::{
    Biclique,
    Circuit,
    Complete,
    Cycle,
    Empty,
    ErdosRenyi,
    Path,
    RandomTournament,
    Star,
    Wheel,
};

pub use algo::{
    bellman_ford_moore,
    bfs,
    bfs_dist,
    bfs_pred,
    dfs,
    dfs_dist,
    dfs_pred,
    dijkstra,
    dijkstra_dist,
    dijkstra_pred,
    distance_matrix,
    floyd_warshall,
    johnson_75,
    predecessor_tree,
    tarjan,
};

pub use {
    distance_matrix::DistanceMatrix,
    predecessor_tree::PredecessorTree,
};

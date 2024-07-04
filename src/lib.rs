//! # Graaf
//!
//! Work with directed graphs in Rust.
//!
//! # Table of Contents
//!
//! - [Digraph Types](#digraph-types)
//! - [Creating Digraphs](#creating-digraphs)
//! - [Operations](#operations)
//!    - [Basic operations](#basic-operations)
//!    - [Extended operations](#extended-operations)
//! - [Algorithms](#algorithms)
//!    - [Bellman-Ford-Moore](#bellman-ford-moore)
//!    - [Breadth-first search (BFS)](#breadth-first-search-bfs)
//!    - [Depth-first search (DFS)](#depth-first-search-dfs)
//!    - [Dijkstra's algorithm](#dijkstras-algorithm)
//!    - [Floyd-Warshall algorithm](#floyd-warshall)
//!    - [Breath-first tree](#breadth-first-tree)
//!    - [Distance matrix](#distance-matrix)
//!
//! # Digraph Types
//!
//! Graaf provides three representations of directed graphs.
//!
//! - The [Adjacency List] type represents unweighted sparse digraphs.
//! - The [Adjacency Matrix] type represents unweighted dense digraphs.
//! - The [Weighted Adjacency List] type represents weighted sparse digraphs.
//!
//! These types eagerly implement [digraph operations](#operations) and
//! [digraph algorithms](#algorithms).
//!
//! # Creating Digraphs
//!
//! Graaf provides four digraph generators.
//!
//! - The [`Complete`](gen::Complete) trait generates a digraph in which an arc
//!   connects every ordered pair of distinct vertices.
//! - The [`Cycle`](gen::Cycle) trait generates a digraph with a cycle of a
//!   given length.
//! - The [`Empty`](gen::Empty) trait generates a digraph with no arcs.
//! - The [`RandomTournament`](gen::RandomTournament) trait generates a random
//!   digraph in which an arc connects every unordered pair of distinct
//!   vertices.
//!
//! # Operations
//!
//! The [`op`] module provides digraph operation traits. The [digraph
//! types](#digraph-types) implement these traits. One can implement these
//! traits for custom digraph types. Operations form the foundation for
//! [algorithms](#algorithms).
//!
//! ## Basic operations
//!
//! [Individual digraph types](#digraph-types) implement the basic operations.
//!
//! - The [`AddArcWeighted`](op::AddArcWeighted) adds an arc to a weighted
//!   digraph.
//! - The [`AddArc`](op::AddArc) adds an arc to an unweighted digraph.
//! - The [`ArcWeight`](op::ArcWeight) returns the weight of an arc.
//! - The [`ArcsWeighted`](op::ArcsWeighted) returns the arcs and their weights
//!   in a digraph.
//! - The [`Arcs`](op::Arcs) returns the arcs in a digraph.
//! - The [`Converse`](op::Converse) returns the converse of a digraph.
//! - The [`HasArc`](op::HasArc) trait checks if an arc exists in a digraph.
//! - The [`Indegree`](op::Indegree) returns the indegree of a vertex.
//! - The [`IsSimple`](op::IsSimple) trait checks if a digraph contains no loops
//!   or parallel arcs.
//! - The [`Order`](op::Order) returns the number of vertices.
//! - The [`OutNeighborsWeighted`](op::OutNeighborsWeighted) returns the
//!   weighted out-neighbors of a vertex.
//! - The [`OutNeighbors`](op::OutNeighbors) returns the out-neighbors of a
//!   vertex.
//! - The [`Outdegree`](op::Outdegree) returns the outdegree of a vertex.
//! - The [`RemoveArc`](op::RemoveArc) removes an arc from a digraph.
//! - The [`Size`](op::Size) returns the number of arcs in a digraph.
//! - The [`Vertices`](op::Vertices) returns the vertices in a digraph.
//!
//! ## Extended operations
//!
//! The extended traits derive their implementation from the basic
//! operations.
//!
//! - The [`Degree`](op::Degree) returns the degree of a vertex.
//! - The [`DegreeSequence`](op::DegreeSequence) returns the degree sequence of
//!   a digraph.
//! - The [`HasEdge`](op::HasEdge) trait checks if an edge exists in a digraph.
//! - The [`InNeighbors`](op::InNeighbors) gets the in-neighbors of a vertex.
//! - The [`IsBalanced`](op::IsBalanced) trait checks if a digraph is balanced.
//! - The [`IsComplete`](op::IsComplete) trait checks if a digraph is complete.
//! - The [`IsIsolated`](op::IsIsolated) trait checks if a vertex is isolated.
//! - The [`IsOriented`](op::IsOriented) trait checks if a digraph is oriented.
//! - The [`IsPendant`](op::IsPendant) trait checks if a vertex is a pendant.
//! - The [`IsRegular`](op::IsRegular) trait checks if a digraph is regular.
//! - The [`IsSemicomplete`](op::IsSemicomplete) trait checks if a digraph is
//!   semicomplete.
//! - The [`IsSubdigraph`](op::IsSubdigraph) trait checks if a digraph is a
//!   subdigraph.
//! - The [`IsSuperdigraph`](op::IsSuperdigraph) trait checks if a digraph is a
//!   superdigraph.
//! - The [`IsSymmetric`](op::IsSymmetric) trait checks if a digraph is
//!   symmetric.
//! - The [`IsWalk`](op::IsWalk) trait checks if a sequence of vertices is a
//!   walk in a digraph.
//!
//! # Algorithms
//!
//! The [`algo`] module provides digraph algorithms.
//!
//! ## Bellman-Ford-Moore
//!
//! The Bellman-Ford-Moore algorithm finds the shortest paths in a weighted
//! digraph with negative weights.
//!
//! - The [`single_source_distances`](algo::bellman_ford_moore::single_source_distances)
//!   function finds the shortest distances.
//!
//! ## Breadth-first search (BFS)
//!
//! A breadth-first search explores the vertices of an unweighted digraph in
//! order of their distance from a source.
//!
//! These functions start from one or more source vertices and allow a custom
//! step function, target predicate, distance array, breadth-first tree, and
//! queue, where applicable.
//!
//! - The [`distances`](algo::bfs::distances) function finds the shortest
//!   distances.
//! - The [`predecessors`](algo::bfs::predecessors) function finds the
//!   predecessors.
//! - The [`shortest_path`](algo::bfs::shortest_path) function finds the
//!   shortest path.
//!
//! These functions start from one source vertex.
//!
//! - The [`single_source_distances`](algo::bfs::single_source_distances)
//!   function finds the shortest distances.
//! - The [`single_source_predecessors`](algo::bfs::single_source_predecessors)
//!   function finds the predecessors.
//! - The [`single_pair_shortest_path`](algo::bfs::single_pair_shortest_path)
//!   function finds the shortest path.
//!
//! ## Depth-first search (DFS)
//!
//! A depth-first search explores the vertices of an unweighted digraph in order
//! of their depth from a source.
//!
//! - The [`dfsa`](algo::dfs::dfsa) function traverses the digraph.
//! - The [`dfsa_predecessors`](algo::dfs::dfsa_predecessors) function finds the
//!   predecessors.
//! - The [`acyclic_ordering`](algo::dfs::acyclic_ordering) function generates
//!   an acyclic ordering.
//!
//! ## Dijkstra's algorithm
//!
//! Dijkstra's algorithm finds the shortest paths from one or more source
//! vertices in a weighted digraph.
//!
//! These functions start from one or more source vertices and allow a custom
//! step function, target predicate, distance array, and heap, where
//! applicable.
//!
//! - The [`distances`](algo::dijkstra::distances) function finds the shortest
//!   distances.
//! - The [`predecessors`](algo::dijkstra::predecessors) function finds the
//!   predecessors.
//! - The [`shortest_path`](algo::dijkstra::shortest_path) function finds the
//!   shortest path.
//!
//! These functions start from one source vertex.
//!
//! - The [`single_source_distances`](algo::dijkstra::single_source_distances)
//!   function finds the shortest distances.
//! - The [`single_source_predecessors`](algo::dijkstra::single_source_predecessors)
//!   function finds the predecessors.
//! - The [`single_pair_shortest_path`](algo::dijkstra::single_pair_shortest_path)
//!   function finds the shortest path.
//!
//! ## Floyd-Warshall
//!
//! The Floyd-Warshall algorithm finds the shortest paths between all pairs
//! of vertices in a weighted digraph.
//!
//! - The [`distances`](algo::floyd_warshall::distances) function finds the
//!   shortest distances.
//!
//! ## Breadth-first tree
//!
//! A breadth-first tree is the result of a breadth-first search and contains
//! the predecessors of the vertices on the shortest paths.
//!
//! - The [`search`](algo::breadth_first_tree::BreadthFirstTree::search) method
//!   finds a vertex by value.
//! - The [`search_by`](algo::breadth_first_tree::BreadthFirstTree::search_by)
//!   method finds a vertex by predicate.
//!
//! These functions produce a breadth-first tree.
//!
//! - [`bfs::single_source_predecessors`](algo::bfs::single_source_predecessors)
//! - [`bfs::predecessors`](algo::bfs::predecessors)
//! - [`dijkstra::single_source_predecessors`](algo::dijkstra::single_source_predecessors)
//! - [`dijkstra::predecessors`](algo::dijkstra::predecessors)
//!
//! ## Distance matrix
//!
//! A distance matrix contains the shortest distances between all pairs of
//! vertices in a digraph.
//!
//! - The [`center`](algo::distance_matrix::DistanceMatrix::center) method finds
//!   the center of the digraph.
//! - The [`diameter`](algo::distance_matrix::DistanceMatrix::diameter) method
//!   finds the diameter of the digraph.
//! - The [`eccentricities`](algo::distance_matrix::DistanceMatrix::eccentricities)
//!   method returns the eccentricities of the vertices.
//! - The [`is_connected`](algo::distance_matrix::DistanceMatrix::is_connected)
//!   method checks if the digraph is connected.
//!
//! [Adjacency List]: `adjacency_list::Digraph`
//! [Adjacency Matrix]: `adjacency_matrix::Digraph`
//! [Weighted Adjacency List]: `adjacency_list_weighted::Digraph`

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

pub mod adjacency_list;
pub mod adjacency_list_weighted;
pub mod adjacency_matrix;
pub mod algo;
pub mod gen;
pub mod op;

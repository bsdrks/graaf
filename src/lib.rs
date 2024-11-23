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
//! - [`AdjacencyListWeighted`] represents a digraph as a vector of maps.
//!
//! ## Unweighted Dense Digraphs
//!
//! - [`AdjacencyMatrix`] represents a digraph as a matrix using a bit vector.
//!
//! ## Unweighted Sparse Digraphs
//!
//! - [`AdjacencyList`] represents a digraph as a vector of sets.
//! - [`AdjacencyMap`] represents a digraph as a map of sets.
//! - [`EdgeList`] represents a digraph as a vector of tuples.
//!
//! # Generators
//!
//! - [`Biclique`] generates a complete bipartite digraph.
//! - [`Circuit`] generates a circuit digraph.
//! - [`Complete`] generates a complete digraph.
//! - [`Cycle`] generates a bidirectional circuit.
//! - [`Empty`] generates a digraph without arcs.
//! - [`ErdosRenyi`] generates a random digraph.
//! - [`GrowingNetwork`] generates a growing network.
//! - [`Path`] generates a path digraph.
//! - [`RandomTournament`] generates a random tournament.
//! - [`Star`] generates a star digraph.
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
//! - [`Order`] counts the vertices in a digraph.
//! - [`OutNeighborsWeighted`] iterates a vertex's weighted out-neighbors.
//! - [`OutNeighbors`] iterates a vertex's out-neighbors.
//! - [`OutdegreeSequence`] iterates a digraph's outdegrees.
//! - [`Outdegree`] returns a vertex's outdegree.
//! - [`RemoveArc`] removes an arc from a digraph.
//! - [`SemidegreeSequence`] iterates a digraph's semidegrees.
//! - [`Sinks`] iterates a digraph's sinks.
//! - [`Size`] counts the arcs in a digraph.
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

//! - [`BellmanFordMoore::distances`] finds the shortest distances.
//!
//! ## Breadth-First Search
//!
//! A breadth-first search explores an unweighted digraph's vertices in order
//! of their distance from a source.
//!
//! - [`Bfs`] iterates the vertices.
//! - [`BfsDist`] iterates the vertices and their distance from the source.
//! - [`BfsPred`] iterates the vertices and their predecessors.
//! - [`BfsDist::distances`](BfsDist::distances) finds the shortest distances.
//! - [`BfsPred::cycles`](BfsPred::predecessors) returns the cycles along the
//!   shortest path.
//! - [`BfsPred::predecessors`](BfsPred::predecessors) finds the predecessors.
//! - [`BfsPred::shortest_path`](BfsPred::shortest_path) finds the shortest
//!   path.
//!
//! ## Depth-First Search
//!
//! A depth-first search explores an unweighted digraph's vertices in order of
//! their distance from a source.
//!
//! - [`Dfs`] iterates the vertices.
//! - [`DfsDist`] iterates the vertices and their distance from the source.
//! - [`DfsPred`] iterates the vertices and their predecessors.
//! - [`DfsPred::predecessors`](DfsPred::predecessors) finds the predecessors.
//!
//! ## Dijkstra
//!
//! Dijkstra's algorithm finds the shortest paths from one or more source
//! vertices in an arc-weighted digraph.
//!
//! - [`Dijkstra`] iterates the vertices.
//! - [`DijkstraDist`] iterates the vertices and their distance from the
//!   source.
//! - [`DijkstraPred`] iterates the vertices and their predecessors.
//! - [`DijkstraDist::distances`](DijkstraDist::distances) finds the shortest
//!   distances.
//! - [`DijkstraPred::predecessors`](DijkstraPred::predecessors) finds the
//!   predecessors.
//! - [`DijkstraPred::shortest_path`](DijkstraPred::shortest_path) finds the
//!   shortest path.
//!
//! ## Distance Matrix
//!
//! A [`DistanceMatrix`] contains the shortest distances between all vertex
//! pairs in a digraph.
//!
//! - [`DistanceMatrix::center`](DistanceMatrix::center) finds the digraph's
//!   center.
//! - [`DistanceMatrix::diameter`](DistanceMatrix::diameter) finds the
//!   digraph's diameter.
//! - [`DistanceMatrix::eccentricities`](DistanceMatrix::eccentricities) the
//!   vertices' eccentricities.
//! - [`DistanceMatrix::is_connected`](DistanceMatrix::is_connected) checks the
//!   digraph's connectedness.
//! - [`DistanceMatrix::periphery`](DistanceMatrix::periphery) finds the
//!   digraph's periphery.
//!
//! ## Floyd-Warshall
//!
//! The Floyd-Warshall algorithm finds the distance between each vertex pair in
//! an arc-weighted digraph.
//!
//! - [`FloydWarshall::distances`] finds the shortest distances.
//!
//! ## Johnson's Circuit-Finding Algorithm
//!
//! Johnson's circuit-finding algorithm finds a digraph's circuits.
//!
//! - [`Johnson75::circuits`] finds a digraph's circuits.
//!
//! ## Predecessor Tree
//!
//! A [`PredecessorTree`] is the result of a search and contains the vertices'
//! predecessors.
//!
//! - [`PredecessorTree::search`] finds a vertex by value.
//! - [`PredecessorTree::search_by`] finds a vertex by predicate.
//!
//! ## Tarjan
//!
//! Tarjan's algorithm finds strongly connected components in a digraph.
//!
//! - [`Tarjan::components`] finds strongly connected components.

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
    GrowingNetwork,
    Path,
    RandomTournament,
    Star,
    Wheel,
};

pub use algo::{
    bellman_ford_moore::BellmanFordMoore,
    bfs::Bfs,
    bfs_dist::BfsDist,
    bfs_pred::BfsPred,
    dfs::Dfs,
    dfs_dist::DfsDist,
    dfs_pred::DfsPred,
    dijkstra::Dijkstra,
    dijkstra_dist::DijkstraDist,
    dijkstra_pred::DijkstraPred,
    distance_matrix::DistanceMatrix,
    floyd_warshall::FloydWarshall,
    johnson_75::Johnson75,
    predecessor_tree::PredecessorTree,
    tarjan::Tarjan,
};

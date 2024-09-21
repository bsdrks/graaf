# Graaf &emsp; [![Crates.io](https://img.shields.io/crates/v/graaf.svg)](https://crates.io/crates/graaf) [![Build status](https://github.com/bsdrks/graaf/actions/workflows/rust.yml/badge.svg)](https://github.com/bsdrks/graaf/actions) [![API reference](https://docs.rs/graaf/badge.svg)](https://docs.rs/graaf) [![Coverage Status](https://coveralls.io/repos/github/bsdrks/graaf/badge.svg?branch=main)](https://coveralls.io/github/bsdrks/graaf?branch=main)

Rust-powered directed graphs.

## Table of Contents

- [Installation](#installation)
- [Representations](#representations)
- [Generators](#generators)
- [Operations](#operations)
- [Algorithms](#algorithms)
  - [Bellman-Ford-Moore](#bellman-ford-moore)
  - [Breadth-First Search](#breadth-first-search)
  - [Depth-First Search](#depth-first-search)
  - [Dijkstra](#dijkstra)
  - [Distance Matrix](#distance-matrix)
  - [Floyd-Warshall](#floyd-warshall)
  - [Johnson's Circuit-Finding Algorithm](#johnsons-circuit-finding-algorithm)
  - [Predecessor Tree](#predecessor-tree)
  - [Tarjan](#tarjan)
- [Changelog](#changelog)
- [License](#license)
- [Contact](#contact)
- [Links](#links)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
graaf = "0.92.0"
```

## Representations

### Arc-Weighted Sparse Digraphs

- [`AdjacencyListWeighted`] represents digraphs as a vector of maps.

### Unweighted Dense Digraphs

- [`AdjacencyMatrix`] represents digraphs as a matrix using a bit vector.

### Unweighted Sparse Digraphs

- [`AdjacencyList`] represents digraphs as a vector of sets.
- [`AdjacencyMap`] represents digraphs as a map of sets.
- [`EdgeList`] represents digraphs as a vector of tuples.

## Generators

- [`Biclique`] generates a complete bipartite digraph.
- [`Circuit`] generates a circuit digraph.
- [`Complete`] generates a complete digraph.
- [`Cycle`] generates a bidirectional circuit.
- [`Empty`] generates a digraph without arcs.
- [`ErdosRenyi`] generates a random digraph.
- [`GrowingNetwork`] generates a growing network digraph.
- [`Path`] generates a path digraph.
- [`RandomTournament`] generates a random tournament.
- [`Star`] generates a star digraph.
- [`Wheel`] generates a wheel digraph.

## Operations

- [`AddArcWeighted`] adds an arc to an arc-weighted digraph.
- [`AddArc`] adds an arc to an unweighted digraph.
- [`ArcWeight`] returns an arc's weight.
- [`ArcsWeighted`] iterates over a digraph's weighted arcs.
- [`Arcs`] iterates over a digraph's arcs.
- [`Complement`] returns a digraph's complement.
- [`Converse`] returns a digraph's converse.
- [`DegreeSequence`] iterates over a digraph's degrees.
- [`Degree`] returns a vertex's degree.
- [`FilterVertices`] filters a digraph's vertices.
- [`HasArc`] checks whether a digraph contains an arc.
- [`HasEdge`] checks whether a digraph contains an edge.
- [`HasWalk`] checks whether a digraph contains a walk.
- [`InNeighbors`] iterates over a vertex's in-neighbors.
- [`IndegreeSequence`] iterates over a digraph's indegrees.
- [`Indegree`] returns a vertex's indegree.
- [`IsBalanced`] checks whether a digraph is balanced.
- [`IsComplete`] checks whether a digraph is complete.
- [`IsIsolated`] checks whether a vertex is isolated.
- [`IsOriented`] checks whether a digraph is oriented.
- [`IsPendant`] checks whether a vertex is a pendant.
- [`IsRegular`] checks whether a digraph is regular.
- [`IsSemicomplete`] checks whether a digraph is semicomplete.
- [`IsSimple`] checks whether a digraph is simple.
- [`IsSpanningSubdigraph`] checks whether a digraph spans a superdigraph.
- [`IsSubdigraph`] checks whether a digraph is a subdigraph.
- [`IsSuperdigraph`] checks whether a digraph is a superdigraph.
- [`IsSymmetric`] checks whether a digraph is symmetric.
- [`IsTournament`] checks whether a digraph is a tournament.
- [`Order`] returns the number of vertices in a digraph.
- [`OutNeighborsWeighted`] iterates over a vertex's weighted out-neighbors.
- [`OutNeighbors`] iterates over a vertex's out-neighbors.
- [`OutdegreeSequence`] iterates over a digraph's outdegrees.
- [`Outdegree`] returns a vertex's outdegree.
- [`RemoveArc`] removes an arc from a digraph.
- [`SemidegreeSequence`] iterates over a digraph's semidegrees.
- [`Sinks`] iterates over a digraph's sinks.
- [`Size`] returns the number of arcs in a digraph.
- [`Sources`] iterates over a digraph's sources.
- [`Union`] returns the union of two digraphs.
- [`Vertices`] iterates over a digraph's vertices.

## Algorithms

### Bellman-Ford-Moore

The Bellman-Ford-Moore algorithm finds the shortest distances from a source vertex to all other vertices in an arc-weighted digraph with negative weights.

- [`BellmanFordMoore::distances`] finds the shortest distances.

### Breadth-First Search

A breadth-first search explores an unweighted digraph's vertices in order of their distance from a source.

- [`Bfs`] iterates the vertices.
- [`BfsDist`] iterates the vertices and their distance from the source.
- [`BfsPred`] iterates the vertices and their predecessors.
- [`BfsDist::distances`] finds the shortest distances.
- [`BfsPred::cycles`] returns the cycles along the shortest path.
- [`BfsPred::predecessors`] finds the predecessors.
- [`BfsPred::shortest_path`] finds the shortest path.

### Depth-First Search

A depth-first search explores an unweighted digraph's vertices in order of their depth from a source.

- [`Dfs`] iterates the vertices.
- [`DfsDist`] iterates the vertices and their distance from the source.
- [`DfsPred`] iterates the vertices and their predecessors.
- [`DfsPred::predecessors`] finds the predecessors.

### Dijkstra

Dijkstra's algorithm finds the shortest paths in an arc-weighted digraph.

- [`Dijkstra`] iterates the vertices.
- [`DijkstraDist`] iterates the vertices.
- [`DijkstraPred`] iterates the vertices and their predecessors.
- [`DijkstraDist::distances`] finds the shortest distances.
- [`DijkstraPred::predecessors`] finds the predecessors.
- [`DijkstraPred::shortest_path`] finds the shortest path.

### Distance Matrix

A [`DistanceMatrix`] contains the shortest distances between all vertex pairs in a digraph.

- [`DistanceMatrix::center`] finds the digraph's center.
- [`DistanceMatrix::diameter`] finds the digraph's diameter.
- [`DistanceMatrix::eccentricities`] returns the vertices' eccentricities.
- [`DistanceMatrix::is_connected`] checks the digraph's connectedness.
- [`DistanceMatrix::periphery`] finds the digraph's periphery.

### Floyd-Warshall

The Floyd-Warshall algorithm finds the distance between each vertex pair in an arc-weighted digraph.

- [`FloydWarshall::distances`] finds the shortest distances.

### Johnson's Circuit-Finding Algorithm

Johnson's circuit-finding algorithm finds all circuits in a digraph.

- [`Johnson75::circuits`] finds all circuits.

### Predecessor Tree

A [`PredecessorTree`] contains the vertex predecessors.

- [`PredecessorTree::search`] finds a vertex by value.
- [`PredecessorTree::search_by`] finds a vertex by predicate.

### Tarjan

Tarjan's algorithm finds strongly connected components in a digraph.

- [`Tarjan::components`] finds strongly connected components.

[`AddArcWeighted`]: https://docs.rs/graaf/latest/graaf/op/add_arc_weighted/trait.AddArcWeighted.html
[`AddArc`]: https://docs.rs/graaf/latest/graaf/op/add_arc/trait.AddArc.html
[`AdjacencyListWeighted`]: https://docs.rs/graaf/latest/graaf/repr/adjacency_list_weighted/struct.AdjacencyListWeighted.html
[`AdjacencyList`]: https://docs.rs/graaf/latest/graaf/repr/adjacency_list/struct.AdjacencyList.html
[`AdjacencyMap`]: https://docs.rs/graaf/latest/graaf/repr/adjacency_map/struct.AdjacencyMap.html
[`AdjacencyMatrix`]: https://docs.rs/graaf/latest/graaf/repr/adjacency_matrix/struct.AdjacencyMatrix.html
[`ArcWeight`]: https://docs.rs/graaf/latest/graaf/op/arc_weight/trait.ArcWeight.html
[`ArcsWeighted`]: https://docs.rs/graaf/latest/graaf/op/arcs_weighted/trait.ArcsWeighted.html
[`Arcs`]: https://docs.rs/graaf/latest/graaf/op/arcs/trait.Arcs.html
[`BellmanFordMoore::distances`]: https://docs.rs/graaf/latest/graaf/algo/bellman_ford_moore/struct.BellmanFordMoore#method.distances
[`BfsDist::distances`]: https://docs.rs/graaf/latest/graaf/algo/bfs_dist/struct.BfsDist.html#method.distances
[`BfsDist`]: https://docs.rs/graaf/latest/graaf/algo/bfs_dist/struct.BfsDist.html
[`BfsPred::cycles`]: https://docs.rs/graaf/latest/graaf/algo/bfs_pred/struct.BfsPred.html#method.cycles
[`BfsPred::predecessors`]: https://docs.rs/graaf/latest/graaf/algo/bfs_pred/struct.BfsPred.html#method.predecessors
[`BfsPred::shortest_path`]: https://docs.rs/graaf/latest/graaf/algo/bfs_pred/struct.BfsPred.html#method.shortest_path
[`BfsPred`]: https://docs.rs/graaf/latest/graaf/algo/bfs_pred/struct.BfsPred.html
[`Bfs`]: https://docs.rs/graaf/latest/graaf/algo/bfs/struct.Bfs.html
[`Biclique`]: https://docs.rs/graaf/latest/graaf/gen/biclique/trait.Biclique.html
[`Circuit`]: https://docs.rs/graaf/latest/graaf/gen/circuit/trait.Circuit.html
[`Complement`]: https://docs.rs/graaf/latest/graaf/op/complement/trait.Complement.html
[`Complete`]: https://docs.rs/graaf/latest/graaf/gen/complete/trait.Complete.html
[`Converse`]: https://docs.rs/graaf/latest/graaf/op/converse/trait.Converse.html
[`Cycle`]: https://docs.rs/graaf/latest/graaf/gen/cycle/trait.Cycle.html
[`DegreeSequence`]: https://docs.rs/graaf/latest/graaf/op/degree_sequence/trait.DegreeSequence.html
[`Degree`]: https://docs.rs/graaf/latest/graaf/op/degree/trait.Degree.html
[`DfsDist`]: https://docs.rs/graaf/latest/graaf/algo/dfs_dist/struct.DfsDist.html
[`DfsPred::predecessors`]: https://docs.rs/graaf/latest/graaf/algo/dfs_pred/struct.DfsPred.html#method.predecessors
[`DfsPred`]: https://docs.rs/graaf/latest/graaf/algo/dfs_pred/struct.DfsPred.html
[`Dfs`]: https://docs.rs/graaf/latest/graaf/algo/dfs/struct.Dfs.html
[`DijkstraDist::distances`]: https://docs.rs/graaf/latest/graaf/algo/dijkstra_dist/struct.DijkstraDist.html#method.distances
[`DijkstraDist`]: https://docs.rs/graaf/latest/graaf/algo/dijkstra_dist/struct.DijkstraDist.html
[`DijkstraPred::predecessors`]: https://docs.rs/graaf/latest/graaf/algo/dijkstra_pred/struct.DijkstraPred.html#method.predecessors
[`DijkstraPred::shortest_path`]: https://docs.rs/graaf/latest/graaf/algo/dijkstra_pred/struct.DijkstraPred.html#method.shortest_path
[`DijkstraPred`]: https://docs.rs/graaf/latest/graaf/algo/dijkstra_pred/struct.DijkstraPred.html
[`Dijkstra`]: https://docs.rs/graaf/latest/graaf/algo/dijkstra/struct.Dijkstra.html
[`DistanceMatrix::center`]: https://docs.rs/graaf/latest/graaf/algo/distance_matrix/struct.DistanceMatrix.html#method.center
[`DistanceMatrix::diameter`]: https://docs.rs/graaf/latest/graaf/algo/distance_matrix/struct.DistanceMatrix.html#method.diameter
[`DistanceMatrix::eccentricities`]: https://docs.rs/graaf/latest/graaf/algo/distance_matrix/struct.DistanceMatrix.html#method.eccentricities
[`DistanceMatrix::is_connected`]: https://docs.rs/graaf/latest/graaf/algo/distance_matrix/struct.DistanceMatrix.html#method.is_connected
[`DistanceMatrix::periphery`]: https://docs.rs/graaf/latest/graaf/algo/distance_matrix/struct.DistanceMatrix.html#method.periphery
[`DistanceMatrix`]: https://docs.rs/graaf/latest/graaf/algo/distance_matrix/struct.DistanceMatrix.html
[`EdgeList`]: https://docs.rs/graaf/latest/graaf/repr/edge_list/struct.EdgeList.html
[`Empty`]: https://docs.rs/graaf/latest/graaf/gen/empty/trait.Empty.html
[`ErdosRenyi`]: https://docs.rs/graaf/latest/graaf/gen/erdos_renyi/trait.ErdosRenyi.html
[`FilterVertices`]: https://docs.rs/graag/latest/graaf/op/filter_vertices/trait.FilterVertices.html
[`FloydWarshall::distances`]: https://docs.rs/graaf/latest/graaf/algo/floyd_warshall/struct.FloydWarshall.html#method.distances
[`GrowingNetwork`]: https://docs.rs/graaf/latest/graaf/gen/growing_network/trait.GrowingNetwork.html
[`HasArc`]: https://docs.rs/graaf/latest/graaf/op/has_arc/trait.HasArc.html
[`HasEdge`]: https://docs.rs/graaf/latest/graaf/op/has_edge/trait.HasEdge.html
[`HasWalk`]: https://docs.rs/graaf/latest/graaf/op/has_walk/trait.HasWalk.html
[`InNeighbors`]: https://docs.rs/graaf/latest/graaf/op/in_neighbors/trait.InNeighbors.html
[`IndegreeSequence`]: https://docs.rs/graaf/latest/graaf/op/indegree_sequence/trait.IndegreeSequence.html
[`Indegree`]: https://docs.rs/graaf/latest/graaf/op/indegree/trait.Indegree.html
[`IsBalanced`]: https://docs.rs/graaf/latest/graaf/op/is_balanced/trait.IsBalanced.html
[`IsComplete`]: https://docs.rs/graaf/latest/graaf/op/is_complete/trait.IsComplete.html
[`IsIsolated`]: https://docs.rs/graaf/latest/graaf/op/is_isolated/trait.IsIsolated.html
[`IsOriented`]: https://docs.rs/graaf/latest/graaf/op/is_oriented/trait.IsOriented.html
[`IsPendant`]: https://docs.rs/graaf/latest/graaf/op/is_pendant/trait.IsPendant.html
[`IsRegular`]: https://docs.rs/graaf/latest/graaf/op/is_regular/trait.IsRegular.html
[`IsSemicomplete`]: https://docs.rs/graaf/latest/graaf/op/is_semicomplete/trait.IsSemicomplete.html
[`IsSimple`]: https://docs.rs/graaf/latest/graaf/op/is_simple/trait.IsSimple.html
[`IsSpanningSubdigraph`]: https://docs.rs/graaf/latest/graaf/op/is_spanning_subdigraph/trait.IsSpanningSubdigraph.html
[`IsSubdigraph`]: https://docs.rs/graaf/latest/graaf/op/is_subdigraph/trait.IsSubdigraph.html
[`IsSuperdigraph`]: https://docs.rs/graaf/latest/graaf/op/is_superdigraph/trait.IsSuperdigraph.html
[`IsSymmetric`]: https://docs.rs/graaf/latest/graaf/op/is_symmetric/trait.IsSymmetric.html
[`IsTournament`]: https://docs.rs/graaf/latest/graaf/op/is_tournament/trait.IsTournament.html
[`Johnson75::circuits`]: https://docs.rs/graaf/latest/graaf/algo/johnson_75/struct.Johnson75.html#method.circuits
[`Order`]: https://docs.rs/graaf/latest/graaf/op/order/trait.Order.html
[`OutNeighborsWeighted`]: https://docs.rs/graaf/latest/graaf/op/out_neighbors_weighted/trait.OutNeighborsWeighted.html
[`OutNeighbors`]: https://docs.rs/graaf/latest/graaf/op/out_neighbors/trait.OutNeighbors.html
[`OutdegreeSequence`]: https://docs.rs/graaf/latest/graaf/op/outdegree_sequence/trait.OutdegreeSequence.html
[`Outdegree`]: https://docs.rs/graaf/latest/graaf/op/outdegree/trait.Outdegree.html
[`Path`]: https://docs.rs/graaf/latest/graaf/gen/path/trait.Path.html
[`PredecessorTree::search_by`]: https://docs.rs/graaf/latest/graaf/algo/predecessor_tree/struct.PredecessorTree.html#method.search_by
[`PredecessorTree::search`]: https://docs.rs/graaf/latest/graaf/algo/predecessor_tree/struct.PredecessorTree.html#method.search
[`PredecessorTree`]: https://docs.rs/graaf/latest/graaf/algo/predecessor_tree/struct.PredecessorTree.html
[`RandomTournament`]: https://docs.rs/graaf/latest/graaf/gen/random_tournament/trait.RandomTournament.html
[`RemoveArc`]: https://docs.rs/graaf/latest/graaf/op/remove_arc/trait.RemoveArc.html
[`SemidegreeSequence`]: https://docs.rs/graaf/latest/graaf/op/semidegree_sequence/trait.SemidegreeSequence.html
[`Sinks`]: https://docs.rs/graaf/latest/graaf/op/sinks/trait.Sinks.html
[`Size`]: https://docs.rs/graaf/latest/graaf/op/size/trait.Size.html
[`Sources`]: https://docs.rs/graaf/latest/graaf/op/sources/trait.Sources.html
[`Star`]: https://docs.rs/graaf/latest/graaf/gen/star/trait.Star.html
[`Tarjan::components`]: https://docs.rs/graaf/latest/graaf/algo/tarjan/struct.Tarjan.html#method.components
[`Union`]: https://docs.rs/graaf/latest/graaf/op/union/trait.Union.html
[`Vertices`]: https://docs.rs/graaf/latest/graaf/op/vertices/trait.Vertices.html
[`Wheel`]: https://docs.rs/graaf/latest/graaf/gen/wheel/trait.Wheel.html

## Changelog

See [CHANGELOG.md] for a list of changes.

## License

Licensed under [Apache License, Version 2.0] or [MIT license] at your option.

[CHANGELOG.md]: https://github.com/bsdrks/graaf/blob/main/CHANGELOG.md
[Apache License, Version 2.0]: LICENSE-APACHE
[MIT license]: LICENSE-MIT

## Contact

Feel free to reach out with questions or suggestions.

- [Email](mailto:bas.dirks@protonmail.com)
- [GitHub](https://github.com/bsdrks/graaf)

## Links

- [Coveralls](https://coveralls.io/github/bsdrks/graaf)
- [Crates.io](https://crates.io/crates/graaf)
- [Docs.rs](https://docs.rs/graaf)

# Graaf &emsp; [![Crates.io](https://img.shields.io/crates/v/graaf.svg)](https://crates.io/crates/graaf) [![Build status](https://github.com/bsdrks/graaf/actions/workflows/rust.yml/badge.svg)](https://github.com/bsdrks/graaf/actions) [![API reference](https://docs.rs/graaf/badge.svg)](https://docs.rs/graaf) [![Coverage Status](https://coveralls.io/repos/github/bsdrks/graaf/badge.svg?branch=main)](https://coveralls.io/github/bsdrks/graaf?branch=main)

Rust-powered directed graphs.

## Table of Contents

- [Installation](#installation)
- [Digraph Types](#digraph-types)
- [Creating Digraphs](#creating-digraphs)
- [Operations](#operations)
- [Algorithms](#algorithms)
  - [Bellman-Ford-Moore](#bellman-ford-moore)
  - [Breadth-First Search (BFS)](#breadth-first-search-bfs)
  - [Depth-First Search (DFS)](#depth-first-search-dfs)
  - [Dijkstra](#dijkstra)
  - [Floyd-Warshall](#floyd-warshall)
  - [Tarjan](#tarjan)
  - [Predecessor Tree](#predecessor-tree)
  - [Distance Matrix](#distance-matrix)
- [Changelog](#changelog)
- [License](#license)
- [Contact](#contact)
- [Links](#links)

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
graaf = "0.87.0"
```

## Digraph Types

Graaf provides representations of directed graphs. These types eagerly implement digraph [operations](#operations) and [algorithms](#algorithms).

- [`adjacency_list`](https://docs.rs/graaf/latest/graaf/adjacency_list/digraph/struct.Digraph.html) represents unweighted sparse digraphs.
- [`adjacency_matrix`](https://docs.rs/graaf/latest/graaf/adjacency_matrix/digraph/struct.Digraph.html) represents unweighted dense digraphs.
- [`adjacency_list_weighted`](https://docs.rs/graaf/latest/graaf/adjacency_list_weighted/digraph/struct.Digraph.html) represents arc-weighted sparse digraphs.
- [`edge_list`](https://docs.rs/graaf/latest/graaf/edge_list/digraph/struct.Digraph.html) represents unweighted sparse digraphs.

## Creating Digraphs

The [`gen`] module provides digraph generators. Each digraph representation can be constructed with the operations in the [`op`] module.

- [`Biclique`] generates a complete bipartite digraph.
- [`Circuit`] generates a circuit digraph.
- [`Complete`] generates a complete digraph.
- [`Cycle`] generates a bidirectional circuit.
- [`Empty`] generates a digraph with no arcs.
- [`ErdosRenyi`] generates a random digraph.
- [`Path`] generates a path digraph.
- [`RandomTournament`] generates a random tournament.
- [`Star`] generates a star digraph.

## Operations

The [`op`] module provides digraph operation traits. The [digraph types](#digraph-types) implement these traits. One can implement these traits for custom digraph types. Operations form the foundation for [algorithms](#algorithms).

- [`AddArcWeighted`] adds an arc to an arc-weighted digraph.
- [`AddArc`] adds an arc to an unweighted digraph.
- [`ArcWeight`] returns an arc's weight.
- [`ArcsWeighted`] returns a digraph's weighted arcs.
- [`Arcs`] returns a digraph's arcs.
- [`Complement`] returns a digraph's complement.
- [`Converse`] returns a digraph's converse.
- [`DegreeSequence`] returns a digraph's degree sequence.
- [`Degree`] returns a vertex's degree.
- [`HasArc`] checks whether a digraph contains an arc.
- [`HasEdge`] checks whether a digraph contains an edge.
- [`HasWalk`] checks whether a digraph contains a walk.
- [`InNeighbors`] returns a vertex's in-neighbors.
- [`IndegreeSequence`] returns a digraph's indegree sequence.
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
- [`OutNeighborsWeighted`] returns a vertex's weighted out-neighbors.
- [`OutNeighbors`] returns a vertex's out-neighbors.
- [`OutdegreeSequence`] returns a digraph's outdegree sequence.
- [`Outdegree`] returns a vertex's outdegree.
- [`RemoveArc`] removes an arc from a digraph.
- [`SemidegreeSequence`] returns a digraph's semidegree sequence.
- [`Sinks`] returns a digraph's sinks.
- [`Size`] returns the number of arcs in a digraph.
- [`Sources`] returns a digraph's sources.
- [`Vertices`] returns a digraph's vertices.

## Algorithms

The [`algo`] module provides digraph algorithms.

### Bellman-Ford-Moore

The Bellman-Ford-Moore algorithm finds the shortest paths in an arc-weighted digraph with negative weights.

- [`single_source_distances`](https://docs.rs/graaf/latest/graaf/algo/bellman_ford_moore/fn.single_source_distances.html) finds the shortest distances.

### Breadth-First Search (BFS)

A breadth-first search explores the vertices of an unweighted digraph in order of their distance from a source.

- [`Bfs`](https://docs.rs/graaf/latest/graaf/algo/bfs/struct.Bfs.html) iterates over the vertices.
- [`BfsDist`](https://docs.rs/graaf/latest/graaf/algo/bfs_dist/struct.BfsDist.html) iterates over the vertices and their distance from the source.
- [`BfsPred`](https://docs.rs/graaf/latest/graaf/algo/bfs_pred/struct.BfsPred.html) iterates over the vertices and their predecessors.
- [`BfsDist::distances`](https://docs.rs/graaf/latest/graaf/algo/bfs_dist/struct.BfsDist.html#method.distances) finds the shortest distances.
- [`BfsPred::predecessors`](https://docs.rs/graaf/latest/graaf/algo/bfs_pred/struct.BfsPred.html#method.predecessors) finds the predecessors.
- [`BfsPred::shortest_path`](https://docs.rs/graaf/latest/graaf/algo/bfs_pred/struct.BfsPred.html#method.shortest_path) finds the shortest path.

### Depth-First Search (DFS)

A depth-first search explores the vertices of an unweighted digraph in order of their depth from a source.

- [`Dfs`](https://docs.rs/graaf/latest/graaf/algo/dfs/struct.Dfs.html) iterates over the vertices.
- [`DfsDist`](https://docs.rs/graaf/latest/graaf/algo/dfs_dist/struct.DfsDist.html) iterates over the vertices and their distance from the source.
- [`DfsPred`](https://docs.rs/graaf/latest/graaf/algo/dfs_pred/struct.DfsPred.html) iterates over the vertices and their predecessors.

### Dijkstra

Dijkstra's algorithm finds the shortest paths in an arc-weighted digraph.

- [`Dijkstra`](https://docs.rs/graaf/latest/graaf/algo/dijkstra/struct.Dijkstra.html) iterates over the vertices.
- [`DijkstraDist`](https://docs.rs/graaf/latest/graaf/algo/dijkstra_dist/struct.DijkstraDist.html) iterates over the vertices.
- [`DijkstraPred`](https://docs.rs/graaf/latest/graaf/algo/dijkstra_pred/struct.DijkstraPred.html) iterates over the vertices and their predecessors.
- [`DijkstraDist::distances`](https://docs.rs/graaf/latest/graaf/algo/dijkstra_dist/struct.DijkstraDist.html#method.distances) finds the shortest distances.
- [`DijkstraPred::predecessors`](https://docs.rs/graaf/latest/graaf/algo/dijkstra_pred/struct.DijkstraPred.html#method.predecessors) finds the predecessors.
- [`DijkstraPred::shortest_path`](https://docs.rs/graaf/latest/graaf/algo/dijkstra_pred/struct.DijkstraPred.html#method.shortest_path) finds the shortest path.

### Floyd-Warshall

The Floyd-Warshall algorithm finds the shortest paths between all pairs of vertices in an arc-weighted digraph.

- [`distances`](https://docs.rs/graaf/latest/graaf/algo/floyd_warshall/fn.distances.html) finds the shortest distances.

### Tarjan

Tarjan's algorithm finds the strongly connected components in a digraph.

- [`strongly_connected_components`](https://docs.rs/graaf/latest/graaf/algo/tarjan/fn.strongly_connected_components.html) finds the strongly connected components.

### Predecessor Tree

A [`PredecessorTree`](https://docs.rs/graaf/latest/graaf/algo/types/predecessor_tree/struct.PredecessorTree.html) is the result of a breadth-first search.

- [`search`](https://docs.rs/graaf/latest/graaf/algo/types/predecessor_tree/struct.PredecessorTree.html#method.search) finds a vertex by value.
- [`search_by`](https://docs.rs/graaf/latest/graaf/algo/types/predecessor_tree/struct.PredecessorTree.html#method.search_by) finds a vertex by predicate.

These functions produce a predecessor tree.

- [`BfsPred::predecessors`](https://docs.rs/graaf/latest/graaf/algo/bfs_pred/struct.BfsPred.html#method.predecessors)
- [`DijkstraPred::predecessors`](https://docs.rs/graaf/latest/graaf/algo/dijkstra_pred/struct.DijkstraPred.html#method.predecessors)

### Distance Matrix

A [`DistanceMatrix`](https://docs.rs/graaf/latest/graaf/algo/types/distance_matrix/struct.DistanceMatrix.html) contains the shortest distances between all pairs of vertices in a digraph.

- [`center`](https://docs.rs/graaf/latest/graaf/algo/distance_matrix/struct.DistanceMatrix.html#method.center) finds the center of the digraph.
- [`diameter`](https://docs.rs/graaf/latest/graaf/algo/distance_matrix/struct.DistanceMatrix.html#method.diameter) finds the diameter of the digraph.
- [`eccentricities`](https://docs.rs/graaf/latest/graaf/algo/distance_matrix/struct.DistanceMatrix.html#method.eccentricities) returns the eccentricities of the vertices.
- [`is_connected`](https://docs.rs/graaf/latest/graaf/algo/distance_matrix/struct.DistanceMatrix.html#method.is_connected) checks if the digraph is connected.
- [`periphery`](https://docs.rs/graaf/latest/graaf/algo/distance_matrix/struct.DistanceMatrix.html#method.periphery) finds the periphery of the digraph.

[`AddArcWeighted`]: https://docs.rs/graaf/latest/graaf/op/add_arc_weighted/trait.AddArcWeighted.html
[`AddArc`]: https://docs.rs/graaf/latest/graaf/op/add_arc/trait.AddArc.html
[`ArcWeight`]: https://docs.rs/graaf/latest/graaf/op/arc_weight/trait.ArcWeight.html
[`ArcsWeighted`]: https://docs.rs/graaf/latest/graaf/op/arcs_weighted/trait.ArcsWeighted.html
[`Arcs`]: https://docs.rs/graaf/latest/graaf/op/arcs/trait.Arcs.html
[`Biclique`]: https://docs.rs/graaf/latest/graaf/gen/biclique/trait.Biclique.html
[`Circuit`]: https://docs.rs/graaf/latest/graaf/gen/circuit/trait.Circuit.html
[`Complement`]: https://docs.rs/graaf/latest/graaf/op/complement/trait.Complement.html
[`Complete`]: https://docs.rs/graaf/latest/graaf/gen/complete/trait.Complete.html
[`Converse`]: https://docs.rs/graaf/latest/graaf/op/converse/trait.Converse.html
[`Cycle`]: https://docs.rs/graaf/latest/graaf/gen/cycle/trait.Cycle.html
[`Degree`]: https://docs.rs/graaf/latest/graaf/op/degree/trait.Degree.html
[`DegreeSequence`]: https://docs.rs/graaf/latest/graaf/op/degree_sequence/trait.DegreeSequence.html
[`Empty`]: https://docs.rs/graaf/latest/graaf/gen/empty/trait.Empty.html
[`ErdosRenyi`]: https://docs.rs/graaf/latest/graaf/gen/erdos_renyi/trait.ErdosRenyi.html
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
[`Order`]: https://docs.rs/graaf/latest/graaf/op/order/trait.Order.html
[`OutNeighborsWeighted`]: https://docs.rs/graaf/latest/graaf/op/out_neighbors_weighted/trait.OutNeighborsWeighted.html
[`OutNeighbors`]: https://docs.rs/graaf/latest/graaf/op/out_neighbors/trait.OutNeighbors.html
[`Outdegree`]: https://docs.rs/graaf/latest/graaf/op/outdegree/trait.Outdegree.html
[`OutdegreeSequence`]: https://docs.rs/graaf/latest/graaf/op/outdegree_sequence/trait.OutdegreeSequence.html
[`Path`]: https://docs.rs/graaf/latest/graaf/gen/path/trait.Path.html
[`RandomTournament`]: https://docs.rs/graaf/latest/graaf/gen/random_tournament/trait.RandomTournament.html
[`RemoveArc`]: https://docs.rs/graaf/latest/graaf/op/remove_arc/trait.RemoveArc.html
[`SemidegreeSequence`]: https://docs.rs/graaf/latest/graaf/op/semidegree_sequence/trait.SemidegreeSequence.html
[`Sinks`]: https://docs.rs/graaf/latest/graaf/op/sinks/trait.Sinks.html
[`Size`]: https://docs.rs/graaf/latest/graaf/op/size/trait.Size.html
[`Sources`]: https://docs.rs/graaf/latest/graaf/op/sources/trait.Sources.html
[`Star`]: https://docs.rs/graaf/latest/graaf/gen/star/trait.Star.html
[`Vertices`]: https://docs.rs/graaf/latest/graaf/op/vertices/trait.Vertices.html
[`algo`]: https://docs.rs/graaf/latest/graaf/algo/index.html
[`gen`]: https://docs.rs/graaf/latest/graaf/gen/index.html
[`op`]: https://docs.rs/graaf/latest/graaf/op/index.html

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

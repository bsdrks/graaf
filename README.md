# ![Graaf!](/logo.png "Graaf") &emsp; [![Build status](https://github.com/bsdrks/graaf/actions/workflows/rust.yml/badge.svg)](https://github.com/bsdrks/graaf/actions) [![Crates.io](https://img.shields.io/crates/v/graaf.svg)](https://crates.io/crates/graaf) [![API reference](https://docs.rs/graaf/badge.svg)](https://docs.rs/graaf) [![Coverage Status](https://coveralls.io/repos/github/bsdrks/graaf/badge.svg?branch=main)](https://coveralls.io/github/bsdrks/graaf?branch=main)

Functions and types for working with graphs

**Graaf** is Dutch for

1. graph
2. count
3. dig

This crate is in alpha, and the API will change.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
graaf = "0.16.0"
```

## Usage

```rust
use graaf::{
    op::{
        AddEdge,
        Indegree,
        Outdegree,
    },
    repr::AdjacencyMatrix,
};

let mut adj = AdjacencyMatrix::<4>::new();

adj.add_edge(0, 1);
adj.add_edge(0, 2);
adj.add_edge(1, 3);
adj.add_edge(2, 3);

assert_eq!(adj.indegree(0), 0);
assert_eq!(adj.indegree(1), 1);
assert_eq!(adj.indegree(2), 1);
assert_eq!(adj.indegree(3), 2);

assert_eq!(adj.outdegree(0), 2);
assert_eq!(adj.outdegree(1), 1);
assert_eq!(adj.outdegree(2), 1);
assert_eq!(adj.outdegree(3), 0);
```

## Features

### Algorithms: [`algo`](https://docs.rs/graaf/latest/graaf/algo/index.html)

#### Breadth-first search: [`bfs`](https://docs.rs/graaf/latest/graaf/algo/bfs/index.html)

- [`distances_single_source`](https://docs.rs/graaf/latest/graaf/algo/bfs/fn.distances_single_source.html)
- [`distances`](https://docs.rs/graaf/latest/graaf/algo/bfs/fn.distances.html)
- [`predecessors_single_source`](https://docs.rs/graaf/latest/graaf/algo/bfs/fn.predecessors_single_source.html)
- [`predecessors`](https://docs.rs/graaf/latest/graaf/algo/bfs/fn.predecessors.html)

#### Dijkstra's algorithm: [`dijkstra`](https://docs.rs/graaf/latest/graaf/algo/dijkstra/index.html)

- [`distances_single_source`](https://docs.rs/graaf/latest/graaf/algo/dijkstra/fn.distances_single_source.html)
- [`distances`](https://docs.rs/graaf/latest/graaf/algo/dijkstra/fn.distances.html)
- [`predecessors_single_source`](https://docs.rs/graaf/latest/graaf/algo/dijkstra/fn.predecessors_single_source.html)
- [`predecessors`](https://docs.rs/graaf/latest/graaf/algo/dijkstra/fn.predecessors.html)

### Operations: [`op`](https://docs.rs/graaf/latest/graaf/op/index.html)

These traits are implemented for various graph representations built from standard library containers.

- [`AddEdge`](https://docs.rs/graaf/latest/graaf/op/add_edge/trait.AddEdge.html) adds an unweighted edge.
- [`AddWeightedEdge`](https://docs.rs/graaf/latest/graaf/op/add_weighted_edge/trait.AddWeightedEdge.html) adds a weighted edge.
- [`CountAllEdges`](https://docs.rs/graaf/latest/graaf/op/count_all_edges/trait.CountAllEdges.html) counts all edges.
- [`CountAllVertices`](https://docs.rs/graaf/latest/graaf/op/count_all_vertices/trait.CountAllVertices.html) counts all vertices.
- [`EdgeWeight`](https://docs.rs/graaf/latest/graaf/op/edge_weight/trait.EdgeWeight.html) gets the weight of an edge.
- [`Indegree`](https://docs.rs/graaf/latest/graaf/op/indegree/trait.Indegree.html) returns the indegree of a vertex.
- [`IsEdge`](https://docs.rs/graaf/latest/graaf/op/is_edge/trait.IsEdge.html) returns whether an edge exists.
- [`IsSimple`](https://docs.rs/graaf/latest/graaf/op/is_simple/trait.IsSimple.html) returns whether a graph is simple.
- [`IterAllEdges`](https://docs.rs/graaf/latest/graaf/op/iter_all_edges/trait.IterAllEdges.html) iterates over all unweighted edges.
- [`IterAllWeightedEdges`](https://docs.rs/graaf/latest/graaf/op/iter_all_weighted_edges/trait.IterAllWeightedEdges.html) iterates over all weighted edges.
- [`IterEdges`](https://docs.rs/graaf/latest/graaf/op/iter_edges/trait.IterEdges.html) iterates over all unweighted edges of a source vertex.
- [`IterVertices`](https://docs.rs/graaf/latest/graaf/op/iter_vertices/trait.IterVertices.html) iterates over all vertices.
- [`IterWeightedEdges`](https://docs.rs/graaf/latest/graaf/op/iter_weighted_edges/trait.IterWeightedEdges.html) iterates over all weighted edges of a source vertex.
- [`Outdegree`](https://docs.rs/graaf/latest/graaf/op/outdegree/trait.Outdegree.html) returns the outdegree of a vertex.
- [`RemoveEdge`](https://docs.rs/graaf/latest/graaf/op/remove_edge/trait.RemoveEdge.html) removes an edge.

### Representations: [`repr`](https://docs.rs/graaf/latest/graaf/repr/index.html)

#### Adjacency matrix, unweighted

- [`AdjacencyMatrix`](https://docs.rs/graaf/latest/graaf/repr/adjacency_matrix/index.html): an adjacency matrix representation of an unweighted directed graph stored as a bit array.

#### Adjacency list, unweighted

- `BTreeMap<usize, BTreeSet<usize>>`
- `BTreeMap<usize, Vec<usize>>`
- `HashMap<usize, HashSet<usize>>`
- `HashMap<usize, Vec<usize>>`
- `Vec<BTreeSet<usize>>`
- `Vec<HashSet<usize>>`
- `Vec<Vec<usize>>`
- `[BTreeSet<usize>; V]`
- `[BTreeSet<usize>]`
- `[HashSet<usize>; V]`
- `[HashSet<usize>]`
- `[Vec<usize>; V]`
- `[Vec<usize>]`

#### Adjacency list, weighted

- `BTreeMap<usize, BTreeMap<usize, W>>`
- `BTreeMap<usize, BTreeSet<(usize, W)>>`
- `BTreeMap<usize, Vec<(usize, W)>>`
- `HashMap<usize, HashMap<usize, W>>`
- `HashMap<usize, HashSet<(usize, W)>>`
- `HashMap<usize, Vec<(usize, W)>>`
- `Vec<BTreeMap<usize, W>>`
- `Vec<BTreeSet<(usize, W)>>`
- `Vec<HashMap<usize, W>>`
- `Vec<HashSet<(usize, W)>>`
- `Vec<Vec<(usize, W)>>`
- `[BTreeMap<usize, W>; V]`
- `[BTreeMap<usize, W>]`
- `[BTreeSet<(usize, W)>; V]`
- `[BTreeSet<(usize, W)>]`
- `[HashMap<usize, W>; V]`
- `[HashMap<usize, W>]`
- `[HashSet<(usize, W)>; V]`
- `[HashSet<(usize, W)>]`
- `[Vec<(usize, W)>; V]`
- `[Vec<(usize, W)>]`

#### Edge list, unweighted

- `BTreeSet<(usize, usize)>`
- `HashSet<(usize, usize)>`
- `Vec<(usize, usize)>`
- `[(usize, usize); V]`
- `[(usize, usize)]`

#### Edge list, weighted

- `BTreeSet<(usize, usize, W)>`
- `HashSet<(usize, usize, W)>`
- `Vec<(usize, usize, W)>`
- `[(usize, usize, W); V]`
- `[(usize, usize, W)]`

# ![Graaf!](/logo.png "Graaf") &emsp; [![Build status](https://github.com/bsdrks/graaf/actions/workflows/rust.yml/badge.svg)](https://github.com/bsdrks/graaf/actions) [![Crates.io](https://img.shields.io/crates/v/graaf.svg)](https://crates.io/crates/graaf) [![API reference](https://docs.rs/graaf/badge.svg)](https://docs.rs/graaf) [![Coverage Status](https://coveralls.io/repos/github/bsdrks/graaf/badge.svg?branch=main)](https://coveralls.io/github/bsdrks/graaf?branch=main)

Functions and types for working with graphs

**Graaf** is Dutch for

1. graph
2. count
3. dig

This crate is in alpha, and the API will change. See the [changelog](https://github.com/bsdrks/graaf/blob/main/CHANGELOG.md#provisional-roadmap) for a provisional roadmap.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
graaf = "0.25.2"
```

## Usage

```rust
use {
    graaf::{
        algo::bfs::single_pair_shortest_path,
        op::{
            AddEdge,
            Indegree,
        },
    },
    std::collections::HashSet,
};

let mut graph = [
    HashSet::new(), 
    HashSet::new(), 
    HashSet::new(), 
    HashSet::new()
];

// ╭───╮       ╭───╮
// │ 0 │   →   │ 1 │
// ╰───╯       ╰───╯
//   ↑           ↓
// ╭───╮       ╭───╮
// │ 3 │       │ 2 │
// ╰───╯       ╰───╯

graph.add_edge(3, 0);
graph.add_edge(0, 1);
graph.add_edge(1, 2);

assert_eq!(graph.indegree(0), 1);
assert_eq!(graph.indegree(1), 1);
assert_eq!(graph.indegree(2), 1);
assert_eq!(graph.indegree(3), 0);

let path = single_pair_shortest_path(&graph, 3, 2);

assert_eq!(path, Some(vec![3, 0, 1, 2]));
```

## Overview

### Algorithms

Common graph algorithms:

- [`bfs`](https://docs.rs/graaf/latest/graaf/algo/bfs/index.html): breadth-first search
- [`dijkstra`](https://docs.rs/graaf/latest/graaf/algo/dijkstra/index.html): Dijkstra's algorithm
- [`predecessor`](https://docs.rs/graaf/latest/graaf/algo/predecessor/index.html): predecessor search

### Operations

Graph operation traits and implementations:

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

### Generators

Generators for common graph types:

- [`Linear`](https://docs.rs/graaf/latest/graaf/gen/linear/trait.Linear.html): a linear graph, also known as a path graph.

### Features

- `adjacency_matrix`: a representation for dense graphs.
- `nightly`: required for `adjacency_matrix`.

To disable these features, add the following to your `Cargo.toml`:

```toml
[dependencies]
graaf = { version = "0.25.2", default-features = false }
```

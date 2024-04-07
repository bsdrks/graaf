# Graaf &emsp; [![Build status](https://github.com/bsdrks/graaf/actions/workflows/rust.yml/badge.svg)](https://github.com/bsdrks/graaf/actions) [![Crates.io](https://img.shields.io/crates/v/graaf.svg)](https://crates.io/crates/graaf) [![API reference](https://docs.rs/graaf/badge.svg)](https://docs.rs/graaf) [![Coverage Status](https://coveralls.io/repos/github/bsdrks/graaf/badge.svg?branch=main)](https://coveralls.io/github/bsdrks/graaf?branch=main)

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
graaf = "0.7.0"
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

### Algorithms: `algo`

#### Breadth-first search: `bfs`

- `min_distances_single_source`
- `min_distances`
- `predecessors_single_source`
- `predecessors`

#### Dijkstra's algorithm: `dijkstra`

- `min_distances_single_source`
- `min_distances`
- `predecessors_single_source`
- `predecessors`

### Operations: `op`

These traits are implemented for various graph representations built from standard library containers.

- `AddEdge` adds an unweighted edge.
- `AddWeightedEdge` adds a weighted edge.
- `CountAllEdges` counts all edges.
- `CountAllVertices` counts all vertices.
- `EdgeWeight` gets the weight of an edge.
- `Indegree` returns the indegree of a vertex.
- `IsEdge` returns whether an edge exists.
- `IterAllEdges` iterates over all unweighted edges.
- `IterAllWeightedEdges` iterates over all weighted edges.
- `IterEdges` iterates over all unweighted edges of a source vertex.
- `IterVertices` iterates over all vertices.
- `IterWeightedEdges` iterates over all weighted edges of a source vertex.
- `Outdegree` returns the outdegree of a vertex.
- `RemoveEdge` removes an edge.

### Representations: `repr`

#### Adjacency list, unweighted

- `Vec<Vec<usize>>`
- `Vec<HashSet<usize>>`
- `[Vec<usize>]`
- `[HashSet<usize>]`
- `[Vec<usize>; V]`
- `[HashSet<usize>; V]`
- `HashMap<usize, Vec<usize>>`
- `HashMap<usize, HashSet<usize>>`

#### Adjacency list, weighted

- `Vec<Vec<(usize, W)>>`
- `Vec<HashSet<(usize, W)>>`
- `Vec<HashMap<usize, W>>`
- `[Vec<(usize, W)>]`
- `[HashSet<(usize, W)>]`
- `[HashMap<usize, W>]`
- `[Vec<(usize, W)>; V]`
- `[HashSet<(usize, W)>; V]`
- `[HashMap<usize, W>; V]`
- `HashMap<usize, Vec<(usize, W)>>`
- `HashMap<usize, HashSet<(usize, W)>>`
- `HashMap<usize, HashMap<usize, W>>`

#### Adjacency matrix, unweighted

- `AdjacencyMatrix`: an adjacency matrix representation of an unweighted directed graph stored as a bit array.

#### Edge list, unweighted

- `Vec<(usize, usize)>`
- `[(usize, usize)]`
- `[(usize, usize); V]`
- `HashSet<(usize, usize)>`

#### Edge list, weighted

- `Vec<(usize, usize, W)>`
- `[(usize, usize, W)]`
- `[(usize, usize, W); V]`
- `HashSet<(usize, usize, W)>`

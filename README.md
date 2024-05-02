# ![Graaf!](/logo.png "Graaf") &emsp; [![Build status](https://github.com/bsdrks/graaf/actions/workflows/rust.yml/badge.svg)](https://github.com/bsdrks/graaf/actions) [![Crates.io](https://img.shields.io/crates/v/graaf.svg)](https://crates.io/crates/graaf) [![API reference](https://docs.rs/graaf/badge.svg)](https://docs.rs/graaf) [![Coverage Status](https://coveralls.io/repos/github/bsdrks/graaf/badge.svg?branch=main)](https://coveralls.io/github/bsdrks/graaf?branch=main)

Graph algorithms, operations, generators, and representations.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
graaf = "0.32.0"
```

To use stable Rust, disable the `adjacency_matrix` feature:

```toml
[dependencies]
graaf = { version = "0.32.0", default-features = false }
```

## Overview

### Operations

Build and query graphs made with standard collections, or implement the operation traits for your own types.

```rust
use {
    graaf::op::{
        AddEdge,
        Indegree,
        Outdegree,
        RemoveEdge,
    },
    std::collections::BTreeSet,
};

let mut graph = vec![BTreeSet::new(); 3];

// 1 ← 0 → 2

graph.add_edge(0, 1);
graph.add_edge(0, 2);

assert_eq!(graph.outdegree(0), 2);
assert_eq!(graph.indegree(1), 1);
assert_eq!(graph.indegree(2), 1);

graph.remove_edge(0, 1);

assert_eq!(graph.outdegree(0), 1);
assert_eq!(graph.indegree(1), 0);
assert_eq!(graph.indegree(2), 1);
```

### Algorithms

Search, traverse, and analyze graphs built from the types that implement the operation traits.

```rust
use graaf::algo::bfs::single_pair_shortest_path as spsp;

let graph = [Vec::new(), vec![0], vec![1], vec![0, 2]];

assert_eq!(spsp(&graph, 3, 0), Some(vec![3, 0]));
assert_eq!(spsp(&graph, 3, 1), Some(vec![3, 2, 1]));
assert_eq!(spsp(&graph, 3, 2), Some(vec![3, 2]));
assert_eq!(spsp(&graph, 0, 3), None);
```

### Representations

Use custom graph representations. An adjacency matrix representation is available with the `adjacency_matrix` feature.

```rust
use graaf::{
    op::{
        AddEdge,
        IsSimple,
    },
    repr::AdjacencyMatrix,
};

let mut graph = AdjacencyMatrix::<3>::new();

graph.add_edge(0, 1);

assert!(graph.is_simple());

graph.add_edge(1, 1);

assert!(!graph.is_simple());
```

### Generators

Generate parameterized graphs.

```rust
use graaf::gen::Cycle;

let graph = Vec<Vec<usize>>::cycle(5);

assert_eq!(graph, vec![
    vec![0, 1],
    vec![1, 2],
    vec![2, 3],
    vec![3, 4],
    vec![4, 0],
]);
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

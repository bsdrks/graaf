# ![Graaf!](/logo.png "Graaf") &emsp; [![Crates.io](https://img.shields.io/crates/v/graaf.svg)](https://crates.io/crates/graaf) [![Build status](https://github.com/bsdrks/graaf/actions/workflows/rust.yml/badge.svg)](https://github.com/bsdrks/graaf/actions) [![API reference](https://docs.rs/graaf/badge.svg)](https://docs.rs/graaf) [![Coverage Status](https://coveralls.io/repos/github/bsdrks/graaf/badge.svg?branch=main)](https://coveralls.io/github/bsdrks/graaf?branch=main)

Algorithms, operations, generators, and representations for directed graphs

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
graaf = "0.42.4"
```

## Overview

### Operations

Build and query graphs made with standard collections, or implement the
operation traits for your own types.

```rust
use {
    graaf::{
        gen::EmptyConst,
        op::*,
    },
    std::collections::BTreeSet,
};

let mut graph = <[BTreeSet<usize>; 3]>::empty();

graph.add_edge(0, 1);
graph.add_edge(0, 2);

assert_eq!(graph.degree(0), 2);
assert_eq!(graph.degree(1), 1);
assert_eq!(graph.degree(2), 1);
```

### Algorithms

Search, traverse, and analyze graphs built from the types that implement the
operation traits.

```rust
use graaf::algo::bfs::single_pair_shortest_path as spsp;

// 0  ←  1
// ↑     ↑
// 3  →  2

let graph = [Vec::new(), vec![0], vec![1], vec![0, 2]];

assert_eq!(spsp(&graph, 3, 0), Some(vec![3, 0]));
assert_eq!(spsp(&graph, 3, 1), Some(vec![3, 2, 1]));
assert_eq!(spsp(&graph, 3, 2), Some(vec![3, 2]));
assert_eq!(spsp(&graph, 0, 3), None);
```

### Representations

An adjacency matrix representation is available with the `adjacency_matrix`
feature.

```rust
use graaf::{
    op::*,
    repr::AdjacencyMatrix,
};

let mut graph = AdjacencyMatrix::<3>::new();

graph.add_edge(0, 1);
graph.add_edge(1, 1);

assert!(!graph.is_simple());
```

### Generators

Generate parameterized graphs.

```rust
use graaf::gen::*;

assert_eq!(Vec::<Vec<usize>>::empty(2), vec![Vec::new(), Vec::new()]);
assert_eq!(Vec::<Vec<usize>>::cycle(3), vec![vec![1], vec![2], vec![0]]);

assert_eq!(
    <[Vec::<usize>; 3]>::complete(),
    [vec![1, 2], vec![0, 2], vec![0, 1]]
);
```

## Features

`adjacency_matrix` enables the adjacency matrix representation. Requires nightly.

## Changelog

See [CHANGELOG.md](https://github.com/bsdrks/graaf/blob/main/CHANGELOG.md).

## License

Licensed under either the [Apache License, Version 2.0] or the [MIT license] at your option.

[Apache License, Version 2.0]: LICENSE-APACHE
[MIT license]: LICENSE-MIT

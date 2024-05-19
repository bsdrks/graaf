# ![Graaf](/logo.png "Graaf") &emsp; [![Crates.io](https://img.shields.io/crates/v/graaf.svg)](https://crates.io/crates/graaf) [![Build status](https://github.com/bsdrks/graaf/actions/workflows/rust.yml/badge.svg)](https://github.com/bsdrks/graaf/actions) [![API reference](https://docs.rs/graaf/badge.svg)](https://docs.rs/graaf) [![Coverage Status](https://coveralls.io/repos/github/bsdrks/graaf/badge.svg?branch=main)](https://coveralls.io/github/bsdrks/graaf?branch=main)

Work with directed graphs

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
graaf = "0.47.2"
```

## Overview

### Operations

Build and query digraphs.

```rust
use {
    graaf::{
        gen::EmptyConst,
        op::*,
    },
    std::collections::BTreeSet,
};

let mut graph = <[BTreeSet<usize>; 3]>::empty();

graph.add_arc(0, 1);
graph.add_arc(0, 2);

assert_eq!(graph.degree(0), 2);
assert_eq!(graph.degree(1), 1);
assert_eq!(graph.degree(2), 1);
```

### Algorithms

Search, traverse, and analyze digraphs.

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

graph.add_arc(0, 1);
graph.add_arc(1, 1);

assert!(!graph.is_simple());
```

### Generators

Generate parameterized digraphs.

```rust
use graaf::gen::*;

assert_eq!(
    Vec::<Vec<usize>>::empty(2),
    vec![Vec::new(), Vec::new()]
);

assert_eq!(
    Vec::<Vec<usize>>::cycle(3),
    vec![vec![1], vec![2], vec![0]]
);

assert_eq!(
    <[Vec::<usize>; 3]>::complete(),
    [vec![1, 2], vec![0, 2], vec![0, 1]]
);
```

Generate random digraphs.

```rust
use {
    graaf::{
        gen::RandomTournament,
        op::{
            Degree,
            Indegree,
            IterVertices,
            Order,
            Outdegree,
            Size,
        },
    },
    std::collections::BTreeSet,
};

let tournament = Vec::<BTreeSet<usize>>::random_tournament(4);

assert_eq!(tournament.size(), 6);
assert_eq!(tournament.order(), 4);

for s in tournament.iter_vertices() {
    assert_eq!(tournament.degree(s), 3);
    assert!((0..3).contains(&tournament.outdegree(s)));
    assert!((0..3).contains(&tournament.indegree(s)));
}
```

## Features

These features require nightly Rust.

- `adjacency_matrix` enables the adjacency matrix representation.

## Changelog

See [CHANGELOG.md] for a list of changes.

## License

Licensed under [Apache License, Version 2.0] or [MIT license] at your option.

[Apache License, Version 2.0]: LICENSE-APACHE
[MIT license]: LICENSE-MIT
[CHANGELOG.md]: https://github.com/bsdrks/graaf/blob/main/CHANGELOG.md

# Graaf &emsp; [![Crates.io](https://img.shields.io/crates/v/graaf.svg)](https://crates.io/crates/graaf) [![Build status](https://github.com/bsdrks/graaf/actions/workflows/rust.yml/badge.svg)](https://github.com/bsdrks/graaf/actions) [![API reference](https://docs.rs/graaf/badge.svg)](https://docs.rs/graaf) [![Coverage Status](https://coveralls.io/repos/github/bsdrks/graaf/badge.svg?branch=main)](https://coveralls.io/github/bsdrks/graaf?branch=main)

Build, search, and manipulate digraphs.

## Project goals

- A flexible API for digraph operations
- A comprehensive set of algorithms
- Generators for common digraphs
- Competitive performance
- Full documentation
- Extensive property tests
- Complete unit test and benchmark coverage

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
graaf = "0.57.1"
```

## Example

```rust
use {
    graaf::{
        algo::bfs::single_pair_shortest_path as spsp,
        gen::*,
        op::*,
        repr::*,
    },
    std::collections::BTreeSet,
};

// 0 -> {1, 2}
// 1 -> {}
// 2 -> {}

let mut digraph = <[BTreeSet<usize>; 3]>::empty();

digraph.add_arc(0, 1);
digraph.add_arc(0, 2);

assert_eq!(digraph.degree(0), 2);
assert_eq!(digraph.degree(1), 1);
assert_eq!(digraph.degree(2), 1);

// 0 -> {}
// 1 -> {0}
// 2 -> {1}
// 3 -> {0, 2}

let digraph = [Vec::new(), vec![0], vec![1], vec![0, 2]];

assert_eq!(spsp(&digraph, 3, 0), Some(vec![3, 0]));
assert_eq!(spsp(&digraph, 3, 1), Some(vec![3, 2, 1]));

// 0 -> {}
// 1 -> {}
// 2 -> {}

assert!(Vec::<Vec<usize>>::empty(3)
    .iter()
    .eq(&[Vec::new(), Vec::new(), Vec::new()]));

// 0 -> {1}
// 1 -> {2}
// 2 -> {0}

assert!(Vec::<Vec<usize>>::cycle(3)
    .iter()
    .eq(&[vec![1], vec![2], vec![0]]));

// 0 -> {1, 2}
// 1 -> {0, 2}
// 2 -> {0, 1}

assert!(<[Vec::<usize>; 3]>::complete()
    .iter()
    .eq(&[vec![1, 2], vec![0, 2], vec![0, 1]]));

let tournament = Vec::<BTreeSet<usize>>::random_tournament(4);

assert_eq!(tournament.size(), 6);
assert_eq!(tournament.order(), 4);

for s in tournament.iter_vertices() {
    assert_eq!(tournament.degree(s), 3);
    assert!((0..=2).contains(&tournament.outdegree(s)));
    assert!((0..=2).contains(&tournament.indegree(s)));
}

// 0 -> {1}
// 1 -> {1}

let mut digraph = AdjacencyMatrix::<3>::new();

digraph.add_arc(0, 1);
digraph.add_arc(1, 1);

assert!(!digraph.is_simple());
```

## Features

These features require nightly Rust.

- `adjacency_matrix` enables the adjacency matrix representation.

## Changelog

See [CHANGELOG.md] for a list of changes.

## License

Licensed under [Apache License, Version 2.0] or [MIT license] at your option.

[CHANGELOG.md]: https://github.com/bsdrks/graaf/blob/main/CHANGELOG.md
[Apache License, Version 2.0]: LICENSE-APACHE
[MIT license]: LICENSE-MIT

# 0.1.0

- Initial release

# 0.2.0

- Rename `Indegree.in_degree` to `Indegree.indegree`.
- Rename `Outdegree.out_degree` to `Outdegree.outdegree`.

# 0.2.1

## Features

- Add `dijkstra_sssp_unweighted`.

## API

- Export `algo`, `ops`, and `repr` modules.

## Documentation

- Make descriptions more consistent.
- Add missing documentation for the public API.

## Clippy

- Enable selected lints from `restriction` group.
- Group lints into groups, restrictions, `rustdoc`, and `rustc` lints.
- Use `core` and `alloc` imports over `std` where possible.

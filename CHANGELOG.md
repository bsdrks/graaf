# Changelog

## Unreleased

### Added

- Add doctest example for `dijkstra_sssp_weighted`.

## [0.2.3] - 2024-03-31

### Added

- Add `dijkstra_sssp_weighted`.
- Add GitHub Action on push to main and PRs on main.

### Changed

- Test `DijkstraUnweighted.dijkstra_sssp_unweighted` for every source vertex.
- Make `CHANGELOG.md` adhere to [keep a changelog](https://keepachangelog.com/en/1.0.0/).

## [0.2.2] - 2024-03-31

### Added

- Add doctest example for `DijkstraWeighted.dijkstra`.

### Fixed

- Fix trait descriptions in `README.md`.

## [0.2.1] - 2024-03-31

### Added

- Add `dijkstra_sssp_unweighted`.
- Add missing documentation for the public API.

### Changed

- Export `algo`, `ops`, and `repr` modules.
- Enable selected lints from `restriction` group.
- Group lints into groups, restrictions, `rustdoc`, and `rustc` lints.
- Use `core` and `alloc` imports over `std` where possible.

### Fixed

- Make descriptions more consistent.

## [0.2.0] - 2024-03-30

### Changed

- Rename `Indegree.in_degree` to `Indegree.indegree`.
- Rename `Outdegree.out_degree` to `Outdegree.outdegree`.

## [0.1.0] - 2024-03-30

- Initial release

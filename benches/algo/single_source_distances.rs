#![macro_use]
//! Benchmark different algorithms and types to calculate the distance from a
//! source vertex to all other vertices in an arc-weighted digraph.

use graaf::{
    adjacency_list::fixture::bang_jensen_94,
    adjacency_list_weighted::fixture::{
        bang_jensen_94_weighted_isize,
        bang_jensen_94_weighted_usize,
        bang_jensen_96_isize,
        bang_jensen_96_usize,
        bang_jensen_99,
        kattis_bryr_1_isize,
        kattis_bryr_1_usize,
        kattis_bryr_2_isize,
        kattis_bryr_2_usize,
        kattis_bryr_3_isize,
        kattis_bryr_3_usize,
        kattis_crosscountry_isize,
        kattis_crosscountry_usize,
        kattis_shortestpath1_isize,
        kattis_shortestpath1_usize,
    },
    algo::bellman_ford_moore,
};

fn main() {
    divan::main();
}

mod bang_jensen_94 {
    use {
        super::*,
        divan::Bencher,
        graaf::algo::Bfs,
        std::collections::{
            BTreeSet,
            VecDeque,
        },
    };

    #[divan::bench]
    fn bellman_ford_moore() {
        let _ = bellman_ford_moore::single_source_distances(
            &bang_jensen_94_weighted_isize(),
            0,
        );
    }

    #[divan::bench]
    fn bfs(bencher: Bencher<'_, '_>) {
        let digraph = bang_jensen_94();

        let mut bfs = Bfs::new(
            &digraph,
            VecDeque::from([(0, 0)]),
            |w| w + 1,
            BTreeSet::from([0]),
        );

        bencher.bench_local(|| {
            let _ = bfs.distances();
        });
    }

    #[divan::bench]
    fn dijkstra() {
        let _ = graaf::algo::dijkstra::single_source_distances(
            &bang_jensen_94_weighted_usize(),
            0,
        );
    }

    #[divan::bench]
    fn floyd_warshall() {
        let _ = graaf::algo::floyd_warshall::distances(
            &bang_jensen_94_weighted_isize(),
        )[0];
    }
}

mod bang_jensen_96 {
    use super::*;

    #[divan::bench]
    fn bellman_ford_moore() {
        let _ = bellman_ford_moore::single_source_distances(
            &bang_jensen_96_isize(),
            0,
        );
    }

    #[divan::bench]
    fn dijkstra() {
        let _ = graaf::algo::dijkstra::single_source_distances(
            &bang_jensen_96_usize(),
            0,
        );
    }

    #[divan::bench]
    fn floyd_warshall() {
        let _ =
            graaf::algo::floyd_warshall::distances(&bang_jensen_96_isize())[0];
    }
}

mod bang_jensen_99 {
    use super::*;

    #[divan::bench]
    fn bellman_ford_moore() {
        let _ =
            bellman_ford_moore::single_source_distances(&bang_jensen_99(), 0);
    }

    #[divan::bench]
    fn floyd_warshall() {
        let _ = graaf::algo::floyd_warshall::distances(&bang_jensen_99())[0];
    }
}

mod kattis_bryr_1 {
    use super::*;

    #[divan::bench]
    fn bellman_ford_moore() {
        let _ = bellman_ford_moore::single_source_distances(
            &kattis_bryr_1_isize(),
            0,
        );
    }

    #[divan::bench]
    fn dijkstra() {
        let _ = graaf::algo::dijkstra::single_source_distances(
            &kattis_bryr_1_usize(),
            0,
        );
    }

    #[divan::bench]
    fn floyd_warshall() {
        let _ =
            graaf::algo::floyd_warshall::distances(&kattis_bryr_1_isize())[0];
    }
}

mod kattis_bryr_2 {
    use super::*;

    #[divan::bench]
    fn bellman_ford_moore() {
        let _ = bellman_ford_moore::single_source_distances(
            &kattis_bryr_2_isize(),
            0,
        );
    }

    #[divan::bench]
    fn dijkstra() {
        let _ = graaf::algo::dijkstra::single_source_distances(
            &kattis_bryr_2_usize(),
            0,
        );
    }

    #[divan::bench]
    fn floyd_warshall() {
        let _ =
            graaf::algo::floyd_warshall::distances(&kattis_bryr_2_isize())[0];
    }
}

mod kattis_bryr_3 {
    use super::*;

    #[divan::bench]
    fn bellman_ford_moore() {
        let _ = bellman_ford_moore::single_source_distances(
            &kattis_bryr_3_isize(),
            0,
        );
    }

    #[divan::bench]
    fn dijkstra() {
        let _ = graaf::algo::dijkstra::single_source_distances(
            &kattis_bryr_3_usize(),
            0,
        );
    }

    #[divan::bench]
    fn floyd_warshall() {
        let _ =
            graaf::algo::floyd_warshall::distances(&kattis_bryr_3_isize())[0];
    }
}

mod kattis_crosscountry {
    use super::*;

    #[divan::bench]
    fn bellman_ford_moore() {
        let _ = bellman_ford_moore::single_source_distances(
            &kattis_crosscountry_isize(),
            0,
        );
    }

    #[divan::bench]
    fn dijkstra() {
        let _ = graaf::algo::dijkstra::single_source_distances(
            &kattis_crosscountry_usize(),
            0,
        );
    }

    #[divan::bench]
    fn floyd_warshall() {
        let _ = graaf::algo::floyd_warshall::distances(
            &kattis_crosscountry_isize(),
        )[0];
    }
}

mod kattis_shortestpath1 {
    use super::*;

    #[divan::bench]
    fn bellman_ford_moore() {
        let _ = bellman_ford_moore::single_source_distances(
            &kattis_shortestpath1_isize(),
            0,
        );
    }

    #[divan::bench]
    fn dijkstra() {
        let _ = graaf::algo::dijkstra::single_source_distances(
            &kattis_shortestpath1_usize(),
            0,
        );
    }

    #[divan::bench]
    fn floyd_warshall() {
        let _ = graaf::algo::floyd_warshall::distances(
            &kattis_shortestpath1_isize(),
        )[0];
    }
}

mod random_tournament {
    use {
        divan::Bencher,
        graaf::{
            adjacency_list,
            algo::Bfs,
            gen::RandomTournament,
        },
        std::collections::{
            BTreeSet,
            VecDeque,
        },
    };

    #[divan::bench]
    fn bfs(bencher: Bencher<'_, '_>) {
        let digraph = adjacency_list::Digraph::random_tournament(100);

        let mut bfs = Bfs::new(
            &digraph,
            VecDeque::from([(0, 0)]),
            |w| w + 1,
            BTreeSet::from([0]),
        );

        bencher.bench_local(|| {
            let _ = bfs.distances();
        });
    }
}

//! Digraph fixtures for testing and benchmarking.

extern crate alloc;

use alloc::collections::{
    BTreeMap,
    BTreeSet,
};

use crate::{
    gen::Empty,
    op::AddWeightedArc,
};

/// Bang-Jensen, J., & Gutin, G. Z. (2010). Digraphs theory, algorithms and
/// applications (pp. 94). Springer.
#[must_use]
pub fn bang_jensen_94() -> Vec<BTreeSet<usize>> {
    vec![
        BTreeSet::from([1, 2]),
        BTreeSet::from([3]),
        BTreeSet::from([1, 3, 4, 5]),
        BTreeSet::from([5]),
        BTreeSet::from([6]),
        BTreeSet::new(),
        BTreeSet::new(),
    ]
}

/// Bang-Jensen, J., & Gutin, G. Z. (2010). Digraphs theory, algorithms and
/// applications (pp. 96). Springer.
#[must_use]
pub fn bang_jensen_96() -> Vec<Vec<(usize, usize)>> {
    vec![
        vec![(1, 9), (2, 3)],
        vec![(2, 6), (3, 2)],
        vec![(1, 2), (4, 1)],
        vec![(5, 1)],
        vec![(2, 2), (3, 2), (5, 7)],
        vec![(3, 2)],
    ]
}

/// Bang-Jensen, J., & Gutin, G. Z. (2010). Digraphs theory, algorithms and
/// applications (pp. 96). Springer.
#[must_use]
pub fn bang_jensen_96_isize() -> Vec<Vec<(usize, isize)>> {
    vec![
        vec![(1, 9), (2, 3)],
        vec![(2, 6), (3, 2)],
        vec![(1, 2), (4, 1)],
        vec![(5, 1)],
        vec![(2, 2), (3, 2), (5, 7)],
        vec![(3, 2)],
    ]
}

/// Bang-Jensen, J., & Gutin, G. Z. (2010). Digraphs theory, algorithms and
/// applications (pp. 99). Springer.
#[must_use]
pub fn bang_jensen_99() -> Vec<Vec<(usize, isize)>> {
    vec![
        vec![(1, 8), (2, 4)],
        vec![(2, -5)],
        vec![(3, -2), (4, 4)],
        vec![(5, -2)],
        vec![(3, 10), (5, 9)],
        vec![(3, 5), (4, -3)],
    ]
}

/// Arnarson, A. B., & Erlendsson, U. F. (2019). Bridges (Sample Input 1). Kattis. [https://open.kattis.com/problems/bryr](https://open.kattis.com/problems/bridges).
#[must_use]
pub fn kattis_bryr_1() -> Vec<BTreeMap<usize, usize>> {
    let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(3);

    for (s, t, w) in &[(0, 1, 1), (1, 2, 1), (2, 0, 1)] {
        digraph.add_weighted_arc(*s, *t, *w);
        digraph.add_weighted_arc(*t, *s, *w);
    }

    digraph
}

/// Arnarson, A. B., & Erlendsson, U. F. (2019). Bridges (Sample Input 2). Kattis. [https://open.kattis.com/problems/bryr](https://open.kattis.com/problems/bridges).
#[must_use]
pub fn kattis_bryr_2() -> Vec<BTreeMap<usize, usize>> {
    let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(6);

    for (s, t, w) in &[
        (0, 3, 1),
        (1, 0, 1),
        (1, 2, 1),
        (3, 2, 1),
        (4, 3, 1),
        (4, 5, 1),
    ] {
        digraph.add_weighted_arc(*s, *t, *w);
        digraph.add_weighted_arc(*t, *s, *w);
    }

    digraph
}

/// Arnarson, A. B., & Erlendsson, U. F. (2019). Bridges (Sample Input 3). Kattis. [https://open.kattis.com/problems/bryr](https://open.kattis.com/problems/bridges).
#[must_use]
pub fn kattis_bryr_3() -> Vec<BTreeMap<usize, usize>> {
    let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(10);

    for (s, t, w) in &[
        (2, 9, 0),
        (3, 0, 0),
        (3, 4, 0),
        (3, 5, 0),
        (3, 7, 0),
        (4, 6, 1),
        (5, 8, 0),
        (6, 2, 0),
        (6, 5, 1),
        (6, 9, 1),
        (7, 1, 0),
        (8, 4, 1),
        (9, 1, 1),
    ] {
        digraph.add_weighted_arc(*s, *t, *w);
        digraph.add_weighted_arc(*t, *s, *w);
    }

    digraph
}

/// Heimark. K. J. S. (2018). Cross Country. Kattis. [https://open.kattis.com/problems/crosscountry](https://open.kattis.com/problems/crosscountry).
#[must_use]
pub fn kattis_crosscountry() -> Vec<Vec<(usize, usize)>> {
    vec![
        vec![(1, 1), (2, 3), (3, 14)],
        vec![(0, 2), (2, 4), (3, 22)],
        vec![(0, 3), (1, 10), (3, 7)],
        vec![(0, 13), (1, 8), (2, 2)],
    ]
}

/// Austrin. P. (2005). Single source shortest path, non-negative weights. Kattis. [https://open.kattis.com/problems/shortestpath1](https://open.kattis.com/problems/shortestpath1).
#[must_use]
pub fn kattis_shortestpath1() -> Vec<Vec<(usize, usize)>> {
    vec![vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]]
}

//! Digraph fixtures for testing and benchmarking.

use {
    crate::{
        gen::Empty,
        op::{
            AddArc,
            AddWeightedArc,
        },
    },
    std::collections::{
        BTreeMap,
        BTreeSet,
    },
};

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory, Algorithms
/// and Applications (2nd ed.). Springer, London, 34.
/// <https://doi.org/10.1007/978-1-84800-998-1>
///
/// ```text
/// 0 -> {4}
/// 1 -> {0}
/// 2 -> {1, 3, 5}
/// 3 -> {}
/// 4 -> {}
/// 5 -> {4}
/// ```
#[must_use]
pub fn bang_jensen_34() -> Vec<BTreeSet<usize>> {
    vec![
        BTreeSet::from([4]),
        BTreeSet::from([0]),
        BTreeSet::from([1, 3, 5]),
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::from([4]),
    ]
}

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory, Algorithms
/// and Applications (2nd ed.). Springer, London, 94.
/// <https://doi.org/10.1007/978-1-84800-998-1>
///
/// ```text
/// 0 -> {1, 2}
/// 1 -> {3}
/// 2 -> {1, 3, 4, 5}
/// 3 -> {5}
/// 4 -> {6}
/// 5 -> {}
/// 6 -> {}
/// ```
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

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory, Algorithms
/// and Applications (2nd ed.). Springer, London, 94.
/// <https://doi.org/10.1007/978-1-84800-998-1>
///
/// ```text
/// 0 -> {1, 2}
/// 1 -> {3}
/// 2 -> {1, 3, 4, 5}
/// 3 -> {5}
/// 4 -> {6}
/// 5 -> {}
/// 6 -> {}
/// ```
#[must_use]
pub fn bang_jensen_94_weighted_usize() -> Vec<BTreeMap<usize, usize>> {
    vec![
        BTreeMap::from([(1, 1), (2, 1)]),
        BTreeMap::from([(3, 1)]),
        BTreeMap::from([(1, 1), (3, 1), (4, 1), (5, 1)]),
        BTreeMap::from([(5, 1)]),
        BTreeMap::from([(6, 1)]),
        BTreeMap::new(),
        BTreeMap::new(),
    ]
}

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory, Algorithms
/// and Applications (2nd ed.). Springer, London, 94.
/// <https://doi.org/10.1007/978-1-84800-998-1>
///
/// ```text
/// 0 -> {1, 2}
/// 1 -> {3}
/// 2 -> {1, 3, 4, 5}
/// 3 -> {5}
/// 4 -> {6}
/// 5 -> {}
/// 6 -> {}
/// ```
#[must_use]
pub fn bang_jensen_94_weighted_isize() -> Vec<BTreeMap<usize, isize>> {
    vec![
        BTreeMap::from([(1, 1), (2, 1)]),
        BTreeMap::from([(3, 1)]),
        BTreeMap::from([(1, 1), (3, 1), (4, 1), (5, 1)]),
        BTreeMap::from([(5, 1)]),
        BTreeMap::from([(6, 1)]),
        BTreeMap::new(),
        BTreeMap::new(),
    ]
}

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory, Algorithms
/// and Applications (2nd ed.). Springer, London, 96.
/// <https://doi.org/10.1007/978-1-84800-998-1>
///
/// ```text
/// 0 -> {1 (9), 2 (3)}
/// 1 -> {2 (6), 3 (2)}
/// 2 -> {1 (2), 4 (1)}
/// 3 -> {5 (1)}
/// 4 -> {2 (2), 3 (2), 5 (7)}
/// 5 -> {3 (2)}
/// ```
#[must_use]
pub fn bang_jensen_96() -> Vec<BTreeMap<usize, usize>> {
    vec![
        BTreeMap::from([(1, 9), (2, 3)]),
        BTreeMap::from([(2, 6), (3, 2)]),
        BTreeMap::from([(1, 2), (4, 1)]),
        BTreeMap::from([(5, 1)]),
        BTreeMap::from([(2, 2), (3, 2), (5, 7)]),
        BTreeMap::from([(3, 2)]),
    ]
}

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory, Algorithms
/// and Applications (2nd ed.). Springer, London, 96.
/// <https://doi.org/10.1007/978-1-84800-998-1>
///
/// ```text
/// 0 -> {1 (9), 2 (3)}
/// 1 -> {2 (6), 3 (2)}
/// 2 -> {1 (2), 4 (1)}
/// 3 -> {5 (1)}
/// 4 -> {2 (2), 3 (2), 5 (7)}
/// 5 -> {3 (2)}
/// ```
#[must_use]
pub fn bang_jensen_96_isize() -> Vec<BTreeMap<usize, isize>> {
    vec![
        BTreeMap::from([(1, 9), (2, 3)]),
        BTreeMap::from([(2, 6), (3, 2)]),
        BTreeMap::from([(1, 2), (4, 1)]),
        BTreeMap::from([(5, 1)]),
        BTreeMap::from([(2, 2), (3, 2), (5, 7)]),
        BTreeMap::from([(3, 2)]),
    ]
}

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory, Algorithms
/// and Applications (2nd ed.). Springer, London, 99.
/// <https://doi.org/10.1007/978-1-84800-998-1>
///
/// ```text
/// 0 -> {1 (8), 2 (4)}
/// 1 -> {2 (-5)}
/// 2 -> {3 (-2), 4 (4)}
/// 3 -> {5 (-2)}
/// 4 -> {3 (10), 5 (9)}
/// 5 -> {3 (5), 4 (-3)}
/// ```
#[must_use]
pub fn bang_jensen_99() -> Vec<BTreeMap<usize, isize>> {
    vec![
        BTreeMap::from([(1, 8), (2, 4)]),
        BTreeMap::from([(2, -5)]),
        BTreeMap::from([(3, -2), (4, 4)]),
        BTreeMap::from([(5, -2)]),
        BTreeMap::from([(3, 10), (5, 9)]),
        BTreeMap::from([(3, 5), (4, -3)]),
    ]
}

/// Arnar Bjarni Arnarson and Unnar Freyr Erlendsson. 2019. Bridges (Sample
/// Input 1). Kattis. <https://open.kattis.com/problems/bryr>
///
/// ```text
/// 0 -> {1 (1), 2 (1)}
/// 1 -> {0 (1), 2 (1)}
/// 2 -> {0 (1), 1 (1)}
/// ```
#[must_use]
pub fn kattis_bryr_1() -> Vec<BTreeMap<usize, usize>> {
    let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(3);

    for (s, t, w) in &[(0, 1, 1), (1, 2, 1), (2, 0, 1)] {
        digraph.add_weighted_arc(*s, *t, *w);
        digraph.add_weighted_arc(*t, *s, *w);
    }

    digraph
}

/// Arnar Bjarni Arnarson and Unnar Freyr Erlendsson. 2019. Bridges (Sample
/// Input 1). Kattis. <https://open.kattis.com/problems/bryr>
///
/// ```text
/// 0 -> {1 (1), 2 (1)}
/// 1 -> {0 (1), 2 (1)}
/// 2 -> {0 (1), 1 (1)}
/// ```
#[must_use]
pub fn kattis_bryr_1_isize() -> Vec<BTreeMap<usize, isize>> {
    let mut digraph = Vec::<BTreeMap<usize, isize>>::empty(3);

    for (s, t, w) in &[(0, 1, 1), (1, 2, 1), (2, 0, 1)] {
        digraph.add_weighted_arc(*s, *t, *w);
        digraph.add_weighted_arc(*t, *s, *w);
    }

    digraph
}

/// Arnar Bjarni Arnarson and Unnar Freyr Erlendsson. 2019. Bridges (Sample
/// Input 2). Kattis. <https://open.kattis.com/problems/bryr>
///
/// ```text
/// 0 -> {3 (1), 1 (1)}
/// 1 -> {0 (1), 2 (1)}
/// 2 -> {1 (1), 3 (1)}
/// 3 -> {0 (1), 2 (1), 4 (1)}
/// 4 -> {3 (1), 5 (1)}
/// 5 -> {4 (1)}
/// ```
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

/// Arnar Bjarni Arnarson and Unnar Freyr Erlendsson. 2019. Bridges (Sample
/// Input 2). Kattis. <https://open.kattis.com/problems/bryr>
///
/// ```text
/// 0 -> {3 (1), 1 (1)}
/// 1 -> {0 (1), 2 (1)}
/// 2 -> {1 (1), 3 (1)}
/// 3 -> {0 (1), 2 (1), 4 (1)}
/// 4 -> {3 (1), 5 (1)}
/// 5 -> {4 (1)}
/// ```
#[must_use]
pub fn kattis_bryr_2_isize() -> Vec<BTreeMap<usize, isize>> {
    let mut digraph = Vec::<BTreeMap<usize, isize>>::empty(6);

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

/// Arnar Bjarni Arnarson and Unnar Freyr Erlendsson. 2019. Bridges (Sample
/// Input 3). Kattis. <https://open.kattis.com/problems/bryr>
///
/// ```text
/// 0 -> {3 (0)}
/// 1 -> {7 (0), 9 (1)}
/// 2 -> {6 (0)}
/// 3 -> {0 (0), 4 (0), 5 (0), 7 (0)}
/// 4 -> {3 (0), 6 (1), 8 (1)}
/// 5 -> {3 (0), 6 (1), 8 (0)}
/// 6 -> {2 (0), 4 (1), 5 (1), 9 (1)}
/// 7 -> {1 (0), 3 (0)}
/// 8 -> {4 (1), 5 (0)}
/// 9 -> {1 (1), 2 (0), 6 (1)}
/// ```
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

/// Arnar Bjarni Arnarson and Unnar Freyr Erlendsson. 2019. Bridges (Sample
/// Input 3). Kattis. <https://open.kattis.com/problems/bryr>
///
/// ```text
/// 0 -> {3 (0)}
/// 1 -> {7 (0), 9 (1)}
/// 2 -> {6 (0)}
/// 3 -> {0 (0), 4 (0), 5 (0), 7 (0)}
/// 4 -> {3 (0), 6 (1), 8 (1)}
/// 5 -> {3 (0), 6 (1), 8 (0)}
/// 6 -> {2 (0), 4 (1), 5 (1), 9 (1)}
/// 7 -> {1 (0), 3 (0)}
/// 8 -> {4 (1), 5 (0)}
/// 9 -> {1 (1), 2 (0), 6 (1)}
/// ```
#[must_use]
pub fn kattis_bryr_3_isize() -> Vec<BTreeMap<usize, isize>> {
    let mut digraph = Vec::<BTreeMap<usize, isize>>::empty(10);

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

/// Jeroen Bransen. 2015. Build Dependencies. Kattis.
/// <https://open.kattis.com/problems/builddeps>
///
/// ```text
/// 0 = gmp
/// 1 = solution
/// 2 = base
/// 3 = set
/// 4 = map
/// 5 = queue
/// ```
///
/// ```text
/// 0 -> {3, 4}
/// 1 -> {}
/// 2 -> {3, 4, 5}
/// 3 -> {1}
/// 4 -> {1}
/// 5 -> {1}
/// ```
#[must_use]
pub fn kattis_builddeps() -> Vec<BTreeSet<usize>> {
    vec![
        BTreeSet::from([3, 4]),
        BTreeSet::new(),
        BTreeSet::from([3, 4, 5]),
        BTreeSet::from([1]),
        BTreeSet::from([1]),
        BTreeSet::from([1]),
    ]
}

/// Karl Johan Sande Heimark. 2018. Cross Country. Kattis.
/// <https://open.kattis.com/problems/crosscountry>
///
/// ```text
/// 0 -> {1 (1), 2 (3), 3 (14)}
/// 1 -> {0 (2), 2 (4), 3 (22)}
/// 2 -> {0 (3), 1 (10), 3 (7)}
/// 3 -> {0 (13), 1 (8), 2 (2)}
/// ```
#[must_use]
pub fn kattis_crosscountry() -> Vec<Vec<(usize, usize)>> {
    vec![
        vec![(1, 1), (2, 3), (3, 14)],
        vec![(0, 2), (2, 4), (3, 22)],
        vec![(0, 3), (1, 10), (3, 7)],
        vec![(0, 13), (1, 8), (2, 2)],
    ]
}

/// Karl Johan Sande Heimark. 2018. Cross Country. Kattis.
/// <https://open.kattis.com/problems/crosscountry>
///
/// ```text
/// 0 -> {1 (1), 2 (3), 3 (14)}
/// 1 -> {0 (2), 2 (4), 3 (22)}
/// 2 -> {0 (3), 1 (10), 3 (7)}
/// 3 -> {0 (13), 1 (8), 2 (2)}
/// ```
#[must_use]
pub fn kattis_crosscountry_isize() -> Vec<BTreeMap<usize, isize>> {
    vec![
        BTreeMap::from([(1, 1), (2, 3), (3, 14)]),
        BTreeMap::from([(0, 2), (2, 4), (3, 22)]),
        BTreeMap::from([(0, 3), (1, 10), (3, 7)]),
        BTreeMap::from([(0, 13), (1, 8), (2, 2)]),
    ]
}

/// Arash Behpour. 2019. Escape Wall Maria. Kattis. (Sample Input 1)
/// <https://open.kattis.com/problems/escapewallmaria>
///
/// ```text
/// 5  -> {6, 9}
/// 6  -> {5}
/// 9  -> {5, 13}
/// 13 -> {9, 12}
/// ```
#[must_use]
pub fn kattis_escapewallmaria_1() -> Vec<BTreeSet<usize>> {
    let mut digraph = Vec::<BTreeSet<usize>>::empty(14);

    for (s, t) in &[(5, 6), (5, 9), (6, 5), (9, 5), (9, 13), (13, 9), (13, 12)] {
        digraph.add_arc(*s, *t);
    }

    digraph
}

/// Arash Behpour. 2019. Escape Wall Maria. Kattis. (Sample Input 2)
/// <https://open.kattis.com/problems/escapewallmaria>
///
/// ```text
/// 5  -> {6, 9}
/// 6  -> {5}
/// 9  -> {5}
/// 13 -> {9, 12}
/// 12 -> {13}
/// ```
#[must_use]
pub fn kattis_escapewallmaria_2() -> Vec<BTreeSet<usize>> {
    let mut digraph = Vec::<BTreeSet<usize>>::empty(14);

    for (s, t) in &[(5, 6), (5, 9), (6, 5), (9, 5), (13, 9), (13, 12), (12, 13)] {
        digraph.add_arc(*s, *t);
    }

    digraph
}

/// Arash Behpour. 2019. Escape Wall Maria. Kattis. (Sample Input 3)
/// <https://open.kattis.com/problems/escapewallmaria>
///
/// ```text
/// 1  -> {2, 5}
/// 2  -> {1, 6}
/// 5  -> {1, 6, 9}
/// 6  -> {2, 5}
/// 9  -> {5, 13}
/// 12 -> {13}
/// 13 -> {9, 12}
/// ```
#[must_use]
pub fn kattis_escapewallmaria_3() -> Vec<BTreeSet<usize>> {
    let mut digraph = Vec::<BTreeSet<usize>>::empty(14);

    for (s, t) in &[
        (1, 2),
        (1, 5),
        (2, 1),
        (2, 6),
        (5, 1),
        (5, 6),
        (5, 9),
        (6, 2),
        (6, 5),
        (9, 5),
        (9, 13),
        (12, 13),
        (13, 9),
        (13, 12),
    ] {
        digraph.add_arc(*s, *t);
    }

    digraph
}

/// Per Austrin. 2005. Single source shortest path, non-negative weights.
/// Kattis. <https://open.kattis.com/problems/shortestpath1>
///
/// ```text
/// 0 -> {1 (2)}
/// 1 -> {2 (2)}
/// 2 -> {}
/// 3 -> {0 (2)}
/// ```
#[must_use]
pub fn kattis_shortestpath1() -> Vec<Vec<(usize, usize)>> {
    vec![vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]]
}

/// Per Austrin. 2005. Single source shortest path, non-negative weights.
/// Kattis. <https://open.kattis.com/problems/shortestpath1>
///
/// ```text
/// 0 -> {1 (2)}
/// 1 -> {2 (2)}
/// 2 -> {}
/// 3 -> {0 (2)}
/// ```
#[must_use]
pub fn kattis_shortestpath1_isize() -> Vec<Vec<(usize, isize)>> {
    vec![vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]]
}

//! Weighted adjacency list fixtures for testing and benchmarking

use {
    super::Digraph,
    crate::adjacency_list::fixture::bang_jensen_94,
    std::collections::BTreeMap,
};

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory,
/// Algorithms and Applications (2nd ed.). Springer, London, 94.
/// <https://doi.org/10.1007/978-1-84800-998-1>
///
/// ```text
/// 0 -> {1 (1), 2 (1)}
/// 1 -> {3 (1)}
/// 2 -> {1 (1), 3 (1), 4 (1), 5 (1)}
/// 3 -> {5 (1)}
/// 4 -> {6 (1)}
/// 5 -> {}
/// 6 -> {}
/// ```
#[must_use]
pub fn bang_jensen_94_usize() -> Digraph<usize> {
    Digraph::from(bang_jensen_94())
}

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory,
/// Algorithms and Applications (2nd ed.). Springer, London, 94.
/// <https://doi.org/10.1007/978-1-84800-998-1>
///
/// ```text
/// 0 -> {1 (1), 2 (1)}
/// 1 -> {3 (1)}
/// 2 -> {1 (1), 3 (1), 4 (1), 5 (1)}
/// 3 -> {5 (1)}
/// 4 -> {6 (1)}
/// 5 -> {}
/// 6 -> {}
/// ```
#[must_use]
pub fn bang_jensen_94_isize() -> Digraph<isize> {
    Digraph::from(bang_jensen_94())
}

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory,
/// Algorithms and Applications (2nd ed.). Springer, London, 96.
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
pub fn bang_jensen_96_usize() -> Digraph<usize> {
    Digraph::from(vec![
        BTreeMap::from([(1, 9), (2, 3)]),
        BTreeMap::from([(2, 6), (3, 2)]),
        BTreeMap::from([(1, 2), (4, 1)]),
        BTreeMap::from([(5, 1)]),
        BTreeMap::from([(2, 2), (3, 2), (5, 7)]),
        BTreeMap::from([(3, 2)]),
    ])
}

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory,
/// Algorithms and Applications (2nd ed.). Springer, London, 96.
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
pub fn bang_jensen_96_isize() -> Digraph<isize> {
    Digraph::from(vec![
        BTreeMap::from([(1, 9), (2, 3)]),
        BTreeMap::from([(2, 6), (3, 2)]),
        BTreeMap::from([(1, 2), (4, 1)]),
        BTreeMap::from([(5, 1)]),
        BTreeMap::from([(2, 2), (3, 2), (5, 7)]),
        BTreeMap::from([(3, 2)]),
    ])
}

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory,
/// Algorithms and Applications (2nd ed.). Springer, London, 99.
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
pub fn bang_jensen_99() -> Digraph<isize> {
    Digraph::from(vec![
        BTreeMap::from([(1, 8), (2, 4)]),
        BTreeMap::from([(2, -5)]),
        BTreeMap::from([(3, -2), (4, 4)]),
        BTreeMap::from([(5, -2)]),
        BTreeMap::from([(3, 10), (5, 9)]),
        BTreeMap::from([(3, 5), (4, -3)]),
    ])
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
pub fn kattis_bryr_1_usize() -> Digraph<usize> {
    Digraph::from(vec![
        BTreeMap::from([(1, 1), (2, 1)]),
        BTreeMap::from([(0, 1), (2, 1)]),
        BTreeMap::from([(0, 1), (1, 1)]),
    ])
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
pub fn kattis_bryr_1_isize() -> Digraph<isize> {
    Digraph::from(vec![
        BTreeMap::from([(1, 1), (2, 1)]),
        BTreeMap::from([(0, 1), (2, 1)]),
        BTreeMap::from([(0, 1), (1, 1)]),
    ])
}

/// Arnar Bjarni Arnarson and Unnar Freyr Erlendsson. 2019. Bridges (Sample
/// Input 2). Kattis. <https://open.kattis.com/problems/bryr>
///
/// ```text
/// 0 -> {1 (1), 3 (1)}
/// 1 -> {0 (1), 2 (1)}
/// 2 -> {1 (1), 3 (1)}
/// 3 -> {0 (1), 2 (1), 4 (1)}
/// 4 -> {3 (1), 5 (1)}
/// 5 -> {4 (1)}
/// ```
#[must_use]
pub fn kattis_bryr_2_usize() -> Digraph<usize> {
    Digraph::from(vec![
        BTreeMap::from([(1, 1), (3, 1)]),
        BTreeMap::from([(0, 1), (2, 1)]),
        BTreeMap::from([(1, 1), (3, 1)]),
        BTreeMap::from([(0, 1), (2, 1), (4, 1)]),
        BTreeMap::from([(3, 1), (5, 1)]),
        BTreeMap::from([(4, 1)]),
    ])
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
pub fn kattis_bryr_2_isize() -> Digraph<isize> {
    Digraph::from(vec![
        BTreeMap::from([(1, 1), (3, 1)]),
        BTreeMap::from([(0, 1), (2, 1)]),
        BTreeMap::from([(1, 1), (3, 1)]),
        BTreeMap::from([(0, 1), (2, 1), (4, 1)]),
        BTreeMap::from([(3, 1), (5, 1)]),
        BTreeMap::from([(4, 1)]),
    ])
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
pub fn kattis_bryr_3_usize() -> Digraph<usize> {
    Digraph::from(vec![
        BTreeMap::from([(3, 0)]),
        BTreeMap::from([(7, 0), (9, 1)]),
        BTreeMap::from([(6, 0)]),
        BTreeMap::from([(0, 0), (4, 0), (5, 0), (7, 0)]),
        BTreeMap::from([(3, 0), (6, 1), (8, 1)]),
        BTreeMap::from([(3, 0), (6, 1), (8, 0)]),
        BTreeMap::from([(2, 0), (4, 1), (5, 1), (9, 1)]),
        BTreeMap::from([(1, 0), (3, 0)]),
        BTreeMap::from([(4, 1), (5, 0)]),
        BTreeMap::from([(1, 1), (2, 0), (6, 1)]),
    ])
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
pub fn kattis_bryr_3_isize() -> Digraph<isize> {
    Digraph::from(vec![
        BTreeMap::from([(3, 0)]),
        BTreeMap::from([(7, 0), (9, 1)]),
        BTreeMap::from([(6, 0)]),
        BTreeMap::from([(0, 0), (4, 0), (5, 0), (7, 0)]),
        BTreeMap::from([(3, 0), (6, 1), (8, 1)]),
        BTreeMap::from([(3, 0), (6, 1), (8, 0)]),
        BTreeMap::from([(2, 0), (4, 1), (5, 1), (9, 1)]),
        BTreeMap::from([(1, 0), (3, 0)]),
        BTreeMap::from([(4, 1), (5, 0)]),
        BTreeMap::from([(1, 1), (2, 0), (6, 1)]),
    ])
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
pub fn kattis_crosscountry_usize() -> Digraph<usize> {
    Digraph::from(vec![
        BTreeMap::from([(1, 1), (2, 3), (3, 14)]),
        BTreeMap::from([(0, 2), (2, 4), (3, 22)]),
        BTreeMap::from([(0, 3), (1, 10), (3, 7)]),
        BTreeMap::from([(0, 13), (1, 8), (2, 2)]),
    ])
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
pub fn kattis_crosscountry_isize() -> Digraph<isize> {
    Digraph::from(vec![
        BTreeMap::from([(1, 1), (2, 3), (3, 14)]),
        BTreeMap::from([(0, 2), (2, 4), (3, 22)]),
        BTreeMap::from([(0, 3), (1, 10), (3, 7)]),
        BTreeMap::from([(0, 13), (1, 8), (2, 2)]),
    ])
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
pub fn kattis_shortestpath1_usize() -> Digraph<usize> {
    Digraph::from(vec![
        BTreeMap::from([(1, 2)]),
        BTreeMap::from([(2, 2)]),
        BTreeMap::new(),
        BTreeMap::from([(0, 2)]),
    ])
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
pub fn kattis_shortestpath1_isize() -> Digraph<isize> {
    Digraph::from(vec![
        BTreeMap::from([(1, 2)]),
        BTreeMap::from([(2, 2)]),
        BTreeMap::new(),
        BTreeMap::from([(0, 2)]),
    ])
}

/// Per Austrin. 2005. Single source shortest path, negative weights.
/// Kattis. <https://open.kattis.com/problems/shortestpath3>
///
/// ```text
/// 0 -> {1 (999), 3 (2)}
/// 1 -> {2 (-2)}
/// 2 -> {1 (1)}
/// ```
#[must_use]
pub fn kattis_shortestpath3() -> Digraph<isize> {
    Digraph::from(vec![
        BTreeMap::from([(1, 999), (3, 2)]),
        BTreeMap::from([(2, -2)]),
        BTreeMap::from([(1, 1)]),
        BTreeMap::new(),
        BTreeMap::new(),
    ])
}

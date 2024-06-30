//! Weighted digraph fixtures for testing and benchmarking.

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
#[macro_export]
macro_rules! bang_jensen_94_weighted {
    () => {
        $crate::adjacency_list_weighted::Digraph::from(vec![
            std::collections::BTreeMap::from([(1, 1), (2, 1)]),
            std::collections::BTreeMap::from([(3, 1)]),
            std::collections::BTreeMap::from([(1, 1), (3, 1), (4, 1), (5, 1)]),
            std::collections::BTreeMap::from([(5, 1)]),
            std::collections::BTreeMap::from([(6, 1)]),
            std::collections::BTreeMap::new(),
            std::collections::BTreeMap::new(),
        ])
    };
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
#[macro_export]
macro_rules! bang_jensen_96 {
    () => {
        $crate::adjacency_list_weighted::Digraph::from(vec![
            std::collections::BTreeMap::from([(1, 9), (2, 3)]),
            std::collections::BTreeMap::from([(2, 6), (3, 2)]),
            std::collections::BTreeMap::from([(1, 2), (4, 1)]),
            std::collections::BTreeMap::from([(5, 1)]),
            std::collections::BTreeMap::from([(2, 2), (3, 2), (5, 7)]),
            std::collections::BTreeMap::from([(3, 2)]),
        ])
    };
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
#[macro_export]
macro_rules! bang_jensen_99 {
    () => {
        $crate::adjacency_list_weighted::Digraph::from(vec![
            std::collections::BTreeMap::from([(1, 8), (2, 4)]),
            std::collections::BTreeMap::from([(2, -5)]),
            std::collections::BTreeMap::from([(3, -2), (4, 4)]),
            std::collections::BTreeMap::from([(5, -2)]),
            std::collections::BTreeMap::from([(3, 10), (5, 9)]),
            std::collections::BTreeMap::from([(3, 5), (4, -3)]),
        ])
    };
}

/// Arnar Bjarni Arnarson and Unnar Freyr Erlendsson. 2019. Bridges (Sample
/// Input 1). Kattis. <https://open.kattis.com/problems/bryr>
///
/// ```text
/// 0 -> {1 (1), 2 (1)}
/// 1 -> {0 (1), 2 (1)}
/// 2 -> {0 (1), 1 (1)}
/// ```
#[macro_export]
macro_rules! kattis_bryr_1 {
    () => {
        $crate::adjacency_list_weighted::Digraph::from(vec![
            std::collections::BTreeMap::from([(1, 1), (2, 1)]),
            std::collections::BTreeMap::from([(0, 1), (2, 1)]),
            std::collections::BTreeMap::from([(0, 1), (1, 1)]),
        ])
    };
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
#[macro_export]
macro_rules! kattis_bryr_2 {
    () => {
        $crate::adjacency_list_weighted::Digraph::from(vec![
            std::collections::BTreeMap::from([(1, 1), (3, 1)]),
            std::collections::BTreeMap::from([(0, 1), (2, 1)]),
            std::collections::BTreeMap::from([(1, 1), (3, 1)]),
            std::collections::BTreeMap::from([(0, 1), (2, 1), (4, 1)]),
            std::collections::BTreeMap::from([(3, 1), (5, 1)]),
            std::collections::BTreeMap::from([(4, 1)]),
        ])
    };
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
#[macro_export]
macro_rules! kattis_bryr_3 {
    () => {
        $crate::adjacency_list_weighted::Digraph::from(vec![
            std::collections::BTreeMap::from([(3, 0)]),
            std::collections::BTreeMap::from([(7, 0), (9, 1)]),
            std::collections::BTreeMap::from([(6, 0)]),
            std::collections::BTreeMap::from([(0, 0), (4, 0), (5, 0), (7, 0)]),
            std::collections::BTreeMap::from([(3, 0), (6, 1), (8, 1)]),
            std::collections::BTreeMap::from([(3, 0), (6, 1), (8, 0)]),
            std::collections::BTreeMap::from([(2, 0), (4, 1), (5, 1), (9, 1)]),
            std::collections::BTreeMap::from([(1, 0), (3, 0)]),
            std::collections::BTreeMap::from([(4, 1), (5, 0)]),
            std::collections::BTreeMap::from([(1, 1), (2, 0), (6, 1)]),
        ])
    };
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
#[macro_export]
macro_rules! kattis_crosscountry {
    () => {
        $crate::adjacency_list_weighted::Digraph::from(vec![
            std::collections::BTreeMap::from([(1, 1), (2, 3), (3, 14)]),
            std::collections::BTreeMap::from([(0, 2), (2, 4), (3, 22)]),
            std::collections::BTreeMap::from([(0, 3), (1, 10), (3, 7)]),
            std::collections::BTreeMap::from([(0, 13), (1, 8), (2, 2)]),
        ])
    };
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
#[macro_export]
macro_rules! kattis_shortestpath1 {
    () => {
        $crate::adjacency_list_weighted::Digraph::from(vec![
            std::collections::BTreeMap::from([(1, 2)]),
            std::collections::BTreeMap::from([(2, 2)]),
            std::collections::BTreeMap::new(),
            std::collections::BTreeMap::from([(0, 2)]),
        ])
    };
}

#[allow(unused_imports)]
pub(crate) use {
    bang_jensen_94_weighted,
    bang_jensen_96,
    bang_jensen_99,
    kattis_bryr_1,
    kattis_bryr_2,
    kattis_bryr_3,
    kattis_crosscountry,
    kattis_shortestpath1,
};

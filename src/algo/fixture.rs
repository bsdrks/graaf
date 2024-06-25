//! Digraph fixtures for testing and benchmarking.

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory,
/// Algorithms and Applications (2nd ed.). Springer, London, 34.
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
#[macro_export]
macro_rules! bang_jensen_34 {
    () => {
        vec![
            std::collections::BTreeSet::from([4]),
            std::collections::BTreeSet::from([0]),
            std::collections::BTreeSet::from([1, 3, 5]),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::from([4]),
        ]
    };
}

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory,
/// Algorithms and Applications (2nd ed.). Springer, London, 94.
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
#[macro_export]
macro_rules! bang_jensen_94 {
    () => {
        vec![
            std::collections::BTreeSet::from([1, 2]),
            std::collections::BTreeSet::from([3]),
            std::collections::BTreeSet::from([1, 3, 4, 5]),
            std::collections::BTreeSet::from([5]),
            std::collections::BTreeSet::from([6]),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::new(),
        ]
    };
}

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory,
/// Algorithms and Applications (2nd ed.). Springer, London, 94.
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
#[macro_export]
macro_rules! bang_jensen_94_weighted {
    () => {
        vec![
            std::collections::BTreeMap::from([(1, 1), (2, 1)]),
            std::collections::BTreeMap::from([(3, 1)]),
            std::collections::BTreeMap::from([(1, 1), (3, 1), (4, 1), (5, 1)]),
            std::collections::BTreeMap::from([(5, 1)]),
            std::collections::BTreeMap::from([(6, 1)]),
            std::collections::BTreeMap::new(),
            std::collections::BTreeMap::new(),
        ]
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
        vec![
            std::collections::BTreeMap::from([(1, 9), (2, 3)]),
            std::collections::BTreeMap::from([(2, 6), (3, 2)]),
            std::collections::BTreeMap::from([(1, 2), (4, 1)]),
            std::collections::BTreeMap::from([(5, 1)]),
            std::collections::BTreeMap::from([(2, 2), (3, 2), (5, 7)]),
            std::collections::BTreeMap::from([(3, 2)]),
        ]
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
        vec![
            std::collections::BTreeMap::from([(1, 8), (2, 4)]),
            std::collections::BTreeMap::from([(2, -5)]),
            std::collections::BTreeMap::from([(3, -2), (4, 4)]),
            std::collections::BTreeMap::from([(5, -2)]),
            std::collections::BTreeMap::from([(3, 10), (5, 9)]),
            std::collections::BTreeMap::from([(3, 5), (4, -3)]),
        ]
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
        vec![
            std::collections::BTreeMap::from([(1, 1), (2, 1)]),
            std::collections::BTreeMap::from([(0, 1), (2, 1)]),
            std::collections::BTreeMap::from([(0, 1), (1, 1)]),
        ]
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
        vec![
            std::collections::BTreeMap::from([(3, 1), (1, 1)]),
            std::collections::BTreeMap::from([(0, 1), (2, 1)]),
            std::collections::BTreeMap::from([(1, 1), (3, 1)]),
            std::collections::BTreeMap::from([(0, 1), (2, 1), (4, 1)]),
            std::collections::BTreeMap::from([(3, 1), (5, 1)]),
            std::collections::BTreeMap::from([(4, 1)]),
        ]
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
        vec![
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
        ]
    };
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
#[macro_export]
macro_rules! kattis_builddeps {
    () => {
        vec![
            std::collections::BTreeSet::from([3, 4]),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::from([3, 4, 5]),
            std::collections::BTreeSet::from([1]),
            std::collections::BTreeSet::from([1]),
            std::collections::BTreeSet::from([1]),
        ]
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
        vec![
            std::collections::BTreeMap::from([(1, 1), (2, 3), (3, 14)]),
            std::collections::BTreeMap::from([(0, 2), (2, 4), (3, 22)]),
            std::collections::BTreeMap::from([(0, 3), (1, 10), (3, 7)]),
            std::collections::BTreeMap::from([(0, 13), (1, 8), (2, 2)]),
        ]
    };
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
#[macro_export]
macro_rules! kattis_escapewallmaria_1 {
    () => {
        vec![
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::from([6, 9]),
            std::collections::BTreeSet::from([5]),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::from([5, 13]),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::from([9, 12]),
        ]
    };
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
#[macro_export]
macro_rules! kattis_escapewallmaria_2 {
    () => {
        vec![
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::from([6, 9]),
            std::collections::BTreeSet::from([5]),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::from([5]),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::from([13]),
            std::collections::BTreeSet::from([9, 12]),
        ]
    };
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
#[macro_export]
macro_rules! kattis_escapewallmaria_3 {
    () => {
        vec![
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::from([2, 5]),
            std::collections::BTreeSet::from([1, 6]),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::from([1, 6, 9]),
            std::collections::BTreeSet::from([2, 5]),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::from([5, 13]),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::from([13]),
            std::collections::BTreeSet::from([9, 12]),
        ]
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
        vec![
            std::collections::BTreeMap::from([(1, 2)]),
            std::collections::BTreeMap::from([(2, 2)]),
            std::collections::BTreeMap::new(),
            std::collections::BTreeMap::from([(0, 2)]),
        ]
    };
}

#[allow(unused_imports)]
pub(crate) use {
    bang_jensen_34,
    bang_jensen_94,
    bang_jensen_94_weighted,
    bang_jensen_96,
    bang_jensen_99,
    kattis_bryr_1,
    kattis_bryr_2,
    kattis_bryr_3,
    kattis_builddeps,
    kattis_crosscountry,
    kattis_escapewallmaria_1,
    kattis_escapewallmaria_2,
    kattis_escapewallmaria_3,
    kattis_shortestpath1,
};

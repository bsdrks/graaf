//! Unweighted digraph fixtures for testing and benchmarking.

use {
    super::Digraph,
    std::collections::BTreeSet,
};

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
#[must_use]
pub fn bang_jensen_34() -> Digraph {
    Digraph::from(vec![
        BTreeSet::from([4]),
        BTreeSet::from([0]),
        BTreeSet::from([1, 3, 5]),
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::from([4]),
    ])
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
#[must_use]
pub fn bang_jensen_94() -> Digraph {
    Digraph::from(vec![
        BTreeSet::from([1, 2]),
        BTreeSet::from([3]),
        BTreeSet::from([1, 3, 4, 5]),
        BTreeSet::from([5]),
        BTreeSet::from([6]),
        BTreeSet::new(),
        BTreeSet::new(),
    ])
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
pub fn kattis_builddeps() -> Digraph {
    Digraph::from(vec![
        BTreeSet::from([3, 4]),
        BTreeSet::new(),
        BTreeSet::from([3, 4, 5]),
        BTreeSet::from([1]),
        BTreeSet::from([1]),
        BTreeSet::from([1]),
    ])
}
/// Arash Behpour. 2019. Escape Wall Maria. Kattis. (Sample Input 1)
/// <https://open.kattis.com/problems/escapewallmaria>
///
/// ```text
/// 0  -> {}
/// 1  -> {}
/// 2  -> {}
/// 3  -> {}
/// 4  -> {}
/// 5  -> {6, 9}
/// 6  -> {5}
/// 7  -> {}
/// 8  -> {}
/// 9  -> {5, 13}
/// 10 -> {}
/// 11 -> {}
/// 12 -> {}
/// 13 -> {9, 12}
/// ```
#[must_use]
pub fn kattis_escapewallmaria_1() -> Digraph {
    Digraph::from(vec![
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::from([6, 9]),
        BTreeSet::from([5]),
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::from([5, 13]),
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::from([9, 12]),
    ])
}

/// Arash Behpour. 2019. Escape Wall Maria. Kattis. (Sample Input 2)
/// <https://open.kattis.com/problems/escapewallmaria>
///
/// ```text
/// 0  -> {}
/// 1  -> {}
/// 2  -> {}
/// 3  -> {}
/// 4  -> {}
/// 5  -> {6, 9}
/// 6  -> {5}
/// 7  -> {}
/// 8  -> {}
/// 9  -> {5}
/// 10 -> {}
/// 11 -> {}
/// 12 -> {13}
/// 13 -> {9, 12}
/// ```
#[must_use]
pub fn kattis_escapewallmaria_2() -> Digraph {
    Digraph::from(vec![
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::from([6, 9]),
        BTreeSet::from([5]),
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::from([5]),
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::from([13]),
        BTreeSet::from([9, 12]),
    ])
}

/// Arash Behpour. 2019. Escape Wall Maria. Kattis. (Sample Input 3)
/// <https://open.kattis.com/problems/escapewallmaria>
///
/// ```text
/// 0  -> {}
/// 1  -> {2, 5}
/// 2  -> {1, 6}
/// 3  -> {}
/// 4  -> {}
/// 5  -> {1, 6, 9}
/// 6  -> {2, 5}
/// 7  -> {}
/// 8  -> {}
/// 9  -> {5, 13}
/// 10 -> {}
/// 11 -> {}
/// 12 -> {13}
/// 13 -> {9, 12}
/// ```
#[must_use]
pub fn kattis_escapewallmaria_3() -> Digraph {
    Digraph::from(vec![
        BTreeSet::new(),
        BTreeSet::from([2, 5]),
        BTreeSet::from([1, 6]),
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::from([1, 6, 9]),
        BTreeSet::from([2, 5]),
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::from([5, 13]),
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::from([13]),
        BTreeSet::from([9, 12]),
    ])
}

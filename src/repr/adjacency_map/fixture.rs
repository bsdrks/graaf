//! Adjacency list fixtures for testing and benchmarking
//!
//! [`bang_jensen_196`]:
//!
//! ![Bang-Jensen, 196](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bang_jensen_196.svg)
//!
//! [`bang_jensen_34`]:
//!
//! ![Bang-Jensen, 34](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bang_jensen_34.svg)
//!
//! [`bang_jensen_94`]:
//!
//! ![Bang-Jensen, 94](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bang_jensen_94.svg)
//!
//! [`kattis_builddeps`]:
//!
//! ![Kattis, builddeps](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_builddeps.svg)
//!
//! [`kattis_cantinaofbabel_1`]:
//!
//! ![Kattis, cantinaofbabel, sample 1](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_cantinaofbabel_1.svg)
//!
//! [`kattis_cantinaofbabel_2`]:
//!
//! ![Kattis, cantinaofbabel, sample 2](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_cantinaofbabel_2.svg)
//!
//! [`kattis_escapewallmaria_1`]:
//!
//! ![Kattis, escapewallmaria, sample 1](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_escapewallmaria_1.svg)
//!
//! [`kattis_escapewallmaria_2`]:
//!
//! ![Kattis, escapewallmaria, sample 2](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_escapewallmaria_2.svg)
//!
//! [`kattis_escapewallmaria_3`]:
//!
//! ![Kattis, escapewallmaria, sample 3](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_escapewallmaria_3.svg)

use {
    crate::AdjacencyMap,
    std::collections::BTreeSet,
};

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory,
/// Algorithms and Applications (2nd ed.). Springer, London, 196.
/// <https://doi.org/10.1007/978-1-84800-998-1>
///
/// ![Bang-Jensen, 196](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bang_jensen_196.svg)
#[must_use]
pub fn bang_jensen_196() -> AdjacencyMap {
    AdjacencyMap::from([
        BTreeSet::from([1, 4, 7]),
        BTreeSet::from([0, 2, 7]),
        BTreeSet::from([3]),
        BTreeSet::from([2, 4]),
        BTreeSet::from([2]),
        BTreeSet::from([6]),
        BTreeSet::from([7]),
        BTreeSet::from([5]),
    ])
}

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory,
/// Algorithms and Applications (2nd ed.). Springer, London, 34.
/// <https://doi.org/10.1007/978-1-84800-998-1>
///
/// ![Bang-Jensen, 34](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bang_jensen_34.svg)
#[must_use]
pub fn bang_jensen_34() -> AdjacencyMap {
    AdjacencyMap::from([
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
/// ![Bang-Jensen, 94](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bang_jensen_94.svg)
#[must_use]
pub fn bang_jensen_94() -> AdjacencyMap {
    AdjacencyMap::from([
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
/// ![Kattis, builddeps](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_builddeps.svg)
#[must_use]
pub fn kattis_builddeps() -> AdjacencyMap {
    AdjacencyMap::from([
        BTreeSet::from([3, 4]),
        BTreeSet::new(),
        BTreeSet::from([3, 4, 5]),
        BTreeSet::from([1]),
        BTreeSet::from([1]),
        BTreeSet::from([1]),
    ])
}

/// David Sturgill. 2015. Cantina of Babel. (Sample Input 1). Kattis.
/// <https://open.kattis.com/problems/cantinaofbabel>
///
/// ![Kattis, cantinaofbabel, sample 1](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_cantinaofbabel_1.svg)
#[must_use]
pub fn kattis_cantinaofbabel_1() -> AdjacencyMap {
    AdjacencyMap::from([
        BTreeSet::from([1]),
        BTreeSet::from([0, 2, 4]),
        BTreeSet::from([1]),
        BTreeSet::from([2, 4, 5, 7, 10, 11]),
        BTreeSet::from([3]),
        BTreeSet::from([6]),
        BTreeSet::from([5, 10]),
        BTreeSet::from([3]),
        BTreeSet::from([7, 10]),
        BTreeSet::from([7, 11]),
        BTreeSet::from([6]),
        BTreeSet::from([9]),
    ])
}

/// David Sturgill. 2015. Cantina of Babel. (Sample Input 2). Kattis.
/// <https://open.kattis.com/problems/cantinaofbabel>
///
/// ![Kattis, cantinaofbabel, sample 2](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_cantinaofbabel_2.svg)
#[must_use]
pub fn kattis_cantinaofbabel_2() -> AdjacencyMap {
    AdjacencyMap::from([
        BTreeSet::from([1]),
        BTreeSet::from([0, 7]),
        BTreeSet::from([0, 5, 7]),
        BTreeSet::from([4]),
        BTreeSet::from([3]),
        BTreeSet::from([3, 6]),
        BTreeSet::from([5]),
        BTreeSet::from([2]),
        BTreeSet::from([7, 9, 11]),
        BTreeSet::from([8]),
        BTreeSet::from([9, 11]),
        BTreeSet::from([10]),
    ])
}

/// Arash Behpour. 2019. Escape Wall Maria. Kattis. (Sample Input 1)
/// <https://open.kattis.com/problems/escapewallmaria>
///
/// ![Kattis, escapewallmaria, sample 1](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_escapewallmaria_1.svg)
#[must_use]
pub fn kattis_escapewallmaria_1() -> AdjacencyMap {
    AdjacencyMap::from([
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
        BTreeSet::new(),
        BTreeSet::new(),
    ])
}

/// Arash Behpour. 2019. Escape Wall Maria. Kattis. (Sample Input 2)
/// <https://open.kattis.com/problems/escapewallmaria>
///
/// ![Kattis, escapewallmaria, sample 2](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_escapewallmaria_2.svg)
#[must_use]
pub fn kattis_escapewallmaria_2() -> AdjacencyMap {
    AdjacencyMap::from([
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
        BTreeSet::new(),
        BTreeSet::new(),
    ])
}

/// Arash Behpour. 2019. Escape Wall Maria. Kattis. (Sample Input 3)
/// <https://open.kattis.com/problems/escapewallmaria>
///
/// ![Kattis, escapewallmaria, sample 3](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_escapewallmaria_3.svg)
#[must_use]
pub fn kattis_escapewallmaria_3() -> AdjacencyMap {
    AdjacencyMap::from([
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
        BTreeSet::new(),
        BTreeSet::new(),
    ])
}

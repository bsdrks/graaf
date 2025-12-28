//! Unweighted edge list fixtures for testing and benchmarking
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

use crate::{
    EdgeList,
    repr::adjacency_list::fixture,
};

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory,
/// Algorithms and Applications (2nd ed.). Springer, London, 196.
/// <https://doi.org/10.1007/978-1-84800-998-1>
///
/// ![Bang-Jensen, 196](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bang_jensen_196.svg)
#[must_use]
pub fn bang_jensen_196() -> EdgeList {
    EdgeList::from(fixture::bang_jensen_196())
}

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory,
/// Algorithms and Applications (2nd ed.). Springer, London, 34.
/// <https://doi.org/10.1007/978-1-84800-998-1>
#[must_use]
pub fn bang_jensen_34() -> EdgeList {
    EdgeList::from(fixture::bang_jensen_34())
}

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory,
/// Algorithms and Applications (2nd ed.). Springer, London, 94.
/// <https://doi.org/10.1007/978-1-84800-998-1>
#[must_use]
pub fn bang_jensen_94() -> EdgeList {
    EdgeList::from(fixture::bang_jensen_94())
}

/// Jeroen Bransen. 2015. Build Dependencies. Kattis.
/// <https://open.kattis.com/problems/builddeps>
#[must_use]
pub fn kattis_builddeps() -> EdgeList {
    EdgeList::from(fixture::kattis_builddeps())
}

/// David Sturgill. 2015. Cantina of Babel. (Sample Input 1). Kattis.
/// <https://open.kattis.com/problems/cantinaofbabel>
///
/// ![Kattis, cantinaofbabel, sample 1](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_cantinaofbabel_1.svg)
#[must_use]
pub fn kattis_cantinaofbabel_1() -> EdgeList {
    EdgeList::from(fixture::kattis_cantinaofbabel_1())
}

/// David Sturgill. 2015. Cantina of Babel. (Sample Input 2). Kattis.
/// <https://open.kattis.com/problems/cantinaofbabel>
///
/// ![Kattis, cantinaofbabel, sample 2](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_cantinaofbabel_2.svg)
#[must_use]
pub fn kattis_cantinaofbabel_2() -> EdgeList {
    EdgeList::from(fixture::kattis_cantinaofbabel_2())
}

/// Arash Behpour. 2019. Escape Wall Maria. Kattis. (Sample Input 1)
/// <https://open.kattis.com/problems/escapewallmaria>
///
/// ![Kattis, escapewallmaria, sample 1](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_escapewallmaria_1.svg)
#[must_use]
pub fn kattis_escapewallmaria_1() -> EdgeList {
    EdgeList::from(fixture::kattis_escapewallmaria_1())
}

/// Arash Behpour. 2019. Escape Wall Maria. Kattis. (Sample Input 2)
/// <https://open.kattis.com/problems/escapewallmaria>
///
/// ![Kattis, escapewallmaria, sample 2](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_escapewallmaria_2.svg)
#[must_use]
pub fn kattis_escapewallmaria_2() -> EdgeList {
    EdgeList::from(fixture::kattis_escapewallmaria_2())
}

/// Arash Behpour. 2019. Escape Wall Maria. Kattis. (Sample Input 3)
/// <https://open.kattis.com/problems/escapewallmaria>
///
/// ![Kattis, escapewallmaria, sample 3](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_escapewallmaria_3.svg)
#[must_use]
pub fn kattis_escapewallmaria_3() -> EdgeList {
    EdgeList::from(fixture::kattis_escapewallmaria_3())
}

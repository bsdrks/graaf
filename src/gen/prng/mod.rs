//! Pseudo-random number generators
//!
//! # Examples
//!
//! ```
//! use graaf::gen::prng::Xoshiro256StarStar;
//!
//! let mut rng = Xoshiro256StarStar::new(123);
//!
//! assert_ne!(rng.next(), rng.next());
//! ```

mod split_mix64;

pub mod xoshiro256_star_star;

use split_mix64::SplitMix64;

pub use xoshiro256_star_star::Xoshiro256StarStar;

//! # memchr-rs
//!
//! Fast memchr and memchr2 implementations in Rust.
//!
//! Originally extracted from [microsoft/edit](https://github.com/microsoft/edit) under the [MIT license](https://opensource.org/licenses/MIT).
//!
//! ## Usage
//!
//! Add the following crate to your `Cargo.toml` and enable either/both the `memchr` and `memchr2` feature as needed:
//!
//! ```toml
//! [dependencies]
//! memchr-rs = { version = "1", features = ["memchr"] }
//! ```
//!
//! Then, use the provided APIs to perform fast byte searches:
//!
//! ```
//! use memchr_rs::memchr;
//!
//! fn main() {
//!     let haystack = b"Hello, world!";
//!     let index = memchr(b'w', haystack, 0);
//!     assert_eq!(index, 7);
//!     println!("Found 'w' at position: {index}");
//! }
//! ```

#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]
#![cfg_attr(
    target_arch = "loongarch64",
    feature(stdarch_loongarch, stdarch_loongarch_feature_detection, loongarch_target_feature)
)]

#[cfg(feature = "memchr")]
mod memchr;
#[cfg(feature = "memchr2")]
mod memchr2;

#[cfg(feature = "memchr")]
pub use memchr::memchr;
#[cfg(feature = "memchr2")]
pub use memchr2::memchr2;

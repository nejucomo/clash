//! Create separate test instances for each each entry in a directory
//!
//! # Introduction
//!
//! Suppose we want to test many input cases of a parser and serializer. We will define the inputs and expected serializations in external files later.
//!
//! ## Test functions
//!
//! Test functions are annotated with the [test_case_dir] proc-macro. In our example case, we might specify it like this:
//!
//! ```ignore
//! use test_case_dir::test_case_dir;
//!
//! #[test_case_dir("test-vectors/deserialize-then-serialize")]
//! fn deserialize_then_serialize(input: &[u8], expected: &[u8]) {
//!   let obj = my_parser::parse_slice(input);
//!   let actual = obj.serialize_to_vec();
//!   assert_eq!(actual.as_slice(), expected);
//! }
//! ```
//!
//! The expansion uses the [`test-case`](https://docs.rs/test-case) crate, which this crate re-exports for the expansion to refer to, so the crate using [test_case_dir] need only depend on `test-case-dir`.
//!
//! ## Test Case Directories
//!
//! The macro call-side specifies a _Test Vector Directory_. In our example, this is: `test-vectors/deserialize-then-serialize/`. A relative path is resolved relative to the directory of the source file containing the macro call, just like [`include_bytes!`].
//!
//! Inside a Test Vector Directory, there must be one or more _Test Case Directories_. In our example, these might be:
//!
//! - `empty/`
//! - `hellow-world/`
//! - `nested/`
//!
//! Within each Test Case Directory there are one or more _Test Vector Files_. Their names must match the test function, so `input` and `expected` in our example.
//!
//! Each parameter must have type `&[u8]` or `&str` (with any lifetime), and receives its Test Vector File's contents via [`include_bytes!`] or [`include_str!`] respectively. The two may be mixed within one test function. A `&str` Test Vector File that is not valid UTF-8 is a compile error.
//!
//! It is a compile error if a Test Vector Directory contains anything other than Test Case Directories, or if a Test Case Directory is missing a Test Vector File for some parameter or contains any other entries.
//!
//! ## Macro Expansion
//!
//! Each of the Test Case Directories is translated into a separate `#[test]` function that uses the function's logic with different inputs. In fact, this is translated directly into `test-case` expansions. Our example would use this intermediate expansion (Test Case Directories are sorted by name and Test Vector Files are passed positionally in parameter order):
//!
//! ```text
//! #[test_case::test_case(
//!   include_bytes!("test-vectors/deserialize-then-serialize/empty/input"),
//!   include_bytes!("test-vectors/deserialize-then-serialize/empty/expected")
//!   ; "empty"
//! )]
//! #[test_case::test_case(
//!   include_bytes!("test-vectors/deserialize-then-serialize/hello-world/input"),
//!   include_bytes!("test-vectors/deserialize-then-serialize/hello-world/expected")
//!   ; "hello-world"
//! )]
//! #[test_case::test_case(
//!   include_bytes!("test-vectors/deserialize-then-serialize/nested/input"),
//!   include_bytes!("test-vectors/deserialize-then-serialize/nested/expected")
//!   ; "nested"
//! )]
//! fn deserialize_then_serialize(input: &[u8], expected: &[u8]) { ... }
//! ```
//!
//! A `&str` parameter would use `include_str!` in place of `include_bytes!`. The actual expansion uses absolute paths in the `include_*!` calls, and refers to `test_case` through a hidden re-export in this crate.
//!
//! ## Rebuilds
//!
//! Each Test Vector File is included with [`include_bytes!`] or [`include_str!`], so editing one triggers a rebuild. However, adding or removing Test Case Directories is not tracked by cargo, so a change to the test source file (or a `cargo clean`) is needed to pick up new cases.

pub use test_case_dir_macros::test_case_dir;

/// Not public API: items the [test_case_dir] expansion refers to
#[doc(hidden)]
pub mod __private {
    // The leading `::` selects the crate over the prelude's unstable built-in `#[test_case]` attribute.
    pub use ::test_case;
}

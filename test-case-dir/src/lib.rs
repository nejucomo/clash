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
//! ```
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
//! ## Test Case Directories
//!
//! The macro call-side specifies a _Test Vector Directory_. In our example, this is: `test-vectors/deserialize-then-serialize/`.
//!
//! Inside a Test Vector Directory, there must be one or more _Test Case Directories_. In our example, these might be:
//!
//! - `empty/`
//! - `hellow-world/`
//! - `nested/`
//!
//! Within each Test Case Directory there are one or more _Test Vector Files_. Their names must match the test function, so `input` and `expected` in our example.
//!
//! ## Macro Expansion
//!
//! Each of the Test Case Directories is translated into a separate `#[test]` function that uses the function's logic with different inputs. In fact, this is translated directly into `test-case` expansions. Our example would use this intermediate expansion:
//!
//! ```
//! #[test_case::test_case(
//!   input = include_bytes!("test-vectors/deserialize-then-serialize/empty/input"),
//!   expected = include_bytes!("test-vectors/deserialize-then-serialize/empty/expected")
//!   ; "empty"
//! )]
//! #[test_case::test_case(
//!   input = include_bytes!("test-vectors/deserialize-then-serialize/hello-world/input"),
//!   expected = include_bytes!("test-vectors/deserialize-then-serialize/hello-world/expected")
//!   ; "hello-world"
//! )]
//! #[test_case::test_case(
//!   input = include_bytes!("test-vectors/deserialize-then-serialize/nested/input"),
//!   expected = include_bytes!("test-vectors/deserialize-then-serialize/nested/expected")
//!   ; "nested"
//! )]
//! fn deserialize_then_serialize(input: &[u8], expected: &[u8]) { ... }
//! ```

// TODO: implement the documented API.

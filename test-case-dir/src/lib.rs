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
//! The expansion uses the [`test-case`](https://docs.rs/test-case) crate, so the crate using [test_case_dir] must also depend on `test-case` (typically as a dev-dependency).
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
//! The actual expansion uses absolute paths in the `include_bytes!` calls.
//!
//! ## Rebuilds
//!
//! Each Test Vector File is included with [`include_bytes!`], so editing one triggers a rebuild. However, adding or removing Test Case Directories is not tracked by cargo, so a change to the test source file (or a `cargo clean`) is needed to pick up new cases.

use std::path::{Path, PathBuf};

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::ext::IdentExt;
use syn::{Error, FnArg, Ident, ItemFn, LitStr, Pat, Result, parse_macro_input};

/// Generate a `test-case` test for each Test Case Directory in the given Test Vector Directory
///
/// See the [crate docs](crate) for details.
#[proc_macro_attribute]
pub fn test_case_dir(attr: TokenStream, item: TokenStream) -> TokenStream {
    let dir = parse_macro_input!(attr as LitStr);
    let item = parse_macro_input!(item as ItemFn);
    expand(&dir, item)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn expand(dir: &LitStr, item: ItemFn) -> Result<proc_macro2::TokenStream> {
    let params = param_names(&item)?;
    let vector_dir = resolve_vector_dir(dir);
    let cases = read_case_dirs(dir, &vector_dir)?;

    let attrs = cases
        .iter()
        .map(|(name, path)| {
            let files = read_vector_files(dir, path, &params)?;
            Ok(quote! {
                #[test_case::test_case( #( include_bytes!(#files) ),* ; #name )]
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        #( #attrs )*
        #item
    })
}

/// The parameter names of the test function, which must all be plain identifiers
fn param_names(item: &ItemFn) -> Result<Vec<Ident>> {
    if item.sig.inputs.is_empty() {
        return Err(Error::new_spanned(
            &item.sig,
            "test_case_dir functions require at least one parameter",
        ));
    }
    item.sig
        .inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Typed(pt) => match &*pt.pat {
                Pat::Ident(pi) if pi.subpat.is_none() => Ok(pi.ident.unraw()),
                other => Err(Error::new_spanned(
                    other,
                    "test_case_dir parameters must be plain identifiers",
                )),
            },
            FnArg::Receiver(r) => Err(Error::new_spanned(
                r,
                "test_case_dir functions cannot take `self`",
            )),
        })
        .collect()
}

/// Resolve the Test Vector Directory relative to the calling source file's directory
///
/// If the calling source file is unavailable (e.g. in some IDE contexts), fall back to `CARGO_MANIFEST_DIR`.
fn resolve_vector_dir(dir: &LitStr) -> PathBuf {
    let rel = PathBuf::from(dir.value());
    if rel.is_absolute() {
        return rel;
    }
    let base = proc_macro::Span::call_site()
        .local_file()
        .and_then(|f| f.parent().map(Path::to_path_buf))
        .map(|p| {
            if p.is_absolute() {
                p
            } else {
                // `local_file` may be relative to the workspace root (rustc's cwd), not the manifest dir; rustc's cwd is also the proc-macro's cwd.
                std::env::current_dir().map(|cwd| cwd.join(&p)).unwrap_or(p)
            }
        })
        .or_else(|| std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from))
        .unwrap_or_default();
    base.join(rel)
}

/// The sorted `(name, path)` pairs of Test Case Directories
fn read_case_dirs(dir: &LitStr, vector_dir: &Path) -> Result<Vec<(String, PathBuf)>> {
    let err = |msg: String| Error::new(dir.span(), msg);
    let entries = std::fs::read_dir(vector_dir)
        .map_err(|e| err(format!("cannot read {}: {e}", vector_dir.display())))?;

    let mut cases = vec![];
    for entry in entries {
        let entry = entry.map_err(|e| err(format!("cannot read {}: {e}", vector_dir.display())))?;
        let path = entry.path();
        if !path.is_dir() {
            return Err(err(format!(
                "not a Test Case Directory: {}",
                path.display()
            )));
        }
        let name = entry
            .file_name()
            .into_string()
            .map_err(|n| err(format!("non-UTF-8 Test Case Directory name: {n:?}")))?;
        cases.push((name, path));
    }
    if cases.is_empty() {
        return Err(err(format!(
            "no Test Case Directories in {}",
            vector_dir.display()
        )));
    }
    cases.sort();
    Ok(cases)
}

/// The absolute Test Vector File paths, in parameter order, as string literals
fn read_vector_files(dir: &LitStr, case_dir: &Path, params: &[Ident]) -> Result<Vec<LitStr>> {
    let err = |msg: String| Error::new(dir.span(), msg);

    let mut files = vec![];
    for param in params {
        let path = case_dir.join(param.to_string());
        if !path.is_file() {
            return Err(err(format!("missing Test Vector File: {}", path.display())));
        }
        let s = path
            .to_str()
            .ok_or_else(|| err(format!("non-UTF-8 path: {}", path.display())))?;
        files.push(LitStr::new(s, Span::call_site()));
    }

    let entries = std::fs::read_dir(case_dir)
        .map_err(|e| err(format!("cannot read {}: {e}", case_dir.display())))?;
    for entry in entries {
        let entry = entry.map_err(|e| err(format!("cannot read {}: {e}", case_dir.display())))?;
        let name = entry.file_name();
        if !params.iter().any(|p| name == p.to_string().as_str()) {
            return Err(err(format!(
                "unexpected entry (no matching parameter): {}",
                entry.path().display()
            )));
        }
    }

    Ok(files)
}

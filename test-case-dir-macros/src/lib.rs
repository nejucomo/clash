//! The proc-macro implementation of the `test-case-dir` crate
//!
//! Use `test-case-dir` rather than depending on this crate directly: the macro expansion refers to `::test_case_dir`, which re-exports `test-case` so that callers need not depend on it.

use std::path::{Path, PathBuf};

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::ext::IdentExt;
use syn::{Error, FnArg, Ident, ItemFn, LitStr, Pat, Result, Type, parse_macro_input};

/// Generate a `test-case` test for each Test Case Directory in the given Test Vector Directory
///
/// See the `test-case-dir` crate docs for details.
#[proc_macro_attribute]
pub fn test_case_dir(attr: TokenStream, item: TokenStream) -> TokenStream {
    let dir = parse_macro_input!(attr as LitStr);
    let item = parse_macro_input!(item as ItemFn);
    expand(&dir, item)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn expand(dir: &LitStr, item: ItemFn) -> Result<proc_macro2::TokenStream> {
    let params = params(&item)?;
    let names: Vec<Ident> = params.iter().map(|p| p.name.clone()).collect();
    let vector_dir = resolve_vector_dir(dir);
    let cases = read_case_dirs(dir, &vector_dir)?;

    // Only the first (outermost) attribute is resolved by the compiler, so it names `test_case` through the `test-case-dir` re-export, which means callers need not depend on `test-case`. That expansion consumes the remaining attributes itself, and recognizes them only by the bare `test_case` path.
    let attrs = cases
        .iter()
        .enumerate()
        .map(|(i, (name, path))| {
            let files = read_vector_files(dir, path, &names)?;
            let includes = params.iter().zip(&files).map(|(param, file)| {
                let include = param.kind.include_macro();
                quote! { #include!(#file) }
            });
            let test_case = if i == 0 {
                quote! { ::test_case_dir::__private::test_case::test_case }
            } else {
                quote! { test_case }
            };
            Ok(quote! {
                #[#test_case( #( #includes ),* ; #name )]
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        #( #attrs )*
        #item
    })
}

/// A test function parameter, which receives the contents of the Test Vector File of the same name
struct Param {
    name: Ident,
    kind: VectorKind,
}

/// How a Test Vector File is passed to its parameter, determined by the parameter type
enum VectorKind {
    /// `&[u8]`, via [`include_bytes!`]
    Bytes,
    /// `&str`, via [`include_str!`]
    Str,
}

impl VectorKind {
    /// Classify a parameter type, which must be `&[u8]` or `&str` (with any lifetime)
    fn of(ty: &Type) -> Result<Self> {
        let err = || {
            Error::new_spanned(
                ty,
                "test_case_dir parameters must have type `&[u8]` or `&str`",
            )
        };
        let Type::Reference(r) = strip_groups(ty) else {
            return Err(err());
        };
        if r.mutability.is_some() {
            return Err(err());
        }
        match strip_groups(&r.elem) {
            Type::Slice(s) if is_ident_type(&s.elem, "u8") => Ok(VectorKind::Bytes),
            elem if is_ident_type(elem, "str") => Ok(VectorKind::Str),
            _ => Err(err()),
        }
    }

    /// The macro which includes a Test Vector File as this kind
    fn include_macro(&self) -> proc_macro2::TokenStream {
        match self {
            VectorKind::Bytes => quote! { include_bytes },
            VectorKind::Str => quote! { include_str },
        }
    }
}

/// Look through invisible groups, which wrap types substituted by `macro_rules!`
fn strip_groups(ty: &Type) -> &Type {
    match ty {
        Type::Group(g) => strip_groups(&g.elem),
        ty => ty,
    }
}

/// Whether `ty` is the single-identifier type path `name`
fn is_ident_type(ty: &Type, name: &str) -> bool {
    matches!(strip_groups(ty), Type::Path(p) if p.qself.is_none() && p.path.is_ident(name))
}

/// The parameters of the test function, which must all be plain identifiers of a supported type
fn params(item: &ItemFn) -> Result<Vec<Param>> {
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
                Pat::Ident(pi) if pi.subpat.is_none() => Ok(Param {
                    name: pi.ident.unraw(),
                    kind: VectorKind::of(&pt.ty)?,
                }),
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

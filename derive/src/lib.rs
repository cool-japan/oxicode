//! Derive macros for OxiCode
//!
//! This crate provides derive macros for the `Encode` and `Decode` traits in the
//! [OxiCode](https://crates.io/crates/oxicode) binary serialization library.
//!
//! # Usage
//!
//! Add `oxicode` to your `Cargo.toml` with the `derive` feature enabled:
//!
//! ```toml
//! [dependencies]
//! oxicode = { version = "0.1", features = ["derive"] }
//! ```
//!
//! Then derive `Encode`, `Decode`, and `BorrowDecode` on your types:
//!
//! ```ignore
//! use oxicode::{Encode, Decode};
//!
//! #[derive(Encode, Decode)]
//! struct Point {
//!     x: f32,
//!     y: f32,
//! }
//!
//! #[derive(Encode, Decode)]
//! enum Message {
//!     Quit,
//!     Move { x: i32, y: i32 },
//!     Write(String),
//! }
//! ```
//!
//! # Field Attributes
//!
//! The derive macros support field-level attributes via `#[oxicode(...)]`:
//!
//! ## `#[oxicode(skip)]`
//!
//! Skip the field during encoding. During decoding, the field's value will be
//! set to `Default::default()`. This is useful for fields that should not be
//! serialized (e.g., computed caches, internal state, or transient fields).
//!
//! ```ignore
//! use oxicode::{Encode, Decode};
//!
//! #[derive(Encode, Decode)]
//! struct Config {
//!     id: u32,
//!     name: String,
//!     #[oxicode(skip)]
//!     cached_hash: u64,  // not encoded; restored as 0 on decode
//! }
//! ```
//!
//! ## `#[oxicode(default = "fn_path")]`
//!
//! Use a custom default function instead of `Decode::decode` when decoding.
//! The function is called with no arguments and must return the field type.
//! This is useful for providing a non-`Default` fallback value.
//!
//! ```ignore
//! use oxicode::{Encode, Decode};
//!
//! fn default_limit() -> u32 { 100 }
//!
//! #[derive(Encode, Decode)]
//! struct Settings {
//!     name: String,
//!     #[oxicode(default = "default_limit")]
//!     limit: u32,  // decoded normally, but if needed default_limit() is the fallback
//! }
//! ```
//!
//! Note: `default = "fn_path"` implies skipping the field during both encode and
//! decode (the field is excluded from the binary stream and the default function
//! is called to produce the value). To encode normally while specifying a default
//! factory, combine with struct-level versioning.
//!
//! ## `#[oxicode(flatten)]`
//!
//! Inline the fields of the nested struct directly into the encoding stream.
//! In OxiCode's binary format, structs are already encoded as a plain sequential
//! byte stream with no struct headers or length prefixes, so `flatten` is a
//! **semantic no-op** — the binary output is identical to normal encoding.
//!
//! This attribute exists primarily for compatibility with code migrating from
//! `serde` where `#[serde(flatten)]` is commonly used. The nested type must
//! implement `Encode` + `Decode`.
//!
//! ```ignore
//! use oxicode::{Encode, Decode};
//!
//! #[derive(Encode, Decode)]
//! struct Address { city: String, zip: u32 }
//!
//! #[derive(Encode, Decode)]
//! struct Person {
//!     name: String,
//!     #[oxicode(flatten)]
//!     address: Address,  // city and zip encoded in-sequence, no wrapper
//! }
//! ```
//!
//! # Container Attributes
//!
//! ## `#[oxicode(bound = "T: SomeTrait")]`
//!
//! Override the auto-generated where clause for all three trait impls (Encode, Decode,
//! BorrowDecode). When present, the auto-generated bounds are replaced by the predicates
//! in the string. An empty string `""` means no bounds are added.
//!
//! ## `#[oxicode(rename_all = "camelCase")]`
//!
//! Accepted without error for serde migration compatibility. In OxiCode's binary format
//! this is a no-op on the wire (fields are positional).
//!
//! ## `#[oxicode(crate = "my_oxicode")]`
//!
//! Override the path used to reference OxiCode items in generated code. Default is `::oxicode`.
//!
//! # Supported Types
//!
//! The derive macros support:
//!
//! - Structs with named fields
//! - Structs with unnamed fields (tuple structs)
//! - Unit structs
//! - Enums with any combination of named, unnamed, and unit variants
//! - Full generic type parameter support
//! - Lifetime parameter support
//! - Where clauses and bounds
//!
//! # Generics
//!
//! The derive macros automatically add appropriate trait bounds to generic type parameters:
//!
//! ```ignore
//! use oxicode::{Encode, Decode};
//!
//! #[derive(Encode, Decode)]
//! struct Container<T> {
//!     value: T,
//! }
//!
//! // This generates:
//! // impl<T: Encode> Encode for Container<T> { ... }
//! // impl<T: Decode> Decode for Container<T> { ... }
//! ```
//!
//! # Limitations
//!
//! - Unions are not supported due to safety concerns
//! - For complex scenarios requiring custom serialization logic, implement the traits manually

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod attrs;
mod decode_impl;
mod encode_impl;

use attrs::parse_container_attrs;
use decode_impl::{
    build_borrow_decode_generics, build_decode_generics, derive_borrow_decode_body,
    derive_decode_body,
};
use encode_impl::{build_encode_generics, derive_encode_body};

// ---------------------------------------------------------------------------
// Encode derive
// ---------------------------------------------------------------------------

/// Derive macro for the `Encode` trait
///
/// Supports structs and enums with full generic and lifetime support.
///
/// # Example
///
/// ```ignore
/// use oxicode::Encode;
///
/// #[derive(Encode)]
/// struct Point {
///     x: f32,
///     y: f32,
/// }
///
/// #[derive(Encode)]
/// enum Message {
///     Quit,
///     Move { x: i32, y: i32 },
///     Write(String),
/// }
///
/// // Skip a field during encoding
/// #[derive(Encode)]
/// struct Config {
///     id: u32,
///     #[oxicode(skip)]
///     cache: Vec<u8>,
/// }
/// ```
#[proc_macro_derive(Encode, attributes(oxicode))]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;

    let container_attrs = match parse_container_attrs(&input.attrs) {
        Ok(a) => a,
        Err(e) => return e.to_compile_error().into(),
    };
    let crate_path = &container_attrs.crate_path;

    let (_impl_generics, ty_generics, _where_clause) = generics.split_for_impl();

    let (impl_generics_tokens, effective_where) =
        build_encode_generics(generics, crate_path, &container_attrs.bound);

    let encode_body = match derive_encode_body(
        &input.data,
        crate_path,
        container_attrs.transparent,
        container_attrs.tag_type,
    ) {
        Ok(body) => body,
        Err(e) => return e.to_compile_error().into(),
    };

    let expanded = quote! {
        impl #impl_generics_tokens #crate_path::Encode for #name #ty_generics #effective_where {
            fn encode<__E: #crate_path::enc::Encoder>(&self, encoder: &mut __E) -> Result<(), #crate_path::Error> {
                #encode_body
            }
        }
    };

    TokenStream::from(expanded)
}

// ---------------------------------------------------------------------------
// Decode derive
// ---------------------------------------------------------------------------

/// Derive macro for the `Decode` trait
///
/// Supports structs and enums with full generic and lifetime support.
///
/// # Attributes
///
/// - `#[oxicode(skip)]` — don't decode this field; fill it with `Default::default()`
/// - `#[oxicode(default = "fn_path")]` — don't decode this field; call `fn_path()` to produce it
///
/// # Example
///
/// ```ignore
/// use oxicode::Decode;
///
/// fn default_score() -> u32 { 100 }
///
/// #[derive(Decode)]
/// struct Player {
///     name: String,
///     health: u32,
///     #[oxicode(skip)]
///     is_dirty: bool,            // always false after decode
///     #[oxicode(default = "default_score")]
///     score: u32,                // not in stream; set via default_score()
/// }
/// ```
#[proc_macro_derive(Decode, attributes(oxicode))]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;

    let container_attrs = match parse_container_attrs(&input.attrs) {
        Ok(a) => a,
        Err(e) => return e.to_compile_error().into(),
    };
    let crate_path = &container_attrs.crate_path;

    let (_impl_generics, ty_generics, _where_clause) = generics.split_for_impl();

    let (impl_generics_tokens, effective_where) =
        build_decode_generics(generics, crate_path, &container_attrs.bound);

    let decode_body = match derive_decode_body(
        &input.data,
        crate_path,
        container_attrs.transparent,
        container_attrs.tag_type,
    ) {
        Ok(body) => body,
        Err(e) => return e.to_compile_error().into(),
    };

    let expanded = quote! {
        impl #impl_generics_tokens #crate_path::Decode for #name #ty_generics #effective_where {
            fn decode<__D: #crate_path::de::Decoder<Context = ()>>(decoder: &mut __D) -> Result<Self, #crate_path::Error> {
                #decode_body
            }
        }
    };

    TokenStream::from(expanded)
}

// ---------------------------------------------------------------------------
// BorrowDecode derive
// ---------------------------------------------------------------------------

/// Derive macro for the `BorrowDecode` trait
///
/// Supports structs and enums with zero-copy decoding of borrowed types.
/// Fields that implement `BorrowDecode<'de>` (like `&'de str`, `&'de [u8]`)
/// will be decoded without copying. All other types delegate to `Decode`.
///
/// # Attributes
///
/// - `#[oxicode(skip)]` — don't borrow-decode this field; fill with `Default::default()`
/// - `#[oxicode(default = "fn_path")]` — don't decode; call `fn_path()` to produce it
///
/// # Example
///
/// ```ignore
/// use oxicode::{Encode, BorrowDecode};
///
/// #[derive(Encode, BorrowDecode)]
/// struct ZeroCopy<'a> {
///     data: &'a [u8],
///     name: &'a str,
///     #[oxicode(skip)]
///     cached: u64,
/// }
/// ```
#[proc_macro_derive(BorrowDecode, attributes(oxicode))]
pub fn derive_borrow_decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;

    let container_attrs = match parse_container_attrs(&input.attrs) {
        Ok(a) => a,
        Err(e) => return e.to_compile_error().into(),
    };
    let crate_path = &container_attrs.crate_path;

    let (_impl_generics, ty_generics, _where_clause) = generics.split_for_impl();

    let (impl_generics_with_bounds, de_lifetime, effective_where) =
        build_borrow_decode_generics(generics, crate_path, &container_attrs.bound);

    let decode_body = match derive_borrow_decode_body(
        &input.data,
        &de_lifetime,
        crate_path,
        container_attrs.transparent,
        container_attrs.tag_type,
    ) {
        Ok(body) => body,
        Err(e) => return e.to_compile_error().into(),
    };

    let expanded = quote! {
        impl #impl_generics_with_bounds #crate_path::de::BorrowDecode<#de_lifetime> for #name #ty_generics #effective_where {
            fn borrow_decode<__D: #crate_path::de::BorrowDecoder<#de_lifetime, Context = ()>>(
                decoder: &mut __D,
            ) -> Result<Self, #crate_path::Error> {
                #decode_body
            }
        }
    };

    TokenStream::from(expanded)
}

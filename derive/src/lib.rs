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
//! oxicode = { version = "0.2", features = ["derive"] }
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
//! ### Important divergence from `serde`
//!
//! **`#[oxicode(default = "fn_path")]` skips the field during *encode* as well as decode.**
//! The field is excluded from the binary stream entirely and `fn_path()` is called to
//! reconstruct it on decode. This is the opposite of `#[serde(default = ...)]`, which only
//! affects deserialization and still *serializes* the field. A type migrated from
//! serde/bincode that annotates a still-transmitted field with `default` will silently drop
//! that field from the wire, breaking round-trips with any peer that expects it. If you want
//! the field encoded normally with only a decode-side fallback, do not use this attribute —
//! implement the fallback in your own decode logic or a struct-level versioning scheme.
//!
//! ## `#[oxicode(bytes)]`
//!
//! Encode/decode the field as a length-prefixed raw byte run using a bulk read/write path
//! (a `u64` length followed by the bytes). Works on any field whose type can be viewed as
//! `&[u8]` for encoding and reconstructed from bytes for decoding — `Vec<u8>`, `Box<[u8]>`,
//! `Arc<[u8]>`/`Rc<[u8]>`, `[u8; N]`, `bytes::Bytes`, and similar containers (the decode side
//! materializes via `TryFrom<Vec<u8>>`, or `TryFrom<&'de [u8]>` for zero-copy `BorrowDecode`).
//! Cannot be combined with `skip`, `default`, `seq_len`, `with`, `encode_with`, or
//! `decode_with` — doing so is a compile error.
//!
//! ## `#[oxicode(seq_len = "u8" | "u16" | "u32" | "u64")]`
//!
//! Use a fixed-width length prefix of the given width for a `Vec<T>` field instead of the
//! default `u64` length. **Wire-incompatible with bincode** (which always uses its varint/fixint
//! length encoding) — only interoperates with other oxicode peers using the same `seq_len`.
//! The decode path claims the container against the configured decode memory limit before
//! allocating, so a malicious length cannot drive an unbounded pre-allocation.
//!
//! ## `#[oxicode(with = "module_path")]`
//!
//! Use `module_path::encode(&field, encoder)` and `module_path::decode(decoder)` for this field
//! instead of its own `Encode`/`Decode` impls. Useful for third-party types or custom framing.
//!
//! ## `#[oxicode(encode_with = "path::to::fn")]` / `#[oxicode(decode_with = "path::to::fn")]`
//!
//! Like `with`, but specify the encode and/or decode function independently.
//! `encode_with` signature: `fn<E: Encoder>(&T, &mut E) -> Result<(), Error>`;
//! `decode_with` signature: `fn<D: Decoder>(&mut D) -> Result<T, Error>`.
//!
//! ## `#[oxicode(default_value = "expr")]`
//!
//! An inline expression used as the decode-side value when the field is skipped (via `skip` or
//! `default`). Takes precedence over `Default::default()` and over the `default` function path.
//! Only affects decode, never encode.
//!
//! ## `#[oxicode(rename = "name")]`
//!
//! Accepted for serde-migration source compatibility. Because oxicode's binary format is
//! positional (fields carry no names on the wire), this is a **no-op on the wire**. It is
//! retained so serde-annotated structs compile unchanged; it has no runtime effect.
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
//! ## `#[oxicode(transparent)]`
//!
//! On a struct with exactly one field, encode/decode as that inner field directly with no
//! additional framing. A compile error is emitted for structs with a different field count and
//! for enums/unions.
//!
//! ## `#[oxicode(tag_type = "u8" | "u16" | "u32" | "u64")]`
//!
//! Width of the enum discriminant tag written before each variant's payload. Defaults to `u32`
//! (bincode-compatible). Narrower widths save space for small enums; `u64` allows explicit
//! discriminants above `u32::MAX`. **Non-default widths are wire-incompatible with bincode.**
//! A supplied `#[oxicode(variant = N)]` value that does not fit the configured width is a
//! compile error. Only meaningful on enums; ignored on structs.
//!
//! # Variant Attributes
//!
//! Applied to individual enum variants via `#[oxicode(...)]`:
//!
//! ## `#[oxicode(variant = N)]`
//!
//! Assign the explicit discriminant `N` to the variant instead of its declaration-order index.
//! `N` may be up to `u64::MAX` when `tag_type = "u64"` is set on the enum (otherwise it must fit
//! the configured tag width). Note that native Rust explicit discriminants (`enum E { A = 5 }`)
//! are **ignored** by the derive — use this attribute to control the wire discriminant.
//!
//! ## `#[oxicode(rename = "name")]`
//!
//! Accepted for serde-migration source compatibility; a **no-op on the wire** (variants are
//! positional in the binary format).
//!
//! ## `#[oxicode(skip)]` (variant-level)
//!
//! Exclude the variant from the discriminant space. On **encode** the skipped variant is
//! assigned the same discriminant as the next non-skipped variant in declaration order (so it
//! aliases that successor on the wire). On **decode** no arm is generated for it, so that
//! discriminant decodes into the successor. A skipped variant with **no** following non-skipped
//! variant has nothing to alias onto and is a compile error (it could never round-trip).
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
    derive_decode_body, generic_context_ident,
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
            fn encode<__E: #crate_path::enc::Encoder>(&self, encoder: &mut __E) -> ::core::result::Result<(), #crate_path::Error> {
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
/// - `#[oxicode(decode_context = "Ctx")]` — implement `Decode<Ctx>` instead of
///   `Decode<()>`, so the type can be decoded with `oxicode`'s
///   `decode_from_slice_with_context` and friends, and can hold fields whose
///   own impls need that context. `Ctx` may be a concrete type or a generic
///   parameter the container already declares.
/// - `#[oxicode(context_generic)]` — implement `Decode<C>` for *every* `C`, so
///   the type composes into any context. Prefer this when the type itself does
///   not care about the context.
/// - `#[oxicode(context = "Ctx")]` sets both `decode_context` and
///   `borrow_decode_context`; `#[oxicode(context_generic)]` likewise sets both
///   generic forms.
///
/// Without any of these the generated impl is `Decode<()>`, exactly as before —
/// the default path is unchanged in both the API and the wire format.
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

    let decode_context = container_attrs.decode_context.clone();
    let context_generic = container_attrs.decode_context_generic;
    let (impl_generics_tokens, effective_where) = build_decode_generics(
        generics,
        crate_path,
        &container_attrs.bound,
        decode_context.as_ref(),
        context_generic,
    );
    // `Context = ()` unless the container opted into a context, so the default
    // derive output — and therefore the wire format and the public API — is
    // unchanged.
    let decode_context_ty: syn::Type = if context_generic {
        let ctx = generic_context_ident();
        syn::parse_quote!(#ctx)
    } else {
        decode_context.unwrap_or_else(|| syn::parse_quote!(()))
    };

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
        impl #impl_generics_tokens #crate_path::Decode<#decode_context_ty> for #name #ty_generics #effective_where {
            fn decode<__D: #crate_path::de::Decoder<Context = #decode_context_ty>>(decoder: &mut __D) -> ::core::result::Result<Self, #crate_path::Error> {
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
/// - `#[oxicode(borrow_decode_context = "Ctx")]` — implement
///   `BorrowDecode<'de, Ctx>` instead of `BorrowDecode<'de, ()>`
/// - `#[oxicode(borrow_decode_context_generic)]` — implement
///   `BorrowDecode<'de, C>` for every `C`
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

    let borrow_context = container_attrs.borrow_decode_context.clone();
    let context_generic = container_attrs.borrow_decode_context_generic;
    let (impl_generics_with_bounds, de_lifetime, effective_where) = build_borrow_decode_generics(
        generics,
        crate_path,
        &container_attrs.bound,
        borrow_context.as_ref(),
        context_generic,
    );
    let borrow_context_ty: syn::Type = if context_generic {
        let ctx = generic_context_ident();
        syn::parse_quote!(#ctx)
    } else {
        borrow_context.unwrap_or_else(|| syn::parse_quote!(()))
    };

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
        impl #impl_generics_with_bounds #crate_path::de::BorrowDecode<#de_lifetime, #borrow_context_ty> for #name #ty_generics #effective_where {
            fn borrow_decode<__D: #crate_path::de::BorrowDecoder<#de_lifetime, Context = #borrow_context_ty>>(
                decoder: &mut __D,
            ) -> ::core::result::Result<Self, #crate_path::Error> {
                #decode_body
            }
        }
    };

    TokenStream::from(expanded)
}

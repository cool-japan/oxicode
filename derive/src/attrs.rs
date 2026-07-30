//! Attribute parsing for OxiCode derive macros.
//!
//! Contains container-level, field-level, and variant-level attribute structs and
//! their parsers.

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::Parse, parse::Parser, punctuated::Punctuated, spanned::Spanned, LitInt, LitStr, Token,
};

// ---------------------------------------------------------------------------
// TagType — enum discriminant width control
// ---------------------------------------------------------------------------

/// Width of the discriminant tag used when encoding/decoding enum variants.
///
/// Set via `#[oxicode(tag_type = "u8")]` on the container. Defaults to `U32`
/// for bincode compatibility.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TagType {
    /// 1-byte discriminant — saves space for enums with ≤ 256 variants.
    U8,
    /// 2-byte discriminant.
    U16,
    /// 4-byte discriminant (default, bincode-compatible).
    U32,
    /// 8-byte discriminant — maximum range.
    U64,
}

// ---------------------------------------------------------------------------
// Container attribute parsing
// ---------------------------------------------------------------------------

/// Parsed container-level `#[oxicode(...)]` attributes (on the struct/enum itself).
pub(crate) struct ContainerAttrs {
    /// `#[oxicode(bound = "T: Trait, ...")]` — replaces auto-generated where clause.
    /// `Some(vec![])` means no where predicates at all.
    /// `None` means use auto-generated bounds.
    pub(crate) bound: Option<Vec<syn::WherePredicate>>,
    /// `#[oxicode(rename_all = "camelCase")]` — naming convention (no-op on wire).
    /// Stored for diagnostic/display layers; not used in code generation.
    #[allow(dead_code)]
    pub(crate) rename_all: Option<String>,
    /// `#[oxicode(crate = "path")]` — path to the oxicode crate in generated code.
    pub(crate) crate_path: syn::Path,
    /// `#[oxicode(transparent)]` — encode/decode as the single inner field directly.
    /// The struct must have exactly one field. Emits a compile error otherwise.
    pub(crate) transparent: bool,
    /// `#[oxicode(tag_type = "u8|u16|u32|u64")]` — enum discriminant width.
    /// Only meaningful on enums; silently ignored on structs. Defaults to `U32`.
    pub(crate) tag_type: TagType,
}

/// Valid `rename_all` convention names.
const VALID_RENAME_ALL: &[&str] = &[
    "lowercase",
    "UPPERCASE",
    "camelCase",
    "PascalCase",
    "snake_case",
    "SCREAMING_SNAKE_CASE",
    "kebab-case",
];

/// Parse the `#[oxicode(...)]` attributes on a container (struct or enum item).
pub(crate) fn parse_container_attrs(attrs: &[syn::Attribute]) -> syn::Result<ContainerAttrs> {
    let default_crate_path =
        syn::parse_str::<syn::Path>("::oxicode").unwrap_or_else(|_| syn::parse_quote!(::oxicode));

    let mut bound: Option<Vec<syn::WherePredicate>> = None;
    let mut rename_all: Option<String> = None;
    let mut crate_path: syn::Path = default_crate_path;
    let mut transparent: bool = false;
    let mut tag_type: TagType = TagType::U32;

    for attr in attrs {
        if !attr.path().is_ident("oxicode") {
            continue;
        }
        attr.parse_args_with(|input: syn::parse::ParseStream<'_>| -> syn::Result<()> {
            loop {
                if input.is_empty() {
                    break;
                }
                // `crate` is a keyword, so we must peek for it before trying to parse an Ident.
                if input.peek(Token![crate]) {
                    let kw_span = input.parse::<Token![crate]>()?.span;
                    let _eq: Token![=] = input.parse()?;
                    let lit: LitStr = input.parse()?;
                    let path_str = lit.value();
                    crate_path = syn::parse_str::<syn::Path>(&path_str).map_err(|_| {
                        syn::Error::new(kw_span, format!("invalid crate path `{}`", path_str))
                    })?;
                } else {
                    let ident: syn::Ident = input.parse()?;
                    match ident.to_string().as_str() {
                        "bound" => {
                            let _eq: Token![=] = input.parse()?;
                            let lit: LitStr = input.parse()?;
                            let raw = lit.value();
                            let predicates = parse_bound_string(&raw, lit.span())?;
                            bound = Some(predicates);
                        }
                        "rename_all" => {
                            let _eq: Token![=] = input.parse()?;
                            let lit: LitStr = input.parse()?;
                            let convention = lit.value();
                            if !VALID_RENAME_ALL.contains(&convention.as_str()) {
                                return Err(syn::Error::new(
                                    lit.span(),
                                    format!(
                                        "unknown rename_all convention `{}`. Valid values: {}",
                                        convention,
                                        VALID_RENAME_ALL.join(", ")
                                    ),
                                ));
                            }
                            rename_all = Some(convention);
                        }
                        "transparent" => {
                            transparent = true;
                        }
                        "tag_type" => {
                            let _eq: Token![=] = input.parse()?;
                            let lit: LitStr = input.parse()?;
                            tag_type = match lit.value().as_str() {
                                "u8" => TagType::U8,
                                "u16" => TagType::U16,
                                "u32" => TagType::U32,
                                "u64" => TagType::U64,
                                other => {
                                    return Err(syn::Error::new(
                                        lit.span(),
                                        format!(
                                            "unsupported tag_type `{}`: expected u8, u16, u32, or u64",
                                            other
                                        ),
                                    ));
                                }
                            };
                        }
                        other => {
                            return Err(syn::Error::new(
                                ident.span(),
                                format!("unknown oxicode container attribute `{}`", other),
                            ));
                        }
                    }
                }
                if input.peek(Token![,]) {
                    let _: Token![,] = input.parse()?;
                }
            }
            Ok(())
        })?;
    }

    Ok(ContainerAttrs {
        bound,
        rename_all,
        crate_path,
        transparent,
        tag_type,
    })
}

/// Parse a `bound = "..."` string into a list of `WherePredicate`s.
/// The string may be empty (returns `Ok(vec![])`) or contain comma-separated predicates.
pub(crate) fn parse_bound_string(
    raw: &str,
    span: proc_macro2::Span,
) -> syn::Result<Vec<syn::WherePredicate>> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(vec![]);
    }
    // Parse the whole string as a comma-terminated list of `where` predicates using syn's
    // punctuated parser. Unlike a hand-rolled comma splitter, this correctly tracks nesting
    // of `<>`, `()`, and `[]`, so Fn-trait predicates such as `T: Fn(u8, u8) -> ()` — whose
    // argument list contains top-level-looking commas inside parentheses — are handled
    // correctly instead of being split into invalid fragments.
    let parser = Punctuated::<syn::WherePredicate, Token![,]>::parse_terminated;
    let parsed = parser.parse_str(trimmed).map_err(|e| {
        syn::Error::new(
            span,
            format!("invalid where predicates `{}`: {}", trimmed, e),
        )
    })?;
    Ok(parsed.into_iter().collect())
}

/// Apply a `rename_all` naming convention to a field name.
/// This is currently a no-op on the wire (binary format is positional),
/// but is provided as a utility for diagnostic/display layers.
#[allow(dead_code)]
pub(crate) fn apply_rename_all(name: &str, convention: &str) -> String {
    match convention {
        "lowercase" => name.to_lowercase(),
        "UPPERCASE" => name.to_uppercase(),
        "camelCase" => {
            let mut words = name.split('_');
            let first = words.next().unwrap_or("").to_lowercase();
            let rest: String = words
                .map(|w| {
                    let mut chars = w.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                })
                .collect();
            first + &rest
        }
        "PascalCase" => name
            .split('_')
            .map(|w| {
                let mut chars = w.chars();
                match chars.next() {
                    None => String::new(),
                    Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect(),
        "snake_case" => name.to_owned(),
        "SCREAMING_SNAKE_CASE" => name.to_uppercase(),
        "kebab-case" => name.replace('_', "-"),
        _ => name.to_owned(),
    }
}

/// Produce the enum-discriminant literal token at the integer width selected by `tag_type`,
/// validating that `disc` actually fits that width.
///
/// Both the encode side (which writes the tag) and the decode side (which matches on it) call
/// this so the literal width always agrees with the decoded tag width. Returning an error when
/// a supplied `#[oxicode(variant = N)]` value exceeds the configured `tag_type` avoids silently
/// truncating the discriminant on the wire.
pub(crate) fn discriminant_literal(
    disc: u64,
    tag_type: TagType,
    span: proc_macro2::Span,
) -> syn::Result<proc_macro2::Literal> {
    match tag_type {
        TagType::U8 => {
            if disc > u8::MAX as u64 {
                return Err(syn::Error::new(
                    span,
                    format!(
                        "enum discriminant {} does not fit in tag_type `u8` (max {})",
                        disc,
                        u8::MAX
                    ),
                ));
            }
            Ok(proc_macro2::Literal::u8_suffixed(disc as u8))
        }
        TagType::U16 => {
            if disc > u16::MAX as u64 {
                return Err(syn::Error::new(
                    span,
                    format!(
                        "enum discriminant {} does not fit in tag_type `u16` (max {})",
                        disc,
                        u16::MAX
                    ),
                ));
            }
            Ok(proc_macro2::Literal::u16_suffixed(disc as u16))
        }
        TagType::U32 => {
            if disc > u32::MAX as u64 {
                return Err(syn::Error::new(
                    span,
                    format!(
                        "enum discriminant {} does not fit in tag_type `u32` (max {}); use #[oxicode(tag_type = \"u64\")]",
                        disc,
                        u32::MAX
                    ),
                ));
            }
            Ok(proc_macro2::Literal::u32_suffixed(disc as u32))
        }
        TagType::U64 => Ok(proc_macro2::Literal::u64_suffixed(disc)),
    }
}

/// Extract the element type `T` from a field whose declared type is `Vec<T>`
/// (matching either `Vec<T>` or a path ending in `Vec<T>` such as `std::vec::Vec<T>`).
///
/// Used by the `seq_len` decode path so it can emit `claim_container_read::<T>` and honor the
/// configured decode memory limit before reserving capacity.
pub(crate) fn vec_element_type(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(type_path) = ty {
        if type_path.qself.is_none() {
            if let Some(segment) = type_path.path.segments.last() {
                if segment.ident == "Vec" {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        for arg in &args.args {
                            if let syn::GenericArgument::Type(inner) = arg {
                                return Some(inner);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// Build a `where` clause `TokenStream2` from a list of `WherePredicate`s.
/// Returns an empty token stream when the list is empty (no where clause).
pub(crate) fn predicates_to_where_clause(predicates: &[syn::WherePredicate]) -> TokenStream2 {
    if predicates.is_empty() {
        quote! {}
    } else {
        quote! { where #(#predicates),* }
    }
}

// ---------------------------------------------------------------------------
// Field attribute parsing helpers
// ---------------------------------------------------------------------------

/// Parsed field-level `#[oxicode(...)]` attributes.
#[derive(Default)]
pub(crate) struct FieldAttrs {
    /// `#[oxicode(skip)]` — skip during encode; use Default on decode.
    pub(crate) skip: bool,
    /// `#[oxicode(default = "fn_path")]` — skip during encode; call fn_path() on decode.
    pub(crate) default_fn: Option<syn::Path>,
    /// `#[oxicode(flatten)]` — accepted as a no-op; the nested struct's fields are already
    /// encoded sequentially without any struct header, so flatten == default.
    pub(crate) flatten: bool,
    /// `#[oxicode(bytes)]` — encode/decode as raw bytes using a bulk write path.
    pub(crate) bytes: bool,
    /// `#[oxicode(with = "module_path")]` — use custom encode/decode module functions.
    pub(crate) with_module: Option<syn::Path>,
    /// `#[oxicode(encode_with = "path::to::fn")]` — custom encode function only.
    /// Signature: `fn<E: Encoder>(&T, &mut E) -> Result<(), Error>`
    pub(crate) encode_with: Option<syn::Path>,
    /// `#[oxicode(decode_with = "path::to::fn")]` — custom decode function only.
    /// Signature: `fn<D: Decoder>(&mut D) -> Result<T, Error>`
    pub(crate) decode_with: Option<syn::Path>,
    /// `#[oxicode(rename = "name")]` — no-op on the binary wire format (fields are positional).
    /// Stored for documentation, future text/JSON layers, and serde migration compatibility.
    #[allow(dead_code)]
    pub(crate) rename: Option<String>,
    /// `#[oxicode(seq_len = "u8"/"u16"/"u32"/"u64")]` — compact fixed-width length prefix for
    /// Vec/sequence fields instead of the default varint encoding.
    pub(crate) seq_len: Option<String>,
    /// `#[oxicode(default_value = "expr")]` — inline expression used as default on decode when
    /// the field is skipped (via `skip` or `default`). Takes precedence over `Default::default()`
    /// and over the `default_fn` path. Only applies to Decode, not Encode.
    pub(crate) default_expr: Option<syn::Expr>,
}

impl FieldAttrs {
    /// Return `true` when the field should be excluded from the binary stream.
    pub(crate) fn is_skipped(&self) -> bool {
        self.skip || self.default_fn.is_some()
    }
}

/// Parse the `#[oxicode(...)]` helper attributes on a single field.
pub(crate) fn parse_field_attrs(field: &syn::Field) -> Result<FieldAttrs, syn::Error> {
    let mut attrs = FieldAttrs::default();

    for attr in &field.attrs {
        if !attr.path().is_ident("oxicode") {
            continue;
        }
        attr.parse_args_with(|input: syn::parse::ParseStream<'_>| -> syn::Result<()> {
            loop {
                if input.is_empty() {
                    break;
                }
                let ident: syn::Ident = input.parse()?;
                match ident.to_string().as_str() {
                    "skip" => {
                        attrs.skip = true;
                    }
                    "default" => {
                        let _eq: Token![=] = input.parse()?;
                        let lit: LitStr = input.parse()?;
                        let path: syn::Path = lit.parse()?;
                        attrs.default_fn = Some(path);
                    }
                    "flatten" => {
                        attrs.flatten = true;
                    }
                    "bytes" => {
                        attrs.bytes = true;
                    }
                    "with" => {
                        let _eq: Token![=] = input.parse()?;
                        let lit: LitStr = input.parse()?;
                        let path: syn::Path = lit.parse()?;
                        attrs.with_module = Some(path);
                    }
                    "encode_with" => {
                        let _eq: Token![=] = input.parse()?;
                        let lit: LitStr = input.parse()?;
                        let path: syn::Path = lit.parse()?;
                        attrs.encode_with = Some(path);
                    }
                    "decode_with" => {
                        let _eq: Token![=] = input.parse()?;
                        let lit: LitStr = input.parse()?;
                        let path: syn::Path = lit.parse()?;
                        attrs.decode_with = Some(path);
                    }
                    "rename" => {
                        let _eq: Token![=] = input.parse()?;
                        let lit: LitStr = input.parse()?;
                        attrs.rename = Some(lit.value());
                    }
                    "seq_len" => {
                        let _eq: Token![=] = input.parse()?;
                        let lit: LitStr = input.parse()?;
                        let val = lit.value();
                        match val.as_str() {
                            "u8" | "u16" | "u32" | "u64" => {
                                attrs.seq_len = Some(val);
                            }
                            other => {
                                return Err(syn::Error::new(
                                    lit.span(),
                                    format!(
                                        "unsupported seq_len value `{}`: expected one of u8, u16, u32, u64",
                                        other
                                    ),
                                ));
                            }
                        }
                    }
                    "default_value" => {
                        let _eq: Token![=] = input.parse()?;
                        let lit: LitStr = input.parse()?;
                        attrs.default_expr = Some(lit.parse_with(syn::Expr::parse)?);
                    }
                    other => {
                        return Err(syn::Error::new(
                            ident.span(),
                            format!("unknown oxicode field attribute `{}`", other),
                        ));
                    }
                }
                if input.peek(Token![,]) {
                    let _: Token![,] = input.parse()?;
                }
            }
            Ok(())
        })?;
    }

    // Reject mutually-exclusive attribute combinations up front so that code generation never
    // has to resolve them by silent precedence. In particular `#[oxicode(skip, bytes)]` on a
    // named enum-variant field previously produced code that referenced an unbound identifier
    // (the field is pattern-bound as `_field` when skipped, but the `bytes` branch emitted the
    // raw `field` ident), so surface the conflict as a clear compile error instead.
    let with_like =
        attrs.with_module.is_some() || attrs.encode_with.is_some() || attrs.decode_with.is_some();
    let excluded = attrs.skip || attrs.default_fn.is_some() || attrs.seq_len.is_some() || with_like;
    if attrs.bytes && excluded {
        return Err(syn::Error::new(
            field.span(),
            "#[oxicode(bytes)] cannot be combined with skip, default, seq_len, with, encode_with, or decode_with",
        ));
    }
    if (attrs.skip || attrs.default_fn.is_some()) && with_like {
        return Err(syn::Error::new(
            field.span(),
            "#[oxicode(skip)]/#[oxicode(default = ...)] cannot be combined with with, encode_with, or decode_with",
        ));
    }

    Ok(attrs)
}

/// Parsed variant-level `#[oxicode(...)]` attributes.
#[derive(Default)]
pub(crate) struct VariantAttrs {
    /// `#[oxicode(variant = N)]` — use N as the discriminant tag instead of position index.
    ///
    /// Stored as `u64` so that discriminants above `u32::MAX` can be assigned when the
    /// container selects `#[oxicode(tag_type = "u64")]`. The value is validated against the
    /// configured tag width at code-generation time.
    pub(crate) tag: Option<u64>,
    /// `#[oxicode(rename = "name")]` — no-op on the binary wire format (variants are positional).
    /// Stored for documentation, future text/JSON layers, and serde migration compatibility.
    #[allow(dead_code)]
    pub(crate) rename: Option<String>,
    /// `#[oxicode(skip)]` — exclude this variant from the discriminant space.
    ///
    /// On **encode**: a skipped variant is assigned the same discriminant as the next
    /// non-skipped variant that follows it in declaration order.  Encoding a skipped
    /// variant is therefore identical to encoding that successor variant.
    ///
    /// On **decode**: no match arm is generated for a skipped variant, so any byte
    /// stream that reaches a discriminant occupied by the successor variant decodes
    /// into that successor, not into the skipped one.
    pub(crate) skip: bool,
}

/// Parse the `#[oxicode(...)]` helper attributes on a single enum variant.
pub(crate) fn parse_variant_attrs(attrs: &[syn::Attribute]) -> Result<VariantAttrs, syn::Error> {
    let mut variant_attrs = VariantAttrs::default();

    for attr in attrs {
        if !attr.path().is_ident("oxicode") {
            continue;
        }
        attr.parse_args_with(|input: syn::parse::ParseStream<'_>| -> syn::Result<()> {
            loop {
                if input.is_empty() {
                    break;
                }
                let ident: syn::Ident = input.parse()?;
                match ident.to_string().as_str() {
                    "variant" => {
                        let _eq: Token![=] = input.parse()?;
                        let lit: LitInt = input.parse()?;
                        let value: u64 = lit.base10_parse().map_err(|_| {
                            syn::Error::new(
                                lit.span(),
                                "oxicode `variant` value must be a valid u64 literal",
                            )
                        })?;
                        variant_attrs.tag = Some(value);
                    }
                    "rename" => {
                        let _eq: Token![=] = input.parse()?;
                        let lit: LitStr = input.parse()?;
                        variant_attrs.rename = Some(lit.value());
                    }
                    "skip" => {
                        variant_attrs.skip = true;
                    }
                    other => {
                        return Err(syn::Error::new(
                            ident.span(),
                            format!("unknown oxicode variant attribute `{}`", other),
                        ));
                    }
                }
                if input.peek(Token![,]) {
                    let _: Token![,] = input.parse()?;
                }
            }
            Ok(())
        })?;
    }

    Ok(variant_attrs)
}

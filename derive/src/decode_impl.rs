//! Decode and BorrowDecode derive implementations for OxiCode.
//!
//! Contains code-generation helpers for `Decode` and `BorrowDecode` trait derive macros.

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, Fields, LifetimeParam};

use crate::attrs::{
    discriminant_literal, parse_field_attrs, parse_variant_attrs, predicates_to_where_clause,
    vec_element_type, TagType,
};

// ---------------------------------------------------------------------------
// Shared seq_len decode helper
// ---------------------------------------------------------------------------

/// Generate a seq_len decode block for a Vec/sequence field.
///
/// `len_ty_str` is one of "u8", "u16", "u32", "u64".
/// `elem_ty` is the sequence element type (extracted from a `Vec<T>` field) when known, so the
/// decoder can claim the container against the configured memory limit before allocating.
///
/// The decoded length is converted with `usize::try_from` (never a lossy `as usize`) so that a
/// stream carrying a length above `usize::MAX` on a narrow target is rejected rather than
/// silently truncated. Before touching the allocator the block claims the read against the
/// decoder's limit — either `claim_container_read::<T>` when the element type is known or the
/// byte-based `claim_bytes_read` otherwise — and it bounds the initial reservation so an
/// attacker-supplied length cannot drive an unbounded pre-allocation.
pub(crate) fn make_seq_len_decode_expr(
    len_ty_str: &str,
    crate_path: &syn::Path,
    elem_ty: Option<&syn::Type>,
) -> TokenStream2 {
    let len_ty_tokens: TokenStream2 = match len_ty_str {
        "u8" => quote! { u8 },
        "u16" => quote! { u16 },
        "u32" => quote! { u32 },
        _ => quote! { u64 },
    };
    let claim = match elem_ty {
        Some(ty) => quote! { decoder.claim_container_read::<#ty>(__seq_len)?; },
        None => quote! { decoder.claim_bytes_read(__seq_len)?; },
    };
    quote! {
        {
            let __seq_len = ::core::convert::TryInto::<usize>::try_into(
                <#len_ty_tokens as #crate_path::Decode<_>>::decode(decoder)?
            ).map_err(|_| #crate_path::Error::InvalidData {
                message: "seq_len exceeds usize on this platform"
            })?;
            #claim
            let mut __vec = #crate_path::__private::Vec::with_capacity(::core::cmp::min(__seq_len, 4096));
            for _ in 0..__seq_len {
                __vec.push(<_ as #crate_path::Decode<_>>::decode(decoder)?);
            }
            __vec
        }
    }
}

/// Generate an owned `#[oxicode(bytes)]` decode block.
///
/// Reads a length-prefixed byte run and materializes it into the field's declared type via
/// `TryFrom<Vec<u8>>` rather than hard-coding `Vec<u8>`, so `#[oxicode(bytes)]` also works on
/// `Box<[u8]>`, `Arc<[u8]>`, `[u8; N]`, `bytes::Bytes`, and any other byte container that
/// implements the conversion. The length is converted with a checked `try_into` and the read is
/// claimed against the decode memory limit before any allocation.
pub(crate) fn make_bytes_decode_expr(field_ty: &syn::Type, crate_path: &syn::Path) -> TokenStream2 {
    quote! {
        {
            let __len = ::core::convert::TryInto::<usize>::try_into(
                <u64 as #crate_path::de::Decode<_>>::decode(decoder)?
            ).map_err(|_| #crate_path::Error::InvalidData {
                message: "byte length exceeds usize on this platform"
            })?;
            decoder.claim_bytes_read(__len)?;
            // Never reserve the full attacker-controlled length up front. When
            // the reader knows how much input is left the length is rejected
            // before any allocation; otherwise the buffer is materialized in
            // bounded steps, each of which must actually be filled from the
            // reader before the next is reserved, so a forged length prefix
            // fails on the first short read rather than committing a
            // multi-gigabyte allocation from a handful of bytes.
            let __buf: #crate_path::__private::Vec<u8> =
                #crate_path::__private::read_bytes_bounded(decoder, __len)?;
            <#field_ty as ::core::convert::TryFrom<#crate_path::__private::Vec<u8>>>::try_from(__buf)
                .map_err(|_| #crate_path::Error::InvalidData {
                    message: "byte field does not fit its declared type"
                })?
        }
    }
}

/// Generate a borrowing `#[oxicode(bytes)]` decode block for the `BorrowDecode` derive.
///
/// Borrows the byte run directly from the input via `take_bytes` and converts it into the field
/// type with `TryFrom<&'de [u8]>`, giving true zero-copy for `&'de [u8]` fields while still
/// supporting owned containers (`Vec<u8>`, `Box<[u8]>`, `[u8; N]`, ...). Previously the derive
/// had no `bytes` branch, so the attribute was silently dropped under `BorrowDecode`.
pub(crate) fn make_bytes_borrow_decode_expr(
    field_ty: &syn::Type,
    de_lifetime: &syn::Lifetime,
    crate_path: &syn::Path,
) -> TokenStream2 {
    quote! {
        {
            let __len = ::core::convert::TryInto::<usize>::try_into(
                <u64 as #crate_path::de::Decode<_>>::decode(decoder)?
            ).map_err(|_| #crate_path::Error::InvalidData {
                message: "byte length exceeds usize on this platform"
            })?;
            decoder.claim_bytes_read(__len)?;
            let __bytes: & #de_lifetime [u8] =
                #crate_path::de::BorrowReader::take_bytes(decoder.borrow_reader(), __len)?;
            <#field_ty as ::core::convert::TryFrom<& #de_lifetime [u8]>>::try_from(__bytes)
                .map_err(|_| #crate_path::Error::InvalidData {
                    message: "byte field does not fit its declared type"
                })?
        }
    }
}

// ---------------------------------------------------------------------------
// Decode helpers
// ---------------------------------------------------------------------------

/// Generate the body of the `Decode::decode` method.
pub(crate) fn derive_decode_body(
    data: &Data,
    crate_path: &syn::Path,
    transparent: bool,
    tag_type: TagType,
) -> Result<TokenStream2, syn::Error> {
    if transparent {
        return match data {
            Data::Struct(data_struct) => derive_decode_transparent(&data_struct.fields, crate_path),
            Data::Enum(data_enum) => Err(syn::Error::new(
                data_enum.enum_token.span(),
                "#[oxicode(transparent)] is not supported on enums",
            )),
            Data::Union(data_union) => Err(syn::Error::new(
                data_union.union_token.span(),
                "#[oxicode(transparent)] is not supported on unions",
            )),
        };
    }
    match data {
        Data::Struct(data_struct) => derive_decode_struct(&data_struct.fields, crate_path),
        Data::Enum(data_enum) => {
            // Skip variants marked `#[oxicode(skip)]` — they have no decode arm; their
            // discriminant (shared with the next non-skipped successor) is decoded as
            // that successor instead.
            let variant_decodings: Vec<TokenStream2> = data_enum
                .variants
                .iter()
                .enumerate()
                .filter_map(|(idx, variant)| match parse_variant_attrs(&variant.attrs) {
                    Ok(attrs) if attrs.skip => None,
                    Ok(_) => Some(derive_decode_variant(idx, variant, crate_path, tag_type)),
                    Err(e) => Some(Err(e)),
                })
                .collect::<Result<_, _>>()?;

            // Decode the discriminant tag at its native width. The match arms use literals of the
            // same width (see `discriminant_literal`), so no lossy `as u32` narrowing is needed:
            // a `tag_type = "u64"` discriminant above `u32::MAX` is preserved and matched exactly.
            let decode_tag = match tag_type {
                TagType::U8 => quote! {
                    let __variant_tag = <u8 as #crate_path::Decode<_>>::decode(decoder)?;
                },
                TagType::U16 => quote! {
                    let __variant_tag = <u16 as #crate_path::Decode<_>>::decode(decoder)?;
                },
                TagType::U32 => quote! {
                    let __variant_tag = <u32 as #crate_path::Decode<_>>::decode(decoder)?;
                },
                TagType::U64 => quote! {
                    let __variant_tag = <u64 as #crate_path::Decode<_>>::decode(decoder)?;
                },
            };

            Ok(quote! {
                #decode_tag
                match __variant_tag {
                    #(#variant_decodings,)*
                    _ => Err(#crate_path::Error::InvalidData {
                        message: "Invalid enum variant"
                    })
                }
            })
        }
        Data::Union(data_union) => Err(syn::Error::new(
            data_union.union_token.span(),
            "Decode cannot be derived for unions",
        )),
    }
}

/// Generate decode body for a `#[oxicode(transparent)]` struct (exactly one field).
fn derive_decode_transparent(
    fields: &Fields,
    crate_path: &syn::Path,
) -> Result<TokenStream2, syn::Error> {
    match fields {
        Fields::Named(named) => {
            if named.named.len() != 1 {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                        "#[oxicode(transparent)] requires exactly one field, but found {}",
                        named.named.len()
                    ),
                ));
            }
            let field = named
                .named
                .first()
                .ok_or_else(|| syn::Error::new(proc_macro2::Span::call_site(), "expected field"))?;
            let field_name = &field.ident;
            let field_ty = &field.ty;
            Ok(quote! {
                Ok(Self {
                    #field_name: <#field_ty as #crate_path::Decode<_>>::decode(decoder)?,
                })
            })
        }
        Fields::Unnamed(unnamed) => {
            if unnamed.unnamed.len() != 1 {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                        "#[oxicode(transparent)] requires exactly one field, but found {}",
                        unnamed.unnamed.len()
                    ),
                ));
            }
            let field = unnamed
                .unnamed
                .first()
                .ok_or_else(|| syn::Error::new(proc_macro2::Span::call_site(), "expected field"))?;
            let field_ty = &field.ty;
            Ok(quote! {
                Ok(Self(<#field_ty as #crate_path::Decode<_>>::decode(decoder)?))
            })
        }
        Fields::Unit => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "#[oxicode(transparent)] requires exactly one field, but found 0 (unit struct)",
        )),
    }
}

/// Generate field initializers for decoding a struct.
fn derive_decode_struct(
    fields: &Fields,
    crate_path: &syn::Path,
) -> Result<TokenStream2, syn::Error> {
    match fields {
        Fields::Named(named) => {
            let field_inits: Vec<TokenStream2> = named
                .named
                .iter()
                .map(|f| {
                    let attrs = parse_field_attrs(f)?;
                    let field_name = &f.ident;
                    let field_ty = &f.ty;
                    if attrs.bytes {
                        let decode_expr = make_bytes_decode_expr(field_ty, crate_path);
                        Ok(quote! { #field_name: #decode_expr })
                    } else if let Some(ref len_ty_str) = attrs.seq_len {
                        let decode_expr =
                            make_seq_len_decode_expr(len_ty_str, crate_path, vec_element_type(field_ty));
                        Ok(quote! { #field_name: #decode_expr })
                    } else if attrs.skip {
                        if let Some(ref expr) = attrs.default_expr {
                            Ok(quote! { #field_name: #expr })
                        } else {
                            Ok(quote! { #field_name: <#field_ty as ::core::default::Default>::default() })
                        }
                    } else if let Some(ref default_fn) = attrs.default_fn {
                        if let Some(ref expr) = attrs.default_expr {
                            Ok(quote! { #field_name: #expr })
                        } else {
                            Ok(quote! { #field_name: #default_fn() })
                        }
                    } else if let Some(ref path) = attrs.with_module {
                        Ok(quote! { #field_name: #path::decode(decoder)? })
                    } else if let Some(ref path) = attrs.decode_with {
                        Ok(quote! { #field_name: #path(decoder)? })
                    } else {
                        Ok(quote! { #field_name: <#field_ty as #crate_path::Decode<_>>::decode(decoder)? })
                    }
                })
                .collect::<Result<_, syn::Error>>()?;
            Ok(quote! {
                Ok(Self {
                    #(#field_inits,)*
                })
            })
        }
        Fields::Unnamed(unnamed) => {
            let field_inits: Vec<TokenStream2> = unnamed
                .unnamed
                .iter()
                .map(|f| {
                    let attrs = parse_field_attrs(f)?;
                    let field_ty = &f.ty;
                    if attrs.bytes {
                        Ok(make_bytes_decode_expr(field_ty, crate_path))
                    } else if let Some(ref len_ty_str) = attrs.seq_len {
                        Ok(make_seq_len_decode_expr(
                            len_ty_str,
                            crate_path,
                            vec_element_type(field_ty),
                        ))
                    } else if attrs.skip {
                        if let Some(ref expr) = attrs.default_expr {
                            Ok(quote! { #expr })
                        } else {
                            Ok(quote! { <#field_ty as ::core::default::Default>::default() })
                        }
                    } else if let Some(ref default_fn) = attrs.default_fn {
                        if let Some(ref expr) = attrs.default_expr {
                            Ok(quote! { #expr })
                        } else {
                            Ok(quote! { #default_fn() })
                        }
                    } else if let Some(ref path) = attrs.with_module {
                        Ok(quote! { #path::decode(decoder)? })
                    } else if let Some(ref path) = attrs.decode_with {
                        Ok(quote! { #path(decoder)? })
                    } else {
                        Ok(quote! { <#field_ty as #crate_path::Decode<_>>::decode(decoder)? })
                    }
                })
                .collect::<Result<_, syn::Error>>()?;
            Ok(quote! {
                Ok(Self(#(#field_inits,)*))
            })
        }
        Fields::Unit => Ok(quote! { Ok(Self) }),
    }
}

/// Generate a single match arm for decoding an enum variant.
fn derive_decode_variant(
    idx: usize,
    variant: &syn::Variant,
    crate_path: &syn::Path,
    tag_type: TagType,
) -> Result<TokenStream2, syn::Error> {
    let variant_name = &variant.ident;
    let variant_attrs = parse_variant_attrs(&variant.attrs)?;
    let discriminant = variant_attrs.tag.unwrap_or(idx as u64);
    let discriminant_lit = discriminant_literal(discriminant, tag_type, variant.ident.span())?;

    match &variant.fields {
        Fields::Named(fields) => {
            let field_inits: Vec<TokenStream2> = fields
                .named
                .iter()
                .map(|f| {
                    let attrs = parse_field_attrs(f)?;
                    let field_name = &f.ident;
                    let field_ty = &f.ty;
                    if attrs.bytes {
                        let decode_expr = make_bytes_decode_expr(field_ty, crate_path);
                        Ok(quote! { #field_name: #decode_expr })
                    } else if let Some(ref len_ty_str) = attrs.seq_len {
                        let decode_expr =
                            make_seq_len_decode_expr(len_ty_str, crate_path, vec_element_type(field_ty));
                        Ok(quote! { #field_name: #decode_expr })
                    } else if attrs.skip {
                        if let Some(ref expr) = attrs.default_expr {
                            Ok(quote! { #field_name: #expr })
                        } else {
                            Ok(quote! { #field_name: <#field_ty as ::core::default::Default>::default() })
                        }
                    } else if let Some(ref default_fn) = attrs.default_fn {
                        if let Some(ref expr) = attrs.default_expr {
                            Ok(quote! { #field_name: #expr })
                        } else {
                            Ok(quote! { #field_name: #default_fn() })
                        }
                    } else if let Some(ref path) = attrs.with_module {
                        Ok(quote! { #field_name: #path::decode(decoder)? })
                    } else if let Some(ref path) = attrs.decode_with {
                        Ok(quote! { #field_name: #path(decoder)? })
                    } else {
                        Ok(quote! { #field_name: <#field_ty as #crate_path::Decode<_>>::decode(decoder)? })
                    }
                })
                .collect::<Result<_, syn::Error>>()?;
            Ok(quote! {
                #discriminant_lit => Ok(Self::#variant_name { #(#field_inits,)* })
            })
        }
        Fields::Unnamed(fields) => {
            let field_inits: Vec<TokenStream2> = fields
                .unnamed
                .iter()
                .map(|f| {
                    let attrs = parse_field_attrs(f)?;
                    let field_ty = &f.ty;
                    if attrs.bytes {
                        Ok(make_bytes_decode_expr(field_ty, crate_path))
                    } else if let Some(ref len_ty_str) = attrs.seq_len {
                        Ok(make_seq_len_decode_expr(
                            len_ty_str,
                            crate_path,
                            vec_element_type(field_ty),
                        ))
                    } else if attrs.skip {
                        if let Some(ref expr) = attrs.default_expr {
                            Ok(quote! { #expr })
                        } else {
                            Ok(quote! { <#field_ty as ::core::default::Default>::default() })
                        }
                    } else if let Some(ref default_fn) = attrs.default_fn {
                        if let Some(ref expr) = attrs.default_expr {
                            Ok(quote! { #expr })
                        } else {
                            Ok(quote! { #default_fn() })
                        }
                    } else if let Some(ref path) = attrs.with_module {
                        Ok(quote! { #path::decode(decoder)? })
                    } else if let Some(ref path) = attrs.decode_with {
                        Ok(quote! { #path(decoder)? })
                    } else {
                        Ok(quote! { <#field_ty as #crate_path::Decode<_>>::decode(decoder)? })
                    }
                })
                .collect::<Result<_, syn::Error>>()?;
            Ok(quote! {
                #discriminant_lit => Ok(Self::#variant_name(#(#field_inits,)*))
            })
        }
        Fields::Unit => Ok(quote! {
            #discriminant_lit => Ok(Self::#variant_name)
        }),
    }
}

/// Build the `impl_generics` and `effective_where` tokens for a Decode impl.
pub(crate) fn build_decode_generics(
    generics: &syn::Generics,
    crate_path: &syn::Path,
    bound: &Option<Vec<syn::WherePredicate>>,
    context: Option<&syn::Type>,
    context_generic: bool,
) -> (TokenStream2, TokenStream2) {
    let (_impl_generics, _ty_generics, where_clause) = generics.split_for_impl();

    // `#[oxicode(context_generic)]` makes the impl generic over a fresh context
    // parameter, which is what lets a derived type be decoded under *any*
    // context. `#[oxicode(decode_context = "...")]` instead names one specific
    // context — either a concrete type or a parameter the container declares.
    let fresh_ctx = context_generic.then(generic_context_ident);

    // The auto-bound on each type parameter has to name the same context, or a
    // generic field would only be decodable under `()`.
    let decode_bound: syn::TypeParamBound = match context {
        Some(ctx) => syn::parse_quote!(#crate_path::Decode<#ctx>),
        None => syn::parse_quote!(#crate_path::Decode),
    };

    let mut generics_with_bounds = generics.clone();
    insert_context_param(&mut generics_with_bounds, fresh_ctx.as_ref());
    if bound.is_none() {
        for param in &mut generics_with_bounds.params {
            if let syn::GenericParam::Type(type_param) = param {
                // Never bound the context parameter by `Decode`: neither the
                // one the container declares (`C: Decode<C>`) nor the fresh one
                // the derive just introduced.
                if is_context_param(context, &type_param.ident)
                    || fresh_ctx.as_ref() == Some(&type_param.ident)
                {
                    continue;
                }
                type_param.bounds.push(decode_bound.clone());
            }
        }
    }

    let (impl_generics_with_bounds, _, generated_where) = generics_with_bounds.split_for_impl();
    let impl_generics_tokens = quote! { #impl_generics_with_bounds };

    let effective_where = if let Some(ref predicates) = bound {
        match where_clause {
            Some(wc) if !predicates.is_empty() => {
                quote! { #wc #(, #predicates)* }
            }
            Some(wc) => quote! { #wc },
            None => predicates_to_where_clause(predicates),
        }
    } else {
        match generated_where {
            Some(wc) => quote! { #wc },
            None => quote! {},
        }
    };

    (impl_generics_tokens, effective_where)
}

/// Is `ident` the type parameter named by the container's context attribute?
///
/// Only true when the attribute named a bare identifier that the container also
/// declares as a generic parameter — the `#[oxicode(decode_context = "Ctx")]`
/// on `struct Foo<Ctx, T>` case. Such a parameter must not get a `Decode` bound
/// of its own: `Ctx: Decode<Ctx>` is not what the user asked for.
fn is_context_param(context: Option<&syn::Type>, ident: &syn::Ident) -> bool {
    match context.and_then(context_ident) {
        Some(ctx_ident) => ctx_ident == *ident,
        None => false,
    }
}

/// The single identifier a context type is written as, if it is one.
///
/// `"Ctx"` yields `Some(Ctx)`; `"my_crate::Arena"` or `"&'a Bump"` yield `None`
/// because those are unambiguously concrete types.
fn context_ident(ty: &syn::Type) -> Option<syn::Ident> {
    let syn::Type::Path(type_path) = ty else {
        return None;
    };
    if type_path.qself.is_some() {
        return None;
    }
    let ident = type_path.path.get_ident()?;
    Some(ident.clone())
}

/// Name of the context parameter introduced by `#[oxicode(context_generic)]`.
///
/// Deliberately not user-chosen: a name given in an attribute string cannot be
/// told apart from a concrete type at macro-expansion time, so accepting one
/// would silently shadow real types (`decode_context = "Tracker"` becoming
/// `impl<Tracker> ...`). The flag form has no such ambiguity.
pub(crate) fn generic_context_ident() -> syn::Ident {
    syn::Ident::new("__Ctx", proc_macro2::Span::call_site())
}

/// Declare `ctx` on the impl, after every lifetime parameter (Rust requires
/// lifetimes to come first in a generic parameter list).
fn insert_context_param(generics: &mut syn::Generics, ctx: Option<&syn::Ident>) {
    let Some(ident) = ctx else {
        return;
    };
    let position = generics
        .params
        .iter()
        .position(|param| !matches!(param, syn::GenericParam::Lifetime(_)))
        .unwrap_or(generics.params.len());
    generics.params.insert(
        position,
        syn::GenericParam::Type(syn::TypeParam::from(ident.clone())),
    );
}

// ---------------------------------------------------------------------------
// BorrowDecode helpers
// ---------------------------------------------------------------------------

/// Generate the body of `BorrowDecode::borrow_decode`.
pub(crate) fn derive_borrow_decode_body(
    data: &Data,
    de_lifetime: &syn::Lifetime,
    crate_path: &syn::Path,
    transparent: bool,
    tag_type: TagType,
) -> Result<TokenStream2, syn::Error> {
    if transparent {
        return match data {
            Data::Struct(data_struct) => {
                derive_borrow_decode_transparent(&data_struct.fields, de_lifetime, crate_path)
            }
            Data::Enum(data_enum) => Err(syn::Error::new(
                data_enum.enum_token.span(),
                "#[oxicode(transparent)] is not supported on enums",
            )),
            Data::Union(data_union) => Err(syn::Error::new(
                data_union.union_token.span(),
                "#[oxicode(transparent)] is not supported on unions",
            )),
        };
    }
    match data {
        Data::Struct(data_struct) => {
            derive_borrow_decode_struct(&data_struct.fields, de_lifetime, crate_path)
        }
        Data::Enum(data_enum) => {
            // Mirror the `Decode` derive: skipped variants get no borrow-decode arm, so the
            // accepted byte set stays identical between `Decode` and `BorrowDecode`.
            let variant_decodings: Vec<TokenStream2> = data_enum
                .variants
                .iter()
                .enumerate()
                .filter_map(|(idx, variant)| match parse_variant_attrs(&variant.attrs) {
                    Ok(attrs) if attrs.skip => None,
                    Ok(_) => Some(derive_borrow_decode_variant(
                        idx,
                        variant,
                        de_lifetime,
                        crate_path,
                        tag_type,
                    )),
                    Err(e) => Some(Err(e)),
                })
                .collect::<Result<_, _>>()?;

            // Primitive integers implement Decode (not BorrowDecode), so use Decode here.
            // Decode the tag at its native width; the arm literals match that width exactly.
            let decode_tag = match tag_type {
                TagType::U8 => quote! {
                    let __variant_tag = <u8 as #crate_path::de::Decode<_>>::decode(decoder)?;
                },
                TagType::U16 => quote! {
                    let __variant_tag = <u16 as #crate_path::de::Decode<_>>::decode(decoder)?;
                },
                TagType::U32 => quote! {
                    let __variant_tag = <u32 as #crate_path::de::Decode<_>>::decode(decoder)?;
                },
                TagType::U64 => quote! {
                    let __variant_tag = <u64 as #crate_path::de::Decode<_>>::decode(decoder)?;
                },
            };

            Ok(quote! {
                #decode_tag
                match __variant_tag {
                    #(#variant_decodings,)*
                    _ => Err(#crate_path::Error::InvalidData {
                        message: "Invalid enum variant"
                    })
                }
            })
        }
        Data::Union(data_union) => Err(syn::Error::new(
            data_union.union_token.span(),
            "BorrowDecode cannot be derived for unions",
        )),
    }
}

/// Generate borrow-decode body for a `#[oxicode(transparent)]` struct (exactly one field).
fn derive_borrow_decode_transparent(
    fields: &Fields,
    de_lifetime: &syn::Lifetime,
    crate_path: &syn::Path,
) -> Result<TokenStream2, syn::Error> {
    match fields {
        Fields::Named(named) => {
            if named.named.len() != 1 {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                        "#[oxicode(transparent)] requires exactly one field, but found {}",
                        named.named.len()
                    ),
                ));
            }
            let field = named
                .named
                .first()
                .ok_or_else(|| syn::Error::new(proc_macro2::Span::call_site(), "expected field"))?;
            let field_name = &field.ident;
            let field_ty = &field.ty;
            Ok(quote! {
                Ok(Self {
                    #field_name: <#field_ty as #crate_path::de::BorrowDecode<#de_lifetime, _>>::borrow_decode(decoder)?,
                })
            })
        }
        Fields::Unnamed(unnamed) => {
            if unnamed.unnamed.len() != 1 {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                        "#[oxicode(transparent)] requires exactly one field, but found {}",
                        unnamed.unnamed.len()
                    ),
                ));
            }
            let field = unnamed
                .unnamed
                .first()
                .ok_or_else(|| syn::Error::new(proc_macro2::Span::call_site(), "expected field"))?;
            let field_ty = &field.ty;
            Ok(quote! {
                Ok(Self(<#field_ty as #crate_path::de::BorrowDecode<#de_lifetime, _>>::borrow_decode(decoder)?))
            })
        }
        Fields::Unit => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "#[oxicode(transparent)] requires exactly one field, but found 0 (unit struct)",
        )),
    }
}

/// Generate field initializers for borrow-decoding a struct.
fn derive_borrow_decode_struct(
    fields: &Fields,
    de_lifetime: &syn::Lifetime,
    crate_path: &syn::Path,
) -> Result<TokenStream2, syn::Error> {
    match fields {
        Fields::Named(named) => {
            let field_inits: Vec<TokenStream2> = named
                .named
                .iter()
                .map(|f| {
                    let attrs = parse_field_attrs(f)?;
                    let field_name = &f.ident;
                    let field_ty = &f.ty;
                    if attrs.bytes {
                        let decode_expr =
                            make_bytes_borrow_decode_expr(field_ty, de_lifetime, crate_path);
                        Ok(quote! { #field_name: #decode_expr })
                    } else if let Some(ref len_ty_str) = attrs.seq_len {
                        let decode_expr =
                            make_seq_len_decode_expr(len_ty_str, crate_path, vec_element_type(field_ty));
                        Ok(quote! { #field_name: #decode_expr })
                    } else if attrs.skip {
                        if let Some(ref expr) = attrs.default_expr {
                            Ok(quote! { #field_name: #expr })
                        } else {
                            Ok(quote! {
                                #field_name: <#field_ty as ::core::default::Default>::default()
                            })
                        }
                    } else if let Some(ref default_fn) = attrs.default_fn {
                        if let Some(ref expr) = attrs.default_expr {
                            Ok(quote! { #field_name: #expr })
                        } else {
                            Ok(quote! { #field_name: #default_fn() })
                        }
                    } else if let Some(ref path) = attrs.with_module {
                        Ok(quote! { #field_name: #path::decode(decoder)? })
                    } else if let Some(ref path) = attrs.decode_with {
                        Ok(quote! { #field_name: #path(decoder)? })
                    } else {
                        Ok(quote! {
                            #field_name: <#field_ty as #crate_path::de::BorrowDecode<#de_lifetime, _>>::borrow_decode(decoder)?
                        })
                    }
                })
                .collect::<Result<_, syn::Error>>()?;
            Ok(quote! {
                Ok(Self {
                    #(#field_inits,)*
                })
            })
        }
        Fields::Unnamed(unnamed) => {
            let field_inits: Vec<TokenStream2> = unnamed
                .unnamed
                .iter()
                .map(|f| {
                    let attrs = parse_field_attrs(f)?;
                    let field_ty = &f.ty;
                    if attrs.bytes {
                        Ok(make_bytes_borrow_decode_expr(field_ty, de_lifetime, crate_path))
                    } else if let Some(ref len_ty_str) = attrs.seq_len {
                        Ok(make_seq_len_decode_expr(len_ty_str, crate_path, vec_element_type(field_ty)))
                    } else if attrs.skip {
                        if let Some(ref expr) = attrs.default_expr {
                            Ok(quote! { #expr })
                        } else {
                            Ok(quote! { <#field_ty as ::core::default::Default>::default() })
                        }
                    } else if let Some(ref default_fn) = attrs.default_fn {
                        if let Some(ref expr) = attrs.default_expr {
                            Ok(quote! { #expr })
                        } else {
                            Ok(quote! { #default_fn() })
                        }
                    } else if let Some(ref path) = attrs.with_module {
                        Ok(quote! { #path::decode(decoder)? })
                    } else if let Some(ref path) = attrs.decode_with {
                        Ok(quote! { #path(decoder)? })
                    } else {
                        Ok(quote! {
                            <#field_ty as #crate_path::de::BorrowDecode<#de_lifetime, _>>::borrow_decode(decoder)?
                        })
                    }
                })
                .collect::<Result<_, syn::Error>>()?;
            Ok(quote! {
                Ok(Self(#(#field_inits,)*))
            })
        }
        Fields::Unit => Ok(quote! { Ok(Self) }),
    }
}

/// Generate a single match arm for borrow-decoding an enum variant.
fn derive_borrow_decode_variant(
    idx: usize,
    variant: &syn::Variant,
    de_lifetime: &syn::Lifetime,
    crate_path: &syn::Path,
    tag_type: TagType,
) -> Result<TokenStream2, syn::Error> {
    let variant_name = &variant.ident;
    let variant_attrs = parse_variant_attrs(&variant.attrs)?;
    let discriminant = variant_attrs.tag.unwrap_or(idx as u64);
    let discriminant_lit = discriminant_literal(discriminant, tag_type, variant.ident.span())?;

    match &variant.fields {
        Fields::Named(fields) => {
            let field_inits: Vec<TokenStream2> = fields
                .named
                .iter()
                .map(|f| {
                    let attrs = parse_field_attrs(f)?;
                    let field_name = &f.ident;
                    let field_ty = &f.ty;
                    if attrs.bytes {
                        let decode_expr =
                            make_bytes_borrow_decode_expr(field_ty, de_lifetime, crate_path);
                        Ok(quote! { #field_name: #decode_expr })
                    } else if let Some(ref len_ty_str) = attrs.seq_len {
                        let decode_expr =
                            make_seq_len_decode_expr(len_ty_str, crate_path, vec_element_type(field_ty));
                        Ok(quote! { #field_name: #decode_expr })
                    } else if attrs.skip {
                        if let Some(ref expr) = attrs.default_expr {
                            Ok(quote! { #field_name: #expr })
                        } else {
                            Ok(quote! {
                                #field_name: <#field_ty as ::core::default::Default>::default()
                            })
                        }
                    } else if let Some(ref default_fn) = attrs.default_fn {
                        if let Some(ref expr) = attrs.default_expr {
                            Ok(quote! { #field_name: #expr })
                        } else {
                            Ok(quote! { #field_name: #default_fn() })
                        }
                    } else if let Some(ref path) = attrs.with_module {
                        Ok(quote! { #field_name: #path::decode(decoder)? })
                    } else if let Some(ref path) = attrs.decode_with {
                        Ok(quote! { #field_name: #path(decoder)? })
                    } else {
                        Ok(quote! {
                            #field_name: <#field_ty as #crate_path::de::BorrowDecode<#de_lifetime, _>>::borrow_decode(decoder)?
                        })
                    }
                })
                .collect::<Result<_, syn::Error>>()?;
            Ok(quote! {
                #discriminant_lit => Ok(Self::#variant_name { #(#field_inits,)* })
            })
        }
        Fields::Unnamed(fields) => {
            let field_inits: Vec<TokenStream2> = fields
                .unnamed
                .iter()
                .map(|f| {
                    let attrs = parse_field_attrs(f)?;
                    let field_ty = &f.ty;
                    if attrs.bytes {
                        Ok(make_bytes_borrow_decode_expr(field_ty, de_lifetime, crate_path))
                    } else if let Some(ref len_ty_str) = attrs.seq_len {
                        Ok(make_seq_len_decode_expr(len_ty_str, crate_path, vec_element_type(field_ty)))
                    } else if attrs.skip {
                        if let Some(ref expr) = attrs.default_expr {
                            Ok(quote! { #expr })
                        } else {
                            Ok(quote! { <#field_ty as ::core::default::Default>::default() })
                        }
                    } else if let Some(ref default_fn) = attrs.default_fn {
                        if let Some(ref expr) = attrs.default_expr {
                            Ok(quote! { #expr })
                        } else {
                            Ok(quote! { #default_fn() })
                        }
                    } else if let Some(ref path) = attrs.with_module {
                        Ok(quote! { #path::decode(decoder)? })
                    } else if let Some(ref path) = attrs.decode_with {
                        Ok(quote! { #path(decoder)? })
                    } else {
                        Ok(quote! {
                            <#field_ty as #crate_path::de::BorrowDecode<#de_lifetime, _>>::borrow_decode(decoder)?
                        })
                    }
                })
                .collect::<Result<_, syn::Error>>()?;
            Ok(quote! {
                #discriminant_lit => Ok(Self::#variant_name(#(#field_inits,)*))
            })
        }
        Fields::Unit => Ok(quote! {
            #discriminant_lit => Ok(Self::#variant_name)
        }),
    }
}

/// Build `impl_generics_with_bounds`, `de_lifetime`, and `effective_where` for a BorrowDecode impl.
pub(crate) fn build_borrow_decode_generics(
    generics: &syn::Generics,
    crate_path: &syn::Path,
    bound: &Option<Vec<syn::WherePredicate>>,
    context: Option<&syn::Type>,
    context_generic: bool,
) -> (TokenStream2, syn::Lifetime, TokenStream2) {
    let (_impl_generics, _ty_generics, where_clause) = generics.split_for_impl();

    // Collect existing lifetime params on the struct
    let struct_lifetimes: Vec<syn::Lifetime> = generics
        .params
        .iter()
        .filter_map(|p| {
            if let syn::GenericParam::Lifetime(lp) = p {
                Some(lp.lifetime.clone())
            } else {
                None
            }
        })
        .collect();

    // If the struct has exactly one lifetime, reuse it as the decode lifetime.
    let de_lifetime: syn::Lifetime = if struct_lifetimes.len() == 1 {
        struct_lifetimes[0].clone()
    } else {
        syn::parse_quote!('__de)
    };

    // Build generics with de_lifetime prepended (only if it's a fresh lifetime)
    // and BorrowDecode bounds on type params.
    let mut generics_with_bounds = generics.clone();

    let fresh_ctx = context_generic.then(generic_context_ident);
    insert_context_param(&mut generics_with_bounds, fresh_ctx.as_ref());

    let borrow_bound: syn::TypeParamBound = match context {
        Some(ctx) => {
            syn::parse_quote!(#crate_path::de::BorrowDecode<#de_lifetime, #ctx>)
        }
        None => syn::parse_quote!(#crate_path::de::BorrowDecode<#de_lifetime>),
    };

    if bound.is_none() {
        // Auto-generate BorrowDecode bounds.
        for param in &mut generics_with_bounds.params {
            if let syn::GenericParam::Type(type_param) = param {
                if is_context_param(context, &type_param.ident)
                    || fresh_ctx.as_ref() == Some(&type_param.ident)
                {
                    continue;
                }
                type_param.bounds.push(borrow_bound.clone());
            }
        }
    }

    // Only prepend the fresh '__de lifetime if we're not reusing a struct lifetime
    if struct_lifetimes.len() != 1 {
        let de_lifetime_param = LifetimeParam::new(de_lifetime.clone());
        generics_with_bounds
            .params
            .insert(0, syn::GenericParam::Lifetime(de_lifetime_param));
    }

    let (impl_generics_with_bounds, _, _) = generics_with_bounds.split_for_impl();

    let effective_where = if let Some(ref predicates) = bound {
        match where_clause {
            Some(wc) if !predicates.is_empty() => {
                quote! { #wc #(, #predicates)* }
            }
            Some(wc) => quote! { #wc },
            None => predicates_to_where_clause(predicates),
        }
    } else {
        let (_, _, wc) = generics_with_bounds.split_for_impl();
        match wc {
            Some(wc) => quote! { #wc },
            None => quote! {},
        }
    };

    (
        quote! { #impl_generics_with_bounds },
        de_lifetime,
        effective_where,
    )
}

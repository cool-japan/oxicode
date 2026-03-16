//! Decode and BorrowDecode derive implementations for OxiCode.
//!
//! Contains code-generation helpers for `Decode` and `BorrowDecode` trait derive macros.

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, Fields, LifetimeParam};

use crate::attrs::{parse_field_attrs, parse_variant_attrs, predicates_to_where_clause, TagType};

// ---------------------------------------------------------------------------
// Shared seq_len decode helper
// ---------------------------------------------------------------------------

/// Generate a seq_len decode block for a Vec/sequence field.
///
/// `len_ty_str` is one of "u8", "u16", "u32", "u64".
/// Uses type inference so the compiler resolves the element type from the field type.
pub(crate) fn make_seq_len_decode_expr(len_ty_str: &str, crate_path: &syn::Path) -> TokenStream2 {
    let len_ty_tokens: TokenStream2 = match len_ty_str {
        "u8" => quote! { u8 },
        "u16" => quote! { u16 },
        "u32" => quote! { u32 },
        _ => quote! { u64 },
    };
    quote! {
        {
            let __seq_len = <#len_ty_tokens as #crate_path::Decode>::decode(decoder)? as usize;
            let mut __vec = Vec::with_capacity(__seq_len);
            for _ in 0..__seq_len {
                __vec.push(<_ as #crate_path::Decode>::decode(decoder)?);
            }
            __vec
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
            Data::Enum(_) => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "#[oxicode(transparent)] is not supported on enums",
            )),
            Data::Union(_) => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
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
                    Ok(_) => Some(derive_decode_variant(idx, variant, crate_path)),
                    Err(e) => Some(Err(e)),
                })
                .collect::<Result<_, _>>()?;

            // Decode the discriminant tag as the appropriate integer width, then widen to u32.
            let decode_tag = match tag_type {
                TagType::U8 => quote! {
                    let __variant_tag = <u8 as #crate_path::Decode>::decode(decoder)? as u32;
                },
                TagType::U16 => quote! {
                    let __variant_tag = <u16 as #crate_path::Decode>::decode(decoder)? as u32;
                },
                TagType::U32 => quote! {
                    let __variant_tag = <u32 as #crate_path::Decode>::decode(decoder)?;
                },
                TagType::U64 => quote! {
                    let __variant_tag = <u64 as #crate_path::Decode>::decode(decoder)? as u32;
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
        Data::Union(_) => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
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
                    #field_name: <#field_ty as #crate_path::Decode>::decode(decoder)?,
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
                Ok(Self(<#field_ty as #crate_path::Decode>::decode(decoder)?))
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
                        Ok(quote! {
                            #field_name: {
                                let __len = <u64 as #crate_path::de::Decode>::decode(decoder)? as usize;
                                decoder.claim_bytes_read(__len)?;
                                let mut __buf: Vec<u8> = Vec::with_capacity(__len);
                                __buf.resize(__len, 0u8);
                                <_ as #crate_path::de::read::Reader>::read(decoder.reader(), &mut __buf)?;
                                __buf
                            }
                        })
                    } else if let Some(ref len_ty_str) = attrs.seq_len {
                        let decode_expr = make_seq_len_decode_expr(len_ty_str, crate_path);
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
                        Ok(quote! { #field_name: <#field_ty>::decode(decoder)? })
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
                        Ok(quote! {
                            {
                                let __len = <u64 as #crate_path::de::Decode>::decode(decoder)? as usize;
                                decoder.claim_bytes_read(__len)?;
                                let mut __buf: Vec<u8> = Vec::with_capacity(__len);
                                __buf.resize(__len, 0u8);
                                <_ as #crate_path::de::read::Reader>::read(decoder.reader(), &mut __buf)?;
                                __buf
                            }
                        })
                    } else if let Some(ref len_ty_str) = attrs.seq_len {
                        Ok(make_seq_len_decode_expr(len_ty_str, crate_path))
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
                        Ok(quote! { <#field_ty>::decode(decoder)? })
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
) -> Result<TokenStream2, syn::Error> {
    let variant_name = &variant.ident;
    let variant_attrs = parse_variant_attrs(&variant.attrs)?;
    let discriminant = variant_attrs.tag.unwrap_or(idx as u32);
    let discriminant_lit = proc_macro2::Literal::u32_suffixed(discriminant);

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
                        Ok(quote! {
                            #field_name: {
                                let __len = <u64 as #crate_path::de::Decode>::decode(decoder)? as usize;
                                decoder.claim_bytes_read(__len)?;
                                let mut __buf: Vec<u8> = Vec::with_capacity(__len);
                                __buf.resize(__len, 0u8);
                                <_ as #crate_path::de::read::Reader>::read(decoder.reader(), &mut __buf)?;
                                __buf
                            }
                        })
                    } else if let Some(ref len_ty_str) = attrs.seq_len {
                        let decode_expr = make_seq_len_decode_expr(len_ty_str, crate_path);
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
                        Ok(quote! { #field_name: <#field_ty>::decode(decoder)? })
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
                        Ok(quote! {
                            {
                                let __len = <u64 as #crate_path::de::Decode>::decode(decoder)? as usize;
                                decoder.claim_bytes_read(__len)?;
                                let mut __buf: Vec<u8> = Vec::with_capacity(__len);
                                __buf.resize(__len, 0u8);
                                <_ as #crate_path::de::read::Reader>::read(decoder.reader(), &mut __buf)?;
                                __buf
                            }
                        })
                    } else if let Some(ref len_ty_str) = attrs.seq_len {
                        Ok(make_seq_len_decode_expr(len_ty_str, crate_path))
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
                        Ok(quote! { <#field_ty>::decode(decoder)? })
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
) -> (TokenStream2, TokenStream2) {
    let (impl_generics, _ty_generics, where_clause) = generics.split_for_impl();

    let impl_generics_tokens = if bound.is_some() {
        quote! { #impl_generics }
    } else {
        let mut generics_with_bounds = generics.clone();
        for param in &mut generics_with_bounds.params {
            if let syn::GenericParam::Type(type_param) = param {
                type_param
                    .bounds
                    .push(syn::parse_quote!(#crate_path::Decode));
            }
        }
        let (ig, _, _) = generics_with_bounds.split_for_impl();
        quote! { #ig }
    };

    let effective_where = if let Some(ref predicates) = bound {
        match where_clause {
            Some(wc) if !predicates.is_empty() => {
                quote! { #wc #(, #predicates)* }
            }
            Some(wc) => quote! { #wc },
            None => predicates_to_where_clause(predicates),
        }
    } else {
        let mut generics_with_bounds = generics.clone();
        for param in &mut generics_with_bounds.params {
            if let syn::GenericParam::Type(type_param) = param {
                type_param
                    .bounds
                    .push(syn::parse_quote!(#crate_path::Decode));
            }
        }
        let (_, _, wc) = generics_with_bounds.split_for_impl();
        match wc {
            Some(wc) => quote! { #wc },
            None => quote! {},
        }
    };

    (impl_generics_tokens, effective_where)
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
            Data::Enum(_) => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "#[oxicode(transparent)] is not supported on enums",
            )),
            Data::Union(_) => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "#[oxicode(transparent)] is not supported on unions",
            )),
        };
    }
    match data {
        Data::Struct(data_struct) => {
            derive_borrow_decode_struct(&data_struct.fields, de_lifetime, crate_path)
        }
        Data::Enum(data_enum) => {
            let variant_decodings: Vec<TokenStream2> = data_enum
                .variants
                .iter()
                .enumerate()
                .map(|(idx, variant)| {
                    derive_borrow_decode_variant(idx, variant, de_lifetime, crate_path)
                })
                .collect::<Result<_, _>>()?;

            // Primitive integers implement Decode (not BorrowDecode), so use Decode here.
            let decode_tag = match tag_type {
                TagType::U8 => quote! {
                    let __variant_tag = <u8 as #crate_path::de::Decode>::decode(decoder)? as u32;
                },
                TagType::U16 => quote! {
                    let __variant_tag = <u16 as #crate_path::de::Decode>::decode(decoder)? as u32;
                },
                TagType::U32 => quote! {
                    let __variant_tag = <u32 as #crate_path::de::Decode>::decode(decoder)?;
                },
                TagType::U64 => quote! {
                    let __variant_tag = <u64 as #crate_path::de::Decode>::decode(decoder)? as u32;
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
        Data::Union(_) => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
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
                    #field_name: <#field_ty as #crate_path::de::BorrowDecode<#de_lifetime>>::borrow_decode(decoder)?,
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
                Ok(Self(<#field_ty as #crate_path::de::BorrowDecode<#de_lifetime>>::borrow_decode(decoder)?))
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
                    if let Some(ref len_ty_str) = attrs.seq_len {
                        let decode_expr = make_seq_len_decode_expr(len_ty_str, crate_path);
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
                            #field_name: <#field_ty as #crate_path::de::BorrowDecode<#de_lifetime>>::borrow_decode(decoder)?
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
                    if let Some(ref len_ty_str) = attrs.seq_len {
                        Ok(make_seq_len_decode_expr(len_ty_str, crate_path))
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
                            <#field_ty as #crate_path::de::BorrowDecode<#de_lifetime>>::borrow_decode(decoder)?
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
) -> Result<TokenStream2, syn::Error> {
    let variant_name = &variant.ident;
    let variant_attrs = parse_variant_attrs(&variant.attrs)?;
    let discriminant = variant_attrs.tag.unwrap_or(idx as u32);
    let discriminant_lit = proc_macro2::Literal::u32_suffixed(discriminant);

    match &variant.fields {
        Fields::Named(fields) => {
            let field_inits: Vec<TokenStream2> = fields
                .named
                .iter()
                .map(|f| {
                    let attrs = parse_field_attrs(f)?;
                    let field_name = &f.ident;
                    let field_ty = &f.ty;
                    if let Some(ref len_ty_str) = attrs.seq_len {
                        let decode_expr = make_seq_len_decode_expr(len_ty_str, crate_path);
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
                            #field_name: <#field_ty as #crate_path::de::BorrowDecode<#de_lifetime>>::borrow_decode(decoder)?
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
                    if let Some(ref len_ty_str) = attrs.seq_len {
                        Ok(make_seq_len_decode_expr(len_ty_str, crate_path))
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
                            <#field_ty as #crate_path::de::BorrowDecode<#de_lifetime>>::borrow_decode(decoder)?
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

    if bound.is_none() {
        // Auto-generate BorrowDecode bounds.
        for param in &mut generics_with_bounds.params {
            if let syn::GenericParam::Type(type_param) = param {
                type_param
                    .bounds
                    .push(syn::parse_quote!(#crate_path::de::BorrowDecode<#de_lifetime>));
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

//! Encode derive implementation for OxiCode.
//!
//! Contains code-generation helpers for the `Encode` trait derive macro.

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, Fields, Index};

use crate::attrs::{parse_field_attrs, parse_variant_attrs, predicates_to_where_clause, TagType};

/// Generate the body of the `Encode::encode` method.
pub(crate) fn derive_encode_body(
    data: &Data,
    crate_path: &syn::Path,
    transparent: bool,
    tag_type: TagType,
) -> Result<TokenStream2, syn::Error> {
    if transparent {
        return match data {
            Data::Struct(data_struct) => derive_encode_transparent(&data_struct.fields, crate_path),
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
        Data::Struct(data_struct) => derive_encode_struct(&data_struct.fields, crate_path),
        Data::Enum(data_enum) => {
            // Pre-compute discriminants for all variants, accounting for variant-level
            // `#[oxicode(skip)]`.  A skipped variant is assigned the same discriminant
            // as its nearest non-skipped successor; if there is no successor, the skip
            // is effectively a dead variant and we fall back to its position index.
            let variants: Vec<&syn::Variant> = data_enum.variants.iter().collect();
            let n = variants.len();

            // First pass: determine the "natural" discriminant for each variant.
            // Natural = explicit `#[oxicode(variant = N)]` tag, or position index.
            let natural: Vec<u32> = variants
                .iter()
                .enumerate()
                .map(|(idx, v)| parse_variant_attrs(&v.attrs).map(|a| a.tag.unwrap_or(idx as u32)))
                .collect::<Result<_, _>>()?;

            // Second pass: for each variant, if it is marked `skip`, walk forward to
            // find the first non-skipped successor and borrow its natural discriminant.
            let mut effective: Vec<u32> = natural.clone();
            for i in 0..n {
                let attrs_i = parse_variant_attrs(&variants[i].attrs)?;
                if attrs_i.skip {
                    // Find the first non-skipped successor.
                    let successor_disc = (i + 1..n)
                        .find(|&j| {
                            parse_variant_attrs(&variants[j].attrs)
                                .map(|a| !a.skip)
                                .unwrap_or(true)
                        })
                        .map(|j| natural[j]);
                    if let Some(disc) = successor_disc {
                        effective[i] = disc;
                    }
                    // If no successor exists, keep the natural index (unreachable in
                    // valid code since the user cannot construct the skipped variant
                    // without hitting a compile error from the encode arm below).
                }
            }

            let variant_encodings: Vec<TokenStream2> = variants
                .iter()
                .enumerate()
                .map(|(idx, variant)| {
                    derive_encode_variant(effective[idx], variant, crate_path, tag_type)
                })
                .collect::<Result<_, _>>()?;

            Ok(quote! {
                match self {
                    #(#variant_encodings,)*
                }
            })
        }
        Data::Union(_) => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Encode cannot be derived for unions",
        )),
    }
}

/// Generate encode body for a `#[oxicode(transparent)]` struct (exactly one field).
fn derive_encode_transparent(
    fields: &Fields,
    _crate_path: &syn::Path,
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
            Ok(quote! {
                self.#field_name.encode(encoder)?;
                Ok(())
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
            Ok(quote! {
                self.0.encode(encoder)?;
                Ok(())
            })
        }
        Fields::Unit => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "#[oxicode(transparent)] requires exactly one field, but found 0 (unit struct)",
        )),
    }
}

/// Generate a seq_len encode block for a Vec/sequence field.
///
/// `field_expr` is the token stream expression to access the field
/// (e.g. `self.#field_name` or just `#field_name` for destructured patterns).
/// `len_ty_str` is one of "u8", "u16", "u32", "u64".
pub(crate) fn make_seq_len_encode_expr(field_expr: TokenStream2, len_ty_str: &str) -> TokenStream2 {
    let len_ty_tokens: TokenStream2 = match len_ty_str {
        "u8" => quote! { u8 },
        "u16" => quote! { u16 },
        "u32" => quote! { u32 },
        _ => quote! { u64 },
    };
    quote! {
        {
            let __seq_len = (#field_expr).len() as #len_ty_tokens;
            __seq_len.encode(encoder)?;
            for __item in (#field_expr).iter() {
                __item.encode(encoder)?;
            }
        }
    }
}

/// Generate encode statements for a struct's fields.
fn derive_encode_struct(
    fields: &Fields,
    crate_path: &syn::Path,
) -> Result<TokenStream2, syn::Error> {
    match fields {
        Fields::Named(named) => {
            let stmts: Vec<TokenStream2> = named
                .named
                .iter()
                .map(|f| {
                    let attrs = parse_field_attrs(f)?;
                    let field_name = &f.ident;
                    if attrs.bytes {
                        Ok(quote! {
                            {
                                let __bytes: &[u8] = &self.#field_name[..];
                                (__bytes.len() as u64).encode(encoder)?;
                                <_ as #crate_path::enc::write::Writer>::write(encoder.writer(), __bytes)?;
                            }
                        })
                    } else if let Some(ref len_ty_str) = attrs.seq_len {
                        Ok(make_seq_len_encode_expr(quote! { self.#field_name }, len_ty_str))
                    } else if attrs.is_skipped() {
                        Ok(quote! {})
                    } else if let Some(ref path) = attrs.with_module {
                        Ok(quote! { #path::encode(&self.#field_name, encoder)?; })
                    } else if let Some(ref path) = attrs.encode_with {
                        Ok(quote! { #path(&self.#field_name, encoder)?; })
                    } else {
                        Ok(quote! { self.#field_name.encode(encoder)?; })
                    }
                })
                .collect::<Result<_, syn::Error>>()?;
            Ok(quote! {
                #(#stmts)*
                Ok(())
            })
        }
        Fields::Unnamed(unnamed) => {
            let stmts: Vec<TokenStream2> = unnamed
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, f)| {
                    let attrs = parse_field_attrs(f)?;
                    let idx = Index::from(i);
                    if attrs.bytes {
                        Ok(quote! {
                            {
                                let __bytes: &[u8] = &self.#idx[..];
                                (__bytes.len() as u64).encode(encoder)?;
                                <_ as #crate_path::enc::write::Writer>::write(encoder.writer(), __bytes)?;
                            }
                        })
                    } else if let Some(ref len_ty_str) = attrs.seq_len {
                        Ok(make_seq_len_encode_expr(quote! { self.#idx }, len_ty_str))
                    } else if attrs.is_skipped() {
                        Ok(quote! {})
                    } else if let Some(ref path) = attrs.with_module {
                        Ok(quote! { #path::encode(&self.#idx, encoder)?; })
                    } else if let Some(ref path) = attrs.encode_with {
                        Ok(quote! { #path(&self.#idx, encoder)?; })
                    } else {
                        Ok(quote! { self.#idx.encode(encoder)?; })
                    }
                })
                .collect::<Result<_, syn::Error>>()?;
            Ok(quote! {
                #(#stmts)*
                Ok(())
            })
        }
        Fields::Unit => Ok(quote! { Ok(()) }),
    }
}

/// Generate a single match arm for encoding an enum variant.
///
/// `discriminant` is the pre-computed effective discriminant value for this
/// variant, already accounting for any variant-level `#[oxicode(skip)]`
/// attributes (skipped variants receive the same discriminant as their nearest
/// non-skipped successor).
fn derive_encode_variant(
    discriminant: u32,
    variant: &syn::Variant,
    crate_path: &syn::Path,
    tag_type: TagType,
) -> Result<TokenStream2, syn::Error> {
    let variant_name = &variant.ident;
    let discriminant_lit = proc_macro2::Literal::u32_suffixed(discriminant);

    // Generate the tag encode expression based on the requested tag width.
    let tag_encode = match tag_type {
        TagType::U8 => quote! { (#discriminant_lit as u8).encode(encoder)?; },
        TagType::U16 => quote! { (#discriminant_lit as u16).encode(encoder)?; },
        TagType::U32 => quote! { (#discriminant_lit as u32).encode(encoder)?; },
        TagType::U64 => quote! { (#discriminant_lit as u64).encode(encoder)?; },
    };

    match &variant.fields {
        Fields::Named(fields) => {
            let pattern_bindings: Vec<TokenStream2> = fields
                .named
                .iter()
                .map(|f| {
                    let attrs = parse_field_attrs(f)?;
                    let field_name = f.ident.as_ref().ok_or_else(|| {
                        syn::Error::new(proc_macro2::Span::call_site(), "named field has no ident")
                    })?;
                    if attrs.is_skipped() {
                        let underscore_name =
                            syn::Ident::new(&format!("_{}", field_name), field_name.span());
                        Ok(quote! { #field_name: #underscore_name })
                    } else {
                        Ok(quote! { #field_name })
                    }
                })
                .collect::<Result<_, syn::Error>>()?;
            let encode_stmts: Vec<TokenStream2> = fields
                .named
                .iter()
                .map(|f| {
                    let attrs = parse_field_attrs(f)?;
                    let field_name = &f.ident;
                    if attrs.bytes {
                        Ok(quote! {
                            {
                                let __bytes: &[u8] = &#field_name[..];
                                (__bytes.len() as u64).encode(encoder)?;
                                <_ as #crate_path::enc::write::Writer>::write(encoder.writer(), __bytes)?;
                            }
                        })
                    } else if let Some(ref len_ty_str) = attrs.seq_len {
                        Ok(make_seq_len_encode_expr(quote! { #field_name }, len_ty_str))
                    } else if attrs.is_skipped() {
                        Ok(quote! {})
                    } else if let Some(ref path) = attrs.with_module {
                        Ok(quote! { #path::encode(&#field_name, encoder)?; })
                    } else if let Some(ref path) = attrs.encode_with {
                        Ok(quote! { #path(&#field_name, encoder)?; })
                    } else {
                        Ok(quote! { #field_name.encode(encoder)?; })
                    }
                })
                .collect::<Result<_, syn::Error>>()?;
            Ok(quote! {
                Self::#variant_name { #(#pattern_bindings),* } => {
                    #tag_encode
                    #(#encode_stmts)*
                    Ok(())
                }
            })
        }
        Fields::Unnamed(fields) => {
            let pattern_names: Vec<syn::Ident> = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, f)| {
                    let attrs = parse_field_attrs(f)?;
                    let name = if attrs.is_skipped() {
                        format!("_f{}", i)
                    } else {
                        format!("f{}", i)
                    };
                    Ok(syn::Ident::new(&name, proc_macro2::Span::call_site()))
                })
                .collect::<Result<_, syn::Error>>()?;
            let encode_stmts: Vec<TokenStream2> = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, f)| {
                    let attrs = parse_field_attrs(f)?;
                    if attrs.bytes {
                        let field_name = &pattern_names[i];
                        Ok(quote! {
                            {
                                let __bytes: &[u8] = &#field_name[..];
                                (__bytes.len() as u64).encode(encoder)?;
                                <_ as #crate_path::enc::write::Writer>::write(encoder.writer(), __bytes)?;
                            }
                        })
                    } else if let Some(ref len_ty_str) = attrs.seq_len {
                        let field_name = &pattern_names[i];
                        Ok(make_seq_len_encode_expr(quote! { #field_name }, len_ty_str))
                    } else if attrs.is_skipped() {
                        Ok(quote! {})
                    } else if let Some(ref path) = attrs.with_module {
                        let field_name = &pattern_names[i];
                        Ok(quote! { #path::encode(&#field_name, encoder)?; })
                    } else if let Some(ref path) = attrs.encode_with {
                        let field_name = &pattern_names[i];
                        Ok(quote! { #path(&#field_name, encoder)?; })
                    } else {
                        let field_name = &pattern_names[i];
                        Ok(quote! { #field_name.encode(encoder)?; })
                    }
                })
                .collect::<Result<_, syn::Error>>()?;
            Ok(quote! {
                Self::#variant_name(#(#pattern_names),*) => {
                    #tag_encode
                    #(#encode_stmts)*
                    Ok(())
                }
            })
        }
        Fields::Unit => {
            // For unit variants, the tag_encode already ends in `?;` which returns (),
            // but we need to return Ok(()). Restructure to avoid the trailing `;` issue.
            let tag_encode_expr = match tag_type {
                TagType::U8 => quote! { (#discriminant_lit as u8).encode(encoder) },
                TagType::U16 => quote! { (#discriminant_lit as u16).encode(encoder) },
                TagType::U32 => quote! { (#discriminant_lit as u32).encode(encoder) },
                TagType::U64 => quote! { (#discriminant_lit as u64).encode(encoder) },
            };
            Ok(quote! {
                Self::#variant_name => #tag_encode_expr
            })
        }
    }
}

/// Build the `impl_generics` and `effective_where` tokens for an Encode impl.
pub(crate) fn build_encode_generics(
    generics: &syn::Generics,
    crate_path: &syn::Path,
    bound: &Option<Vec<syn::WherePredicate>>,
) -> (TokenStream2, TokenStream2) {
    let (impl_generics, _ty_generics, where_clause) = generics.split_for_impl();

    let final_where = if let Some(ref predicates) = bound {
        predicates_to_where_clause(predicates)
    } else {
        let mut generics_with_bounds = generics.clone();
        for param in &mut generics_with_bounds.params {
            if let syn::GenericParam::Type(type_param) = param {
                type_param
                    .bounds
                    .push(syn::parse_quote!(#crate_path::Encode));
            }
        }
        let (_, _, wc) = generics_with_bounds.split_for_impl();
        match wc {
            Some(wc) => quote! { #wc },
            None => quote! {},
        }
    };

    let impl_generics_tokens = if bound.is_some() {
        quote! { #impl_generics }
    } else {
        let mut generics_with_bounds = generics.clone();
        for param in &mut generics_with_bounds.params {
            if let syn::GenericParam::Type(type_param) = param {
                type_param
                    .bounds
                    .push(syn::parse_quote!(#crate_path::Encode));
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
        final_where
    };

    (impl_generics_tokens, effective_where)
}

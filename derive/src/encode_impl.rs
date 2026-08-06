//! Encode derive implementation for OxiCode.
//!
//! Contains code-generation helpers for the `Encode` trait derive macro.

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, Fields, Index};

use crate::attrs::{
    discriminant_literal, parse_field_attrs, parse_variant_attrs, predicates_to_where_clause,
    TagType,
};

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
        Data::Struct(data_struct) => derive_encode_struct(&data_struct.fields, crate_path),
        Data::Enum(data_enum) => {
            // Pre-compute discriminants for all variants, accounting for variant-level
            // `#[oxicode(skip)]`.  A skipped variant is assigned the same discriminant
            // as its nearest non-skipped successor; if there is no successor, the skip
            // is effectively a dead variant and we fall back to its position index.
            let variants: Vec<&syn::Variant> = data_enum.variants.iter().collect();
            let n = variants.len();

            // First pass: per-variant skip flag and "natural" discriminant.
            // Natural = explicit `#[oxicode(variant = N)]` tag, or position index.
            let mut skipped: Vec<bool> = Vec::with_capacity(n);
            let mut natural: Vec<u64> = Vec::with_capacity(n);
            for (idx, v) in variants.iter().enumerate() {
                let a = parse_variant_attrs(&v.attrs)?;
                skipped.push(a.skip);
                natural.push(a.tag.unwrap_or(idx as u64));
            }

            // Second pass: for each variant, if it is marked `skip`, walk forward to
            // find the first non-skipped successor and borrow its natural discriminant.
            let mut effective: Vec<u64> = natural.clone();
            for i in 0..n {
                if !skipped[i] {
                    continue;
                }
                match (i + 1..n).find(|&j| !skipped[j]) {
                    Some(j) => {
                        // Aliasing writes the successor's discriminant but the *skipped*
                        // variant's own payload; on decode that payload is read back through
                        // the successor's fields. That only round-trips without data
                        // corruption or stream desync when the two variants carry the same
                        // field shape (as every documented/tested use does: unit->unit,
                        // `T(u32)`->`U(u32)`). A shape mismatch would silently mis-decode into
                        // the successor and desynchronize everything after it, so reject it as
                        // a compile error — extending the trailing-skip error below to the
                        // shape-mismatched case. Field names never reach the positional wire,
                        // so only the field *types* are compared.
                        if variant_field_types(variants[i]) != variant_field_types(variants[j]) {
                            return Err(syn::Error::new(
                                variants[i].ident.span(),
                                format!(
                                    "#[oxicode(skip)] variant `{}` aliases the discriminant of `{}`, but their field shapes differ; encoding `{}` would decode as a corrupted `{}` and desynchronize the stream. Give the skipped variant the same field types as its successor, or remove it.",
                                    variants[i].ident,
                                    variants[j].ident,
                                    variants[i].ident,
                                    variants[j].ident
                                ),
                            ));
                        }
                        effective[i] = natural[j];
                    }
                    // A skipped variant with no non-skipped successor has no discriminant to
                    // alias onto: `Encode` would still emit an arm writing its natural index,
                    // but `Decode` filters it out, so the value would fail to round-trip. Make
                    // that a clear compile error instead of a silent latent failure.
                    None => {
                        return Err(syn::Error::new(
                            variants[i].ident.span(),
                            "#[oxicode(skip)] on a variant requires a following non-skipped variant whose discriminant it can alias; a trailing skipped variant cannot be decoded",
                        ));
                    }
                }
            }

            // Reject duplicate discriminants across *decodable* (non-skipped) variants.
            // Skipped variants intentionally alias their successor's discriminant, so they
            // are excluded. A collision between two non-skipped variants makes the second
            // unreachable — `Decode`'s `match` takes the first arm — so a value of the
            // second silently mis-decodes as the first. Surface it as a compile error
            // instead of leaving it to a confusing `unreachable_patterns` lint pointing at
            // the `#[derive]`. `n` is small, so a linear scan is fine.
            let mut seen: Vec<(u64, usize)> = Vec::with_capacity(n);
            for i in 0..n {
                if skipped[i] {
                    continue;
                }
                if let Some(&(_, first)) = seen.iter().find(|&&(d, _)| d == effective[i]) {
                    return Err(syn::Error::new(
                        variants[i].ident.span(),
                        format!(
                            "duplicate enum discriminant {}: variant `{}` collides with `{}`; each decodable variant needs a distinct discriminant (set one with #[oxicode(variant = N)])",
                            effective[i], variants[i].ident, variants[first].ident
                        ),
                    ));
                }
                seen.push((effective[i], i));
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
        Data::Union(data_union) => Err(syn::Error::new(
            data_union.union_token.span(),
            "Encode cannot be derived for unions",
        )),
    }
}

/// Token-string of each field's declared type, in declaration order.
///
/// Used to decide whether a `#[oxicode(skip)]` variant may safely alias its
/// successor's discriminant: aliasing only round-trips when both variants carry
/// the same sequence of field types. Field *names* never appear on the
/// positional binary wire, so they are deliberately excluded from the
/// comparison — only the types matter.
fn variant_field_types(variant: &syn::Variant) -> Vec<String> {
    variant
        .fields
        .iter()
        .map(|f| {
            let ty = &f.ty;
            quote! { #ty }.to_string()
        })
        .collect()
}

/// Generate encode body for a `#[oxicode(transparent)]` struct (exactly one field).
fn derive_encode_transparent(
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
            Ok(quote! {
                #crate_path::Encode::encode(&self.#field_name, encoder)?;
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
                #crate_path::Encode::encode(&self.0, encoder)?;
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
pub(crate) fn make_seq_len_encode_expr(
    field_expr: TokenStream2,
    len_ty_str: &str,
    crate_path: &syn::Path,
) -> TokenStream2 {
    let len_ty_tokens: TokenStream2 = match len_ty_str {
        "u8" => quote! { u8 },
        "u16" => quote! { u16 },
        "u32" => quote! { u32 },
        _ => quote! { u64 },
    };
    quote! {
        {
            let __seq_len = (#field_expr).len() as #len_ty_tokens;
            #crate_path::Encode::encode(&__seq_len, encoder)?;
            for __item in (#field_expr).iter() {
                #crate_path::Encode::encode(__item, encoder)?;
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
                                #crate_path::Encode::encode(&(__bytes.len() as u64), encoder)?;
                                <_ as #crate_path::enc::write::Writer>::write(encoder.writer(), __bytes)?;
                            }
                        })
                    } else if let Some(ref len_ty_str) = attrs.seq_len {
                        Ok(make_seq_len_encode_expr(quote! { self.#field_name }, len_ty_str, crate_path))
                    } else if attrs.is_skipped() {
                        Ok(quote! {})
                    } else if let Some(ref path) = attrs.with_module {
                        Ok(quote! { #path::encode(&self.#field_name, encoder)?; })
                    } else if let Some(ref path) = attrs.encode_with {
                        Ok(quote! { #path(&self.#field_name, encoder)?; })
                    } else {
                        Ok(quote! { #crate_path::Encode::encode(&self.#field_name, encoder)?; })
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
                                #crate_path::Encode::encode(&(__bytes.len() as u64), encoder)?;
                                <_ as #crate_path::enc::write::Writer>::write(encoder.writer(), __bytes)?;
                            }
                        })
                    } else if let Some(ref len_ty_str) = attrs.seq_len {
                        Ok(make_seq_len_encode_expr(quote! { self.#idx }, len_ty_str, crate_path))
                    } else if attrs.is_skipped() {
                        Ok(quote! {})
                    } else if let Some(ref path) = attrs.with_module {
                        Ok(quote! { #path::encode(&self.#idx, encoder)?; })
                    } else if let Some(ref path) = attrs.encode_with {
                        Ok(quote! { #path(&self.#idx, encoder)?; })
                    } else {
                        Ok(quote! { #crate_path::Encode::encode(&self.#idx, encoder)?; })
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
    discriminant: u64,
    variant: &syn::Variant,
    crate_path: &syn::Path,
    tag_type: TagType,
) -> Result<TokenStream2, syn::Error> {
    let variant_name = &variant.ident;
    // Emit the discriminant literal already narrowed to the configured tag width, validating the
    // fit. The literal is the exact type written to the wire, so no `as` truncation is involved.
    let discriminant_lit = discriminant_literal(discriminant, tag_type, variant.ident.span())?;

    // Generate the tag encode expression at the requested tag width.
    let tag_encode = quote! { #crate_path::Encode::encode(&#discriminant_lit, encoder)?; };

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
                                #crate_path::Encode::encode(&(__bytes.len() as u64), encoder)?;
                                <_ as #crate_path::enc::write::Writer>::write(encoder.writer(), __bytes)?;
                            }
                        })
                    } else if let Some(ref len_ty_str) = attrs.seq_len {
                        Ok(make_seq_len_encode_expr(quote! { #field_name }, len_ty_str, crate_path))
                    } else if attrs.is_skipped() {
                        Ok(quote! {})
                    } else if let Some(ref path) = attrs.with_module {
                        Ok(quote! { #path::encode(&#field_name, encoder)?; })
                    } else if let Some(ref path) = attrs.encode_with {
                        Ok(quote! { #path(&#field_name, encoder)?; })
                    } else {
                        Ok(quote! { #crate_path::Encode::encode(#field_name, encoder)?; })
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
                                #crate_path::Encode::encode(&(__bytes.len() as u64), encoder)?;
                                <_ as #crate_path::enc::write::Writer>::write(encoder.writer(), __bytes)?;
                            }
                        })
                    } else if let Some(ref len_ty_str) = attrs.seq_len {
                        let field_name = &pattern_names[i];
                        Ok(make_seq_len_encode_expr(quote! { #field_name }, len_ty_str, crate_path))
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
                        Ok(quote! { #crate_path::Encode::encode(#field_name, encoder)?; })
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
            // For unit variants we want the arm to evaluate to `Ok(())` directly, so emit the tag
            // encode as an expression (no trailing `;`). The literal already carries the configured
            // tag width, so `.encode()` writes exactly that many bytes.
            let tag_encode_expr =
                quote! { #crate_path::Encode::encode(&#discriminant_lit, encoder) };
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

//! Property-based tests for OxiCode using the compiler / programming language domain.

#![allow(
    clippy::approx_constant,
    clippy::useless_vec,
    clippy::len_zero,
    clippy::unnecessary_cast,
    clippy::redundant_closure,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::needless_borrow,
    clippy::enum_variant_names,
    clippy::upper_case_acronyms,
    clippy::inconsistent_digit_grouping,
    clippy::unit_cmp,
    clippy::assertions_on_constants,
    clippy::iter_on_single_items,
    clippy::expect_fun_call,
    clippy::redundant_pattern_matching,
    variant_size_differences,
    clippy::absurd_extreme_comparisons,
    clippy::nonminimal_bool,
    clippy::for_kv_map,
    clippy::needless_range_loop,
    clippy::single_match,
    clippy::collapsible_if,
    clippy::needless_return,
    clippy::redundant_clone,
    clippy::map_entry,
    clippy::match_single_binding,
    clippy::bool_comparison,
    clippy::derivable_impls,
    clippy::manual_range_contains,
    clippy::needless_borrows_for_generic_args,
    clippy::manual_map,
    clippy::vec_init_then_push,
    clippy::identity_op,
    clippy::manual_flatten,
    clippy::single_char_pattern,
    clippy::search_is_some,
    clippy::option_map_unit_fn,
    clippy::while_let_on_iterator,
    clippy::clone_on_copy,
    clippy::box_collection,
    clippy::redundant_field_names,
    clippy::ptr_arg,
    clippy::large_enum_variant,
    clippy::match_ref_pats,
    clippy::needless_pass_by_value,
    clippy::unused_unit,
    clippy::let_and_return,
    clippy::suspicious_else_formatting,
    clippy::manual_strip,
    clippy::match_like_matches_macro,
    clippy::from_over_into,
    clippy::wrong_self_convention,
    clippy::inherent_to_string,
    clippy::new_without_default,
    clippy::unnecessary_wraps,
    clippy::field_reassign_with_default,
    clippy::manual_find,
    clippy::unnecessary_lazy_evaluations,
    clippy::should_implement_trait,
    clippy::missing_safety_doc,
    clippy::unusual_byte_groupings,
    clippy::bool_assert_comparison,
    clippy::zero_prefixed_literal,
    clippy::await_holding_lock,
    clippy::manual_saturating_arithmetic,
    clippy::explicit_counter_loop,
    clippy::needless_lifetimes,
    clippy::single_component_path_imports,
    clippy::uninlined_format_args,
    clippy::iter_cloned_collect,
    clippy::manual_str_repeat,
    clippy::excessive_precision,
    clippy::precedence,
    clippy::unnecessary_literal_unwrap
)]
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use proptest::prelude::*;

// ── Domain types ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum TokenKind {
    Identifier,
    IntLiteral,
    FloatLiteral,
    StringLiteral,
    Operator,
    Keyword,
    Punctuation,
    Comment,
    Whitespace,
    Eof,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum PrimitiveType {
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
    Str,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Token {
    kind: TokenKind,
    lexeme: String,
    line: u32,
    column: u32,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct CompileError {
    message: String,
    file: String,
    line: u32,
    column: u32,
    code: u32,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct CompilationUnit {
    source_file: String,
    tokens: Vec<Token>,
    errors: Vec<CompileError>,
    warnings: Vec<CompileError>,
}

// ── Proptest strategies ───────────────────────────────────────────────────────

fn arb_token_kind() -> impl Strategy<Value = TokenKind> {
    prop_oneof![
        Just(TokenKind::Identifier),
        Just(TokenKind::IntLiteral),
        Just(TokenKind::FloatLiteral),
        Just(TokenKind::StringLiteral),
        Just(TokenKind::Operator),
        Just(TokenKind::Keyword),
        Just(TokenKind::Punctuation),
        Just(TokenKind::Comment),
        Just(TokenKind::Whitespace),
        Just(TokenKind::Eof),
    ]
}

fn arb_primitive_type() -> impl Strategy<Value = PrimitiveType> {
    prop_oneof![
        Just(PrimitiveType::Bool),
        Just(PrimitiveType::Int8),
        Just(PrimitiveType::Int16),
        Just(PrimitiveType::Int32),
        Just(PrimitiveType::Int64),
        Just(PrimitiveType::UInt8),
        Just(PrimitiveType::UInt16),
        Just(PrimitiveType::UInt32),
        Just(PrimitiveType::UInt64),
        Just(PrimitiveType::Float32),
        Just(PrimitiveType::Float64),
        Just(PrimitiveType::Str),
    ]
}

fn arb_token() -> impl Strategy<Value = Token> {
    (arb_token_kind(), ".*", 0u32..100_000, 0u32..10_000).prop_map(
        |(kind, lexeme, line, column)| Token {
            kind,
            lexeme,
            line,
            column,
        },
    )
}

fn arb_compile_error() -> impl Strategy<Value = CompileError> {
    (".*", ".*", 0u32..100_000, 0u32..10_000, 0u32..10_000).prop_map(
        |(message, file, line, column, code)| CompileError {
            message,
            file,
            line,
            column,
            code,
        },
    )
}

fn arb_compilation_unit() -> impl Strategy<Value = CompilationUnit> {
    (
        ".*",
        prop::collection::vec(arb_token(), 0..20),
        prop::collection::vec(arb_compile_error(), 0..5),
        prop::collection::vec(arb_compile_error(), 0..5),
    )
        .prop_map(|(source_file, tokens, errors, warnings)| CompilationUnit {
            source_file,
            tokens,
            errors,
            warnings,
        })
}

// ── Tests ─────────────────────────────────────────────────────────────────────

proptest! {
    // 1. TokenKind roundtrip
    #[test]
    fn prop_token_kind_roundtrip(kind in arb_token_kind()) {
        let bytes = encode_to_vec(&kind).expect("encode TokenKind");
        let (decoded, _): (TokenKind, _) = decode_from_slice(&bytes).expect("decode TokenKind");
        prop_assert_eq!(kind, decoded);
    }

    // 2. TokenKind consumed == bytes.len()
    #[test]
    fn prop_token_kind_consumed_equals_len(kind in arb_token_kind()) {
        let bytes = encode_to_vec(&kind).expect("encode TokenKind");
        let (_, consumed): (TokenKind, _) = decode_from_slice(&bytes).expect("decode TokenKind");
        prop_assert_eq!(consumed, bytes.len());
    }

    // 3. PrimitiveType roundtrip
    #[test]
    fn prop_primitive_type_roundtrip(pt in arb_primitive_type()) {
        let bytes = encode_to_vec(&pt).expect("encode PrimitiveType");
        let (decoded, _): (PrimitiveType, _) =
            decode_from_slice(&bytes).expect("decode PrimitiveType");
        prop_assert_eq!(pt, decoded);
    }

    // 4. PrimitiveType consumed == bytes.len()
    #[test]
    fn prop_primitive_type_consumed_equals_len(pt in arb_primitive_type()) {
        let bytes = encode_to_vec(&pt).expect("encode PrimitiveType");
        let (_, consumed): (PrimitiveType, _) =
            decode_from_slice(&bytes).expect("decode PrimitiveType");
        prop_assert_eq!(consumed, bytes.len());
    }

    // 5. All TokenKind variants encoded deterministically
    #[test]
    fn prop_token_kind_deterministic(kind in arb_token_kind()) {
        let bytes1 = encode_to_vec(&kind).expect("encode TokenKind first");
        let bytes2 = encode_to_vec(&kind).expect("encode TokenKind second");
        prop_assert_eq!(bytes1, bytes2);
    }

    // 6. All PrimitiveType variants encoded deterministically
    #[test]
    fn prop_primitive_type_deterministic(pt in arb_primitive_type()) {
        let bytes1 = encode_to_vec(&pt).expect("encode PrimitiveType first");
        let bytes2 = encode_to_vec(&pt).expect("encode PrimitiveType second");
        prop_assert_eq!(bytes1, bytes2);
    }

    // 7. Token roundtrip
    #[test]
    fn prop_token_roundtrip(token in arb_token()) {
        let bytes = encode_to_vec(&token).expect("encode Token");
        let (decoded, _): (Token, _) = decode_from_slice(&bytes).expect("decode Token");
        prop_assert_eq!(token, decoded);
    }

    // 8. Token consumed == bytes.len()
    #[test]
    fn prop_token_consumed_equals_len(token in arb_token()) {
        let bytes = encode_to_vec(&token).expect("encode Token");
        let (_, consumed): (Token, _) = decode_from_slice(&bytes).expect("decode Token");
        prop_assert_eq!(consumed, bytes.len());
    }

    // 9. Token deterministic encoding
    #[test]
    fn prop_token_deterministic(token in arb_token()) {
        let bytes1 = encode_to_vec(&token).expect("encode Token first");
        let bytes2 = encode_to_vec(&token).expect("encode Token second");
        prop_assert_eq!(bytes1, bytes2);
    }

    // 10. CompileError roundtrip
    #[test]
    fn prop_compile_error_roundtrip(err in arb_compile_error()) {
        let bytes = encode_to_vec(&err).expect("encode CompileError");
        let (decoded, _): (CompileError, _) =
            decode_from_slice(&bytes).expect("decode CompileError");
        prop_assert_eq!(err, decoded);
    }

    // 11. CompileError consumed == bytes.len()
    #[test]
    fn prop_compile_error_consumed_equals_len(err in arb_compile_error()) {
        let bytes = encode_to_vec(&err).expect("encode CompileError");
        let (_, consumed): (CompileError, _) =
            decode_from_slice(&bytes).expect("decode CompileError");
        prop_assert_eq!(consumed, bytes.len());
    }

    // 12. CompileError deterministic encoding
    #[test]
    fn prop_compile_error_deterministic(err in arb_compile_error()) {
        let bytes1 = encode_to_vec(&err).expect("encode CompileError first");
        let bytes2 = encode_to_vec(&err).expect("encode CompileError second");
        prop_assert_eq!(bytes1, bytes2);
    }

    // 13. CompilationUnit roundtrip
    #[test]
    fn prop_compilation_unit_roundtrip(unit in arb_compilation_unit()) {
        let bytes = encode_to_vec(&unit).expect("encode CompilationUnit");
        let (decoded, _): (CompilationUnit, _) =
            decode_from_slice(&bytes).expect("decode CompilationUnit");
        prop_assert_eq!(unit, decoded);
    }

    // 14. CompilationUnit consumed == bytes.len()
    #[test]
    fn prop_compilation_unit_consumed_equals_len(unit in arb_compilation_unit()) {
        let bytes = encode_to_vec(&unit).expect("encode CompilationUnit");
        let (_, consumed): (CompilationUnit, _) =
            decode_from_slice(&bytes).expect("decode CompilationUnit");
        prop_assert_eq!(consumed, bytes.len());
    }

    // 15. CompilationUnit deterministic encoding
    #[test]
    fn prop_compilation_unit_deterministic(unit in arb_compilation_unit()) {
        let bytes1 = encode_to_vec(&unit).expect("encode CompilationUnit first");
        let bytes2 = encode_to_vec(&unit).expect("encode CompilationUnit second");
        prop_assert_eq!(bytes1, bytes2);
    }

    // 16. Vec<Token> roundtrip
    #[test]
    fn prop_vec_tokens_roundtrip(tokens in prop::collection::vec(arb_token(), 0..30)) {
        let bytes = encode_to_vec(&tokens).expect("encode Vec<Token>");
        let (decoded, _): (Vec<Token>, _) =
            decode_from_slice(&bytes).expect("decode Vec<Token>");
        prop_assert_eq!(tokens, decoded);
    }

    // 17. Vec<Token> consumed == bytes.len()
    #[test]
    fn prop_vec_tokens_consumed_equals_len(tokens in prop::collection::vec(arb_token(), 0..30)) {
        let bytes = encode_to_vec(&tokens).expect("encode Vec<Token>");
        let (_, consumed): (Vec<Token>, _) =
            decode_from_slice(&bytes).expect("decode Vec<Token>");
        prop_assert_eq!(consumed, bytes.len());
    }

    // 18. Empty CompilationUnit roundtrip
    #[test]
    fn prop_empty_compilation_unit_roundtrip(source_file in ".*") {
        let unit = CompilationUnit {
            source_file,
            tokens: Vec::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
        };
        let bytes = encode_to_vec(&unit).expect("encode empty CompilationUnit");
        let (decoded, _): (CompilationUnit, _) =
            decode_from_slice(&bytes).expect("decode empty CompilationUnit");
        prop_assert_eq!(unit, decoded);
    }

    // 19. Large token list preserves order after roundtrip
    #[test]
    fn prop_large_token_list_order_preserved(
        tokens in prop::collection::vec(arb_token(), 50..200)
    ) {
        let bytes = encode_to_vec(&tokens).expect("encode large Vec<Token>");
        let (decoded, _): (Vec<Token>, _) =
            decode_from_slice(&bytes).expect("decode large Vec<Token>");
        prop_assert_eq!(tokens.len(), decoded.len());
        for (orig, dec) in tokens.iter().zip(decoded.iter()) {
            prop_assert_eq!(orig, dec);
        }
    }

    // 20. CompilationUnit with many errors roundtrip
    #[test]
    fn prop_compilation_unit_many_errors(
        errors in prop::collection::vec(arb_compile_error(), 10..50),
        source_file in ".*"
    ) {
        let unit = CompilationUnit {
            source_file,
            tokens: Vec::new(),
            errors: errors.clone(),
            warnings: Vec::new(),
        };
        let bytes = encode_to_vec(&unit).expect("encode CompilationUnit many errors");
        let (decoded, _): (CompilationUnit, _) =
            decode_from_slice(&bytes).expect("decode CompilationUnit many errors");
        prop_assert_eq!(unit, decoded);
    }

    // 21. Token lexeme survives roundtrip unchanged
    #[test]
    fn prop_token_lexeme_preserved(lexeme in ".*", line in 0u32..100_000, col in 0u32..10_000) {
        let token = Token {
            kind: TokenKind::Identifier,
            lexeme: lexeme.clone(),
            line,
            column: col,
        };
        let bytes = encode_to_vec(&token).expect("encode Token lexeme");
        let (decoded, _): (Token, _) = decode_from_slice(&bytes).expect("decode Token lexeme");
        prop_assert_eq!(decoded.lexeme, lexeme);
    }

    // 22. CompileError code survives roundtrip unchanged
    #[test]
    fn prop_compile_error_code_preserved(
        message in ".*",
        file in ".*",
        line in 0u32..100_000,
        column in 0u32..10_000,
        code in 0u32..10_000
    ) {
        let err = CompileError {
            message,
            file,
            line,
            column,
            code,
        };
        let bytes = encode_to_vec(&err).expect("encode CompileError code");
        let (decoded, _): (CompileError, _) =
            decode_from_slice(&bytes).expect("decode CompileError code");
        prop_assert_eq!(decoded.code, err.code);
    }
}

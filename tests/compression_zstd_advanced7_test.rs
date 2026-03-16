//! Integration tests for Zstd compression with a Natural Language Processing domain theme.
//!
//! Tests exercise compress/decompress round-trips using NLP domain types:
//! tokens, documents, corpora, and various language codes.

#![cfg(feature = "compression-zstd")]
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
use oxicode::compression::{compress, decompress, Compression};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// NLP domain types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum TokenType {
    Word,
    Punctuation,
    Number,
    Symbol,
    Whitespace,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Token {
    text: String,
    token_type: TokenType,
    position: u32,
    length: u32,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum LanguageCode {
    En,
    Es,
    Fr,
    De,
    Zh,
    Ja,
    Ko,
    Ar,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Document {
    id: u64,
    language: LanguageCode,
    content: String,
    tokens: Vec<Token>,
    word_count: u32,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Corpus {
    name: String,
    documents: Vec<Document>,
    total_tokens: u64,
}

// ---------------------------------------------------------------------------
// Helper constructors
// ---------------------------------------------------------------------------

fn make_word_token(text: &str, position: u32) -> Token {
    let length = text.len() as u32;
    Token {
        text: text.to_string(),
        token_type: TokenType::Word,
        position,
        length,
    }
}

fn make_document(id: u64, language: LanguageCode, words: &[&str]) -> Document {
    let content = words.join(" ");
    let mut tokens = Vec::with_capacity(words.len());
    let mut pos = 0u32;
    for word in words {
        tokens.push(make_word_token(word, pos));
        pos += word.len() as u32 + 1;
    }
    let word_count = words.len() as u32;
    Document {
        id,
        language,
        content,
        tokens,
        word_count,
    }
}

fn make_simple_corpus(name: &str, doc_count: usize) -> Corpus {
    let words = [
        "the", "quick", "brown", "fox", "jumps", "over", "lazy", "dog",
    ];
    let mut documents = Vec::with_capacity(doc_count);
    let mut total_tokens = 0u64;
    for i in 0..doc_count {
        let doc = make_document(i as u64, LanguageCode::En, &words);
        total_tokens += doc.tokens.len() as u64;
        documents.push(doc);
    }
    Corpus {
        name: name.to_string(),
        documents,
        total_tokens,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_roundtrip_single_token() {
    let token = Token {
        text: "linguistics".to_string(),
        token_type: TokenType::Word,
        position: 0,
        length: 11,
    };
    let encoded = encode_to_vec(&token).expect("encode Token failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress Token failed");
    let decompressed = decompress(&compressed).expect("decompress Token failed");
    let (decoded, _): (Token, usize) =
        decode_from_slice(&decompressed).expect("decode Token failed");
    assert_eq!(token, decoded);
}

#[test]
fn test_zstd_roundtrip_document() {
    let doc = make_document(
        1,
        LanguageCode::En,
        &["natural", "language", "processing", "is", "fascinating"],
    );
    let encoded = encode_to_vec(&doc).expect("encode Document failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress Document failed");
    let decompressed = decompress(&compressed).expect("decompress Document failed");
    let (decoded, _): (Document, usize) =
        decode_from_slice(&decompressed).expect("decode Document failed");
    assert_eq!(doc, decoded);
}

#[test]
fn test_zstd_roundtrip_corpus() {
    let corpus = make_simple_corpus("test-corpus", 10);
    let encoded = encode_to_vec(&corpus).expect("encode Corpus failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress Corpus failed");
    let decompressed = decompress(&compressed).expect("decompress Corpus failed");
    let (decoded, _): (Corpus, usize) =
        decode_from_slice(&decompressed).expect("decode Corpus failed");
    assert_eq!(corpus, decoded);
}

#[test]
fn test_zstd_compressed_size_less_than_original_for_repetitive_corpus() {
    // A corpus with many identical documents produces highly repetitive encoded bytes.
    let corpus = make_simple_corpus("repetitive-nlp-corpus", 200);
    let encoded = encode_to_vec(&corpus).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "Zstd compressed ({} bytes) should be smaller than original ({} bytes)",
        compressed.len(),
        encoded.len()
    );
}

#[test]
fn test_zstd_empty_corpus() {
    let corpus = Corpus {
        name: "empty".to_string(),
        documents: vec![],
        total_tokens: 0,
    };
    let encoded = encode_to_vec(&corpus).expect("encode empty Corpus failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress empty Corpus failed");
    let decompressed = decompress(&compressed).expect("decompress empty Corpus failed");
    let (decoded, _): (Corpus, usize) =
        decode_from_slice(&decompressed).expect("decode empty Corpus failed");
    assert_eq!(corpus, decoded);
    assert_eq!(decoded.documents.len(), 0);
    assert_eq!(decoded.total_tokens, 0);
}

#[test]
fn test_zstd_large_document_500_tokens() {
    // Build a document with 500+ tokens covering multiple token types.
    let word_bank = [
        "transformer",
        "embedding",
        "attention",
        "encoder",
        "decoder",
        "tokenizer",
        "vocabulary",
        "perplexity",
        "fine-tuning",
        "pre-training",
    ];
    let mut tokens: Vec<Token> = Vec::with_capacity(520);
    let mut pos = 0u32;
    for i in 0..520usize {
        let word = word_bank[i % word_bank.len()];
        tokens.push(Token {
            text: word.to_string(),
            token_type: TokenType::Word,
            position: pos,
            length: word.len() as u32,
        });
        pos += word.len() as u32 + 1;
    }
    let content: String = tokens
        .iter()
        .map(|t| t.text.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    let word_count = tokens.len() as u32;
    let doc = Document {
        id: 9001,
        language: LanguageCode::En,
        content,
        tokens,
        word_count,
    };
    assert!(doc.tokens.len() >= 500, "need at least 500 tokens");
    let encoded = encode_to_vec(&doc).expect("encode large Document failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress large Document failed");
    let decompressed = decompress(&compressed).expect("decompress large Document failed");
    let (decoded, _): (Document, usize) =
        decode_from_slice(&decompressed).expect("decode large Document failed");
    assert_eq!(doc, decoded);
    assert!(decoded.tokens.len() >= 500);
}

#[test]
fn test_zstd_language_code_english() {
    let doc = make_document(10, LanguageCode::En, &["hello", "world"]);
    let encoded = encode_to_vec(&doc).expect("encode En Document failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress En failed");
    let decompressed = decompress(&compressed).expect("decompress En failed");
    let (decoded, _): (Document, usize) =
        decode_from_slice(&decompressed).expect("decode En failed");
    assert_eq!(decoded.language, LanguageCode::En);
}

#[test]
fn test_zstd_language_code_spanish() {
    let doc = make_document(20, LanguageCode::Es, &["hola", "mundo"]);
    let encoded = encode_to_vec(&doc).expect("encode Es Document failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress Es failed");
    let decompressed = decompress(&compressed).expect("decompress Es failed");
    let (decoded, _): (Document, usize) =
        decode_from_slice(&decompressed).expect("decode Es failed");
    assert_eq!(decoded.language, LanguageCode::Es);
}

#[test]
fn test_zstd_language_code_japanese() {
    let doc = make_document(30, LanguageCode::Ja, &["日本語", "自然言語処理"]);
    let encoded = encode_to_vec(&doc).expect("encode Ja Document failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress Ja failed");
    let decompressed = decompress(&compressed).expect("decompress Ja failed");
    let (decoded, _): (Document, usize) =
        decode_from_slice(&decompressed).expect("decode Ja failed");
    assert_eq!(decoded.language, LanguageCode::Ja);
    assert_eq!(decoded.content, "日本語 自然言語処理");
}

#[test]
fn test_zstd_language_code_chinese() {
    let doc = make_document(40, LanguageCode::Zh, &["自然", "语言", "处理"]);
    let encoded = encode_to_vec(&doc).expect("encode Zh Document failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress Zh failed");
    let decompressed = decompress(&compressed).expect("decompress Zh failed");
    let (decoded, _): (Document, usize) =
        decode_from_slice(&decompressed).expect("decode Zh failed");
    assert_eq!(decoded.language, LanguageCode::Zh);
}

#[test]
fn test_zstd_language_code_other() {
    let doc = make_document(
        50,
        LanguageCode::Other("sw".to_string()),
        &["habari", "dunia"],
    );
    let encoded = encode_to_vec(&doc).expect("encode Other Document failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress Other failed");
    let decompressed = decompress(&compressed).expect("decompress Other failed");
    let (decoded, _): (Document, usize) =
        decode_from_slice(&decompressed).expect("decode Other failed");
    assert_eq!(decoded.language, LanguageCode::Other("sw".to_string()));
}

#[test]
fn test_zstd_corruption_returns_error() {
    let doc = make_document(99, LanguageCode::Fr, &["bonjour", "monde"]);
    let encoded = encode_to_vec(&doc).expect("encode failed");
    let mut compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    // Corrupt some bytes in the payload area (after the 5-byte header).
    let payload_start = 5;
    if compressed.len() > payload_start + 4 {
        compressed[payload_start] ^= 0xFF;
        compressed[payload_start + 1] ^= 0xAB;
        compressed[payload_start + 2] ^= 0x55;
    }
    let result = decompress(&compressed);
    assert!(
        result.is_err(),
        "decompress should fail on corrupted Zstd payload"
    );
}

#[test]
fn test_zstd_idempotent_decompression() {
    // Decompressing the same data twice yields the same result.
    let doc = make_document(
        100,
        LanguageCode::De,
        &["maschinelles", "lernen", "verarbeitung"],
    );
    let encoded = encode_to_vec(&doc).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");

    let decompressed_1 = decompress(&compressed).expect("first decompress failed");
    let decompressed_2 = decompress(&compressed).expect("second decompress failed");
    assert_eq!(
        decompressed_1, decompressed_2,
        "decompression must be idempotent"
    );
}

#[test]
fn test_zstd_token_type_word() {
    let token = Token {
        text: "neural".to_string(),
        token_type: TokenType::Word,
        position: 0,
        length: 6,
    };
    let encoded = encode_to_vec(&token).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Token, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(decoded.token_type, TokenType::Word);
}

#[test]
fn test_zstd_token_type_punctuation() {
    let token = Token {
        text: ".".to_string(),
        token_type: TokenType::Punctuation,
        position: 10,
        length: 1,
    };
    let encoded = encode_to_vec(&token).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Token, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(decoded.token_type, TokenType::Punctuation);
}

#[test]
fn test_zstd_token_type_number() {
    let token = Token {
        text: "42".to_string(),
        token_type: TokenType::Number,
        position: 20,
        length: 2,
    };
    let encoded = encode_to_vec(&token).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Token, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(decoded.token_type, TokenType::Number);
    assert_eq!(decoded.text, "42");
}

#[test]
fn test_zstd_token_type_symbol() {
    let token = Token {
        text: "@".to_string(),
        token_type: TokenType::Symbol,
        position: 30,
        length: 1,
    };
    let encoded = encode_to_vec(&token).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Token, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(decoded.token_type, TokenType::Symbol);
}

#[test]
fn test_zstd_token_type_whitespace() {
    let token = Token {
        text: "   ".to_string(),
        token_type: TokenType::Whitespace,
        position: 40,
        length: 3,
    };
    let encoded = encode_to_vec(&token).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Token, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(decoded.token_type, TokenType::Whitespace);
}

#[test]
fn test_zstd_token_type_unknown() {
    let token = Token {
        text: "\u{FFFD}".to_string(),
        token_type: TokenType::Unknown,
        position: 50,
        length: 3,
    };
    let encoded = encode_to_vec(&token).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Token, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(decoded.token_type, TokenType::Unknown);
}

#[test]
fn test_zstd_all_token_types_in_document() {
    // A document containing every TokenType variant.
    let tokens = vec![
        Token {
            text: "NLP".to_string(),
            token_type: TokenType::Word,
            position: 0,
            length: 3,
        },
        Token {
            text: ",".to_string(),
            token_type: TokenType::Punctuation,
            position: 3,
            length: 1,
        },
        Token {
            text: "2024".to_string(),
            token_type: TokenType::Number,
            position: 5,
            length: 4,
        },
        Token {
            text: "#".to_string(),
            token_type: TokenType::Symbol,
            position: 10,
            length: 1,
        },
        Token {
            text: " ".to_string(),
            token_type: TokenType::Whitespace,
            position: 11,
            length: 1,
        },
        Token {
            text: "\u{00AD}".to_string(),
            token_type: TokenType::Unknown,
            position: 12,
            length: 2,
        },
    ];
    let doc = Document {
        id: 200,
        language: LanguageCode::En,
        content: "NLP, 2024# \u{00AD}".to_string(),
        word_count: 1,
        tokens,
    };
    let encoded = encode_to_vec(&doc).expect("encode all-token-types Document failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress all-token-types failed");
    let decompressed = decompress(&compressed).expect("decompress all-token-types failed");
    let (decoded, _): (Document, usize) =
        decode_from_slice(&decompressed).expect("decode all-token-types failed");
    assert_eq!(doc, decoded);
    let types: Vec<&TokenType> = decoded.tokens.iter().map(|t| &t.token_type).collect();
    assert!(types.contains(&&TokenType::Word));
    assert!(types.contains(&&TokenType::Punctuation));
    assert!(types.contains(&&TokenType::Number));
    assert!(types.contains(&&TokenType::Symbol));
    assert!(types.contains(&&TokenType::Whitespace));
    assert!(types.contains(&&TokenType::Unknown));
}

#[test]
fn test_zstd_multilingual_corpus_roundtrip() {
    // A corpus mixing multiple language codes.
    let documents = vec![
        make_document(1, LanguageCode::En, &["the", "model", "predicts"]),
        make_document(2, LanguageCode::Es, &["el", "modelo", "predice"]),
        make_document(3, LanguageCode::Fr, &["le", "modele", "predit"]),
        make_document(4, LanguageCode::De, &["das", "modell", "sagt"]),
        make_document(5, LanguageCode::Ar, &["النموذج", "يتنبأ"]),
        make_document(6, LanguageCode::Ko, &["모델", "예측"]),
        make_document(
            7,
            LanguageCode::Other("pt".to_string()),
            &["o", "modelo", "prediz"],
        ),
    ];
    let total_tokens: u64 = documents.iter().map(|d| d.tokens.len() as u64).sum();
    let corpus = Corpus {
        name: "multilingual-nlp".to_string(),
        documents,
        total_tokens,
    };
    let encoded = encode_to_vec(&corpus).expect("encode multilingual Corpus failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress multilingual Corpus failed");
    let decompressed = decompress(&compressed).expect("decompress multilingual Corpus failed");
    let (decoded, _): (Corpus, usize) =
        decode_from_slice(&decompressed).expect("decode multilingual Corpus failed");
    assert_eq!(corpus, decoded);
    assert_eq!(decoded.documents.len(), 7);
    assert_eq!(decoded.total_tokens, total_tokens);
}

#[test]
fn test_zstd_empty_document_tokens() {
    // A document with no tokens (e.g., an empty string after tokenization).
    let doc = Document {
        id: 300,
        language: LanguageCode::En,
        content: String::new(),
        tokens: vec![],
        word_count: 0,
    };
    let encoded = encode_to_vec(&doc).expect("encode empty-token Document failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress empty-token Document failed");
    let decompressed = decompress(&compressed).expect("decompress empty-token Document failed");
    let (decoded, _): (Document, usize) =
        decode_from_slice(&decompressed).expect("decode empty-token Document failed");
    assert_eq!(doc, decoded);
    assert_eq!(decoded.tokens.len(), 0);
    assert_eq!(decoded.word_count, 0);
}

#[test]
fn test_zstd_corpus_total_tokens_preserved() {
    // Verify that total_tokens field survives the compression round-trip exactly.
    let corpus = make_simple_corpus("token-count-check", 50);
    let expected_total = corpus.total_tokens;
    let encoded = encode_to_vec(&corpus).expect("encode Corpus failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress Corpus failed");
    let decompressed = decompress(&compressed).expect("decompress Corpus failed");
    let (decoded, _): (Corpus, usize) =
        decode_from_slice(&decompressed).expect("decode Corpus failed");
    assert_eq!(decoded.total_tokens, expected_total);
}

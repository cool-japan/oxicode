#![cfg(feature = "compression-lz4")]
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

#[derive(Debug, PartialEq, Encode, Decode)]
enum Sentiment {
    VeryNegative,
    Negative,
    Neutral,
    Positive,
    VeryPositive,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum EntityType {
    Person,
    Organization,
    Location,
    Date,
    Money,
    Percent,
    Other,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct NamedEntity {
    text: String,
    entity_type: EntityType,
    start_char: u32,
    end_char: u32,
    confidence_pct: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TextDocument {
    doc_id: u64,
    content: String,
    sentiment: Sentiment,
    entities: Vec<NamedEntity>,
    word_count: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WordEmbedding {
    word: String,
    vector: Vec<i16>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DocumentCorpus {
    corpus_id: u64,
    documents: Vec<TextDocument>,
    language: String,
}

// Test 1: Sentiment::VeryNegative compress/decompress roundtrip
#[test]
fn test_sentiment_very_negative_compress_decompress() {
    let sentiment = Sentiment::VeryNegative;
    let encoded = encode_to_vec(&sentiment).expect("encode VeryNegative");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress VeryNegative");
    let decompressed = decompress(&compressed).expect("decompress VeryNegative");
    let decoded: Sentiment = decode_from_slice(&decompressed)
        .expect("decode VeryNegative")
        .0;
    assert_eq!(decoded, Sentiment::VeryNegative);
}

// Test 2: Sentiment::Negative compress/decompress roundtrip
#[test]
fn test_sentiment_negative_compress_decompress() {
    let sentiment = Sentiment::Negative;
    let encoded = encode_to_vec(&sentiment).expect("encode Negative");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress Negative");
    let decompressed = decompress(&compressed).expect("decompress Negative");
    let decoded: Sentiment = decode_from_slice(&decompressed).expect("decode Negative").0;
    assert_eq!(decoded, Sentiment::Negative);
}

// Test 3: Sentiment::Neutral compress/decompress roundtrip
#[test]
fn test_sentiment_neutral_compress_decompress() {
    let sentiment = Sentiment::Neutral;
    let encoded = encode_to_vec(&sentiment).expect("encode Neutral");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress Neutral");
    let decompressed = decompress(&compressed).expect("decompress Neutral");
    let decoded: Sentiment = decode_from_slice(&decompressed).expect("decode Neutral").0;
    assert_eq!(decoded, Sentiment::Neutral);
}

// Test 4: Sentiment::Positive compress/decompress roundtrip
#[test]
fn test_sentiment_positive_compress_decompress() {
    let sentiment = Sentiment::Positive;
    let encoded = encode_to_vec(&sentiment).expect("encode Positive");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress Positive");
    let decompressed = decompress(&compressed).expect("decompress Positive");
    let decoded: Sentiment = decode_from_slice(&decompressed).expect("decode Positive").0;
    assert_eq!(decoded, Sentiment::Positive);
}

// Test 5: Sentiment::VeryPositive compress/decompress roundtrip
#[test]
fn test_sentiment_very_positive_compress_decompress() {
    let sentiment = Sentiment::VeryPositive;
    let encoded = encode_to_vec(&sentiment).expect("encode VeryPositive");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress VeryPositive");
    let decompressed = decompress(&compressed).expect("decompress VeryPositive");
    let decoded: Sentiment = decode_from_slice(&decompressed)
        .expect("decode VeryPositive")
        .0;
    assert_eq!(decoded, Sentiment::VeryPositive);
}

// Test 6: EntityType variants compress/decompress roundtrip
#[test]
fn test_entity_type_all_variants_compress_decompress() {
    let variants = vec![
        EntityType::Person,
        EntityType::Organization,
        EntityType::Location,
        EntityType::Date,
        EntityType::Money,
        EntityType::Percent,
        EntityType::Other,
    ];
    for variant in variants {
        let encoded = encode_to_vec(&variant).expect("encode EntityType variant");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress EntityType variant");
        let decompressed = decompress(&compressed).expect("decompress EntityType variant");
        let decoded: EntityType = decode_from_slice(&decompressed)
            .expect("decode EntityType variant")
            .0;
        assert_eq!(decoded, variant);
    }
}

// Test 7: NamedEntity roundtrip via compress
#[test]
fn test_named_entity_compress_decompress_roundtrip() {
    let entity = NamedEntity {
        text: "Apple Inc.".to_string(),
        entity_type: EntityType::Organization,
        start_char: 10,
        end_char: 20,
        confidence_pct: 97,
    };
    let encoded = encode_to_vec(&entity).expect("encode NamedEntity");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress NamedEntity");
    let decompressed = decompress(&compressed).expect("decompress NamedEntity");
    let decoded: NamedEntity = decode_from_slice(&decompressed)
        .expect("decode NamedEntity")
        .0;
    assert_eq!(decoded, entity);
}

// Test 8: TextDocument with entities compress/decompress
#[test]
fn test_text_document_with_entities_compress_decompress() {
    let doc = TextDocument {
        doc_id: 42,
        content: "Apple Inc. was founded by Steve Jobs in Cupertino, California.".to_string(),
        sentiment: Sentiment::Positive,
        entities: vec![
            NamedEntity {
                text: "Apple Inc.".to_string(),
                entity_type: EntityType::Organization,
                start_char: 0,
                end_char: 10,
                confidence_pct: 99,
            },
            NamedEntity {
                text: "Steve Jobs".to_string(),
                entity_type: EntityType::Person,
                start_char: 26,
                end_char: 36,
                confidence_pct: 98,
            },
            NamedEntity {
                text: "Cupertino".to_string(),
                entity_type: EntityType::Location,
                start_char: 40,
                end_char: 49,
                confidence_pct: 95,
            },
        ],
        word_count: 11,
    };
    let encoded = encode_to_vec(&doc).expect("encode TextDocument");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress TextDocument");
    let decompressed = decompress(&compressed).expect("decompress TextDocument");
    let decoded: TextDocument = decode_from_slice(&decompressed)
        .expect("decode TextDocument")
        .0;
    assert_eq!(decoded, doc);
}

// Test 9: DocumentCorpus compress/decompress
#[test]
fn test_document_corpus_compress_decompress() {
    let corpus = DocumentCorpus {
        corpus_id: 1001,
        documents: vec![
            TextDocument {
                doc_id: 1,
                content: "The market showed strong growth this quarter.".to_string(),
                sentiment: Sentiment::Positive,
                entities: vec![],
                word_count: 8,
            },
            TextDocument {
                doc_id: 2,
                content: "Losses continued to mount despite cost-cutting measures.".to_string(),
                sentiment: Sentiment::Negative,
                entities: vec![],
                word_count: 9,
            },
        ],
        language: "en".to_string(),
    };
    let encoded = encode_to_vec(&corpus).expect("encode DocumentCorpus");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress DocumentCorpus");
    let decompressed = decompress(&compressed).expect("decompress DocumentCorpus");
    let decoded: DocumentCorpus = decode_from_slice(&decompressed)
        .expect("decode DocumentCorpus")
        .0;
    assert_eq!(decoded, corpus);
}

// Test 10: Large corpus (100 documents) — compressed size <= raw size
#[test]
fn test_large_corpus_compression_ratio() {
    let documents: Vec<TextDocument> = (0..100)
        .map(|i| TextDocument {
            doc_id: i as u64,
            content: format!(
                "This is document number {}. It contains repeated phrases for NLP analysis. \
                 The quick brown fox jumps over the lazy dog. Natural language processing \
                 involves tokenization, parsing, and semantic understanding.",
                i
            ),
            sentiment: Sentiment::Neutral,
            entities: vec![NamedEntity {
                text: "NLP".to_string(),
                entity_type: EntityType::Other,
                start_char: 50,
                end_char: 53,
                confidence_pct: 90,
            }],
            word_count: 35,
        })
        .collect();
    let corpus = DocumentCorpus {
        corpus_id: 9999,
        documents,
        language: "en".to_string(),
    };
    let encoded = encode_to_vec(&corpus).expect("encode large corpus");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress large corpus");
    assert!(
        compressed.len() <= encoded.len(),
        "Expected compressed ({} bytes) <= raw ({} bytes)",
        compressed.len(),
        encoded.len()
    );
}

// Test 11: Repetitive content compresses smaller than raw bytes
#[test]
fn test_repetitive_content_compresses_smaller() {
    let repeated_phrase =
        "natural language processing tokenization sentiment analysis named entity recognition "
            .repeat(50);
    let documents: Vec<TextDocument> = (0..20)
        .map(|i| TextDocument {
            doc_id: i as u64,
            content: repeated_phrase.clone(),
            sentiment: Sentiment::Neutral,
            entities: vec![],
            word_count: 800,
        })
        .collect();
    let corpus = DocumentCorpus {
        corpus_id: 777,
        documents,
        language: "en".to_string(),
    };
    let encoded = encode_to_vec(&corpus).expect("encode repetitive corpus");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress repetitive corpus");
    assert!(
        compressed.len() < encoded.len(),
        "Repetitive data should compress: compressed={} raw={}",
        compressed.len(),
        encoded.len()
    );
}

// Test 12: Empty document corpus compress/decompress
#[test]
fn test_empty_document_corpus_compress_decompress() {
    let corpus = DocumentCorpus {
        corpus_id: 0,
        documents: vec![],
        language: "en".to_string(),
    };
    let encoded = encode_to_vec(&corpus).expect("encode empty corpus");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress empty corpus");
    let decompressed = decompress(&compressed).expect("decompress empty corpus");
    let decoded: DocumentCorpus = decode_from_slice(&decompressed)
        .expect("decode empty corpus")
        .0;
    assert_eq!(decoded, corpus);
}

// Test 13: Vec<NamedEntity> compress/decompress
#[test]
fn test_vec_named_entity_compress_decompress() {
    let entities: Vec<NamedEntity> = vec![
        NamedEntity {
            text: "Barack Obama".to_string(),
            entity_type: EntityType::Person,
            start_char: 0,
            end_char: 12,
            confidence_pct: 99,
        },
        NamedEntity {
            text: "United Nations".to_string(),
            entity_type: EntityType::Organization,
            start_char: 20,
            end_char: 34,
            confidence_pct: 97,
        },
        NamedEntity {
            text: "New York".to_string(),
            entity_type: EntityType::Location,
            start_char: 40,
            end_char: 48,
            confidence_pct: 96,
        },
        NamedEntity {
            text: "January 2024".to_string(),
            entity_type: EntityType::Date,
            start_char: 55,
            end_char: 67,
            confidence_pct: 88,
        },
        NamedEntity {
            text: "$5 billion".to_string(),
            entity_type: EntityType::Money,
            start_char: 70,
            end_char: 80,
            confidence_pct: 94,
        },
    ];
    let encoded = encode_to_vec(&entities).expect("encode Vec<NamedEntity>");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress Vec<NamedEntity>");
    let decompressed = decompress(&compressed).expect("decompress Vec<NamedEntity>");
    let decoded: Vec<NamedEntity> = decode_from_slice(&decompressed)
        .expect("decode Vec<NamedEntity>")
        .0;
    assert_eq!(decoded, entities);
}

// Test 14: WordEmbedding with 300-dimensional vector
#[test]
fn test_word_embedding_300_dim_compress_decompress() {
    let vector: Vec<i16> = (0..300).map(|i| (i as i16 % 200) - 100).collect();
    let embedding = WordEmbedding {
        word: "transformer".to_string(),
        vector,
    };
    let encoded = encode_to_vec(&embedding).expect("encode WordEmbedding 300-dim");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress WordEmbedding 300-dim");
    let decompressed = decompress(&compressed).expect("decompress WordEmbedding 300-dim");
    let decoded: WordEmbedding = decode_from_slice(&decompressed)
        .expect("decode WordEmbedding 300-dim")
        .0;
    assert_eq!(decoded, embedding);
}

// Test 15: Multiple WordEmbeddings compress/decompress
#[test]
fn test_multiple_word_embeddings_compress_decompress() {
    let words = [
        "king",
        "queen",
        "man",
        "woman",
        "neural",
        "network",
        "bert",
        "attention",
    ];
    let embeddings: Vec<WordEmbedding> = words
        .iter()
        .enumerate()
        .map(|(idx, word)| WordEmbedding {
            word: word.to_string(),
            vector: (0..128)
                .map(|i| ((i as i16 * (idx as i16 + 1)) % 500) - 250)
                .collect(),
        })
        .collect();
    let encoded = encode_to_vec(&embeddings).expect("encode multiple WordEmbeddings");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress multiple WordEmbeddings");
    let decompressed = decompress(&compressed).expect("decompress multiple WordEmbeddings");
    let decoded: Vec<WordEmbedding> = decode_from_slice(&decompressed)
        .expect("decode multiple WordEmbeddings")
        .0;
    assert_eq!(decoded, embeddings);
}

// Test 16: Decompress gives original bytes
#[test]
fn test_decompress_gives_original_bytes() {
    let doc = TextDocument {
        doc_id: 100,
        content: "Semantic similarity between word embeddings enables analogy reasoning."
            .to_string(),
        sentiment: Sentiment::Neutral,
        entities: vec![],
        word_count: 10,
    };
    let encoded = encode_to_vec(&doc).expect("encode for byte check");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress for byte check");
    let decompressed = decompress(&compressed).expect("decompress for byte check");
    assert_eq!(
        decompressed, encoded,
        "Decompressed bytes must equal original encoded bytes"
    );
}

// Test 17: Sentiment distribution across documents
#[test]
fn test_sentiment_distribution_across_documents() {
    let sentiment_variants = [
        Sentiment::VeryNegative,
        Sentiment::Negative,
        Sentiment::Neutral,
        Sentiment::Positive,
        Sentiment::VeryPositive,
    ];
    let documents: Vec<TextDocument> = sentiment_variants
        .into_iter()
        .enumerate()
        .map(|(i, sentiment)| TextDocument {
            doc_id: i as u64,
            content: format!("Document {} with specific sentiment classification.", i),
            sentiment,
            entities: vec![],
            word_count: 7,
        })
        .collect();
    let corpus = DocumentCorpus {
        corpus_id: 500,
        documents,
        language: "en".to_string(),
    };
    let encoded = encode_to_vec(&corpus).expect("encode sentiment distribution corpus");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress sentiment distribution corpus");
    let decompressed = decompress(&compressed).expect("decompress sentiment distribution corpus");
    let decoded: DocumentCorpus = decode_from_slice(&decompressed)
        .expect("decode sentiment distribution corpus")
        .0;
    assert_eq!(decoded, corpus);
    assert_eq!(decoded.documents.len(), 5);
    assert_eq!(decoded.documents[0].sentiment, Sentiment::VeryNegative);
    assert_eq!(decoded.documents[4].sentiment, Sentiment::VeryPositive);
}

// Test 18: Long document content (5000 chars) compress/decompress
#[test]
fn test_long_document_content_5000_chars() {
    let base = "The field of natural language processing has evolved dramatically with the advent of \
                transformer architectures and attention mechanisms. Models such as BERT, GPT, and T5 \
                have redefined benchmarks across tasks including question answering, summarization, \
                and named entity recognition. ";
    let content = base.repeat(20);
    assert!(
        content.len() >= 5000,
        "Content should be at least 5000 chars"
    );
    let doc = TextDocument {
        doc_id: 8888,
        content,
        sentiment: Sentiment::Positive,
        entities: vec![NamedEntity {
            text: "BERT".to_string(),
            entity_type: EntityType::Other,
            start_char: 120,
            end_char: 124,
            confidence_pct: 85,
        }],
        word_count: 900,
    };
    let encoded = encode_to_vec(&doc).expect("encode long document");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress long document");
    let decompressed = decompress(&compressed).expect("decompress long document");
    let decoded: TextDocument = decode_from_slice(&decompressed)
        .expect("decode long document")
        .0;
    assert_eq!(decoded, doc);
}

// Test 19: Entity-dense document compress/decompress
#[test]
fn test_entity_dense_document_compress_decompress() {
    let entities: Vec<NamedEntity> = (0..50)
        .map(|i| NamedEntity {
            text: format!("Entity_{}", i),
            entity_type: match i % 7 {
                0 => EntityType::Person,
                1 => EntityType::Organization,
                2 => EntityType::Location,
                3 => EntityType::Date,
                4 => EntityType::Money,
                5 => EntityType::Percent,
                _ => EntityType::Other,
            },
            start_char: i as u32 * 20,
            end_char: i as u32 * 20 + 10,
            confidence_pct: 70 + (i % 30) as u8,
        })
        .collect();
    let doc = TextDocument {
        doc_id: 7777,
        content: "An entity-dense financial report mentioning many persons, organizations, and monetary figures."
            .to_string(),
        sentiment: Sentiment::Neutral,
        entities,
        word_count: 15,
    };
    let encoded = encode_to_vec(&doc).expect("encode entity-dense document");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress entity-dense document");
    let decompressed = decompress(&compressed).expect("decompress entity-dense document");
    let decoded: TextDocument = decode_from_slice(&decompressed)
        .expect("decode entity-dense document")
        .0;
    assert_eq!(decoded.entities.len(), 50);
    assert_eq!(decoded, doc);
}

// Test 20: Multilingual corpus compress/decompress
#[test]
fn test_multilingual_corpus_compress_decompress() {
    let corpora: Vec<DocumentCorpus> = vec![
        DocumentCorpus {
            corpus_id: 1,
            documents: vec![TextDocument {
                doc_id: 1,
                content: "Natural language processing enables machines to understand human text.".to_string(),
                sentiment: Sentiment::Neutral,
                entities: vec![],
                word_count: 11,
            }],
            language: "en".to_string(),
        },
        DocumentCorpus {
            corpus_id: 2,
            documents: vec![TextDocument {
                doc_id: 2,
                content: "Le traitement du langage naturel permet aux machines de comprendre le texte humain.".to_string(),
                sentiment: Sentiment::Neutral,
                entities: vec![],
                word_count: 15,
            }],
            language: "fr".to_string(),
        },
        DocumentCorpus {
            corpus_id: 3,
            documents: vec![TextDocument {
                doc_id: 3,
                content: "Die Verarbeitung natürlicher Sprache ermöglicht Maschinen, menschlichen Text zu verstehen.".to_string(),
                sentiment: Sentiment::Neutral,
                entities: vec![],
                word_count: 13,
            }],
            language: "de".to_string(),
        },
    ];
    for corpus in &corpora {
        let encoded = encode_to_vec(corpus).expect("encode multilingual corpus");
        let compressed =
            compress(&encoded, Compression::Lz4).expect("compress multilingual corpus");
        let decompressed = decompress(&compressed).expect("decompress multilingual corpus");
        let decoded: DocumentCorpus = decode_from_slice(&decompressed)
            .expect("decode multilingual corpus")
            .0;
        assert_eq!(&decoded, corpus);
    }
}

// Test 21: Compressed data byte count is positive
#[test]
fn test_compressed_data_byte_count_positive() {
    let doc = TextDocument {
        doc_id: 999,
        content: "Machine learning models process textual data to extract meaningful patterns."
            .to_string(),
        sentiment: Sentiment::Positive,
        entities: vec![NamedEntity {
            text: "Machine learning".to_string(),
            entity_type: EntityType::Other,
            start_char: 0,
            end_char: 16,
            confidence_pct: 80,
        }],
        word_count: 12,
    };
    let encoded = encode_to_vec(&doc).expect("encode for byte count check");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress for byte count check");
    assert!(
        compressed.len() > 0,
        "Compressed data must have a positive byte count, got {}",
        compressed.len()
    );
}

// Test 22: Round-trip chain encode→compress→decompress→decode
#[test]
fn test_round_trip_chain_encode_compress_decompress_decode() {
    let corpus = DocumentCorpus {
        corpus_id: 12345,
        documents: vec![
            TextDocument {
                doc_id: 1,
                content: "Attention is all you need — the transformer architecture revolutionized NLP.".to_string(),
                sentiment: Sentiment::VeryPositive,
                entities: vec![
                    NamedEntity {
                        text: "transformer".to_string(),
                        entity_type: EntityType::Other,
                        start_char: 40,
                        end_char: 51,
                        confidence_pct: 92,
                    },
                ],
                word_count: 13,
            },
            TextDocument {
                doc_id: 2,
                content: "Pre-trained language models transfer knowledge across downstream tasks effectively.".to_string(),
                sentiment: Sentiment::Positive,
                entities: vec![],
                word_count: 11,
            },
            TextDocument {
                doc_id: 3,
                content: "Bias in training data leads to unfair model predictions and societal harm.".to_string(),
                sentiment: Sentiment::VeryNegative,
                entities: vec![],
                word_count: 13,
            },
        ],
        language: "en".to_string(),
    };
    // Step 1: encode
    let encoded = encode_to_vec(&corpus).expect("encode round-trip corpus");
    // Step 2: compress
    let compressed = compress(&encoded, Compression::Lz4).expect("compress round-trip corpus");
    // Step 3: decompress
    let decompressed = decompress(&compressed).expect("decompress round-trip corpus");
    // Step 4: decode
    let decoded: DocumentCorpus = decode_from_slice(&decompressed)
        .expect("decode round-trip corpus")
        .0;
    assert_eq!(decoded, corpus);
    assert_eq!(decoded.documents.len(), 3);
    assert_eq!(decoded.documents[0].sentiment, Sentiment::VeryPositive);
    assert_eq!(decoded.documents[2].sentiment, Sentiment::VeryNegative);
    assert_eq!(decoded.language, "en");
}

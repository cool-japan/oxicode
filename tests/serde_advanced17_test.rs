#![cfg(feature = "serde")]
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
use oxicode::config;
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct SearchQuery {
    query: String,
    filters: Vec<String>,
    max_results: u32,
    offset: u64,
    include_deleted: bool,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct SearchResult {
    total_count: u64,
    results: Vec<String>,
    next_offset: Option<u64>,
    search_time_ms: u32,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum SortOrder {
    Asc,
    Desc,
    Relevance,
    Recent { days: u32 },
}

// Test 1: Basic SearchQuery roundtrip
#[test]
fn test_search_query_basic_roundtrip() {
    let query = SearchQuery {
        query: "rust programming".to_string(),
        filters: vec!["language:rust".to_string()],
        max_results: 10,
        offset: 0,
        include_deleted: false,
    };
    let bytes = encode_to_vec(&query, config::standard()).expect("encode SearchQuery failed");
    let (decoded, _): (SearchQuery, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode SearchQuery failed");
    assert_eq!(query, decoded);
}

// Test 2: SearchQuery with empty filters
#[test]
fn test_search_query_empty_filters() {
    let query = SearchQuery {
        query: "empty filters test".to_string(),
        filters: vec![],
        max_results: 50,
        offset: 100,
        include_deleted: true,
    };
    let bytes = encode_to_vec(&query, config::standard()).expect("encode empty filters failed");
    let (decoded, _): (SearchQuery, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode empty filters failed");
    assert_eq!(query, decoded);
}

// Test 3: SearchQuery with many filters
#[test]
fn test_search_query_many_filters() {
    let query = SearchQuery {
        query: "multi-filter search".to_string(),
        filters: vec![
            "tag:important".to_string(),
            "author:alice".to_string(),
            "date:2025".to_string(),
            "status:active".to_string(),
            "category:tech".to_string(),
        ],
        max_results: 100,
        offset: 0,
        include_deleted: false,
    };
    let bytes = encode_to_vec(&query, config::standard()).expect("encode many filters failed");
    let (decoded, _): (SearchQuery, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode many filters failed");
    assert_eq!(query, decoded);
}

// Test 4: SearchQuery with max u32 value
#[test]
fn test_search_query_max_results_boundary() {
    let query = SearchQuery {
        query: "boundary test".to_string(),
        filters: vec![],
        max_results: u32::MAX,
        offset: u64::MAX,
        include_deleted: true,
    };
    let bytes =
        encode_to_vec(&query, config::standard()).expect("encode max boundary values failed");
    let (decoded, _): (SearchQuery, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode max boundary values failed");
    assert_eq!(query, decoded);
}

// Test 5: SearchResult with no results
#[test]
fn test_search_result_empty() {
    let result = SearchResult {
        total_count: 0,
        results: vec![],
        next_offset: None,
        search_time_ms: 5,
    };
    let bytes =
        encode_to_vec(&result, config::standard()).expect("encode empty SearchResult failed");
    let (decoded, _): (SearchResult, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode empty SearchResult failed");
    assert_eq!(result, decoded);
}

// Test 6: SearchResult with next_offset Some
#[test]
fn test_search_result_with_next_offset() {
    let result = SearchResult {
        total_count: 1000,
        results: vec!["result_a".to_string(), "result_b".to_string()],
        next_offset: Some(20),
        search_time_ms: 42,
    };
    let bytes =
        encode_to_vec(&result, config::standard()).expect("encode SearchResult with offset failed");
    let (decoded, _): (SearchResult, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode SearchResult with offset failed");
    assert_eq!(result, decoded);
}

// Test 7: SearchResult total_count matches results length
#[test]
fn test_search_result_count_matches_results() {
    let results_data: Vec<String> = (0..5).map(|i| format!("item_{}", i)).collect();
    let result = SearchResult {
        total_count: results_data.len() as u64,
        results: results_data.clone(),
        next_offset: None,
        search_time_ms: 12,
    };
    let bytes =
        encode_to_vec(&result, config::standard()).expect("encode count-matched result failed");
    let (decoded, _): (SearchResult, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode count-matched result failed");
    assert_eq!(decoded.results.len(), 5);
    assert_eq!(decoded.total_count, 5);
}

// Test 8: SortOrder::Asc roundtrip
#[test]
fn test_sort_order_asc() {
    let order = SortOrder::Asc;
    let bytes = encode_to_vec(&order, config::standard()).expect("encode SortOrder::Asc failed");
    let (decoded, _): (SortOrder, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode SortOrder::Asc failed");
    assert_eq!(order, decoded);
}

// Test 9: SortOrder::Desc roundtrip
#[test]
fn test_sort_order_desc() {
    let order = SortOrder::Desc;
    let bytes = encode_to_vec(&order, config::standard()).expect("encode SortOrder::Desc failed");
    let (decoded, _): (SortOrder, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode SortOrder::Desc failed");
    assert_eq!(order, decoded);
}

// Test 10: SortOrder::Relevance roundtrip
#[test]
fn test_sort_order_relevance() {
    let order = SortOrder::Relevance;
    let bytes =
        encode_to_vec(&order, config::standard()).expect("encode SortOrder::Relevance failed");
    let (decoded, _): (SortOrder, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode SortOrder::Relevance failed");
    assert_eq!(order, decoded);
}

// Test 11: SortOrder::Recent with days roundtrip
#[test]
fn test_sort_order_recent_with_days() {
    let order = SortOrder::Recent { days: 30 };
    let bytes = encode_to_vec(&order, config::standard()).expect("encode SortOrder::Recent failed");
    let (decoded, _): (SortOrder, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode SortOrder::Recent failed");
    assert_eq!(order, decoded);
    if let SortOrder::Recent { days } = decoded {
        assert_eq!(days, 30);
    } else {
        panic!("expected SortOrder::Recent");
    }
}

// Test 12: SortOrder::Recent with zero days
#[test]
fn test_sort_order_recent_zero_days() {
    let order = SortOrder::Recent { days: 0 };
    let bytes = encode_to_vec(&order, config::standard()).expect("encode Recent zero days failed");
    let (decoded, _): (SortOrder, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode Recent zero days failed");
    assert_eq!(order, decoded);
}

// Test 13: Encode size is non-zero for non-empty SearchQuery
#[test]
fn test_encode_size_non_zero() {
    let query = SearchQuery {
        query: "non-empty".to_string(),
        filters: vec!["f1".to_string()],
        max_results: 1,
        offset: 0,
        include_deleted: false,
    };
    let bytes = encode_to_vec(&query, config::standard()).expect("encode non-empty query failed");
    assert!(!bytes.is_empty(), "encoded bytes should not be empty");
}

// Test 14: Decoded usize reflects consumed bytes
#[test]
fn test_decode_consumed_bytes() {
    let query = SearchQuery {
        query: "byte consumption test".to_string(),
        filters: vec![],
        max_results: 7,
        offset: 3,
        include_deleted: true,
    };
    let bytes =
        encode_to_vec(&query, config::standard()).expect("encode for consumed bytes failed");
    let expected_len = bytes.len();
    let (_, consumed) = decode_owned_from_slice::<SearchQuery, _>(&bytes, config::standard())
        .expect("decode consumed bytes failed");
    assert_eq!(
        consumed, expected_len,
        "consumed should equal total encoded length"
    );
}

// Test 15: Vec of SortOrder roundtrip
#[test]
fn test_vec_of_sort_orders() {
    let orders = vec![
        SortOrder::Asc,
        SortOrder::Desc,
        SortOrder::Relevance,
        SortOrder::Recent { days: 7 },
        SortOrder::Recent { days: 365 },
    ];
    let bytes = encode_to_vec(&orders, config::standard()).expect("encode vec of SortOrder failed");
    let (decoded, _): (Vec<SortOrder>, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode vec of SortOrder failed");
    assert_eq!(orders, decoded);
}

// Test 16: SearchQuery with unicode query string
#[test]
fn test_search_query_unicode() {
    let query = SearchQuery {
        query: "日本語テスト 검색 テスト".to_string(),
        filters: vec!["lang:ja".to_string(), "lang:ko".to_string()],
        max_results: 25,
        offset: 0,
        include_deleted: false,
    };
    let bytes = encode_to_vec(&query, config::standard()).expect("encode unicode query failed");
    let (decoded, _): (SearchQuery, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode unicode query failed");
    assert_eq!(query.query, decoded.query);
    assert_eq!(query.filters, decoded.filters);
}

// Test 17: SearchResult with large total_count
#[test]
fn test_search_result_large_total_count() {
    let result = SearchResult {
        total_count: u64::MAX,
        results: vec!["only_one".to_string()],
        next_offset: Some(u64::MAX - 1),
        search_time_ms: u32::MAX,
    };
    let bytes =
        encode_to_vec(&result, config::standard()).expect("encode large total_count failed");
    let (decoded, _): (SearchResult, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode large total_count failed");
    assert_eq!(result, decoded);
}

// Test 18: Multiple encode/decode cycles preserve data
#[test]
fn test_multiple_encode_decode_cycles() {
    let original = SearchQuery {
        query: "multi-cycle".to_string(),
        filters: vec!["cycle:1".to_string()],
        max_results: 3,
        offset: 9,
        include_deleted: false,
    };
    let bytes1 = encode_to_vec(&original, config::standard()).expect("first encode cycle failed");
    let (round1, _): (SearchQuery, usize) =
        decode_owned_from_slice(&bytes1, config::standard()).expect("first decode cycle failed");
    let bytes2 = encode_to_vec(&round1, config::standard()).expect("second encode cycle failed");
    let (round2, _): (SearchQuery, usize) =
        decode_owned_from_slice(&bytes2, config::standard()).expect("second decode cycle failed");
    assert_eq!(original, round2);
    assert_eq!(bytes1, bytes2, "bytes should be identical across cycles");
}

// Test 19: SearchQuery include_deleted true vs false produce different bytes
#[test]
fn test_search_query_include_deleted_differs() {
    let query_true = SearchQuery {
        query: "same".to_string(),
        filters: vec![],
        max_results: 1,
        offset: 0,
        include_deleted: true,
    };
    let query_false = SearchQuery {
        query: "same".to_string(),
        filters: vec![],
        max_results: 1,
        offset: 0,
        include_deleted: false,
    };
    let bytes_true =
        encode_to_vec(&query_true, config::standard()).expect("encode include_deleted=true failed");
    let bytes_false = encode_to_vec(&query_false, config::standard())
        .expect("encode include_deleted=false failed");
    assert_ne!(
        bytes_true, bytes_false,
        "different bool values should produce different bytes"
    );
}

// Test 20: Tuple of (SearchQuery, SearchResult) roundtrip
#[test]
fn test_tuple_search_query_and_result() {
    let pair = (
        SearchQuery {
            query: "tuple test".to_string(),
            filters: vec!["paired".to_string()],
            max_results: 20,
            offset: 40,
            include_deleted: false,
        },
        SearchResult {
            total_count: 2,
            results: vec!["r1".to_string(), "r2".to_string()],
            next_offset: Some(60),
            search_time_ms: 8,
        },
    );
    let bytes = encode_to_vec(&pair, config::standard()).expect("encode tuple pair failed");
    let (decoded, _): ((SearchQuery, SearchResult), usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode tuple pair failed");
    assert_eq!(pair, decoded);
}

// Test 21: SortOrder variants are distinct in encoding
#[test]
fn test_sort_order_variants_distinct_encoding() {
    let asc_bytes = encode_to_vec(&SortOrder::Asc, config::standard())
        .expect("encode Asc for distinctness failed");
    let desc_bytes = encode_to_vec(&SortOrder::Desc, config::standard())
        .expect("encode Desc for distinctness failed");
    let relevance_bytes = encode_to_vec(&SortOrder::Relevance, config::standard())
        .expect("encode Relevance for distinctness failed");
    let recent_bytes = encode_to_vec(&SortOrder::Recent { days: 1 }, config::standard())
        .expect("encode Recent for distinctness failed");
    assert_ne!(asc_bytes, desc_bytes);
    assert_ne!(asc_bytes, relevance_bytes);
    assert_ne!(asc_bytes, recent_bytes);
    assert_ne!(desc_bytes, relevance_bytes);
    assert_ne!(desc_bytes, recent_bytes);
    assert_ne!(relevance_bytes, recent_bytes);
}

// Test 22: SearchResult next_offset None vs Some produce different bytes
#[test]
fn test_search_result_next_offset_option_differs() {
    let with_none = SearchResult {
        total_count: 10,
        results: vec!["item".to_string()],
        next_offset: None,
        search_time_ms: 1,
    };
    let with_some = SearchResult {
        total_count: 10,
        results: vec!["item".to_string()],
        next_offset: Some(10),
        search_time_ms: 1,
    };
    let none_bytes =
        encode_to_vec(&with_none, config::standard()).expect("encode None offset failed");
    let some_bytes =
        encode_to_vec(&with_some, config::standard()).expect("encode Some offset failed");
    assert_ne!(
        none_bytes, some_bytes,
        "None and Some offsets should produce different bytes"
    );

    let (decoded_none, _): (SearchResult, usize) =
        decode_owned_from_slice(&none_bytes, config::standard())
            .expect("decode None offset failed");
    let (decoded_some, _): (SearchResult, usize) =
        decode_owned_from_slice(&some_bytes, config::standard())
            .expect("decode Some offset failed");
    assert_eq!(decoded_none.next_offset, None);
    assert_eq!(decoded_some.next_offset, Some(10));
}

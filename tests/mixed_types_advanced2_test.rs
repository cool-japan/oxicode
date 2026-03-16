//! Advanced mixed-type encoding scenarios testing complex interactions between
//! structs, enums, collections, options, tuples, and nested encodings.

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
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};
// ---------------------------------------------------------------------------
// Shared type definitions
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct User {
    id: u64,
    name: String,
    email: String,
    age: u8,
    active: bool,
    score: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Event {
    Login { user_id: u64, timestamp: u64 },
    Logout { user_id: u64 },
    Message { from: u64, to: u64, content: String },
    Error(String),
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ApiResponse {
    status_code: u16,
    message: String,
    data: Vec<u8>,
    success: bool,
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn make_user(id: u64) -> User {
    User {
        id,
        name: format!("user_{}", id),
        email: format!("user{}@example.com", id),
        age: (20 + id % 50) as u8,
        active: id % 2 == 0,
        score: (id * 37) as u32,
    }
}

// ---------------------------------------------------------------------------
// Test 1: User struct roundtrip with realistic data
// ---------------------------------------------------------------------------

#[test]
fn test_user_struct_roundtrip_realistic() {
    let user = User {
        id: 1_000_001,
        name: String::from("Alice Johnson"),
        email: String::from("alice.johnson@example.com"),
        age: 34,
        active: true,
        score: 98_765,
    };
    let encoded = encode_to_vec(&user).expect("Failed to encode User");
    let (decoded, _consumed): (User, usize) =
        decode_from_slice(&encoded).expect("Failed to decode User");
    assert_eq!(user, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: User struct with all-zero / empty values roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_user_struct_zero_values_roundtrip() {
    let user = User {
        id: 0,
        name: String::new(),
        email: String::new(),
        age: 0,
        active: false,
        score: 0,
    };
    let encoded = encode_to_vec(&user).expect("Failed to encode zero User");
    let (decoded, _consumed): (User, usize) =
        decode_from_slice(&encoded).expect("Failed to decode zero User");
    assert_eq!(user, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: User struct consumed equals encoded length
// ---------------------------------------------------------------------------

#[test]
fn test_user_consumed_equals_encoded_length() {
    let user = make_user(42);
    let encoded = encode_to_vec(&user).expect("Failed to encode User");
    let (_decoded, consumed): (User, usize) =
        decode_from_slice(&encoded).expect("Failed to decode User");
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 4: Event::Login roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_event_login_roundtrip() {
    let event = Event::Login {
        user_id: 7,
        timestamp: 1_700_000_000,
    };
    let encoded = encode_to_vec(&event).expect("Failed to encode Event::Login");
    let (decoded, _consumed): (Event, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Event::Login");
    assert_eq!(event, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: Event::Logout roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_event_logout_roundtrip() {
    let event = Event::Logout { user_id: 99 };
    let encoded = encode_to_vec(&event).expect("Failed to encode Event::Logout");
    let (decoded, _consumed): (Event, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Event::Logout");
    assert_eq!(event, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: Event::Message roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_event_message_roundtrip() {
    let event = Event::Message {
        from: 1,
        to: 2,
        content: String::from("Hello, world!"),
    };
    let encoded = encode_to_vec(&event).expect("Failed to encode Event::Message");
    let (decoded, _consumed): (Event, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Event::Message");
    assert_eq!(event, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: Event::Error roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_event_error_roundtrip() {
    let event = Event::Error(String::from("Something went wrong: timeout after 30s"));
    let encoded = encode_to_vec(&event).expect("Failed to encode Event::Error");
    let (decoded, _consumed): (Event, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Event::Error");
    assert_eq!(event, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: Vec<User> roundtrip (3 users)
// ---------------------------------------------------------------------------

#[test]
fn test_vec_user_roundtrip_three_items() {
    let users: Vec<User> = vec![make_user(1), make_user(2), make_user(3)];
    let encoded = encode_to_vec(&users).expect("Failed to encode Vec<User>");
    let (decoded, _consumed): (Vec<User>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<User>");
    assert_eq!(users, decoded);
}

// ---------------------------------------------------------------------------
// Test 9: Vec<Event> mixed variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_event_mixed_variants_roundtrip() {
    let events: Vec<Event> = vec![
        Event::Login {
            user_id: 1,
            timestamp: 1_700_000_001,
        },
        Event::Message {
            from: 1,
            to: 2,
            content: String::from("hi"),
        },
        Event::Logout { user_id: 1 },
        Event::Error(String::from("oops")),
    ];
    let encoded = encode_to_vec(&events).expect("Failed to encode Vec<Event>");
    let (decoded, _consumed): (Vec<Event>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<Event>");
    assert_eq!(events, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: ApiResponse roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_api_response_roundtrip() {
    let resp = ApiResponse {
        status_code: 200,
        message: String::from("OK"),
        data: vec![1, 2, 3, 4, 5],
        success: true,
    };
    let encoded = encode_to_vec(&resp).expect("Failed to encode ApiResponse");
    let (decoded, _consumed): (ApiResponse, usize) =
        decode_from_slice(&encoded).expect("Failed to decode ApiResponse");
    assert_eq!(resp, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: Option<User> Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_user_some_roundtrip() {
    let maybe_user: Option<User> = Some(make_user(5));
    let encoded = encode_to_vec(&maybe_user).expect("Failed to encode Option<User>");
    let (decoded, _consumed): (Option<User>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Option<User>");
    assert_eq!(maybe_user, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: Option<Event> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_event_none_roundtrip() {
    let maybe_event: Option<Event> = None;
    let encoded = encode_to_vec(&maybe_event).expect("Failed to encode Option<Event> None");
    let (decoded, _consumed): (Option<Event>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Option<Event> None");
    assert_eq!(maybe_event, decoded);
}

// ---------------------------------------------------------------------------
// Test 13: User struct with fixed-int config
// ---------------------------------------------------------------------------

#[test]
fn test_user_struct_with_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let user = make_user(100);
    let encoded =
        encode_to_vec_with_config(&user, cfg).expect("Failed to encode User with fixed-int config");
    let (decoded, consumed): (User, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("Failed to decode User with fixed-int config");
    assert_eq!(user, decoded);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 14: (User, Event) tuple roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_user_event_tuple_roundtrip() {
    let pair = (
        make_user(7),
        Event::Login {
            user_id: 7,
            timestamp: 1_700_000_007,
        },
    );
    let encoded = encode_to_vec(&pair).expect("Failed to encode (User, Event) tuple");
    let (decoded, _consumed): ((User, Event), usize) =
        decode_from_slice(&encoded).expect("Failed to decode (User, Event) tuple");
    assert_eq!(pair, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: Vec<ApiResponse> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_api_response_roundtrip() {
    let responses: Vec<ApiResponse> = vec![
        ApiResponse {
            status_code: 200,
            message: String::from("OK"),
            data: vec![0xAA, 0xBB],
            success: true,
        },
        ApiResponse {
            status_code: 404,
            message: String::from("Not Found"),
            data: vec![],
            success: false,
        },
        ApiResponse {
            status_code: 500,
            message: String::from("Internal Server Error"),
            data: vec![0xFF; 8],
            success: false,
        },
    ];
    let encoded = encode_to_vec(&responses).expect("Failed to encode Vec<ApiResponse>");
    let (decoded, _consumed): (Vec<ApiResponse>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<ApiResponse>");
    assert_eq!(responses, decoded);
}

// ---------------------------------------------------------------------------
// Test 16: User encoded size > Event::Logout encoded size (more fields)
// ---------------------------------------------------------------------------

#[test]
fn test_user_encoded_larger_than_event_logout() {
    let user = make_user(1);
    let event = Event::Logout { user_id: 1 };
    let user_bytes = encode_to_vec(&user).expect("Failed to encode User");
    let event_bytes = encode_to_vec(&event).expect("Failed to encode Event::Logout");
    // User has 6 fields including strings; Logout has only 1 integer field
    assert!(user_bytes.len() > event_bytes.len());
}

// ---------------------------------------------------------------------------
// Test 17: Different Event variants produce different encoded sizes
// ---------------------------------------------------------------------------

#[test]
fn test_different_event_variants_different_sizes() {
    let logout = Event::Logout { user_id: 1 };
    let message = Event::Message {
        from: 1,
        to: 2,
        content: String::from("A long message content that is clearly larger"),
    };
    let logout_bytes = encode_to_vec(&logout).expect("Failed to encode Logout");
    let message_bytes = encode_to_vec(&message).expect("Failed to encode Message");
    assert_ne!(logout_bytes.len(), message_bytes.len());
    assert!(message_bytes.len() > logout_bytes.len());
}

// ---------------------------------------------------------------------------
// Test 18: User roundtrip preserves all fields exactly
// ---------------------------------------------------------------------------

#[test]
fn test_user_roundtrip_preserves_all_fields() {
    let user = User {
        id: 999_888_777,
        name: String::from("Bob Smith"),
        email: String::from("bob@example.org"),
        age: 45,
        active: false,
        score: 12_345,
    };
    let encoded = encode_to_vec(&user).expect("Failed to encode User");
    let (decoded, _consumed): (User, usize) =
        decode_from_slice(&encoded).expect("Failed to decode User");

    assert_eq!(decoded.id, 999_888_777);
    assert_eq!(decoded.name, "Bob Smith");
    assert_eq!(decoded.email, "bob@example.org");
    assert_eq!(decoded.age, 45);
    assert!(!decoded.active);
    assert_eq!(decoded.score, 12_345);
}

// ---------------------------------------------------------------------------
// Test 19: Event::Message with empty content roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_event_message_empty_content_roundtrip() {
    let event = Event::Message {
        from: 10,
        to: 20,
        content: String::new(),
    };
    let encoded = encode_to_vec(&event).expect("Failed to encode Event::Message empty content");
    let (decoded, consumed): (Event, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Event::Message empty content");
    assert_eq!(event, decoded);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 20: ApiResponse with empty data vec roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_api_response_empty_data_roundtrip() {
    let resp = ApiResponse {
        status_code: 204,
        message: String::from("No Content"),
        data: vec![],
        success: true,
    };
    let encoded = encode_to_vec(&resp).expect("Failed to encode ApiResponse empty data");
    let (decoded, consumed): (ApiResponse, usize) =
        decode_from_slice(&encoded).expect("Failed to decode ApiResponse empty data");
    assert_eq!(resp, decoded);
    assert!(decoded.data.is_empty());
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 21: Vec<Option<User>> with Some and None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_option_user_some_and_none_roundtrip() {
    let items: Vec<Option<User>> = vec![
        Some(make_user(10)),
        None,
        Some(make_user(20)),
        None,
        Some(make_user(30)),
    ];
    let encoded = encode_to_vec(&items).expect("Failed to encode Vec<Option<User>>");
    let (decoded, consumed): (Vec<Option<User>>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<Option<User>>");
    assert_eq!(items, decoded);
    assert_eq!(consumed, encoded.len());
    // Verify None items stayed None
    assert!(decoded[1].is_none());
    assert!(decoded[3].is_none());
}

// ---------------------------------------------------------------------------
// Test 22: Nested — User inside ApiResponse data as encoded bytes
// ---------------------------------------------------------------------------

#[test]
fn test_user_nested_in_api_response_data() {
    let inner_user = make_user(55);
    let inner_bytes = encode_to_vec(&inner_user).expect("Failed to encode inner User");

    let resp = ApiResponse {
        status_code: 200,
        message: String::from("user payload"),
        data: inner_bytes.clone(),
        success: true,
    };

    // Encode the whole ApiResponse (which contains User bytes in `data`)
    let outer_encoded = encode_to_vec(&resp).expect("Failed to encode outer ApiResponse");
    let (outer_decoded, outer_consumed): (ApiResponse, usize) =
        decode_from_slice(&outer_encoded).expect("Failed to decode outer ApiResponse");

    assert_eq!(resp, outer_decoded);
    assert_eq!(outer_consumed, outer_encoded.len());

    // Now decode the inner User from the recovered data bytes
    let (recovered_user, inner_consumed): (User, usize) =
        decode_from_slice(&outer_decoded.data).expect("Failed to decode inner User from data");

    assert_eq!(inner_user, recovered_user);
    assert_eq!(inner_consumed, inner_bytes.len());
}

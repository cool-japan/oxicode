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
#[test]
fn test_encoded_bytes_display() {
    let bytes = [0x01u8, 0x2f, 0xff];
    let display = oxicode::encoded_bytes(&bytes);
    assert_eq!(format!("{}", display), "01 2f ff");
}

#[test]
fn test_encoded_bytes_lower_hex() {
    let bytes = [0xabu8, 0xcd];
    let display = oxicode::encoded_bytes(&bytes);
    assert_eq!(format!("{:x}", display), "abcd");
}

#[test]
fn test_encoded_bytes_upper_hex() {
    let bytes = [0xabu8, 0xcd];
    let display = oxicode::encoded_bytes(&bytes);
    assert_eq!(format!("{:X}", display), "ABCD");
}

#[test]
fn test_hex_dump_basic() {
    let bytes = b"Hello, World!";
    let eb = oxicode::encoded_bytes(bytes);
    let dump = eb.hex_dump();
    assert!(
        dump.contains("Hello"),
        "ASCII sidebar should show printable chars"
    );
}

#[test]
fn test_encode_to_display() {
    let value = 42u32;
    let eb = oxicode::encode_to_display(&value).expect("encode");
    let hex = format!("{:x}", eb);
    assert!(!hex.is_empty());
    let display = format!("{}", eb);
    assert!(!display.is_empty());
    // hex_dump should always start with the zero address
    let dump = eb.hex_dump();
    assert!(dump.contains("00000000:"));
}

#[test]
fn test_buffered_io_reader_roundtrip() {
    use std::io::Cursor;
    let original = vec![1u32, 2, 3, 4, 5];
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let cursor = Cursor::new(encoded);
    let decoded: Vec<u32> =
        oxicode::decode_from_buffered_read(cursor, oxicode::config::standard()).expect("decode");
    assert_eq!(original, decoded);
}

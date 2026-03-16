//! Advanced roundtrip tests for Cell, RefCell, and OnceCell type encoding.
//!
//! These tests are distinct from the basic Cell/RefCell coverage in
//! `std_extra_types_test.rs` and explore edge cases, composite types,
//! special float values, struct derive interactions, HashMap content,
//! and structural patterns such as structs that combine Cell and RefCell fields.

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
use std::cell::{Cell, RefCell};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// 1. Cell<u32> roundtrip (distinct value from the basic test which uses 99)
// ---------------------------------------------------------------------------

#[test]
fn test_cell_u32_roundtrip_max() {
    let cell = Cell::new(u32::MAX);
    let encoded = encode_to_vec(&cell).expect("encode Cell<u32>");
    let (decoded, _): (Cell<u32>, _) = decode_from_slice(&encoded).expect("decode Cell<u32>");
    assert_eq!(cell.get(), decoded.get());
}

// ---------------------------------------------------------------------------
// 2. Cell<bool> roundtrip – both variants in separate assertions
// ---------------------------------------------------------------------------

#[test]
fn test_cell_bool_false_roundtrip() {
    let cell = Cell::new(false);
    let encoded = encode_to_vec(&cell).expect("encode Cell<bool>");
    let (decoded, _): (Cell<bool>, _) = decode_from_slice(&encoded).expect("decode Cell<bool>");
    assert_eq!(cell.get(), decoded.get());
}

// ---------------------------------------------------------------------------
// 3. Cell<i64> roundtrip with i64::MIN
// ---------------------------------------------------------------------------

#[test]
fn test_cell_i64_min_roundtrip() {
    let cell = Cell::new(i64::MIN);
    let encoded = encode_to_vec(&cell).expect("encode Cell<i64>");
    let (decoded, _): (Cell<i64>, _) = decode_from_slice(&encoded).expect("decode Cell<i64>");
    assert_eq!(cell.get(), decoded.get());
}

// ---------------------------------------------------------------------------
// 4. Cell<f32> roundtrip – NaN and infinity
// ---------------------------------------------------------------------------

#[test]
fn test_cell_f32_nan_roundtrip() {
    let cell = Cell::new(f32::NAN);
    let encoded = encode_to_vec(&cell).expect("encode Cell<f32> NaN");
    let (decoded, _): (Cell<f32>, _) = decode_from_slice(&encoded).expect("decode Cell<f32> NaN");
    // NaN != NaN by IEEE 754, so compare the bit pattern instead
    assert_eq!(cell.get().to_bits(), decoded.get().to_bits());
}

#[test]
fn test_cell_f32_infinity_roundtrip() {
    let cell = Cell::new(f32::INFINITY);
    let encoded = encode_to_vec(&cell).expect("encode Cell<f32> INFINITY");
    let (decoded, _): (Cell<f32>, _) =
        decode_from_slice(&encoded).expect("decode Cell<f32> INFINITY");
    assert_eq!(cell.get(), decoded.get());
}

// ---------------------------------------------------------------------------
// 5. Cell<Option<u32>> roundtrip – None variant
// ---------------------------------------------------------------------------

#[test]
fn test_cell_option_u32_none_roundtrip() {
    let cell: Cell<Option<u32>> = Cell::new(None);
    let encoded = encode_to_vec(&cell).expect("encode Cell<Option<u32>> None");
    let (decoded, _): (Cell<Option<u32>>, _) =
        decode_from_slice(&encoded).expect("decode Cell<Option<u32>> None");
    assert_eq!(cell.get(), decoded.get());
}

// ---------------------------------------------------------------------------
// 6. Cell<Option<u32>> roundtrip – Some variant
// ---------------------------------------------------------------------------

#[test]
fn test_cell_option_u32_some_roundtrip() {
    let cell: Cell<Option<u32>> = Cell::new(Some(12345));
    let encoded = encode_to_vec(&cell).expect("encode Cell<Option<u32>> Some");
    let (decoded, _): (Cell<Option<u32>>, _) =
        decode_from_slice(&encoded).expect("decode Cell<Option<u32>> Some");
    assert_eq!(cell.get(), decoded.get());
}

// ---------------------------------------------------------------------------
// 7. RefCell<String> roundtrip – non-ASCII content
// ---------------------------------------------------------------------------

#[test]
fn test_refcell_string_unicode_roundtrip() {
    let rc = RefCell::new("こんにちは世界 🌏".to_string());
    let encoded = encode_to_vec(&rc).expect("encode RefCell<String>");
    let (decoded, _): (RefCell<String>, _) =
        decode_from_slice(&encoded).expect("decode RefCell<String>");
    assert_eq!(*rc.borrow(), *decoded.borrow());
}

// ---------------------------------------------------------------------------
// 8. RefCell<Vec<u32>> roundtrip – large vector
// ---------------------------------------------------------------------------

#[test]
fn test_refcell_vec_u32_large_roundtrip() {
    let data: Vec<u32> = (0u32..256).collect();
    let rc = RefCell::new(data.clone());
    let encoded = encode_to_vec(&rc).expect("encode RefCell<Vec<u32>>");
    let (decoded, _): (RefCell<Vec<u32>>, _) =
        decode_from_slice(&encoded).expect("decode RefCell<Vec<u32>>");
    assert_eq!(*rc.borrow(), *decoded.borrow());
}

// ---------------------------------------------------------------------------
// 9. RefCell<i32> roundtrip – negative value
// ---------------------------------------------------------------------------

#[test]
fn test_refcell_i32_negative_roundtrip() {
    let rc = RefCell::new(-2_147_483_648_i32);
    let encoded = encode_to_vec(&rc).expect("encode RefCell<i32>");
    let (decoded, _): (RefCell<i32>, _) = decode_from_slice(&encoded).expect("decode RefCell<i32>");
    assert_eq!(*rc.borrow(), *decoded.borrow());
}

// ---------------------------------------------------------------------------
// 10. RefCell<bool> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_refcell_bool_roundtrip() {
    for val in [true, false] {
        let rc = RefCell::new(val);
        let encoded = encode_to_vec(&rc).expect("encode RefCell<bool>");
        let (decoded, _): (RefCell<bool>, _) =
            decode_from_slice(&encoded).expect("decode RefCell<bool>");
        assert_eq!(*rc.borrow(), *decoded.borrow());
    }
}

// ---------------------------------------------------------------------------
// 11. Vec<Cell<u32>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_of_cell_u32_roundtrip() {
    let cells: Vec<Cell<u32>> = vec![
        Cell::new(1),
        Cell::new(2),
        Cell::new(3),
        Cell::new(u32::MAX),
    ];
    let encoded = encode_to_vec(&cells).expect("encode Vec<Cell<u32>>");
    let (decoded, _): (Vec<Cell<u32>>, _) =
        decode_from_slice(&encoded).expect("decode Vec<Cell<u32>>");
    assert_eq!(cells.len(), decoded.len());
    for (orig, dec) in cells.iter().zip(decoded.iter()) {
        assert_eq!(orig.get(), dec.get());
    }
}

// ---------------------------------------------------------------------------
// 12. Vec<RefCell<String>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_of_refcell_string_roundtrip() {
    let items: Vec<RefCell<String>> = vec![
        RefCell::new("alpha".to_string()),
        RefCell::new("beta".to_string()),
        RefCell::new("gamma".to_string()),
    ];
    let encoded = encode_to_vec(&items).expect("encode Vec<RefCell<String>>");
    let (decoded, _): (Vec<RefCell<String>>, _) =
        decode_from_slice(&encoded).expect("decode Vec<RefCell<String>>");
    assert_eq!(items.len(), decoded.len());
    for (orig, dec) in items.iter().zip(decoded.iter()) {
        assert_eq!(*orig.borrow(), *dec.borrow());
    }
}

// ---------------------------------------------------------------------------
// 13. Cell<u32> value is preserved exactly after encode/decode
// ---------------------------------------------------------------------------

#[test]
fn test_cell_u32_value_preserved_exactly() {
    let original_value: u32 = 0xDEAD_BEEF;
    let cell = Cell::new(original_value);
    let encoded = encode_to_vec(&cell).expect("encode Cell<u32>");
    let (decoded, _): (Cell<u32>, _) = decode_from_slice(&encoded).expect("decode Cell<u32>");
    assert_eq!(
        original_value,
        decoded.get(),
        "decoded value must be bit-for-bit identical"
    );
    // Mutate decoded and confirm independence from original encoding
    decoded.set(0);
    assert_eq!(
        original_value,
        cell.get(),
        "original Cell must be unchanged"
    );
}

// ---------------------------------------------------------------------------
// 14. RefCell<Vec<u8>> with empty vec
// ---------------------------------------------------------------------------

#[test]
fn test_refcell_vec_u8_empty_roundtrip() {
    let rc: RefCell<Vec<u8>> = RefCell::new(Vec::new());
    let encoded = encode_to_vec(&rc).expect("encode RefCell<Vec<u8>> empty");
    let (decoded, _): (RefCell<Vec<u8>>, _) =
        decode_from_slice(&encoded).expect("decode RefCell<Vec<u8>> empty");
    assert!(decoded.borrow().is_empty());
}

// ---------------------------------------------------------------------------
// 15. Cell<[u8; 4]> – fixed-size array inside a Cell
// ---------------------------------------------------------------------------

#[test]
fn test_cell_fixed_array_roundtrip() {
    let cell: Cell<[u8; 4]> = Cell::new([0xDE, 0xAD, 0xBE, 0xEF]);
    let encoded = encode_to_vec(&cell).expect("encode Cell<[u8; 4]>");
    let (decoded, _): (Cell<[u8; 4]>, _) =
        decode_from_slice(&encoded).expect("decode Cell<[u8; 4]>");
    assert_eq!(cell.get(), decoded.get());
}

// ---------------------------------------------------------------------------
// 16. Option<Cell<i32>> – None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_cell_i32_none_roundtrip() {
    let opt: Option<Cell<i32>> = None;
    let encoded = encode_to_vec(&opt).expect("encode Option<Cell<i32>> None");
    let (decoded, _): (Option<Cell<i32>>, _) =
        decode_from_slice(&encoded).expect("decode Option<Cell<i32>> None");
    assert!(decoded.is_none());
}

// ---------------------------------------------------------------------------
// 17. Option<Cell<i32>> – Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_cell_i32_some_roundtrip() {
    let opt: Option<Cell<i32>> = Some(Cell::new(-42));
    let encoded = encode_to_vec(&opt).expect("encode Option<Cell<i32>> Some");
    let (decoded, _): (Option<Cell<i32>>, _) =
        decode_from_slice(&encoded).expect("decode Option<Cell<i32>> Some");
    let decoded_inner = decoded.expect("should be Some");
    assert_eq!(-42, decoded_inner.get());
}

// ---------------------------------------------------------------------------
// 18. Struct with Cell and RefCell fields (derive Encode/Decode)
// ---------------------------------------------------------------------------

#[derive(Debug, Encode, Decode)]
struct CellHolder {
    counter: Cell<u32>,
    label: RefCell<String>,
    active: Cell<bool>,
}

#[test]
fn test_struct_with_cell_and_refcell_roundtrip() {
    let holder = CellHolder {
        counter: Cell::new(42),
        label: RefCell::new("oxicode".to_string()),
        active: Cell::new(true),
    };
    let encoded = encode_to_vec(&holder).expect("encode CellHolder");
    let (decoded, _): (CellHolder, _) = decode_from_slice(&encoded).expect("decode CellHolder");
    assert_eq!(holder.counter.get(), decoded.counter.get());
    assert_eq!(*holder.label.borrow(), *decoded.label.borrow());
    assert_eq!(holder.active.get(), decoded.active.get());
}

// ---------------------------------------------------------------------------
// 19. Cell<u8> single byte encoding – verify encoded length is minimal
// ---------------------------------------------------------------------------

#[test]
fn test_cell_u8_single_byte_encoding() {
    let cell = Cell::new(255u8);
    let encoded = encode_to_vec(&cell).expect("encode Cell<u8>");
    let (decoded, _): (Cell<u8>, _) = decode_from_slice(&encoded).expect("decode Cell<u8>");
    assert_eq!(cell.get(), decoded.get());
    // A Cell<u8> should encode as its inner u8, which is 1 byte in standard config
    assert_eq!(
        encoded.len(),
        1,
        "Cell<u8> should occupy exactly 1 byte on the wire"
    );
}

// ---------------------------------------------------------------------------
// 20. RefCell<HashMap<String, u32>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_refcell_hashmap_roundtrip() {
    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("alpha".to_string(), 1);
    map.insert("beta".to_string(), 2);
    map.insert("gamma".to_string(), 3);

    let rc = RefCell::new(map);
    let encoded = encode_to_vec(&rc).expect("encode RefCell<HashMap<String, u32>>");
    let (decoded, _): (RefCell<HashMap<String, u32>>, _) =
        decode_from_slice(&encoded).expect("decode RefCell<HashMap<String, u32>>");

    let orig = rc.borrow();
    let dec = decoded.borrow();
    assert_eq!(orig.len(), dec.len());
    for (k, v) in orig.iter() {
        assert_eq!(Some(v), dec.get(k), "entry {k} must survive roundtrip");
    }
}

// ---------------------------------------------------------------------------
// 21. Cell<f64> special values – positive/negative infinity
// ---------------------------------------------------------------------------

#[test]
fn test_cell_f64_pos_infinity_roundtrip() {
    let cell = Cell::new(f64::INFINITY);
    let encoded = encode_to_vec(&cell).expect("encode Cell<f64> +inf");
    let (decoded, _): (Cell<f64>, _) = decode_from_slice(&encoded).expect("decode Cell<f64> +inf");
    assert!(decoded.get().is_infinite() && decoded.get().is_sign_positive());
}

#[test]
fn test_cell_f64_neg_infinity_roundtrip() {
    let cell = Cell::new(f64::NEG_INFINITY);
    let encoded = encode_to_vec(&cell).expect("encode Cell<f64> -inf");
    let (decoded, _): (Cell<f64>, _) = decode_from_slice(&encoded).expect("decode Cell<f64> -inf");
    assert!(decoded.get().is_infinite() && decoded.get().is_sign_negative());
}

// ---------------------------------------------------------------------------
// 22. Struct containing both Cell and RefCell at different nesting depths
// ---------------------------------------------------------------------------

#[derive(Debug, Encode, Decode)]
struct NestedCellRefCell {
    /// A Cell<u32> represents a simple interior-mutable scalar.
    id: Cell<u32>,
    /// A RefCell<Vec<Cell<u8>>> combines both wrapper types.
    bytes: RefCell<Vec<u8>>,
    /// An Option<Cell<i64>> tests optional cell encoding.
    opt_value: Option<Cell<i64>>,
}

#[test]
fn test_struct_nested_cell_refcell_roundtrip() {
    let original = NestedCellRefCell {
        id: Cell::new(0xCAFE_BABE),
        bytes: RefCell::new(vec![10, 20, 30, 40, 50]),
        opt_value: Some(Cell::new(-9_999_999_999_i64)),
    };

    let encoded = encode_to_vec(&original).expect("encode NestedCellRefCell");
    let (decoded, _): (NestedCellRefCell, _) =
        decode_from_slice(&encoded).expect("decode NestedCellRefCell");

    assert_eq!(original.id.get(), decoded.id.get());
    assert_eq!(*original.bytes.borrow(), *decoded.bytes.borrow());

    let orig_opt = original.opt_value.as_ref().expect("orig Some");
    let dec_opt = decoded.opt_value.as_ref().expect("decoded Some");
    assert_eq!(orig_opt.get(), dec_opt.get());
}

//! Supply chain / shipment tracking checksum integrity tests
//! Theme: ShipmentStatus, Shipment, Package with checksum integrity

#![cfg(feature = "checksum")]
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
use oxicode::checksum::{unwrap_with_checksum, wrap_with_checksum, HEADER_SIZE};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum ShipmentStatus {
    Created,
    InTransit,
    OutForDelivery,
    Delivered,
    Failed,
    Returned,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Package {
    weight_g: u32,
    length_mm: u16,
    width_mm: u16,
    height_mm: u16,
    fragile: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Shipment {
    tracking_id: String,
    status: ShipmentStatus,
    sender: String,
    recipient: String,
    packages: Vec<Package>,
    total_value_cents: u64,
    notes: Option<String>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_package(weight_g: u32, fragile: bool) -> Package {
    Package {
        weight_g,
        length_mm: 300,
        width_mm: 200,
        height_mm: 150,
        fragile,
    }
}

fn make_shipment(
    tracking_id: &str,
    status: ShipmentStatus,
    packages: Vec<Package>,
    notes: Option<String>,
) -> Shipment {
    Shipment {
        tracking_id: tracking_id.to_string(),
        status,
        sender: "Warehouse A".to_string(),
        recipient: "Customer B".to_string(),
        packages,
        total_value_cents: 9999,
        notes,
    }
}

// ---------------------------------------------------------------------------
// Test 1: ShipmentStatus::Created roundtrip via checksum
// ---------------------------------------------------------------------------

#[test]
fn test_status_created_roundtrip() {
    let status = ShipmentStatus::Created;
    let bytes = encode_to_vec(&status).expect("encode ShipmentStatus::Created");
    let wrapped = wrap_with_checksum(&bytes);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap ShipmentStatus::Created");
    let (decoded, _): (ShipmentStatus, _) =
        decode_from_slice(&payload).expect("decode ShipmentStatus::Created");
    assert_eq!(status, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: ShipmentStatus::InTransit roundtrip via checksum
// ---------------------------------------------------------------------------

#[test]
fn test_status_in_transit_roundtrip() {
    let status = ShipmentStatus::InTransit;
    let bytes = encode_to_vec(&status).expect("encode InTransit");
    let wrapped = wrap_with_checksum(&bytes);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap InTransit");
    let (decoded, _): (ShipmentStatus, _) = decode_from_slice(&payload).expect("decode InTransit");
    assert_eq!(status, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: ShipmentStatus::OutForDelivery roundtrip via checksum
// ---------------------------------------------------------------------------

#[test]
fn test_status_out_for_delivery_roundtrip() {
    let status = ShipmentStatus::OutForDelivery;
    let bytes = encode_to_vec(&status).expect("encode OutForDelivery");
    let wrapped = wrap_with_checksum(&bytes);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap OutForDelivery");
    let (decoded, _): (ShipmentStatus, _) =
        decode_from_slice(&payload).expect("decode OutForDelivery");
    assert_eq!(status, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: ShipmentStatus::Delivered roundtrip via checksum
// ---------------------------------------------------------------------------

#[test]
fn test_status_delivered_roundtrip() {
    let status = ShipmentStatus::Delivered;
    let bytes = encode_to_vec(&status).expect("encode Delivered");
    let wrapped = wrap_with_checksum(&bytes);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap Delivered");
    let (decoded, _): (ShipmentStatus, _) = decode_from_slice(&payload).expect("decode Delivered");
    assert_eq!(status, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: ShipmentStatus::Failed roundtrip via checksum
// ---------------------------------------------------------------------------

#[test]
fn test_status_failed_roundtrip() {
    let status = ShipmentStatus::Failed;
    let bytes = encode_to_vec(&status).expect("encode Failed");
    let wrapped = wrap_with_checksum(&bytes);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap Failed");
    let (decoded, _): (ShipmentStatus, _) = decode_from_slice(&payload).expect("decode Failed");
    assert_eq!(status, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: ShipmentStatus::Returned roundtrip via checksum
// ---------------------------------------------------------------------------

#[test]
fn test_status_returned_roundtrip() {
    let status = ShipmentStatus::Returned;
    let bytes = encode_to_vec(&status).expect("encode Returned");
    let wrapped = wrap_with_checksum(&bytes);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap Returned");
    let (decoded, _): (ShipmentStatus, _) = decode_from_slice(&payload).expect("decode Returned");
    assert_eq!(status, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: Full Shipment roundtrip with notes present
// ---------------------------------------------------------------------------

#[test]
fn test_shipment_roundtrip_with_notes() {
    let shipment = make_shipment(
        "TRK-001",
        ShipmentStatus::InTransit,
        vec![make_package(500, false)],
        Some("Handle with care".to_string()),
    );
    let bytes = encode_to_vec(&shipment).expect("encode Shipment with notes");
    let wrapped = wrap_with_checksum(&bytes);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap Shipment with notes");
    let (decoded, _): (Shipment, _) =
        decode_from_slice(&payload).expect("decode Shipment with notes");
    assert_eq!(shipment, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: Full Shipment roundtrip with notes absent (None)
// ---------------------------------------------------------------------------

#[test]
fn test_shipment_roundtrip_no_notes() {
    let shipment = make_shipment(
        "TRK-002",
        ShipmentStatus::Delivered,
        vec![make_package(1200, true)],
        None,
    );
    let bytes = encode_to_vec(&shipment).expect("encode Shipment no notes");
    let wrapped = wrap_with_checksum(&bytes);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap Shipment no notes");
    let (decoded, _): (Shipment, _) =
        decode_from_slice(&payload).expect("decode Shipment no notes");
    assert_eq!(shipment, decoded);
}

// ---------------------------------------------------------------------------
// Test 9: HEADER_SIZE constant equals 16
// ---------------------------------------------------------------------------

#[test]
fn test_header_size_is_16() {
    assert_eq!(
        HEADER_SIZE, 16,
        "HEADER_SIZE must be 16, got {}",
        HEADER_SIZE
    );
}

// ---------------------------------------------------------------------------
// Test 10: wrapped length invariant — len == HEADER_SIZE + payload len
// ---------------------------------------------------------------------------

#[test]
fn test_wrapped_length_invariant() {
    let shipment = make_shipment(
        "TRK-003",
        ShipmentStatus::Created,
        vec![make_package(250, false), make_package(800, true)],
        None,
    );
    let bytes = encode_to_vec(&shipment).expect("encode for length invariant");
    let payload_len = bytes.len();
    let wrapped = wrap_with_checksum(&bytes);
    assert_eq!(
        wrapped.len(),
        HEADER_SIZE + payload_len,
        "wrapped.len() must equal HEADER_SIZE + payload_len"
    );
}

// ---------------------------------------------------------------------------
// Test 11: corruption detection — XOR flip of payload bytes
// ---------------------------------------------------------------------------

#[test]
fn test_corruption_detection_xor_flip() {
    let shipment = make_shipment(
        "TRK-004",
        ShipmentStatus::OutForDelivery,
        vec![make_package(300, false)],
        Some("Fragile electronics".to_string()),
    );
    let bytes = encode_to_vec(&shipment).expect("encode for corruption test");
    let wrapped = wrap_with_checksum(&bytes);

    let mut corrupted = wrapped.clone();
    for b in corrupted[4..].iter_mut() {
        *b ^= 0xFF;
    }

    let result = unwrap_with_checksum(&corrupted);
    assert!(
        result.is_err(),
        "corrupted payload must be detected, but got Ok"
    );
}

// ---------------------------------------------------------------------------
// Test 12: truncation detection — slice shorter than HEADER_SIZE
// ---------------------------------------------------------------------------

#[test]
fn test_truncation_detection_below_header() {
    let bytes = encode_to_vec(&ShipmentStatus::Created).expect("encode for truncation test");
    let wrapped = wrap_with_checksum(&bytes);

    // Provide only first 8 bytes — far less than HEADER_SIZE
    let truncated = &wrapped[..8];
    let result = unwrap_with_checksum(truncated);
    assert!(
        result.is_err(),
        "slice shorter than HEADER_SIZE must be detected as error"
    );
}

// ---------------------------------------------------------------------------
// Test 13: truncation detection — exactly HEADER_SIZE bytes (payload missing)
// ---------------------------------------------------------------------------

#[test]
fn test_truncation_detection_header_only() {
    let bytes = encode_to_vec(&ShipmentStatus::InTransit).expect("encode for truncation test 2");
    let wrapped = wrap_with_checksum(&bytes);

    // Keep only the header; the payload is missing
    let header_only = &wrapped[..HEADER_SIZE];
    let result = unwrap_with_checksum(header_only);
    assert!(
        result.is_err(),
        "header-only slice must yield error when payload is expected"
    );
}

// ---------------------------------------------------------------------------
// Test 14: zero-fill corruption detection
// ---------------------------------------------------------------------------

#[test]
fn test_zero_fill_corruption_detection() {
    let shipment = make_shipment(
        "TRK-005",
        ShipmentStatus::Failed,
        vec![make_package(150, false)],
        None,
    );
    let bytes = encode_to_vec(&shipment).expect("encode for zero-fill test");
    let wrapped = wrap_with_checksum(&bytes);

    let mut zero_filled = wrapped.clone();
    // Zero-fill everything after the header
    for b in zero_filled[HEADER_SIZE..].iter_mut() {
        *b = 0x00;
    }

    let result = unwrap_with_checksum(&zero_filled);
    assert!(
        result.is_err(),
        "zero-filled payload must fail checksum verification"
    );
}

// ---------------------------------------------------------------------------
// Test 15: Vec<Package> with multiple packages roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_package_roundtrip() {
    let packages = vec![
        make_package(100, false),
        make_package(200, true),
        make_package(300, false),
        make_package(400, true),
        make_package(500, false),
    ];
    let bytes = encode_to_vec(&packages).expect("encode Vec<Package>");
    let wrapped = wrap_with_checksum(&bytes);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap Vec<Package>");
    let (decoded, _): (Vec<Package>, _) = decode_from_slice(&payload).expect("decode Vec<Package>");
    assert_eq!(packages, decoded);
    assert_eq!(decoded.len(), 5);
}

// ---------------------------------------------------------------------------
// Test 16: Option<String> notes field — Some variant integrity
// ---------------------------------------------------------------------------

#[test]
fn test_option_notes_some_integrity() {
    let notes: Option<String> = Some("Priority shipment — next day delivery".to_string());
    let bytes = encode_to_vec(&notes).expect("encode Option<String> Some");
    let wrapped = wrap_with_checksum(&bytes);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap Option<String> Some");
    let (decoded, _): (Option<String>, _) =
        decode_from_slice(&payload).expect("decode Option<String> Some");
    assert_eq!(notes, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: Option<String> notes field — None variant integrity
// ---------------------------------------------------------------------------

#[test]
fn test_option_notes_none_integrity() {
    let notes: Option<String> = None;
    let bytes = encode_to_vec(&notes).expect("encode Option<String> None");
    let wrapped = wrap_with_checksum(&bytes);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap Option<String> None");
    let (decoded, _): (Option<String>, _) =
        decode_from_slice(&payload).expect("decode Option<String> None");
    assert_eq!(notes, decoded);
}

// ---------------------------------------------------------------------------
// Test 18: all ShipmentStatus variants preserve distinct encoded bytes
// ---------------------------------------------------------------------------

#[test]
fn test_all_status_variants_distinct_encodings() {
    let variants = [
        ShipmentStatus::Created,
        ShipmentStatus::InTransit,
        ShipmentStatus::OutForDelivery,
        ShipmentStatus::Delivered,
        ShipmentStatus::Failed,
        ShipmentStatus::Returned,
    ];

    let encodings: Vec<Vec<u8>> = variants
        .iter()
        .map(|v| encode_to_vec(v).expect("encode variant"))
        .collect();

    // All encodings must be distinct
    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "variants at index {} and {} must have distinct encodings",
                i, j
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Test 19: Package with fragile=true and extreme dimensions roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_package_extreme_dimensions_roundtrip() {
    let pkg = Package {
        weight_g: u32::MAX,
        length_mm: u16::MAX,
        width_mm: u16::MAX,
        height_mm: u16::MAX,
        fragile: true,
    };
    let bytes = encode_to_vec(&pkg).expect("encode extreme Package");
    let wrapped = wrap_with_checksum(&bytes);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap extreme Package");
    let (decoded, _): (Package, _) = decode_from_slice(&payload).expect("decode extreme Package");
    assert_eq!(pkg, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: Shipment with empty packages list roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_shipment_empty_packages_roundtrip() {
    let shipment = make_shipment(
        "TRK-EMPTY",
        ShipmentStatus::Returned,
        vec![],
        Some("Return: no packages found".to_string()),
    );
    let bytes = encode_to_vec(&shipment).expect("encode empty-packages Shipment");
    let wrapped = wrap_with_checksum(&bytes);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap empty-packages Shipment");
    let (decoded, _): (Shipment, _) =
        decode_from_slice(&payload).expect("decode empty-packages Shipment");
    assert_eq!(shipment, decoded);
    assert!(decoded.packages.is_empty());
}

// ---------------------------------------------------------------------------
// Test 21: Shipment with many packages roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_shipment_many_packages_roundtrip() {
    let packages: Vec<Package> = (0u32..50)
        .map(|i| Package {
            weight_g: i * 100 + 50,
            length_mm: (i as u16 % 500) + 100,
            width_mm: (i as u16 % 300) + 50,
            height_mm: (i as u16 % 200) + 30,
            fragile: i % 3 == 0,
        })
        .collect();

    let shipment = Shipment {
        tracking_id: "TRK-BULK-2026".to_string(),
        status: ShipmentStatus::InTransit,
        sender: "Mega Warehouse".to_string(),
        recipient: "Distribution Center X".to_string(),
        packages,
        total_value_cents: 1_000_000,
        notes: None,
    };

    let bytes = encode_to_vec(&shipment).expect("encode many-packages Shipment");
    let wrapped = wrap_with_checksum(&bytes);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap many-packages Shipment");
    let (decoded, _): (Shipment, _) =
        decode_from_slice(&payload).expect("decode many-packages Shipment");
    assert_eq!(shipment, decoded);
    assert_eq!(decoded.packages.len(), 50);
}

// ---------------------------------------------------------------------------
// Test 22: single byte flip anywhere in payload range is always detected
// ---------------------------------------------------------------------------

#[test]
fn test_single_byte_flip_always_detected() {
    let shipment = make_shipment(
        "TRK-FLIP",
        ShipmentStatus::OutForDelivery,
        vec![make_package(750, true)],
        Some("Sensitive cargo".to_string()),
    );
    let bytes = encode_to_vec(&shipment).expect("encode for flip detection");
    let wrapped = wrap_with_checksum(&bytes);

    // Flip each byte in the payload region one at a time
    let payload_len = wrapped.len() - HEADER_SIZE;
    let sample_positions: Vec<usize> = (0..payload_len).step_by(1).collect();

    let mut detected_count = 0usize;
    let total = sample_positions.len();

    for offset in &sample_positions {
        let mut corrupted = wrapped.clone();
        corrupted[HEADER_SIZE + offset] ^= 0xFF;
        if unwrap_with_checksum(&corrupted).is_err() {
            detected_count += 1;
        }
    }

    // All flipped positions must be detected
    assert_eq!(
        detected_count, total,
        "all {} byte flips must be detected, only {} were",
        total, detected_count
    );
}

//! Advanced tests for large struct serialization patterns in OxiCode.

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

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct DatabaseRecord {
    id: u64,
    name: String,
    email: String,
    age: u32,
    active: bool,
    score: f64,
    tags: Vec<String>,
    metadata: Vec<u8>,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct NetworkPacket {
    sequence: u64,
    timestamp: u64,
    source_port: u16,
    dest_port: u16,
    flags: u8,
    payload: Vec<u8>,
    checksum: u32,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct GameState {
    level: u32,
    score: u64,
    player_x: f32,
    player_y: f32,
    health: u32,
    mana: u32,
    inventory: Vec<u32>,
    flags: Vec<bool>,
}

// ---------------------------------------------------------------------------
// Test 1: DatabaseRecord full roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_database_record_roundtrip() {
    let record = DatabaseRecord {
        id: 1001,
        name: String::from("Alice Smith"),
        email: String::from("alice@example.com"),
        age: 30,
        active: true,
        score: 99.5f64,
        tags: vec![String::from("admin"), String::from("user")],
        metadata: vec![0x01, 0x02, 0x03, 0x04],
    };
    let enc = encode_to_vec(&record).expect("encode DatabaseRecord");
    let (decoded, _): (DatabaseRecord, usize) =
        decode_from_slice(&enc).expect("decode DatabaseRecord");
    assert_eq!(record, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: DatabaseRecord with empty tags and metadata
// ---------------------------------------------------------------------------

#[test]
fn test_database_record_empty_tags_roundtrip() {
    let record = DatabaseRecord {
        id: 0,
        name: String::from("Bob"),
        email: String::from("bob@example.com"),
        age: 25,
        active: false,
        score: 0.0f64,
        tags: vec![],
        metadata: vec![],
    };
    let enc = encode_to_vec(&record).expect("encode DatabaseRecord empty");
    let (decoded, _): (DatabaseRecord, usize) =
        decode_from_slice(&enc).expect("decode DatabaseRecord empty");
    assert_eq!(record, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: DatabaseRecord with large metadata (500 bytes)
// ---------------------------------------------------------------------------

#[test]
fn test_database_record_large_metadata() {
    let metadata: Vec<u8> = (0u8..=255u8).cycle().take(500).collect();
    let record = DatabaseRecord {
        id: 9999,
        name: String::from("LargeMeta"),
        email: String::from("large@example.com"),
        age: 40,
        active: true,
        score: 42.0f64,
        tags: vec![String::from("tag1")],
        metadata,
    };
    let enc = encode_to_vec(&record).expect("encode large metadata");
    let (decoded, _): (DatabaseRecord, usize) =
        decode_from_slice(&enc).expect("decode large metadata");
    assert_eq!(record, decoded);
    assert_eq!(decoded.metadata.len(), 500);
}

// ---------------------------------------------------------------------------
// Test 4: DatabaseRecord with many tags (10)
// ---------------------------------------------------------------------------

#[test]
fn test_database_record_many_tags() {
    let tags: Vec<String> = (0..10).map(|i| format!("tag_{}", i)).collect();
    let record = DatabaseRecord {
        id: 42,
        name: String::from("MultiTag"),
        email: String::from("multitag@example.com"),
        age: 28,
        active: true,
        score: 75.5f64,
        tags,
        metadata: vec![0xAB, 0xCD],
    };
    let enc = encode_to_vec(&record).expect("encode many tags");
    let (decoded, _): (DatabaseRecord, usize) = decode_from_slice(&enc).expect("decode many tags");
    assert_eq!(record, decoded);
    assert_eq!(decoded.tags.len(), 10);
}

// ---------------------------------------------------------------------------
// Test 5: DatabaseRecord field preservation after roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_database_record_field_preservation() {
    let record = DatabaseRecord {
        id: 7654321,
        name: String::from("Charlie"),
        email: String::from("charlie@domain.org"),
        age: 55,
        active: false,
        score: 88.8f64,
        tags: vec![String::from("vip"), String::from("legacy")],
        metadata: vec![1, 2, 3],
    };
    let enc = encode_to_vec(&record).expect("encode field preservation");
    let (decoded, _): (DatabaseRecord, usize) =
        decode_from_slice(&enc).expect("decode field preservation");
    assert_eq!(decoded.id, 7654321);
    assert_eq!(decoded.name, "Charlie");
    assert_eq!(decoded.email, "charlie@domain.org");
    assert_eq!(decoded.age, 55);
    assert!(!decoded.active);
    assert_eq!(decoded.score, 88.8f64);
    assert_eq!(decoded.tags, vec!["vip", "legacy"]);
    assert_eq!(decoded.metadata, vec![1u8, 2, 3]);
}

// ---------------------------------------------------------------------------
// Test 6: NetworkPacket full roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_network_packet_roundtrip() {
    let packet = NetworkPacket {
        sequence: 100,
        timestamp: 1_700_000_000,
        source_port: 8080,
        dest_port: 443,
        flags: 0b0000_0110,
        payload: vec![0xDE, 0xAD, 0xBE, 0xEF],
        checksum: 0x1234_5678,
    };
    let enc = encode_to_vec(&packet).expect("encode NetworkPacket");
    let (decoded, _): (NetworkPacket, usize) =
        decode_from_slice(&enc).expect("decode NetworkPacket");
    assert_eq!(packet, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: NetworkPacket with empty payload
// ---------------------------------------------------------------------------

#[test]
fn test_network_packet_empty_payload() {
    let packet = NetworkPacket {
        sequence: 0,
        timestamp: 0,
        source_port: 1024,
        dest_port: 80,
        flags: 0x00,
        payload: vec![],
        checksum: 0,
    };
    let enc = encode_to_vec(&packet).expect("encode empty NetworkPacket");
    let (decoded, _): (NetworkPacket, usize) =
        decode_from_slice(&enc).expect("decode empty NetworkPacket");
    assert_eq!(packet, decoded);
    assert_eq!(decoded.payload.len(), 0);
}

// ---------------------------------------------------------------------------
// Test 8: NetworkPacket with large payload (1000 bytes)
// ---------------------------------------------------------------------------

#[test]
fn test_network_packet_large_payload() {
    let payload: Vec<u8> = (0u16..1000).map(|i| (i % 256) as u8).collect();
    let packet = NetworkPacket {
        sequence: u64::MAX,
        timestamp: 9_999_999_999,
        source_port: 65535,
        dest_port: 65534,
        flags: 0xFF,
        payload,
        checksum: u32::MAX,
    };
    let enc = encode_to_vec(&packet).expect("encode large NetworkPacket");
    let (decoded, _): (NetworkPacket, usize) =
        decode_from_slice(&enc).expect("decode large NetworkPacket");
    assert_eq!(packet, decoded);
    assert_eq!(decoded.payload.len(), 1000);
}

// ---------------------------------------------------------------------------
// Test 9: NetworkPacket field preservation
// ---------------------------------------------------------------------------

#[test]
fn test_network_packet_field_preservation() {
    let packet = NetworkPacket {
        sequence: 555,
        timestamp: 1_234_567_890,
        source_port: 12345,
        dest_port: 80,
        flags: 0x02,
        payload: vec![0x01, 0x02, 0x03],
        checksum: 0xDEAD_BEEF,
    };
    let enc = encode_to_vec(&packet).expect("encode NetworkPacket fields");
    let (decoded, _): (NetworkPacket, usize) =
        decode_from_slice(&enc).expect("decode NetworkPacket fields");
    assert_eq!(decoded.sequence, 555);
    assert_eq!(decoded.timestamp, 1_234_567_890);
    assert_eq!(decoded.source_port, 12345);
    assert_eq!(decoded.dest_port, 80);
    assert_eq!(decoded.flags, 0x02);
    assert_eq!(decoded.payload, vec![0x01u8, 0x02, 0x03]);
    assert_eq!(decoded.checksum, 0xDEAD_BEEF);
}

// ---------------------------------------------------------------------------
// Test 10: GameState full roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_game_state_roundtrip() {
    let state = GameState {
        level: 5,
        score: 100_000,
        player_x: 100.0f32,
        player_y: 200.0f32,
        health: 90,
        mana: 50,
        inventory: vec![1, 2, 3, 4, 5],
        flags: vec![true, false, true],
    };
    let enc = encode_to_vec(&state).expect("encode GameState");
    let (decoded, _): (GameState, usize) = decode_from_slice(&enc).expect("decode GameState");
    assert_eq!(state, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: GameState with empty inventory and flags
// ---------------------------------------------------------------------------

#[test]
fn test_game_state_empty_inventory() {
    let state = GameState {
        level: 1,
        score: 0,
        player_x: 0.0f32,
        player_y: 0.0f32,
        health: 100,
        mana: 100,
        inventory: vec![],
        flags: vec![],
    };
    let enc = encode_to_vec(&state).expect("encode empty GameState");
    let (decoded, _): (GameState, usize) = decode_from_slice(&enc).expect("decode empty GameState");
    assert_eq!(state, decoded);
    assert_eq!(decoded.inventory.len(), 0);
    assert_eq!(decoded.flags.len(), 0);
}

// ---------------------------------------------------------------------------
// Test 12: GameState field preservation
// ---------------------------------------------------------------------------

#[test]
fn test_game_state_field_preservation() {
    let state = GameState {
        level: 99,
        score: 9_999_999,
        player_x: 512.5f32,
        player_y: 768.25f32,
        health: 1,
        mana: 255,
        inventory: vec![10, 20, 30],
        flags: vec![false, true, false, true],
    };
    let enc = encode_to_vec(&state).expect("encode GameState fields");
    let (decoded, _): (GameState, usize) =
        decode_from_slice(&enc).expect("decode GameState fields");
    assert_eq!(decoded.level, 99);
    assert_eq!(decoded.score, 9_999_999);
    assert_eq!(decoded.player_x, 512.5f32);
    assert_eq!(decoded.player_y, 768.25f32);
    assert_eq!(decoded.health, 1);
    assert_eq!(decoded.mana, 255);
    assert_eq!(decoded.inventory, vec![10u32, 20, 30]);
    assert_eq!(decoded.flags, vec![false, true, false, true]);
}

// ---------------------------------------------------------------------------
// Test 13: Vec<DatabaseRecord> roundtrip with 3 records
// ---------------------------------------------------------------------------

#[test]
fn test_vec_database_record_roundtrip() {
    let records: Vec<DatabaseRecord> = vec![
        DatabaseRecord {
            id: 1,
            name: String::from("Alice"),
            email: String::from("alice@test.com"),
            age: 20,
            active: true,
            score: 10.0f64,
            tags: vec![String::from("a")],
            metadata: vec![1],
        },
        DatabaseRecord {
            id: 2,
            name: String::from("Bob"),
            email: String::from("bob@test.com"),
            age: 30,
            active: false,
            score: 20.0f64,
            tags: vec![String::from("b"), String::from("c")],
            metadata: vec![2, 3],
        },
        DatabaseRecord {
            id: 3,
            name: String::from("Carol"),
            email: String::from("carol@test.com"),
            age: 40,
            active: true,
            score: 30.5f64,
            tags: vec![],
            metadata: vec![],
        },
    ];
    let enc = encode_to_vec(&records).expect("encode Vec<DatabaseRecord>");
    let (decoded, _): (Vec<DatabaseRecord>, usize) =
        decode_from_slice(&enc).expect("decode Vec<DatabaseRecord>");
    assert_eq!(records, decoded);
    assert_eq!(decoded.len(), 3);
}

// ---------------------------------------------------------------------------
// Test 14: Vec<NetworkPacket> roundtrip with 3 packets
// ---------------------------------------------------------------------------

#[test]
fn test_vec_network_packet_roundtrip() {
    let packets: Vec<NetworkPacket> = vec![
        NetworkPacket {
            sequence: 1,
            timestamp: 1000,
            source_port: 100,
            dest_port: 200,
            flags: 0x01,
            payload: vec![0x11],
            checksum: 111,
        },
        NetworkPacket {
            sequence: 2,
            timestamp: 2000,
            source_port: 300,
            dest_port: 400,
            flags: 0x02,
            payload: vec![0x22, 0x33],
            checksum: 222,
        },
        NetworkPacket {
            sequence: 3,
            timestamp: 3000,
            source_port: 500,
            dest_port: 600,
            flags: 0x04,
            payload: vec![],
            checksum: 333,
        },
    ];
    let enc = encode_to_vec(&packets).expect("encode Vec<NetworkPacket>");
    let (decoded, _): (Vec<NetworkPacket>, usize) =
        decode_from_slice(&enc).expect("decode Vec<NetworkPacket>");
    assert_eq!(packets, decoded);
    assert_eq!(decoded.len(), 3);
}

// ---------------------------------------------------------------------------
// Test 15: Option<DatabaseRecord> Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_database_record_some_roundtrip() {
    let record = Some(DatabaseRecord {
        id: 42,
        name: String::from("Dave"),
        email: String::from("dave@example.com"),
        age: 35,
        active: true,
        score: 55.5f64,
        tags: vec![String::from("opt")],
        metadata: vec![0xFF],
    });
    let enc = encode_to_vec(&record).expect("encode Option<DatabaseRecord> Some");
    let (decoded, _): (Option<DatabaseRecord>, usize) =
        decode_from_slice(&enc).expect("decode Option<DatabaseRecord> Some");
    assert_eq!(record, decoded);
    assert!(decoded.is_some());
}

// ---------------------------------------------------------------------------
// Test 16: Option<DatabaseRecord> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_database_record_none_roundtrip() {
    let record: Option<DatabaseRecord> = None;
    let enc = encode_to_vec(&record).expect("encode Option<DatabaseRecord> None");
    let (decoded, _): (Option<DatabaseRecord>, usize) =
        decode_from_slice(&enc).expect("decode Option<DatabaseRecord> None");
    assert_eq!(record, decoded);
    assert!(decoded.is_none());
}

// ---------------------------------------------------------------------------
// Test 17: DatabaseRecord consumed bytes equals encoded length
// ---------------------------------------------------------------------------

#[test]
fn test_database_record_consumed_equals_len() {
    let record = DatabaseRecord {
        id: 12345,
        name: String::from("Eve"),
        email: String::from("eve@example.com"),
        age: 22,
        active: true,
        score: 66.6f64,
        tags: vec![String::from("x"), String::from("y"), String::from("z")],
        metadata: vec![10, 20, 30, 40, 50],
    };
    let enc = encode_to_vec(&record).expect("encode for consumed check");
    let (_, consumed): (DatabaseRecord, usize) =
        decode_from_slice(&enc).expect("decode for consumed check");
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 18: NetworkPacket consumed bytes equals encoded length
// ---------------------------------------------------------------------------

#[test]
fn test_network_packet_consumed_equals_len() {
    let packet = NetworkPacket {
        sequence: 777,
        timestamp: 8_888_888,
        source_port: 9000,
        dest_port: 9001,
        flags: 0x0F,
        payload: vec![1, 2, 3, 4, 5, 6, 7, 8],
        checksum: 99999,
    };
    let enc = encode_to_vec(&packet).expect("encode for consumed check");
    let (_, consumed): (NetworkPacket, usize) =
        decode_from_slice(&enc).expect("decode for consumed check");
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 19: DatabaseRecord with fixed_int config
// ---------------------------------------------------------------------------

#[test]
fn test_database_record_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let record = DatabaseRecord {
        id: 500,
        name: String::from("Frank"),
        email: String::from("frank@example.com"),
        age: 45,
        active: false,
        score: 12.5f64,
        tags: vec![String::from("fixed")],
        metadata: vec![0xAA, 0xBB],
    };
    let enc = encode_to_vec_with_config(&record, cfg).expect("encode fixed_int DatabaseRecord");
    let (decoded, consumed): (DatabaseRecord, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode fixed_int DatabaseRecord");
    assert_eq!(record, decoded);
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 20: Re-encoding decoded DatabaseRecord gives same bytes (stability)
// ---------------------------------------------------------------------------

#[test]
fn test_database_record_reencode_stability() {
    let record = DatabaseRecord {
        id: 8888,
        name: String::from("Grace"),
        email: String::from("grace@example.com"),
        age: 31,
        active: true,
        score: 77.7f64,
        tags: vec![String::from("stable"), String::from("test")],
        metadata: vec![0x01, 0x23, 0x45, 0x67, 0x89],
    };
    let enc1 = encode_to_vec(&record).expect("first encode");
    let (decoded, _): (DatabaseRecord, usize) =
        decode_from_slice(&enc1).expect("decode for stability");
    let enc2 = encode_to_vec(&decoded).expect("re-encode");
    assert_eq!(enc1, enc2);
}

// ---------------------------------------------------------------------------
// Test 21: Vec<DatabaseRecord> with 20 records
// ---------------------------------------------------------------------------

#[test]
fn test_large_vec_database_records() {
    let records: Vec<DatabaseRecord> = (0u64..20)
        .map(|i| DatabaseRecord {
            id: i,
            name: format!("User_{}", i),
            email: format!("user{}@example.com", i),
            age: (20 + i % 50) as u32,
            active: i % 2 == 0,
            score: (i as f64) * 1.5f64,
            tags: vec![format!("tag_{}", i % 5)],
            metadata: vec![(i % 256) as u8; (i % 10) as usize],
        })
        .collect();
    let enc = encode_to_vec(&records).expect("encode large Vec<DatabaseRecord>");
    let (decoded, consumed): (Vec<DatabaseRecord>, usize) =
        decode_from_slice(&enc).expect("decode large Vec<DatabaseRecord>");
    assert_eq!(records, decoded);
    assert_eq!(decoded.len(), 20);
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 22: GameState with 100 inventory items
// ---------------------------------------------------------------------------

#[test]
fn test_game_state_large_inventory() {
    let inventory: Vec<u32> = (0u32..100).map(|i| i * 10).collect();
    let flags: Vec<bool> = (0..20).map(|i| i % 3 == 0).collect();
    let state = GameState {
        level: 50,
        score: 1_000_000,
        player_x: 1024.0f32,
        player_y: 768.0f32,
        health: 75,
        mana: 60,
        inventory,
        flags,
    };
    let enc = encode_to_vec(&state).expect("encode large inventory GameState");
    let (decoded, consumed): (GameState, usize) =
        decode_from_slice(&enc).expect("decode large inventory GameState");
    assert_eq!(state, decoded);
    assert_eq!(decoded.inventory.len(), 100);
    assert_eq!(decoded.flags.len(), 20);
    assert_eq!(consumed, enc.len());
}

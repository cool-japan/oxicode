//! Advanced enum derive pattern tests for OxiCode.
//!
//! Covers 20 comprehensive scenarios:
//!  1.  Enum with u8 tag_type: discriminant fits in 1 byte (fixed encoding)
//!  2.  Enum with u16 tag_type: discriminant fits in 2 bytes (fixed encoding)
//!  3.  Enum with u32 tag_type: discriminant fits in 4 bytes (fixed encoding)
//!  4.  Large enum (100 variants) with u8 tag_type: discriminant 99 encodes as [99]
//!  5.  Large enum with varint tag: discriminant 300 encodes with varint (3 bytes)
//!  6.  Enum with nested enum field
//!  7.  Enum with Option<String> field
//!  8.  Enum with Vec<u32> field
//!  9.  Enum with HashMap field
//! 10.  Enum with tuple variant (many fields)
//! 11.  Enum with struct variant
//! 12.  C-like enum (no data, all unit variants) roundtrip
//! 13.  Enum where one variant is an empty tuple ()
//! 14.  Enum with BTreeMap<String, Vec<u32>> field
//! 15.  Recursive-like enum (Box<T> as field)
//! 16.  Enum discriminants are sequential (0, 1, 2...)
//! 17.  Enum with renamed variants using #[oxicode(rename = "...")]
//! 18.  Enum with skip variant: skipped variant gets next available discriminant
//! 19.  Encode/decode enum in Vec<MyEnum>
//! 20.  Encode/decode enum as map value BTreeMap<String, MyEnum>

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
use std::collections::{BTreeMap, HashMap};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn encode_fixed<T: Encode>(val: &T) -> Vec<u8> {
    oxicode::encode_to_vec_with_config(val, oxicode::config::legacy()).expect("encode_fixed failed")
}

fn decode_fixed<T: Decode>(bytes: &[u8]) -> T {
    let (val, _) = oxicode::decode_from_slice_with_config(bytes, oxicode::config::legacy())
        .expect("decode_fixed failed");
    val
}

fn roundtrip<T: Encode + Decode + PartialEq + std::fmt::Debug>(val: &T) -> T {
    let encoded = encode_to_vec(val).expect("encode failed");
    let (decoded, bytes_read) = decode_from_slice::<T>(&encoded).expect("decode failed");
    assert_eq!(bytes_read, encoded.len(), "not all bytes consumed");
    decoded
}

// ---------------------------------------------------------------------------
// Type definitions
// ---------------------------------------------------------------------------

// Test 1 — u8 tag_type
#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum TagU8 {
    First,
    Second,
    Third,
}

// Test 2 — u16 tag_type
#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u16")]
enum TagU16 {
    Alpha,
    Beta,
    Gamma,
}

// Test 3 — u32 tag_type
#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u32")]
enum TagU32 {
    Only,
    Another,
}

// Test 4 — Large enum (100 variants) with u8 tag_type.
// Macro generates V00..V99 variants.
macro_rules! define_large_u8_enum {
    ($($v:ident),* $(,)?) => {
        #[derive(Debug, PartialEq, Encode, Decode)]
        #[oxicode(tag_type = "u8")]
        enum LargeU8Enum {
            $($v,)*
        }
    };
}

define_large_u8_enum!(
    V00, V01, V02, V03, V04, V05, V06, V07, V08, V09, V10, V11, V12, V13, V14, V15, V16, V17, V18,
    V19, V20, V21, V22, V23, V24, V25, V26, V27, V28, V29, V30, V31, V32, V33, V34, V35, V36, V37,
    V38, V39, V40, V41, V42, V43, V44, V45, V46, V47, V48, V49, V50, V51, V52, V53, V54, V55, V56,
    V57, V58, V59, V60, V61, V62, V63, V64, V65, V66, V67, V68, V69, V70, V71, V72, V73, V74, V75,
    V76, V77, V78, V79, V80, V81, V82, V83, V84, V85, V86, V87, V88, V89, V90, V91, V92, V93, V94,
    V95, V96, V97, V98, V99,
);

// Test 5 — Large varint enum (>250 variants so discriminant 300 is multi-byte).
macro_rules! define_large_varint_enum {
    ($($v:ident),* $(,)?) => {
        #[derive(Debug, PartialEq, Encode, Decode)]
        enum LargeVarintEnum {
            $($v,)*
        }
    };
}

define_large_varint_enum!(
    W000, W001, W002, W003, W004, W005, W006, W007, W008, W009, W010, W011, W012, W013, W014, W015,
    W016, W017, W018, W019, W020, W021, W022, W023, W024, W025, W026, W027, W028, W029, W030, W031,
    W032, W033, W034, W035, W036, W037, W038, W039, W040, W041, W042, W043, W044, W045, W046, W047,
    W048, W049, W050, W051, W052, W053, W054, W055, W056, W057, W058, W059, W060, W061, W062, W063,
    W064, W065, W066, W067, W068, W069, W070, W071, W072, W073, W074, W075, W076, W077, W078, W079,
    W080, W081, W082, W083, W084, W085, W086, W087, W088, W089, W090, W091, W092, W093, W094, W095,
    W096, W097, W098, W099, W100, W101, W102, W103, W104, W105, W106, W107, W108, W109, W110, W111,
    W112, W113, W114, W115, W116, W117, W118, W119, W120, W121, W122, W123, W124, W125, W126, W127,
    W128, W129, W130, W131, W132, W133, W134, W135, W136, W137, W138, W139, W140, W141, W142, W143,
    W144, W145, W146, W147, W148, W149, W150, W151, W152, W153, W154, W155, W156, W157, W158, W159,
    W160, W161, W162, W163, W164, W165, W166, W167, W168, W169, W170, W171, W172, W173, W174, W175,
    W176, W177, W178, W179, W180, W181, W182, W183, W184, W185, W186, W187, W188, W189, W190, W191,
    W192, W193, W194, W195, W196, W197, W198, W199, W200, W201, W202, W203, W204, W205, W206, W207,
    W208, W209, W210, W211, W212, W213, W214, W215, W216, W217, W218, W219, W220, W221, W222, W223,
    W224, W225, W226, W227, W228, W229, W230, W231, W232, W233, W234, W235, W236, W237, W238, W239,
    W240, W241, W242, W243, W244, W245, W246, W247, W248, W249, W250, W251, W252, W253, W254, W255,
    W256, W257, W258, W259, W260, W261, W262, W263, W264, W265, W266, W267, W268, W269, W270, W271,
    W272, W273, W274, W275, W276, W277, W278, W279, W280, W281, W282, W283, W284, W285, W286, W287,
    W288, W289, W290, W291, W292, W293, W294, W295, W296, W297, W298, W299, W300,
);

// Test 6 — Nested enum field
#[derive(Debug, PartialEq, Encode, Decode)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Move {
    Step { dir: Direction, distance: u32 },
    Jump(Direction),
    Stop,
}

// Test 7 — Option<String> field
#[derive(Debug, PartialEq, Encode, Decode)]
enum OptStringEnum {
    WithValue(Option<String>),
    Empty,
}

// Test 8 — Vec<u32> field
#[derive(Debug, PartialEq, Encode, Decode)]
enum VecEnum {
    Nums(Vec<u32>),
    Nothing,
}

// Test 9 — HashMap field
#[derive(Debug, PartialEq, Encode, Decode)]
enum HashMapEnum {
    Table(HashMap<String, u64>),
    Empty,
}

// Test 10 — Tuple variant (many fields)
#[derive(Debug, PartialEq, Encode, Decode)]
enum TupleHeavy {
    BigTuple(u8, u16, u32, u64, i8, i16, i32, i64, bool, String),
    Unit,
}

// Test 11 — Struct variant
#[derive(Debug, PartialEq, Encode, Decode)]
enum StructVariant {
    Record {
        id: u64,
        name: String,
        score: f64,
        active: bool,
    },
    Absent,
}

// Test 12 — C-like enum (all unit variants)
#[derive(Debug, PartialEq, Encode, Decode)]
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

// Test 13 — Variant with empty tuple ()
#[derive(Debug, PartialEq, Encode, Decode)]
enum EmptyTupleVariant {
    WithUnit(()),
    Plain,
}

// Test 14 — BTreeMap<String, Vec<u32>> field
#[derive(Debug, PartialEq, Encode, Decode)]
enum BTreeMapEnum {
    Map(BTreeMap<String, Vec<u32>>),
    Nothing,
}

// Test 15 — Recursive-like enum using Box<T>
#[derive(Debug, PartialEq, Encode, Decode)]
enum Tree {
    Leaf(i64),
    Node(Box<Tree>, Box<Tree>),
}

// Test 16 — Sequential discriminants
#[derive(Debug, PartialEq, Encode, Decode)]
enum Sequential {
    Zero,
    One,
    Two,
    Three,
    Four,
}

// Test 17 — Renamed variants
#[derive(Debug, PartialEq, Encode, Decode)]
enum RenamedEnum {
    #[oxicode(rename = "connect")]
    Connect,
    #[oxicode(rename = "disconnect")]
    Disconnect,
    #[oxicode(rename = "data_packet")]
    DataPacket(Vec<u8>),
}

// Test 18 — Skip variant: skipped variant gets next available discriminant
#[derive(Debug, PartialEq, Encode, Decode)]
enum SkipVariantAdv {
    Alpha,
    #[oxicode(skip)]
    BetaSkipped,
    Gamma,
}

// Tests 19 & 20 — Shared payload enum
#[derive(Debug, PartialEq, Encode, Decode)]
enum Payload {
    Int(i32),
    Text(String),
    Nothing,
}

// ---------------------------------------------------------------------------
// Test 1: Enum with u8 tag_type — discriminant fits in exactly 1 byte
// ---------------------------------------------------------------------------

#[test]
fn test_tag_u8_discriminant_is_one_byte() {
    // Fixed-int encoding: a u8 discriminant must occupy exactly 1 byte.
    let bytes_first = encode_fixed(&TagU8::First);
    let bytes_second = encode_fixed(&TagU8::Second);
    let bytes_third = encode_fixed(&TagU8::Third);

    assert_eq!(
        bytes_first.len(),
        1,
        "u8 tag First must be 1 byte; got {:?}",
        bytes_first
    );
    assert_eq!(
        bytes_second.len(),
        1,
        "u8 tag Second must be 1 byte; got {:?}",
        bytes_second
    );
    assert_eq!(
        bytes_third.len(),
        1,
        "u8 tag Third must be 1 byte; got {:?}",
        bytes_third
    );

    assert_eq!(bytes_first[0], 0u8, "First discriminant must be 0");
    assert_eq!(bytes_second[0], 1u8, "Second discriminant must be 1");
    assert_eq!(bytes_third[0], 2u8, "Third discriminant must be 2");

    assert_eq!(roundtrip(&TagU8::First), TagU8::First);
    assert_eq!(roundtrip(&TagU8::Second), TagU8::Second);
    assert_eq!(roundtrip(&TagU8::Third), TagU8::Third);
}

// ---------------------------------------------------------------------------
// Test 2: Enum with u16 tag_type — discriminant fits in exactly 2 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_tag_u16_discriminant_is_two_bytes() {
    // Fixed-int encoding: a u16 discriminant must occupy exactly 2 bytes.
    let bytes_alpha = encode_fixed(&TagU16::Alpha);
    assert_eq!(
        bytes_alpha.len(),
        2,
        "u16 tag Alpha must be 2 bytes; got {:?}",
        bytes_alpha
    );

    // Discriminant 0 in little-endian u16.
    assert_eq!(bytes_alpha[0], 0u8);
    assert_eq!(bytes_alpha[1], 0u8);

    let bytes_beta = encode_fixed(&TagU16::Beta);
    assert_eq!(bytes_beta.len(), 2, "u16 tag Beta must be 2 bytes");
    assert_eq!(u16::from_le_bytes([bytes_beta[0], bytes_beta[1]]), 1u16);

    for val in [TagU16::Alpha, TagU16::Beta, TagU16::Gamma] {
        let enc = encode_fixed(&val);
        let dec: TagU16 = decode_fixed(&enc);
        assert_eq!(val, dec);
    }
}

// ---------------------------------------------------------------------------
// Test 3: Enum with u32 tag_type — discriminant fits in exactly 4 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_tag_u32_discriminant_is_four_bytes() {
    // Fixed-int encoding: a u32 discriminant must occupy exactly 4 bytes.
    let bytes_only = encode_fixed(&TagU32::Only);
    assert_eq!(
        bytes_only.len(),
        4,
        "u32 tag Only must be 4 bytes; got {:?}",
        bytes_only
    );
    assert_eq!(
        u32::from_le_bytes([bytes_only[0], bytes_only[1], bytes_only[2], bytes_only[3]]),
        0u32
    );

    let bytes_another = encode_fixed(&TagU32::Another);
    assert_eq!(bytes_another.len(), 4, "u32 tag Another must be 4 bytes");
    assert_eq!(
        u32::from_le_bytes([
            bytes_another[0],
            bytes_another[1],
            bytes_another[2],
            bytes_another[3]
        ]),
        1u32
    );

    assert_eq!(roundtrip(&TagU32::Only), TagU32::Only);
    assert_eq!(roundtrip(&TagU32::Another), TagU32::Another);
}

// ---------------------------------------------------------------------------
// Test 4: Large enum (100 variants) with u8 tag_type — discriminant 99 = [99]
// ---------------------------------------------------------------------------

#[test]
fn test_large_enum_u8_tag_discriminant_99() {
    // V99 is the 100th variant (0-indexed), so its discriminant is 99.
    let bytes = encode_fixed(&LargeU8Enum::V99);
    assert_eq!(
        bytes.len(),
        1,
        "u8 tag for V99 must be exactly 1 byte; got {:?}",
        bytes
    );
    assert_eq!(bytes[0], 99u8, "V99 discriminant must be 99");

    // Also verify via roundtrip.
    assert_eq!(roundtrip(&LargeU8Enum::V99), LargeU8Enum::V99);
    assert_eq!(roundtrip(&LargeU8Enum::V00), LargeU8Enum::V00);
    assert_eq!(roundtrip(&LargeU8Enum::V50), LargeU8Enum::V50);
}

// ---------------------------------------------------------------------------
// Test 5: Large varint enum — discriminant 300 encodes with varint (3 bytes)
// ---------------------------------------------------------------------------

#[test]
fn test_large_varint_enum_discriminant_300() {
    // W300 is at position index 300. The varint scheme:
    //   0..=250  → 1 byte
    //   251..=65535 → 3 bytes: [251, lo, hi] (LE u16)
    let bytes = encode_to_vec(&LargeVarintEnum::W300).expect("encode W300 failed");
    assert_eq!(
        bytes.len(),
        3,
        "discriminant 300 should require 3 varint bytes; got {:?}",
        bytes
    );
    assert_eq!(bytes[0], 251u8, "first byte must be varint u16 tag 251");
    let discriminant = u16::from_le_bytes([bytes[1], bytes[2]]);
    assert_eq!(discriminant, 300u16, "decoded discriminant must be 300");

    // Roundtrip.
    assert_eq!(roundtrip(&LargeVarintEnum::W300), LargeVarintEnum::W300);
    // Also check a low discriminant to confirm 1-byte path still works.
    let bytes_low = encode_to_vec(&LargeVarintEnum::W000).expect("encode W000 failed");
    assert_eq!(bytes_low.len(), 1, "discriminant 0 should be 1 byte");
}

// ---------------------------------------------------------------------------
// Test 6: Enum with nested enum field
// ---------------------------------------------------------------------------

#[test]
fn test_enum_nested_enum_field() {
    let step_north = Move::Step {
        dir: Direction::North,
        distance: 10,
    };
    let jump_east = Move::Jump(Direction::East);
    let stop = Move::Stop;

    assert_eq!(roundtrip(&step_north), step_north);
    assert_eq!(roundtrip(&jump_east), jump_east);
    assert_eq!(roundtrip(&stop), stop);

    // Verify binary layout: Move::Jump has discriminant 1, Direction::East has discriminant 2.
    let bytes = encode_to_vec(&jump_east).expect("encode jump_east");
    assert_eq!(bytes[0], 1u8, "Jump discriminant must be 1");
    assert_eq!(bytes[1], 2u8, "East discriminant must be 2");
}

// ---------------------------------------------------------------------------
// Test 7: Enum with Option<String> field
// ---------------------------------------------------------------------------

#[test]
fn test_enum_option_string_field() {
    let with_some = OptStringEnum::WithValue(Some("hello oxicode".to_string()));
    let with_none = OptStringEnum::WithValue(None);
    let empty = OptStringEnum::Empty;

    assert_eq!(roundtrip(&with_some), with_some);
    assert_eq!(roundtrip(&with_none), with_none);
    assert_eq!(roundtrip(&empty), empty);
}

// ---------------------------------------------------------------------------
// Test 8: Enum with Vec<u32> field
// ---------------------------------------------------------------------------

#[test]
fn test_enum_vec_field() {
    let nums = VecEnum::Nums(vec![1, 2, 3, 100, u32::MAX]);
    let empty_vec = VecEnum::Nums(vec![]);
    let nothing = VecEnum::Nothing;

    assert_eq!(roundtrip(&nums), nums);
    assert_eq!(roundtrip(&empty_vec), empty_vec);
    assert_eq!(roundtrip(&nothing), nothing);
}

// ---------------------------------------------------------------------------
// Test 9: Enum with HashMap field
// ---------------------------------------------------------------------------

#[test]
fn test_enum_hashmap_field() {
    let mut map = HashMap::new();
    map.insert("alpha".to_string(), 1u64);
    map.insert("beta".to_string(), 2u64);
    map.insert("gamma".to_string(), 3u64);

    let with_table = HashMapEnum::Table(map.clone());
    let empty = HashMapEnum::Empty;

    // Roundtrip the HashMap variant.
    let encoded = encode_to_vec(&with_table).expect("encode HashMap variant");
    let (decoded, bytes_read): (HashMapEnum, _) =
        decode_from_slice(&encoded).expect("decode HashMap variant");
    assert_eq!(bytes_read, encoded.len());
    if let HashMapEnum::Table(decoded_map) = decoded {
        assert_eq!(decoded_map, map);
    } else {
        panic!("Expected HashMapEnum::Table");
    }

    assert_eq!(roundtrip(&empty), empty);
}

// ---------------------------------------------------------------------------
// Test 10: Enum with tuple variant (many fields)
// ---------------------------------------------------------------------------

#[test]
fn test_enum_tuple_variant_many_fields() {
    let big = TupleHeavy::BigTuple(
        255u8,
        65535u16,
        4294967295u32,
        18446744073709551615u64,
        -128i8,
        -32768i16,
        -2147483648i32,
        -9223372036854775808i64,
        true,
        "oxicode-advanced".to_string(),
    );
    assert_eq!(roundtrip(&big), big);
    assert_eq!(roundtrip(&TupleHeavy::Unit), TupleHeavy::Unit);
}

// ---------------------------------------------------------------------------
// Test 11: Enum with struct variant
// ---------------------------------------------------------------------------

#[test]
fn test_enum_struct_variant() {
    let record = StructVariant::Record {
        id: 9999999u64,
        name: "advanced-test".to_string(),
        score: std::f64::consts::PI,
        active: true,
    };
    let absent = StructVariant::Absent;

    assert_eq!(roundtrip(&record), record);
    assert_eq!(roundtrip(&absent), absent);

    // Check struct variant binary layout: discriminant 0 for Record.
    let bytes = encode_to_vec(&record).expect("encode Record");
    assert_eq!(bytes[0], 0u8, "Record discriminant must be 0");
}

// ---------------------------------------------------------------------------
// Test 12: C-like enum (all unit variants) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_clike_enum_all_unit_variants_roundtrip() {
    for season in [
        Season::Spring,
        Season::Summer,
        Season::Autumn,
        Season::Winter,
    ] {
        assert_eq!(roundtrip(&season), season);
    }

    // Verify sequential discriminants 0..=3.
    let bytes: Vec<Vec<u8>> = [
        Season::Spring,
        Season::Summer,
        Season::Autumn,
        Season::Winter,
    ]
    .iter()
    .map(|s| encode_to_vec(s).expect("encode season"))
    .collect();

    for (i, b) in bytes.iter().enumerate() {
        assert_eq!(b.len(), 1, "unit variant must be 1 byte");
        assert_eq!(b[0], i as u8, "Season discriminant {i} mismatch");
    }
}

// ---------------------------------------------------------------------------
// Test 13: Enum where one variant is an empty tuple ()
// ---------------------------------------------------------------------------

#[test]
fn test_enum_empty_tuple_variant() {
    let with_unit = EmptyTupleVariant::WithUnit(());
    let plain = EmptyTupleVariant::Plain;

    assert_eq!(roundtrip(&with_unit), with_unit);
    assert_eq!(roundtrip(&plain), plain);

    // The () field contributes 0 bytes; only the discriminant is encoded.
    let bytes_with = encode_to_vec(&with_unit).expect("encode WithUnit");
    let bytes_plain = encode_to_vec(&plain).expect("encode Plain");
    assert_eq!(
        bytes_with.len(),
        1,
        "WithUnit(()) should encode as 1 byte (discriminant only)"
    );
    assert_eq!(bytes_plain.len(), 1, "Plain should encode as 1 byte");
    assert_eq!(bytes_with[0], 0u8, "WithUnit discriminant must be 0");
    assert_eq!(bytes_plain[0], 1u8, "Plain discriminant must be 1");
}

// ---------------------------------------------------------------------------
// Test 14: Enum with BTreeMap<String, Vec<u32>> field
// ---------------------------------------------------------------------------

#[test]
fn test_enum_btreemap_vec_field() {
    let mut map: BTreeMap<String, Vec<u32>> = BTreeMap::new();
    map.insert("primes".to_string(), vec![2, 3, 5, 7, 11, 13]);
    map.insert("fibs".to_string(), vec![1, 1, 2, 3, 5, 8, 13]);
    map.insert("empty".to_string(), vec![]);

    let with_map = BTreeMapEnum::Map(map.clone());
    let nothing = BTreeMapEnum::Nothing;

    let encoded = encode_to_vec(&with_map).expect("encode BTreeMap variant");
    let (decoded, bytes_read): (BTreeMapEnum, _) =
        decode_from_slice(&encoded).expect("decode BTreeMap variant");
    assert_eq!(bytes_read, encoded.len());
    if let BTreeMapEnum::Map(decoded_map) = decoded {
        assert_eq!(decoded_map, map);
    } else {
        panic!("Expected BTreeMapEnum::Map");
    }

    assert_eq!(roundtrip(&nothing), nothing);
}

// ---------------------------------------------------------------------------
// Test 15: Recursive-like enum (Box<T> as field)
// ---------------------------------------------------------------------------

#[test]
fn test_enum_recursive_box_field() {
    // Build a small binary tree: Node(Node(Leaf(1), Leaf(2)), Leaf(3))
    let tree = Tree::Node(
        Box::new(Tree::Node(Box::new(Tree::Leaf(1)), Box::new(Tree::Leaf(2)))),
        Box::new(Tree::Leaf(3)),
    );

    assert_eq!(roundtrip(&tree), tree);

    // Also test a plain leaf.
    let leaf = Tree::Leaf(-42);
    assert_eq!(roundtrip(&leaf), leaf);
}

// ---------------------------------------------------------------------------
// Test 16: Enum discriminants are sequential (0, 1, 2, ...)
// ---------------------------------------------------------------------------

#[test]
fn test_enum_sequential_discriminants() {
    let variants = [
        Sequential::Zero,
        Sequential::One,
        Sequential::Two,
        Sequential::Three,
        Sequential::Four,
    ];

    for (expected_disc, variant) in variants.iter().enumerate() {
        let bytes = encode_to_vec(variant).expect("encode Sequential variant");
        // Discriminants 0–4 encode as a single varint byte.
        assert_eq!(bytes.len(), 1, "Sequential unit variant must be 1 byte");
        assert_eq!(
            bytes[0], expected_disc as u8,
            "Sequential::{:?} discriminant must be {expected_disc}",
            variant
        );
        assert_eq!(roundtrip(variant), *variant);
    }
}

// ---------------------------------------------------------------------------
// Test 17: Enum with renamed variants using #[oxicode(rename = "...")]
// ---------------------------------------------------------------------------

#[test]
fn test_enum_renamed_variants_roundtrip() {
    // The rename attribute is a no-op on the wire format (binary is unaffected),
    // but the derive macro must accept it without error and roundtrip must work.
    let connect = RenamedEnum::Connect;
    let disconnect = RenamedEnum::Disconnect;
    let data = RenamedEnum::DataPacket(vec![0xDE, 0xAD, 0xBE, 0xEF]);

    assert_eq!(roundtrip(&connect), connect);
    assert_eq!(roundtrip(&disconnect), disconnect);
    assert_eq!(roundtrip(&data), data);

    // Sequential discriminants 0, 1, 2 unchanged by rename.
    let bytes_conn = encode_to_vec(&connect).expect("encode Connect");
    let bytes_disc = encode_to_vec(&disconnect).expect("encode Disconnect");
    assert_eq!(bytes_conn[0], 0u8, "Connect discriminant must be 0");
    assert_eq!(bytes_disc[0], 1u8, "Disconnect discriminant must be 1");
}

// ---------------------------------------------------------------------------
// Test 18: Enum with skip variant — skipped variant shares discriminant with successor
// ---------------------------------------------------------------------------

#[test]
fn test_enum_skip_variant_discriminant_sharing() {
    // Alpha    → discriminant 0
    // BetaSkipped (skip) → shares Gamma's discriminant
    // Gamma   → discriminant 2 (position index)

    let bytes_alpha = encode_to_vec(&SkipVariantAdv::Alpha).expect("encode Alpha");
    assert_eq!(bytes_alpha[0], 0u8, "Alpha must have discriminant 0");

    let bytes_beta = encode_to_vec(&SkipVariantAdv::BetaSkipped).expect("encode BetaSkipped");
    let bytes_gamma = encode_to_vec(&SkipVariantAdv::Gamma).expect("encode Gamma");
    // BetaSkipped encodes with the same bytes as Gamma.
    assert_eq!(
        bytes_beta, bytes_gamma,
        "BetaSkipped must encode identically to Gamma"
    );

    // Decoding BetaSkipped's bytes yields Gamma.
    let (decoded, _): (SkipVariantAdv, _) =
        decode_from_slice(&bytes_beta).expect("decode BetaSkipped bytes");
    assert_eq!(
        decoded,
        SkipVariantAdv::Gamma,
        "decoding skipped bytes must yield Gamma"
    );

    // Alpha and Gamma roundtrip cleanly.
    assert_eq!(roundtrip(&SkipVariantAdv::Alpha), SkipVariantAdv::Alpha);
    assert_eq!(roundtrip(&SkipVariantAdv::Gamma), SkipVariantAdv::Gamma);
}

// ---------------------------------------------------------------------------
// Test 19: Encode/decode enum in Vec<Payload>
// ---------------------------------------------------------------------------

#[test]
fn test_enum_in_vec_roundtrip() {
    let payloads = vec![
        Payload::Int(42),
        Payload::Text("hello".to_string()),
        Payload::Nothing,
        Payload::Int(-1),
        Payload::Text(String::new()),
        Payload::Nothing,
        Payload::Int(i32::MIN),
        Payload::Int(i32::MAX),
    ];

    let encoded = encode_to_vec(&payloads).expect("encode Vec<Payload>");
    let (decoded, bytes_read): (Vec<Payload>, _) =
        decode_from_slice(&encoded).expect("decode Vec<Payload>");

    assert_eq!(
        bytes_read,
        encoded.len(),
        "not all Vec<Payload> bytes consumed"
    );
    assert_eq!(decoded, payloads, "Vec<Payload> roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 20: Encode/decode enum as BTreeMap<String, Payload> value
// ---------------------------------------------------------------------------

#[test]
fn test_enum_as_btreemap_value_roundtrip() {
    let mut map: BTreeMap<String, Payload> = BTreeMap::new();
    map.insert("count".to_string(), Payload::Int(100));
    map.insert("label".to_string(), Payload::Text("oxicode".to_string()));
    map.insert("flag".to_string(), Payload::Nothing);
    map.insert(
        "e_approx".to_string(),
        Payload::Int(
            // Use E as an integer approximation (floor)
            std::f64::consts::E.floor() as i32,
        ),
    );
    map.insert(
        "pi_approx".to_string(),
        Payload::Int(std::f64::consts::PI.floor() as i32),
    );

    let encoded = encode_to_vec(&map).expect("encode BTreeMap<String, Payload>");
    let (decoded, bytes_read): (BTreeMap<String, Payload>, _) =
        decode_from_slice(&encoded).expect("decode BTreeMap<String, Payload>");

    assert_eq!(bytes_read, encoded.len(), "not all BTreeMap bytes consumed");
    assert_eq!(decoded, map, "BTreeMap<String, Payload> roundtrip mismatch");
}

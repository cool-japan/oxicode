//! Tests for complex struct serialization patterns in OxiCode

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
struct Address {
    street: String,
    city: String,
    zip: u32,
    country: String,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct Person {
    name: String,
    age: u32,
    address: Address,
    tags: Vec<String>,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct Config {
    debug: bool,
    max_connections: u32,
    timeout_ms: u64,
    name: String,
    allowed_ips: Vec<String>,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct Matrix3x3 {
    row0: Point3D,
    row1: Point3D,
    row2: Point3D,
}

#[test]
fn test_address_roundtrip() {
    let addr = Address {
        street: "123 Main St".to_string(),
        city: "Springfield".to_string(),
        zip: 12345,
        country: "US".to_string(),
    };
    let enc = encode_to_vec(&addr).expect("Failed to encode Address");
    let (decoded, _): (Address, usize) = decode_from_slice(&enc).expect("Failed to decode Address");
    assert_eq!(addr, decoded);
}

#[test]
fn test_person_roundtrip() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        address: Address {
            street: "456 Oak Ave".to_string(),
            city: "Portland".to_string(),
            zip: 97201,
            country: "US".to_string(),
        },
        tags: vec!["developer".to_string(), "rustacean".to_string()],
    };
    let enc = encode_to_vec(&person).expect("Failed to encode Person");
    let (decoded, _): (Person, usize) = decode_from_slice(&enc).expect("Failed to decode Person");
    assert_eq!(person, decoded);
}

#[test]
fn test_config_debug_true_roundtrip() {
    let cfg = Config {
        debug: true,
        max_connections: 100,
        timeout_ms: 5000,
        name: "production".to_string(),
        allowed_ips: vec!["127.0.0.1".to_string(), "192.168.1.1".to_string()],
    };
    let enc = encode_to_vec(&cfg).expect("Failed to encode Config (debug=true)");
    let (decoded, _): (Config, usize) =
        decode_from_slice(&enc).expect("Failed to decode Config (debug=true)");
    assert_eq!(cfg, decoded);
    assert!(decoded.debug);
}

#[test]
fn test_config_debug_false_roundtrip() {
    let cfg = Config {
        debug: false,
        max_connections: 50,
        timeout_ms: 3000,
        name: "staging".to_string(),
        allowed_ips: vec!["10.0.0.1".to_string()],
    };
    let enc = encode_to_vec(&cfg).expect("Failed to encode Config (debug=false)");
    let (decoded, _): (Config, usize) =
        decode_from_slice(&enc).expect("Failed to decode Config (debug=false)");
    assert_eq!(cfg, decoded);
    assert!(!decoded.debug);
}

#[test]
fn test_point3d_origin_roundtrip() {
    let pt = Point3D {
        x: 0.0f32,
        y: 0.0f32,
        z: 0.0f32,
    };
    let enc = encode_to_vec(&pt).expect("Failed to encode Point3D origin");
    let (decoded, _): (Point3D, usize) =
        decode_from_slice(&enc).expect("Failed to decode Point3D origin");
    assert_eq!(pt, decoded);
}

#[test]
fn test_point3d_values_roundtrip() {
    let pt = Point3D {
        x: 1.5f32,
        y: 2.5f32,
        z: 3.5f32,
    };
    let enc = encode_to_vec(&pt).expect("Failed to encode Point3D values");
    let (decoded, _): (Point3D, usize) =
        decode_from_slice(&enc).expect("Failed to decode Point3D values");
    assert_eq!(pt, decoded);
}

#[test]
fn test_matrix3x3_identity_roundtrip() {
    let mat = Matrix3x3 {
        row0: Point3D {
            x: 1.0f32,
            y: 0.0f32,
            z: 0.0f32,
        },
        row1: Point3D {
            x: 0.0f32,
            y: 1.0f32,
            z: 0.0f32,
        },
        row2: Point3D {
            x: 0.0f32,
            y: 0.0f32,
            z: 1.0f32,
        },
    };
    let enc = encode_to_vec(&mat).expect("Failed to encode identity Matrix3x3");
    let (decoded, _): (Matrix3x3, usize) =
        decode_from_slice(&enc).expect("Failed to decode identity Matrix3x3");
    assert_eq!(mat, decoded);
}

#[test]
fn test_matrix3x3_roundtrip() {
    let mat = Matrix3x3 {
        row0: Point3D {
            x: 1.5f32,
            y: 2.5f32,
            z: 3.5f32,
        },
        row1: Point3D {
            x: 4.0f32,
            y: 5.0f32,
            z: 6.0f32,
        },
        row2: Point3D {
            x: 7.0f32,
            y: 8.0f32,
            z: 9.0f32,
        },
    };
    let enc = encode_to_vec(&mat).expect("Failed to encode arbitrary Matrix3x3");
    let (decoded, _): (Matrix3x3, usize) =
        decode_from_slice(&enc).expect("Failed to decode arbitrary Matrix3x3");
    assert_eq!(mat, decoded);
}

#[test]
fn test_person_empty_tags_roundtrip() {
    let person = Person {
        name: "Bob".to_string(),
        age: 25,
        address: Address {
            street: "789 Elm St".to_string(),
            city: "Seattle".to_string(),
            zip: 98101,
            country: "US".to_string(),
        },
        tags: vec![],
    };
    let enc = encode_to_vec(&person).expect("Failed to encode Person with empty tags");
    let (decoded, _): (Person, usize) =
        decode_from_slice(&enc).expect("Failed to decode Person with empty tags");
    assert_eq!(person, decoded);
    assert!(decoded.tags.is_empty());
}

#[test]
fn test_person_multiple_tags_roundtrip() {
    let person = Person {
        name: "Carol".to_string(),
        age: 35,
        address: Address {
            street: "321 Pine Rd".to_string(),
            city: "Denver".to_string(),
            zip: 80201,
            country: "US".to_string(),
        },
        tags: vec![
            "admin".to_string(),
            "moderator".to_string(),
            "contributor".to_string(),
        ],
    };
    let enc = encode_to_vec(&person).expect("Failed to encode Person with multiple tags");
    let (decoded, _): (Person, usize) =
        decode_from_slice(&enc).expect("Failed to decode Person with multiple tags");
    assert_eq!(person, decoded);
    assert_eq!(decoded.tags.len(), 3);
}

#[test]
fn test_vec_person_roundtrip() {
    let persons = vec![
        Person {
            name: "Dave".to_string(),
            age: 40,
            address: Address {
                street: "100 First St".to_string(),
                city: "Austin".to_string(),
                zip: 78701,
                country: "US".to_string(),
            },
            tags: vec!["user".to_string()],
        },
        Person {
            name: "Eve".to_string(),
            age: 28,
            address: Address {
                street: "200 Second Ave".to_string(),
                city: "Boston".to_string(),
                zip: 2101,
                country: "US".to_string(),
            },
            tags: vec!["admin".to_string(), "tester".to_string()],
        },
    ];
    let enc = encode_to_vec(&persons).expect("Failed to encode Vec<Person>");
    let (decoded, _): (Vec<Person>, usize) =
        decode_from_slice(&enc).expect("Failed to decode Vec<Person>");
    assert_eq!(persons, decoded);
    assert_eq!(decoded.len(), 2);
}

#[test]
fn test_option_person_some_roundtrip() {
    let maybe_person: Option<Person> = Some(Person {
        name: "Frank".to_string(),
        age: 45,
        address: Address {
            street: "303 Third Blvd".to_string(),
            city: "Chicago".to_string(),
            zip: 60601,
            country: "US".to_string(),
        },
        tags: vec!["vip".to_string()],
    });
    let enc = encode_to_vec(&maybe_person).expect("Failed to encode Option<Person> Some");
    let (decoded, _): (Option<Person>, usize) =
        decode_from_slice(&enc).expect("Failed to decode Option<Person> Some");
    assert_eq!(maybe_person, decoded);
    assert!(decoded.is_some());
}

#[test]
fn test_option_person_none_roundtrip() {
    let maybe_person: Option<Person> = None;
    let enc = encode_to_vec(&maybe_person).expect("Failed to encode Option<Person> None");
    let (decoded, _): (Option<Person>, usize) =
        decode_from_slice(&enc).expect("Failed to decode Option<Person> None");
    assert_eq!(maybe_person, decoded);
    assert!(decoded.is_none());
}

#[test]
fn test_person_consumed_equals_len() {
    let person = Person {
        name: "Grace".to_string(),
        age: 22,
        address: Address {
            street: "404 Fourth Ln".to_string(),
            city: "Miami".to_string(),
            zip: 33101,
            country: "US".to_string(),
        },
        tags: vec!["newbie".to_string()],
    };
    let enc = encode_to_vec(&person).expect("Failed to encode Person for consumed check");
    let (_, consumed): (Person, usize) =
        decode_from_slice(&enc).expect("Failed to decode Person for consumed check");
    assert_eq!(consumed, enc.len());
}

#[test]
fn test_person_field_preservation() {
    let person = Person {
        name: "Hank".to_string(),
        age: 55,
        address: Address {
            street: "505 Fifth Way".to_string(),
            city: "Phoenix".to_string(),
            zip: 85001,
            country: "US".to_string(),
        },
        tags: vec!["senior".to_string(), "mentor".to_string()],
    };
    let enc = encode_to_vec(&person).expect("Failed to encode Person for field preservation");
    let (decoded, _): (Person, usize) =
        decode_from_slice(&enc).expect("Failed to decode Person for field preservation");
    assert_eq!(decoded.name, "Hank");
    assert_eq!(decoded.age, 55);
    assert_eq!(decoded.address.city, "Phoenix");
    assert_eq!(decoded.address.zip, 85001);
    assert_eq!(decoded.tags.len(), 2);
}

#[test]
fn test_config_field_preservation() {
    let cfg = Config {
        debug: true,
        max_connections: 256,
        timeout_ms: 10000,
        name: "test-server".to_string(),
        allowed_ips: vec![
            "192.168.0.1".to_string(),
            "10.0.0.2".to_string(),
            "172.16.0.3".to_string(),
        ],
    };
    let enc = encode_to_vec(&cfg).expect("Failed to encode Config for field preservation");
    let (decoded, _): (Config, usize) =
        decode_from_slice(&enc).expect("Failed to decode Config for field preservation");
    assert!(decoded.debug);
    assert_eq!(decoded.max_connections, 256);
    assert_eq!(decoded.timeout_ms, 10000);
    assert_eq!(decoded.name, "test-server");
    assert_eq!(decoded.allowed_ips.len(), 3);
}

#[test]
fn test_address_fixed_int_config() {
    let addr = Address {
        street: "606 Sixth Ct".to_string(),
        city: "San Diego".to_string(),
        zip: 92101,
        country: "US".to_string(),
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let enc =
        encode_to_vec_with_config(&addr, cfg).expect("Failed to encode Address with fixed_int");
    let (decoded, _): (Address, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("Failed to decode Address with fixed_int");
    assert_eq!(addr, decoded);
}

#[test]
fn test_config_big_endian_config() {
    let cfg_val = Config {
        debug: false,
        max_connections: 1024,
        timeout_ms: 30000,
        name: "big-endian-server".to_string(),
        allowed_ips: vec!["0.0.0.0".to_string()],
    };
    let enc_cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&cfg_val, enc_cfg)
        .expect("Failed to encode Config with big_endian + fixed_int");
    let (decoded, _): (Config, usize) = decode_from_slice_with_config(&enc, enc_cfg)
        .expect("Failed to decode Config with big_endian + fixed_int");
    assert_eq!(cfg_val, decoded);
}

#[test]
fn test_person_clone_same_bytes() {
    let person = Person {
        name: "Ivy".to_string(),
        age: 33,
        address: Address {
            street: "707 Seventh Dr".to_string(),
            city: "Las Vegas".to_string(),
            zip: 89101,
            country: "US".to_string(),
        },
        tags: vec!["clone-test".to_string()],
    };
    let cloned = person.clone();
    let enc_original = encode_to_vec(&person).expect("Failed to encode original Person");
    let enc_cloned = encode_to_vec(&cloned).expect("Failed to encode cloned Person");
    assert_eq!(enc_original, enc_cloned);
}

#[test]
fn test_vec_address_roundtrip() {
    let addresses = vec![
        Address {
            street: "1 Alpha St".to_string(),
            city: "New York".to_string(),
            zip: 10001,
            country: "US".to_string(),
        },
        Address {
            street: "2 Beta Ave".to_string(),
            city: "Los Angeles".to_string(),
            zip: 90001,
            country: "US".to_string(),
        },
        Address {
            street: "3 Gamma Blvd".to_string(),
            city: "Houston".to_string(),
            zip: 77001,
            country: "US".to_string(),
        },
    ];
    let enc = encode_to_vec(&addresses).expect("Failed to encode Vec<Address>");
    let (decoded, _): (Vec<Address>, usize) =
        decode_from_slice(&enc).expect("Failed to decode Vec<Address>");
    assert_eq!(addresses, decoded);
    assert_eq!(decoded.len(), 3);
}

#[test]
fn test_config_max_values_roundtrip() {
    let cfg = Config {
        debug: false,
        max_connections: u32::MAX,
        timeout_ms: u64::MAX,
        name: "max-values".to_string(),
        allowed_ips: vec![],
    };
    let enc = encode_to_vec(&cfg).expect("Failed to encode Config with max values");
    let (decoded, _): (Config, usize) =
        decode_from_slice(&enc).expect("Failed to decode Config with max values");
    assert_eq!(cfg, decoded);
    assert_eq!(decoded.max_connections, u32::MAX);
    assert_eq!(decoded.timeout_ms, u64::MAX);
}

#[test]
fn test_matrix3x3_consumed_equals_len() {
    let mat = Matrix3x3 {
        row0: Point3D {
            x: 1.0f32,
            y: 0.0f32,
            z: 0.0f32,
        },
        row1: Point3D {
            x: 0.0f32,
            y: 1.0f32,
            z: 0.0f32,
        },
        row2: Point3D {
            x: 0.0f32,
            y: 0.0f32,
            z: 1.0f32,
        },
    };
    let enc = encode_to_vec(&mat).expect("Failed to encode Matrix3x3 for consumed check");
    let (_, consumed): (Matrix3x3, usize) =
        decode_from_slice(&enc).expect("Failed to decode Matrix3x3 for consumed check");
    assert_eq!(consumed, enc.len());
}

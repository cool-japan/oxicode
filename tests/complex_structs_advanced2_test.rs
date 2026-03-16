//! Tests for complex struct layouts with many fields and nested types.

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
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, PartialEq, Encode, Decode)]
struct Address {
    street: String,
    city: String,
    zip: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Person {
    name: String,
    age: u32,
    email: String,
    active: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Organization {
    name: String,
    members: Vec<Person>,
    hq: Address,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Status {
    Active,
    Inactive,
    Suspended(String),
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Record {
    id: u64,
    data: Vec<u8>,
    label: String,
    status: Status,
    score: f64,
    tags: Vec<String>,
}

// ─── 1. Address roundtrip ────────────────────────────────────────────────────

#[test]
fn test_address_roundtrip() {
    let addr = Address {
        street: "123 Main St".to_string(),
        city: "Springfield".to_string(),
        zip: 62701,
    };
    let enc = encode_to_vec(&addr).expect("encode Address");
    let (dec, _): (Address, _) = decode_from_slice(&enc).expect("decode Address");
    assert_eq!(addr, dec);
}

// ─── 2. Person roundtrip ─────────────────────────────────────────────────────

#[test]
fn test_person_roundtrip() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
        active: true,
    };
    let enc = encode_to_vec(&person).expect("encode Person");
    let (dec, _): (Person, _) = decode_from_slice(&enc).expect("decode Person");
    assert_eq!(person, dec);
}

// ─── 3. Organization with 3 members roundtrip ────────────────────────────────

#[test]
fn test_organization_with_three_members_roundtrip() {
    let org = Organization {
        name: "Acme Corp".to_string(),
        members: vec![
            Person {
                name: "Bob".to_string(),
                age: 25,
                email: "bob@acme.com".to_string(),
                active: true,
            },
            Person {
                name: "Carol".to_string(),
                age: 40,
                email: "carol@acme.com".to_string(),
                active: false,
            },
            Person {
                name: "Dave".to_string(),
                age: 35,
                email: "dave@acme.com".to_string(),
                active: true,
            },
        ],
        hq: Address {
            street: "1 Corporate Ave".to_string(),
            city: "Metropolis".to_string(),
            zip: 10001,
        },
    };
    let enc = encode_to_vec(&org).expect("encode Organization");
    let (dec, _): (Organization, _) = decode_from_slice(&enc).expect("decode Organization");
    assert_eq!(org, dec);
}

// ─── 4. Record with Status::Active roundtrip ─────────────────────────────────

#[test]
fn test_record_status_active_roundtrip() {
    let rec = Record {
        id: 1001,
        data: vec![0xDE, 0xAD, 0xBE, 0xEF],
        label: "active-record".to_string(),
        status: Status::Active,
        score: 9.5,
        tags: vec!["important".to_string(), "reviewed".to_string()],
    };
    let enc = encode_to_vec(&rec).expect("encode Record Active");
    let (dec, _): (Record, _) = decode_from_slice(&enc).expect("decode Record Active");
    assert_eq!(rec, dec);
}

// ─── 5. Record with Status::Inactive roundtrip ───────────────────────────────

#[test]
fn test_record_status_inactive_roundtrip() {
    let rec = Record {
        id: 2002,
        data: vec![1, 2, 3],
        label: "inactive-record".to_string(),
        status: Status::Inactive,
        score: 0.0,
        tags: vec!["archived".to_string()],
    };
    let enc = encode_to_vec(&rec).expect("encode Record Inactive");
    let (dec, _): (Record, _) = decode_from_slice(&enc).expect("decode Record Inactive");
    assert_eq!(rec, dec);
}

// ─── 6. Record with Status::Suspended roundtrip ──────────────────────────────

#[test]
fn test_record_status_suspended_roundtrip() {
    let rec = Record {
        id: 3003,
        data: vec![10, 20, 30, 40, 50],
        label: "suspended-record".to_string(),
        status: Status::Suspended("policy violation".to_string()),
        score: -1.0,
        tags: vec!["flagged".to_string(), "pending-review".to_string()],
    };
    let enc = encode_to_vec(&rec).expect("encode Record Suspended");
    let (dec, _): (Record, _) = decode_from_slice(&enc).expect("decode Record Suspended");
    assert_eq!(rec, dec);
}

// ─── 7. Vec<Person> with 5 persons roundtrip ─────────────────────────────────

#[test]
fn test_vec_of_five_persons_roundtrip() {
    let persons: Vec<Person> = (0u32..5)
        .map(|i| Person {
            name: format!("Person_{}", i),
            age: 20 + i,
            email: format!("person{}@test.com", i),
            active: i % 2 == 0,
        })
        .collect();
    let enc = encode_to_vec(&persons).expect("encode Vec<Person>");
    let (dec, _): (Vec<Person>, _) = decode_from_slice(&enc).expect("decode Vec<Person>");
    assert_eq!(persons, dec);
    assert_eq!(dec.len(), 5);
}

// ─── 8. Vec<Record> with 3 records roundtrip ─────────────────────────────────

#[test]
fn test_vec_of_three_records_roundtrip() {
    let records: Vec<Record> = vec![
        Record {
            id: 1,
            data: vec![0x01, 0x02],
            label: "first".to_string(),
            status: Status::Active,
            score: 1.1,
            tags: vec!["a".to_string()],
        },
        Record {
            id: 2,
            data: vec![0x03, 0x04, 0x05],
            label: "second".to_string(),
            status: Status::Inactive,
            score: 2.2,
            tags: vec!["b".to_string(), "c".to_string()],
        },
        Record {
            id: 3,
            data: vec![],
            label: "third".to_string(),
            status: Status::Suspended("reason".to_string()),
            score: 3.3,
            tags: vec![],
        },
    ];
    let enc = encode_to_vec(&records).expect("encode Vec<Record>");
    let (dec, _): (Vec<Record>, _) = decode_from_slice(&enc).expect("decode Vec<Record>");
    assert_eq!(records, dec);
}

// ─── 9. Option<Organization> Some roundtrip ──────────────────────────────────

#[test]
fn test_option_organization_some_roundtrip() {
    let org = Organization {
        name: "Some Corp".to_string(),
        members: vec![Person {
            name: "Eve".to_string(),
            age: 28,
            email: "eve@some.com".to_string(),
            active: true,
        }],
        hq: Address {
            street: "42 Some Street".to_string(),
            city: "Sometown".to_string(),
            zip: 99999,
        },
    };
    let opt: Option<Organization> = Some(org);
    let enc = encode_to_vec(&opt).expect("encode Option<Organization> Some");
    let (dec, _): (Option<Organization>, _) =
        decode_from_slice(&enc).expect("decode Option<Organization> Some");
    assert_eq!(opt, dec);
}

// ─── 10. Option<Organization> None roundtrip ─────────────────────────────────

#[test]
fn test_option_organization_none_roundtrip() {
    let opt: Option<Organization> = None;
    let enc = encode_to_vec(&opt).expect("encode Option<Organization> None");
    let (dec, _): (Option<Organization>, _) =
        decode_from_slice(&enc).expect("decode Option<Organization> None");
    assert_eq!(opt, dec);
}

// ─── 11. Re-encoding a decoded Organization gives same bytes ─────────────────

#[test]
fn test_organization_reencoded_bytes_identical() {
    let org = Organization {
        name: "Re-Encode Co".to_string(),
        members: vec![
            Person {
                name: "Frank".to_string(),
                age: 50,
                email: "frank@re.com".to_string(),
                active: false,
            },
            Person {
                name: "Grace".to_string(),
                age: 45,
                email: "grace@re.com".to_string(),
                active: true,
            },
        ],
        hq: Address {
            street: "7 Re-encode Blvd".to_string(),
            city: "Codec City".to_string(),
            zip: 55555,
        },
    };
    let enc1 = encode_to_vec(&org).expect("first encode");
    let (dec, _): (Organization, _) = decode_from_slice(&enc1).expect("decode");
    let enc2 = encode_to_vec(&dec).expect("re-encode");
    assert_eq!(
        enc1, enc2,
        "re-encoded bytes must be identical to first encoding"
    );
}

// ─── 12. Person with unicode name roundtrip ──────────────────────────────────

#[test]
fn test_person_unicode_name_roundtrip() {
    let person = Person {
        name: "田中 太郎 / Σωκράτης / Иван".to_string(),
        age: 33,
        email: "unicode@example.org".to_string(),
        active: true,
    };
    let enc = encode_to_vec(&person).expect("encode unicode Person");
    let (dec, _): (Person, _) = decode_from_slice(&enc).expect("decode unicode Person");
    assert_eq!(person, dec);
}

// ─── 13. Record with empty Vec<u8> and empty Vec<String> ─────────────────────

#[test]
fn test_record_empty_vecs_roundtrip() {
    let rec = Record {
        id: 0,
        data: vec![],
        label: "empty-vecs".to_string(),
        status: Status::Active,
        score: 0.0,
        tags: vec![],
    };
    let enc = encode_to_vec(&rec).expect("encode Record empty vecs");
    let (dec, _): (Record, _) = decode_from_slice(&enc).expect("decode Record empty vecs");
    assert_eq!(rec, dec);
    assert!(dec.data.is_empty());
    assert!(dec.tags.is_empty());
}

// ─── 14. Record with large data (500 bytes) roundtrip ────────────────────────

#[test]
fn test_record_large_data_roundtrip() {
    let large_data: Vec<u8> = (0u16..500).map(|i| (i % 256) as u8).collect();
    let rec = Record {
        id: u64::MAX,
        data: large_data.clone(),
        label: "large-data-record".to_string(),
        status: Status::Active,
        score: 3.14159265358979,
        tags: (0..10).map(|i| format!("tag_{}", i)).collect(),
    };
    let enc = encode_to_vec(&rec).expect("encode large Record");
    let (dec, _): (Record, _) = decode_from_slice(&enc).expect("decode large Record");
    assert_eq!(rec, dec);
    assert_eq!(dec.data.len(), 500);
    assert_eq!(dec.data, large_data);
}

// ─── 15. Nested 3-level struct: Org → Person → has fields ────────────────────

#[test]
fn test_three_level_nested_struct_roundtrip() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Department {
        name: String,
        lead: Person,
        org: Organization,
    }

    let dept = Department {
        name: "Engineering".to_string(),
        lead: Person {
            name: "Hana".to_string(),
            age: 38,
            email: "hana@eng.com".to_string(),
            active: true,
        },
        org: Organization {
            name: "Tech LLC".to_string(),
            members: vec![Person {
                name: "Ivan".to_string(),
                age: 29,
                email: "ivan@tech.com".to_string(),
                active: true,
            }],
            hq: Address {
                street: "500 Tech Drive".to_string(),
                city: "Silicon Valley".to_string(),
                zip: 94025,
            },
        },
    };
    let enc = encode_to_vec(&dept).expect("encode Department");
    let (dec, _): (Department, _) = decode_from_slice(&enc).expect("decode Department");
    assert_eq!(dept, dec);
    assert_eq!(dec.lead.name, "Hana");
    assert_eq!(dec.org.members[0].name, "Ivan");
}

// ─── 16. Vec<Address> with 10 elements roundtrip ─────────────────────────────

#[test]
fn test_vec_of_ten_addresses_roundtrip() {
    let addresses: Vec<Address> = (0u32..10)
        .map(|i| Address {
            street: format!("{} Elm Street", i * 10 + 1),
            city: format!("City_{}", i),
            zip: 10000 + i,
        })
        .collect();
    let enc = encode_to_vec(&addresses).expect("encode Vec<Address>");
    let (dec, _): (Vec<Address>, _) = decode_from_slice(&enc).expect("decode Vec<Address>");
    assert_eq!(addresses, dec);
    assert_eq!(dec.len(), 10);
}

// ─── 17. Struct containing HashMap<String, Person> ───────────────────────────

#[test]
fn test_struct_with_hashmap_string_person_roundtrip() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Registry {
        title: String,
        entries: HashMap<String, Person>,
    }

    let mut entries: HashMap<String, Person> = HashMap::new();
    entries.insert(
        "emp_001".to_string(),
        Person {
            name: "Jake".to_string(),
            age: 32,
            email: "jake@company.com".to_string(),
            active: true,
        },
    );
    entries.insert(
        "emp_002".to_string(),
        Person {
            name: "Lara".to_string(),
            age: 27,
            email: "lara@company.com".to_string(),
            active: false,
        },
    );

    let registry = Registry {
        title: "Employee Registry".to_string(),
        entries,
    };
    let enc = encode_to_vec(&registry).expect("encode Registry");
    let (dec, _): (Registry, _) = decode_from_slice(&enc).expect("decode Registry");
    assert_eq!(registry, dec);
    assert_eq!(dec.entries.len(), 2);
    assert_eq!(dec.entries["emp_001"].name, "Jake");
    assert_eq!(dec.entries["emp_002"].name, "Lara");
}

// ─── 18. Address with empty street/city (edge case) ──────────────────────────

#[test]
fn test_address_empty_street_city_roundtrip() {
    let addr = Address {
        street: "".to_string(),
        city: "".to_string(),
        zip: 0,
    };
    let enc = encode_to_vec(&addr).expect("encode Address empty fields");
    let (dec, _): (Address, _) = decode_from_slice(&enc).expect("decode Address empty fields");
    assert_eq!(addr, dec);
    assert!(dec.street.is_empty());
    assert!(dec.city.is_empty());
    assert_eq!(dec.zip, 0);
}

// ─── 19. Person with age=0 roundtrip ─────────────────────────────────────────

#[test]
fn test_person_age_zero_roundtrip() {
    let person = Person {
        name: "Newborn".to_string(),
        age: 0,
        email: "newborn@nursery.com".to_string(),
        active: true,
    };
    let enc = encode_to_vec(&person).expect("encode Person age=0");
    let (dec, _): (Person, _) = decode_from_slice(&enc).expect("decode Person age=0");
    assert_eq!(person, dec);
    assert_eq!(dec.age, 0);
}

// ─── 20. Record with score = f64::INFINITY roundtrip (bit-exact) ─────────────

#[test]
fn test_record_score_infinity_roundtrip() {
    let rec = Record {
        id: 9999,
        data: vec![0xFF],
        label: "infinity-score".to_string(),
        status: Status::Active,
        score: f64::INFINITY,
        tags: vec!["inf".to_string()],
    };
    let enc = encode_to_vec(&rec).expect("encode Record f64::INFINITY");
    let (dec, _): (Record, _) = decode_from_slice(&enc).expect("decode Record f64::INFINITY");
    assert_eq!(rec, dec);
    assert!(dec.score.is_infinite() && dec.score.is_sign_positive());
    assert_eq!(dec.score.to_bits(), f64::INFINITY.to_bits());
}

// ─── 21. Box<Organization> roundtrip ─────────────────────────────────────────

#[test]
fn test_box_organization_roundtrip() {
    let org = Box::new(Organization {
        name: "Boxed Inc".to_string(),
        members: vec![Person {
            name: "Max".to_string(),
            age: 44,
            email: "max@boxed.io".to_string(),
            active: true,
        }],
        hq: Address {
            street: "1 Box Lane".to_string(),
            city: "Boxford".to_string(),
            zip: 77777,
        },
    });
    let enc = encode_to_vec(&org).expect("encode Box<Organization>");
    let (dec, _): (Box<Organization>, _) =
        decode_from_slice(&enc).expect("decode Box<Organization>");
    assert_eq!(org, dec);
    assert_eq!(dec.name, "Boxed Inc");
    assert_eq!(dec.members[0].name, "Max");
}

// ─── 22. Arc<Person> roundtrip ───────────────────────────────────────────────

#[test]
fn test_arc_person_roundtrip() {
    let person = Arc::new(Person {
        name: "Nora".to_string(),
        age: 22,
        email: "nora@arc.dev".to_string(),
        active: true,
    });
    let enc = encode_to_vec(&person).expect("encode Arc<Person>");
    let (dec, _): (Arc<Person>, _) = decode_from_slice(&enc).expect("decode Arc<Person>");
    assert_eq!(*person, *dec);
    assert_eq!(dec.name, "Nora");
    assert_eq!(dec.age, 22);
}

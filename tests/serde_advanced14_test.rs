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
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Repository {
    name: String,
    url: String,
    stars: u64,
    language: Option<String>,
    topics: Vec<String>,
    private: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum GitEvent {
    Push { branch: String, commits: u32 },
    PullRequest { title: String, number: u32 },
    Issue { title: String, labels: Vec<String> },
    Release { tag: String, draft: bool },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Webhook {
    event: GitEvent,
    repo: Repository,
    timestamp: u64,
}

fn sample_repo() -> Repository {
    Repository {
        name: "oxicode".to_string(),
        url: "https://github.com/cool-japan/oxicode".to_string(),
        stars: 1024,
        language: Some("Rust".to_string()),
        topics: vec!["serialization".to_string(), "binary".to_string()],
        private: false,
    }
}

fn sample_webhook_push() -> Webhook {
    Webhook {
        event: GitEvent::Push {
            branch: "main".to_string(),
            commits: 3,
        },
        repo: sample_repo(),
        timestamp: 1_700_000_000,
    }
}

// Test 1: Repository roundtrip with all fields
#[test]
fn test_repository_all_fields_roundtrip() {
    let val = sample_repo();
    let bytes = encode_to_vec(&val, config::standard()).expect("encode Repository all fields");
    let (decoded, _): (Repository, usize) =
        decode_owned_from_slice::<Repository, _>(&bytes, config::standard())
            .expect("decode Repository all fields");
    assert_eq!(val, decoded);
}

// Test 2: Repository with None language
#[test]
fn test_repository_none_language_roundtrip() {
    let val = Repository {
        name: "private-lib".to_string(),
        url: "https://github.com/cool-japan/private-lib".to_string(),
        stars: 0,
        language: None,
        topics: vec!["internal".to_string()],
        private: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode Repository none language");
    let (decoded, _): (Repository, usize) =
        decode_owned_from_slice::<Repository, _>(&bytes, config::standard())
            .expect("decode Repository none language");
    assert_eq!(val, decoded);
}

// Test 3: GitEvent::Push roundtrip
#[test]
fn test_git_event_push_roundtrip() {
    let val = GitEvent::Push {
        branch: "feature/new-api".to_string(),
        commits: 7,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode GitEvent::Push");
    let (decoded, _): (GitEvent, usize) =
        decode_owned_from_slice::<GitEvent, _>(&bytes, config::standard())
            .expect("decode GitEvent::Push");
    assert_eq!(val, decoded);
}

// Test 4: GitEvent::PullRequest roundtrip
#[test]
fn test_git_event_pull_request_roundtrip() {
    let val = GitEvent::PullRequest {
        title: "Add serde support".to_string(),
        number: 42,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode GitEvent::PullRequest");
    let (decoded, _): (GitEvent, usize) =
        decode_owned_from_slice::<GitEvent, _>(&bytes, config::standard())
            .expect("decode GitEvent::PullRequest");
    assert_eq!(val, decoded);
}

// Test 5: GitEvent::Issue with multiple labels
#[test]
fn test_git_event_issue_multiple_labels_roundtrip() {
    let val = GitEvent::Issue {
        title: "Memory leak in encoder".to_string(),
        labels: vec![
            "bug".to_string(),
            "performance".to_string(),
            "help wanted".to_string(),
        ],
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode GitEvent::Issue labels");
    let (decoded, _): (GitEvent, usize) =
        decode_owned_from_slice::<GitEvent, _>(&bytes, config::standard())
            .expect("decode GitEvent::Issue labels");
    assert_eq!(val, decoded);
}

// Test 6: GitEvent::Release roundtrip
#[test]
fn test_git_event_release_roundtrip() {
    let val = GitEvent::Release {
        tag: "v0.2.0".to_string(),
        draft: false,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode GitEvent::Release");
    let (decoded, _): (GitEvent, usize) =
        decode_owned_from_slice::<GitEvent, _>(&bytes, config::standard())
            .expect("decode GitEvent::Release");
    assert_eq!(val, decoded);
}

// Test 7: Webhook with Push event roundtrip
#[test]
fn test_webhook_push_roundtrip() {
    let val = sample_webhook_push();
    let bytes = encode_to_vec(&val, config::standard()).expect("encode Webhook push");
    let (decoded, _): (Webhook, usize) =
        decode_owned_from_slice::<Webhook, _>(&bytes, config::standard())
            .expect("decode Webhook push");
    assert_eq!(val, decoded);
}

// Test 8: Webhook with Issue event roundtrip
#[test]
fn test_webhook_issue_roundtrip() {
    let val = Webhook {
        event: GitEvent::Issue {
            title: "CI pipeline failing on nightly".to_string(),
            labels: vec!["ci".to_string(), "blocker".to_string()],
        },
        repo: Repository {
            name: "oxicode".to_string(),
            url: "https://github.com/cool-japan/oxicode".to_string(),
            stars: 512,
            language: Some("Rust".to_string()),
            topics: vec!["encoding".to_string()],
            private: false,
        },
        timestamp: 1_710_000_000,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode Webhook issue");
    let (decoded, _): (Webhook, usize) =
        decode_owned_from_slice::<Webhook, _>(&bytes, config::standard())
            .expect("decode Webhook issue");
    assert_eq!(val, decoded);
}

// Test 9: Vec<Repository> roundtrip (3 items)
#[test]
fn test_vec_repository_roundtrip() {
    let val = vec![
        Repository {
            name: "alpha".to_string(),
            url: "https://github.com/cool-japan/alpha".to_string(),
            stars: 100,
            language: Some("Rust".to_string()),
            topics: vec!["alpha".to_string()],
            private: false,
        },
        Repository {
            name: "beta".to_string(),
            url: "https://github.com/cool-japan/beta".to_string(),
            stars: 200,
            language: None,
            topics: vec![],
            private: true,
        },
        Repository {
            name: "gamma".to_string(),
            url: "https://github.com/cool-japan/gamma".to_string(),
            stars: 300,
            language: Some("Python".to_string()),
            topics: vec!["data".to_string(), "analysis".to_string()],
            private: false,
        },
    ];
    let bytes = encode_to_vec(&val, config::standard()).expect("encode Vec<Repository>");
    let (decoded, _): (Vec<Repository>, usize) =
        decode_owned_from_slice::<Vec<Repository>, _>(&bytes, config::standard())
            .expect("decode Vec<Repository>");
    assert_eq!(val, decoded);
}

// Test 10: Vec<GitEvent> all 4 variants roundtrip
#[test]
fn test_vec_git_event_all_variants_roundtrip() {
    let val = vec![
        GitEvent::Push {
            branch: "main".to_string(),
            commits: 1,
        },
        GitEvent::PullRequest {
            title: "Refactor encoder".to_string(),
            number: 99,
        },
        GitEvent::Issue {
            title: "Crash on empty input".to_string(),
            labels: vec!["bug".to_string()],
        },
        GitEvent::Release {
            tag: "v1.0.0".to_string(),
            draft: true,
        },
    ];
    let bytes = encode_to_vec(&val, config::standard()).expect("encode Vec<GitEvent> all variants");
    let (decoded, _): (Vec<GitEvent>, usize) =
        decode_owned_from_slice::<Vec<GitEvent>, _>(&bytes, config::standard())
            .expect("decode Vec<GitEvent> all variants");
    assert_eq!(val, decoded);
}

// Test 11: Vec<Webhook> roundtrip (2 items)
#[test]
fn test_vec_webhook_roundtrip() {
    let val = vec![
        sample_webhook_push(),
        Webhook {
            event: GitEvent::Release {
                tag: "v0.2.0".to_string(),
                draft: false,
            },
            repo: Repository {
                name: "oxicode".to_string(),
                url: "https://github.com/cool-japan/oxicode".to_string(),
                stars: 2048,
                language: Some("Rust".to_string()),
                topics: vec!["release".to_string()],
                private: false,
            },
            timestamp: 1_720_000_000,
        },
    ];
    let bytes = encode_to_vec(&val, config::standard()).expect("encode Vec<Webhook>");
    let (decoded, _): (Vec<Webhook>, usize) =
        decode_owned_from_slice::<Vec<Webhook>, _>(&bytes, config::standard())
            .expect("decode Vec<Webhook>");
    assert_eq!(val, decoded);
}

// Test 12: u32 basic serde roundtrip
#[test]
fn test_u32_basic_roundtrip() {
    let val: u32 = 314159;
    let bytes = encode_to_vec(&val, config::standard()).expect("encode u32");
    let (decoded, _): (u32, usize) =
        decode_owned_from_slice::<u32, _>(&bytes, config::standard()).expect("decode u32");
    assert_eq!(val, decoded);
}

// Test 13: String serde roundtrip
#[test]
fn test_string_basic_roundtrip() {
    let val = "Hello from OxiCode serde integration!".to_string();
    let bytes = encode_to_vec(&val, config::standard()).expect("encode String");
    let (decoded, _): (String, usize) =
        decode_owned_from_slice::<String, _>(&bytes, config::standard()).expect("decode String");
    assert_eq!(val, decoded);
}

// Test 14: bool serde roundtrip
#[test]
fn test_bool_roundtrip() {
    for val in [true, false] {
        let bytes = encode_to_vec(&val, config::standard()).expect("encode bool");
        let (decoded, _): (bool, usize) =
            decode_owned_from_slice::<bool, _>(&bytes, config::standard()).expect("decode bool");
        assert_eq!(val, decoded);
    }
}

// Test 15: f64 serde roundtrip
#[test]
fn test_f64_roundtrip() {
    let val: f64 = std::f64::consts::PI;
    let bytes = encode_to_vec(&val, config::standard()).expect("encode f64");
    let (decoded, _): (f64, usize) =
        decode_owned_from_slice::<f64, _>(&bytes, config::standard()).expect("decode f64");
    assert!((val - decoded).abs() < f64::EPSILON);
}

// Test 16: Option<Repository> Some roundtrip
#[test]
fn test_option_repository_some_roundtrip() {
    let val: Option<Repository> = Some(sample_repo());
    let bytes = encode_to_vec(&val, config::standard()).expect("encode Option<Repository> Some");
    let (decoded, _): (Option<Repository>, usize) =
        decode_owned_from_slice::<Option<Repository>, _>(&bytes, config::standard())
            .expect("decode Option<Repository> Some");
    assert_eq!(val, decoded);
}

// Test 17: Option<Repository> None roundtrip
#[test]
fn test_option_repository_none_roundtrip() {
    let val: Option<Repository> = None;
    let bytes = encode_to_vec(&val, config::standard()).expect("encode Option<Repository> None");
    let (decoded, _): (Option<Repository>, usize) =
        decode_owned_from_slice::<Option<Repository>, _>(&bytes, config::standard())
            .expect("decode Option<Repository> None");
    assert_eq!(val, decoded);
}

// Test 18: Repository with empty topics list
#[test]
fn test_repository_empty_topics_roundtrip() {
    let val = Repository {
        name: "minimal-repo".to_string(),
        url: "https://github.com/cool-japan/minimal-repo".to_string(),
        stars: 5,
        language: Some("Rust".to_string()),
        topics: vec![],
        private: false,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode Repository empty topics");
    let (decoded, _): (Repository, usize) =
        decode_owned_from_slice::<Repository, _>(&bytes, config::standard())
            .expect("decode Repository empty topics");
    assert_eq!(val, decoded);
}

// Test 19: Repository with 5 topics
#[test]
fn test_repository_five_topics_roundtrip() {
    let val = Repository {
        name: "featured-crate".to_string(),
        url: "https://github.com/cool-japan/featured-crate".to_string(),
        stars: 9999,
        language: Some("Rust".to_string()),
        topics: vec![
            "serialization".to_string(),
            "binary".to_string(),
            "encoding".to_string(),
            "performance".to_string(),
            "pure-rust".to_string(),
        ],
        private: false,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode Repository 5 topics");
    let (decoded, _): (Repository, usize) =
        decode_owned_from_slice::<Repository, _>(&bytes, config::standard())
            .expect("decode Repository 5 topics");
    assert_eq!(val, decoded);
}

// Test 20: Consumed bytes equals encoded length for Webhook
#[test]
fn test_webhook_consumed_bytes_equals_encoded_length() {
    let val = sample_webhook_push();
    let bytes = encode_to_vec(&val, config::standard()).expect("encode Webhook for size check");
    let (_decoded, consumed): (Webhook, usize) =
        decode_owned_from_slice::<Webhook, _>(&bytes, config::standard())
            .expect("decode Webhook for size check");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes should equal the total encoded length"
    );
}

// Test 21: Repository with fixed-int config
#[test]
fn test_repository_fixed_int_config_roundtrip() {
    let val = sample_repo();
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec(&val, cfg).expect("encode Repository fixed-int");
    let (decoded, _): (Repository, usize) =
        decode_owned_from_slice::<Repository, _>(&bytes, cfg).expect("decode Repository fixed-int");
    assert_eq!(val, decoded);
}

// Test 22: Two equal Webhooks produce identical bytes
#[test]
fn test_two_equal_webhooks_produce_identical_bytes() {
    let val_a = sample_webhook_push();
    let val_b = sample_webhook_push();
    let bytes_a = encode_to_vec(&val_a, config::standard()).expect("encode Webhook A");
    let bytes_b = encode_to_vec(&val_b, config::standard()).expect("encode Webhook B");
    assert_eq!(
        bytes_a, bytes_b,
        "two structurally equal Webhooks must produce identical encoded bytes"
    );
}

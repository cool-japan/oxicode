//! Tests for cloud infrastructure enums and structs — advanced enum roundtrip coverage.

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

#[derive(Debug, PartialEq, Encode, Decode)]
enum CloudProvider {
    Aws,
    Gcp,
    Azure,
    DigitalOcean,
    Hetzner,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum InstanceState {
    Running { cpu_pct: u8, mem_mb: u64 },
    Stopped,
    Pending { eta_ms: u64 },
    Terminated { at_ms: u64, reason: String },
    Failed { error_code: u32, message: String },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Instance {
    id: String,
    provider: CloudProvider,
    state: InstanceState,
    region: String,
    instance_type: String,
    tags: Vec<(String, String)>,
    created_at: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum InfraEvent {
    Provisioned(Instance),
    StateChanged {
        instance_id: String,
        new_state: InstanceState,
    },
    Terminated {
        instance_id: String,
        at_ms: u64,
    },
    ScaleUp {
        count: u32,
        provider: CloudProvider,
    },
    ScaleDown {
        count: u32,
        reason: String,
    },
    Alert {
        severity: u8,
        message: String,
        instance_id: Option<String>,
    },
}

// ── helper ────────────────────────────────────────────────────────────────────

fn make_running_instance(id: &str, provider: CloudProvider) -> Instance {
    Instance {
        id: id.to_string(),
        provider,
        state: InstanceState::Running {
            cpu_pct: 42,
            mem_mb: 4096,
        },
        region: "us-east-1".to_string(),
        instance_type: "t3.medium".to_string(),
        tags: vec![
            ("env".to_string(), "prod".to_string()),
            ("team".to_string(), "infra".to_string()),
        ],
        created_at: 1_700_000_000,
    }
}

// ── test 1: CloudProvider all variants roundtrip + uniqueness ────────────────

#[test]
fn test_cloud_provider_all_variants_roundtrip_and_differ() {
    let variants = [
        CloudProvider::Aws,
        CloudProvider::Gcp,
        CloudProvider::Azure,
        CloudProvider::DigitalOcean,
        CloudProvider::Hetzner,
    ];

    let mut encodings: Vec<Vec<u8>> = Vec::new();
    for variant in &variants {
        let bytes = encode_to_vec(variant).expect("encode CloudProvider");
        encodings.push(bytes);
    }

    // All encodings must be pairwise distinct
    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "variants {i} and {j} must differ"
            );
        }
    }

    // Roundtrip each
    let decoded_variants = [
        CloudProvider::Aws,
        CloudProvider::Gcp,
        CloudProvider::Azure,
        CloudProvider::DigitalOcean,
        CloudProvider::Hetzner,
    ];
    for (bytes, expected) in encodings.iter().zip(decoded_variants.iter()) {
        let (decoded, _): (CloudProvider, usize) =
            decode_from_slice(bytes).expect("decode CloudProvider");
        assert_eq!(&decoded, expected);
    }
}

// ── test 2: InstanceState::Running roundtrip ─────────────────────────────────

#[test]
fn test_instance_state_running_roundtrip() {
    let val = InstanceState::Running {
        cpu_pct: 77,
        mem_mb: 8192,
    };
    let bytes = encode_to_vec(&val).expect("encode Running");
    let (decoded, _): (InstanceState, usize) = decode_from_slice(&bytes).expect("decode Running");
    assert_eq!(val, decoded);
}

// ── test 3: InstanceState::Stopped roundtrip ─────────────────────────────────

#[test]
fn test_instance_state_stopped_roundtrip() {
    let val = InstanceState::Stopped;
    let bytes = encode_to_vec(&val).expect("encode Stopped");
    let (decoded, _): (InstanceState, usize) = decode_from_slice(&bytes).expect("decode Stopped");
    assert_eq!(val, decoded);
}

// ── test 4: InstanceState::Pending roundtrip ─────────────────────────────────

#[test]
fn test_instance_state_pending_roundtrip() {
    let val = InstanceState::Pending { eta_ms: 30_000 };
    let bytes = encode_to_vec(&val).expect("encode Pending");
    let (decoded, _): (InstanceState, usize) = decode_from_slice(&bytes).expect("decode Pending");
    assert_eq!(val, decoded);
}

// ── test 5: InstanceState::Terminated roundtrip ──────────────────────────────

#[test]
fn test_instance_state_terminated_roundtrip() {
    let val = InstanceState::Terminated {
        at_ms: 1_700_999_000,
        reason: "spot-reclaim".to_string(),
    };
    let bytes = encode_to_vec(&val).expect("encode Terminated");
    let (decoded, _): (InstanceState, usize) =
        decode_from_slice(&bytes).expect("decode Terminated");
    assert_eq!(val, decoded);
}

// ── test 6: InstanceState::Failed roundtrip ──────────────────────────────────

#[test]
fn test_instance_state_failed_roundtrip() {
    let val = InstanceState::Failed {
        error_code: 503,
        message: "health check timeout".to_string(),
    };
    let bytes = encode_to_vec(&val).expect("encode Failed");
    let (decoded, _): (InstanceState, usize) = decode_from_slice(&bytes).expect("decode Failed");
    assert_eq!(val, decoded);
}

// ── test 7: Instance with Running state roundtrip ────────────────────────────

#[test]
fn test_instance_running_state_roundtrip() {
    let val = make_running_instance("i-0abc123", CloudProvider::Aws);
    let bytes = encode_to_vec(&val).expect("encode Instance Running");
    let (decoded, _): (Instance, usize) =
        decode_from_slice(&bytes).expect("decode Instance Running");
    assert_eq!(val, decoded);
}

// ── test 8: Instance with empty tags ─────────────────────────────────────────

#[test]
fn test_instance_empty_tags_roundtrip() {
    let val = Instance {
        id: "i-empty-tags".to_string(),
        provider: CloudProvider::Gcp,
        state: InstanceState::Stopped,
        region: "europe-west1".to_string(),
        instance_type: "n1-standard-2".to_string(),
        tags: vec![],
        created_at: 1_710_000_000,
    };
    let bytes = encode_to_vec(&val).expect("encode Instance empty tags");
    let (decoded, _): (Instance, usize) =
        decode_from_slice(&bytes).expect("decode Instance empty tags");
    assert_eq!(val, decoded);
}

// ── test 9: Instance with multiple tags ──────────────────────────────────────

#[test]
fn test_instance_multiple_tags_roundtrip() {
    let val = Instance {
        id: "i-multi-tags".to_string(),
        provider: CloudProvider::Azure,
        state: InstanceState::Pending { eta_ms: 5_000 },
        region: "eastus".to_string(),
        instance_type: "Standard_D4s_v3".to_string(),
        tags: vec![
            ("env".to_string(), "staging".to_string()),
            ("cost-center".to_string(), "infra-ops".to_string()),
            ("owner".to_string(), "platform-team".to_string()),
            ("ttl".to_string(), "24h".to_string()),
        ],
        created_at: 1_720_000_000,
    };
    let bytes = encode_to_vec(&val).expect("encode Instance multiple tags");
    let (decoded, _): (Instance, usize) =
        decode_from_slice(&bytes).expect("decode Instance multiple tags");
    assert_eq!(val, decoded);
}

// ── test 10: InfraEvent::Provisioned roundtrip ───────────────────────────────

#[test]
fn test_infra_event_provisioned_roundtrip() {
    let instance = make_running_instance("i-prov-001", CloudProvider::DigitalOcean);
    let val = InfraEvent::Provisioned(instance);
    let bytes = encode_to_vec(&val).expect("encode Provisioned");
    let (decoded, _): (InfraEvent, usize) = decode_from_slice(&bytes).expect("decode Provisioned");
    assert_eq!(val, decoded);
}

// ── test 11: InfraEvent::StateChanged roundtrip ──────────────────────────────

#[test]
fn test_infra_event_state_changed_roundtrip() {
    let val = InfraEvent::StateChanged {
        instance_id: "i-abc456".to_string(),
        new_state: InstanceState::Running {
            cpu_pct: 10,
            mem_mb: 2048,
        },
    };
    let bytes = encode_to_vec(&val).expect("encode StateChanged");
    let (decoded, _): (InfraEvent, usize) = decode_from_slice(&bytes).expect("decode StateChanged");
    assert_eq!(val, decoded);
}

// ── test 12: InfraEvent::Terminated roundtrip ────────────────────────────────

#[test]
fn test_infra_event_terminated_roundtrip() {
    let val = InfraEvent::Terminated {
        instance_id: "i-dead-beef".to_string(),
        at_ms: 1_730_000_000,
    };
    let bytes = encode_to_vec(&val).expect("encode InfraEvent::Terminated");
    let (decoded, _): (InfraEvent, usize) =
        decode_from_slice(&bytes).expect("decode InfraEvent::Terminated");
    assert_eq!(val, decoded);
}

// ── test 13: InfraEvent::ScaleUp roundtrip ───────────────────────────────────

#[test]
fn test_infra_event_scale_up_roundtrip() {
    let val = InfraEvent::ScaleUp {
        count: 5,
        provider: CloudProvider::Hetzner,
    };
    let bytes = encode_to_vec(&val).expect("encode ScaleUp");
    let (decoded, _): (InfraEvent, usize) = decode_from_slice(&bytes).expect("decode ScaleUp");
    assert_eq!(val, decoded);
}

// ── test 14: InfraEvent::ScaleDown roundtrip ─────────────────────────────────

#[test]
fn test_infra_event_scale_down_roundtrip() {
    let val = InfraEvent::ScaleDown {
        count: 3,
        reason: "low-traffic".to_string(),
    };
    let bytes = encode_to_vec(&val).expect("encode ScaleDown");
    let (decoded, _): (InfraEvent, usize) = decode_from_slice(&bytes).expect("decode ScaleDown");
    assert_eq!(val, decoded);
}

// ── test 15: InfraEvent::Alert with Some(instance_id) roundtrip ──────────────

#[test]
fn test_infra_event_alert_with_some_instance_id_roundtrip() {
    let val = InfraEvent::Alert {
        severity: 2,
        message: "CPU above 95% for 5 minutes".to_string(),
        instance_id: Some("i-hot-cpu".to_string()),
    };
    let bytes = encode_to_vec(&val).expect("encode Alert Some");
    let (decoded, _): (InfraEvent, usize) = decode_from_slice(&bytes).expect("decode Alert Some");
    assert_eq!(val, decoded);
}

// ── test 16: InfraEvent::Alert with None instance_id roundtrip ───────────────

#[test]
fn test_infra_event_alert_with_none_instance_id_roundtrip() {
    let val = InfraEvent::Alert {
        severity: 1,
        message: "global quota approaching limit".to_string(),
        instance_id: None,
    };
    let bytes = encode_to_vec(&val).expect("encode Alert None");
    let (decoded, _): (InfraEvent, usize) = decode_from_slice(&bytes).expect("decode Alert None");
    assert_eq!(val, decoded);
}

// ── test 17: Vec<Instance> with 3 providers roundtrip ────────────────────────

#[test]
fn test_vec_instance_three_providers_roundtrip() {
    let val: Vec<Instance> = vec![
        make_running_instance("i-aws-1", CloudProvider::Aws),
        Instance {
            id: "i-gcp-2".to_string(),
            provider: CloudProvider::Gcp,
            state: InstanceState::Stopped,
            region: "us-central1".to_string(),
            instance_type: "e2-medium".to_string(),
            tags: vec![("billing".to_string(), "shared".to_string())],
            created_at: 1_715_000_000,
        },
        Instance {
            id: "i-az-3".to_string(),
            provider: CloudProvider::Azure,
            state: InstanceState::Failed {
                error_code: 500,
                message: "disk attach failed".to_string(),
            },
            region: "westeurope".to_string(),
            instance_type: "Standard_B2s".to_string(),
            tags: vec![],
            created_at: 1_718_000_000,
        },
    ];
    let bytes = encode_to_vec(&val).expect("encode Vec<Instance>");
    let (decoded, _): (Vec<Instance>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Instance>");
    assert_eq!(val, decoded);
}

// ── test 18: Vec<InfraEvent> mixed roundtrip ─────────────────────────────────

#[test]
fn test_vec_infra_event_mixed_roundtrip() {
    let val: Vec<InfraEvent> = vec![
        InfraEvent::Provisioned(make_running_instance("i-mix-1", CloudProvider::Aws)),
        InfraEvent::StateChanged {
            instance_id: "i-mix-1".to_string(),
            new_state: InstanceState::Pending { eta_ms: 10_000 },
        },
        InfraEvent::ScaleUp {
            count: 2,
            provider: CloudProvider::Gcp,
        },
        InfraEvent::Alert {
            severity: 3,
            message: "disk full".to_string(),
            instance_id: Some("i-mix-2".to_string()),
        },
        InfraEvent::Terminated {
            instance_id: "i-mix-3".to_string(),
            at_ms: 1_740_000_000,
        },
        InfraEvent::ScaleDown {
            count: 1,
            reason: "scheduled".to_string(),
        },
    ];
    let bytes = encode_to_vec(&val).expect("encode Vec<InfraEvent>");
    let (decoded, _): (Vec<InfraEvent>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<InfraEvent>");
    assert_eq!(val, decoded);
}

// ── test 19: consumed bytes == encoded length for Instance ───────────────────

#[test]
fn test_instance_consumed_bytes_equals_encoded_length() {
    let val = make_running_instance("i-len-check", CloudProvider::Hetzner);
    let bytes = encode_to_vec(&val).expect("encode Instance for length check");
    let (decoded, consumed): (Instance, usize) =
        decode_from_slice(&bytes).expect("decode Instance for length check");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal encoded length"
    );
}

// ── test 20: big-endian config Instance roundtrip ────────────────────────────

#[test]
fn test_instance_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let val = Instance {
        id: "i-be-001".to_string(),
        provider: CloudProvider::DigitalOcean,
        state: InstanceState::Running {
            cpu_pct: 55,
            mem_mb: 16384,
        },
        region: "nyc3".to_string(),
        instance_type: "s-4vcpu-8gb".to_string(),
        tags: vec![("env".to_string(), "prod".to_string())],
        created_at: 1_705_000_000,
    };
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode Instance big-endian");
    let (decoded, _): (Instance, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode Instance big-endian");
    assert_eq!(val, decoded);
}

// ── test 21: fixed-int config InstanceState roundtrip ────────────────────────

#[test]
fn test_instance_state_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = InstanceState::Terminated {
        at_ms: 1_750_000_000,
        reason: "manual-shutdown".to_string(),
    };
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode InstanceState fixed-int");
    let (decoded, _): (InstanceState, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode InstanceState fixed-int");
    assert_eq!(val, decoded);
}

// ── test 22: discriminant uniqueness — all 5 InstanceState variants differ ───

#[test]
fn test_instance_state_discriminant_uniqueness() {
    let variants: Vec<InstanceState> = vec![
        InstanceState::Running {
            cpu_pct: 0,
            mem_mb: 0,
        },
        InstanceState::Stopped,
        InstanceState::Pending { eta_ms: 0 },
        InstanceState::Terminated {
            at_ms: 0,
            reason: String::new(),
        },
        InstanceState::Failed {
            error_code: 0,
            message: String::new(),
        },
    ];

    let encodings: Vec<Vec<u8>> = variants
        .iter()
        .map(|v| encode_to_vec(v).expect("encode InstanceState variant"))
        .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "InstanceState variants {i} and {j} must encode differently"
            );
        }
    }
}

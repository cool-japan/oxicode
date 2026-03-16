//! Advanced cybersecurity / threat detection enum tests for oxicode (set 16)

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
enum ThreatCategory {
    Malware,
    Phishing,
    DDoS,
    Ransomware,
    InsiderThreat,
    ZeroDay,
    SocialEngineering,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum AlertAction {
    Block,
    Monitor,
    Quarantine,
    Alert { recipients: Vec<String> },
    AutoRemediate { script: String },
    Escalate { tier: u8 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SecurityAlert {
    alert_id: u64,
    category: ThreatCategory,
    severity: Severity,
    action: AlertAction,
    source_ip: u32,
    timestamp: u64,
    description: String,
}

// ── ThreatCategory variant tests ─────────────────────────────────────────────

#[test]
fn test_threat_category_malware_roundtrip() {
    let val = ThreatCategory::Malware;
    let encoded = encode_to_vec(&val).expect("encode ThreatCategory::Malware");
    let (decoded, _): (ThreatCategory, _) =
        decode_from_slice(&encoded).expect("decode ThreatCategory::Malware");
    assert_eq!(val, decoded);
}

#[test]
fn test_threat_category_phishing_roundtrip() {
    let val = ThreatCategory::Phishing;
    let encoded = encode_to_vec(&val).expect("encode ThreatCategory::Phishing");
    let (decoded, _): (ThreatCategory, _) =
        decode_from_slice(&encoded).expect("decode ThreatCategory::Phishing");
    assert_eq!(val, decoded);
}

#[test]
fn test_threat_category_ddos_and_ransomware_differ() {
    let ddos = ThreatCategory::DDoS;
    let ransomware = ThreatCategory::Ransomware;
    let enc_ddos = encode_to_vec(&ddos).expect("encode DDoS");
    let enc_ransomware = encode_to_vec(&ransomware).expect("encode Ransomware");
    assert_ne!(
        enc_ddos, enc_ransomware,
        "DDoS and Ransomware must have distinct encodings"
    );
}

#[test]
fn test_threat_category_insider_threat_roundtrip() {
    let val = ThreatCategory::InsiderThreat;
    let encoded = encode_to_vec(&val).expect("encode InsiderThreat");
    let (decoded, _): (ThreatCategory, _) =
        decode_from_slice(&encoded).expect("decode InsiderThreat");
    assert_eq!(val, decoded);
}

#[test]
fn test_threat_category_zero_day_roundtrip() {
    let val = ThreatCategory::ZeroDay;
    let encoded = encode_to_vec(&val).expect("encode ZeroDay");
    let (decoded, _): (ThreatCategory, _) = decode_from_slice(&encoded).expect("decode ZeroDay");
    assert_eq!(val, decoded);
}

#[test]
fn test_threat_category_social_engineering_roundtrip() {
    let val = ThreatCategory::SocialEngineering;
    let encoded = encode_to_vec(&val).expect("encode SocialEngineering");
    let (decoded, _): (ThreatCategory, _) =
        decode_from_slice(&encoded).expect("decode SocialEngineering");
    assert_eq!(val, decoded);
}

// ── Severity variant tests ────────────────────────────────────────────────────

#[test]
fn test_severity_all_variants_unique_encodings() {
    let variants = [
        Severity::Critical,
        Severity::High,
        Severity::Medium,
        Severity::Low,
        Severity::Informational,
    ];
    let encodings: Vec<Vec<u8>> = variants
        .iter()
        .map(|v| encode_to_vec(v).expect("encode Severity variant"))
        .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "Severity variants {i} and {j} must differ"
            );
        }
    }
}

#[test]
fn test_severity_critical_roundtrip() {
    let val = Severity::Critical;
    let encoded = encode_to_vec(&val).expect("encode Severity::Critical");
    let (decoded, consumed): (Severity, _) =
        decode_from_slice(&encoded).expect("decode Severity::Critical");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal encoded length"
    );
}

#[test]
fn test_severity_informational_roundtrip() {
    let val = Severity::Informational;
    let encoded = encode_to_vec(&val).expect("encode Severity::Informational");
    let (decoded, _): (Severity, _) =
        decode_from_slice(&encoded).expect("decode Severity::Informational");
    assert_eq!(val, decoded);
}

// ── AlertAction variant tests ─────────────────────────────────────────────────

#[test]
fn test_alert_action_block_and_monitor_roundtrip() {
    for action in [AlertAction::Block, AlertAction::Monitor] {
        let encoded = encode_to_vec(&action).expect("encode simple AlertAction");
        let (decoded, _): (AlertAction, _) =
            decode_from_slice(&encoded).expect("decode simple AlertAction");
        assert_eq!(action, decoded);
    }
}

#[test]
fn test_alert_action_quarantine_roundtrip() {
    let val = AlertAction::Quarantine;
    let encoded = encode_to_vec(&val).expect("encode Quarantine");
    let (decoded, _): (AlertAction, _) = decode_from_slice(&encoded).expect("decode Quarantine");
    assert_eq!(val, decoded);
}

#[test]
fn test_alert_action_alert_with_recipients_roundtrip() {
    let val = AlertAction::Alert {
        recipients: vec![
            String::from("soc@example.com"),
            String::from("ciso@example.com"),
            String::from("ops@example.com"),
        ],
    };
    let encoded = encode_to_vec(&val).expect("encode Alert{recipients}");
    let (decoded, consumed): (AlertAction, _) =
        decode_from_slice(&encoded).expect("decode Alert{recipients}");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal full encoded length"
    );
}

#[test]
fn test_alert_action_alert_empty_recipients_roundtrip() {
    let val = AlertAction::Alert { recipients: vec![] };
    let encoded = encode_to_vec(&val).expect("encode Alert with empty recipients");
    let (decoded, _): (AlertAction, _) =
        decode_from_slice(&encoded).expect("decode Alert with empty recipients");
    assert_eq!(val, decoded);
}

#[test]
fn test_alert_action_auto_remediate_roundtrip() {
    let val = AlertAction::AutoRemediate {
        script: String::from("/opt/remediate/isolate_host.sh --force --log=/var/log/sec.log"),
    };
    let encoded = encode_to_vec(&val).expect("encode AutoRemediate");
    let (decoded, consumed): (AlertAction, _) =
        decode_from_slice(&encoded).expect("decode AutoRemediate");
    assert_eq!(val, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_alert_action_escalate_roundtrip() {
    let val = AlertAction::Escalate { tier: 3 };
    let encoded = encode_to_vec(&val).expect("encode Escalate");
    let (decoded, _): (AlertAction, _) = decode_from_slice(&encoded).expect("decode Escalate");
    assert_eq!(val, decoded);
}

// ── SecurityAlert struct tests ────────────────────────────────────────────────

#[test]
fn test_security_alert_full_roundtrip() {
    let alert = SecurityAlert {
        alert_id: 0xDEAD_BEEF_CAFE_1234_u64,
        category: ThreatCategory::Ransomware,
        severity: Severity::Critical,
        action: AlertAction::Quarantine,
        source_ip: 0xC0A8_0101_u32, // 192.168.1.1
        timestamp: 1_700_000_000_u64,
        description: String::from("Ransomware encryption activity detected on host WKSTN-042"),
    };
    let encoded = encode_to_vec(&alert).expect("encode SecurityAlert");
    let (decoded, consumed): (SecurityAlert, _) =
        decode_from_slice(&encoded).expect("decode SecurityAlert");
    assert_eq!(alert, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_security_alert_with_alert_action_recipients() {
    let alert = SecurityAlert {
        alert_id: 99,
        category: ThreatCategory::Phishing,
        severity: Severity::High,
        action: AlertAction::Alert {
            recipients: vec![
                String::from("analyst1@corp.local"),
                String::from("analyst2@corp.local"),
            ],
        },
        source_ip: 0x0A00_0001_u32, // 10.0.0.1
        timestamp: 1_710_000_000_u64,
        description: String::from("Credential harvesting page reported by user"),
    };
    let encoded = encode_to_vec(&alert).expect("encode SecurityAlert with Alert action");
    let (decoded, _): (SecurityAlert, _) =
        decode_from_slice(&encoded).expect("decode SecurityAlert with Alert action");
    assert_eq!(alert, decoded);
}

// ── Config-based tests ────────────────────────────────────────────────────────

#[test]
fn test_security_alert_with_config_big_endian() {
    let cfg = config::standard().with_big_endian();
    let alert = SecurityAlert {
        alert_id: 1,
        category: ThreatCategory::ZeroDay,
        severity: Severity::Critical,
        action: AlertAction::Escalate { tier: 1 },
        source_ip: 0x7F00_0001_u32,
        timestamp: 9_999_999_999_u64,
        description: String::from("Zero-day exploit attempt via CVE-XXXX-YYYY"),
    };
    let encoded = encode_to_vec_with_config(&alert, cfg).expect("encode SecurityAlert big-endian");
    let (decoded, consumed): (SecurityAlert, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode SecurityAlert big-endian");
    assert_eq!(alert, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_threat_category_all_variants_config_fixed_int() {
    let cfg = config::standard().with_fixed_int_encoding();
    let variants = [
        ThreatCategory::Malware,
        ThreatCategory::Phishing,
        ThreatCategory::DDoS,
        ThreatCategory::Ransomware,
        ThreatCategory::InsiderThreat,
        ThreatCategory::ZeroDay,
        ThreatCategory::SocialEngineering,
    ];
    for variant in variants {
        let encoded =
            encode_to_vec_with_config(&variant, cfg).expect("encode ThreatCategory fixed-int");
        let (decoded, _): (ThreatCategory, _) =
            decode_from_slice_with_config(&encoded, cfg).expect("decode ThreatCategory fixed-int");
        assert_eq!(variant, decoded);
    }
}

// ── Vec<SecurityAlert> tests ──────────────────────────────────────────────────

#[test]
fn test_vec_of_security_alerts_roundtrip() {
    let alerts = vec![
        SecurityAlert {
            alert_id: 1001,
            category: ThreatCategory::Malware,
            severity: Severity::High,
            action: AlertAction::Block,
            source_ip: 0x0101_0101_u32,
            timestamp: 1_000_000_u64,
            description: String::from("Trojan dropper detected"),
        },
        SecurityAlert {
            alert_id: 1002,
            category: ThreatCategory::DDoS,
            severity: Severity::Medium,
            action: AlertAction::Monitor,
            source_ip: 0x0808_0808_u32,
            timestamp: 1_000_100_u64,
            description: String::from("Volumetric UDP flood from external source"),
        },
        SecurityAlert {
            alert_id: 1003,
            category: ThreatCategory::InsiderThreat,
            severity: Severity::Low,
            action: AlertAction::AutoRemediate {
                script: String::from("/scripts/revoke_session.sh"),
            },
            source_ip: 0xAC10_0102_u32,
            timestamp: 1_000_200_u64,
            description: String::from("Unusual data exfiltration pattern by privileged user"),
        },
    ];
    let encoded = encode_to_vec(&alerts).expect("encode Vec<SecurityAlert>");
    let (decoded, consumed): (Vec<SecurityAlert>, _) =
        decode_from_slice(&encoded).expect("decode Vec<SecurityAlert>");
    assert_eq!(alerts, decoded);
    assert_eq!(consumed, encoded.len());
}

// ── Consumed bytes / partial buffer test ─────────────────────────────────────

#[test]
fn test_consumed_bytes_security_alert_with_trailing_data() {
    let alert = SecurityAlert {
        alert_id: 42,
        category: ThreatCategory::SocialEngineering,
        severity: Severity::Medium,
        action: AlertAction::Alert {
            recipients: vec![String::from("noc@example.org")],
        },
        source_ip: 0xC0A8_0A0A_u32,
        timestamp: 1_720_000_000_u64,
        description: String::from("Pretexting call targeting finance team"),
    };
    let encoded = encode_to_vec(&alert).expect("encode SecurityAlert for consumed-bytes test");
    let mut buf = encoded.clone();
    buf.extend_from_slice(&[0xFF, 0xFE, 0xFD]); // trailing garbage
    let (decoded, consumed): (SecurityAlert, _) =
        decode_from_slice(&buf).expect("decode SecurityAlert from extended buffer");
    assert_eq!(alert, decoded);
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal original encoded length, not include trailing bytes"
    );
}

// ── Discriminant uniqueness test ──────────────────────────────────────────────

#[test]
fn test_alert_action_all_discriminants_unique() {
    let variants: Vec<AlertAction> = vec![
        AlertAction::Block,
        AlertAction::Monitor,
        AlertAction::Quarantine,
        AlertAction::Alert {
            recipients: vec![String::from("a@b.com")],
        },
        AlertAction::AutoRemediate {
            script: String::from("script.sh"),
        },
        AlertAction::Escalate { tier: 2 },
    ];
    let encodings: Vec<Vec<u8>> = variants
        .iter()
        .map(|v| encode_to_vec(v).expect("encode AlertAction variant"))
        .collect();
    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "AlertAction variants {i} and {j} must have distinct encodings"
            );
        }
    }
}

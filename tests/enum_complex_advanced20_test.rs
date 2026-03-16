//! Space exploration / mission control domain tests for oxicode enum and struct encoding.

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
enum MissionPhase {
    Launch,
    AscendOrbit,
    TransferOrbit,
    Cruise,
    OrbitInsertion,
    Landing,
    SurfaceOps,
    Reentry,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum SubsystemStatus {
    Nominal,
    Degraded,
    Fault,
    Off,
    SafeMode,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum PropulsionType {
    Chemical,
    IonThruster,
    HallEffect,
    SolarSail,
    NuclearThermal,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SubsystemTelemetry {
    subsystem_id: u16,
    status: SubsystemStatus,
    temperature_mk: u32,
    power_mw: u32,
    timestamp_s: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ManeuverCommand {
    maneuver_id: u32,
    propulsion: PropulsionType,
    delta_v_mms: i64,
    burn_duration_ms: u32,
    scheduled_at: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MissionSnapshot {
    mission_id: u64,
    phase: MissionPhase,
    telemetry: Vec<SubsystemTelemetry>,
    pending_maneuvers: Vec<ManeuverCommand>,
    elapsed_s: u64,
}

// --- Test 1: MissionPhase variants roundtrip (all 8 variants) ---
#[test]
fn test_all_mission_phase_variants() {
    let variants = [
        MissionPhase::Launch,
        MissionPhase::AscendOrbit,
        MissionPhase::TransferOrbit,
        MissionPhase::Cruise,
        MissionPhase::OrbitInsertion,
        MissionPhase::Landing,
        MissionPhase::SurfaceOps,
        MissionPhase::Reentry,
    ];
    for phase in variants {
        let bytes = encode_to_vec(&phase).expect("encode MissionPhase");
        let (decoded, _): (MissionPhase, usize) =
            decode_from_slice(&bytes).expect("decode MissionPhase");
        assert_eq!(phase, decoded);
    }
}

// --- Test 2: SubsystemStatus variants roundtrip (all 5 variants) ---
#[test]
fn test_all_subsystem_status_variants() {
    let variants = [
        SubsystemStatus::Nominal,
        SubsystemStatus::Degraded,
        SubsystemStatus::Fault,
        SubsystemStatus::Off,
        SubsystemStatus::SafeMode,
    ];
    for status in variants {
        let bytes = encode_to_vec(&status).expect("encode SubsystemStatus");
        let (decoded, _): (SubsystemStatus, usize) =
            decode_from_slice(&bytes).expect("decode SubsystemStatus");
        assert_eq!(status, decoded);
    }
}

// --- Test 3: PropulsionType variants roundtrip (all 5 variants) ---
#[test]
fn test_all_propulsion_type_variants() {
    let variants = [
        PropulsionType::Chemical,
        PropulsionType::IonThruster,
        PropulsionType::HallEffect,
        PropulsionType::SolarSail,
        PropulsionType::NuclearThermal,
    ];
    for propulsion in variants {
        let bytes = encode_to_vec(&propulsion).expect("encode PropulsionType");
        let (decoded, _): (PropulsionType, usize) =
            decode_from_slice(&bytes).expect("decode PropulsionType");
        assert_eq!(propulsion, decoded);
    }
}

// --- Test 4: SubsystemTelemetry struct roundtrip ---
#[test]
fn test_subsystem_telemetry_roundtrip() {
    let telemetry = SubsystemTelemetry {
        subsystem_id: 42,
        status: SubsystemStatus::Nominal,
        temperature_mk: 293_000,
        power_mw: 1500,
        timestamp_s: 1_700_000_000,
    };
    let bytes = encode_to_vec(&telemetry).expect("encode SubsystemTelemetry");
    let (decoded, _): (SubsystemTelemetry, usize) =
        decode_from_slice(&bytes).expect("decode SubsystemTelemetry");
    assert_eq!(telemetry, decoded);
}

// --- Test 5: ManeuverCommand roundtrip with positive delta-v ---
#[test]
fn test_maneuver_command_positive_delta_v() {
    let cmd = ManeuverCommand {
        maneuver_id: 1001,
        propulsion: PropulsionType::Chemical,
        delta_v_mms: 3_500_000,
        burn_duration_ms: 120_000,
        scheduled_at: 1_700_100_000,
    };
    let bytes = encode_to_vec(&cmd).expect("encode ManeuverCommand positive delta-v");
    let (decoded, _): (ManeuverCommand, usize) =
        decode_from_slice(&bytes).expect("decode ManeuverCommand positive delta-v");
    assert_eq!(cmd, decoded);
}

// --- Test 6: ManeuverCommand roundtrip with negative delta-v (retrograde burn) ---
#[test]
fn test_maneuver_command_negative_delta_v() {
    let cmd = ManeuverCommand {
        maneuver_id: 2002,
        propulsion: PropulsionType::HallEffect,
        delta_v_mms: -750_000,
        burn_duration_ms: 45_000,
        scheduled_at: 1_700_200_000,
    };
    let bytes = encode_to_vec(&cmd).expect("encode ManeuverCommand negative delta-v");
    let (decoded, _): (ManeuverCommand, usize) =
        decode_from_slice(&bytes).expect("decode ManeuverCommand negative delta-v");
    assert_eq!(cmd, decoded);
}

// --- Test 7: MissionSnapshot with empty telemetry and maneuvers ---
#[test]
fn test_mission_snapshot_empty() {
    let snapshot = MissionSnapshot {
        mission_id: 9_001,
        phase: MissionPhase::Cruise,
        telemetry: vec![],
        pending_maneuvers: vec![],
        elapsed_s: 0,
    };
    let bytes = encode_to_vec(&snapshot).expect("encode empty MissionSnapshot");
    let (decoded, _): (MissionSnapshot, usize) =
        decode_from_slice(&bytes).expect("decode empty MissionSnapshot");
    assert_eq!(snapshot, decoded);
}

// --- Test 8: MissionSnapshot with 5 telemetry items ---
#[test]
fn test_mission_snapshot_five_telemetry_items() {
    let telemetry: Vec<SubsystemTelemetry> = (0..5)
        .map(|i| SubsystemTelemetry {
            subsystem_id: i as u16,
            status: SubsystemStatus::Nominal,
            temperature_mk: 270_000 + i as u32 * 1_000,
            power_mw: 500 + i as u32 * 100,
            timestamp_s: 1_700_000_000 + i as u64 * 60,
        })
        .collect();
    let snapshot = MissionSnapshot {
        mission_id: 9_002,
        phase: MissionPhase::TransferOrbit,
        telemetry,
        pending_maneuvers: vec![],
        elapsed_s: 86_400,
    };
    let bytes = encode_to_vec(&snapshot).expect("encode MissionSnapshot 5 telemetry");
    let (decoded, _): (MissionSnapshot, usize) =
        decode_from_slice(&bytes).expect("decode MissionSnapshot 5 telemetry");
    assert_eq!(snapshot, decoded);
}

// --- Test 9: MissionSnapshot with 3 pending maneuvers ---
#[test]
fn test_mission_snapshot_three_maneuvers() {
    let maneuvers: Vec<ManeuverCommand> = (0..3)
        .map(|i| ManeuverCommand {
            maneuver_id: 3000 + i as u32,
            propulsion: PropulsionType::IonThruster,
            delta_v_mms: 100_000 * (i as i64 + 1),
            burn_duration_ms: 10_000 * (i as u32 + 1),
            scheduled_at: 1_700_300_000 + i as u64 * 3600,
        })
        .collect();
    let snapshot = MissionSnapshot {
        mission_id: 9_003,
        phase: MissionPhase::OrbitInsertion,
        telemetry: vec![],
        pending_maneuvers: maneuvers,
        elapsed_s: 172_800,
    };
    let bytes = encode_to_vec(&snapshot).expect("encode MissionSnapshot 3 maneuvers");
    let (decoded, _): (MissionSnapshot, usize) =
        decode_from_slice(&bytes).expect("decode MissionSnapshot 3 maneuvers");
    assert_eq!(snapshot, decoded);
}

// --- Test 10: big_endian config roundtrip ---
#[test]
fn test_big_endian_config() {
    let cfg = config::standard().with_big_endian();
    let telemetry = SubsystemTelemetry {
        subsystem_id: 7,
        status: SubsystemStatus::Degraded,
        temperature_mk: 500_000,
        power_mw: 2_000,
        timestamp_s: 1_700_400_000,
    };
    let bytes = encode_to_vec_with_config(&telemetry, cfg).expect("encode big endian");
    let (decoded, _): (SubsystemTelemetry, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode big endian");
    assert_eq!(telemetry, decoded);
}

// --- Test 11: fixed_int config roundtrip ---
#[test]
fn test_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let cmd = ManeuverCommand {
        maneuver_id: 5555,
        propulsion: PropulsionType::SolarSail,
        delta_v_mms: 10_000,
        burn_duration_ms: 0,
        scheduled_at: 1_700_500_000,
    };
    let bytes = encode_to_vec_with_config(&cmd, cfg).expect("encode fixed int");
    let (decoded, _): (ManeuverCommand, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode fixed int");
    assert_eq!(cmd, decoded);
}

// --- Test 12: big_endian + fixed_int combined config ---
#[test]
fn test_big_endian_fixed_int_combined_config() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let snapshot = MissionSnapshot {
        mission_id: 99_999,
        phase: MissionPhase::Landing,
        telemetry: vec![SubsystemTelemetry {
            subsystem_id: 1,
            status: SubsystemStatus::Nominal,
            temperature_mk: 300_000,
            power_mw: 1_000,
            timestamp_s: 1_700_600_000,
        }],
        pending_maneuvers: vec![],
        elapsed_s: 259_200,
    };
    let bytes = encode_to_vec_with_config(&snapshot, cfg).expect("encode big endian + fixed int");
    let (decoded, _): (MissionSnapshot, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode big endian + fixed int");
    assert_eq!(snapshot, decoded);
}

// --- Test 13: consumed bytes check ---
#[test]
fn test_consumed_bytes_check() {
    let phase = MissionPhase::AscendOrbit;
    let bytes = encode_to_vec(&phase).expect("encode for consumed bytes check");
    let (decoded, consumed): (MissionPhase, usize) =
        decode_from_slice(&bytes).expect("decode for consumed bytes check");
    assert_eq!(phase, decoded);
    assert!(consumed > 0, "consumed bytes must be positive");
    assert!(
        consumed <= bytes.len(),
        "consumed bytes cannot exceed buffer length"
    );
}

// --- Test 14: Vec<SubsystemTelemetry> roundtrip ---
#[test]
fn test_vec_subsystem_telemetry_roundtrip() {
    let items: Vec<SubsystemTelemetry> = vec![
        SubsystemTelemetry {
            subsystem_id: 10,
            status: SubsystemStatus::Nominal,
            temperature_mk: 280_000,
            power_mw: 800,
            timestamp_s: 1_700_700_000,
        },
        SubsystemTelemetry {
            subsystem_id: 11,
            status: SubsystemStatus::Fault,
            temperature_mk: 450_000,
            power_mw: 0,
            timestamp_s: 1_700_700_060,
        },
        SubsystemTelemetry {
            subsystem_id: 12,
            status: SubsystemStatus::SafeMode,
            temperature_mk: 200_000,
            power_mw: 50,
            timestamp_s: 1_700_700_120,
        },
    ];
    let bytes = encode_to_vec(&items).expect("encode Vec<SubsystemTelemetry>");
    let (decoded, _): (Vec<SubsystemTelemetry>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<SubsystemTelemetry>");
    assert_eq!(items, decoded);
}

// --- Test 15: Vec<ManeuverCommand> roundtrip ---
#[test]
fn test_vec_maneuver_command_roundtrip() {
    let cmds: Vec<ManeuverCommand> = vec![
        ManeuverCommand {
            maneuver_id: 7001,
            propulsion: PropulsionType::Chemical,
            delta_v_mms: 900_000,
            burn_duration_ms: 60_000,
            scheduled_at: 1_700_800_000,
        },
        ManeuverCommand {
            maneuver_id: 7002,
            propulsion: PropulsionType::NuclearThermal,
            delta_v_mms: -200_000,
            burn_duration_ms: 30_000,
            scheduled_at: 1_700_900_000,
        },
    ];
    let bytes = encode_to_vec(&cmds).expect("encode Vec<ManeuverCommand>");
    let (decoded, _): (Vec<ManeuverCommand>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<ManeuverCommand>");
    assert_eq!(cmds, decoded);
}

// --- Test 16: large snapshot with 20 subsystems ---
#[test]
fn test_large_snapshot_twenty_subsystems() {
    let telemetry: Vec<SubsystemTelemetry> = (0..20)
        .map(|i| SubsystemTelemetry {
            subsystem_id: i as u16,
            status: if i % 5 == 0 {
                SubsystemStatus::Degraded
            } else {
                SubsystemStatus::Nominal
            },
            temperature_mk: 200_000 + i as u32 * 5_000,
            power_mw: 100 + i as u32 * 50,
            timestamp_s: 1_701_000_000 + i as u64 * 10,
        })
        .collect();
    let snapshot = MissionSnapshot {
        mission_id: 100_000,
        phase: MissionPhase::Cruise,
        telemetry,
        pending_maneuvers: vec![],
        elapsed_s: 604_800,
    };
    let bytes = encode_to_vec(&snapshot).expect("encode large snapshot 20 subsystems");
    let (decoded, _): (MissionSnapshot, usize) =
        decode_from_slice(&bytes).expect("decode large snapshot 20 subsystems");
    assert_eq!(snapshot, decoded);
    assert_eq!(decoded.telemetry.len(), 20);
}

// --- Test 17: fault subsystem detection after roundtrip ---
#[test]
fn test_fault_subsystem_detection() {
    let telemetry = vec![
        SubsystemTelemetry {
            subsystem_id: 0,
            status: SubsystemStatus::Nominal,
            temperature_mk: 293_000,
            power_mw: 1_200,
            timestamp_s: 1_701_100_000,
        },
        SubsystemTelemetry {
            subsystem_id: 1,
            status: SubsystemStatus::Fault,
            temperature_mk: 800_000,
            power_mw: 0,
            timestamp_s: 1_701_100_010,
        },
        SubsystemTelemetry {
            subsystem_id: 2,
            status: SubsystemStatus::Nominal,
            temperature_mk: 295_000,
            power_mw: 1_100,
            timestamp_s: 1_701_100_020,
        },
    ];
    let bytes = encode_to_vec(&telemetry).expect("encode fault telemetry");
    let (decoded, _): (Vec<SubsystemTelemetry>, usize) =
        decode_from_slice(&bytes).expect("decode fault telemetry");

    let fault_count = decoded
        .iter()
        .filter(|t| t.status == SubsystemStatus::Fault)
        .count();
    assert_eq!(fault_count, 1, "exactly one faulty subsystem expected");
    assert_eq!(decoded[1].subsystem_id, 1);
}

// --- Test 18: safe mode scenario roundtrip ---
#[test]
fn test_safe_mode_scenario() {
    let snapshot = MissionSnapshot {
        mission_id: 55_555,
        phase: MissionPhase::SurfaceOps,
        telemetry: vec![
            SubsystemTelemetry {
                subsystem_id: 0,
                status: SubsystemStatus::SafeMode,
                temperature_mk: 100_000,
                power_mw: 10,
                timestamp_s: 1_701_200_000,
            },
            SubsystemTelemetry {
                subsystem_id: 1,
                status: SubsystemStatus::Off,
                temperature_mk: 50_000,
                power_mw: 0,
                timestamp_s: 1_701_200_005,
            },
        ],
        pending_maneuvers: vec![],
        elapsed_s: 1_296_000,
    };
    let bytes = encode_to_vec(&snapshot).expect("encode safe mode scenario");
    let (decoded, _): (MissionSnapshot, usize) =
        decode_from_slice(&bytes).expect("decode safe mode scenario");
    assert_eq!(snapshot, decoded);
    assert_eq!(decoded.phase, MissionPhase::SurfaceOps);
    let safe_mode_count = decoded
        .telemetry
        .iter()
        .filter(|t| t.status == SubsystemStatus::SafeMode)
        .count();
    assert_eq!(safe_mode_count, 1);
}

// --- Test 19: ion thruster maneuver roundtrip ---
#[test]
fn test_ion_thruster_maneuver() {
    let cmd = ManeuverCommand {
        maneuver_id: 8888,
        propulsion: PropulsionType::IonThruster,
        delta_v_mms: 25_000,
        burn_duration_ms: 7_200_000,
        scheduled_at: 1_701_300_000,
    };
    let bytes = encode_to_vec(&cmd).expect("encode ion thruster maneuver");
    let (decoded, _): (ManeuverCommand, usize) =
        decode_from_slice(&bytes).expect("decode ion thruster maneuver");
    assert_eq!(cmd, decoded);
    assert_eq!(decoded.propulsion, PropulsionType::IonThruster);
    assert_eq!(decoded.burn_duration_ms, 7_200_000);
}

// --- Test 20: temperature extremes — near absolute zero and very high ---
#[test]
fn test_temperature_extremes() {
    // Near absolute zero: 2.7 K = 2700 mK (cosmic microwave background)
    let cold_telemetry = SubsystemTelemetry {
        subsystem_id: 200,
        status: SubsystemStatus::Nominal,
        temperature_mk: 2_700,
        power_mw: 1,
        timestamp_s: 1_701_400_000,
    };
    // Very high: 3000 K = 3_000_000 mK (thruster nozzle area)
    let hot_telemetry = SubsystemTelemetry {
        subsystem_id: 201,
        status: SubsystemStatus::Degraded,
        temperature_mk: 3_000_000,
        power_mw: 500_000,
        timestamp_s: 1_701_400_001,
    };

    let bytes_cold = encode_to_vec(&cold_telemetry).expect("encode cold telemetry");
    let (decoded_cold, _): (SubsystemTelemetry, usize) =
        decode_from_slice(&bytes_cold).expect("decode cold telemetry");
    assert_eq!(cold_telemetry, decoded_cold);
    assert_eq!(decoded_cold.temperature_mk, 2_700);

    let bytes_hot = encode_to_vec(&hot_telemetry).expect("encode hot telemetry");
    let (decoded_hot, _): (SubsystemTelemetry, usize) =
        decode_from_slice(&bytes_hot).expect("decode hot telemetry");
    assert_eq!(hot_telemetry, decoded_hot);
    assert_eq!(decoded_hot.temperature_mk, 3_000_000);
}

// --- Test 21: distinct discriminants for MissionPhase ---
#[test]
fn test_distinct_discriminants_mission_phase() {
    let variants = [
        MissionPhase::Launch,
        MissionPhase::AscendOrbit,
        MissionPhase::TransferOrbit,
        MissionPhase::Cruise,
        MissionPhase::OrbitInsertion,
        MissionPhase::Landing,
        MissionPhase::SurfaceOps,
        MissionPhase::Reentry,
    ];
    let encoded_variants: Vec<Vec<u8>> = variants
        .iter()
        .map(|v| encode_to_vec(v).expect("encode MissionPhase variant for discriminant check"))
        .collect();

    // Each variant must produce a distinct byte sequence
    for i in 0..encoded_variants.len() {
        for j in (i + 1)..encoded_variants.len() {
            assert_ne!(
                encoded_variants[i], encoded_variants[j],
                "MissionPhase variants at index {} and {} must have distinct encodings",
                i, j
            );
        }
    }
}

// --- Test 22: distinct discriminants for SubsystemStatus ---
#[test]
fn test_distinct_discriminants_subsystem_status() {
    let variants = [
        SubsystemStatus::Nominal,
        SubsystemStatus::Degraded,
        SubsystemStatus::Fault,
        SubsystemStatus::Off,
        SubsystemStatus::SafeMode,
    ];
    let encoded_variants: Vec<Vec<u8>> = variants
        .iter()
        .map(|v| encode_to_vec(v).expect("encode SubsystemStatus variant for discriminant check"))
        .collect();

    // Each variant must produce a distinct byte sequence
    for i in 0..encoded_variants.len() {
        for j in (i + 1)..encoded_variants.len() {
            assert_ne!(
                encoded_variants[i], encoded_variants[j],
                "SubsystemStatus variants at index {} and {} must have distinct encodings",
                i, j
            );
        }
    }
}

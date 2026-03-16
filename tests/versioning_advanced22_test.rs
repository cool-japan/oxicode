#![cfg(feature = "versioning")]

//! Digital twin / industrial IoT simulation — versioning feature tests.
//!
//! 22 #[test] functions covering physical asset modeling, state synchronization,
//! and sensor fusion using encode_versioned_value / decode_versioned_value.

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
use oxicode::versioning::Version;
use oxicode::{decode_versioned_value, encode_versioned_value, Decode, Encode};

// ── Domain types ─────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum AssetClass {
    Pump,
    Compressor,
    HeatExchanger,
    Valve,
    Turbine,
    Conveyor,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum TwinState {
    Synchronized,
    Drifting,
    Stale,
    Offline,
    Simulating,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SensorFusion {
    sensor_count: u8,
    confidence_pct: u8,
    lag_ms: u32,
    fused_value: i64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AssetTwinV1 {
    asset_id: u64,
    asset_class: AssetClass,
    state: TwinState,
    health_pct: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AssetTwinV2 {
    asset_id: u64,
    asset_class: AssetClass,
    state: TwinState,
    health_pct: u8,
    fusion: SensorFusion,
    maintenance_due_s: u64,
}

// ── Test 1: AssetTwinV1 at version 1.0.0 roundtrip ───────────────────────────
#[test]
fn test_asset_twin_v1_version_1_0_0_roundtrip() {
    let version = Version::new(1, 0, 0);
    let twin = AssetTwinV1 {
        asset_id: 1001,
        asset_class: AssetClass::Pump,
        state: TwinState::Synchronized,
        health_pct: 95,
    };
    let bytes = encode_versioned_value(&twin, version).expect("encode_versioned_value failed");
    let (decoded, ver, _consumed): (AssetTwinV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode_versioned_value failed");
    assert_eq!(decoded, twin);
    assert_eq!(ver, version);
}

// ── Test 2: AssetTwinV2 at version 2.0.0 roundtrip ───────────────────────────
#[test]
fn test_asset_twin_v2_version_2_0_0_roundtrip() {
    let version = Version::new(2, 0, 0);
    let twin = AssetTwinV2 {
        asset_id: 2002,
        asset_class: AssetClass::Compressor,
        state: TwinState::Synchronized,
        health_pct: 87,
        fusion: SensorFusion {
            sensor_count: 4,
            confidence_pct: 91,
            lag_ms: 50,
            fused_value: 1_234_567,
        },
        maintenance_due_s: 86400 * 30,
    };
    let bytes = encode_versioned_value(&twin, version).expect("encode_versioned_value failed");
    let (decoded, ver, _consumed): (AssetTwinV2, Version, usize) =
        decode_versioned_value(&bytes).expect("decode_versioned_value failed");
    assert_eq!(decoded, twin);
    assert_eq!(ver, version);
}

// ── Test 3: each AssetClass variant versioned ─────────────────────────────────
#[test]
fn test_each_asset_class_variant_versioned() {
    let version = Version::new(1, 0, 0);
    let classes = [
        AssetClass::Pump,
        AssetClass::Compressor,
        AssetClass::HeatExchanger,
        AssetClass::Valve,
        AssetClass::Turbine,
        AssetClass::Conveyor,
    ];
    // Encode and decode each variant independently
    let bytes_pump =
        encode_versioned_value(&AssetClass::Pump, version).expect("encode Pump failed");
    let bytes_compressor =
        encode_versioned_value(&AssetClass::Compressor, version).expect("encode Compressor failed");
    let bytes_heat = encode_versioned_value(&AssetClass::HeatExchanger, version)
        .expect("encode HeatExchanger failed");
    let bytes_valve =
        encode_versioned_value(&AssetClass::Valve, version).expect("encode Valve failed");
    let bytes_turbine =
        encode_versioned_value(&AssetClass::Turbine, version).expect("encode Turbine failed");
    let bytes_conveyor =
        encode_versioned_value(&AssetClass::Conveyor, version).expect("encode Conveyor failed");

    let encoded_list = [
        bytes_pump,
        bytes_compressor,
        bytes_heat,
        bytes_valve,
        bytes_turbine,
        bytes_conveyor,
    ];

    for (bytes, expected) in encoded_list.iter().zip(classes.iter()) {
        let (decoded, ver, _consumed): (AssetClass, Version, usize) =
            decode_versioned_value(bytes).expect("decode AssetClass failed");
        assert_eq!(&decoded, expected);
        assert_eq!(ver, version);
    }
}

// ── Test 4: each TwinState variant versioned ──────────────────────────────────
#[test]
fn test_each_twin_state_variant_versioned() {
    let version = Version::new(1, 0, 0);
    let states = [
        TwinState::Synchronized,
        TwinState::Drifting,
        TwinState::Stale,
        TwinState::Offline,
        TwinState::Simulating,
    ];
    for state in &states {
        // Use discriminant index to distinguish states in assertion
        let bytes = encode_versioned_value(state, version).expect("encode TwinState failed");
        let (decoded, ver, _consumed): (TwinState, Version, usize) =
            decode_versioned_value(&bytes).expect("decode TwinState failed");
        assert_eq!(&decoded, state);
        assert_eq!(ver, version);
    }
}

// ── Test 5: SensorFusion versioned ────────────────────────────────────────────
#[test]
fn test_sensor_fusion_versioned_roundtrip() {
    let version = Version::new(1, 1, 0);
    let fusion = SensorFusion {
        sensor_count: 8,
        confidence_pct: 78,
        lag_ms: 120,
        fused_value: -9_876_543,
    };
    let bytes = encode_versioned_value(&fusion, version).expect("encode SensorFusion failed");
    let (decoded, ver, _consumed): (SensorFusion, Version, usize) =
        decode_versioned_value(&bytes).expect("decode SensorFusion failed");
    assert_eq!(decoded, fusion);
    assert_eq!(ver, version);
}

// ── Test 6: version triple major/minor/patch preserved ───────────────────────
#[test]
fn test_version_triple_major_minor_patch_preserved() {
    let version = Version::new(3, 7, 19);
    let twin = AssetTwinV1 {
        asset_id: 555,
        asset_class: AssetClass::Valve,
        state: TwinState::Simulating,
        health_pct: 60,
    };
    let bytes = encode_versioned_value(&twin, version).expect("encode failed");
    let (_decoded, ver, _consumed): (AssetTwinV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode failed");
    assert_eq!(ver.major, 3);
    assert_eq!(ver.minor, 7);
    assert_eq!(ver.patch, 19);
    assert_eq!(ver, version);
}

// ── Test 7: v1.0.0 < v2.0.0 ordering ────────────────────────────────────────
#[test]
fn test_v1_less_than_v2_ordering() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    assert!(v1 < v2);
    assert!(v2 > v1);
    assert!(v2.is_breaking_change_from(&v1));
    assert!(!v1.is_compatible_with(&v2));
}

// ── Test 8: Vec<AssetTwinV1> versioned ────────────────────────────────────────
#[test]
fn test_vec_asset_twin_v1_versioned_roundtrip() {
    let version = Version::new(1, 0, 0);
    let twins = vec![
        AssetTwinV1 {
            asset_id: 101,
            asset_class: AssetClass::Pump,
            state: TwinState::Synchronized,
            health_pct: 100,
        },
        AssetTwinV1 {
            asset_id: 102,
            asset_class: AssetClass::Turbine,
            state: TwinState::Drifting,
            health_pct: 72,
        },
        AssetTwinV1 {
            asset_id: 103,
            asset_class: AssetClass::Conveyor,
            state: TwinState::Stale,
            health_pct: 45,
        },
    ];
    let bytes = encode_versioned_value(&twins, version).expect("encode Vec<AssetTwinV1> failed");
    let (decoded, ver, _consumed): (Vec<AssetTwinV1>, Version, usize) =
        decode_versioned_value(&bytes).expect("decode Vec<AssetTwinV1> failed");
    assert_eq!(decoded, twins);
    assert_eq!(ver, version);
    assert_eq!(decoded.len(), 3);
}

// ── Test 9: drifting state detection ─────────────────────────────────────────
#[test]
fn test_drifting_state_detection() {
    let version = Version::new(1, 0, 0);
    let twin = AssetTwinV1 {
        asset_id: 300,
        asset_class: AssetClass::HeatExchanger,
        state: TwinState::Drifting,
        health_pct: 80,
    };
    let bytes = encode_versioned_value(&twin, version).expect("encode drifting twin failed");
    let (decoded, ver, _consumed): (AssetTwinV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode drifting twin failed");
    assert_eq!(decoded.state, TwinState::Drifting);
    assert_eq!(decoded.asset_id, 300);
    assert_eq!(ver, version);
}

// ── Test 10: offline asset ────────────────────────────────────────────────────
#[test]
fn test_offline_asset_twin_v1() {
    let version = Version::new(1, 0, 0);
    let twin = AssetTwinV1 {
        asset_id: 404,
        asset_class: AssetClass::Compressor,
        state: TwinState::Offline,
        health_pct: 0,
    };
    let bytes = encode_versioned_value(&twin, version).expect("encode offline twin failed");
    let (decoded, ver, _consumed): (AssetTwinV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode offline twin failed");
    assert_eq!(decoded.state, TwinState::Offline);
    assert_eq!(decoded.health_pct, 0);
    assert_eq!(ver, version);
}

// ── Test 11: simulating state ─────────────────────────────────────────────────
#[test]
fn test_simulating_state_twin_v1() {
    let version = Version::new(1, 2, 0);
    let twin = AssetTwinV1 {
        asset_id: 999,
        asset_class: AssetClass::Turbine,
        state: TwinState::Simulating,
        health_pct: 100,
    };
    let bytes = encode_versioned_value(&twin, version).expect("encode simulating twin failed");
    let (decoded, ver, _consumed): (AssetTwinV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode simulating twin failed");
    assert_eq!(decoded.state, TwinState::Simulating);
    assert_eq!(decoded.asset_class, AssetClass::Turbine);
    assert_eq!(ver, version);
}

// ── Test 12: pump health degradation — v1 then v2 with sensor fusion ──────────
#[test]
fn test_pump_health_degradation_v1_to_v2_with_fusion() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);

    // First snapshot at v1: pump healthy
    let twin_v1 = AssetTwinV1 {
        asset_id: 500,
        asset_class: AssetClass::Pump,
        state: TwinState::Synchronized,
        health_pct: 92,
    };
    let bytes_v1 = encode_versioned_value(&twin_v1, v1).expect("encode v1 pump failed");
    let (decoded_v1, ver1, _): (AssetTwinV1, Version, usize) =
        decode_versioned_value(&bytes_v1).expect("decode v1 pump failed");
    assert_eq!(decoded_v1.health_pct, 92);
    assert_eq!(ver1, v1);

    // Second snapshot at v2: pump degraded, fusion data added
    let twin_v2 = AssetTwinV2 {
        asset_id: 500,
        asset_class: AssetClass::Pump,
        state: TwinState::Drifting,
        health_pct: 63,
        fusion: SensorFusion {
            sensor_count: 3,
            confidence_pct: 74,
            lag_ms: 200,
            fused_value: 4_500,
        },
        maintenance_due_s: 86400 * 7,
    };
    let bytes_v2 = encode_versioned_value(&twin_v2, v2).expect("encode v2 pump failed");
    let (decoded_v2, ver2, _): (AssetTwinV2, Version, usize) =
        decode_versioned_value(&bytes_v2).expect("decode v2 pump failed");
    assert_eq!(decoded_v2.health_pct, 63);
    assert!(decoded_v2.health_pct < decoded_v1.health_pct);
    assert_eq!(decoded_v2.state, TwinState::Drifting);
    assert_eq!(ver2, v2);
    assert!(ver2 > ver1);
}

// ── Test 13: turbine predictive maintenance ───────────────────────────────────
#[test]
fn test_turbine_predictive_maintenance_roundtrip() {
    let version = Version::new(2, 0, 0);
    // Turbine approaching maintenance threshold in 3 days
    let twin = AssetTwinV2 {
        asset_id: 7001,
        asset_class: AssetClass::Turbine,
        state: TwinState::Synchronized,
        health_pct: 71,
        fusion: SensorFusion {
            sensor_count: 12,
            confidence_pct: 96,
            lag_ms: 30,
            fused_value: 15_000_000,
        },
        maintenance_due_s: 86400 * 3,
    };
    let bytes = encode_versioned_value(&twin, version).expect("encode turbine failed");
    let (decoded, ver, _consumed): (AssetTwinV2, Version, usize) =
        decode_versioned_value(&bytes).expect("decode turbine failed");
    assert_eq!(decoded.asset_class, AssetClass::Turbine);
    assert_eq!(decoded.maintenance_due_s, 86400 * 3);
    assert_eq!(decoded.fusion.sensor_count, 12);
    assert_eq!(decoded.fusion.confidence_pct, 96);
    assert_eq!(ver, version);
}

// ── Test 14: compressor sync quality ─────────────────────────────────────────
#[test]
fn test_compressor_sync_quality_v2() {
    let version = Version::new(2, 1, 0);
    let twin = AssetTwinV2 {
        asset_id: 8080,
        asset_class: AssetClass::Compressor,
        state: TwinState::Synchronized,
        health_pct: 98,
        fusion: SensorFusion {
            sensor_count: 6,
            confidence_pct: 99,
            lag_ms: 5,
            fused_value: 760_000,
        },
        maintenance_due_s: 86400 * 90,
    };
    let bytes = encode_versioned_value(&twin, version).expect("encode compressor failed");
    let (decoded, ver, _consumed): (AssetTwinV2, Version, usize) =
        decode_versioned_value(&bytes).expect("decode compressor failed");
    assert_eq!(decoded.state, TwinState::Synchronized);
    assert_eq!(decoded.fusion.lag_ms, 5);
    assert_eq!(decoded.fusion.confidence_pct, 99);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 1);
}

// ── Test 15: zero lag sensor fusion ──────────────────────────────────────────
#[test]
fn test_zero_lag_sensor_fusion_roundtrip() {
    let version = Version::new(2, 0, 0);
    let fusion = SensorFusion {
        sensor_count: 1,
        confidence_pct: 85,
        lag_ms: 0,
        fused_value: 0,
    };
    let bytes = encode_versioned_value(&fusion, version).expect("encode zero-lag fusion failed");
    let (decoded, ver, _consumed): (SensorFusion, Version, usize) =
        decode_versioned_value(&bytes).expect("decode zero-lag fusion failed");
    assert_eq!(decoded.lag_ms, 0);
    assert_eq!(decoded.fused_value, 0);
    assert_eq!(ver, version);
}

// ── Test 16: perfect confidence (100%) ───────────────────────────────────────
#[test]
fn test_perfect_confidence_100_pct_fusion() {
    let version = Version::new(2, 0, 0);
    let twin = AssetTwinV2 {
        asset_id: 42,
        asset_class: AssetClass::Valve,
        state: TwinState::Synchronized,
        health_pct: 100,
        fusion: SensorFusion {
            sensor_count: 16,
            confidence_pct: 100,
            lag_ms: 1,
            fused_value: 1,
        },
        maintenance_due_s: u64::MAX,
    };
    let bytes = encode_versioned_value(&twin, version).expect("encode 100% confidence failed");
    let (decoded, ver, _consumed): (AssetTwinV2, Version, usize) =
        decode_versioned_value(&bytes).expect("decode 100% confidence failed");
    assert_eq!(decoded.fusion.confidence_pct, 100);
    assert_eq!(decoded.health_pct, 100);
    assert_eq!(decoded.maintenance_due_s, u64::MAX);
    assert_eq!(ver, version);
}

// ── Test 17: stale twin recovery ─────────────────────────────────────────────
#[test]
fn test_stale_twin_recovery_state_transition() {
    let version = Version::new(1, 0, 0);

    // Record stale state
    let stale_twin = AssetTwinV1 {
        asset_id: 6060,
        asset_class: AssetClass::HeatExchanger,
        state: TwinState::Stale,
        health_pct: 55,
    };
    let bytes_stale =
        encode_versioned_value(&stale_twin, version).expect("encode stale twin failed");
    let (decoded_stale, ver_stale, _): (AssetTwinV1, Version, usize) =
        decode_versioned_value(&bytes_stale).expect("decode stale twin failed");
    assert_eq!(decoded_stale.state, TwinState::Stale);
    assert_eq!(ver_stale, version);

    // Record recovered (synchronized) state
    let recovered_twin = AssetTwinV1 {
        asset_id: 6060,
        asset_class: AssetClass::HeatExchanger,
        state: TwinState::Synchronized,
        health_pct: 55,
    };
    let bytes_recovered =
        encode_versioned_value(&recovered_twin, version).expect("encode recovered twin failed");
    let (decoded_recovered, ver_recovered, _): (AssetTwinV1, Version, usize) =
        decode_versioned_value(&bytes_recovered).expect("decode recovered twin failed");
    assert_eq!(decoded_recovered.state, TwinState::Synchronized);
    assert_eq!(ver_recovered, version);
    // Same asset_id across both snapshots
    assert_eq!(decoded_stale.asset_id, decoded_recovered.asset_id);
}

// ── Test 18: multi-asset factory floor (5 assets) ────────────────────────────
#[test]
fn test_multi_asset_factory_floor_5_assets_versioned() {
    let version = Version::new(2, 0, 0);
    let assets: Vec<AssetTwinV2> = vec![
        AssetTwinV2 {
            asset_id: 1,
            asset_class: AssetClass::Pump,
            state: TwinState::Synchronized,
            health_pct: 95,
            fusion: SensorFusion {
                sensor_count: 2,
                confidence_pct: 88,
                lag_ms: 10,
                fused_value: 1000,
            },
            maintenance_due_s: 86400 * 60,
        },
        AssetTwinV2 {
            asset_id: 2,
            asset_class: AssetClass::Compressor,
            state: TwinState::Synchronized,
            health_pct: 90,
            fusion: SensorFusion {
                sensor_count: 4,
                confidence_pct: 92,
                lag_ms: 8,
                fused_value: 2000,
            },
            maintenance_due_s: 86400 * 45,
        },
        AssetTwinV2 {
            asset_id: 3,
            asset_class: AssetClass::HeatExchanger,
            state: TwinState::Drifting,
            health_pct: 70,
            fusion: SensorFusion {
                sensor_count: 3,
                confidence_pct: 75,
                lag_ms: 50,
                fused_value: 3000,
            },
            maintenance_due_s: 86400 * 10,
        },
        AssetTwinV2 {
            asset_id: 4,
            asset_class: AssetClass::Valve,
            state: TwinState::Stale,
            health_pct: 50,
            fusion: SensorFusion {
                sensor_count: 1,
                confidence_pct: 60,
                lag_ms: 500,
                fused_value: -100,
            },
            maintenance_due_s: 86400 * 2,
        },
        AssetTwinV2 {
            asset_id: 5,
            asset_class: AssetClass::Turbine,
            state: TwinState::Offline,
            health_pct: 0,
            fusion: SensorFusion {
                sensor_count: 0,
                confidence_pct: 0,
                lag_ms: 0,
                fused_value: 0,
            },
            maintenance_due_s: 0,
        },
    ];
    let bytes = encode_versioned_value(&assets, version).expect("encode factory floor failed");
    let (decoded, ver, _consumed): (Vec<AssetTwinV2>, Version, usize) =
        decode_versioned_value(&bytes).expect("decode factory floor failed");
    assert_eq!(decoded.len(), 5);
    assert_eq!(decoded, assets);
    assert_eq!(ver, version);
    // Verify degraded assets can be found
    let offline_count = decoded
        .iter()
        .filter(|a| a.state == TwinState::Offline)
        .count();
    assert_eq!(offline_count, 1);
}

// ── Test 19: patch for calibration fix (1.0.0 → 1.0.1) ──────────────────────
#[test]
fn test_patch_for_calibration_fix_version_bump() {
    let v_before_calibration = Version::new(1, 0, 0);
    let v_after_calibration = Version::new(1, 0, 1);

    let reading_before = SensorFusion {
        sensor_count: 4,
        confidence_pct: 81,
        lag_ms: 15,
        fused_value: 9_999, // slightly off due to calibration bug
    };
    let reading_after = SensorFusion {
        sensor_count: 4,
        confidence_pct: 81,
        lag_ms: 15,
        fused_value: 10_000, // corrected value after calibration patch
    };

    let bytes_before = encode_versioned_value(&reading_before, v_before_calibration)
        .expect("encode pre-calibration failed");
    let bytes_after = encode_versioned_value(&reading_after, v_after_calibration)
        .expect("encode post-calibration failed");

    let (decoded_before, ver_before, _): (SensorFusion, Version, usize) =
        decode_versioned_value(&bytes_before).expect("decode pre-calibration failed");
    let (decoded_after, ver_after, _): (SensorFusion, Version, usize) =
        decode_versioned_value(&bytes_after).expect("decode post-calibration failed");

    assert_eq!(decoded_before.fused_value, 9_999);
    assert_eq!(decoded_after.fused_value, 10_000);
    assert!(ver_after.is_patch_update_from(&ver_before));
    assert_eq!(ver_after.patch, 1);
    assert_eq!(ver_before.patch, 0);
}

// ── Test 20: minor for new sensor type (1.0.0 → 1.1.0) ──────────────────────
#[test]
fn test_minor_version_for_new_sensor_type_added() {
    let v_without_extra_sensors = Version::new(1, 0, 0);
    let v_with_extra_sensors = Version::new(1, 1, 0);

    let basic_fusion = SensorFusion {
        sensor_count: 2,
        confidence_pct: 85,
        lag_ms: 20,
        fused_value: 5_000,
    };
    let extended_fusion = SensorFusion {
        sensor_count: 6, // additional sensor type added in 1.1.0
        confidence_pct: 93,
        lag_ms: 12,
        fused_value: 5_100,
    };

    let bytes_basic = encode_versioned_value(&basic_fusion, v_without_extra_sensors)
        .expect("encode basic fusion failed");
    let bytes_extended = encode_versioned_value(&extended_fusion, v_with_extra_sensors)
        .expect("encode extended fusion failed");

    let (decoded_basic, ver_basic, _): (SensorFusion, Version, usize) =
        decode_versioned_value(&bytes_basic).expect("decode basic fusion failed");
    let (decoded_extended, ver_extended, _): (SensorFusion, Version, usize) =
        decode_versioned_value(&bytes_extended).expect("decode extended fusion failed");

    assert_eq!(decoded_basic.sensor_count, 2);
    assert_eq!(decoded_extended.sensor_count, 6);
    assert!(ver_extended.is_minor_update_from(&ver_basic));
    assert_eq!(ver_basic.minor, 0);
    assert_eq!(ver_extended.minor, 1);
}

// ── Test 21: consumed bytes check ────────────────────────────────────────────
#[test]
fn test_consumed_bytes_reflect_full_versioned_payload() {
    let version = Version::new(1, 0, 0);
    let twin = AssetTwinV1 {
        asset_id: 111,
        asset_class: AssetClass::Conveyor,
        state: TwinState::Synchronized,
        health_pct: 88,
    };
    let bytes =
        encode_versioned_value(&twin, version).expect("encode for consumed-bytes test failed");
    let total_len = bytes.len();
    let (_decoded, _ver, consumed): (AssetTwinV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode for consumed-bytes test failed");
    // consumed now includes the full versioned envelope (header + payload).
    assert_eq!(
        consumed, total_len,
        "consumed must equal the full encoded length"
    );
}

// ── Test 22: maintenance scheduling roundtrip ─────────────────────────────────
#[test]
fn test_maintenance_scheduling_roundtrip_multiple_assets() {
    let version = Version::new(2, 0, 0);

    // Three assets with different maintenance windows
    let urgent = AssetTwinV2 {
        asset_id: 901,
        asset_class: AssetClass::Pump,
        state: TwinState::Drifting,
        health_pct: 40,
        fusion: SensorFusion {
            sensor_count: 2,
            confidence_pct: 70,
            lag_ms: 300,
            fused_value: -50,
        },
        maintenance_due_s: 3600, // 1 hour
    };
    let scheduled = AssetTwinV2 {
        asset_id: 902,
        asset_class: AssetClass::HeatExchanger,
        state: TwinState::Synchronized,
        health_pct: 78,
        fusion: SensorFusion {
            sensor_count: 5,
            confidence_pct: 89,
            lag_ms: 25,
            fused_value: 25_000,
        },
        maintenance_due_s: 86400 * 14, // 2 weeks
    };
    let deferred = AssetTwinV2 {
        asset_id: 903,
        asset_class: AssetClass::Compressor,
        state: TwinState::Synchronized,
        health_pct: 97,
        fusion: SensorFusion {
            sensor_count: 8,
            confidence_pct: 99,
            lag_ms: 3,
            fused_value: 500_000,
        },
        maintenance_due_s: 86400 * 180, // 6 months
    };

    let bytes_urgent =
        encode_versioned_value(&urgent, version).expect("encode urgent asset failed");
    let bytes_scheduled =
        encode_versioned_value(&scheduled, version).expect("encode scheduled asset failed");
    let bytes_deferred =
        encode_versioned_value(&deferred, version).expect("encode deferred asset failed");

    let (dec_urgent, ver_u, _): (AssetTwinV2, Version, usize) =
        decode_versioned_value(&bytes_urgent).expect("decode urgent asset failed");
    let (dec_scheduled, ver_s, _): (AssetTwinV2, Version, usize) =
        decode_versioned_value(&bytes_scheduled).expect("decode scheduled asset failed");
    let (dec_deferred, ver_d, _): (AssetTwinV2, Version, usize) =
        decode_versioned_value(&bytes_deferred).expect("decode deferred asset failed");

    assert_eq!(dec_urgent.maintenance_due_s, 3600);
    assert_eq!(dec_scheduled.maintenance_due_s, 86400 * 14);
    assert_eq!(dec_deferred.maintenance_due_s, 86400 * 180);

    // All encoded at the same version
    assert_eq!(ver_u, version);
    assert_eq!(ver_s, version);
    assert_eq!(ver_d, version);

    // Maintenance urgency ordering
    assert!(dec_urgent.maintenance_due_s < dec_scheduled.maintenance_due_s);
    assert!(dec_scheduled.maintenance_due_s < dec_deferred.maintenance_due_s);
}

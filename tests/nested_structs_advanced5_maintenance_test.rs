//! Predictive-maintenance/asset-lifecycle/OPC-UA-focused tests for nested_structs_advanced5 (split from nested_structs_advanced5_test.rs).

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

// ---------------------------------------------------------------------------
// Predictive maintenance models
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VibrationReading {
    axis: String,
    rms_velocity: f64,
    peak_acceleration: f64,
    dominant_freq_hz: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TemperatureReading {
    sensor_id: String,
    value_celsius: f64,
    location_on_asset: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AcousticReading {
    db_level: f64,
    frequency_spectrum: Vec<f64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MaintenanceModel {
    asset_id: u64,
    model_version: String,
    vibration: Vec<VibrationReading>,
    temperatures: Vec<TemperatureReading>,
    acoustic: AcousticReading,
    remaining_useful_life_hours: f64,
    confidence: f64,
}

// ---------------------------------------------------------------------------
// OPC UA node hierarchy
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OpcUaVariable {
    browse_name: String,
    node_id: String,
    data_type: String,
    value_as_f64: f64,
    writable: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OpcUaObject {
    browse_name: String,
    node_id: String,
    variables: Vec<OpcUaVariable>,
    children: Vec<OpcUaObject>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OpcUaNamespace {
    uri: String,
    index: u16,
    root_objects: Vec<OpcUaObject>,
}

// ---------------------------------------------------------------------------
// Asset lifecycle states
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AssetEvent {
    event_type: String,
    timestamp_ms: u64,
    description: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AssetLifecycle {
    asset_id: u64,
    asset_name: String,
    current_state: String,
    install_date: String,
    history: Vec<AssetEvent>,
    maintenance_model: Option<MaintenanceModel>,
}

// ===========================================================================
// Helper constructors
// ===========================================================================

fn roundtrip<T: Encode + Decode + std::fmt::Debug + PartialEq>(val: &T, ctx: &str) {
    let bytes = encode_to_vec(val).unwrap_or_else(|_| panic!("encode {ctx}"));
    let (decoded, _): (T, usize) =
        decode_from_slice(&bytes).unwrap_or_else(|_| panic!("decode {ctx}"));
    assert_eq!(val, &decoded, "roundtrip failed for {ctx}");
}

// ===========================================================================
// Tests
// ===========================================================================

// Test 3: Predictive maintenance model with vibration, temperature, and acoustic data
#[test]
fn test_predictive_maintenance_model_roundtrip() {
    let model = MaintenanceModel {
        asset_id: 42,
        model_version: "v3.1.0".to_string(),
        vibration: vec![
            VibrationReading {
                axis: "X".to_string(),
                rms_velocity: 2.4,
                peak_acceleration: 9.8,
                dominant_freq_hz: 120.0,
            },
            VibrationReading {
                axis: "Y".to_string(),
                rms_velocity: 1.8,
                peak_acceleration: 7.2,
                dominant_freq_hz: 60.0,
            },
            VibrationReading {
                axis: "Z".to_string(),
                rms_velocity: 3.1,
                peak_acceleration: 12.5,
                dominant_freq_hz: 240.0,
            },
        ],
        temperatures: vec![
            TemperatureReading {
                sensor_id: "T-001".to_string(),
                value_celsius: 72.5,
                location_on_asset: "bearing_drive_end".to_string(),
            },
            TemperatureReading {
                sensor_id: "T-002".to_string(),
                value_celsius: 68.3,
                location_on_asset: "bearing_non_drive_end".to_string(),
            },
        ],
        acoustic: AcousticReading {
            db_level: 85.2,
            frequency_spectrum: vec![0.1, 0.4, 0.9, 1.2, 0.7, 0.3],
        },
        remaining_useful_life_hours: 1200.5,
        confidence: 0.87,
    };
    roundtrip(&model, "predictive maintenance model");
}

// Test 4: OPC UA namespace with nested object hierarchy (3 levels deep)
#[test]
fn test_opcua_namespace_hierarchy_roundtrip() {
    let ns = OpcUaNamespace {
        uri: "urn:factory:opcua:plant-a".to_string(),
        index: 2,
        root_objects: vec![OpcUaObject {
            browse_name: "ProductionLine1".to_string(),
            node_id: "ns=2;s=Line1".to_string(),
            variables: vec![OpcUaVariable {
                browse_name: "LineStatus".to_string(),
                node_id: "ns=2;s=Line1.Status".to_string(),
                data_type: "Int32".to_string(),
                value_as_f64: 1.0,
                writable: false,
            }],
            children: vec![OpcUaObject {
                browse_name: "Station1".to_string(),
                node_id: "ns=2;s=Line1.St1".to_string(),
                variables: vec![OpcUaVariable {
                    browse_name: "CycleTime".to_string(),
                    node_id: "ns=2;s=Line1.St1.CT".to_string(),
                    data_type: "Double".to_string(),
                    value_as_f64: 45.3,
                    writable: false,
                }],
                children: vec![OpcUaObject {
                    browse_name: "Motor1".to_string(),
                    node_id: "ns=2;s=Line1.St1.M1".to_string(),
                    variables: vec![
                        OpcUaVariable {
                            browse_name: "Speed".to_string(),
                            node_id: "ns=2;s=Line1.St1.M1.Speed".to_string(),
                            data_type: "Double".to_string(),
                            value_as_f64: 1750.0,
                            writable: true,
                        },
                        OpcUaVariable {
                            browse_name: "Current".to_string(),
                            node_id: "ns=2;s=Line1.St1.M1.Current".to_string(),
                            data_type: "Float".to_string(),
                            value_as_f64: 14.2,
                            writable: false,
                        },
                    ],
                    children: vec![],
                }],
            }],
        }],
    };
    roundtrip(&ns, "OPC UA namespace");
}

// Test 11: Asset lifecycle with embedded maintenance model
#[test]
fn test_asset_lifecycle_roundtrip() {
    let lifecycle = AssetLifecycle {
        asset_id: 1001,
        asset_name: "CNC-Mill-07".to_string(),
        current_state: "operational".to_string(),
        install_date: "2022-06-15".to_string(),
        history: vec![
            AssetEvent {
                event_type: "installed".to_string(),
                timestamp_ms: 1655308800000,
                description: "Initial installation and commissioning".to_string(),
            },
            AssetEvent {
                event_type: "maintenance".to_string(),
                timestamp_ms: 1670000000000,
                description: "Spindle bearing replacement".to_string(),
            },
            AssetEvent {
                event_type: "upgrade".to_string(),
                timestamp_ms: 1700000000000,
                description: "Controller firmware update to v5.2".to_string(),
            },
        ],
        maintenance_model: Some(MaintenanceModel {
            asset_id: 1001,
            model_version: "v2.0".to_string(),
            vibration: vec![VibrationReading {
                axis: "Z".to_string(),
                rms_velocity: 1.9,
                peak_acceleration: 6.5,
                dominant_freq_hz: 180.0,
            }],
            temperatures: vec![TemperatureReading {
                sensor_id: "T-SPINDLE".to_string(),
                value_celsius: 55.0,
                location_on_asset: "spindle_housing".to_string(),
            }],
            acoustic: AcousticReading {
                db_level: 78.0,
                frequency_spectrum: vec![0.2, 0.5, 0.8, 0.6, 0.3],
            },
            remaining_useful_life_hours: 3500.0,
            confidence: 0.92,
        }),
    };
    roundtrip(&lifecycle, "asset lifecycle");
}

// Test 20: Asset lifecycle with no maintenance model
#[test]
fn test_asset_lifecycle_no_model_roundtrip() {
    let lifecycle = AssetLifecycle {
        asset_id: 2002,
        asset_name: "Conveyor-Belt-12".to_string(),
        current_state: "decommissioned".to_string(),
        install_date: "2018-01-10".to_string(),
        history: vec![
            AssetEvent {
                event_type: "installed".to_string(),
                timestamp_ms: 1515542400000,
                description: "Installed on Line-3".to_string(),
            },
            AssetEvent {
                event_type: "decommissioned".to_string(),
                timestamp_ms: 1700000000000,
                description: "Replaced by higher-capacity belt".to_string(),
            },
        ],
        maintenance_model: None,
    };
    roundtrip(&lifecycle, "asset lifecycle no model");
}

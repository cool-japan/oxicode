#![cfg(feature = "versioning")]
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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use oxicode::{decode_versioned_value, encode_versioned_value};

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GridZone {
    Residential,
    Commercial,
    Industrial,
    Agricultural,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EnergySource {
    Solar,
    Wind,
    Gas,
    Coal,
    Nuclear,
    Hydro,
    Battery,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DemandResponseEvent {
    PeakShave,
    ValleyFill,
    EmergencyReduction,
    PriceResponse,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MeterStatus {
    Active,
    Tampered,
    Offline,
    Disconnected,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SmartMeter {
    meter_id: u64,
    household_id: u64,
    zone: GridZone,
    kwh_x100: u64,
    timestamp: u64,
    status: MeterStatus,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GridNode {
    node_id: u32,
    name: String,
    voltage_v_x10: u32,
    frequency_hz_x100: u32,
    load_mw_x100: u32,
    source: EnergySource,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DemandResponseSignal {
    signal_id: u64,
    zone: GridZone,
    event: DemandResponseEvent,
    start_time: u64,
    duration_min: u32,
    reduction_target_kw: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BatteryStorage {
    battery_id: u32,
    node_id: u32,
    capacity_kwh_x100: u32,
    charge_pct: u8,
    max_power_kw: u32,
    charging: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EnergyForecast {
    forecast_id: u64,
    zone: GridZone,
    timestamp: u64,
    periods: Vec<u32>,
}

// --- Test 1: SmartMeter v1.0.0 encode/decode roundtrip ---
#[test]
fn test_smart_meter_versioned_v1() {
    let meter = SmartMeter {
        meter_id: 100001,
        household_id: 555,
        zone: GridZone::Residential,
        kwh_x100: 34250,
        timestamp: 1_700_000_000,
        status: MeterStatus::Active,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&meter, ver).expect("encode SmartMeter v1.0.0");
    let (decoded, decoded_ver, consumed) =
        decode_versioned_value::<SmartMeter>(&bytes).expect("decode SmartMeter v1.0.0");
    assert_eq!(decoded, meter);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 0);
    assert_eq!(decoded_ver.patch, 0);
    assert_eq!(consumed, bytes.len());
}

// --- Test 2: SmartMeter v2.0.0 version fields ---
#[test]
fn test_smart_meter_versioned_v2() {
    let meter = SmartMeter {
        meter_id: 200002,
        household_id: 1024,
        zone: GridZone::Commercial,
        kwh_x100: 98765,
        timestamp: 1_710_000_000,
        status: MeterStatus::Tampered,
    };
    let ver = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&meter, ver).expect("encode SmartMeter v2.0.0");
    let (decoded, decoded_ver, _consumed) =
        decode_versioned_value::<SmartMeter>(&bytes).expect("decode SmartMeter v2.0.0");
    assert_eq!(decoded, meter);
    assert_eq!(decoded_ver.major, 2);
    assert_eq!(decoded_ver.minor, 0);
    assert_eq!(decoded_ver.patch, 0);
}

// --- Test 3: SmartMeter v1.5.2 patch version ---
#[test]
fn test_smart_meter_versioned_v1_5_2() {
    let meter = SmartMeter {
        meter_id: 300003,
        household_id: 777,
        zone: GridZone::Industrial,
        kwh_x100: 500000,
        timestamp: 1_720_000_000,
        status: MeterStatus::Offline,
    };
    let ver = Version::new(1, 5, 2);
    let bytes = encode_versioned_value(&meter, ver).expect("encode SmartMeter v1.5.2");
    let (decoded, decoded_ver, _consumed) =
        decode_versioned_value::<SmartMeter>(&bytes).expect("decode SmartMeter v1.5.2");
    assert_eq!(decoded, meter);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 5);
    assert_eq!(decoded_ver.patch, 2);
}

// --- Test 4: GridNode Solar source v1.0.0 ---
#[test]
fn test_grid_node_solar_v1() {
    let node = GridNode {
        node_id: 1,
        name: "SubstationAlpha".to_string(),
        voltage_v_x10: 1200000,
        frequency_hz_x100: 5000,
        load_mw_x100: 45000,
        source: EnergySource::Solar,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&node, ver).expect("encode GridNode Solar v1.0.0");
    let (decoded, decoded_ver, consumed) =
        decode_versioned_value::<GridNode>(&bytes).expect("decode GridNode Solar v1.0.0");
    assert_eq!(decoded, node);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 0);
    assert_eq!(consumed, bytes.len());
}

// --- Test 5: GridNode Wind source v2.0.0 ---
#[test]
fn test_grid_node_wind_v2() {
    let node = GridNode {
        node_id: 2,
        name: "WindFarm_North".to_string(),
        voltage_v_x10: 660000,
        frequency_hz_x100: 5000,
        load_mw_x100: 120000,
        source: EnergySource::Wind,
    };
    let ver = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&node, ver).expect("encode GridNode Wind v2.0.0");
    let (decoded, decoded_ver, _consumed) =
        decode_versioned_value::<GridNode>(&bytes).expect("decode GridNode Wind v2.0.0");
    assert_eq!(decoded, node);
    assert_eq!(decoded_ver.major, 2);
    assert_eq!(decoded_ver.patch, 0);
}

// --- Test 6: GridNode Nuclear source v1.5.2 ---
#[test]
fn test_grid_node_nuclear_v1_5_2() {
    let node = GridNode {
        node_id: 3,
        name: "NuclearPlantBeta".to_string(),
        voltage_v_x10: 4000000,
        frequency_hz_x100: 5001,
        load_mw_x100: 900000,
        source: EnergySource::Nuclear,
    };
    let ver = Version::new(1, 5, 2);
    let bytes = encode_versioned_value(&node, ver).expect("encode GridNode Nuclear v1.5.2");
    let (decoded, decoded_ver, consumed) =
        decode_versioned_value::<GridNode>(&bytes).expect("decode GridNode Nuclear v1.5.2");
    assert_eq!(decoded, node);
    assert_eq!(decoded_ver.minor, 5);
    assert_eq!(decoded_ver.patch, 2);
    assert_eq!(consumed, bytes.len());
}

// --- Test 7: DemandResponseSignal PeakShave v1.0.0 ---
#[test]
fn test_demand_response_peak_shave_v1() {
    let signal = DemandResponseSignal {
        signal_id: 9001,
        zone: GridZone::Commercial,
        event: DemandResponseEvent::PeakShave,
        start_time: 1_700_100_000,
        duration_min: 60,
        reduction_target_kw: 5000,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&signal, ver).expect("encode DRSignal PeakShave v1.0.0");
    let (decoded, decoded_ver, consumed) = decode_versioned_value::<DemandResponseSignal>(&bytes)
        .expect("decode DRSignal PeakShave v1.0.0");
    assert_eq!(decoded, signal);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 0);
    assert_eq!(decoded_ver.patch, 0);
    assert_eq!(consumed, bytes.len());
}

// --- Test 8: DemandResponseSignal EmergencyReduction v2.0.0 ---
#[test]
fn test_demand_response_emergency_v2() {
    let signal = DemandResponseSignal {
        signal_id: 9002,
        zone: GridZone::Industrial,
        event: DemandResponseEvent::EmergencyReduction,
        start_time: 1_710_200_000,
        duration_min: 120,
        reduction_target_kw: 50000,
    };
    let ver = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&signal, ver).expect("encode DRSignal Emergency v2.0.0");
    let (decoded, decoded_ver, _consumed) = decode_versioned_value::<DemandResponseSignal>(&bytes)
        .expect("decode DRSignal Emergency v2.0.0");
    assert_eq!(decoded, signal);
    assert_eq!(decoded_ver.major, 2);
    assert_eq!(decoded_ver.minor, 0);
}

// --- Test 9: DemandResponseSignal ValleyFill v1.5.2 ---
#[test]
fn test_demand_response_valley_fill_v1_5_2() {
    let signal = DemandResponseSignal {
        signal_id: 9003,
        zone: GridZone::Residential,
        event: DemandResponseEvent::ValleyFill,
        start_time: 1_720_300_000,
        duration_min: 30,
        reduction_target_kw: 200,
    };
    let ver = Version::new(1, 5, 2);
    let bytes = encode_versioned_value(&signal, ver).expect("encode DRSignal ValleyFill v1.5.2");
    let (decoded, decoded_ver, consumed) = decode_versioned_value::<DemandResponseSignal>(&bytes)
        .expect("decode DRSignal ValleyFill v1.5.2");
    assert_eq!(decoded, signal);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 5);
    assert_eq!(decoded_ver.patch, 2);
    assert_eq!(consumed, bytes.len());
}

// --- Test 10: BatteryStorage charging v1.0.0 ---
#[test]
fn test_battery_storage_charging_v1() {
    let battery = BatteryStorage {
        battery_id: 501,
        node_id: 1,
        capacity_kwh_x100: 100000,
        charge_pct: 85,
        max_power_kw: 5000,
        charging: true,
    };
    let ver = Version::new(1, 0, 0);
    let bytes =
        encode_versioned_value(&battery, ver).expect("encode BatteryStorage charging v1.0.0");
    let (decoded, decoded_ver, consumed) = decode_versioned_value::<BatteryStorage>(&bytes)
        .expect("decode BatteryStorage charging v1.0.0");
    assert_eq!(decoded, battery);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 0);
    assert_eq!(decoded_ver.patch, 0);
    assert_eq!(consumed, bytes.len());
}

// --- Test 11: BatteryStorage discharging v2.0.0 ---
#[test]
fn test_battery_storage_discharging_v2() {
    let battery = BatteryStorage {
        battery_id: 502,
        node_id: 2,
        capacity_kwh_x100: 250000,
        charge_pct: 42,
        max_power_kw: 10000,
        charging: false,
    };
    let ver = Version::new(2, 0, 0);
    let bytes =
        encode_versioned_value(&battery, ver).expect("encode BatteryStorage discharging v2.0.0");
    let (decoded, decoded_ver, _consumed) = decode_versioned_value::<BatteryStorage>(&bytes)
        .expect("decode BatteryStorage discharging v2.0.0");
    assert_eq!(decoded, battery);
    assert_eq!(decoded_ver.major, 2);
    assert_eq!(decoded_ver.patch, 0);
    assert!(!decoded.charging);
}

// --- Test 12: BatteryStorage zero charge v1.5.2 ---
#[test]
fn test_battery_storage_zero_charge_v1_5_2() {
    let battery = BatteryStorage {
        battery_id: 503,
        node_id: 3,
        capacity_kwh_x100: 50000,
        charge_pct: 0,
        max_power_kw: 2000,
        charging: false,
    };
    let ver = Version::new(1, 5, 2);
    let bytes = encode_versioned_value(&battery, ver).expect("encode BatteryStorage zero v1.5.2");
    let (decoded, decoded_ver, consumed) = decode_versioned_value::<BatteryStorage>(&bytes)
        .expect("decode BatteryStorage zero v1.5.2");
    assert_eq!(decoded, battery);
    assert_eq!(decoded_ver.minor, 5);
    assert_eq!(decoded_ver.patch, 2);
    assert_eq!(consumed, bytes.len());
}

// --- Test 13: EnergyForecast short periods v1.0.0 ---
#[test]
fn test_energy_forecast_short_v1() {
    let forecast = EnergyForecast {
        forecast_id: 70001,
        zone: GridZone::Residential,
        timestamp: 1_700_500_000,
        periods: vec![150, 200, 175, 220, 195, 160],
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&forecast, ver).expect("encode EnergyForecast short v1.0.0");
    let (decoded, decoded_ver, consumed) = decode_versioned_value::<EnergyForecast>(&bytes)
        .expect("decode EnergyForecast short v1.0.0");
    assert_eq!(decoded, forecast);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 0);
    assert_eq!(decoded_ver.patch, 0);
    assert_eq!(consumed, bytes.len());
}

// --- Test 14: EnergyForecast full 24 periods v2.0.0 ---
#[test]
fn test_energy_forecast_full_day_v2() {
    let periods: Vec<u32> = (0..24).map(|h| 100 + h * 15).collect();
    let forecast = EnergyForecast {
        forecast_id: 70002,
        zone: GridZone::Commercial,
        timestamp: 1_710_600_000,
        periods,
    };
    let ver = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&forecast, ver).expect("encode EnergyForecast 24h v2.0.0");
    let (decoded, decoded_ver, consumed) =
        decode_versioned_value::<EnergyForecast>(&bytes).expect("decode EnergyForecast 24h v2.0.0");
    assert_eq!(decoded.periods.len(), 24);
    assert_eq!(decoded, forecast);
    assert_eq!(decoded_ver.major, 2);
    assert_eq!(consumed, bytes.len());
}

// --- Test 15: EnergyForecast empty periods v1.5.2 ---
#[test]
fn test_energy_forecast_empty_periods_v1_5_2() {
    let forecast = EnergyForecast {
        forecast_id: 70003,
        zone: GridZone::Agricultural,
        timestamp: 1_720_700_000,
        periods: vec![],
    };
    let ver = Version::new(1, 5, 2);
    let bytes = encode_versioned_value(&forecast, ver).expect("encode EnergyForecast empty v1.5.2");
    let (decoded, decoded_ver, consumed) = decode_versioned_value::<EnergyForecast>(&bytes)
        .expect("decode EnergyForecast empty v1.5.2");
    assert_eq!(decoded, forecast);
    assert!(decoded.periods.is_empty());
    assert_eq!(decoded_ver.minor, 5);
    assert_eq!(decoded_ver.patch, 2);
    assert_eq!(consumed, bytes.len());
}

// --- Test 16: SmartMeter Disconnected status plain encode/decode ---
#[test]
fn test_smart_meter_disconnected_plain_encode() {
    let meter = SmartMeter {
        meter_id: 400004,
        household_id: 9999,
        zone: GridZone::Agricultural,
        kwh_x100: 0,
        timestamp: 1_730_000_000,
        status: MeterStatus::Disconnected,
    };
    let bytes = encode_to_vec(&meter).expect("encode SmartMeter plain");
    let (decoded, consumed) =
        decode_from_slice::<SmartMeter>(&bytes).expect("decode SmartMeter plain");
    assert_eq!(decoded, meter);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.status, MeterStatus::Disconnected);
}

// --- Test 17: Vec<SmartMeter> versioned collection v1.0.0 ---
#[test]
fn test_vec_smart_meters_versioned_v1() {
    let meters = vec![
        SmartMeter {
            meter_id: 1,
            household_id: 10,
            zone: GridZone::Residential,
            kwh_x100: 10000,
            timestamp: 1_700_000_001,
            status: MeterStatus::Active,
        },
        SmartMeter {
            meter_id: 2,
            household_id: 20,
            zone: GridZone::Commercial,
            kwh_x100: 30000,
            timestamp: 1_700_000_002,
            status: MeterStatus::Offline,
        },
        SmartMeter {
            meter_id: 3,
            household_id: 30,
            zone: GridZone::Industrial,
            kwh_x100: 80000,
            timestamp: 1_700_000_003,
            status: MeterStatus::Active,
        },
    ];
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&meters, ver).expect("encode Vec<SmartMeter> v1.0.0");
    let (decoded, decoded_ver, consumed) =
        decode_versioned_value::<Vec<SmartMeter>>(&bytes).expect("decode Vec<SmartMeter> v1.0.0");
    assert_eq!(decoded, meters);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 0);
    assert_eq!(decoded_ver.patch, 0);
    assert_eq!(consumed, bytes.len());
}

// --- Test 18: Vec<GridNode> versioned collection v2.0.0 ---
#[test]
fn test_vec_grid_nodes_versioned_v2() {
    let nodes = vec![
        GridNode {
            node_id: 10,
            name: "NodeA".to_string(),
            voltage_v_x10: 1100000,
            frequency_hz_x100: 5000,
            load_mw_x100: 20000,
            source: EnergySource::Gas,
        },
        GridNode {
            node_id: 11,
            name: "NodeB".to_string(),
            voltage_v_x10: 2200000,
            frequency_hz_x100: 4999,
            load_mw_x100: 60000,
            source: EnergySource::Hydro,
        },
    ];
    let ver = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&nodes, ver).expect("encode Vec<GridNode> v2.0.0");
    let (decoded, decoded_ver, consumed) =
        decode_versioned_value::<Vec<GridNode>>(&bytes).expect("decode Vec<GridNode> v2.0.0");
    assert_eq!(decoded, nodes);
    assert_eq!(decoded_ver.major, 2);
    assert_eq!(consumed, bytes.len());
}

// --- Test 19: GridNode Battery source — consumed bytes non-zero ---
#[test]
fn test_grid_node_battery_source_consumed_bytes() {
    let node = GridNode {
        node_id: 99,
        name: "BatteryHub".to_string(),
        voltage_v_x10: 480000,
        frequency_hz_x100: 5000,
        load_mw_x100: 8000,
        source: EnergySource::Battery,
    };
    let ver = Version::new(1, 5, 2);
    let bytes = encode_versioned_value(&node, ver).expect("encode GridNode Battery v1.5.2");
    assert!(!bytes.is_empty());
    let (decoded, decoded_ver, consumed) =
        decode_versioned_value::<GridNode>(&bytes).expect("decode GridNode Battery v1.5.2");
    assert_eq!(decoded, node);
    assert_eq!(decoded_ver.patch, 2);
    assert!(consumed > 0);
    assert_eq!(consumed, bytes.len());
}

// --- Test 20: DemandResponseSignal PriceResponse Agricultural v1.0.0 ---
#[test]
fn test_demand_response_price_response_agricultural_v1() {
    let signal = DemandResponseSignal {
        signal_id: 8888,
        zone: GridZone::Agricultural,
        event: DemandResponseEvent::PriceResponse,
        start_time: 1_705_000_000,
        duration_min: 45,
        reduction_target_kw: 300,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&signal, ver).expect("encode DRSignal PriceResponse v1.0.0");
    let (decoded, decoded_ver, consumed) = decode_versioned_value::<DemandResponseSignal>(&bytes)
        .expect("decode DRSignal PriceResponse v1.0.0");
    assert_eq!(decoded, signal);
    assert_eq!(decoded.zone, GridZone::Agricultural);
    assert_eq!(decoded.event, DemandResponseEvent::PriceResponse);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 0);
    assert_eq!(decoded_ver.patch, 0);
    assert_eq!(consumed, bytes.len());
}

// --- Test 21: EnergyForecast Industrial 12-period v1.5.2 ---
#[test]
fn test_energy_forecast_industrial_12_periods_v1_5_2() {
    let forecast = EnergyForecast {
        forecast_id: 80001,
        zone: GridZone::Industrial,
        timestamp: 1_715_000_000,
        periods: vec![500, 520, 490, 610, 700, 680, 720, 690, 640, 600, 560, 510],
    };
    let ver = Version::new(1, 5, 2);
    let bytes =
        encode_versioned_value(&forecast, ver).expect("encode EnergyForecast 12-period v1.5.2");
    let (decoded, decoded_ver, consumed) = decode_versioned_value::<EnergyForecast>(&bytes)
        .expect("decode EnergyForecast 12-period v1.5.2");
    assert_eq!(decoded.periods.len(), 12);
    assert_eq!(decoded, forecast);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 5);
    assert_eq!(decoded_ver.patch, 2);
    assert_eq!(consumed, bytes.len());
}

// --- Test 22: SmartMeter Coal-zone high-consumption roundtrip version fields ---
#[test]
fn test_smart_meter_high_consumption_version_fields() {
    let meter = SmartMeter {
        meter_id: u64::MAX / 2,
        household_id: 123456789,
        zone: GridZone::Industrial,
        kwh_x100: 9_999_999,
        timestamp: 1_740_000_000,
        status: MeterStatus::Active,
    };
    let ver_v1 = Version::new(1, 0, 0);
    let ver_v2 = Version::new(2, 0, 0);
    let ver_v152 = Version::new(1, 5, 2);

    let bytes_v1 = encode_versioned_value(&meter, ver_v1).expect("encode SmartMeter v1.0.0 high");
    let bytes_v2 = encode_versioned_value(&meter, ver_v2).expect("encode SmartMeter v2.0.0 high");
    let bytes_v152 =
        encode_versioned_value(&meter, ver_v152).expect("encode SmartMeter v1.5.2 high");

    let (decoded_v1, dver1, c1) =
        decode_versioned_value::<SmartMeter>(&bytes_v1).expect("decode SmartMeter v1.0.0 high");
    let (decoded_v2, dver2, c2) =
        decode_versioned_value::<SmartMeter>(&bytes_v2).expect("decode SmartMeter v2.0.0 high");
    let (decoded_v152, dver152, c3) =
        decode_versioned_value::<SmartMeter>(&bytes_v152).expect("decode SmartMeter v1.5.2 high");

    assert_eq!(decoded_v1, meter);
    assert_eq!(decoded_v2, meter);
    assert_eq!(decoded_v152, meter);

    assert_eq!(dver1.major, 1);
    assert_eq!(dver1.minor, 0);
    assert_eq!(dver1.patch, 0);

    assert_eq!(dver2.major, 2);
    assert_eq!(dver2.minor, 0);
    assert_eq!(dver2.patch, 0);

    assert_eq!(dver152.major, 1);
    assert_eq!(dver152.minor, 5);
    assert_eq!(dver152.patch, 2);

    assert_eq!(c1, bytes_v1.len());
    assert_eq!(c2, bytes_v2.len());
    assert_eq!(c3, bytes_v152.len());
}

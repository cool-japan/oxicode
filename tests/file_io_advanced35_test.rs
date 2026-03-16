//! Advanced file I/O tests for OxiCode — domain: nuclear power plant monitoring and control systems

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
use oxicode::{decode_from_file, decode_from_slice, encode_to_file, encode_to_vec, Decode, Encode};
use std::env::temp_dir;

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ControlRodStatus {
    Inserted,
    Withdrawn,
    Partial { percent_x100: u16 },
    Jammed,
    Calibrating,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CoolantType {
    LightWater,
    HeavyWater,
    Helium,
    SodiumLiquid,
    MoltenSalt,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RadiationAlarmLevel {
    Normal,
    Advisory,
    Alert,
    Emergency,
    Evacuation,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ScramTrigger {
    HighNeutronFlux,
    LowCoolantFlow,
    HighCoolantTemperature,
    LossOfFeedwaterFlow,
    HighContainmentPressure,
    ManualOperatorAction,
    SeismicEvent,
    ElectricalGridFault,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WasteCategory {
    LowLevel,
    IntermediateLevel,
    HighLevel,
    TransuranicWaste,
    SpentFuel,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ControlRodAssembly {
    rod_id: u32,
    bank_id: u8,
    status: ControlRodStatus,
    insertion_depth_mm: u32,
    last_moved_timestamp: u64,
    drive_mechanism_ok: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CoolantTemperatureSensor {
    sensor_id: u32,
    loop_id: u8,
    coolant_type: CoolantType,
    inlet_temp_milli_c: i32,
    outlet_temp_milli_c: i32,
    flow_rate_liters_per_min_x100: u32,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RadiationMonitor {
    monitor_id: u32,
    location: String,
    dose_rate_micro_sv_per_hr: u32,
    alarm_level: RadiationAlarmLevel,
    detector_online: bool,
    last_calibration_timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NeutronFluxReading {
    detector_id: u32,
    axial_position_cm: i16,
    radial_position_cm: u16,
    thermal_flux_n_per_cm2_per_s_x1e6: u64,
    fast_flux_n_per_cm2_per_s_x1e6: u64,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SafetyInterlock {
    interlock_id: u32,
    system_name: String,
    armed: bool,
    tripped: bool,
    setpoint_value_x1000: i64,
    current_value_x1000: i64,
    trip_count: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TurbineGeneratorOutput {
    unit_id: u8,
    active_power_mw_x100: u32,
    reactive_power_mvar_x100: i32,
    frequency_mhz: u32,
    shaft_speed_rpm_x100: u32,
    steam_inlet_pressure_kpa: u32,
    online: bool,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FuelRodAssembly {
    assembly_id: u32,
    fuel_type: String,
    enrichment_percent_x1000: u32,
    burnup_mwd_per_tonne_x100: u32,
    insertion_date_unix: u64,
    active_length_mm: u32,
    peak_cladding_temp_milli_c: u32,
    defect_detected: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ContainmentPressureSensor {
    sensor_id: u32,
    zone: u8,
    pressure_pa: u32,
    temperature_milli_c: i32,
    humidity_percent_x100: u16,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EccsActivationRecord {
    record_id: u64,
    trigger: ScramTrigger,
    activation_timestamp: u64,
    coolant_injection_rate_l_per_s_x100: u32,
    accumulator_pressure_kpa: u32,
    pump_ids_active: Vec<u8>,
    suppression_pool_temp_milli_c: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ScramEvent {
    event_id: u64,
    reactor_id: u8,
    trigger: ScramTrigger,
    scram_timestamp: u64,
    reactor_period_ms: u32,
    rods_inserted_count: u16,
    rods_total_count: u16,
    power_at_scram_percent_x100: u32,
    operator_id: u32,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RadioactiveWasteRecord {
    record_id: u64,
    waste_category: WasteCategory,
    activity_bq_x1e6: u64,
    mass_kg_x100: u32,
    volume_liters_x100: u32,
    container_id: String,
    generation_timestamp: u64,
    disposal_approved: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ControlRoomDisplay {
    display_id: u16,
    panel_name: String,
    reactor_power_percent_x100: u32,
    all_rods_inserted: bool,
    eccs_armed: bool,
    radiation_alarm_active: bool,
    active_alarms: Vec<String>,
    snapshot_timestamp: u64,
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn unique_tmp(name: &str) -> std::path::PathBuf {
    temp_dir().join(name)
}

// ---------------------------------------------------------------------------
// Tests — 22 total
// ---------------------------------------------------------------------------

/// 1. ControlRodAssembly partial insertion — vec roundtrip
#[test]
fn test_control_rod_assembly_partial_vec_roundtrip() {
    let rod = ControlRodAssembly {
        rod_id: 101,
        bank_id: 3,
        status: ControlRodStatus::Partial { percent_x100: 5500 },
        insertion_depth_mm: 2200,
        last_moved_timestamp: 1_740_000_000,
        drive_mechanism_ok: true,
    };
    let bytes = encode_to_vec(&rod).expect("encode ControlRodAssembly partial");
    let (decoded, consumed): (ControlRodAssembly, usize) =
        decode_from_slice(&bytes).expect("decode ControlRodAssembly partial");
    assert_eq!(rod, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 2. ControlRodAssembly all ControlRodStatus variants — vec roundtrip
#[test]
fn test_control_rod_status_all_variants_vec_roundtrip() {
    let statuses = [
        ControlRodStatus::Inserted,
        ControlRodStatus::Withdrawn,
        ControlRodStatus::Partial { percent_x100: 7500 },
        ControlRodStatus::Jammed,
        ControlRodStatus::Calibrating,
    ];
    for (i, status) in statuses.into_iter().enumerate() {
        let rod = ControlRodAssembly {
            rod_id: i as u32 + 200,
            bank_id: (i % 4) as u8,
            status,
            insertion_depth_mm: i as u32 * 400,
            last_moved_timestamp: 1_740_000_000 + i as u64 * 60,
            drive_mechanism_ok: i % 2 == 0,
        };
        let bytes = encode_to_vec(&rod).expect("encode ControlRodAssembly variant");
        let (decoded, _): (ControlRodAssembly, usize) =
            decode_from_slice(&bytes).expect("decode ControlRodAssembly variant");
        assert_eq!(rod, decoded);
    }
}

/// 3. CoolantTemperatureSensor file roundtrip — sodium-cooled loop
#[test]
fn test_coolant_temperature_sensor_sodium_file_roundtrip() {
    let path = unique_tmp("coolant_sensor_sodium_35.bin");
    let sensor = CoolantTemperatureSensor {
        sensor_id: 3001,
        loop_id: 2,
        coolant_type: CoolantType::SodiumLiquid,
        inlet_temp_milli_c: 370_000,
        outlet_temp_milli_c: 545_000,
        flow_rate_liters_per_min_x100: 1_800_000,
        timestamp: 1_740_001_000,
    };
    encode_to_file(&sensor, &path).expect("encode_to_file CoolantTemperatureSensor");
    let decoded: CoolantTemperatureSensor =
        decode_from_file(&path).expect("decode_from_file CoolantTemperatureSensor");
    assert_eq!(sensor, decoded);
    std::fs::remove_file(&path).expect("cleanup coolant_sensor_sodium_35.bin");
}

/// 4. RadiationMonitor all alarm levels — vec roundtrip
#[test]
fn test_radiation_monitor_all_alarm_levels_vec_roundtrip() {
    let levels = [
        RadiationAlarmLevel::Normal,
        RadiationAlarmLevel::Advisory,
        RadiationAlarmLevel::Alert,
        RadiationAlarmLevel::Emergency,
        RadiationAlarmLevel::Evacuation,
    ];
    for (i, alarm_level) in levels.into_iter().enumerate() {
        let monitor = RadiationMonitor {
            monitor_id: 4000 + i as u32,
            location: format!("Zone-{}", i + 1),
            dose_rate_micro_sv_per_hr: (10 * (i as u32 + 1)) * 100,
            alarm_level,
            detector_online: true,
            last_calibration_timestamp: 1_739_000_000 + i as u64 * 86400,
        };
        let bytes = encode_to_vec(&monitor).expect("encode RadiationMonitor");
        let (decoded, _): (RadiationMonitor, usize) =
            decode_from_slice(&bytes).expect("decode RadiationMonitor");
        assert_eq!(monitor, decoded);
    }
}

/// 5. NeutronFluxReading file roundtrip — high flux detector
#[test]
fn test_neutron_flux_reading_file_roundtrip() {
    let path = unique_tmp("neutron_flux_35.bin");
    let reading = NeutronFluxReading {
        detector_id: 5001,
        axial_position_cm: -120,
        radial_position_cm: 85,
        thermal_flux_n_per_cm2_per_s_x1e6: 3_200_000,
        fast_flux_n_per_cm2_per_s_x1e6: 850_000,
        timestamp: 1_740_005_000,
    };
    encode_to_file(&reading, &path).expect("encode_to_file NeutronFluxReading");
    let decoded: NeutronFluxReading =
        decode_from_file(&path).expect("decode_from_file NeutronFluxReading");
    assert_eq!(reading, decoded);
    std::fs::remove_file(&path).expect("cleanup neutron_flux_35.bin");
}

/// 6. SafetyInterlock vec roundtrip — tripped interlock
#[test]
fn test_safety_interlock_tripped_vec_roundtrip() {
    let interlock = SafetyInterlock {
        interlock_id: 6001,
        system_name: "High-Neutron-Flux-Trip".to_string(),
        armed: true,
        tripped: true,
        setpoint_value_x1000: 118_000,
        current_value_x1000: 121_500,
        trip_count: 3,
    };
    let bytes = encode_to_vec(&interlock).expect("encode SafetyInterlock tripped");
    let (decoded, consumed): (SafetyInterlock, usize) =
        decode_from_slice(&bytes).expect("decode SafetyInterlock tripped");
    assert_eq!(interlock, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 7. TurbineGeneratorOutput file roundtrip — unit online at full power
#[test]
fn test_turbine_generator_output_full_power_file_roundtrip() {
    let path = unique_tmp("turbine_output_35.bin");
    let output = TurbineGeneratorOutput {
        unit_id: 1,
        active_power_mw_x100: 99_500,
        reactive_power_mvar_x100: 15_200,
        frequency_mhz: 50_010,
        shaft_speed_rpm_x100: 300_000,
        steam_inlet_pressure_kpa: 6_895,
        online: true,
        timestamp: 1_740_010_000,
    };
    encode_to_file(&output, &path).expect("encode_to_file TurbineGeneratorOutput");
    let decoded: TurbineGeneratorOutput =
        decode_from_file(&path).expect("decode_from_file TurbineGeneratorOutput");
    assert_eq!(output, decoded);
    std::fs::remove_file(&path).expect("cleanup turbine_output_35.bin");
}

/// 8. FuelRodAssembly vec roundtrip — MOX fuel with defect
#[test]
fn test_fuel_rod_assembly_mox_defect_vec_roundtrip() {
    let assembly = FuelRodAssembly {
        assembly_id: 8001,
        fuel_type: "MOX-UPuO2".to_string(),
        enrichment_percent_x1000: 4_500,
        burnup_mwd_per_tonne_x100: 3_400_000,
        insertion_date_unix: 1_680_000_000,
        active_length_mm: 3_660,
        peak_cladding_temp_milli_c: 350_000,
        defect_detected: true,
    };
    let bytes = encode_to_vec(&assembly).expect("encode FuelRodAssembly defect");
    let (decoded, _): (FuelRodAssembly, usize) =
        decode_from_slice(&bytes).expect("decode FuelRodAssembly defect");
    assert_eq!(assembly, decoded);
}

/// 9. ContainmentPressureSensor file roundtrip — elevated pressure
#[test]
fn test_containment_pressure_sensor_elevated_file_roundtrip() {
    let path = unique_tmp("containment_pressure_35.bin");
    let sensor = ContainmentPressureSensor {
        sensor_id: 9001,
        zone: 1,
        pressure_pa: 115_000,
        temperature_milli_c: 65_000,
        humidity_percent_x100: 7_500,
        timestamp: 1_740_015_000,
    };
    encode_to_file(&sensor, &path).expect("encode_to_file ContainmentPressureSensor");
    let decoded: ContainmentPressureSensor =
        decode_from_file(&path).expect("decode_from_file ContainmentPressureSensor");
    assert_eq!(sensor, decoded);
    std::fs::remove_file(&path).expect("cleanup containment_pressure_35.bin");
}

/// 10. EccsActivationRecord vec roundtrip — multiple pumps
#[test]
fn test_eccs_activation_record_multi_pump_vec_roundtrip() {
    let record = EccsActivationRecord {
        record_id: 10_001,
        trigger: ScramTrigger::LowCoolantFlow,
        activation_timestamp: 1_740_020_000,
        coolant_injection_rate_l_per_s_x100: 32_000,
        accumulator_pressure_kpa: 6_000,
        pump_ids_active: vec![1, 2, 3, 4],
        suppression_pool_temp_milli_c: 35_500,
    };
    let bytes = encode_to_vec(&record).expect("encode EccsActivationRecord multi-pump");
    let (decoded, consumed): (EccsActivationRecord, usize) =
        decode_from_slice(&bytes).expect("decode EccsActivationRecord multi-pump");
    assert_eq!(record, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 11. ScramEvent all ScramTrigger variants — vec roundtrip
#[test]
fn test_scram_event_all_triggers_vec_roundtrip() {
    let triggers = [
        ScramTrigger::HighNeutronFlux,
        ScramTrigger::LowCoolantFlow,
        ScramTrigger::HighCoolantTemperature,
        ScramTrigger::LossOfFeedwaterFlow,
        ScramTrigger::HighContainmentPressure,
        ScramTrigger::ManualOperatorAction,
        ScramTrigger::SeismicEvent,
        ScramTrigger::ElectricalGridFault,
    ];
    for (i, trigger) in triggers.into_iter().enumerate() {
        let event = ScramEvent {
            event_id: 11_000 + i as u64,
            reactor_id: 1,
            trigger,
            scram_timestamp: 1_740_025_000 + i as u64 * 3600,
            reactor_period_ms: 50 + i as u32 * 5,
            rods_inserted_count: 185 - i as u16,
            rods_total_count: 185,
            power_at_scram_percent_x100: 10_000 - i as u32 * 500,
            operator_id: 7001 + i as u32,
            notes: format!("Scram event index {}", i),
        };
        let bytes = encode_to_vec(&event).expect("encode ScramEvent");
        let (decoded, _): (ScramEvent, usize) =
            decode_from_slice(&bytes).expect("decode ScramEvent");
        assert_eq!(event, decoded);
    }
}

/// 12. ScramEvent file roundtrip — manual operator action
#[test]
fn test_scram_event_manual_operator_file_roundtrip() {
    let path = unique_tmp("scram_event_manual_35.bin");
    let event = ScramEvent {
        event_id: 12_001,
        reactor_id: 2,
        trigger: ScramTrigger::ManualOperatorAction,
        scram_timestamp: 1_740_030_000,
        reactor_period_ms: 120,
        rods_inserted_count: 185,
        rods_total_count: 185,
        power_at_scram_percent_x100: 7_500,
        operator_id: 8042,
        notes: "Controlled shutdown for refuelling outage".to_string(),
    };
    encode_to_file(&event, &path).expect("encode_to_file ScramEvent manual");
    let decoded: ScramEvent = decode_from_file(&path).expect("decode_from_file ScramEvent manual");
    assert_eq!(event, decoded);
    std::fs::remove_file(&path).expect("cleanup scram_event_manual_35.bin");
}

/// 13. RadioactiveWasteRecord all WasteCategory variants — vec roundtrip
#[test]
fn test_radioactive_waste_all_categories_vec_roundtrip() {
    let categories = [
        WasteCategory::LowLevel,
        WasteCategory::IntermediateLevel,
        WasteCategory::HighLevel,
        WasteCategory::TransuranicWaste,
        WasteCategory::SpentFuel,
    ];
    for (i, waste_category) in categories.into_iter().enumerate() {
        let record = RadioactiveWasteRecord {
            record_id: 13_000 + i as u64,
            waste_category,
            activity_bq_x1e6: (1_000 * (i as u64 + 1)) * 1_000_000,
            mass_kg_x100: 50_000 + i as u32 * 5000,
            volume_liters_x100: 20_000 + i as u32 * 2000,
            container_id: format!("DRUM-{:06}", 900 + i),
            generation_timestamp: 1_700_000_000 + i as u64 * 86400 * 30,
            disposal_approved: i % 2 == 0,
        };
        let bytes = encode_to_vec(&record).expect("encode RadioactiveWasteRecord");
        let (decoded, _): (RadioactiveWasteRecord, usize) =
            decode_from_slice(&bytes).expect("decode RadioactiveWasteRecord");
        assert_eq!(record, decoded);
    }
}

/// 14. ControlRoomDisplay file roundtrip — multiple active alarms
#[test]
fn test_control_room_display_alarms_file_roundtrip() {
    let path = unique_tmp("control_room_display_35.bin");
    let display = ControlRoomDisplay {
        display_id: 101,
        panel_name: "Main Control Panel - Reactor Hall".to_string(),
        reactor_power_percent_x100: 10_050,
        all_rods_inserted: false,
        eccs_armed: true,
        radiation_alarm_active: true,
        active_alarms: vec![
            "HIGH_COOLANT_TEMP_LOOP_1".to_string(),
            "NEUTRON_FLUX_ABOVE_SETPOINT".to_string(),
            "TURBINE_VIBRATION_HIGH".to_string(),
        ],
        snapshot_timestamp: 1_740_035_000,
    };
    encode_to_file(&display, &path).expect("encode_to_file ControlRoomDisplay");
    let decoded: ControlRoomDisplay =
        decode_from_file(&path).expect("decode_from_file ControlRoomDisplay");
    assert_eq!(display, decoded);
    std::fs::remove_file(&path).expect("cleanup control_room_display_35.bin");
}

/// 15. Large batch of NeutronFluxReadings — 256 detectors, vec roundtrip
#[test]
fn test_large_neutron_flux_batch_256_detectors_vec_roundtrip() {
    let readings: Vec<NeutronFluxReading> = (0u32..256)
        .map(|i| NeutronFluxReading {
            detector_id: i,
            axial_position_cm: (i as i16 % 200) - 100,
            radial_position_cm: (i % 150) as u16,
            thermal_flux_n_per_cm2_per_s_x1e6: 2_000_000 + (i as u64 * 7_500),
            fast_flux_n_per_cm2_per_s_x1e6: 500_000 + (i as u64 * 1_200),
            timestamp: 1_740_040_000 + i as u64,
        })
        .collect();
    assert_eq!(readings.len(), 256);
    let bytes = encode_to_vec(&readings).expect("encode large NeutronFlux batch");
    let (decoded, _): (Vec<NeutronFluxReading>, usize) =
        decode_from_slice(&bytes).expect("decode large NeutronFlux batch");
    assert_eq!(readings, decoded);
}

/// 16. Encoding determinism — identical ScramEvent encodes to identical bytes
#[test]
fn test_scram_event_encoding_determinism() {
    let event = ScramEvent {
        event_id: 16_001,
        reactor_id: 1,
        trigger: ScramTrigger::HighNeutronFlux,
        scram_timestamp: 1_740_045_000,
        reactor_period_ms: 35,
        rods_inserted_count: 185,
        rods_total_count: 185,
        power_at_scram_percent_x100: 11_500,
        operator_id: 9001,
        notes: "Automatic SCRAM — flux trip channel A".to_string(),
    };
    let bytes_a = encode_to_vec(&event).expect("encode ScramEvent determinism first");
    let bytes_b = encode_to_vec(&event).expect("encode ScramEvent determinism second");
    assert_eq!(
        bytes_a, bytes_b,
        "ScramEvent encoding must be deterministic"
    );
}

/// 17. Vec of FuelRodAssemblies file roundtrip — full core subset
#[test]
fn test_fuel_rod_assemblies_full_core_subset_file_roundtrip() {
    let path = unique_tmp("fuel_rods_core_35.bin");
    let assemblies: Vec<FuelRodAssembly> = (0u32..64)
        .map(|i| FuelRodAssembly {
            assembly_id: 17_000 + i,
            fuel_type: if i % 3 == 0 {
                "MOX-UPuO2".to_string()
            } else {
                "UO2".to_string()
            },
            enrichment_percent_x1000: 3_200 + (i % 10) * 100,
            burnup_mwd_per_tonne_x100: (i as u32) * 100_000,
            insertion_date_unix: 1_680_000_000 + (i as u64) * 86400 * 30,
            active_length_mm: 3_658,
            peak_cladding_temp_milli_c: 300_000 + i * 1_000,
            defect_detected: i % 17 == 0,
        })
        .collect();
    encode_to_file(&assemblies, &path).expect("encode_to_file FuelRodAssembly batch");
    let decoded: Vec<FuelRodAssembly> =
        decode_from_file(&path).expect("decode_from_file FuelRodAssembly batch");
    assert_eq!(assemblies, decoded);
    std::fs::remove_file(&path).expect("cleanup fuel_rods_core_35.bin");
}

/// 18. Config with fixed_int_encoding — CoolantTemperatureSensor roundtrip
#[test]
fn test_coolant_sensor_fixed_int_encoding_roundtrip() {
    let sensor = CoolantTemperatureSensor {
        sensor_id: 18_001,
        loop_id: 1,
        coolant_type: CoolantType::LightWater,
        inlet_temp_milli_c: 285_000,
        outlet_temp_milli_c: 321_000,
        flow_rate_liters_per_min_x100: 2_500_000,
        timestamp: 1_740_050_000,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes =
        oxicode::encode_to_vec_with_config(&sensor, cfg).expect("encode CoolantSensor fixed_int");
    let (decoded, consumed): (CoolantTemperatureSensor, usize) =
        oxicode::decode_from_slice_with_config(&bytes, cfg)
            .expect("decode CoolantSensor fixed_int");
    assert_eq!(sensor, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 19. Config with big_endian — NeutronFluxReading roundtrip
#[test]
fn test_neutron_flux_big_endian_config_roundtrip() {
    let reading = NeutronFluxReading {
        detector_id: 19_001,
        axial_position_cm: 45,
        radial_position_cm: 120,
        thermal_flux_n_per_cm2_per_s_x1e6: 2_750_000,
        fast_flux_n_per_cm2_per_s_x1e6: 720_000,
        timestamp: 1_740_055_000,
    };
    let cfg = config::standard().with_big_endian();
    let bytes =
        oxicode::encode_to_vec_with_config(&reading, cfg).expect("encode NeutronFlux big_endian");
    let (decoded, consumed): (NeutronFluxReading, usize) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode NeutronFlux big_endian");
    assert_eq!(reading, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 20. Config with fixed_int_encoding + big_endian — ScramEvent file roundtrip
#[test]
fn test_scram_event_fixed_int_big_endian_file_roundtrip() {
    let path = unique_tmp("scram_event_fixed_be_35.bin");
    let event = ScramEvent {
        event_id: 20_001,
        reactor_id: 3,
        trigger: ScramTrigger::SeismicEvent,
        scram_timestamp: 1_740_060_000,
        reactor_period_ms: 28,
        rods_inserted_count: 185,
        rods_total_count: 185,
        power_at_scram_percent_x100: 8_700,
        operator_id: 6001,
        notes: "Seismic protection trip — magnitude 4.2 recorded".to_string(),
    };
    let cfg = config::standard()
        .with_fixed_int_encoding()
        .with_big_endian();
    let bytes =
        oxicode::encode_to_vec_with_config(&event, cfg).expect("encode ScramEvent fixed+be");
    std::fs::write(&path, &bytes).expect("write ScramEvent fixed+be to file");
    let raw = std::fs::read(&path).expect("read ScramEvent fixed+be file");
    let (decoded, _): (ScramEvent, usize) = oxicode::decode_from_slice_with_config(&raw, cfg)
        .expect("decode ScramEvent fixed+be from slice");
    assert_eq!(event, decoded);
    std::fs::remove_file(&path).expect("cleanup scram_event_fixed_be_35.bin");
}

/// 21. File overwrite — second EccsActivationRecord replaces first
#[test]
fn test_eccs_record_file_overwrite_second_wins() {
    let path = unique_tmp("eccs_overwrite_35.bin");
    let first = EccsActivationRecord {
        record_id: 21_001,
        trigger: ScramTrigger::HighContainmentPressure,
        activation_timestamp: 1_740_065_000,
        coolant_injection_rate_l_per_s_x100: 10_000,
        accumulator_pressure_kpa: 5_500,
        pump_ids_active: vec![1],
        suppression_pool_temp_milli_c: 30_000,
    };
    let second = EccsActivationRecord {
        record_id: 21_002,
        trigger: ScramTrigger::LossOfFeedwaterFlow,
        activation_timestamp: 1_740_065_900,
        coolant_injection_rate_l_per_s_x100: 28_000,
        accumulator_pressure_kpa: 5_800,
        pump_ids_active: vec![1, 2, 3],
        suppression_pool_temp_milli_c: 32_500,
    };
    encode_to_file(&first, &path).expect("first write EccsActivationRecord");
    encode_to_file(&second, &path).expect("second write EccsActivationRecord (overwrite)");
    let decoded: EccsActivationRecord =
        decode_from_file(&path).expect("decode after overwrite EccsActivationRecord");
    assert_eq!(second, decoded);
    assert_ne!(first.record_id, decoded.record_id);
    std::fs::remove_file(&path).expect("cleanup eccs_overwrite_35.bin");
}

/// 22. Full pipeline — RadiationMonitor encode to file, read raw bytes,
///     decode from slice and from file, verify both match
#[test]
fn test_full_pipeline_radiation_monitor_file_bytes_slice() {
    let path = unique_tmp("radiation_pipeline_35.bin");
    let monitor = RadiationMonitor {
        monitor_id: 22_001,
        location: "Turbine-Hall-Exhaust-Duct".to_string(),
        dose_rate_micro_sv_per_hr: 850,
        alarm_level: RadiationAlarmLevel::Alert,
        detector_online: true,
        last_calibration_timestamp: 1_738_000_000,
    };
    encode_to_file(&monitor, &path).expect("pipeline: encode_to_file RadiationMonitor");
    let raw_bytes = std::fs::read(&path).expect("pipeline: read raw bytes RadiationMonitor");
    let (decoded_slice, consumed): (RadiationMonitor, usize) =
        decode_from_slice(&raw_bytes).expect("pipeline: decode_from_slice RadiationMonitor");
    let decoded_file: RadiationMonitor =
        decode_from_file(&path).expect("pipeline: decode_from_file RadiationMonitor");
    assert_eq!(monitor, decoded_slice);
    assert_eq!(monitor, decoded_file);
    assert_eq!(consumed, raw_bytes.len());
    std::fs::remove_file(&path).expect("cleanup radiation_pipeline_35.bin");
}

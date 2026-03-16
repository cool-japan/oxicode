//! Advanced complex enum tests for OxiCode — nuclear power plant instrumentation & control domain.
//! 22 test functions covering deeply nested enums, enums with Vec/Option fields, and enums
//! containing other enums across reactor operations, safety systems, and monitoring.

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
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ReactorOperatingMode {
    Startup {
        phase: StartupPhase,
        neutron_count_rate_cps: f64,
        estimated_criticality_pct: f64,
    },
    Power {
        thermal_power_mw: f64,
        electrical_output_mw: f64,
        capacity_factor_pct: f64,
    },
    Shutdown {
        reason: ShutdownReason,
        cooldown_rate_deg_per_hr: f64,
        estimated_cold_shutdown_hrs: f64,
    },
    Scram {
        trigger: ScramTrigger,
        timestamp_epoch_ms: u64,
        automatic: bool,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum StartupPhase {
    SubcriticalMultiplication,
    ApproachToCriticality,
    LowPowerPhysicsTesting,
    PowerAscension,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ShutdownReason {
    Scheduled,
    Refueling,
    MaintenanceRequired { component: String },
    RegulatoryOrder,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ScramTrigger {
    HighNeutronFlux,
    LowCoolantFlow,
    HighContainmentPressure,
    SeismicEvent { magnitude: u32 },
    ManualTrip { operator_id: String },
    MultipleTriggers(Vec<String>),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ControlRodClusterPosition {
    FullyInserted,
    FullyWithdrawn,
    PartialPosition {
        steps_withdrawn: u16,
        max_steps: u16,
        cluster_id: String,
    },
    StuckRod {
        cluster_id: String,
        last_known_steps: u16,
        alarm_active: bool,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PrimaryCoolantLoopStatus {
    NormalOperation {
        loop_id: u8,
        flow_rate_kg_per_s: f64,
        hot_leg_temp_c: f64,
        cold_leg_temp_c: f64,
        pressure_mpa: f64,
    },
    ReducedFlow {
        loop_id: u8,
        flow_rate_kg_per_s: f64,
        reason: String,
    },
    Isolated {
        loop_id: u8,
        isolation_reason: String,
        valves_closed: Vec<String>,
    },
    NaturalCirculation {
        loop_id: u8,
        estimated_flow_kg_per_s: f64,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ContainmentMonitoring {
    Normal {
        pressure_kpa: f64,
        temperature_c: f64,
        humidity_pct: f64,
    },
    ElevatedRadiation {
        zone: RadiationZone,
        dose_rate_usv_per_hr: f64,
        isotopes_detected: Vec<String>,
    },
    PressureTransient {
        peak_pressure_kpa: f64,
        current_pressure_kpa: f64,
        spray_system_active: bool,
    },
    IntegrityChallenge {
        breach_location: Option<String>,
        leak_rate_pct_per_day: f64,
        hydrogen_concentration_pct: f64,
        recombiner_status: HydrogenRecombinerStatus,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum HydrogenRecombinerStatus {
    Standby,
    Operating { recombination_rate_kg_per_hr: f64 },
    Faulted { fault_description: String },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RadiationZone {
    Green,
    Yellow,
    Orange,
    Red,
    Exclusion {
        reason: String,
        boundary_meters: f64,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EmergencyCoolingState {
    Standby,
    Actuated {
        train: SafetyTrain,
        flow_rate_kg_per_s: f64,
        injection_mode: InjectionMode,
    },
    Recirculation {
        sump_level_pct: f64,
        trains_running: Vec<SafetyTrain>,
    },
    TestMode {
        test_id: String,
        parameters: Vec<String>,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SafetyTrain {
    TrainA,
    TrainB,
    BothTrains,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum InjectionMode {
    HighPressure,
    LowPressure,
    Accumulator {
        tank_pressure_mpa: f64,
        volume_m3: f64,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TurbineGeneratorStatus {
    Offline,
    Warming {
        steam_temp_c: f64,
        target_rpm: u32,
    },
    Synchronizing {
        frequency_hz: f64,
        voltage_kv: f64,
    },
    LoadedOnGrid {
        output_mw: f64,
        steam_flow_kg_per_s: f64,
        condenser_vacuum_kpa: f64,
    },
    Tripped {
        trip_cause: String,
        coast_down_rpm: u32,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpentFuelPoolMonitoring {
    pool_id: String,
    water_temperature_c: f64,
    water_level_m: f64,
    boron_concentration_ppm: f64,
    cooling_status: SpentFuelCoolingStatus,
    radiation_dose_rate_usv_per_hr: f64,
    assemblies_stored: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SpentFuelCoolingStatus {
    NormalCooling {
        inlet_temp_c: f64,
        outlet_temp_c: f64,
    },
    BackupCooling,
    LossOfCooling {
        time_to_boiling_hrs: f64,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum NeutronFluxMeasurement {
    SourceRange {
        detector_id: String,
        count_rate_cps: f64,
        startup_rate_dpm: f64,
    },
    IntermediateRange {
        detector_id: String,
        current_amps: f64,
        power_pct: f64,
    },
    PowerRange {
        detector_id: String,
        upper_pct: f64,
        lower_pct: f64,
        axial_flux_difference_pct: f64,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SteamGeneratorIntegrity {
    Healthy {
        sg_id: String,
        tubes_inspected: u32,
        tubes_plugged: u32,
    },
    DegradedTube {
        sg_id: String,
        tube_row: u16,
        tube_col: u16,
        wall_thinning_pct: f64,
        mechanism: DegradationMechanism,
    },
    LeakDetected {
        sg_id: String,
        leak_rate_liters_per_hr: f64,
        n16_activity_detected: bool,
        action: SteamGeneratorAction,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DegradationMechanism {
    PrimaryWaterStressCorrosionCracking,
    OuterDiameterStressCorrosionCracking,
    Pitting,
    Fretting { support_plate_id: String },
    Denting,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SteamGeneratorAction {
    MonitorAndTrend,
    PlugTube,
    SleeveRepair,
    ForceOutage,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AuxiliaryFeedwaterSystem {
    Standby,
    MotorDrivenPump {
        pump_id: String,
        flow_rate_kg_per_s: f64,
        discharge_pressure_mpa: f64,
    },
    TurbineDrivenPump {
        steam_supply_sg: String,
        flow_rate_kg_per_s: f64,
    },
    AlignedToSteamGenerators {
        sg_ids: Vec<String>,
        total_flow_kg_per_s: f64,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CriticalitySafetyParameter {
    BoronConcentration {
        measured_ppm: f64,
        required_ppm: f64,
        margin_ppm: f64,
    },
    ShutdownMargin {
        delta_k_over_k_pcm: f64,
        most_reactive_rod_worth_pcm: f64,
        adequate: bool,
    },
    ModerationControl {
        moderator_temp_coefficient: f64,
        doppler_coefficient: f64,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DecayHeatRemoval {
    ResidualHeatRemoval {
        train: SafetyTrain,
        rhr_flow_rate_kg_per_s: f64,
        rhr_heat_exchanger_outlet_c: f64,
    },
    SteamDump {
        condenser_available: bool,
        atmospheric_dump_valves_open: Vec<String>,
    },
    FeedAndBleed {
        charging_flow_kg_per_s: f64,
        porv_open: bool,
        letdown_isolated: bool,
    },
    PassiveCooling {
        pccs_tank_level_pct: f64,
        natural_circulation_established: bool,
    },
}

/// Composite plant snapshot for deeply nested testing.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PlantSnapshot {
    reactor_mode: ReactorOperatingMode,
    control_rods: Vec<ControlRodClusterPosition>,
    coolant_loops: Vec<PrimaryCoolantLoopStatus>,
    containment: ContainmentMonitoring,
    eccs: EmergencyCoolingState,
    turbine: TurbineGeneratorStatus,
    spent_fuel: Option<SpentFuelPoolMonitoring>,
    neutron_flux: Vec<NeutronFluxMeasurement>,
    decay_heat: Option<DecayHeatRemoval>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_reactor_startup_mode_roundtrip() {
    let val = ReactorOperatingMode::Startup {
        phase: StartupPhase::ApproachToCriticality,
        neutron_count_rate_cps: 5_200.75,
        estimated_criticality_pct: 92.3,
    };
    let bytes = encode_to_vec(&val).expect("encode reactor startup");
    let (decoded, _) =
        decode_from_slice::<ReactorOperatingMode>(&bytes).expect("decode reactor startup");
    assert_eq!(val, decoded);
}

#[test]
fn test_reactor_scram_with_multiple_triggers() {
    let val = ReactorOperatingMode::Scram {
        trigger: ScramTrigger::MultipleTriggers(vec![
            "High neutron flux".to_string(),
            "Low coolant flow loop 2".to_string(),
            "High containment pressure".to_string(),
        ]),
        timestamp_epoch_ms: 1_700_000_000_000,
        automatic: true,
    };
    let bytes = encode_to_vec(&val).expect("encode scram multiple triggers");
    let (decoded, _) =
        decode_from_slice::<ReactorOperatingMode>(&bytes).expect("decode scram multiple triggers");
    assert_eq!(val, decoded);
}

#[test]
fn test_reactor_shutdown_with_maintenance_reason() {
    let val = ReactorOperatingMode::Shutdown {
        reason: ShutdownReason::MaintenanceRequired {
            component: "RCP-1A Main Seal".to_string(),
        },
        cooldown_rate_deg_per_hr: 28.0,
        estimated_cold_shutdown_hrs: 14.5,
    };
    let bytes = encode_to_vec(&val).expect("encode shutdown maintenance");
    let (decoded, _) =
        decode_from_slice::<ReactorOperatingMode>(&bytes).expect("decode shutdown maintenance");
    assert_eq!(val, decoded);
}

#[test]
fn test_control_rod_stuck_rod_alarm() {
    let val = ControlRodClusterPosition::StuckRod {
        cluster_id: "RCC-D7".to_string(),
        last_known_steps: 186,
        alarm_active: true,
    };
    let bytes = encode_to_vec(&val).expect("encode stuck rod");
    let (decoded, _) =
        decode_from_slice::<ControlRodClusterPosition>(&bytes).expect("decode stuck rod");
    assert_eq!(val, decoded);
}

#[test]
fn test_control_rod_cluster_vec_roundtrip() {
    let rods = vec![
        ControlRodClusterPosition::FullyWithdrawn,
        ControlRodClusterPosition::PartialPosition {
            steps_withdrawn: 200,
            max_steps: 228,
            cluster_id: "RCC-A3".to_string(),
        },
        ControlRodClusterPosition::FullyInserted,
        ControlRodClusterPosition::StuckRod {
            cluster_id: "RCC-B5".to_string(),
            last_known_steps: 45,
            alarm_active: false,
        },
    ];
    let bytes = encode_to_vec(&rods).expect("encode rod cluster vec");
    let (decoded, _) = decode_from_slice::<Vec<ControlRodClusterPosition>>(&bytes)
        .expect("decode rod cluster vec");
    assert_eq!(rods, decoded);
}

#[test]
fn test_primary_coolant_isolated_loop() {
    let val = PrimaryCoolantLoopStatus::Isolated {
        loop_id: 3,
        isolation_reason: "RCP vibration high".to_string(),
        valves_closed: vec![
            "V-31A".to_string(),
            "V-31B".to_string(),
            "V-32A".to_string(),
        ],
    };
    let bytes = encode_to_vec(&val).expect("encode isolated loop");
    let (decoded, _) =
        decode_from_slice::<PrimaryCoolantLoopStatus>(&bytes).expect("decode isolated loop");
    assert_eq!(val, decoded);
}

#[test]
fn test_containment_integrity_challenge_with_optional_breach() {
    let val = ContainmentMonitoring::IntegrityChallenge {
        breach_location: Some("Equipment hatch seal region".to_string()),
        leak_rate_pct_per_day: 0.35,
        hydrogen_concentration_pct: 3.8,
        recombiner_status: HydrogenRecombinerStatus::Operating {
            recombination_rate_kg_per_hr: 0.12,
        },
    };
    let bytes = encode_to_vec(&val).expect("encode integrity challenge");
    let (decoded, _) =
        decode_from_slice::<ContainmentMonitoring>(&bytes).expect("decode integrity challenge");
    assert_eq!(val, decoded);
}

#[test]
fn test_containment_integrity_no_breach_location() {
    let val = ContainmentMonitoring::IntegrityChallenge {
        breach_location: None,
        leak_rate_pct_per_day: 0.08,
        hydrogen_concentration_pct: 1.2,
        recombiner_status: HydrogenRecombinerStatus::Standby,
    };
    let bytes = encode_to_vec(&val).expect("encode integrity no breach");
    let (decoded, _) =
        decode_from_slice::<ContainmentMonitoring>(&bytes).expect("decode integrity no breach");
    assert_eq!(val, decoded);
}

#[test]
fn test_containment_elevated_radiation_exclusion_zone() {
    let val = ContainmentMonitoring::ElevatedRadiation {
        zone: RadiationZone::Exclusion {
            reason: "Primary-to-secondary leak".to_string(),
            boundary_meters: 150.0,
        },
        dose_rate_usv_per_hr: 2500.0,
        isotopes_detected: vec![
            "I-131".to_string(),
            "Cs-137".to_string(),
            "Xe-133".to_string(),
        ],
    };
    let bytes = encode_to_vec(&val).expect("encode elevated radiation");
    let (decoded, _) =
        decode_from_slice::<ContainmentMonitoring>(&bytes).expect("decode elevated radiation");
    assert_eq!(val, decoded);
}

#[test]
fn test_eccs_actuated_high_pressure_injection() {
    let val = EmergencyCoolingState::Actuated {
        train: SafetyTrain::TrainA,
        flow_rate_kg_per_s: 45.0,
        injection_mode: InjectionMode::HighPressure,
    };
    let bytes = encode_to_vec(&val).expect("encode ECCS actuated");
    let (decoded, _) =
        decode_from_slice::<EmergencyCoolingState>(&bytes).expect("decode ECCS actuated");
    assert_eq!(val, decoded);
}

#[test]
fn test_eccs_recirculation_both_trains() {
    let val = EmergencyCoolingState::Recirculation {
        sump_level_pct: 78.5,
        trains_running: vec![SafetyTrain::TrainA, SafetyTrain::TrainB],
    };
    let bytes = encode_to_vec(&val).expect("encode ECCS recirc");
    let (decoded, _) =
        decode_from_slice::<EmergencyCoolingState>(&bytes).expect("decode ECCS recirc");
    assert_eq!(val, decoded);
}

#[test]
fn test_eccs_accumulator_injection_mode() {
    let val = EmergencyCoolingState::Actuated {
        train: SafetyTrain::BothTrains,
        flow_rate_kg_per_s: 120.0,
        injection_mode: InjectionMode::Accumulator {
            tank_pressure_mpa: 4.5,
            volume_m3: 38.0,
        },
    };
    let bytes = encode_to_vec(&val).expect("encode accumulator injection");
    let (decoded, _) =
        decode_from_slice::<EmergencyCoolingState>(&bytes).expect("decode accumulator injection");
    assert_eq!(val, decoded);
}

#[test]
fn test_turbine_loaded_on_grid() {
    let val = TurbineGeneratorStatus::LoadedOnGrid {
        output_mw: 1150.0,
        steam_flow_kg_per_s: 1820.5,
        condenser_vacuum_kpa: 5.1,
    };
    let bytes = encode_to_vec(&val).expect("encode turbine loaded");
    let (decoded, _) =
        decode_from_slice::<TurbineGeneratorStatus>(&bytes).expect("decode turbine loaded");
    assert_eq!(val, decoded);
}

#[test]
fn test_spent_fuel_pool_loss_of_cooling() {
    let val = SpentFuelPoolMonitoring {
        pool_id: "SFP-1".to_string(),
        water_temperature_c: 62.0,
        water_level_m: 7.2,
        boron_concentration_ppm: 2400.0,
        cooling_status: SpentFuelCoolingStatus::LossOfCooling {
            time_to_boiling_hrs: 8.5,
        },
        radiation_dose_rate_usv_per_hr: 15.0,
        assemblies_stored: 1247,
    };
    let bytes = encode_to_vec(&val).expect("encode SFP monitoring");
    let (decoded, _) =
        decode_from_slice::<SpentFuelPoolMonitoring>(&bytes).expect("decode SFP monitoring");
    assert_eq!(val, decoded);
}

#[test]
fn test_neutron_flux_power_range_with_axial_offset() {
    let val = NeutronFluxMeasurement::PowerRange {
        detector_id: "N44".to_string(),
        upper_pct: 52.3,
        lower_pct: 48.8,
        axial_flux_difference_pct: 3.5,
    };
    let bytes = encode_to_vec(&val).expect("encode power range flux");
    let (decoded, _) =
        decode_from_slice::<NeutronFluxMeasurement>(&bytes).expect("decode power range flux");
    assert_eq!(val, decoded);
}

#[test]
fn test_steam_generator_tube_degradation() {
    let val = SteamGeneratorIntegrity::DegradedTube {
        sg_id: "SG-2".to_string(),
        tube_row: 34,
        tube_col: 71,
        wall_thinning_pct: 38.2,
        mechanism: DegradationMechanism::Fretting {
            support_plate_id: "SP-7C".to_string(),
        },
    };
    let bytes = encode_to_vec(&val).expect("encode SG degradation");
    let (decoded, _) =
        decode_from_slice::<SteamGeneratorIntegrity>(&bytes).expect("decode SG degradation");
    assert_eq!(val, decoded);
}

#[test]
fn test_steam_generator_leak_with_action() {
    let val = SteamGeneratorIntegrity::LeakDetected {
        sg_id: "SG-4".to_string(),
        leak_rate_liters_per_hr: 0.45,
        n16_activity_detected: true,
        action: SteamGeneratorAction::ForceOutage,
    };
    let bytes = encode_to_vec(&val).expect("encode SG leak");
    let (decoded, _) =
        decode_from_slice::<SteamGeneratorIntegrity>(&bytes).expect("decode SG leak");
    assert_eq!(val, decoded);
}

#[test]
fn test_auxiliary_feedwater_aligned_to_multiple_sgs() {
    let val = AuxiliaryFeedwaterSystem::AlignedToSteamGenerators {
        sg_ids: vec![
            "SG-1".to_string(),
            "SG-2".to_string(),
            "SG-3".to_string(),
            "SG-4".to_string(),
        ],
        total_flow_kg_per_s: 88.0,
    };
    let bytes = encode_to_vec(&val).expect("encode AFW aligned");
    let (decoded, _) =
        decode_from_slice::<AuxiliaryFeedwaterSystem>(&bytes).expect("decode AFW aligned");
    assert_eq!(val, decoded);
}

#[test]
fn test_criticality_shutdown_margin_adequate() {
    let val = CriticalitySafetyParameter::ShutdownMargin {
        delta_k_over_k_pcm: -3200.0,
        most_reactive_rod_worth_pcm: 1500.0,
        adequate: true,
    };
    let bytes = encode_to_vec(&val).expect("encode SDM");
    let (decoded, _) = decode_from_slice::<CriticalitySafetyParameter>(&bytes).expect("decode SDM");
    assert_eq!(val, decoded);
}

#[test]
fn test_decay_heat_feed_and_bleed() {
    let val = DecayHeatRemoval::FeedAndBleed {
        charging_flow_kg_per_s: 12.5,
        porv_open: true,
        letdown_isolated: true,
    };
    let bytes = encode_to_vec(&val).expect("encode feed-and-bleed");
    let (decoded, _) =
        decode_from_slice::<DecayHeatRemoval>(&bytes).expect("decode feed-and-bleed");
    assert_eq!(val, decoded);
}

#[test]
fn test_decay_heat_steam_dump_with_atmospheric_valves() {
    let val = DecayHeatRemoval::SteamDump {
        condenser_available: false,
        atmospheric_dump_valves_open: vec!["ADV-SG1".to_string(), "ADV-SG2".to_string()],
    };
    let bytes = encode_to_vec(&val).expect("encode steam dump");
    let (decoded, _) = decode_from_slice::<DecayHeatRemoval>(&bytes).expect("decode steam dump");
    assert_eq!(val, decoded);
}

#[test]
fn test_full_plant_snapshot_deeply_nested() {
    let snapshot = PlantSnapshot {
        reactor_mode: ReactorOperatingMode::Scram {
            trigger: ScramTrigger::SeismicEvent { magnitude: 6 },
            timestamp_epoch_ms: 1_710_000_000_000,
            automatic: true,
        },
        control_rods: vec![
            ControlRodClusterPosition::FullyInserted,
            ControlRodClusterPosition::FullyInserted,
            ControlRodClusterPosition::StuckRod {
                cluster_id: "RCC-C4".to_string(),
                last_known_steps: 12,
                alarm_active: true,
            },
        ],
        coolant_loops: vec![
            PrimaryCoolantLoopStatus::NaturalCirculation {
                loop_id: 1,
                estimated_flow_kg_per_s: 320.0,
            },
            PrimaryCoolantLoopStatus::NaturalCirculation {
                loop_id: 2,
                estimated_flow_kg_per_s: 310.0,
            },
            PrimaryCoolantLoopStatus::Isolated {
                loop_id: 3,
                isolation_reason: "RCP trip on under-voltage".to_string(),
                valves_closed: vec!["V-31A".to_string(), "V-31B".to_string()],
            },
        ],
        containment: ContainmentMonitoring::IntegrityChallenge {
            breach_location: None,
            leak_rate_pct_per_day: 0.15,
            hydrogen_concentration_pct: 2.1,
            recombiner_status: HydrogenRecombinerStatus::Operating {
                recombination_rate_kg_per_hr: 0.08,
            },
        },
        eccs: EmergencyCoolingState::Actuated {
            train: SafetyTrain::BothTrains,
            flow_rate_kg_per_s: 95.0,
            injection_mode: InjectionMode::HighPressure,
        },
        turbine: TurbineGeneratorStatus::Tripped {
            trip_cause: "Reactor trip signal".to_string(),
            coast_down_rpm: 450,
        },
        spent_fuel: Some(SpentFuelPoolMonitoring {
            pool_id: "SFP-1".to_string(),
            water_temperature_c: 38.0,
            water_level_m: 7.8,
            boron_concentration_ppm: 2500.0,
            cooling_status: SpentFuelCoolingStatus::NormalCooling {
                inlet_temp_c: 35.0,
                outlet_temp_c: 41.0,
            },
            radiation_dose_rate_usv_per_hr: 3.2,
            assemblies_stored: 1100,
        }),
        neutron_flux: vec![
            NeutronFluxMeasurement::SourceRange {
                detector_id: "N31".to_string(),
                count_rate_cps: 1_500_000.0,
                startup_rate_dpm: -0.5,
            },
            NeutronFluxMeasurement::IntermediateRange {
                detector_id: "N35".to_string(),
                current_amps: 1.2e-6,
                power_pct: 0.001,
            },
        ],
        decay_heat: Some(DecayHeatRemoval::ResidualHeatRemoval {
            train: SafetyTrain::TrainA,
            rhr_flow_rate_kg_per_s: 250.0,
            rhr_heat_exchanger_outlet_c: 55.0,
        }),
    };
    let bytes = encode_to_vec(&snapshot).expect("encode full plant snapshot");
    let (decoded, _) =
        decode_from_slice::<PlantSnapshot>(&bytes).expect("decode full plant snapshot");
    assert_eq!(snapshot, decoded);
}

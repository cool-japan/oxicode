//! Advanced tests for smart home IoT ecosystem domain types.
//! 22 test functions covering complex enums, nested enums, and rich domain modeling.

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
enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ColorMode {
    Rgb {
        r: u8,
        g: u8,
        b: u8,
    },
    Temperature {
        kelvin: u16,
    },
    Hsl {
        hue: u16,
        saturation: u8,
        lightness: u8,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LockState {
    Locked,
    Unlocked,
    Jammed,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SecurityMode {
    Away { armed_sensors: Vec<String> },
    Home { bypassed_zones: Vec<u16> },
    Night { perimeter_only: bool },
    Disarmed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CameraEventKind {
    Motion { confidence: u8 },
    PersonDetected { count: u8, familiar: bool },
    PackageDetected { at_door: bool },
    VehicleDetected { plate: Option<String> },
    AnimalDetected { species: String },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CameraEvent {
    camera_id: String,
    timestamp_ms: u64,
    kind: CameraEventKind,
    thumbnail_bytes: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DeviceType {
    Thermostat {
        current_temp: i16,
        target_temp: i16,
        unit: TemperatureUnit,
        humidity_pct: u8,
    },
    Light {
        brightness: u8,
        color: ColorMode,
        on: bool,
    },
    Lock {
        state: LockState,
        battery_pct: u8,
        auto_lock_secs: Option<u32>,
    },
    Camera {
        resolution_w: u16,
        resolution_h: u16,
        night_vision: bool,
        recent_events: Vec<CameraEvent>,
    },
    Sensor(SensorReading),
    Speaker {
        volume: u8,
        playing: Option<String>,
        grouped_with: Vec<String>,
    },
    Appliance {
        model: String,
        firmware: FirmwareState,
        power_watts: u16,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SensorReading {
    Temperature {
        value_c_x10: i16,
    },
    Humidity {
        pct: u8,
    },
    WaterLeak {
        detected: bool,
        flow_ml_per_min: Option<u32>,
    },
    SmokeDetected {
        density_ppm: u16,
    },
    CoDetected {
        level_ppm: u16,
    },
    Motion {
        active: bool,
    },
    Door {
        open: bool,
    },
    Light {
        lux: u32,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FirmwareState {
    UpToDate {
        version: String,
    },
    UpdateAvailable {
        current: String,
        latest: String,
        size_bytes: u32,
    },
    Downloading {
        progress_pct: u8,
    },
    Installing {
        step: u8,
        total_steps: u8,
    },
    Failed {
        error_code: u16,
        message: String,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TriggerCondition {
    TimeOfDay {
        hour: u8,
        minute: u8,
    },
    SensorThreshold {
        sensor_id: String,
        reading: SensorReading,
    },
    Geofence {
        lat_x1e6: i32,
        lon_x1e6: i32,
        radius_m: u32,
        entering: bool,
    },
    VoiceCommand {
        phrase: String,
        assistant: String,
    },
    DeviceStateChange {
        device_id: String,
        previous: Box<DeviceType>,
        current: Box<DeviceType>,
    },
    Composite {
        all_of: Vec<TriggerCondition>,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ActionType {
    TurnOn {
        device_id: String,
    },
    TurnOff {
        device_id: String,
    },
    Dim {
        device_id: String,
        level: u8,
    },
    SetTemperature {
        device_id: String,
        temp: i16,
        unit: TemperatureUnit,
    },
    LockDoor {
        device_id: String,
    },
    UnlockDoor {
        device_id: String,
    },
    Notify {
        title: String,
        body: String,
        targets: Vec<String>,
    },
    SetColor {
        device_id: String,
        color: ColorMode,
    },
    RunScene {
        scene_name: String,
    },
    Delay {
        millis: u32,
        then: Box<ActionType>,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AutomationRule {
    rule_id: u32,
    name: String,
    enabled: bool,
    trigger: TriggerCondition,
    actions: Vec<ActionType>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Scene {
    scene_name: String,
    actions: Vec<ActionType>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EnergyRecord {
    device_id: String,
    watt_hours: u32,
    peak_watts: u16,
    device: DeviceType,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum HvacMode {
    Heat,
    Cool,
    Auto,
    FanOnly,
    Off,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HvacZone {
    zone_id: u16,
    name: String,
    mode: HvacMode,
    target_temp_x10: i16,
    damper_open_pct: u8,
    sensors: Vec<SensorReading>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GarageDoorState {
    Open,
    Closed,
    Opening { progress_pct: u8 },
    Closing { progress_pct: u8 },
    Stopped { at_pct: u8, reason: String },
    Error { code: u16 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum IrrigationDayMask {
    EveryDay,
    OddDays,
    EvenDays,
    Custom { days: Vec<u8> },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IrrigationSchedule {
    zone_name: String,
    start_hour: u8,
    start_minute: u8,
    duration_minutes: u16,
    day_mask: IrrigationDayMask,
    rain_skip: bool,
    soil_moisture_threshold: Option<u8>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VoiceRoutine {
    trigger_phrase: String,
    actions: Vec<ActionType>,
    confirmation_response: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MeshNodeRole {
    Router,
    EndDevice,
    Coordinator,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MeshNode {
    node_id: String,
    role: MeshNodeRole,
    signal_dbm: i8,
    parent_id: Option<String>,
    children: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MeshTopology {
    network_name: String,
    channel: u8,
    nodes: Vec<MeshNode>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

// --- Test 1: DeviceType::Thermostat roundtrip ---
#[test]
fn test_thermostat_device_roundtrip() {
    let val = DeviceType::Thermostat {
        current_temp: 215,
        target_temp: 220,
        unit: TemperatureUnit::Celsius,
        humidity_pct: 48,
    };
    let bytes = encode_to_vec(&val).expect("encode thermostat");
    let (decoded, _): (DeviceType, usize) = decode_from_slice(&bytes).expect("decode thermostat");
    assert_eq!(val, decoded);
}

// --- Test 2: DeviceType::Light with RGB color ---
#[test]
fn test_light_rgb_roundtrip() {
    let val = DeviceType::Light {
        brightness: 200,
        color: ColorMode::Rgb {
            r: 255,
            g: 128,
            b: 0,
        },
        on: true,
    };
    let bytes = encode_to_vec(&val).expect("encode light rgb");
    let (decoded, _): (DeviceType, usize) = decode_from_slice(&bytes).expect("decode light rgb");
    assert_eq!(val, decoded);
}

// --- Test 3: DeviceType::Lock with auto-lock ---
#[test]
fn test_lock_auto_lock_roundtrip() {
    let val = DeviceType::Lock {
        state: LockState::Locked,
        battery_pct: 87,
        auto_lock_secs: Some(300),
    };
    let bytes = encode_to_vec(&val).expect("encode lock");
    let (decoded, _): (DeviceType, usize) = decode_from_slice(&bytes).expect("decode lock");
    assert_eq!(val, decoded);
}

// --- Test 4: Camera with multiple events ---
#[test]
fn test_camera_events_roundtrip() {
    let val = DeviceType::Camera {
        resolution_w: 1920,
        resolution_h: 1080,
        night_vision: true,
        recent_events: vec![
            CameraEvent {
                camera_id: "cam_front".into(),
                timestamp_ms: 1700000000000,
                kind: CameraEventKind::PersonDetected {
                    count: 2,
                    familiar: true,
                },
                thumbnail_bytes: vec![0xFF, 0xD8, 0xFF],
            },
            CameraEvent {
                camera_id: "cam_front".into(),
                timestamp_ms: 1700000060000,
                kind: CameraEventKind::PackageDetected { at_door: true },
                thumbnail_bytes: vec![0xFF, 0xD8],
            },
        ],
    };
    let bytes = encode_to_vec(&val).expect("encode camera");
    let (decoded, _): (DeviceType, usize) = decode_from_slice(&bytes).expect("decode camera");
    assert_eq!(val, decoded);
}

// --- Test 5: SecurityMode::Away with armed sensors ---
#[test]
fn test_security_mode_away_roundtrip() {
    let val = SecurityMode::Away {
        armed_sensors: vec![
            "motion_hallway".into(),
            "door_front".into(),
            "window_kitchen".into(),
        ],
    };
    let bytes = encode_to_vec(&val).expect("encode security away");
    let (decoded, _): (SecurityMode, usize) =
        decode_from_slice(&bytes).expect("decode security away");
    assert_eq!(val, decoded);
}

// --- Test 6: SecurityMode::Night roundtrip ---
#[test]
fn test_security_mode_night_roundtrip() {
    let val = SecurityMode::Night {
        perimeter_only: true,
    };
    let bytes = encode_to_vec(&val).expect("encode security night");
    let (decoded, _): (SecurityMode, usize) =
        decode_from_slice(&bytes).expect("decode security night");
    assert_eq!(val, decoded);
}

// --- Test 7: CameraEventKind::VehicleDetected with plate ---
#[test]
fn test_camera_vehicle_with_plate_roundtrip() {
    let val = CameraEventKind::VehicleDetected {
        plate: Some("ABC1234".into()),
    };
    let bytes = encode_to_vec(&val).expect("encode vehicle event");
    let (decoded, _): (CameraEventKind, usize) =
        decode_from_slice(&bytes).expect("decode vehicle event");
    assert_eq!(val, decoded);
}

// --- Test 8: Sensor water leak detection ---
#[test]
fn test_water_leak_sensor_roundtrip() {
    let val = DeviceType::Sensor(SensorReading::WaterLeak {
        detected: true,
        flow_ml_per_min: Some(150),
    });
    let bytes = encode_to_vec(&val).expect("encode water leak");
    let (decoded, _): (DeviceType, usize) = decode_from_slice(&bytes).expect("decode water leak");
    assert_eq!(val, decoded);
}

// --- Test 9: Smoke/CO detector alerts ---
#[test]
fn test_smoke_co_alerts_roundtrip() {
    let readings = vec![
        SensorReading::SmokeDetected { density_ppm: 320 },
        SensorReading::CoDetected { level_ppm: 55 },
    ];
    let bytes = encode_to_vec(&readings).expect("encode smoke co");
    let (decoded, _): (Vec<SensorReading>, usize) =
        decode_from_slice(&bytes).expect("decode smoke co");
    assert_eq!(readings, decoded);
}

// --- Test 10: AutomationRule with composite trigger ---
#[test]
fn test_automation_composite_trigger_roundtrip() {
    let rule = AutomationRule {
        rule_id: 42,
        name: "Evening routine".into(),
        enabled: true,
        trigger: TriggerCondition::Composite {
            all_of: vec![
                TriggerCondition::TimeOfDay {
                    hour: 18,
                    minute: 30,
                },
                TriggerCondition::SensorThreshold {
                    sensor_id: "lux_living".into(),
                    reading: SensorReading::Light { lux: 200 },
                },
            ],
        },
        actions: vec![
            ActionType::Dim {
                device_id: "light_living".into(),
                level: 60,
            },
            ActionType::SetColor {
                device_id: "light_living".into(),
                color: ColorMode::Temperature { kelvin: 2700 },
            },
        ],
    };
    let bytes = encode_to_vec(&rule).expect("encode composite rule");
    let (decoded, _): (AutomationRule, usize) =
        decode_from_slice(&bytes).expect("decode composite rule");
    assert_eq!(rule, decoded);
}

// --- Test 11: Scene composition with nested delay ---
#[test]
fn test_scene_with_delay_roundtrip() {
    let scene = Scene {
        scene_name: "Goodnight".into(),
        actions: vec![
            ActionType::TurnOff {
                device_id: "light_bedroom".into(),
            },
            ActionType::LockDoor {
                device_id: "lock_front".into(),
            },
            ActionType::Delay {
                millis: 5000,
                then: Box::new(ActionType::Notify {
                    title: "Goodnight".into(),
                    body: "All doors locked, lights off".into(),
                    targets: vec!["phone_main".into()],
                }),
            },
        ],
    };
    let bytes = encode_to_vec(&scene).expect("encode scene");
    let (decoded, _): (Scene, usize) = decode_from_slice(&bytes).expect("decode scene");
    assert_eq!(scene, decoded);
}

// --- Test 12: Energy monitoring records ---
#[test]
fn test_energy_records_roundtrip() {
    let records = vec![
        EnergyRecord {
            device_id: "washer_01".into(),
            watt_hours: 1250,
            peak_watts: 2100,
            device: DeviceType::Appliance {
                model: "WashMaster 9000".into(),
                firmware: FirmwareState::UpToDate {
                    version: "3.2.1".into(),
                },
                power_watts: 450,
            },
        },
        EnergyRecord {
            device_id: "hvac_main".into(),
            watt_hours: 8700,
            peak_watts: 3500,
            device: DeviceType::Thermostat {
                current_temp: 198,
                target_temp: 210,
                unit: TemperatureUnit::Fahrenheit,
                humidity_pct: 55,
            },
        },
    ];
    let bytes = encode_to_vec(&records).expect("encode energy");
    let (decoded, _): (Vec<EnergyRecord>, usize) =
        decode_from_slice(&bytes).expect("decode energy");
    assert_eq!(records, decoded);
}

// --- Test 13: HVAC zone configuration ---
#[test]
fn test_hvac_zone_config_roundtrip() {
    let zone = HvacZone {
        zone_id: 3,
        name: "Upstairs bedrooms".into(),
        mode: HvacMode::Auto,
        target_temp_x10: 215,
        damper_open_pct: 75,
        sensors: vec![
            SensorReading::Temperature { value_c_x10: 228 },
            SensorReading::Humidity { pct: 42 },
        ],
    };
    let bytes = encode_to_vec(&zone).expect("encode hvac zone");
    let (decoded, _): (HvacZone, usize) = decode_from_slice(&bytes).expect("decode hvac zone");
    assert_eq!(zone, decoded);
}

// --- Test 14: GarageDoorState stopped mid-travel ---
#[test]
fn test_garage_door_stopped_roundtrip() {
    let val = GarageDoorState::Stopped {
        at_pct: 42,
        reason: "Obstruction detected".into(),
    };
    let bytes = encode_to_vec(&val).expect("encode garage stopped");
    let (decoded, _): (GarageDoorState, usize) =
        decode_from_slice(&bytes).expect("decode garage stopped");
    assert_eq!(val, decoded);
}

// --- Test 15: Irrigation schedule with custom days ---
#[test]
fn test_irrigation_custom_days_roundtrip() {
    let schedule = IrrigationSchedule {
        zone_name: "Front lawn".into(),
        start_hour: 5,
        start_minute: 30,
        duration_minutes: 20,
        day_mask: IrrigationDayMask::Custom {
            days: vec![1, 3, 5],
        },
        rain_skip: true,
        soil_moisture_threshold: Some(30),
    };
    let bytes = encode_to_vec(&schedule).expect("encode irrigation");
    let (decoded, _): (IrrigationSchedule, usize) =
        decode_from_slice(&bytes).expect("decode irrigation");
    assert_eq!(schedule, decoded);
}

// --- Test 16: Voice routine with chained actions ---
#[test]
fn test_voice_routine_roundtrip() {
    let routine = VoiceRoutine {
        trigger_phrase: "I'm leaving".into(),
        actions: vec![
            ActionType::TurnOff {
                device_id: "light_all".into(),
            },
            ActionType::LockDoor {
                device_id: "lock_front".into(),
            },
            ActionType::LockDoor {
                device_id: "lock_back".into(),
            },
            ActionType::SetTemperature {
                device_id: "thermostat_main".into(),
                temp: 160,
                unit: TemperatureUnit::Celsius,
            },
            ActionType::RunScene {
                scene_name: "Away".into(),
            },
        ],
        confirmation_response: "OK, locking up and setting away mode.".into(),
    };
    let bytes = encode_to_vec(&routine).expect("encode voice routine");
    let (decoded, _): (VoiceRoutine, usize) =
        decode_from_slice(&bytes).expect("decode voice routine");
    assert_eq!(routine, decoded);
}

// --- Test 17: FirmwareState::Installing roundtrip ---
#[test]
fn test_firmware_installing_roundtrip() {
    let val = FirmwareState::Installing {
        step: 3,
        total_steps: 7,
    };
    let bytes = encode_to_vec(&val).expect("encode firmware installing");
    let (decoded, _): (FirmwareState, usize) =
        decode_from_slice(&bytes).expect("decode firmware installing");
    assert_eq!(val, decoded);
}

// --- Test 18: Mesh network topology ---
#[test]
fn test_mesh_topology_roundtrip() {
    let topo = MeshTopology {
        network_name: "HomeZigbee".into(),
        channel: 15,
        nodes: vec![
            MeshNode {
                node_id: "coord_01".into(),
                role: MeshNodeRole::Coordinator,
                signal_dbm: -30,
                parent_id: None,
                children: vec!["router_a".into(), "router_b".into()],
            },
            MeshNode {
                node_id: "router_a".into(),
                role: MeshNodeRole::Router,
                signal_dbm: -55,
                parent_id: Some("coord_01".into()),
                children: vec!["end_1".into(), "end_2".into()],
            },
            MeshNode {
                node_id: "end_1".into(),
                role: MeshNodeRole::EndDevice,
                signal_dbm: -72,
                parent_id: Some("router_a".into()),
                children: vec![],
            },
        ],
    };
    let bytes = encode_to_vec(&topo).expect("encode mesh topology");
    let (decoded, _): (MeshTopology, usize) =
        decode_from_slice(&bytes).expect("decode mesh topology");
    assert_eq!(topo, decoded);
}

// --- Test 19: DeviceStateChange trigger with boxed enums ---
#[test]
fn test_device_state_change_trigger_roundtrip() {
    let trigger = TriggerCondition::DeviceStateChange {
        device_id: "lock_front".into(),
        previous: Box::new(DeviceType::Lock {
            state: LockState::Locked,
            battery_pct: 90,
            auto_lock_secs: Some(300),
        }),
        current: Box::new(DeviceType::Lock {
            state: LockState::Unlocked,
            battery_pct: 90,
            auto_lock_secs: Some(300),
        }),
    };
    let bytes = encode_to_vec(&trigger).expect("encode state change trigger");
    let (decoded, _): (TriggerCondition, usize) =
        decode_from_slice(&bytes).expect("decode state change trigger");
    assert_eq!(trigger, decoded);
}

// --- Test 20: Geofence trigger roundtrip ---
#[test]
fn test_geofence_trigger_roundtrip() {
    let trigger = TriggerCondition::Geofence {
        lat_x1e6: 37_774_930,
        lon_x1e6: -122_419_420,
        radius_m: 150,
        entering: false,
    };
    let bytes = encode_to_vec(&trigger).expect("encode geofence");
    let (decoded, _): (TriggerCondition, usize) =
        decode_from_slice(&bytes).expect("decode geofence");
    assert_eq!(trigger, decoded);
}

// --- Test 21: Deeply nested action with multiple delays ---
#[test]
fn test_deeply_nested_delayed_actions_roundtrip() {
    let action = ActionType::Delay {
        millis: 1000,
        then: Box::new(ActionType::Delay {
            millis: 2000,
            then: Box::new(ActionType::Delay {
                millis: 3000,
                then: Box::new(ActionType::Notify {
                    title: "Staged warmup".into(),
                    body: "All three delays completed".into(),
                    targets: vec!["phone_a".into(), "tablet_b".into()],
                }),
            }),
        }),
    };
    let bytes = encode_to_vec(&action).expect("encode nested delay");
    let (decoded, _): (ActionType, usize) = decode_from_slice(&bytes).expect("decode nested delay");
    assert_eq!(action, decoded);
}

// --- Test 22: Full smart home snapshot with mixed device types ---
#[test]
fn test_full_home_snapshot_roundtrip() {
    let snapshot: Vec<DeviceType> = vec![
        DeviceType::Thermostat {
            current_temp: 210,
            target_temp: 215,
            unit: TemperatureUnit::Celsius,
            humidity_pct: 50,
        },
        DeviceType::Light {
            brightness: 255,
            color: ColorMode::Hsl {
                hue: 240,
                saturation: 80,
                lightness: 50,
            },
            on: true,
        },
        DeviceType::Lock {
            state: LockState::Jammed,
            battery_pct: 12,
            auto_lock_secs: None,
        },
        DeviceType::Sensor(SensorReading::Door { open: true }),
        DeviceType::Speaker {
            volume: 45,
            playing: Some("Jazz FM".into()),
            grouped_with: vec!["speaker_kitchen".into()],
        },
        DeviceType::Appliance {
            model: "SmartOven X1".into(),
            firmware: FirmwareState::UpdateAvailable {
                current: "1.0.0".into(),
                latest: "1.2.3".into(),
                size_bytes: 4_194_304,
            },
            power_watts: 1800,
        },
        DeviceType::Camera {
            resolution_w: 3840,
            resolution_h: 2160,
            night_vision: false,
            recent_events: vec![CameraEvent {
                camera_id: "cam_yard".into(),
                timestamp_ms: 1700001000000,
                kind: CameraEventKind::AnimalDetected {
                    species: "raccoon".into(),
                },
                thumbnail_bytes: vec![0x89, 0x50, 0x4E, 0x47],
            }],
        },
    ];
    let bytes = encode_to_vec(&snapshot).expect("encode home snapshot");
    let (decoded, _): (Vec<DeviceType>, usize) =
        decode_from_slice(&bytes).expect("decode home snapshot");
    assert_eq!(snapshot, decoded);
}

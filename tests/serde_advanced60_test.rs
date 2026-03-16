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
use ::serde::{Deserialize, Serialize};
use oxicode::config;
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};

// --- Domain types: Solar Panel Installation & Maintenance ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SiteSurveyAssessment {
    survey_id: u64,
    address: String,
    roof_pitch_degrees: f64,
    azimuth_degrees: f64,
    roof_area_sqft: f64,
    roof_material: String,
    structural_rating: u8,
    shading_loss_pct: f64,
    tree_obstruction_count: u32,
    annual_sun_hours: f64,
    surveyor_name: String,
    survey_date: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ShadingAnalysis {
    analysis_id: u64,
    site_id: u64,
    hour_of_day: u8,
    month: u8,
    shade_fraction: f64,
    source_description: String,
    impact_on_production_pct: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PanelLayoutDesign {
    design_id: u64,
    site_id: u64,
    panel_model: String,
    panel_wattage: u32,
    total_panels: u32,
    strings: Vec<StringConfiguration>,
    system_size_kw: f64,
    tilt_angle: f64,
    orientation: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct StringConfiguration {
    string_id: u32,
    panels_in_string: u32,
    voltage_voc: f64,
    current_isc: f64,
    mppt_channel: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct InverterSizing {
    inverter_id: u64,
    manufacturer: String,
    model: String,
    rated_power_kw: f64,
    max_input_voltage: f64,
    mppt_channels: u8,
    efficiency_pct: f64,
    dc_ac_ratio: f64,
    assigned_strings: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PermitApplication {
    permit_id: u64,
    jurisdiction: String,
    applicant_name: String,
    project_address: String,
    system_size_kw: f64,
    application_date: String,
    status: String,
    plan_set_version: String,
    structural_stamp_required: bool,
    electrical_stamp_required: bool,
    fee_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CrewSchedule {
    schedule_id: u64,
    crew_lead: String,
    crew_members: Vec<String>,
    installation_date: String,
    site_address: String,
    estimated_hours: f64,
    equipment_list: Vec<String>,
    weather_forecast: String,
    backup_date: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ElectricalInspectionReport {
    report_id: u64,
    inspector_name: String,
    inspection_date: String,
    permit_id: u64,
    grounding_pass: bool,
    conduit_pass: bool,
    wiring_pass: bool,
    disconnect_pass: bool,
    label_pass: bool,
    overall_result: String,
    deficiency_notes: Vec<String>,
    reinspection_required: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct InterconnectionAgreement {
    agreement_id: u64,
    utility_name: String,
    customer_name: String,
    account_number: String,
    system_size_kw: f64,
    meter_number: String,
    interconnection_type: String,
    application_date: String,
    approval_date: Option<String>,
    status: String,
    insurance_required: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ProductionMonitoringDaily {
    record_id: u64,
    system_id: u64,
    date: String,
    kwh_produced: f64,
    peak_power_kw: f64,
    sunshine_hours: f64,
    ambient_temp_c: f64,
    panel_temp_c: f64,
    performance_ratio: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ProductionMonitoringMonthly {
    system_id: u64,
    year: u16,
    month: u8,
    total_kwh: f64,
    expected_kwh: f64,
    deviation_pct: f64,
    peak_day_kwh: f64,
    lowest_day_kwh: f64,
    average_daily_kwh: f64,
    days_below_threshold: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PanelDegradationRecord {
    panel_serial: String,
    installation_date: String,
    measurement_date: String,
    original_wattage: f64,
    measured_wattage: f64,
    degradation_pct: f64,
    cumulative_degradation_pct: f64,
    years_in_service: f64,
    annual_degradation_rate: f64,
    within_warranty_spec: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct InverterFaultRecord {
    fault_id: u64,
    inverter_serial: String,
    fault_code: u32,
    fault_description: String,
    severity: String,
    timestamp: String,
    dc_voltage_at_fault: f64,
    ac_voltage_at_fault: f64,
    temperature_c_at_fault: f64,
    auto_cleared: bool,
    downtime_minutes: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BatteryStoragePairing {
    pairing_id: u64,
    battery_model: String,
    battery_serial: String,
    capacity_kwh: f64,
    usable_capacity_kwh: f64,
    max_charge_rate_kw: f64,
    max_discharge_rate_kw: f64,
    inverter_serial: String,
    operating_mode: String,
    backup_reserve_pct: f64,
    cycles_to_date: u32,
    state_of_health_pct: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct NetMeteringBill {
    bill_id: u64,
    billing_period_start: String,
    billing_period_end: String,
    kwh_consumed: f64,
    kwh_exported: f64,
    net_kwh: f64,
    credit_balance_kwh: f64,
    retail_rate_cents: f64,
    export_rate_cents: f64,
    total_charges_cents: i64,
    credits_applied_cents: i64,
    amount_due_cents: i64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SolarRenewableEnergyCertificate {
    srec_id: u64,
    system_id: u64,
    vintage_year: u16,
    vintage_quarter: u8,
    mwh_generated: f64,
    certificates_issued: u32,
    market_price_cents: u64,
    state_program: String,
    status: String,
    broker_name: Option<String>,
    sale_date: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct WarrantyClaim {
    claim_id: u64,
    product_serial: String,
    product_type: String,
    manufacturer: String,
    installation_date: String,
    claim_date: String,
    issue_description: String,
    warranty_type: String,
    warranty_years: u32,
    claim_status: String,
    replacement_serial: Option<String>,
    labor_covered: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CustomerFinancing {
    contract_id: u64,
    customer_name: String,
    financing_type: String,
    system_cost_cents: u64,
    down_payment_cents: u64,
    term_months: u32,
    interest_rate_bps: u32,
    monthly_payment_cents: u64,
    escalator_pct: f64,
    itc_amount_cents: u64,
    contract_start_date: String,
    buyout_price_cents: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DroneInspectionResult {
    inspection_id: u64,
    system_id: u64,
    flight_date: String,
    drone_model: String,
    thermal_camera: String,
    hotspot_findings: Vec<ThermalHotspot>,
    cracked_cells_count: u32,
    soiling_rating: u8,
    physical_damage_notes: Vec<String>,
    overall_health_score: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ThermalHotspot {
    panel_position: String,
    delta_temp_c: f64,
    hotspot_area_cm2: f64,
    severity: String,
    probable_cause: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MaintenanceWorkOrder {
    work_order_id: u64,
    system_id: u64,
    requested_date: String,
    scheduled_date: Option<String>,
    task_type: String,
    priority: String,
    description: String,
    assigned_technician: Option<String>,
    parts_required: Vec<String>,
    completed: bool,
    completion_notes: Option<String>,
}

// --- Tests ---

#[test]
fn test_site_survey_assessment_roundtrip() {
    let cfg = config::standard();
    let survey = SiteSurveyAssessment {
        survey_id: 90001,
        address: "742 Evergreen Terrace, Springfield".into(),
        roof_pitch_degrees: 22.5,
        azimuth_degrees: 185.0,
        roof_area_sqft: 1800.0,
        roof_material: "Asphalt shingle, 30-year architectural".into(),
        structural_rating: 4,
        shading_loss_pct: 8.3,
        tree_obstruction_count: 2,
        annual_sun_hours: 1650.0,
        surveyor_name: "Carlos Mendez".into(),
        survey_date: "2026-02-10".into(),
    };
    let bytes = encode_to_vec(&survey, cfg).expect("encode site survey");
    let (decoded, _): (SiteSurveyAssessment, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode site survey");
    assert_eq!(survey, decoded);
}

#[test]
fn test_shading_analysis_multiple_entries() {
    let cfg = config::standard();
    let analyses: Vec<ShadingAnalysis> = vec![
        ShadingAnalysis {
            analysis_id: 1001,
            site_id: 90001,
            hour_of_day: 9,
            month: 12,
            shade_fraction: 0.45,
            source_description: "Large oak tree, southeast corner".into(),
            impact_on_production_pct: 12.7,
        },
        ShadingAnalysis {
            analysis_id: 1002,
            site_id: 90001,
            hour_of_day: 15,
            month: 6,
            shade_fraction: 0.05,
            source_description: "Chimney shadow, minimal".into(),
            impact_on_production_pct: 1.2,
        },
        ShadingAnalysis {
            analysis_id: 1003,
            site_id: 90001,
            hour_of_day: 12,
            month: 3,
            shade_fraction: 0.0,
            source_description: "No shading at solar noon".into(),
            impact_on_production_pct: 0.0,
        },
    ];
    let bytes = encode_to_vec(&analyses, cfg).expect("encode shading analyses");
    let (decoded, _): (Vec<ShadingAnalysis>, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode shading analyses");
    assert_eq!(analyses, decoded);
}

#[test]
fn test_panel_layout_design_with_strings() {
    let cfg = config::standard();
    let layout = PanelLayoutDesign {
        design_id: 5001,
        site_id: 90001,
        panel_model: "REC Alpha Pure-R 430W".into(),
        panel_wattage: 430,
        total_panels: 28,
        strings: vec![
            StringConfiguration {
                string_id: 1,
                panels_in_string: 14,
                voltage_voc: 588.0,
                current_isc: 11.2,
                mppt_channel: 1,
            },
            StringConfiguration {
                string_id: 2,
                panels_in_string: 14,
                voltage_voc: 588.0,
                current_isc: 11.2,
                mppt_channel: 2,
            },
        ],
        system_size_kw: 12.04,
        tilt_angle: 22.5,
        orientation: "South-Southwest".into(),
    };
    let bytes = encode_to_vec(&layout, cfg).expect("encode panel layout");
    let (decoded, _): (PanelLayoutDesign, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode panel layout");
    assert_eq!(layout, decoded);
}

#[test]
fn test_inverter_sizing_roundtrip() {
    let cfg = config::standard();
    let inverter = InverterSizing {
        inverter_id: 7001,
        manufacturer: "Enphase Energy".into(),
        model: "IQ8A-72-2-US".into(),
        rated_power_kw: 10.08,
        max_input_voltage: 600.0,
        mppt_channels: 2,
        efficiency_pct: 97.5,
        dc_ac_ratio: 1.19,
        assigned_strings: vec![1, 2],
    };
    let bytes = encode_to_vec(&inverter, cfg).expect("encode inverter sizing");
    let (decoded, _): (InverterSizing, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode inverter sizing");
    assert_eq!(inverter, decoded);
}

#[test]
fn test_permit_application_roundtrip() {
    let cfg = config::standard();
    let permit = PermitApplication {
        permit_id: 20260301,
        jurisdiction: "County of Maricopa, AZ".into(),
        applicant_name: "SunBright Solar LLC".into(),
        project_address: "4521 Desert Vista Dr, Mesa, AZ 85201".into(),
        system_size_kw: 12.04,
        application_date: "2026-03-01".into(),
        status: "Under Review".into(),
        plan_set_version: "v2.1".into(),
        structural_stamp_required: true,
        electrical_stamp_required: true,
        fee_cents: 45000,
    };
    let bytes = encode_to_vec(&permit, cfg).expect("encode permit application");
    let (decoded, _): (PermitApplication, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode permit application");
    assert_eq!(permit, decoded);
}

#[test]
fn test_crew_schedule_with_backup_date() {
    let cfg = config::standard();
    let schedule = CrewSchedule {
        schedule_id: 3300,
        crew_lead: "Mike Torres".into(),
        crew_members: vec![
            "Jake Wilson".into(),
            "Priya Sharma".into(),
            "Liam O'Brien".into(),
        ],
        installation_date: "2026-03-20".into(),
        site_address: "1025 Maple Lane, Tempe, AZ".into(),
        estimated_hours: 8.5,
        equipment_list: vec![
            "Racking system (IronRidge XR100)".into(),
            "Panel clamps x56".into(),
            "Conduit EMT 3/4 inch x40ft".into(),
            "Junction box NEMA 3R".into(),
        ],
        weather_forecast: "Sunny, high 78F, wind 5mph".into(),
        backup_date: Some("2026-03-22".into()),
    };
    let bytes = encode_to_vec(&schedule, cfg).expect("encode crew schedule");
    let (decoded, _): (CrewSchedule, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode crew schedule");
    assert_eq!(schedule, decoded);
}

#[test]
fn test_crew_schedule_no_backup_date() {
    let cfg = config::standard();
    let schedule = CrewSchedule {
        schedule_id: 3301,
        crew_lead: "Ana Reyes".into(),
        crew_members: vec!["David Kim".into(), "Sarah Chen".into()],
        installation_date: "2026-03-25".into(),
        site_address: "880 Ironwood Ct, Chandler, AZ".into(),
        estimated_hours: 6.0,
        equipment_list: vec!["Microinverter kit x18".into(), "Racking rails x9".into()],
        weather_forecast: "Partly cloudy, high 82F".into(),
        backup_date: None,
    };
    let bytes = encode_to_vec(&schedule, cfg).expect("encode crew schedule no backup");
    let (decoded, _): (CrewSchedule, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode crew schedule no backup");
    assert_eq!(schedule, decoded);
}

#[test]
fn test_electrical_inspection_pass() {
    let cfg = config::standard();
    let report = ElectricalInspectionReport {
        report_id: 8800,
        inspector_name: "Robert Flanagan, PE".into(),
        inspection_date: "2026-03-28".into(),
        permit_id: 20260301,
        grounding_pass: true,
        conduit_pass: true,
        wiring_pass: true,
        disconnect_pass: true,
        label_pass: true,
        overall_result: "PASS".into(),
        deficiency_notes: vec![],
        reinspection_required: false,
    };
    let bytes = encode_to_vec(&report, cfg).expect("encode inspection pass");
    let (decoded, _): (ElectricalInspectionReport, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode inspection pass");
    assert_eq!(report, decoded);
}

#[test]
fn test_electrical_inspection_with_deficiencies() {
    let cfg = config::standard();
    let report = ElectricalInspectionReport {
        report_id: 8801,
        inspector_name: "Linda Chow".into(),
        inspection_date: "2026-04-02".into(),
        permit_id: 20260315,
        grounding_pass: true,
        conduit_pass: false,
        wiring_pass: true,
        disconnect_pass: true,
        label_pass: false,
        overall_result: "FAIL".into(),
        deficiency_notes: vec![
            "Conduit support spacing exceeds 4ft maximum at eave run".into(),
            "Missing NEC 690.56 rapid shutdown label at main panel".into(),
        ],
        reinspection_required: true,
    };
    let bytes = encode_to_vec(&report, cfg).expect("encode inspection deficiency");
    let (decoded, _): (ElectricalInspectionReport, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode inspection deficiency");
    assert_eq!(report, decoded);
}

#[test]
fn test_interconnection_agreement_approved() {
    let cfg = config::standard();
    let agreement = InterconnectionAgreement {
        agreement_id: 44001,
        utility_name: "Arizona Public Service (APS)".into(),
        customer_name: "Homeowner Jane Doe".into(),
        account_number: "APS-2026-887654".into(),
        system_size_kw: 12.04,
        meter_number: "MTR-99281".into(),
        interconnection_type: "Net Energy Metering".into(),
        application_date: "2026-03-05".into(),
        approval_date: Some("2026-03-29".into()),
        status: "Approved - PTO Granted".into(),
        insurance_required: false,
    };
    let bytes = encode_to_vec(&agreement, cfg).expect("encode interconnection");
    let (decoded, _): (InterconnectionAgreement, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode interconnection");
    assert_eq!(agreement, decoded);
}

#[test]
fn test_daily_production_monitoring() {
    let cfg = config::standard();
    let records: Vec<ProductionMonitoringDaily> = (1..=5)
        .map(|day| ProductionMonitoringDaily {
            record_id: 60000 + day,
            system_id: 9001,
            date: format!("2026-04-{:02}", day),
            kwh_produced: 38.5 + (day as f64) * 1.2,
            peak_power_kw: 9.8 + (day as f64) * 0.1,
            sunshine_hours: 7.5 + (day as f64) * 0.3,
            ambient_temp_c: 28.0 + (day as f64) * 0.5,
            panel_temp_c: 45.0 + (day as f64) * 0.8,
            performance_ratio: 0.82 + (day as f64) * 0.005,
        })
        .collect();
    let bytes = encode_to_vec(&records, cfg).expect("encode daily production");
    let (decoded, _): (Vec<ProductionMonitoringDaily>, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode daily production");
    assert_eq!(records, decoded);
}

#[test]
fn test_monthly_production_monitoring() {
    let cfg = config::standard();
    let monthly = ProductionMonitoringMonthly {
        system_id: 9001,
        year: 2026,
        month: 3,
        total_kwh: 1245.8,
        expected_kwh: 1300.0,
        deviation_pct: -4.17,
        peak_day_kwh: 52.3,
        lowest_day_kwh: 18.7,
        average_daily_kwh: 40.19,
        days_below_threshold: 3,
    };
    let bytes = encode_to_vec(&monthly, cfg).expect("encode monthly production");
    let (decoded, _): (ProductionMonitoringMonthly, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode monthly production");
    assert_eq!(monthly, decoded);
}

#[test]
fn test_panel_degradation_tracking() {
    let cfg = config::standard();
    let record = PanelDegradationRecord {
        panel_serial: "REC-2024-A7X-00451".into(),
        installation_date: "2024-06-15".into(),
        measurement_date: "2026-03-10".into(),
        original_wattage: 430.0,
        measured_wattage: 422.6,
        degradation_pct: 1.72,
        cumulative_degradation_pct: 1.72,
        years_in_service: 1.74,
        annual_degradation_rate: 0.99,
        within_warranty_spec: true,
    };
    let bytes = encode_to_vec(&record, cfg).expect("encode degradation");
    let (decoded, _): (PanelDegradationRecord, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode degradation");
    assert_eq!(record, decoded);
}

#[test]
fn test_inverter_fault_codes() {
    let cfg = config::standard();
    let faults: Vec<InverterFaultRecord> = vec![
        InverterFaultRecord {
            fault_id: 10001,
            inverter_serial: "ENP-IQ8-20240615-0042".into(),
            fault_code: 302,
            fault_description: "AC over-voltage: grid voltage exceeded upper limit".into(),
            severity: "Warning".into(),
            timestamp: "2026-03-12T14:22:07Z".into(),
            dc_voltage_at_fault: 385.2,
            ac_voltage_at_fault: 253.1,
            temperature_c_at_fault: 52.0,
            auto_cleared: true,
            downtime_minutes: 5,
        },
        InverterFaultRecord {
            fault_id: 10002,
            inverter_serial: "ENP-IQ8-20240615-0042".into(),
            fault_code: 501,
            fault_description: "Ground fault detected on DC side".into(),
            severity: "Critical".into(),
            timestamp: "2026-03-13T08:05:33Z".into(),
            dc_voltage_at_fault: 0.0,
            ac_voltage_at_fault: 240.1,
            temperature_c_at_fault: 31.0,
            auto_cleared: false,
            downtime_minutes: 480,
        },
    ];
    let bytes = encode_to_vec(&faults, cfg).expect("encode inverter faults");
    let (decoded, _): (Vec<InverterFaultRecord>, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode inverter faults");
    assert_eq!(faults, decoded);
}

#[test]
fn test_battery_storage_pairing() {
    let cfg = config::standard();
    let battery = BatteryStoragePairing {
        pairing_id: 2200,
        battery_model: "Tesla Powerwall 3".into(),
        battery_serial: "TW3-2026-US-009871".into(),
        capacity_kwh: 13.5,
        usable_capacity_kwh: 12.15,
        max_charge_rate_kw: 5.0,
        max_discharge_rate_kw: 11.5,
        inverter_serial: "ENP-IQ8-20240615-0042".into(),
        operating_mode: "Self-Consumption with TOU optimization".into(),
        backup_reserve_pct: 20.0,
        cycles_to_date: 387,
        state_of_health_pct: 98.2,
    };
    let bytes = encode_to_vec(&battery, cfg).expect("encode battery pairing");
    let (decoded, _): (BatteryStoragePairing, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode battery pairing");
    assert_eq!(battery, decoded);
}

#[test]
fn test_net_metering_bill_credit() {
    let cfg = config::standard();
    let bill = NetMeteringBill {
        bill_id: 77001,
        billing_period_start: "2026-02-15".into(),
        billing_period_end: "2026-03-14".into(),
        kwh_consumed: 820.0,
        kwh_exported: 950.0,
        net_kwh: -130.0,
        credit_balance_kwh: 215.0,
        retail_rate_cents: 14.5,
        export_rate_cents: 10.8,
        total_charges_cents: 11890,
        credits_applied_cents: 10260,
        amount_due_cents: 1630,
    };
    let bytes = encode_to_vec(&bill, cfg).expect("encode net metering bill");
    let (decoded, _): (NetMeteringBill, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode net metering bill");
    assert_eq!(bill, decoded);
}

#[test]
fn test_srec_certificates() {
    let cfg = config::standard();
    let srecs = vec![
        SolarRenewableEnergyCertificate {
            srec_id: 55001,
            system_id: 9001,
            vintage_year: 2026,
            vintage_quarter: 1,
            mwh_generated: 3.24,
            certificates_issued: 3,
            market_price_cents: 4200,
            state_program: "New Jersey SREC-II".into(),
            status: "Listed for Sale".into(),
            broker_name: Some("SRECTrade Inc.".into()),
            sale_date: None,
        },
        SolarRenewableEnergyCertificate {
            srec_id: 55002,
            system_id: 9001,
            vintage_year: 2025,
            vintage_quarter: 4,
            mwh_generated: 2.88,
            certificates_issued: 2,
            market_price_cents: 3950,
            state_program: "Massachusetts SMART".into(),
            status: "Sold".into(),
            broker_name: Some("Sol Systems".into()),
            sale_date: Some("2026-01-18".into()),
        },
    ];
    let bytes = encode_to_vec(&srecs, cfg).expect("encode SRECs");
    let (decoded, _): (Vec<SolarRenewableEnergyCertificate>, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SRECs");
    assert_eq!(srecs, decoded);
}

#[test]
fn test_warranty_claim_processing() {
    let cfg = config::standard();
    let claim = WarrantyClaim {
        claim_id: 66001,
        product_serial: "ENP-IQ8-20240615-0042".into(),
        product_type: "Microinverter".into(),
        manufacturer: "Enphase Energy".into(),
        installation_date: "2024-06-15".into(),
        claim_date: "2026-03-14".into(),
        issue_description:
            "Unit producing zero output, fault code 501 ground fault persistent after reset".into(),
        warranty_type: "Product Warranty".into(),
        warranty_years: 25,
        claim_status: "Approved - Replacement Shipped".into(),
        replacement_serial: Some("ENP-IQ8-20260320-1188".into()),
        labor_covered: true,
    };
    let bytes = encode_to_vec(&claim, cfg).expect("encode warranty claim");
    let (decoded, _): (WarrantyClaim, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode warranty claim");
    assert_eq!(claim, decoded);
}

#[test]
fn test_customer_financing_ppa() {
    let cfg = config::standard();
    let contract = CustomerFinancing {
        contract_id: 88001,
        customer_name: "Jane Doe".into(),
        financing_type: "PPA".into(),
        system_cost_cents: 3600000,
        down_payment_cents: 0,
        term_months: 300,
        interest_rate_bps: 0,
        monthly_payment_cents: 0,
        escalator_pct: 2.9,
        itc_amount_cents: 1080000,
        contract_start_date: "2026-04-01".into(),
        buyout_price_cents: Some(1200000),
    };
    let bytes = encode_to_vec(&contract, cfg).expect("encode PPA contract");
    let (decoded, _): (CustomerFinancing, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PPA contract");
    assert_eq!(contract, decoded);
}

#[test]
fn test_customer_financing_loan() {
    let cfg = config::standard();
    let contract = CustomerFinancing {
        contract_id: 88002,
        customer_name: "John Smith".into(),
        financing_type: "Solar Loan".into(),
        system_cost_cents: 3200000,
        down_payment_cents: 500000,
        term_months: 180,
        interest_rate_bps: 499,
        monthly_payment_cents: 21340,
        escalator_pct: 0.0,
        itc_amount_cents: 960000,
        contract_start_date: "2026-04-15".into(),
        buyout_price_cents: None,
    };
    let bytes = encode_to_vec(&contract, cfg).expect("encode loan contract");
    let (decoded, _): (CustomerFinancing, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode loan contract");
    assert_eq!(contract, decoded);
}

#[test]
fn test_drone_thermal_inspection() {
    let cfg = config::standard();
    let result = DroneInspectionResult {
        inspection_id: 11001,
        system_id: 9001,
        flight_date: "2026-03-10".into(),
        drone_model: "DJI Matrice 350 RTK".into(),
        thermal_camera: "DJI Zenmuse H30T".into(),
        hotspot_findings: vec![
            ThermalHotspot {
                panel_position: "Row 2, Panel 5".into(),
                delta_temp_c: 18.3,
                hotspot_area_cm2: 45.0,
                severity: "Moderate".into(),
                probable_cause: "Bypass diode failure in substring".into(),
            },
            ThermalHotspot {
                panel_position: "Row 3, Panel 1".into(),
                delta_temp_c: 7.2,
                hotspot_area_cm2: 12.0,
                severity: "Low".into(),
                probable_cause: "Bird dropping partial shading".into(),
            },
        ],
        cracked_cells_count: 1,
        soiling_rating: 2,
        physical_damage_notes: vec!["Minor frame scratch on Row 1, Panel 3 - cosmetic only".into()],
        overall_health_score: 91.5,
    };
    let bytes = encode_to_vec(&result, cfg).expect("encode drone inspection");
    let (decoded, _): (DroneInspectionResult, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode drone inspection");
    assert_eq!(result, decoded);
}

#[test]
fn test_maintenance_work_order_pending() {
    let cfg = config::standard();
    let order = MaintenanceWorkOrder {
        work_order_id: 44100,
        system_id: 9001,
        requested_date: "2026-03-14".into(),
        scheduled_date: None,
        task_type: "Corrective Maintenance".into(),
        priority: "High".into(),
        description: "Replace failed microinverter at string 1 position 7, ground fault 501".into(),
        assigned_technician: None,
        parts_required: vec![
            "Enphase IQ8A microinverter".into(),
            "MC4 connectors x2".into(),
            "Panel removal tool kit".into(),
        ],
        completed: false,
        completion_notes: None,
    };
    let bytes = encode_to_vec(&order, cfg).expect("encode work order pending");
    let (decoded, _): (MaintenanceWorkOrder, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode work order pending");
    assert_eq!(order, decoded);
}

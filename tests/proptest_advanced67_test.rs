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
use proptest::prelude::*;

// ── Domain types ─────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EnergySource {
    Coal,
    NaturalGas,
    Oil,
    Nuclear,
    Wind,
    Solar,
    Hydro,
    Biomass,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CertBody {
    VCS,
    GoldStandard,
    CDM,
    CAR,
    ACR,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EmissionScope {
    Scope1,
    Scope2,
    Scope3Direct,
    Scope3Indirect,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Co2Reading {
    station_id: u32,
    timestamp: u64,
    co2_ppm_x100: u32,
    ch4_ppb_x10: u32,
    n2o_ppb_x100: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EnergyConsumption {
    entity_id: u64,
    period_start: u64,
    period_end: u64,
    kwh_x100: u64,
    fuel_liters_x100: u32,
    source_type: EnergySource,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CarbonOffset {
    offset_id: u64,
    project_id: u32,
    tonnes_co2e_x100: u32,
    vintage_year: u16,
    certification: CertBody,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ClimatePrediction {
    model_id: u32,
    target_year: u16,
    temp_increase_x100: u32,
    sea_level_rise_mm: u32,
    confidence_pct: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EmissionRecord {
    entity_id: u64,
    scope: EmissionScope,
    co2e_tonnes_x100: u64,
    reporting_year: u16,
}

// ── Strategies ───────────────────────────────────────────────────────────────

fn arb_energy_source() -> impl Strategy<Value = EnergySource> {
    prop_oneof![
        Just(EnergySource::Coal),
        Just(EnergySource::NaturalGas),
        Just(EnergySource::Oil),
        Just(EnergySource::Nuclear),
        Just(EnergySource::Wind),
        Just(EnergySource::Solar),
        Just(EnergySource::Hydro),
        Just(EnergySource::Biomass),
    ]
}

fn arb_cert_body() -> impl Strategy<Value = CertBody> {
    prop_oneof![
        Just(CertBody::VCS),
        Just(CertBody::GoldStandard),
        Just(CertBody::CDM),
        Just(CertBody::CAR),
        Just(CertBody::ACR),
    ]
}

fn arb_emission_scope() -> impl Strategy<Value = EmissionScope> {
    prop_oneof![
        Just(EmissionScope::Scope1),
        Just(EmissionScope::Scope2),
        Just(EmissionScope::Scope3Direct),
        Just(EmissionScope::Scope3Indirect),
    ]
}

fn arb_co2_reading() -> impl Strategy<Value = Co2Reading> {
    (
        any::<u32>(),
        any::<u64>(),
        any::<u32>(),
        any::<u32>(),
        any::<u32>(),
    )
        .prop_map(
            |(station_id, timestamp, co2_ppm_x100, ch4_ppb_x10, n2o_ppb_x100)| Co2Reading {
                station_id,
                timestamp,
                co2_ppm_x100,
                ch4_ppb_x10,
                n2o_ppb_x100,
            },
        )
}

fn arb_energy_consumption() -> impl Strategy<Value = EnergyConsumption> {
    (
        any::<u64>(),
        any::<u64>(),
        any::<u64>(),
        any::<u64>(),
        any::<u32>(),
        arb_energy_source(),
    )
        .prop_map(
            |(entity_id, period_start, period_end, kwh_x100, fuel_liters_x100, source_type)| {
                EnergyConsumption {
                    entity_id,
                    period_start,
                    period_end,
                    kwh_x100,
                    fuel_liters_x100,
                    source_type,
                }
            },
        )
}

fn arb_carbon_offset() -> impl Strategy<Value = CarbonOffset> {
    (
        any::<u64>(),
        any::<u32>(),
        any::<u32>(),
        any::<u16>(),
        arb_cert_body(),
    )
        .prop_map(
            |(offset_id, project_id, tonnes_co2e_x100, vintage_year, certification)| CarbonOffset {
                offset_id,
                project_id,
                tonnes_co2e_x100,
                vintage_year,
                certification,
            },
        )
}

fn arb_climate_prediction() -> impl Strategy<Value = ClimatePrediction> {
    (
        any::<u32>(),
        any::<u16>(),
        any::<u32>(),
        any::<u32>(),
        any::<u8>(),
    )
        .prop_map(
            |(model_id, target_year, temp_increase_x100, sea_level_rise_mm, confidence_pct)| {
                ClimatePrediction {
                    model_id,
                    target_year,
                    temp_increase_x100,
                    sea_level_rise_mm,
                    confidence_pct,
                }
            },
        )
}

fn arb_emission_record() -> impl Strategy<Value = EmissionRecord> {
    (
        any::<u64>(),
        arb_emission_scope(),
        any::<u64>(),
        any::<u16>(),
    )
        .prop_map(
            |(entity_id, scope, co2e_tonnes_x100, reporting_year)| EmissionRecord {
                entity_id,
                scope,
                co2e_tonnes_x100,
                reporting_year,
            },
        )
}

// ── Tests ────────────────────────────────────────────────────────────────────

proptest! {
    // 1. Co2Reading roundtrip
    #[test]
    fn test_co2_reading_roundtrip(reading in arb_co2_reading()) {
        let bytes = encode_to_vec(&reading).expect("encode Co2Reading");
        let (decoded, _) = decode_from_slice::<Co2Reading>(&bytes).expect("decode Co2Reading");
        prop_assert_eq!(reading, decoded);
    }

    // 2. Co2Reading deterministic encoding
    #[test]
    fn test_co2_reading_deterministic(reading in arb_co2_reading()) {
        let bytes_a = encode_to_vec(&reading).expect("encode Co2Reading (a)");
        let bytes_b = encode_to_vec(&reading).expect("encode Co2Reading (b)");
        prop_assert_eq!(bytes_a, bytes_b);
    }

    // 3. Co2Reading consumed bytes == encoded length
    #[test]
    fn test_co2_reading_consumed_bytes(reading in arb_co2_reading()) {
        let bytes = encode_to_vec(&reading).expect("encode Co2Reading");
        let (_, consumed) = decode_from_slice::<Co2Reading>(&bytes).expect("decode Co2Reading");
        prop_assert_eq!(consumed, bytes.len());
    }

    // 4. EnergyConsumption roundtrip
    #[test]
    fn test_energy_consumption_roundtrip(rec in arb_energy_consumption()) {
        let bytes = encode_to_vec(&rec).expect("encode EnergyConsumption");
        let (decoded, _) = decode_from_slice::<EnergyConsumption>(&bytes).expect("decode EnergyConsumption");
        prop_assert_eq!(rec, decoded);
    }

    // 5. EnergyConsumption deterministic encoding
    #[test]
    fn test_energy_consumption_deterministic(rec in arb_energy_consumption()) {
        let bytes_a = encode_to_vec(&rec).expect("encode EnergyConsumption (a)");
        let bytes_b = encode_to_vec(&rec).expect("encode EnergyConsumption (b)");
        prop_assert_eq!(bytes_a, bytes_b);
    }

    // 6. EnergyConsumption consumed bytes == encoded length
    #[test]
    fn test_energy_consumption_consumed_bytes(rec in arb_energy_consumption()) {
        let bytes = encode_to_vec(&rec).expect("encode EnergyConsumption");
        let (_, consumed) = decode_from_slice::<EnergyConsumption>(&bytes).expect("decode EnergyConsumption");
        prop_assert_eq!(consumed, bytes.len());
    }

    // 7. CarbonOffset roundtrip
    #[test]
    fn test_carbon_offset_roundtrip(offset in arb_carbon_offset()) {
        let bytes = encode_to_vec(&offset).expect("encode CarbonOffset");
        let (decoded, _) = decode_from_slice::<CarbonOffset>(&bytes).expect("decode CarbonOffset");
        prop_assert_eq!(offset, decoded);
    }

    // 8. CarbonOffset consumed bytes == encoded length
    #[test]
    fn test_carbon_offset_consumed_bytes(offset in arb_carbon_offset()) {
        let bytes = encode_to_vec(&offset).expect("encode CarbonOffset");
        let (_, consumed) = decode_from_slice::<CarbonOffset>(&bytes).expect("decode CarbonOffset");
        prop_assert_eq!(consumed, bytes.len());
    }

    // 9. ClimatePrediction roundtrip
    #[test]
    fn test_climate_prediction_roundtrip(pred in arb_climate_prediction()) {
        let bytes = encode_to_vec(&pred).expect("encode ClimatePrediction");
        let (decoded, _) = decode_from_slice::<ClimatePrediction>(&bytes).expect("decode ClimatePrediction");
        prop_assert_eq!(pred, decoded);
    }

    // 10. ClimatePrediction deterministic encoding
    #[test]
    fn test_climate_prediction_deterministic(pred in arb_climate_prediction()) {
        let bytes_a = encode_to_vec(&pred).expect("encode ClimatePrediction (a)");
        let bytes_b = encode_to_vec(&pred).expect("encode ClimatePrediction (b)");
        prop_assert_eq!(bytes_a, bytes_b);
    }

    // 11. EmissionRecord roundtrip
    #[test]
    fn test_emission_record_roundtrip(rec in arb_emission_record()) {
        let bytes = encode_to_vec(&rec).expect("encode EmissionRecord");
        let (decoded, _) = decode_from_slice::<EmissionRecord>(&bytes).expect("decode EmissionRecord");
        prop_assert_eq!(rec, decoded);
    }

    // 12. EmissionRecord consumed bytes == encoded length
    #[test]
    fn test_emission_record_consumed_bytes(rec in arb_emission_record()) {
        let bytes = encode_to_vec(&rec).expect("encode EmissionRecord");
        let (_, consumed) = decode_from_slice::<EmissionRecord>(&bytes).expect("decode EmissionRecord");
        prop_assert_eq!(consumed, bytes.len());
    }

    // 13. Vec<Co2Reading> roundtrip
    #[test]
    fn test_vec_co2_readings_roundtrip(readings in prop::collection::vec(arb_co2_reading(), 0..16)) {
        let bytes = encode_to_vec(&readings).expect("encode Vec<Co2Reading>");
        let (decoded, _) = decode_from_slice::<Vec<Co2Reading>>(&bytes).expect("decode Vec<Co2Reading>");
        prop_assert_eq!(readings, decoded);
    }

    // 14. Vec<EmissionRecord> roundtrip
    #[test]
    fn test_vec_emission_records_roundtrip(recs in prop::collection::vec(arb_emission_record(), 0..16)) {
        let bytes = encode_to_vec(&recs).expect("encode Vec<EmissionRecord>");
        let (decoded, _) = decode_from_slice::<Vec<EmissionRecord>>(&bytes).expect("decode Vec<EmissionRecord>");
        prop_assert_eq!(recs, decoded);
    }

    // 15. Vec<CarbonOffset> consumed bytes == encoded length
    #[test]
    fn test_vec_carbon_offsets_consumed_bytes(offsets in prop::collection::vec(arb_carbon_offset(), 0..16)) {
        let bytes = encode_to_vec(&offsets).expect("encode Vec<CarbonOffset>");
        let (_, consumed) = decode_from_slice::<Vec<CarbonOffset>>(&bytes).expect("decode Vec<CarbonOffset>");
        prop_assert_eq!(consumed, bytes.len());
    }

    // 16. Option<Co2Reading> roundtrip — Some variant
    #[test]
    fn test_option_co2_reading_some_roundtrip(reading in arb_co2_reading()) {
        let value: Option<Co2Reading> = Some(reading);
        let bytes = encode_to_vec(&value).expect("encode Option<Co2Reading> Some");
        let (decoded, _) = decode_from_slice::<Option<Co2Reading>>(&bytes).expect("decode Option<Co2Reading> Some");
        prop_assert_eq!(value, decoded);
    }

    // 17. Option<Co2Reading> roundtrip — None variant
    #[test]
    fn test_option_co2_reading_none_roundtrip(_unused in any::<u8>()) {
        let value: Option<Co2Reading> = None;
        let bytes = encode_to_vec(&value).expect("encode Option<Co2Reading> None");
        let (decoded, consumed) = decode_from_slice::<Option<Co2Reading>>(&bytes).expect("decode Option<Co2Reading> None");
        prop_assert_eq!(value, decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    // 18. Option<CarbonOffset> roundtrip
    #[test]
    fn test_option_carbon_offset_roundtrip(opt in prop::option::of(arb_carbon_offset())) {
        let bytes = encode_to_vec(&opt).expect("encode Option<CarbonOffset>");
        let (decoded, consumed) = decode_from_slice::<Option<CarbonOffset>>(&bytes).expect("decode Option<CarbonOffset>");
        prop_assert_eq!(opt, decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    // 19. EnergySource enum roundtrip
    #[test]
    fn test_energy_source_roundtrip(src in arb_energy_source()) {
        let bytes = encode_to_vec(&src).expect("encode EnergySource");
        let (decoded, consumed) = decode_from_slice::<EnergySource>(&bytes).expect("decode EnergySource");
        prop_assert_eq!(src, decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    // 20. EmissionScope enum roundtrip
    #[test]
    fn test_emission_scope_roundtrip(scope in arb_emission_scope()) {
        let bytes = encode_to_vec(&scope).expect("encode EmissionScope");
        let (decoded, consumed) = decode_from_slice::<EmissionScope>(&bytes).expect("decode EmissionScope");
        prop_assert_eq!(scope, decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    // 21. Primitive u64 station timestamp roundtrip (greenhouse gas sensor timestamp)
    #[test]
    fn test_primitive_timestamp_roundtrip(ts in any::<u64>()) {
        let bytes = encode_to_vec(&ts).expect("encode u64 timestamp");
        let (decoded, consumed) = decode_from_slice::<u64>(&bytes).expect("decode u64 timestamp");
        prop_assert_eq!(ts, decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    // 22. Mixed Vec<EnergyConsumption> deterministic encoding
    #[test]
    fn test_vec_energy_consumption_deterministic(
        recs in prop::collection::vec(arb_energy_consumption(), 0..8)
    ) {
        let bytes_a = encode_to_vec(&recs).expect("encode Vec<EnergyConsumption> (a)");
        let bytes_b = encode_to_vec(&recs).expect("encode Vec<EnergyConsumption> (b)");
        prop_assert_eq!(bytes_a, bytes_b);
    }
}

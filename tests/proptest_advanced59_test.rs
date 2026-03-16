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

// ── CFD domain types ──────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FlowRegime {
    Laminar,
    Transitional,
    Turbulent,
    Supersonic,
    Hypersonic,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BoundaryCondition {
    NoSlip,
    FreeSlip,
    InletVelocity,
    OutletPressure,
    Periodic,
    Symmetry,
    WallFunction,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TurbulenceModel {
    KepsilonStandard,
    KepsilonRealizable,
    KomegaSst,
    SpalartAllmaras,
    LargeEddySimulation,
    DirectNumericalSimulation,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VelocityField {
    u_x: f64,
    u_y: f64,
    u_z: f64,
    magnitude: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PressureGradient {
    dp_dx: f64,
    dp_dy: f64,
    dp_dz: f64,
    cell_id: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MeshElement {
    element_id: u64,
    node_count: u8,
    volume: f64,
    quality_metric: f64,
    layer_index: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CfdCell {
    cell_id: u64,
    velocity: VelocityField,
    pressure: f64,
    temperature: f64,
    density: f64,
    regime: FlowRegime,
    boundary: BoundaryCondition,
    turbulence_model: TurbulenceModel,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TurbulenceState {
    reynolds_number: f64,
    turbulent_kinetic_energy: f64,
    dissipation_rate: f64,
    turbulent_viscosity: f64,
    model: TurbulenceModel,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VorticityVector {
    omega_x: f64,
    omega_y: f64,
    omega_z: f64,
    magnitude: f64,
    cell_id: u64,
}

// ── Strategy helpers ──────────────────────────────────────────────────────────

fn flow_regime_strategy() -> impl Strategy<Value = FlowRegime> {
    (0u8..5).prop_map(|v| match v {
        0 => FlowRegime::Laminar,
        1 => FlowRegime::Transitional,
        2 => FlowRegime::Turbulent,
        3 => FlowRegime::Supersonic,
        _ => FlowRegime::Hypersonic,
    })
}

fn boundary_condition_strategy() -> impl Strategy<Value = BoundaryCondition> {
    (0u8..7).prop_map(|v| match v {
        0 => BoundaryCondition::NoSlip,
        1 => BoundaryCondition::FreeSlip,
        2 => BoundaryCondition::InletVelocity,
        3 => BoundaryCondition::OutletPressure,
        4 => BoundaryCondition::Periodic,
        5 => BoundaryCondition::Symmetry,
        _ => BoundaryCondition::WallFunction,
    })
}

fn turbulence_model_strategy() -> impl Strategy<Value = TurbulenceModel> {
    (0u8..6).prop_map(|v| match v {
        0 => TurbulenceModel::KepsilonStandard,
        1 => TurbulenceModel::KepsilonRealizable,
        2 => TurbulenceModel::KomegaSst,
        3 => TurbulenceModel::SpalartAllmaras,
        4 => TurbulenceModel::LargeEddySimulation,
        _ => TurbulenceModel::DirectNumericalSimulation,
    })
}

fn velocity_field_strategy() -> impl Strategy<Value = VelocityField> {
    (any::<f64>(), any::<f64>(), any::<f64>(), any::<f64>()).prop_map(
        |(u_x, u_y, u_z, magnitude)| VelocityField {
            u_x,
            u_y,
            u_z,
            magnitude,
        },
    )
}

fn pressure_gradient_strategy() -> impl Strategy<Value = PressureGradient> {
    (any::<f64>(), any::<f64>(), any::<f64>(), any::<u64>()).prop_map(
        |(dp_dx, dp_dy, dp_dz, cell_id)| PressureGradient {
            dp_dx,
            dp_dy,
            dp_dz,
            cell_id,
        },
    )
}

fn mesh_element_strategy() -> impl Strategy<Value = MeshElement> {
    (
        any::<u64>(),
        any::<u8>(),
        any::<f64>(),
        any::<f64>(),
        any::<u32>(),
    )
        .prop_map(
            |(element_id, node_count, volume, quality_metric, layer_index)| MeshElement {
                element_id,
                node_count,
                volume,
                quality_metric,
                layer_index,
            },
        )
}

fn cfd_cell_strategy() -> impl Strategy<Value = CfdCell> {
    (
        any::<u64>(),
        velocity_field_strategy(),
        any::<f64>(),
        any::<f64>(),
        any::<f64>(),
        flow_regime_strategy(),
        boundary_condition_strategy(),
        turbulence_model_strategy(),
    )
        .prop_map(
            |(
                cell_id,
                velocity,
                pressure,
                temperature,
                density,
                regime,
                boundary,
                turbulence_model,
            )| {
                CfdCell {
                    cell_id,
                    velocity,
                    pressure,
                    temperature,
                    density,
                    regime,
                    boundary,
                    turbulence_model,
                }
            },
        )
}

fn turbulence_state_strategy() -> impl Strategy<Value = TurbulenceState> {
    (
        any::<f64>(),
        any::<f64>(),
        any::<f64>(),
        any::<f64>(),
        turbulence_model_strategy(),
    )
        .prop_map(
            |(
                reynolds_number,
                turbulent_kinetic_energy,
                dissipation_rate,
                turbulent_viscosity,
                model,
            )| {
                TurbulenceState {
                    reynolds_number,
                    turbulent_kinetic_energy,
                    dissipation_rate,
                    turbulent_viscosity,
                    model,
                }
            },
        )
}

fn vorticity_vector_strategy() -> impl Strategy<Value = VorticityVector> {
    (
        any::<f64>(),
        any::<f64>(),
        any::<f64>(),
        any::<f64>(),
        any::<u64>(),
    )
        .prop_map(
            |(omega_x, omega_y, omega_z, magnitude, cell_id)| VorticityVector {
                omega_x,
                omega_y,
                omega_z,
                magnitude,
                cell_id,
            },
        )
}

// ── 22 property-based tests ───────────────────────────────────────────────────

proptest! {
    // 1. f64 roundtrip (representative of CFD scalar fields)
    #[test]
    fn test_f64_cfd_scalar_roundtrip(val in any::<f64>()) {
        let encoded = encode_to_vec(&val).expect("encode f64 CFD scalar failed");
        let (decoded, _): (f64, usize) = decode_from_slice(&encoded).expect("decode f64 CFD scalar failed");
        prop_assert_eq!(val.to_bits(), decoded.to_bits());
    }

    // 2. VelocityField struct roundtrip
    #[test]
    fn test_velocity_field_roundtrip(field in velocity_field_strategy()) {
        let encoded = encode_to_vec(&field).expect("encode VelocityField failed");
        let (decoded, _): (VelocityField, usize) = decode_from_slice(&encoded).expect("decode VelocityField failed");
        prop_assert_eq!(field.u_x.to_bits(), decoded.u_x.to_bits());
        prop_assert_eq!(field.u_y.to_bits(), decoded.u_y.to_bits());
        prop_assert_eq!(field.u_z.to_bits(), decoded.u_z.to_bits());
        prop_assert_eq!(field.magnitude.to_bits(), decoded.magnitude.to_bits());
    }

    // 3. PressureGradient struct roundtrip
    #[test]
    fn test_pressure_gradient_roundtrip(grad in pressure_gradient_strategy()) {
        let encoded = encode_to_vec(&grad).expect("encode PressureGradient failed");
        let (decoded, _): (PressureGradient, usize) = decode_from_slice(&encoded).expect("decode PressureGradient failed");
        prop_assert_eq!(grad.cell_id, decoded.cell_id);
        prop_assert_eq!(grad.dp_dx.to_bits(), decoded.dp_dx.to_bits());
        prop_assert_eq!(grad.dp_dy.to_bits(), decoded.dp_dy.to_bits());
        prop_assert_eq!(grad.dp_dz.to_bits(), decoded.dp_dz.to_bits());
    }

    // 4. FlowRegime enum roundtrip
    #[test]
    fn test_flow_regime_enum_roundtrip(regime in flow_regime_strategy()) {
        let encoded = encode_to_vec(&regime).expect("encode FlowRegime failed");
        let (decoded, _): (FlowRegime, usize) = decode_from_slice(&encoded).expect("decode FlowRegime failed");
        prop_assert_eq!(regime, decoded);
    }

    // 5. BoundaryCondition enum roundtrip
    #[test]
    fn test_boundary_condition_enum_roundtrip(bc in boundary_condition_strategy()) {
        let encoded = encode_to_vec(&bc).expect("encode BoundaryCondition failed");
        let (decoded, _): (BoundaryCondition, usize) = decode_from_slice(&encoded).expect("decode BoundaryCondition failed");
        prop_assert_eq!(bc, decoded);
    }

    // 6. TurbulenceModel enum roundtrip
    #[test]
    fn test_turbulence_model_enum_roundtrip(model in turbulence_model_strategy()) {
        let encoded = encode_to_vec(&model).expect("encode TurbulenceModel failed");
        let (decoded, _): (TurbulenceModel, usize) = decode_from_slice(&encoded).expect("decode TurbulenceModel failed");
        prop_assert_eq!(model, decoded);
    }

    // 7. MeshElement struct roundtrip
    #[test]
    fn test_mesh_element_roundtrip(elem in mesh_element_strategy()) {
        let encoded = encode_to_vec(&elem).expect("encode MeshElement failed");
        let (decoded, _): (MeshElement, usize) = decode_from_slice(&encoded).expect("decode MeshElement failed");
        prop_assert_eq!(elem.element_id, decoded.element_id);
        prop_assert_eq!(elem.node_count, decoded.node_count);
        prop_assert_eq!(elem.layer_index, decoded.layer_index);
        prop_assert_eq!(elem.volume.to_bits(), decoded.volume.to_bits());
        prop_assert_eq!(elem.quality_metric.to_bits(), decoded.quality_metric.to_bits());
    }

    // 8. TurbulenceState struct roundtrip
    #[test]
    fn test_turbulence_state_roundtrip(state in turbulence_state_strategy()) {
        let encoded = encode_to_vec(&state).expect("encode TurbulenceState failed");
        let (decoded, _): (TurbulenceState, usize) = decode_from_slice(&encoded).expect("decode TurbulenceState failed");
        prop_assert_eq!(state.model, decoded.model);
        prop_assert_eq!(state.reynolds_number.to_bits(), decoded.reynolds_number.to_bits());
        prop_assert_eq!(state.turbulent_kinetic_energy.to_bits(), decoded.turbulent_kinetic_energy.to_bits());
        prop_assert_eq!(state.dissipation_rate.to_bits(), decoded.dissipation_rate.to_bits());
        prop_assert_eq!(state.turbulent_viscosity.to_bits(), decoded.turbulent_viscosity.to_bits());
    }

    // 9. VorticityVector struct roundtrip
    #[test]
    fn test_vorticity_vector_roundtrip(vort in vorticity_vector_strategy()) {
        let encoded = encode_to_vec(&vort).expect("encode VorticityVector failed");
        let (decoded, _): (VorticityVector, usize) = decode_from_slice(&encoded).expect("decode VorticityVector failed");
        prop_assert_eq!(vort.cell_id, decoded.cell_id);
        prop_assert_eq!(vort.omega_x.to_bits(), decoded.omega_x.to_bits());
        prop_assert_eq!(vort.omega_y.to_bits(), decoded.omega_y.to_bits());
        prop_assert_eq!(vort.omega_z.to_bits(), decoded.omega_z.to_bits());
        prop_assert_eq!(vort.magnitude.to_bits(), decoded.magnitude.to_bits());
    }

    // 10. Nested CfdCell struct roundtrip
    #[test]
    fn test_cfd_cell_nested_struct_roundtrip(cell in cfd_cell_strategy()) {
        let encoded = encode_to_vec(&cell).expect("encode CfdCell failed");
        let (decoded, _): (CfdCell, usize) = decode_from_slice(&encoded).expect("decode CfdCell failed");
        prop_assert_eq!(cell.cell_id, decoded.cell_id);
        prop_assert_eq!(cell.regime, decoded.regime);
        prop_assert_eq!(cell.boundary, decoded.boundary);
        prop_assert_eq!(cell.turbulence_model, decoded.turbulence_model);
        prop_assert_eq!(cell.pressure.to_bits(), decoded.pressure.to_bits());
        prop_assert_eq!(cell.temperature.to_bits(), decoded.temperature.to_bits());
        prop_assert_eq!(cell.density.to_bits(), decoded.density.to_bits());
    }

    // 11. Vec<VelocityField> roundtrip (velocity field array)
    #[test]
    fn test_vec_velocity_field_roundtrip(
        fields in prop::collection::vec(velocity_field_strategy(), 0..8)
    ) {
        let encoded = encode_to_vec(&fields).expect("encode Vec<VelocityField> failed");
        let (decoded, _): (Vec<VelocityField>, usize) = decode_from_slice(&encoded).expect("decode Vec<VelocityField> failed");
        prop_assert_eq!(fields.len(), decoded.len());
        for (orig, dec) in fields.iter().zip(decoded.iter()) {
            prop_assert_eq!(orig.u_x.to_bits(), dec.u_x.to_bits());
            prop_assert_eq!(orig.u_y.to_bits(), dec.u_y.to_bits());
            prop_assert_eq!(orig.u_z.to_bits(), dec.u_z.to_bits());
        }
    }

    // 12. Vec<MeshElement> roundtrip (mesh connectivity)
    #[test]
    fn test_vec_mesh_element_roundtrip(
        elements in prop::collection::vec(mesh_element_strategy(), 0..10)
    ) {
        let encoded = encode_to_vec(&elements).expect("encode Vec<MeshElement> failed");
        let (decoded, _): (Vec<MeshElement>, usize) = decode_from_slice(&encoded).expect("decode Vec<MeshElement> failed");
        prop_assert_eq!(elements.len(), decoded.len());
        for (orig, dec) in elements.iter().zip(decoded.iter()) {
            prop_assert_eq!(orig.element_id, dec.element_id);
            prop_assert_eq!(orig.node_count, dec.node_count);
        }
    }

    // 13. Option<VelocityField> roundtrip (optional field data)
    #[test]
    fn test_option_velocity_field_roundtrip(
        maybe_field in prop::option::of(velocity_field_strategy())
    ) {
        let encoded = encode_to_vec(&maybe_field).expect("encode Option<VelocityField> failed");
        let (decoded, _): (Option<VelocityField>, usize) = decode_from_slice(&encoded).expect("decode Option<VelocityField> failed");
        match (maybe_field, decoded) {
            (None, None) => {}
            (Some(orig), Some(dec)) => {
                prop_assert_eq!(orig.u_x.to_bits(), dec.u_x.to_bits());
                prop_assert_eq!(orig.u_y.to_bits(), dec.u_y.to_bits());
                prop_assert_eq!(orig.u_z.to_bits(), dec.u_z.to_bits());
            }
            _ => prop_assert!(false, "Option mismatch after roundtrip"),
        }
    }

    // 14. Option<TurbulenceState> roundtrip (optional turbulence data)
    #[test]
    fn test_option_turbulence_state_roundtrip(
        maybe_state in prop::option::of(turbulence_state_strategy())
    ) {
        let encoded = encode_to_vec(&maybe_state).expect("encode Option<TurbulenceState> failed");
        let (decoded, _): (Option<TurbulenceState>, usize) = decode_from_slice(&encoded).expect("decode Option<TurbulenceState> failed");
        match (maybe_state, decoded) {
            (None, None) => {}
            (Some(orig), Some(dec)) => {
                prop_assert_eq!(orig.model, dec.model);
                prop_assert_eq!(orig.reynolds_number.to_bits(), dec.reynolds_number.to_bits());
            }
            _ => prop_assert!(false, "Option<TurbulenceState> mismatch"),
        }
    }

    // 15. Consumed bytes equal encoded length for VelocityField
    #[test]
    fn test_velocity_field_consumed_bytes_equals_encoded_length(field in velocity_field_strategy()) {
        let encoded = encode_to_vec(&field).expect("encode VelocityField for consumed bytes check failed");
        let (_, consumed): (VelocityField, usize) = decode_from_slice(&encoded).expect("decode VelocityField for consumed bytes check failed");
        prop_assert_eq!(consumed, encoded.len());
    }

    // 16. Consumed bytes equal encoded length for CfdCell
    #[test]
    fn test_cfd_cell_consumed_bytes_equals_encoded_length(cell in cfd_cell_strategy()) {
        let encoded = encode_to_vec(&cell).expect("encode CfdCell for consumed bytes check failed");
        let (_, consumed): (CfdCell, usize) = decode_from_slice(&encoded).expect("decode CfdCell for consumed bytes check failed");
        prop_assert_eq!(consumed, encoded.len());
    }

    // 17. Deterministic encoding for VelocityField
    #[test]
    fn test_velocity_field_encode_deterministic(field in velocity_field_strategy()) {
        let encoded_first = encode_to_vec(&field).expect("first encode VelocityField failed");
        let encoded_second = encode_to_vec(&field).expect("second encode VelocityField failed");
        prop_assert_eq!(encoded_first, encoded_second);
    }

    // 18. Deterministic encoding for CfdCell
    #[test]
    fn test_cfd_cell_encode_deterministic(cell in cfd_cell_strategy()) {
        let encoded_first = encode_to_vec(&cell).expect("first encode CfdCell failed");
        let encoded_second = encode_to_vec(&cell).expect("second encode CfdCell failed");
        prop_assert_eq!(encoded_first, encoded_second);
    }

    // 19. Reynolds number f64 roundtrip (key CFD dimensionless parameter)
    #[test]
    fn test_reynolds_number_f64_roundtrip(re in any::<f64>()) {
        let encoded = encode_to_vec(&re).expect("encode Reynolds number failed");
        let (decoded, consumed): (f64, usize) = decode_from_slice(&encoded).expect("decode Reynolds number failed");
        prop_assert_eq!(re.to_bits(), decoded.to_bits());
        prop_assert_eq!(consumed, encoded.len());
    }

    // 20. Mach number f64 roundtrip (key CFD compressibility parameter)
    #[test]
    fn test_mach_number_f64_roundtrip(mach in any::<f64>()) {
        let encoded = encode_to_vec(&mach).expect("encode Mach number failed");
        let (decoded, consumed): (f64, usize) = decode_from_slice(&encoded).expect("decode Mach number failed");
        prop_assert_eq!(mach.to_bits(), decoded.to_bits());
        prop_assert_eq!(consumed, encoded.len());
    }

    // 21. Vec<CfdCell> roundtrip (full CFD domain simulation cells)
    #[test]
    fn test_vec_cfd_cell_roundtrip(
        cells in prop::collection::vec(cfd_cell_strategy(), 0..5)
    ) {
        let encoded = encode_to_vec(&cells).expect("encode Vec<CfdCell> failed");
        let (decoded, consumed): (Vec<CfdCell>, usize) = decode_from_slice(&encoded).expect("decode Vec<CfdCell> failed");
        prop_assert_eq!(cells.len(), decoded.len());
        prop_assert_eq!(consumed, encoded.len());
        for (orig, dec) in cells.iter().zip(decoded.iter()) {
            prop_assert_eq!(orig.cell_id, dec.cell_id);
            prop_assert_eq!(&orig.regime, &dec.regime);
            prop_assert_eq!(&orig.boundary, &dec.boundary);
            prop_assert_eq!(&orig.turbulence_model, &dec.turbulence_model);
        }
    }

    // 22. Distinct CfdCells produce distinct or equal encodings (encoding injectivity)
    #[test]
    fn test_distinct_cfd_cells_bytes_reflect_inequality(
        cell_a in cfd_cell_strategy(),
        cell_b in cfd_cell_strategy()
    ) {
        let encoded_a = encode_to_vec(&cell_a).expect("encode CfdCell A failed");
        let encoded_b = encode_to_vec(&cell_b).expect("encode CfdCell B failed");
        if cell_a.cell_id == cell_b.cell_id
            && cell_a.pressure.to_bits() == cell_b.pressure.to_bits()
            && cell_a.temperature.to_bits() == cell_b.temperature.to_bits()
            && cell_a.density.to_bits() == cell_b.density.to_bits()
            && cell_a.regime == cell_b.regime
            && cell_a.boundary == cell_b.boundary
            && cell_a.turbulence_model == cell_b.turbulence_model
            && cell_a.velocity.u_x.to_bits() == cell_b.velocity.u_x.to_bits()
            && cell_a.velocity.u_y.to_bits() == cell_b.velocity.u_y.to_bits()
            && cell_a.velocity.u_z.to_bits() == cell_b.velocity.u_z.to_bits()
            && cell_a.velocity.magnitude.to_bits() == cell_b.velocity.magnitude.to_bits()
        {
            prop_assert_eq!(&encoded_a, &encoded_b);
        } else {
            prop_assert_ne!(&encoded_a, &encoded_b);
        }
    }
}
